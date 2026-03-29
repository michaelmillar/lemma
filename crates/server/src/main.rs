use anyhow::Result;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use lemma_content::ContentStore;
use lemma_core::{
    assessment::{self, AssessmentAxis, SolveOutcome},
    grading,
    problem::{Problem, SubProblem},
    store::{ProgressUpdate, Store},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::{cors::CorsLayer, services::ServeDir};

struct AppState {
    content: ContentStore,
    store: Mutex<Store>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let repo_root = std::env::current_dir()?;
    let content_dir = repo_root.join("content");
    let db_path = repo_root.join(".lemma/lemma.db");

    let content = ContentStore::load(&content_dir)?;
    let store = Store::open(&db_path)?;

    let problem_count = content.problems.len();
    let concept_count = content.concepts.len();

    let state = Arc::new(AppState {
        content,
        store: Mutex::new(store),
    });

    let web_dir = repo_root.join("web/dist");

    let app = Router::new()
        .route("/api/tracks", get(list_tracks))
        .route("/api/problems", get(list_problems))
        .route("/api/problem/{id}", get(get_problem))
        .route("/api/progress", get(get_all_progress))
        .route("/api/progress/{id}", get(get_problem_progress))
        .route("/api/submit/numerical", post(submit_numerical))
        .route("/api/submit/strategy", post(submit_strategy))
        .route("/api/submit/proof", post(submit_proof))
        .route("/api/submit/comprehension", post(submit_comprehension))
        .route("/api/submit/work", post(submit_work))
        .route("/api/advance-phase", post(advance_phase))
        .route("/api/hint", post(get_hint))
        .route("/api/assess/{id}", get(assess_problem))
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new(&web_dir).append_index_html_on_directories(true))
        .with_state(state);

    let addr = "0.0.0.0:3003";
    println!("lemma server starting on {addr}");
    println!("  {problem_count} problems, {concept_count} concepts loaded");
    println!("  web UI: http://localhost:3003");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct TrackSummary {
    id: String,
    problem_count: usize,
}

async fn list_tracks(State(state): State<Arc<AppState>>) -> Json<Vec<TrackSummary>> {
    let tracks: Vec<TrackSummary> = state
        .content
        .tracks()
        .into_iter()
        .map(|t| {
            let count = state.content.problems_for_track(&t).len();
            TrackSummary {
                id: t,
                problem_count: count,
            }
        })
        .collect();
    Json(tracks)
}

#[derive(Serialize)]
struct ProblemSummary {
    id: String,
    title: String,
    track: String,
    tier: String,
    max_score: u32,
    completed: bool,
    total_score: i32,
}

async fn list_problems(State(state): State<Arc<AppState>>) -> Json<Vec<ProblemSummary>> {
    let store = state.store.lock().unwrap();
    let all_progress = store.all_progress().unwrap_or_default();
    let progress_map: std::collections::HashMap<String, _> = all_progress
        .into_iter()
        .map(|p| (p.problem_id.clone(), p))
        .collect();

    let mut problems: Vec<ProblemSummary> = state
        .content
        .problems
        .values()
        .map(|p| {
            let prog = progress_map.get(&p.id);
            ProblemSummary {
                id: p.id.clone(),
                title: p.title.clone(),
                track: p.track.clone(),
                tier: format!("{:?}", p.tier).to_lowercase(),
                max_score: p.max_score(),
                completed: prog.is_some_and(|pr| pr.completed),
                total_score: prog.map_or(0, |pr| pr.total_score),
            }
        })
        .collect();
    problems.sort_by(|a, b| a.id.cmp(&b.id));
    Json(problems)
}

#[derive(Serialize)]
struct ProblemDetail {
    id: String,
    title: String,
    track: String,
    tier: String,
    strategy_tags: Vec<String>,
    concept_tags: Vec<String>,
    spark: SparkJson,
    ground: GroundJson,
    work: Option<WorkJson>,
    sub_problems: Vec<SubProblemJson>,
}

#[derive(Serialize)]
struct WorkJson {
    worked_example: String,
    guided_prompt: String,
}

#[derive(Serialize)]
struct SparkJson {
    scenario: String,
    industry_domain: String,
    connection_question: String,
}

#[derive(Serialize)]
struct GroundJson {
    concept_brief: String,
    definitions: Vec<DefinitionJson>,
    intuition: String,
    connections: Vec<ConnectionJson>,
    field_tags: Vec<String>,
}

#[derive(Serialize)]
struct DefinitionJson {
    term: String,
    statement: String,
    latex: String,
}

#[derive(Serialize)]
struct ConnectionJson {
    target_concept: String,
    relationship: String,
}

#[derive(Serialize)]
struct SubProblemJson {
    index: usize,
    kind: String,
    prompt: String,
    points: u32,
    hint_count: usize,
    choices: Vec<String>,
    rubric: Vec<RubricItemJson>,
    exemplar: Option<String>,
    comprehension: Vec<ComprehensionJson>,
}

#[derive(Serialize)]
struct RubricItemJson {
    criterion: String,
    weight: u32,
}

#[derive(Serialize)]
struct ComprehensionJson {
    question: String,
}

fn problem_to_detail(p: &Problem) -> ProblemDetail {
    ProblemDetail {
        id: p.id.clone(),
        title: p.title.clone(),
        track: p.track.clone(),
        tier: format!("{:?}", p.tier).to_lowercase(),
        strategy_tags: p.strategy_tags.clone(),
        concept_tags: p.concept_tags.clone(),
        spark: SparkJson {
            scenario: p.spark.scenario.clone(),
            industry_domain: p.spark.industry_domain.clone(),
            connection_question: p.spark.connection_question.clone(),
        },
        ground: GroundJson {
            concept_brief: p.ground.concept_brief.clone(),
            definitions: p
                .ground
                .definitions
                .iter()
                .map(|d| DefinitionJson {
                    term: d.term.clone(),
                    statement: d.statement.clone(),
                    latex: d.latex.clone(),
                })
                .collect(),
            intuition: p.ground.intuition.clone(),
            connections: p
                .ground
                .connections
                .iter()
                .map(|c| ConnectionJson {
                    target_concept: c.target_concept.clone(),
                    relationship: c.relationship.clone(),
                })
                .collect(),
            field_tags: p.ground.field_tags.clone(),
        },
        work: p.work.as_ref().map(|w| WorkJson {
            worked_example: w.worked_example.clone(),
            guided_prompt: w.guided_prompt.clone(),
        }),
        sub_problems: p
            .solve
            .sub_problems
            .iter()
            .enumerate()
            .map(|(i, sp)| {
                let (kind, choices, rubric, hint_count, exemplar, comprehension) = match sp {
                    SubProblem::Numerical(n) => (
                        "numerical".to_string(),
                        vec![],
                        vec![],
                        n.hints.len(),
                        None,
                        vec![],
                    ),
                    SubProblem::StrategyId(s) => {
                        let mut opts = s.distractors.clone();
                        opts.push(s.correct_strategy.clone());
                        opts.sort();
                        (
                            "strategy_id".to_string(),
                            opts,
                            vec![],
                            0,
                            None,
                            vec![],
                        )
                    }
                    SubProblem::Proof(pr) => (
                        "proof".to_string(),
                        vec![],
                        pr.rubric
                            .iter()
                            .map(|r| RubricItemJson {
                                criterion: r.criterion.clone(),
                                weight: r.weight,
                            })
                            .collect(),
                        pr.hints.len(),
                        pr.exemplar.clone(),
                        pr.comprehension
                            .iter()
                            .map(|q| ComprehensionJson {
                                question: q.question.clone(),
                            })
                            .collect(),
                    ),
                };
                SubProblemJson {
                    index: i,
                    kind,
                    prompt: sp.prompt().to_string(),
                    points: sp.points(),
                    hint_count,
                    choices,
                    rubric,
                    exemplar,
                    comprehension,
                }
            })
            .collect(),
    }
}

async fn get_problem(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ProblemDetail>, StatusCode> {
    let problem = state.content.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(problem_to_detail(problem)))
}

#[derive(Serialize)]
struct ProgressJson {
    problem_id: String,
    current_phase: String,
    completed: bool,
    total_score: i32,
    max_score: i32,
    strategy_correct: i32,
    strategy_total: i32,
}

async fn get_all_progress(State(state): State<Arc<AppState>>) -> Json<Vec<ProgressJson>> {
    let store = state.store.lock().unwrap();
    let all = store.all_progress().unwrap_or_default();
    Json(
        all.into_iter()
            .map(|p| ProgressJson {
                problem_id: p.problem_id,
                current_phase: p.current_phase,
                completed: p.completed,
                total_score: p.total_score,
                max_score: p.max_score,
                strategy_correct: p.strategy_correct,
                strategy_total: p.strategy_total,
            })
            .collect(),
    )
}

async fn get_problem_progress(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<Option<ProgressJson>> {
    let store = state.store.lock().unwrap();
    let prog = store.get_progress(&id).unwrap_or(None);
    Json(prog.map(|p| ProgressJson {
        problem_id: p.problem_id,
        current_phase: p.current_phase,
        completed: p.completed,
        total_score: p.total_score,
        max_score: p.max_score,
        strategy_correct: p.strategy_correct,
        strategy_total: p.strategy_total,
    }))
}

#[derive(Deserialize)]
struct NumericalSubmission {
    problem_id: String,
    sub_problem: usize,
    answer: f64,
}

#[derive(Serialize)]
struct GradeResponse {
    score: u32,
    max_score: u32,
    feedback: String,
    correct: bool,
}

async fn submit_numerical(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NumericalSubmission>,
) -> Result<Json<GradeResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let sp = problem
        .solve
        .sub_problems
        .get(req.sub_problem)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let SubProblem::Numerical(num) = sp else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let result = grading::grade_numerical(num, req.answer);
    let correct = result.score > 0;

    let store = state.store.lock().unwrap();
    store
        .record_solve(
            &req.problem_id,
            req.sub_problem,
            result.score as i32,
            result.max_score as i32,
            0,
            0,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GradeResponse {
        score: result.score,
        max_score: result.max_score,
        feedback: result.feedback,
        correct,
    }))
}

#[derive(Deserialize)]
struct StrategySubmission {
    problem_id: String,
    sub_problem: usize,
    chosen_strategy: String,
}

async fn submit_strategy(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StrategySubmission>,
) -> Result<Json<GradeResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let sp = problem
        .solve
        .sub_problems
        .get(req.sub_problem)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let SubProblem::StrategyId(strat) = sp else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let result = grading::grade_strategy_id(strat, &req.chosen_strategy);
    let correct = result.score > 0;

    let store = state.store.lock().unwrap();
    store
        .record_solve(
            &req.problem_id,
            req.sub_problem,
            result.score as i32,
            result.max_score as i32,
            0,
            0,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GradeResponse {
        score: result.score,
        max_score: result.max_score,
        feedback: result.feedback,
        correct,
    }))
}

#[derive(Deserialize)]
struct ProofSubmission {
    problem_id: String,
    sub_problem: usize,
    checked_items: Vec<bool>,
}

#[derive(Serialize)]
struct ProofGradeResponse {
    score: u32,
    max_score: u32,
    feedback: String,
    solution_sketch: String,
}

async fn submit_proof(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ProofSubmission>,
) -> Result<Json<ProofGradeResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let sp = problem
        .solve
        .sub_problems
        .get(req.sub_problem)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let SubProblem::Proof(proof) = sp else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let result = grading::grade_proof(proof, &req.checked_items);

    let store = state.store.lock().unwrap();
    store
        .record_solve(
            &req.problem_id,
            req.sub_problem,
            result.score as i32,
            result.max_score as i32,
            0,
            0,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProofGradeResponse {
        score: result.score,
        max_score: result.max_score,
        feedback: result.feedback,
        solution_sketch: proof.solution_sketch.clone(),
    }))
}

#[derive(Deserialize)]
struct WorkSubmission {
    problem_id: String,
    answer: f64,
}

#[derive(Serialize)]
struct WorkGradeResponse {
    correct: bool,
    feedback: String,
}

async fn submit_work(
    State(state): State<Arc<AppState>>,
    Json(req): Json<WorkSubmission>,
) -> Result<Json<WorkGradeResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let work = problem.work.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
    let result = grading::grade_work(work, req.answer);
    Ok(Json(WorkGradeResponse {
        correct: result.correct,
        feedback: result.feedback,
    }))
}

#[derive(Deserialize)]
struct ComprehensionSubmission {
    problem_id: String,
    sub_problem: usize,
    answers: Vec<bool>,
}

#[derive(Serialize)]
struct ComprehensionResponse {
    results: Vec<ComprehensionResultJson>,
    all_correct: bool,
}

#[derive(Serialize)]
struct ComprehensionResultJson {
    correct: bool,
    explanation: String,
}

async fn submit_comprehension(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ComprehensionSubmission>,
) -> Result<Json<ComprehensionResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let sp = problem
        .solve
        .sub_problems
        .get(req.sub_problem)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let SubProblem::Proof(proof) = sp else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let results = grading::grade_comprehension(proof, &req.answers);
    let all_correct = results.iter().all(|r| r.correct);

    Ok(Json(ComprehensionResponse {
        results: results
            .into_iter()
            .map(|r| ComprehensionResultJson {
                correct: r.correct,
                explanation: r.explanation,
            })
            .collect(),
        all_correct,
    }))
}

#[derive(Deserialize)]
struct AdvancePhaseRequest {
    problem_id: String,
    phase: String,
    completed: bool,
    total_score: i32,
    max_score: i32,
    strategy_correct: i32,
    strategy_total: i32,
}

async fn advance_phase(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AdvancePhaseRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    store
        .upsert_progress(&ProgressUpdate {
            problem_id: &req.problem_id,
            phase: &req.phase,
            completed: req.completed,
            total_score: req.total_score,
            max_score: req.max_score,
            strategy_correct: req.strategy_correct,
            strategy_total: req.strategy_total,
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"ok": true})))
}

#[derive(Deserialize)]
struct HintRequest {
    problem_id: String,
    sub_problem: usize,
    hint_index: usize,
}

#[derive(Serialize)]
struct HintResponse {
    text: String,
    cost: u32,
}

async fn get_hint(
    State(state): State<Arc<AppState>>,
    Json(req): Json<HintRequest>,
) -> Result<Json<HintResponse>, StatusCode> {
    let problem = state.content.get(&req.problem_id).ok_or(StatusCode::NOT_FOUND)?;
    let sp = problem
        .solve
        .sub_problems
        .get(req.sub_problem)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let hints = match sp {
        SubProblem::Numerical(n) => &n.hints,
        SubProblem::Proof(p) => &p.hints,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let hint = hints.get(req.hint_index).ok_or(StatusCode::BAD_REQUEST)?;

    Ok(Json(HintResponse {
        text: hint.text.clone(),
        cost: hint.cost,
    }))
}

#[derive(Serialize)]
struct AssessmentResponse {
    axes: Vec<AxisScoreJson>,
    mastery: String,
    total_score: i32,
    max_score: i32,
}

#[derive(Serialize)]
struct AxisScoreJson {
    axis: String,
    label: String,
    score: i32,
}

async fn assess_problem(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<AssessmentResponse>, StatusCode> {
    let problem = state.content.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let store = state.store.lock().unwrap();

    let mut outcomes = Vec::new();
    let mut total_score = 0i32;
    let mut max_score = 0i32;

    for (i, sp) in problem.solve.sub_problems.iter().enumerate() {
        let kind = match sp {
            SubProblem::Numerical(_) => "numerical",
            SubProblem::StrategyId(_) => "strategy_id",
            SubProblem::Proof(_) => "proof",
        };

        let (score, ms) = match store.get_solve(&id, i) {
            Ok(Some(rec)) => (rec.score as u32, rec.max_score as u32),
            _ => (0, sp.points()),
        };

        total_score += score as i32;
        max_score += ms as i32;

        outcomes.push(SolveOutcome {
            kind: kind.to_string(),
            score,
            max_score: ms,
        });
    }

    let progress = store.get_progress(&id).unwrap_or(None);
    let engaged = progress.as_ref().is_some_and(|p| p.current_phase != "spark");

    let result = assessment::compute_assessment(&outcomes, engaged);

    let axes = AssessmentAxis::ALL
        .iter()
        .map(|axis| AxisScoreJson {
            axis: format!("{axis:?}"),
            label: axis.label().to_string(),
            score: result.axes.get(axis).copied().unwrap_or(0),
        })
        .collect();

    Ok(Json(AssessmentResponse {
        axes,
        mastery: result.mastery.label().to_string(),
        total_score,
        max_score,
    }))
}

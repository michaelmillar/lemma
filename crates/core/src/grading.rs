use crate::problem::{NumericalProblem, ProofProblem, RubricItem, StrategyIdProblem, Work};

pub struct GradeResult {
    pub score: u32,
    pub max_score: u32,
    pub feedback: String,
}

pub struct WorkGradeResult {
    pub correct: bool,
    pub feedback: String,
}

pub fn grade_numerical(problem: &NumericalProblem, user_answer: f64) -> GradeResult {
    let correct = (user_answer - problem.answer).abs() <= problem.tolerance;
    GradeResult {
        score: if correct { problem.points } else { 0 },
        max_score: problem.points,
        feedback: if correct {
            "Correct!".to_string()
        } else {
            format!("Incorrect. The answer is {:.4}.", problem.answer)
        },
    }
}

pub fn grade_strategy_id(problem: &StrategyIdProblem, chosen: &str) -> GradeResult {
    let correct = chosen == problem.correct_strategy;
    GradeResult {
        score: if correct { problem.points } else { 0 },
        max_score: problem.points,
        feedback: if correct {
            format!("Correct! {}", problem.explanation)
        } else {
            format!(
                "Not quite. The answer is '{}'. {}",
                problem.correct_strategy, problem.explanation
            )
        },
    }
}

pub fn grade_proof(problem: &ProofProblem, checked_items: &[bool]) -> GradeResult {
    let score: u32 = problem
        .rubric
        .iter()
        .zip(checked_items.iter())
        .filter(|&(_, checked)| *checked)
        .map(|(item, _)| item.weight)
        .sum();

    let feedback = build_proof_feedback(&problem.rubric, checked_items);

    GradeResult {
        score,
        max_score: problem.points,
        feedback,
    }
}

fn build_proof_feedback(rubric: &[RubricItem], checked: &[bool]) -> String {
    let mut lines = Vec::new();
    for (item, &met) in rubric.iter().zip(checked.iter()) {
        let mark = if met { "+" } else { "-" };
        lines.push(format!("[{}] {} ({} pts)", mark, item.criterion, item.weight));
    }
    lines.join("\n")
}

pub fn grade_comprehension(
    problem: &ProofProblem,
    answers: &[bool],
) -> Vec<ComprehensionResult> {
    problem
        .comprehension
        .iter()
        .zip(answers.iter())
        .map(|(q, &user_answer)| ComprehensionResult {
            correct: user_answer == q.answer,
            explanation: q.explanation.clone(),
        })
        .collect()
}

pub struct ComprehensionResult {
    pub correct: bool,
    pub explanation: String,
}

pub fn grade_work(work: &Work, user_answer: f64) -> WorkGradeResult {
    let correct = (user_answer - work.guided_answer).abs() <= work.guided_tolerance;
    WorkGradeResult {
        correct,
        feedback: if correct {
            "Correct! You applied the method successfully.".to_string()
        } else {
            format!(
                "Not quite. The answer is {:.4}. Review the worked example above and try to spot where your approach diverged.",
                work.guided_answer
            )
        },
    }
}

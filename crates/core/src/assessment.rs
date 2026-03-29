use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentAxis {
    ConceptualUnderstanding,
    StrategyIdentification,
    ProblemExecution,
    ProofQuality,
    ApplicationAwareness,
}

impl AssessmentAxis {
    pub const ALL: [AssessmentAxis; 5] = [
        AssessmentAxis::ConceptualUnderstanding,
        AssessmentAxis::StrategyIdentification,
        AssessmentAxis::ProblemExecution,
        AssessmentAxis::ProofQuality,
        AssessmentAxis::ApplicationAwareness,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AssessmentAxis::ConceptualUnderstanding => "Conceptual Understanding",
            AssessmentAxis::StrategyIdentification => "Strategy Identification",
            AssessmentAxis::ProblemExecution => "Problem Execution",
            AssessmentAxis::ProofQuality => "Proof Quality",
            AssessmentAxis::ApplicationAwareness => "Application Awareness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MasteryLevel {
    Novice,
    Competent,
    Proficient,
    Expert,
}

impl MasteryLevel {
    pub fn label(&self) -> &'static str {
        match self {
            MasteryLevel::Novice => "Novice",
            MasteryLevel::Competent => "Competent",
            MasteryLevel::Proficient => "Proficient",
            MasteryLevel::Expert => "Expert",
        }
    }
}

pub struct SolveOutcome {
    pub kind: String,
    pub score: u32,
    pub max_score: u32,
}

pub struct AssessmentResult {
    pub axes: std::collections::HashMap<AssessmentAxis, i32>,
    pub mastery: MasteryLevel,
}

pub fn compute_assessment(outcomes: &[SolveOutcome], engaged_ground: bool) -> AssessmentResult {
    let mut numerical_score = 0u32;
    let mut numerical_max = 0u32;
    let mut strategy_score = 0u32;
    let mut strategy_max = 0u32;
    let mut proof_score = 0u32;
    let mut proof_max = 0u32;

    for o in outcomes {
        match o.kind.as_str() {
            "numerical" => {
                numerical_score += o.score;
                numerical_max += o.max_score;
            }
            "strategy_id" => {
                strategy_score += o.score;
                strategy_max += o.max_score;
            }
            "proof" => {
                proof_score += o.score;
                proof_max += o.max_score;
            }
            _ => {}
        }
    }

    let pct = |s: u32, m: u32| -> i32 {
        if m == 0 {
            0
        } else {
            ((s as f64 / m as f64) * 100.0).round() as i32
        }
    };

    let conceptual = if engaged_ground {
        let base = 60;
        let bonus = pct(numerical_score, numerical_max).min(40);
        (base + bonus).min(100)
    } else {
        pct(numerical_score, numerical_max).min(50)
    };

    let strategy_id = pct(strategy_score, strategy_max);
    let execution = pct(numerical_score, numerical_max);
    let proof_quality = pct(proof_score, proof_max);

    let application = if engaged_ground {
        let base = 50;
        let bonus = pct(numerical_score, numerical_max) / 3;
        (base + bonus).min(100)
    } else {
        pct(numerical_score, numerical_max) / 2
    };

    let mut axes = std::collections::HashMap::new();
    axes.insert(AssessmentAxis::ConceptualUnderstanding, conceptual);
    axes.insert(AssessmentAxis::StrategyIdentification, strategy_id);
    axes.insert(AssessmentAxis::ProblemExecution, execution);
    axes.insert(AssessmentAxis::ProofQuality, proof_quality);
    axes.insert(AssessmentAxis::ApplicationAwareness, application);

    let mastery = mastery_from_axes(&axes);

    AssessmentResult { axes, mastery }
}

pub fn mastery_from_axes(axes: &std::collections::HashMap<AssessmentAxis, i32>) -> MasteryLevel {
    let values: Vec<i32> = AssessmentAxis::ALL
        .iter()
        .map(|a| axes.get(a).copied().unwrap_or(0).clamp(0, 100))
        .collect();

    let avg = if values.is_empty() {
        0
    } else {
        values.iter().sum::<i32>() / values.len() as i32
    };

    let all_above_70 = values.iter().all(|&v| v >= 70);

    if avg >= 85 && all_above_70 {
        MasteryLevel::Expert
    } else if avg >= 65 {
        MasteryLevel::Proficient
    } else if avg >= 40 {
        MasteryLevel::Competent
    } else {
        MasteryLevel::Novice
    }
}

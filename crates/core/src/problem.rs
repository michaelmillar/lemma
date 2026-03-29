use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Foundation,
    Core,
    Advanced,
    Research,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::Foundation => "Foundation",
            Tier::Core => "Core",
            Tier::Advanced => "Advanced",
            Tier::Research => "Research",
        }
    }

    pub fn difficulty_hint(&self) -> f32 {
        match self {
            Tier::Foundation => 0.2,
            Tier::Core => 0.5,
            Tier::Advanced => 0.8,
            Tier::Research => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub track: String,
    pub tier: Tier,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub strategy_tags: Vec<String>,
    #[serde(default)]
    pub concept_tags: Vec<String>,
    pub spark: Spark,
    pub ground: Ground,
    pub solve: Solve,
}

impl Problem {
    pub fn max_score(&self) -> u32 {
        self.solve.sub_problems.iter().map(|sp| sp.points()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spark {
    pub scenario: String,
    pub industry_domain: String,
    pub connection_question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ground {
    pub concept_brief: String,
    #[serde(default)]
    pub definitions: Vec<Definition>,
    pub intuition: String,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub field_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub term: String,
    pub statement: String,
    #[serde(default)]
    pub latex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub target_concept: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solve {
    pub sub_problems: Vec<SubProblem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubProblem {
    Numerical(NumericalProblem),
    StrategyId(StrategyIdProblem),
    Proof(ProofProblem),
}

impl SubProblem {
    pub fn points(&self) -> u32 {
        match self {
            SubProblem::Numerical(p) => p.points,
            SubProblem::StrategyId(p) => p.points,
            SubProblem::Proof(p) => p.points,
        }
    }

    pub fn prompt(&self) -> &str {
        match self {
            SubProblem::Numerical(p) => &p.prompt,
            SubProblem::StrategyId(p) => &p.prompt,
            SubProblem::Proof(p) => &p.prompt,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericalProblem {
    pub prompt: String,
    pub answer: f64,
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,
    pub points: u32,
    #[serde(default)]
    pub hints: Vec<Hint>,
}

fn default_tolerance() -> f64 {
    0.001
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyIdProblem {
    pub prompt: String,
    pub correct_strategy: String,
    #[serde(default)]
    pub distractors: Vec<String>,
    pub explanation: String,
    pub points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofProblem {
    pub prompt: String,
    #[serde(default)]
    pub rubric: Vec<RubricItem>,
    pub solution_sketch: String,
    pub points: u32,
    #[serde(default)]
    pub hints: Vec<Hint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricItem {
    pub criterion: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub cost: u32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: String,
    pub label: String,
    pub description: String,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

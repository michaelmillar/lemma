use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyCategory {
    PolyaPhase,
    ZeitzStrategy,
    ZeitzTactic,
    ZeitzCrossover,
    ProofMethod,
}

impl StrategyCategory {
    pub fn label(&self) -> &'static str {
        match self {
            StrategyCategory::PolyaPhase => "Polya Phase",
            StrategyCategory::ZeitzStrategy => "Zeitz Strategy",
            StrategyCategory::ZeitzTactic => "Zeitz Tactic",
            StrategyCategory::ZeitzCrossover => "Zeitz Crossover",
            StrategyCategory::ProofMethod => "Proof Method",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: String,
    pub label: String,
    pub category: StrategyCategory,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

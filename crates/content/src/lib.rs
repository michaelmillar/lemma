use anyhow::{Context, Result};
use lemma_core::{
    problem::{Concept, Problem},
    strategy::Strategy,
};
use std::{collections::HashMap, path::Path};

#[derive(serde::Deserialize)]
struct ConceptFile {
    concepts: Vec<Concept>,
}

#[derive(serde::Deserialize)]
struct StrategyFile {
    strategies: Vec<Strategy>,
}

pub struct ContentStore {
    pub problems: HashMap<String, Problem>,
    pub concepts: HashMap<String, Concept>,
    pub strategies: HashMap<String, Strategy>,
    pub alias_lookup: HashMap<String, String>,
}

impl ContentStore {
    pub fn load(content_dir: &Path) -> Result<Self> {
        let concepts = load_concepts(&content_dir.join("concepts.toml"))?;
        let strategies = load_strategies(&content_dir.join("strategies.toml"))?;
        let problems = load_all_problems(&content_dir.join("tracks"))?;

        validate_problem_refs(&problems, &concepts, &strategies)?;

        let alias_lookup = build_alias_lookup(&concepts, &strategies);

        Ok(Self {
            problems,
            concepts,
            strategies,
            alias_lookup,
        })
    }

    pub fn get(&self, id: &str) -> Option<&Problem> {
        self.problems.get(id)
    }

    pub fn problems_for_track(&self, track: &str) -> Vec<&Problem> {
        let mut ps: Vec<&Problem> = self
            .problems
            .values()
            .filter(|p| p.track == track)
            .collect();
        ps.sort_by(|a, b| a.id.cmp(&b.id));
        ps
    }

    pub fn tracks(&self) -> Vec<String> {
        let mut tracks: Vec<String> = self
            .problems
            .values()
            .map(|p| p.track.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        tracks.sort();
        tracks
    }
}

fn load_concepts(path: &Path) -> Result<HashMap<String, Concept>> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let file: ConceptFile =
        toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(file
        .concepts
        .into_iter()
        .map(|c| (c.id.clone(), c))
        .collect())
}

fn load_strategies(path: &Path) -> Result<HashMap<String, Strategy>> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let file: StrategyFile =
        toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(file
        .strategies
        .into_iter()
        .map(|s| (s.id.clone(), s))
        .collect())
}

fn load_all_problems(tracks_dir: &Path) -> Result<HashMap<String, Problem>> {
    let mut problems = HashMap::new();

    if !tracks_dir.exists() {
        return Ok(problems);
    }

    for track_entry in std::fs::read_dir(tracks_dir)
        .with_context(|| format!("reading tracks dir: {}", tracks_dir.display()))?
    {
        let track_path = track_entry?.path();
        if !track_path.is_dir() {
            continue;
        }

        for entry in std::fs::read_dir(&track_path)
            .with_context(|| format!("reading track dir: {}", track_path.display()))?
        {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("toml") {
                continue;
            }
            let raw = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let problem: Problem =
                toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
            problems.insert(problem.id.clone(), problem);
        }
    }

    Ok(problems)
}

fn validate_problem_refs(
    problems: &HashMap<String, Problem>,
    concepts: &HashMap<String, Concept>,
    strategies: &HashMap<String, Strategy>,
) -> Result<()> {
    for problem in problems.values() {
        for concept_id in &problem.concept_tags {
            anyhow::ensure!(
                concepts.contains_key(concept_id),
                "problem {} references unknown concept '{}'",
                problem.id,
                concept_id
            );
        }
        for strategy_id in &problem.strategy_tags {
            anyhow::ensure!(
                strategies.contains_key(strategy_id),
                "problem {} references unknown strategy '{}'",
                problem.id,
                strategy_id
            );
        }
        for prereq in &problem.prerequisites {
            anyhow::ensure!(
                problems.contains_key(prereq),
                "problem {} references unknown prerequisite '{}'",
                problem.id,
                prereq
            );
        }
    }
    Ok(())
}

fn normalise_term(s: &str) -> String {
    s.to_lowercase().replace(['-', '_'], " ")
}

fn build_alias_lookup(
    concepts: &HashMap<String, Concept>,
    strategies: &HashMap<String, Strategy>,
) -> HashMap<String, String> {
    let mut lookup = HashMap::new();

    for concept in concepts.values() {
        let canonical = concept.id.clone();
        lookup.insert(normalise_term(&concept.id), canonical.clone());
        lookup.insert(normalise_term(&concept.label), canonical.clone());
        for alias in &concept.aliases {
            lookup.insert(normalise_term(alias), canonical.clone());
        }
    }

    for strategy in strategies.values() {
        let canonical = strategy.id.clone();
        lookup.insert(normalise_term(&strategy.id), canonical.clone());
        lookup.insert(normalise_term(&strategy.label), canonical.clone());
        for alias in &strategy.aliases {
            lookup.insert(normalise_term(alias), canonical.clone());
        }
    }

    lookup
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn content_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content")
    }

    #[test]
    fn content_loads_successfully() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        assert!(
            store.problems.len() >= 13,
            "should load at least 13 probability problems"
        );
        assert!(
            store.concepts.len() >= 20,
            "should load at least 20 concepts"
        );
        assert!(
            store.strategies.len() >= 15,
            "should load at least 15 strategies"
        );
    }

    #[test]
    fn all_concept_refs_valid() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        for problem in store.problems.values() {
            for tag in &problem.concept_tags {
                assert!(
                    store.concepts.contains_key(tag),
                    "problem {} references unknown concept '{}'",
                    problem.id,
                    tag
                );
            }
        }
    }

    #[test]
    fn all_strategy_refs_valid() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        for problem in store.problems.values() {
            for tag in &problem.strategy_tags {
                assert!(
                    store.strategies.contains_key(tag),
                    "problem {} references unknown strategy '{}'",
                    problem.id,
                    tag
                );
            }
        }
    }

    #[test]
    fn all_prerequisites_valid() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        for problem in store.problems.values() {
            for prereq in &problem.prerequisites {
                assert!(
                    store.problems.contains_key(prereq),
                    "problem {} references unknown prerequisite '{}'",
                    problem.id,
                    prereq
                );
            }
        }
    }

    #[test]
    fn tracks_discovered() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        let tracks = store.tracks();
        assert!(tracks.contains(&"probability".to_string()));
    }

    #[test]
    fn problems_ordered_within_track() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        let probs = store.problems_for_track("probability");
        assert!(probs.len() >= 13);
        for window in probs.windows(2) {
            assert!(window[0].id <= window[1].id, "problems should be sorted by id");
        }
    }

    #[test]
    fn alias_lookup_resolves() {
        let store = ContentStore::load(&content_dir()).expect("content should load");
        assert_eq!(
            store.alias_lookup.get("bayes rule"),
            Some(&"bayes-theorem".to_string())
        );
        assert_eq!(
            store.alias_lookup.get("pigeonhole principle"),
            Some(&"pigeonhole".to_string())
        );
    }
}

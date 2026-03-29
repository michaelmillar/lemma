use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub struct Store {
    conn: Connection,
}

pub struct SolveRecord {
    pub score: i32,
    pub max_score: i32,
    pub hints_used: i32,
    pub hint_penalty: i32,
}

pub struct ProgressUpdate<'a> {
    pub problem_id: &'a str,
    pub phase: &'a str,
    pub completed: bool,
    pub total_score: i32,
    pub max_score: i32,
    pub strategy_correct: i32,
    pub strategy_total: i32,
}

pub struct ProblemProgress {
    pub problem_id: String,
    pub current_phase: String,
    pub completed: bool,
    pub total_score: i32,
    pub max_score: i32,
    pub strategy_correct: i32,
    pub strategy_total: i32,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating store dir: {}", parent.display()))?;
        }
        let conn =
            Connection::open(path).with_context(|| format!("opening db: {}", path.display()))?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS solves (
                problem_id    TEXT NOT NULL,
                sub_problem   INTEGER NOT NULL,
                score         INTEGER NOT NULL,
                max_score     INTEGER NOT NULL,
                hints_used    INTEGER NOT NULL DEFAULT 0,
                hint_penalty  INTEGER NOT NULL DEFAULT 0,
                solved_at     TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (problem_id, sub_problem)
            );

            CREATE TABLE IF NOT EXISTS problem_progress (
                problem_id        TEXT NOT NULL PRIMARY KEY,
                current_phase     TEXT NOT NULL DEFAULT 'spark',
                completed         INTEGER NOT NULL DEFAULT 0,
                total_score       INTEGER NOT NULL DEFAULT 0,
                max_score         INTEGER NOT NULL DEFAULT 0,
                strategy_correct  INTEGER NOT NULL DEFAULT 0,
                strategy_total    INTEGER NOT NULL DEFAULT 0,
                started_at        TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at      TEXT
            );

            CREATE TABLE IF NOT EXISTS self_assessments (
                problem_id     TEXT NOT NULL,
                sub_problem    INTEGER NOT NULL,
                rubric_item    TEXT NOT NULL,
                self_score     INTEGER NOT NULL,
                assessed_at    TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (problem_id, sub_problem, rubric_item)
            );",
        )?;
        Ok(())
    }

    pub fn record_solve(
        &self,
        problem_id: &str,
        sub_problem: usize,
        score: i32,
        max_score: i32,
        hints_used: i32,
        hint_penalty: i32,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO solves (problem_id, sub_problem, score, max_score, hints_used, hint_penalty)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![problem_id, sub_problem as i32, score, max_score, hints_used, hint_penalty],
        )?;
        Ok(())
    }

    pub fn get_solve(&self, problem_id: &str, sub_problem: usize) -> Result<Option<SolveRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT score, max_score, hints_used, hint_penalty FROM solves WHERE problem_id = ?1 AND sub_problem = ?2",
        )?;
        let result = stmt.query_row(
            rusqlite::params![problem_id, sub_problem as i32],
            |row| {
                Ok(SolveRecord {
                    score: row.get(0)?,
                    max_score: row.get(1)?,
                    hints_used: row.get(2)?,
                    hint_penalty: row.get(3)?,
                })
            },
        );
        match result {
            Ok(record) => Ok(Some(record)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn upsert_progress(&self, p: &ProgressUpdate) -> Result<()> {
        let completed_at = if p.completed {
            Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
        } else {
            None
        };

        self.conn.execute(
            "INSERT INTO problem_progress (problem_id, current_phase, completed, total_score, max_score, strategy_correct, strategy_total, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(problem_id) DO UPDATE SET
                current_phase = excluded.current_phase,
                completed = excluded.completed,
                total_score = excluded.total_score,
                max_score = excluded.max_score,
                strategy_correct = excluded.strategy_correct,
                strategy_total = excluded.strategy_total,
                completed_at = excluded.completed_at",
            rusqlite::params![p.problem_id, p.phase, p.completed as i32, p.total_score, p.max_score, p.strategy_correct, p.strategy_total, completed_at],
        )?;
        Ok(())
    }

    pub fn get_progress(&self, problem_id: &str) -> Result<Option<ProblemProgress>> {
        let mut stmt = self.conn.prepare(
            "SELECT problem_id, current_phase, completed, total_score, max_score, strategy_correct, strategy_total
             FROM problem_progress WHERE problem_id = ?1",
        )?;
        let result = stmt.query_row(rusqlite::params![problem_id], |row| {
            Ok(ProblemProgress {
                problem_id: row.get(0)?,
                current_phase: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                total_score: row.get(3)?,
                max_score: row.get(4)?,
                strategy_correct: row.get(5)?,
                strategy_total: row.get(6)?,
            })
        });
        match result {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn all_progress(&self) -> Result<Vec<ProblemProgress>> {
        let mut stmt = self.conn.prepare(
            "SELECT problem_id, current_phase, completed, total_score, max_score, strategy_correct, strategy_total
             FROM problem_progress ORDER BY problem_id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ProblemProgress {
                problem_id: row.get(0)?,
                current_phase: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                total_score: row.get(3)?,
                max_score: row.get(4)?,
                strategy_correct: row.get(5)?,
                strategy_total: row.get(6)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let store = Store::open(&db_path).unwrap();

        store.record_solve("prob-01", 0, 20, 20, 0, 0).unwrap();
        let solve = store.get_solve("prob-01", 0).unwrap().unwrap();
        assert_eq!(solve.score, 20);
        assert_eq!(solve.max_score, 20);

        store
            .upsert_progress(&ProgressUpdate {
                problem_id: "prob-01",
                phase: "solve",
                completed: false,
                total_score: 20,
                max_score: 50,
                strategy_correct: 1,
                strategy_total: 1,
            })
            .unwrap();
        let progress = store.get_progress("prob-01").unwrap().unwrap();
        assert_eq!(progress.current_phase, "solve");
        assert!(!progress.completed);
        assert_eq!(progress.total_score, 20);
    }
}

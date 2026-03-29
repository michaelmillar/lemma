<p align="center">
  <img src="assets/logo.svg" alt="Lemma" width="240" />
</p>

<p align="center">
  Learn mathematics through real-world problems.<br/>
  Applied first. Rigorous always.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-workspace-orange?logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/svelte-5-orange?logo=svelte" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/tracks-4-7c3aed" alt="4 tracks" />
  <img src="https://img.shields.io/badge/problems-52-7c3aed" alt="52 problems" />
</p>

---

## What is Lemma?

Lemma is a mathematics learning game that starts with *why* before teaching *what* and *how*. Every problem begins with a real-world scenario from industry (ML, networking, finance, graphics) that motivates the underlying mathematics.

**Three-phase game loop:**

```
SPARK    Real-world scenario. Why should you care about this maths?
GROUND   The concept. Definitions, intuition, connections to other areas.
SOLVE    Problems. Numerical answers, strategy identification, proof writing.
```

**Hybrid grading** mixes auto-checked numerical answers, multiple-choice strategy identification (Zeitz/Polya tactics), and self-assessed proof rubrics.

## Walkthrough

https://github.com/user-attachments/assets/lemma-walkthrough.mp4

Generate the video locally:

```bash
cargo run --bin lemma &
cd scripts && python record_demo.py
```

Requires `playwright`, `Pillow`, and `ffmpeg`.

## Tracks

| Track | Problems | Sparks |
|-------|----------|--------|
| **Probability** | 13 | A/B testing, spam filtering, PageRank, diffusion models, load balancing |
| **Linear Algebra** | 13 | Word embeddings, image filters, cryptography, PCA, recommendation engines |
| **Calculus** | 13 | Gradient descent, backpropagation, game physics, Taylor approximations |
| **Discrete Maths** | 13 | Feature combinations, social networks, hashing, RSA, route optimisation |

Each track spans four difficulty tiers:

- **Foundation** (undergraduate 1-2)
- **Core** (final year / competition)
- **Advanced** (Putnam/IMO level)
- **Research** (frontier-adjacent)

## Problem-Solving Framework

Problems are tagged with strategies from two canonical sources:

**Polya** (How to Solve It): understand, plan, execute, look back

**Zeitz** (The Art and Craft of Problem Solving): get your hands dirty, penultimate step, wishful thinking, symmetry, extreme principle, pigeonhole, invariants

Strategy identification is graded as part of the game loop, building meta-cognitive skills alongside domain knowledge.

## Assessment

After completing a problem, Lemma computes scores across five axes:

| Axis | What it measures |
|------|-----------------|
| Conceptual Understanding | Can you state definitions and explain intuition? |
| Strategy Identification | Did you pick the right Polya/Zeitz tactic? |
| Problem Execution | Did you get the numerical answer right? |
| Proof Quality | Self-assessed clarity and completeness of reasoning |
| Application Awareness | Can you connect the maths to real-world use? |

Mastery levels: **Novice** < 40 | **Competent** 40-65 | **Proficient** 65-85 | **Expert** 85+

## Architecture

```
lemma/
  Cargo.toml                 workspace
  crates/
    core/                    types, grading, SQLite store
    content/                 TOML loader, cross-ref validation
    server/                  axum API (port 3003)
  content/
    concepts.toml            74 mathematical concepts
    strategies.toml          21 problem-solving strategies
    tracks/                  52 problems across 4 tracks
  web/                       Svelte 5 + Vite 8 frontend
```

Part of the [studying](https://github.com/michaelmillar?tab=repositories&q=studying) ecosystem, integrated with the spaced-repetition engine for scheduled review.

## Running

```bash
# build the web frontend
cd web && npm install && npm run build && cd ..

# start the server
cargo run --bin lemma

# open http://localhost:3003
```

For development with hot reload:

```bash
cargo run --bin lemma &
cd web && npm run dev
# open http://localhost:5173 (proxies API to :3003)
```

## Content Format

Problems are authored as TOML files. Each file contains a spark (industry scenario), ground (concept definitions and intuition), and solve section (sub-problems with hints and rubrics).

```toml
id = "prob-02-bayes-theorem"
title = "Bayes' Theorem and Spam Filtering"
track = "probability"
tier = "foundation"
strategy_tags = ["get-hands-dirty", "draw-picture"]
concept_tags = ["conditional-probability", "bayes-theorem"]

[spark]
scenario = "A spam filter classifies 100 million messages per day..."

[ground]
concept_brief = "Bayes' theorem inverts conditional probabilities..."

[[solve.sub_problems]]
kind = "numerical"
prompt = "Compute P(spam|flagged)..."
answer = 0.6667
tolerance = 0.001
points = 20
```

## Sources

The curriculum draws structure and inspiration from:

- *The Princeton Companion to Mathematics* (Timothy Gowers)
- *The Art and Craft of Problem Solving* (Paul Zeitz)
- *How to Solve It* (George Polya)
- *Concrete Mathematics* (Graham, Knuth, Patashnik)
- *Proofs from THE BOOK* (Aigner, Ziegler)

Problem databases: AoPS Wiki, Project Euler, Putnam archives, IMO shortlists.

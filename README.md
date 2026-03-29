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

<video src="https://github.com/michaelmillar/lemma/raw/main/assets/demo.mp4" controls width="100%"></video>

Generate the video locally:

```bash
cargo run --bin lemma &
cd scripts && python record_demo.py
```

Requires `playwright`, `Pillow`, and `ffmpeg`.

## Why Lemma?

There are good maths learning tools. None of them do what Lemma does.

| Tool | Teaches concepts | Applied motivation | Named strategies | Spaced repetition | Practitioner audience |
|------|:---:|:---:|:---:|:---:|:---:|
| **Brilliant** | Yes | Sometimes | No | No | No |
| **Khan Academy** | Yes | Rarely | No | No | No |
| **Project Euler** | No | No | No | No | Partially |
| **3Blue1Brown** | Beautifully | Partially | No | No | No |
| **MIT OCW** | Yes | Depends on lecturer | No | No | Partially |
| **Art of Problem Solving** | Yes | No | Implicitly | No | No |
| **Math Academy** | Yes | No | No | Yes (best in class) | No |
| **Anki maths decks** | No (retention only) | No | No | Yes | Partially |
| **Lemma** | Yes | Every problem | Yes (21 named) | Yes (FSRS) | Yes |

### What is missing elsewhere

**Brilliant** teaches through interactive puzzles but never names a problem-solving strategy, never connects to your actual work, and has no spaced repetition. Subscribers report depth plateaus quickly.

**Khan Academy** is excellent for fundamentals but maxes out at early university level. An engineer brushing up on eigendecomposition for PCA will find the explanation adequate, the practice shallow, and the path to application invisible.

**Project Euler** is a puzzle collection with zero instruction. 900+ problems, no curriculum, no feedback on method. Brilliant for self-starters, useless for systematic skill-building.

**3Blue1Brown** builds beautiful geometric intuition. But you watch someone else think. The LessWrong community identifies "a sizeable chunk of value left on the table by not having exercises." The bridge from visual intuition to computational fluency does not exist.

**Art of Problem Solving** is the closest to teaching meta-strategies, but it targets gifted secondary school students preparing for competitions. Adults and engineers are not the audience. Content is contest maths, not applied maths.

**Math Academy** has the most sophisticated adaptive SRS in the space, but it optimises for procedural fluency in a K-12 curriculum. No applied motivation, no strategy instruction, no practitioner focus.

**Anki** is a retention tool, not a learning tool. Making good maths cards takes hours per theorem, and the gap between "I can recall the definition of eigenvectors" and "I can recognise when eigendecomposition solves my problem" is never bridged.

### The gap Lemma fills

No tool combines these five properties:

1. **Applied-first motivation.** Every problem starts with a real scenario (spam filters, gradient descent, PageRank) before touching the maths. This is not decoration; it is the entry point.

2. **Named meta-strategies.** 21 Polya/Zeitz tactics as first-class content. Strategy identification is graded. No other digital tool does this.

3. **Three-phase pedagogy.** Why (Spark), then what (Ground), then how (Solve). Khan teaches what-then-how. Brilliant teaches how. 3Blue1Brown teaches what. Lemma sequences all three.

4. **FSRS spaced repetition for structured maths.** Not flat flashcards. Multi-axis quality signals (execution + strategy) feed the scheduling algorithm across prerequisite chains.

5. **Practitioner audience.** Every other tool targets students. The working engineer who needs rigorous foundations for the ML, systems, or finance work they already do has zero alternatives.

### The honest criticism

> "52 problems is a weekend of content. Brilliant has hundreds of courses. Your catalogue is thin."

This is true today. The rebuttal is threefold. Each Lemma problem delivers roughly 3x the pedagogical surface of a typical exercise (spark + concept + mixed sub-problems). The TOML content format means adding a track is authoring work, not engineering. And a small catalogue in an empty market segment is more valuable than a large catalogue in a saturated one. The architecture scales; the content follows.

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

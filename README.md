<p align="center">
  <img src="assets/logo.svg" alt="Lemma" width="240" />
</p>

<p align="center">
  The maths behind the systems you build.<br/>
  For engineers and practitioners. Applied first. Rigorous always.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-workspace-orange?logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/svelte-5-orange?logo=svelte" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/tracks-4-7c3aed" alt="4 tracks" />
  <img src="https://img.shields.io/badge/problems-52-7c3aed" alt="52 problems" />
</p>

---

## What is Lemma?

Lemma is a structured mathematics tool for software engineers, ML practitioners, and technical career-switchers who need rigorous foundations for the work they already do. Every problem starts from a real failure, bottleneck, or design decision in production systems, then maps backward into the maths that explains it.

**Four-phase learning loop:**

```
SPARK    A real failure or design problem. Your gradient descent diverges.
         Your A/B test is underpowered. Your PageRank doesn't converge.
GROUND   The concept, compact. Definitions, intuition, one key connection.
WORK     Faded worked examples. Watch one solved, then complete a guided one.
SOLVE    Independent problems. Compute, identify the strategy, prove, transfer.
```

**Hybrid grading** mixes auto-checked numerical answers, named strategy identification (21 Polya/Zeitz tactics), and structured proof assessment with comprehension checks and exemplar comparison.

## Walkthrough

https://github.com/user-attachments/assets/ca81b863-e2f2-4d46-aa5a-ca28466aa301

## Who is this for?

**Software engineers** who use maths daily but never formalised it. You can call `np.linalg.svd` but cannot explain why truncating singular values denoises your data. You tune learning rates by feel because you never properly internalised gradient dynamics.

**Career-switchers** moving into ML, data science, or quantitative roles. You need probability, linear algebra, and calculus, but existing resources teach to students, not practitioners. You want to know why the maths matters for the systems you will build.

## How it compares

| Tool | Applied context | Named strategies | Worked examples | Proof assessment | Practitioner focus |
|------|:---:|:---:|:---:|:---:|:---:|
| **Brilliant** | Sometimes | No | No | No | No |
| **Khan Academy** | Rarely | No | Yes | No | No |
| **Math Academy** | No | No | Yes | No | No |
| **3Blue1Brown** | Partially | No | No | No | No |
| **Art of Problem Solving** | No | Implicitly | Partially | No | No |
| **Lemma** | Every problem | Yes (21 named) | Faded sequence | Structure + comprehension + exemplar | Yes |

### Strengths

1. **Every problem starts from a real system.** Spam filters, gradient descent, PageRank, packet queues, hedge ratios. Not decoration. The entry point.

2. **Strategy identification is graded.** 21 Polya/Zeitz tactics as first-class content. You choose the attack before solving. No other tool does this.

3. **Four-phase pedagogy.** Spark (why), Ground (what), Work (guided practice with faded examples), Solve (independent). Research-aligned sequencing from motivation through to transfer.

4. **Structured proof feedback.** Not just "rate yourself 1 to 5." Structure checklist, comprehension questions about proof logic, and compare-to-exemplar.

5. **FSRS spaced repetition.** Multi-axis quality signals (execution + strategy) feed scheduling across prerequisite chains.

### Weaknesses

**52 problems is thin.** Brilliant has hundreds of courses. Each Lemma problem delivers roughly 3x the pedagogical surface of a typical exercise (spark + concept + worked example + mixed sub-problems), but the catalogue needs to grow. The TOML content format means adding a track is authoring work, not engineering.

**No AI-powered proof critique yet.** The structured assessment (checklist + comprehension + exemplar) is stronger than pure self-grading, but automated feedback on free-form proofs is a clear next step.

**Not for beginners.** If you need to learn what a fraction is, this is not the right tool. Lemma assumes you can read mathematical notation and have some comfort with algebra.

## Tracks

| Track | Problems | Systems that use it |
|-------|----------|---------------------|
| **Probability** | 13 | A/B testing, spam filters, PageRank, diffusion models, load balancers |
| **Linear Algebra** | 13 | Word embeddings, image filters, PCA, recommendation engines, cryptography |
| **Calculus** | 13 | Gradient descent, backpropagation, game physics, Taylor approximations |
| **Discrete Maths** | 13 | Feature combinations, social networks, hashing, RSA, route optimisation |

Four difficulty tiers per track:

- **Foundation** . undergraduate 1-2
- **Core** . final year / competition
- **Advanced** . Putnam/IMO level
- **Research** . frontier-adjacent

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
| Proof Quality | Structure checklist + comprehension + exemplar comparison |
| Application Awareness | Can you connect the maths to a real system? |

Proof assessment is three layers deep. First, a structure checklist (did you start from the right definition, express the key identity, complete the derivation). Then comprehension questions that test whether you understand *why* each step works. Finally, comparison against an exemplar proof to identify gaps in clarity or completeness.

Mastery levels: **Novice** < 40 | **Competent** 40-65 | **Proficient** 65-85 | **Expert** 85+

## Architecture

```
lemma/
  Cargo.toml                 workspace
  crates/
    core/                    types, grading (numerical + strategy + proof), SQLite store
    content/                 TOML loader, cross-ref validation
    server/                  axum API (port 3003)
  content/
    concepts.toml            74 mathematical concepts
    strategies.toml          21 problem-solving strategies
    tracks/                  52 problems across 4 tracks (4 phases each)
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

Problems are authored as TOML files. Each file contains four phases: spark (system scenario), ground (concept), work (faded examples), and solve (independent problems with hints and rubrics).

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

[work]
worked_example = "Step 1: Identify P(A), P(B|A)... Step 2: Total probability..."
guided_prompt = "Same method, different numbers. Compute P(A|defect)..."
guided_answer = 0.4375
guided_tolerance = 0.001

[[solve.sub_problems]]
kind = "numerical"
prompt = "Compute P(spam|flagged)..."
answer = 0.6667
tolerance = 0.001
points = 20

[[solve.sub_problems]]
kind = "proof"
prompt = "Prove Bayes' theorem from the definition..."
exemplar = "Proof. By the definition of conditional probability..."
solution_sketch = "P(A|B) = P(A intersection B)/P(B) = P(B|A)*P(A)/P(B). QED."
points = 20

[[solve.sub_problems.comprehension]]
question = "The proof requires P(A) > 0 as a precondition for all cases."
answer = false
explanation = "P(A) = 0 is handled as a trivial case."
```

## Sources

The curriculum draws structure and inspiration from:

- *The Princeton Companion to Mathematics* (Timothy Gowers)
- *The Art and Craft of Problem Solving* (Paul Zeitz)
- *How to Solve It* (George Polya)
- *Concrete Mathematics* (Graham, Knuth, Patashnik)
- *Proofs from THE BOOK* (Aigner, Ziegler)

Problem databases: AoPS Wiki, Project Euler, Putnam archives, IMO shortlists.

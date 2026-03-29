<script lang="ts">
  import { onMount } from "svelte";
  import {
    listProblems,
    getProblem,
    submitNumerical,
    submitStrategy,
    submitProof,
    submitWork,
    submitComprehension,
    advancePhase,
    revealHint,
    assessProblem,
    type ProblemSummary,
    type ProblemDetail,
    type GradeResponse,
    type ProofGradeResponse,
    type WorkGradeResponse,
    type ComprehensionResponse,
    type HintResponse,
    type AssessmentResponse,
  } from "./lib/api";

  type Screen = "list" | "spark" | "ground" | "work" | "solve" | "review";

  let screen: Screen = $state("list");
  let problems: ProblemSummary[] = $state([]);
  let current: ProblemDetail | null = $state(null);
  let loading = $state(true);
  let subIndex = $state(0);
  let numericalInput = $state("");
  let selectedStrategy = $state("");
  let checkedRubric: boolean[] = $state([]);
  let feedback: string = $state("");
  let feedbackCorrect: boolean = $state(false);
  let solutionSketch: string = $state("");
  let solveResults: { score: number; max: number }[] = $state([]);
  let strategyCorrect = $state(0);
  let strategyTotal = $state(0);
  let showFeedback = $state(false);
  let revealedHints: HintResponse[] = $state([]);
  let hintPenalty = $state(0);
  let selectedTrack = $state("all");
  let assessment: AssessmentResponse | null = $state(null);
  let workInput = $state("");
  let workFeedback: WorkGradeResponse | null = $state(null);
  let proofPhase: "checklist" | "comprehension" | "exemplar" = $state("checklist");
  let comprehensionAnswers: boolean[] = $state([]);
  let comprehensionResults: ComprehensionResponse | null = $state(null);

  $effect(() => {
    function handleKeydown(e: KeyboardEvent) {
      if (e.key === "Escape" && screen !== "list") {
        backToList();
      }
    }
    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });

  const tierColour: Record<string, string> = {
    foundation: "#22863a",
    core: "#b08800",
    advanced: "#d15704",
    research: "#cb2431",
  };

  onMount(async () => {
    problems = await listProblems();
    loading = false;
  });

  async function selectProblem(id: string) {
    loading = true;
    current = await getProblem(id);
    subIndex = 0;
    solveResults = [];
    strategyCorrect = 0;
    strategyTotal = 0;
    feedback = "";
    solutionSketch = "";
    showFeedback = false;
    screen = "spark";
    loading = false;
  }

  function goGround() {
    screen = "ground";
  }

  function goWork() {
    workInput = "";
    workFeedback = null;
    screen = "work";
  }

  function goSolve() {
    subIndex = 0;
    resetSubProblemState();
    screen = "solve";
  }

  function afterGround() {
    if (current?.work) {
      goWork();
    } else {
      goSolve();
    }
  }

  function resetSubProblemState() {
    numericalInput = "";
    selectedStrategy = "";
    feedback = "";
    feedbackCorrect = false;
    solutionSketch = "";
    showFeedback = false;
    revealedHints = [];
    proofPhase = "checklist";
    comprehensionAnswers = [];
    comprehensionResults = null;
    if (current) {
      const sp = current.sub_problems[subIndex];
      if (sp?.kind === "proof") {
        checkedRubric = sp.rubric.map(() => false);
        comprehensionAnswers = sp.comprehension.map(() => false);
      } else {
        checkedRubric = [];
      }
    }
  }

  async function handleRevealHint() {
    if (!current) return;
    const sp = current.sub_problems[subIndex];
    if (revealedHints.length >= sp.hint_count) return;
    const hint = await revealHint(current.id, subIndex, revealedHints.length);
    revealedHints = [...revealedHints, hint];
    hintPenalty += hint.cost;
  }

  async function handleNumericalSubmit() {
    if (!current) return;
    const answer = parseFloat(numericalInput);
    if (isNaN(answer)) return;
    const res: GradeResponse = await submitNumerical(current.id, subIndex, answer);
    feedback = res.feedback;
    feedbackCorrect = res.correct;
    solveResults[subIndex] = { score: res.score, max: res.max_score };
    showFeedback = true;
  }

  async function handleStrategySubmit() {
    if (!current || !selectedStrategy) return;
    const res: GradeResponse = await submitStrategy(current.id, subIndex, selectedStrategy);
    feedback = res.feedback;
    feedbackCorrect = res.correct;
    solveResults[subIndex] = { score: res.score, max: res.max_score };
    strategyTotal++;
    if (res.correct) strategyCorrect++;
    showFeedback = true;
  }

  async function handleProofSubmit() {
    if (!current) return;
    const sp = current.sub_problems[subIndex];
    if (sp.comprehension.length > 0 && proofPhase === "checklist") {
      proofPhase = "comprehension";
      return;
    }
    const res: ProofGradeResponse = await submitProof(current.id, subIndex, checkedRubric);
    feedback = res.feedback;
    feedbackCorrect = res.score > 0;
    solutionSketch = res.solution_sketch;
    solveResults[subIndex] = { score: res.score, max: res.max_score };
    if (sp.exemplar) {
      proofPhase = "exemplar";
    } else {
      showFeedback = true;
    }
  }

  async function handleComprehensionSubmit() {
    if (!current) return;
    const res = await submitComprehension(current.id, subIndex, comprehensionAnswers);
    comprehensionResults = res;
  }

  function finishComprehension() {
    handleProofSubmitFinal();
  }

  async function handleProofSubmitFinal() {
    if (!current) return;
    const sp = current.sub_problems[subIndex];
    const res: ProofGradeResponse = await submitProof(current.id, subIndex, checkedRubric);
    feedback = res.feedback;
    feedbackCorrect = res.score > 0;
    solutionSketch = res.solution_sketch;
    solveResults[subIndex] = { score: res.score, max: res.max_score };
    if (sp.exemplar) {
      proofPhase = "exemplar";
    } else {
      showFeedback = true;
    }
  }

  function finishExemplar() {
    showFeedback = true;
  }

  async function handleWorkSubmit() {
    if (!current) return;
    const answer = parseFloat(workInput);
    if (isNaN(answer)) return;
    workFeedback = await submitWork(current.id, answer);
  }

  function nextSubProblem() {
    if (!current) return;
    if (subIndex < current.sub_problems.length - 1) {
      subIndex++;
      resetSubProblemState();
    } else {
      finishProblem();
    }
  }

  async function finishProblem() {
    if (!current) return;
    const totalScore = solveResults.reduce((sum, r) => sum + (r?.score ?? 0), 0);
    const maxScore = solveResults.reduce((sum, r) => sum + (r?.max ?? 0), 0);
    await advancePhase(current.id, "review", true, totalScore, maxScore, strategyCorrect, strategyTotal);
    assessment = await assessProblem(current.id);
    screen = "review";
  }

  async function backToList() {
    problems = await listProblems();
    current = null;
    screen = "list";
  }
</script>

<main>
  <header>
    <button class="logo-btn" onclick={backToList}>Lemma</button>
    {#if current}
      <span class="breadcrumb">{current.track} / {current.title}</span>
    {/if}
  </header>

  {#if loading}
    <div class="loading">Loading...</div>

  {:else if screen === "list"}
    {@const tracks = [...new Set(problems.map(p => p.track))].sort()}
    {@const filtered = selectedTrack === "all" ? problems : problems.filter(p => p.track === selectedTrack)}
    {@const completed = filtered.filter(p => p.completed).length}
    <section class="problem-list">
      <div class="track-bar">
        <button class="track-pill" class:active={selectedTrack === "all"} onclick={() => selectedTrack = "all"}>
          All ({problems.length})
        </button>
        {#each tracks as t}
          <button class="track-pill" class:active={selectedTrack === t} onclick={() => selectedTrack = t}>
            {t} ({problems.filter(p => p.track === t).length})
          </button>
        {/each}
      </div>
      <div class="progress-summary">
        {completed}/{filtered.length} completed
        <div class="progress-track">
          <div class="progress-fill" style="width:{filtered.length ? (completed/filtered.length*100) : 0}%"></div>
        </div>
      </div>
      <div class="grid">
        {#each filtered as p}
          <button
            class="problem-card"
            class:completed={p.completed}
            onclick={() => selectProblem(p.id)}
          >
            <span class="tier-badge" style="background:{tierColour[p.tier] ?? '#666'}">{p.tier}</span>
            <h3>{p.title}</h3>
            <p class="meta">{p.max_score} pts</p>
            {#if p.completed}
              <p class="score">{p.total_score}/{p.max_score}</p>
            {/if}
          </button>
        {/each}
      </div>
    </section>

  {:else if screen === "spark" && current}
    <section class="phase spark">
      <div class="phase-label">Spark</div>
      <span class="domain-badge">{current.spark.industry_domain.replace(/_/g, " ")}</span>
      <div class="scenario">{current.spark.scenario}</div>
      <div class="question">
        <strong>Before you begin:</strong> {current.spark.connection_question}
      </div>
      <button class="primary" onclick={goGround}>Continue to Ground</button>
    </section>

  {:else if screen === "ground" && current}
    <section class="phase ground">
      <div class="phase-label">Ground</div>
      <div class="ground-layout">
        <div class="concept-panel">
          <h3>Concept</h3>
          <p>{current.ground.concept_brief}</p>
          <h4>Intuition</h4>
          <p class="intuition">{current.ground.intuition}</p>
        </div>
        <div class="definitions-panel">
          <h3>Definitions</h3>
          {#each current.ground.definitions as def}
            <div class="definition">
              <strong>{def.term}</strong>
              <p>{def.statement}</p>
              {#if def.latex}
                <code class="latex">{def.latex}</code>
              {/if}
            </div>
          {/each}
        </div>
      </div>
      {#if current.ground.connections.length > 0}
        <div class="connections">
          <h4>Connections</h4>
          {#each current.ground.connections as conn}
            <p><strong>{conn.target_concept}</strong>: {conn.relationship}</p>
          {/each}
        </div>
      {/if}
      <button class="primary" onclick={afterGround}>
        {current.work ? "Continue to Worked Example" : "Continue to Solve"}
      </button>
    </section>

  {:else if screen === "work" && current && current.work}
    <section class="phase work">
      <div class="phase-label">Work</div>
      <h3>Worked Example</h3>
      <div class="worked-example">{current.work.worked_example}</div>

      <h3>Your Turn</h3>
      <p class="prompt">{current.work.guided_prompt}</p>

      {#if !workFeedback}
        <div class="input-group">
          <input
            type="text"
            bind:value={workInput}
            placeholder="Enter your answer..."
            onkeydown={(e) => e.key === 'Enter' && handleWorkSubmit()}
          />
          <button class="primary" onclick={handleWorkSubmit}>Submit</button>
        </div>
      {:else}
        <div class="feedback" class:correct={workFeedback.correct} class:incorrect={!workFeedback.correct}>
          <pre>{workFeedback.feedback}</pre>
        </div>
        <button class="primary" onclick={goSolve}>Continue to Solve</button>
      {/if}
    </section>

  {:else if screen === "solve" && current}
    {@const sp = current.sub_problems[subIndex]}
    <section class="phase solve">
      <div class="phase-label">Solve</div>
      <div class="progress-bar">
        Problem {subIndex + 1} of {current.sub_problems.length}
        <span class="points">{sp.points} pts</span>
      </div>

      <div class="sub-problem">
        <div class="kind-badge" class:numerical={sp.kind === "numerical"}
          class:strategy={sp.kind === "strategy_id"} class:proof={sp.kind === "proof"}>
          {sp.kind === "strategy_id" ? "strategy" : sp.kind}
        </div>
        <p class="prompt">{sp.prompt}</p>

        {#if !showFeedback}
          {#if sp.hint_count > 0}
            <div class="hint-section">
              {#each revealedHints as hint}
                <div class="hint-card">
                  <span class="hint-cost">-{hint.cost} pts</span>
                  {hint.text}
                </div>
              {/each}
              {#if revealedHints.length < sp.hint_count}
                <button class="hint-btn" onclick={handleRevealHint}>
                  Reveal hint ({revealedHints.length + 1}/{sp.hint_count})
                </button>
              {/if}
            </div>
          {/if}

          {#if sp.kind === "numerical"}
            <div class="input-group">
              <input
                type="text"
                bind:value={numericalInput}
                placeholder="Enter your answer..."
                onkeydown={(e) => e.key === 'Enter' && handleNumericalSubmit()}
              />
              <button class="primary" onclick={handleNumericalSubmit}>Submit</button>
            </div>

          {:else if sp.kind === "strategy_id"}
            <div class="choices">
              {#each sp.choices as choice}
                <label class="choice" class:selected={selectedStrategy === choice}>
                  <input type="radio" bind:group={selectedStrategy} value={choice} />
                  {choice.replace(/-/g, " ")}
                </label>
              {/each}
            </div>
            <button class="primary" onclick={handleStrategySubmit} disabled={!selectedStrategy}>
              Submit
            </button>

          {:else if sp.kind === "proof"}
            {#if proofPhase === "checklist"}
              <p class="rubric-intro">Assess your proof against these structural criteria:</p>
              <div class="rubric">
                {#each sp.rubric as item, i}
                  <label class="rubric-item">
                    <input type="checkbox" bind:checked={checkedRubric[i]} />
                    {item.criterion} <span class="weight">({item.weight} pts)</span>
                  </label>
                {/each}
              </div>
              <button class="primary" onclick={handleProofSubmit}>
                {sp.comprehension.length > 0 ? "Next: Comprehension Check" : "Submit"}
              </button>

            {:else if proofPhase === "comprehension"}
              <h4>Proof Comprehension</h4>
              <p class="rubric-intro">Answer these questions about the proof structure:</p>
              <div class="comprehension-questions">
                {#each sp.comprehension as q, i}
                  <div class="comprehension-item" class:answered={comprehensionResults !== null}>
                    <p>{q.question}</p>
                    <div class="bool-choices">
                      <label class:selected={comprehensionAnswers[i] === true}>
                        <input type="radio" name={`comp-${i}`} value={true}
                          onchange={() => comprehensionAnswers[i] = true} />
                        True
                      </label>
                      <label class:selected={comprehensionAnswers[i] === false}>
                        <input type="radio" name={`comp-${i}`} value={false}
                          onchange={() => comprehensionAnswers[i] = false} />
                        False
                      </label>
                    </div>
                    {#if comprehensionResults}
                      <div class="comp-feedback" class:correct={comprehensionResults.results[i]?.correct} class:incorrect={!comprehensionResults.results[i]?.correct}>
                        {comprehensionResults.results[i]?.correct ? "Correct" : "Incorrect"} &mdash; {comprehensionResults.results[i]?.explanation}
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
              {#if !comprehensionResults}
                <button class="primary" onclick={handleComprehensionSubmit}>Check Answers</button>
              {:else}
                <button class="primary" onclick={finishComprehension}>Continue</button>
              {/if}

            {:else if proofPhase === "exemplar" && sp.exemplar}
              <h4>Compare Your Proof</h4>
              <p class="rubric-intro">Read this exemplar proof and compare it to yours. Note differences in structure, clarity, and completeness.</p>
              <div class="exemplar-proof">
                <pre>{sp.exemplar}</pre>
              </div>
              <button class="primary" onclick={finishExemplar}>Continue</button>
            {/if}
          {/if}

        {:else}
          <div class="feedback" class:correct={feedbackCorrect} class:incorrect={!feedbackCorrect}>
            <pre>{feedback}</pre>
          </div>
          {#if solutionSketch}
            <div class="solution-sketch">
              <h4>Solution Sketch</h4>
              <pre>{solutionSketch}</pre>
            </div>
          {/if}
          <button class="primary" onclick={nextSubProblem}>
            {subIndex < current.sub_problems.length - 1 ? "Next Problem" : "Finish"}
          </button>
        {/if}
      </div>
    </section>

  {:else if screen === "review" && current}
    <section class="phase review">
      <div class="phase-label">Review</div>
      <h2>{current.title}</h2>
      <div class="score-summary">
        <div class="big-score">
          {solveResults.reduce((s, r) => s + (r?.score ?? 0), 0)}
          / {solveResults.reduce((s, r) => s + (r?.max ?? 0), 0)}
        </div>
        <p>points earned</p>
      </div>
      {#if assessment}
        <div class="mastery-badge" class:novice={assessment.mastery === "Novice"}
          class:competent={assessment.mastery === "Competent"}
          class:proficient={assessment.mastery === "Proficient"}
          class:expert={assessment.mastery === "Expert"}>
          {assessment.mastery}
        </div>
        <div class="axes-grid">
          {#each assessment.axes as axis}
            <div class="axis-row">
              <span class="axis-label">{axis.label}</span>
              <div class="axis-bar-track">
                <div class="axis-bar-fill" style="width:{axis.score}%;background:{axis.score >= 70 ? '#22863a' : axis.score >= 40 ? '#b08800' : '#cb2431'}"></div>
              </div>
              <span class="axis-value">{axis.score}</span>
            </div>
          {/each}
        </div>
      {/if}
      {#if strategyTotal > 0}
        <div class="strategy-summary">
          Strategy identification: {strategyCorrect}/{strategyTotal}
        </div>
      {/if}
      <div class="breakdown">
        <h3>Breakdown</h3>
        {#each current.sub_problems as sp, i}
          <div class="breakdown-row">
            <span class="kind-badge small" class:numerical={sp.kind === "numerical"}
              class:strategy={sp.kind === "strategy_id"} class:proof={sp.kind === "proof"}>
              {sp.kind === "strategy_id" ? "strategy" : sp.kind}
            </span>
            <span class="breakdown-score">
              {solveResults[i]?.score ?? 0}/{sp.points}
            </span>
          </div>
        {/each}
      </div>
      <button class="primary" onclick={backToList}>Back to Problems</button>
    </section>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: "Inter", -apple-system, BlinkMacSystemFont, sans-serif;
    background: #fafafa;
    color: #1a1a1a;
  }

  main {
    max-width: 900px;
    margin: 0 auto;
    padding: 1rem;
  }

  header {
    display: flex;
    align-items: baseline;
    gap: 1rem;
    border-bottom: 1px solid #e0e0e0;
    padding-bottom: 0.75rem;
    margin-bottom: 1.5rem;
  }

  .logo-btn {
    all: unset;
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    cursor: pointer;
    letter-spacing: -0.02em;
  }

  .breadcrumb {
    color: #666;
    font-size: 0.875rem;
  }

  .loading {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }

  .problem-card {
    all: unset;
    cursor: pointer;
    display: block;
    background: #fff;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1rem;
    transition: border-color 0.15s;
  }

  .problem-card:hover {
    border-color: #333;
  }

  .problem-card.completed {
    border-left: 3px solid #22863a;
  }

  .problem-card h3 {
    margin: 0.5rem 0 0.25rem;
    font-size: 0.95rem;
  }

  .tier-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 600;
    color: #fff;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .meta {
    margin: 0;
    color: #666;
    font-size: 0.8rem;
  }

  .score {
    margin: 0.25rem 0 0;
    font-weight: 600;
    color: #22863a;
  }

  .phase {
    background: #fff;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .phase-label {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #666;
    margin-bottom: 1rem;
  }

  .domain-badge {
    display: inline-block;
    background: #f0f0f0;
    padding: 2px 10px;
    border-radius: 4px;
    font-size: 0.75rem;
    text-transform: uppercase;
    margin-bottom: 1rem;
  }

  .scenario, .prompt {
    white-space: pre-wrap;
    line-height: 1.6;
    margin-bottom: 1rem;
  }

  .question {
    background: #f8f8f8;
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1.5rem;
    line-height: 1.5;
  }

  .ground-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-bottom: 1rem;
  }

  @media (max-width: 640px) {
    .ground-layout { grid-template-columns: 1fr; }
  }

  .concept-panel h3, .definitions-panel h3 { margin-top: 0; }

  .intuition {
    background: #f8f8f8;
    padding: 0.75rem;
    border-radius: 6px;
    font-style: italic;
  }

  .definition {
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid #f0f0f0;
  }

  .definition p { margin: 0.25rem 0; }

  .latex {
    display: block;
    background: #f0f0f0;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.85rem;
    margin-top: 0.25rem;
  }

  .connections {
    background: #f8f8f8;
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1.5rem;
  }

  .connections h4 { margin-top: 0; }
  .connections p { margin: 0.25rem 0; }

  .progress-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    font-size: 0.85rem;
    color: #666;
  }

  .points {
    font-weight: 600;
    color: #333;
  }

  .kind-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #fff;
    margin-bottom: 0.75rem;
  }

  .kind-badge.numerical { background: #0366d6; }
  .kind-badge.strategy { background: #6f42c1; }
  .kind-badge.proof { background: #d15704; }
  .kind-badge.small { font-size: 0.6rem; padding: 1px 6px; }

  .input-group {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  input[type="text"] {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: 1px solid #ccc;
    border-radius: 6px;
    font-size: 1rem;
    font-family: inherit;
  }

  .choices {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 0.75rem 0;
  }

  .choice {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    cursor: pointer;
    transition: border-color 0.15s;
    text-transform: capitalize;
  }

  .choice:hover { border-color: #6f42c1; }
  .choice.selected { border-color: #6f42c1; background: #f5f0ff; }

  .rubric-intro {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
  }

  .rubric {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .rubric-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    cursor: pointer;
  }

  .weight {
    color: #999;
    font-size: 0.8rem;
  }

  .feedback {
    padding: 1rem;
    border-radius: 6px;
    margin: 1rem 0;
  }

  .feedback.correct {
    background: #f0fff4;
    border: 1px solid #22863a;
  }

  .feedback.incorrect {
    background: #fff5f5;
    border: 1px solid #cb2431;
  }

  .feedback pre {
    margin: 0;
    white-space: pre-wrap;
    font-family: inherit;
  }

  .solution-sketch {
    background: #f8f8f8;
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
  }

  .solution-sketch h4 { margin-top: 0; }
  .solution-sketch pre { margin: 0; white-space: pre-wrap; font-family: inherit; }

  .score-summary {
    text-align: center;
    margin: 2rem 0;
  }

  .big-score {
    font-size: 3rem;
    font-weight: 700;
    letter-spacing: -0.02em;
  }

  .strategy-summary {
    text-align: center;
    color: #666;
    margin-bottom: 2rem;
  }

  .breakdown {
    margin: 1.5rem 0;
  }

  .breakdown-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #f0f0f0;
  }

  .breakdown-score {
    margin-left: auto;
    font-weight: 600;
  }

  button.primary {
    background: #1a1a1a;
    color: #fff;
    border: none;
    padding: 0.6rem 1.5rem;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    font-family: inherit;
  }

  button.primary:hover { background: #333; }
  button.primary:disabled { opacity: 0.5; cursor: not-allowed; }

  h2 { margin-top: 0; }

  .track-bar {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }

  .track-pill {
    all: unset;
    cursor: pointer;
    padding: 0.3rem 0.8rem;
    border: 1px solid #e0e0e0;
    border-radius: 20px;
    font-size: 0.8rem;
    text-transform: capitalize;
    transition: all 0.15s;
  }

  .track-pill:hover { border-color: #6f42c1; }

  .track-pill.active {
    background: #6f42c1;
    color: #fff;
    border-color: #6f42c1;
  }

  .progress-summary {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
    font-size: 0.85rem;
    color: #666;
  }

  .progress-track {
    flex: 1;
    height: 6px;
    background: #e0e0e0;
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: #6f42c1;
    border-radius: 3px;
    transition: width 0.3s;
  }

  .hint-section {
    margin-bottom: 1rem;
  }

  .hint-card {
    background: #fffbe6;
    border: 1px solid #f0e68c;
    border-radius: 6px;
    padding: 0.6rem 0.75rem;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .hint-cost {
    display: inline-block;
    background: #d15704;
    color: #fff;
    padding: 1px 6px;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 600;
    margin-right: 0.5rem;
  }

  .hint-btn {
    all: unset;
    cursor: pointer;
    color: #6f42c1;
    font-size: 0.85rem;
    font-weight: 600;
    padding: 0.3rem 0;
  }

  .hint-btn:hover { text-decoration: underline; }

  .worked-example {
    background: #f0f7ff;
    border: 1px solid #c8ddf0;
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1.5rem;
    white-space: pre-wrap;
    line-height: 1.6;
  }

  .comprehension-questions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .comprehension-item {
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    padding: 0.75rem;
  }

  .comprehension-item p { margin: 0 0 0.5rem; }

  .bool-choices {
    display: flex;
    gap: 1rem;
  }

  .bool-choices label {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.3rem 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    cursor: pointer;
  }

  .bool-choices label.selected {
    border-color: #6f42c1;
    background: #f5f0ff;
  }

  .comp-feedback {
    margin-top: 0.5rem;
    padding: 0.4rem 0.6rem;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .comp-feedback.correct { background: #f0fff4; color: #22863a; }
  .comp-feedback.incorrect { background: #fff5f5; color: #cb2431; }

  .exemplar-proof {
    background: #f8f8f8;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .exemplar-proof pre {
    margin: 0;
    white-space: pre-wrap;
    font-family: inherit;
    line-height: 1.6;
  }

  .mastery-badge {
    display: inline-block;
    padding: 0.4rem 1.2rem;
    border-radius: 20px;
    font-size: 0.9rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0 auto 1.5rem;
    display: block;
    width: fit-content;
  }

  .mastery-badge.novice { background: #fff5f5; color: #cb2431; border: 1px solid #cb2431; }
  .mastery-badge.competent { background: #fffbe6; color: #b08800; border: 1px solid #b08800; }
  .mastery-badge.proficient { background: #f0fff4; color: #22863a; border: 1px solid #22863a; }
  .mastery-badge.expert { background: #f0ebff; color: #6f42c1; border: 1px solid #6f42c1; }

  .axes-grid {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    margin-bottom: 1.5rem;
  }

  .axis-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .axis-label {
    flex: 0 0 180px;
    font-size: 0.85rem;
    color: #333;
    text-align: right;
  }

  .axis-bar-track {
    flex: 1;
    height: 10px;
    background: #e0e0e0;
    border-radius: 5px;
    overflow: hidden;
  }

  .axis-bar-fill {
    height: 100%;
    border-radius: 5px;
    transition: width 0.5s ease;
  }

  .axis-value {
    flex: 0 0 30px;
    font-size: 0.85rem;
    font-weight: 600;
    text-align: right;
  }
</style>

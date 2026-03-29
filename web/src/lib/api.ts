export type ProblemSummary = {
  id: string;
  title: string;
  track: string;
  tier: string;
  max_score: number;
  completed: boolean;
  total_score: number;
};

export type Definition = {
  term: string;
  statement: string;
  latex: string;
};

export type Connection = {
  target_concept: string;
  relationship: string;
};

export type RubricItem = {
  criterion: string;
  weight: number;
};

export type ComprehensionQuestion = {
  question: string;
};

export type SubProblem = {
  index: number;
  kind: "numerical" | "strategy_id" | "proof";
  prompt: string;
  points: number;
  hint_count: number;
  choices: string[];
  rubric: RubricItem[];
  exemplar: string | null;
  comprehension: ComprehensionQuestion[];
};

export type WorkSection = {
  worked_example: string;
  guided_prompt: string;
};

export type ProblemDetail = {
  id: string;
  title: string;
  track: string;
  tier: string;
  strategy_tags: string[];
  concept_tags: string[];
  spark: {
    scenario: string;
    industry_domain: string;
    connection_question: string;
  };
  ground: {
    concept_brief: string;
    definitions: Definition[];
    intuition: string;
    connections: Connection[];
    field_tags: string[];
  };
  work: WorkSection | null;
  sub_problems: SubProblem[];
};

export type GradeResponse = {
  score: number;
  max_score: number;
  feedback: string;
  correct: boolean;
};

export type ProofGradeResponse = {
  score: number;
  max_score: number;
  feedback: string;
  solution_sketch: string;
};

export type ProgressEntry = {
  problem_id: string;
  current_phase: string;
  completed: boolean;
  total_score: number;
  max_score: number;
  strategy_correct: number;
  strategy_total: number;
};

async function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(url, options);
  if (!res.ok) throw new Error(`${url} returned ${res.status}`);
  return res.json() as Promise<T>;
}

export async function listProblems(): Promise<ProblemSummary[]> {
  return fetchJson("/api/problems");
}

export async function getProblem(id: string): Promise<ProblemDetail> {
  return fetchJson(`/api/problem/${encodeURIComponent(id)}`);
}

export async function getProgress(): Promise<ProgressEntry[]> {
  return fetchJson("/api/progress");
}

export async function submitNumerical(
  problemId: string,
  subProblem: number,
  answer: number,
): Promise<GradeResponse> {
  return fetchJson("/api/submit/numerical", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ problem_id: problemId, sub_problem: subProblem, answer }),
  });
}

export async function submitStrategy(
  problemId: string,
  subProblem: number,
  chosenStrategy: string,
): Promise<GradeResponse> {
  return fetchJson("/api/submit/strategy", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      problem_id: problemId,
      sub_problem: subProblem,
      chosen_strategy: chosenStrategy,
    }),
  });
}

export async function submitProof(
  problemId: string,
  subProblem: number,
  checkedItems: boolean[],
): Promise<ProofGradeResponse> {
  return fetchJson("/api/submit/proof", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      problem_id: problemId,
      sub_problem: subProblem,
      checked_items: checkedItems,
    }),
  });
}

export type WorkGradeResponse = {
  correct: boolean;
  feedback: string;
};

export type ComprehensionResult = {
  correct: boolean;
  explanation: string;
};

export type ComprehensionResponse = {
  results: ComprehensionResult[];
  all_correct: boolean;
};

export type HintResponse = {
  text: string;
  cost: number;
};

export type AxisScore = {
  axis: string;
  label: string;
  score: number;
};

export type AssessmentResponse = {
  axes: AxisScore[];
  mastery: string;
  total_score: number;
  max_score: number;
};

export async function assessProblem(id: string): Promise<AssessmentResponse> {
  return fetchJson(`/api/assess/${encodeURIComponent(id)}`);
}

export async function revealHint(
  problemId: string,
  subProblem: number,
  hintIndex: number,
): Promise<HintResponse> {
  return fetchJson("/api/hint", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      problem_id: problemId,
      sub_problem: subProblem,
      hint_index: hintIndex,
    }),
  });
}

export async function submitWork(
  problemId: string,
  answer: number,
): Promise<WorkGradeResponse> {
  return fetchJson("/api/submit/work", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ problem_id: problemId, answer }),
  });
}

export async function submitComprehension(
  problemId: string,
  subProblem: number,
  answers: boolean[],
): Promise<ComprehensionResponse> {
  return fetchJson("/api/submit/comprehension", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      problem_id: problemId,
      sub_problem: subProblem,
      answers,
    }),
  });
}

export async function advancePhase(
  problemId: string,
  phase: string,
  completed: boolean,
  totalScore: number,
  maxScore: number,
  strategyCorrect: number,
  strategyTotal: number,
): Promise<void> {
  await fetchJson("/api/advance-phase", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      problem_id: problemId,
      phase,
      completed,
      total_score: totalScore,
      max_score: maxScore,
      strategy_correct: strategyCorrect,
      strategy_total: strategyTotal,
    }),
  });
}

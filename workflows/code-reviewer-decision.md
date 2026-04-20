# Code Reviewer Decision

## Purpose

Review the drafted code review scope and findings, record the durable review decision, and decide whether the implementation is approved, conditionally acceptable, or blocked pending remediation.

## When To Use

- A code review scope artifact and review findings artifact already exist.
- Downstream roles need an explicit independent review posture rather than only a findings list.
- The review package includes mixed findings, evidence gaps, or upstream routing questions that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md)
  - an instantiated copy of [`artifacts/review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- Expected supporting context:
  - the corresponding implementation package artifacts
- Optional:
  - upstream handoffs, review notes, run outputs, screenshots, logs, or specification references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md) in the target project workspace
- Secondary outputs: explicit approval posture, grouped blocking and non-blocking findings, remediation conditions, upstream routing notes, and downstream watchouts

## Skills And Tools

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing findings and evidence before writing the decision.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the logic is present but not yet expressed cleanly for downstream use.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when decision posture depends on requirement or acceptance-criteria conformance.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when decision posture depends materially on specification-sensitive drift.

## Sequence

1. Read the code review scope and review findings together with the implementation package and any relevant upstream handoffs.
2. If findings or evidence references still need synthesis before the decision can be stated honestly, use `research-documentation` first.
3. Instantiate [`artifacts/review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md) into the project workspace if a working copy does not already exist.
4. Populate the review decision artifact with decision scope, reviewed revision or package version when relevant, overall assessment, explicit review status, approval limits, blocking and non-blocking findings, required remediation, condition owners, residual risks, downstream watchouts, and re-review triggers.
5. Run [`review-decision.check.md`](D:/Projects/orpheum/checks/review-decision.check.md), [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md), and [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).

## Decision Points

- If the change is blocked, state the blocking findings directly rather than hiding them in general caution language.
- If the change is conditionally acceptable, make the conditions and owners explicit rather than leaving them implied.
- If the issue is really missing verification or weak evidence rather than a plain code defect, preserve that distinction in the decision.
- If the review starts drifting into merge approval, release approval, or final QA language, remove that drift and keep the artifact at code-review decision level.
- If the decision only applies to a particular revision, narrow scope, or evidence window, state that explicitly rather than implying future changes are already covered.

## Validation

- [`review-decision.check.md`](D:/Projects/orpheum/checks/review-decision.check.md) passes.
- [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md) passes.
- [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md) passes.
- The package is ready to feed [`code-reviewer-handoff.md`](D:/Projects/orpheum/workflows/code-reviewer-handoff.md).

## Failure Handling

- Stop and ask for clarification if the decision cannot be made honestly from the available findings package.
- If the decision check fails, rework the decision instead of asking downstream roles to infer the real posture.
- If traceability or boundary checks fail, route remediation to the earliest artifact, evidence source, or upstream role decision that introduced the defect.

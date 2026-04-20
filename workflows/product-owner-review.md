# Product Owner Review

## Purpose

Review the drafted product package, record the durable product decision posture, and decide whether the package is ready, conditional, or blocked for downstream solutioning or planning.

## When To Use

- Product direction and backlog prioritization artifacts already exist.
- Downstream roles need an explicit product posture rather than only direction and ordering notes.
- The product package includes mixed evidence, unresolved tradeoffs, or conditional decisions that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md)
  - an instantiated copy of [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md)
- Expected supporting context:
  - the corresponding validated discovery artifacts and any relevant feedback, release, or specification context
- Optional:
  - approval notes, KPI reviews, roadmap notes, or stakeholder meeting notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md) in the target project workspace
- Secondary outputs: explicit product posture, grouped risks and tradeoffs, conditions, and downstream watchouts

## Skills And Tools

- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md) as the default direct-support skill for turning product direction and prioritization into an explicit product posture with tradeoff clarity, acceptance-guardrail discipline, and downstream-readiness limits.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) as the default grounding skill for testing whether the product package is sufficiently supported by validated needs, commitments, and acceptance-sensitive constraints.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing approvals, evidence, and delivery learnings before writing the decision.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when review inputs include rough decision notes or stakeholder meetings that need normalization first.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when the review reveals a material gap in behavioral definition that should be refined rather than improvised.

## Sequence

1. Read the product direction and backlog prioritization artifacts together with the validated discovery inputs and any material feedback, release, or specification context.
2. If tradeoffs, approvals, or evidence references still need synthesis before the decision can be stated honestly, use `research-documentation` first.
3. Instantiate [`artifacts/product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md) into the project workspace if a working copy does not already exist.
4. Use `product-priority-framing` to populate the product decision review artifact with review scope, reviewed inputs, overall assessment, explicit decision status, any distinction between product-priority readiness and implementation or release commitment, decision owner, key risks and tradeoffs, required follow-up, follow-up owners, revisit triggers, and recommended next step.
5. Run [`product-decision-review.check.md`](D:/Projects/orpheum/checks/product-decision-review.check.md), [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md), and [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).

## Decision Points

- If the package is blocked, state the blocking condition directly rather than softening it into roadmap optimism.
- If the package is conditional, make the condition and owner explicit rather than leaving it implied in backlog ordering.
- If the issue is really unresolved discovery, release evidence, or specification clarity rather than product packaging, preserve that distinction in the review.
- If the decision only applies to a particular horizon, segment, or evidence window, state that explicitly rather than implying broader product permission.
- If the package is ready for downstream shaping but not yet committed for implementation, delivery scheduling, or release, say so explicitly rather than allowing "ready" to carry both meanings at once.

## Validation

- [`product-decision-review.check.md`](D:/Projects/orpheum/checks/product-decision-review.check.md) passes.
- [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md) passes.
- [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md) passes.
- The package is ready to feed [`product-owner-handoff.md`](D:/Projects/orpheum/workflows/product-owner-handoff.md).

## Failure Handling

- Stop and ask for clarification if the product posture cannot be made honestly from the available package.
- If the review check fails, rework the decision instead of asking downstream roles to infer the real posture.
- If traceability or boundary checks fail, route remediation to the earliest artifact, evidence source, or role decision that introduced the defect.

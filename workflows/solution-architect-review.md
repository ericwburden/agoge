# Solution Architect Review

## Purpose

Review drafted architecture artifacts, record explicit findings and readiness, and decide whether the architecture is ready to be packaged for downstream use.

## When To Use

- The first architecture design pass is complete.
- Architecture direction needs a durable review record before downstream handoff.
- Human approval, architectural confidence, or remediation routing needs to be explicit rather than implied.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md)
  - an instantiated copy of [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md)
- Optional:
  - relevant upstream BA artifacts
  - relevant Product Owner artifacts when product posture materially shaped the design
  - architecture workshop notes or technical design notes
  - supporting technical references or comparison material

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md) in the target project workspace
- Secondary outputs: explicit readiness status, key findings, required remediation, interface or contract concerns, and review-based next steps
- Secondary outputs: explicit readiness status, decision ownership, condition ownership, key findings, required remediation, interface or contract concerns, and review-based next steps

## Skills And Tools

- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md) as the default path for evaluating architecture readiness and producing the review artifact.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when architecture findings or rationale are still embedded in workshop notes or transcripts.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when local architectural context or technical evidence needs synthesis before the review can be stated clearly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external standards, patterns, or comparison material materially affect the review judgment.

## Sequence

1. Read the solution architecture and architecture decisions artifacts together, using upstream BA and Product Owner artifacts as needed to judge traceability honestly.
2. If workshop notes or design-session transcripts exist, normalize them with `meeting-notes-and-actions` before drafting the review.
3. If local technical context or external comparison material still needs synthesis, use `research-documentation` or `content-research-writer` first.
4. Instantiate [`artifacts/architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md) into the project workspace if a working copy does not already exist.
5. Use `architecture-review` to populate the review artifact with review scope, reviewed inputs, overall assessment, readiness status, decision ownership, key findings, interface or contract observations, unresolved risks, remediation routing, condition ownership when needed, and the recommended next step.
6. Run [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md), [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md), [`architecture-review.check.md`](D:/Projects/agoge/checks/architecture-review.check.md), [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md), and [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md).

## Decision Points

- If the architecture is not ready, keep the work open and route remediation to the earliest artifact that introduced the defect.
- If the review depends on unstated assumptions, fail the review rather than converting uncertainty into approval.
- If interface seams or contract assumptions remain unclear, block handoff until they are explicit enough for downstream roles to work from.

## Validation

- [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md) passes.
- [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md) passes.
- [`architecture-review.check.md`](D:/Projects/agoge/checks/architecture-review.check.md) passes.
- [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) passes.
- [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md) passes.
- The instantiated review output is ready to feed [`solution-architect-handoff.md`](D:/Projects/agoge/workflows/solution-architect-handoff.md).

## Failure Handling

- Stop and ask for clarification if readiness cannot be judged honestly from the available evidence.
- Do not package architecture downstream while review status is blocked or materially conditional.
- If review, traceability, or boundary checks fail, route remediation to the earliest architecture or BA artifact that introduced the defect.

## Notes

This workflow creates the durable review record for the architecture package. It sits between architecture design and downstream handoff.

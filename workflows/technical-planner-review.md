# Technical Planner Review

## Purpose

Review the drafted implementation planning package, record findings and remediation decisions, and determine whether the plan is ready to move into downstream handoff.

## When To Use

- An implementation strategy and sequencing plan both exist and need a readiness review.
- A planning package has been expanded and should be checked for coherence drift.
- A downstream implementation or verification role needs confidence that the plan is usable without hidden conventions.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md)
  - an instantiated copy of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md)
- Expected supporting context:
  - the corresponding instantiated architecture handoff and requirements handoff artifacts
- Optional: supporting notes, dependency references, rollout constraints, prior review findings, and relevant security/compliance artifacts when those constraints materially shaped the plan

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md) in the target working location
- Secondary outputs: explicit findings, remediation routing, readiness status, and identified execution risks

## Skills And Tools

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) as the preferred default path for tightening planning structure when the review exposes weak slice framing, unsupported sequencing, or missing downstream clarity.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when findings still depend on synthesizing multiple local sources or prior notes before the review can be completed honestly.

## Sequence

1. Read the instantiated implementation strategy and sequencing artifact together with the architecture and requirements handoffs, using relevant security/compliance artifacts as needed to judge planning traceability honestly.
2. If local context still needs synthesis before the review can be written clearly, use `research-documentation` first.
3. Instantiate [`artifacts/implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md) into the working location if a working copy does not already exist.
4. Review the package for coherence, readiness, missing support, hidden dependencies, unsupported assumptions, sequencing weaknesses, and role drift.
5. Record findings, remediation decisions, readiness, remaining risks, and next steps in the instantiated review artifact.
6. Run [`implementation-plan-review.check.md`](D:/Projects/agoge/checks/implementation-plan-review.check.md), [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md), and [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md).
7. If the package is ready or conditionally ready with explicit limits, route it to [`technical-planner-handoff.md`](D:/Projects/agoge/workflows/technical-planner-handoff.md) for downstream packaging.

## Decision Points

- If the package is missing required planning structure, mark it not ready rather than hand-waving the gap.
- If failures all trace back to the implementation strategy or sequencing artifact, route remediation there instead of patching the review artifact.
- If the review reveals an upstream architecture or requirement gap, route remediation upstream rather than silently correcting it in the plan.
- If the review reveals that planning depends on unresolved security/compliance posture, route remediation back to the Security / Compliance Specialist chain rather than silently normalizing the gap inside the plan.

## Validation

- [`implementation-plan-review.check.md`](D:/Projects/agoge/checks/implementation-plan-review.check.md) passes.
- [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) passes.
- [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md) passes.
- The package's readiness state is explicit, defensible, and usable as the basis for implementation handoff.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly from the available artifacts.
- If the review check fails, rework the review and use `research-documentation` if findings still depend on unsynthesized local context.
- If traceability or boundary checks fail, route remediation to the earliest artifact or design decision that introduced the defect.

## Notes

This workflow is the final substantive review before downstream implementation handoff for the Technical Planner package.

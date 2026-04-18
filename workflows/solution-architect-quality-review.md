# Solution Architect Quality Review

## Purpose

Apply the Solution Architect check chain to instantiated architecture artifacts, identify failures by artifact and defect type, and route remediation before downstream planning, implementation, or verification work begins.

## When To Use

- The architecture handoff has been drafted and the full Solution Architect artifact chain exists.
- A downstream technical consumer needs final definition-of-done confidence that the full architecture chain is self-sufficient and within role boundaries.
- A check has failed and the work needs a standard rework path instead of ad hoc editing.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md)
- Expected supporting context:
  - the corresponding instantiated solution architecture, architecture decisions, and architecture review artifacts
- Optional: supporting notes, BA artifacts, technical references, or prior check results

## Outputs

- Primary output: a pass/fail review result for each artifact in scope and each required cross-cutting check
- Secondary outputs: explicit remediation routing, identified defect types, rework priorities, and any blocked readiness status that should prevent downstream handoff

## Skills And Tools

- Use [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md) when the review stage or chain-level assessment needs stronger findings, clearer readiness language, or better remediation routing.
- Use [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) when failures trace back to weak architecture structure, unclear boundaries, or unsupported decisions.
- Use [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md) when the architecture handoff needs rework.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when missing evidence or contradictory local context prevents a clean pass.
- Use [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when architecture evidence is still embedded in workshop notes or transcripts.

## Sequence

1. Confirm that the architecture handoff exists and identify the supporting Solution Architect artifacts that feed it.
2. Run the primary checks for the supporting architecture artifacts and the handoff artifact.
3. Run [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) across the full chain.
4. Run [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md) across all artifacts in scope.
5. Record failures by artifact and defect type: missing architectural drivers, weak boundaries, unsupported assumptions, hidden tradeoffs, broken traceability, weak readiness ownership, missing specification relationship, unstable behavioral constraints, or role drift.
6. Route remediation to the appropriate earlier Solution Architect or BA artifact rather than patching the latest artifact in isolation.
7. Re-run the failed checks until the full chain passes or until remaining gaps are explicitly recorded as unresolved and the work is intentionally held open.

## Required Check Set

- Solution architecture review:
  - [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md)
  - [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) when supporting artifacts exist
  - [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)
- Architecture decisions review:
  - [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md)
  - [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
  - [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)
- Architecture review:
  - [`architecture-review.check.md`](D:/Projects/agoge/checks/architecture-review.check.md)
  - [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
  - [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)
- Architecture handoff review:
  - [`architecture-handoff.check.md`](D:/Projects/agoge/checks/architecture-handoff.check.md)
  - [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
  - [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)

## Validation

- Every artifact in scope has an explicit pass/fail result.
- Cross-cutting failures are tied to the earliest artifact that introduced the defect.
- Remediation is routed to the correct earlier artifact or workflow stage.
- No artifact is treated as downstream-ready while any required check is still failing.

## Failure Handling

- If evidence is insufficient to apply a check honestly, fail the check and identify the missing evidence rather than guessing.
- If the same failure appears in multiple downstream artifacts, rework the earliest source artifact instead of patching each symptom separately.
- If the architecture chain passes primary checks but fails traceability or boundary checks, treat the chain as blocked until the cross-cutting defect is resolved.

## Notes

This workflow is the final definition-of-done audit after handoff exists. It is a quality gate and rework loop, not a second substantive architecture review stage.

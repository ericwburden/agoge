---
id: technical-planner-quality-review
kind: workflow
title: Technical Planner Quality Review
version: 1
summary: Apply the Technical Planner check chain to instantiated planning artifacts,
  identify failures by artifact and defect type, and route remediation before downstream
  implementation or verification work begins.
role: technical-planner
inputs:
- implementation-handoff
- architecture-handoff
- requirements-handoff
outputs: []
skills:
- spec-to-implementation
- handoff-packaging
- research-documentation
- meeting-notes-and-actions
checks:
- planning-traceability
- technical-planner-boundary
- implementation-strategy
- sequencing-and-dependencies
- implementation-plan-review
- implementation-handoff
handoff_to: []
---

# Technical Planner Quality Review

## Purpose

Apply the Technical Planner check chain to instantiated planning artifacts, identify failures by artifact and defect type, and route remediation before downstream implementation or verification work begins.

## When To Use

- The implementation handoff has been drafted and the full Technical Planner artifact chain exists.
- A downstream technical consumer needs final definition-of-done confidence that the full planning chain is self-sufficient and within role boundaries.
- A check has failed and the work needs a standard rework path instead of ad hoc editing.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)
  - the corresponding instantiated implementation strategy, sequencing and dependencies, and implementation plan review artifacts
  - the upstream instantiated [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md) and [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) artifacts needed to apply [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) honestly
- Expected supporting context:
- Optional: additional upstream BA or architecture artifacts, supporting notes, dependency references, or prior check results

## Outputs

- Primary output: a pass/fail review result for each artifact in scope and each required cross-cutting check
- Secondary outputs: explicit remediation routing, identified defect types, rework priorities, and any blocked readiness status that should prevent downstream handoff

## Skills And Tools

- Use [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when failures trace back to weak planning structure, unclear sequencing, or unsupported slice strategy.
- Use [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the implementation handoff needs rework.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when missing evidence or contradictory local context prevents a clean pass.
- Use [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when planning evidence is still embedded in workshop notes or transcripts.

## Sequence

1. Confirm that the implementation handoff exists and identify the supporting Technical Planner artifacts that feed it.
2. Run the primary checks for the supporting planning artifacts and the handoff artifact.
3. Run [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) across the full chain.
4. Run [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md) across all artifacts in scope.
5. Record failures by artifact and defect type: weak slice strategy, missing slice-level traceability, hidden dependencies, missing dependency ownership, unstable critical path assumptions, broken traceability, missing readiness ownership, poor specification relationship, rollout blind spots, or role drift.
6. Route remediation to the appropriate earlier Technical Planner, architecture, or BA artifact rather than patching the latest artifact in isolation.
7. Re-run the failed checks until the full chain passes or until remaining gaps are explicitly recorded as unresolved and the work is intentionally held open.

## Required Check Set

- Implementation strategy review:
  - [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md)
  - [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
  - [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)
- Sequencing and dependencies review:
  - [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md)
  - [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
  - [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)
- Implementation plan review:
  - [`implementation-plan-review.check.md`](D:/Projects/agoge/checks/implementation-plan-review.check.md)
  - [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
  - [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)
- Implementation handoff review:
  - [`implementation-handoff.check.md`](D:/Projects/agoge/checks/implementation-handoff.check.md)
  - [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
  - [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

## Validation

- Every artifact in scope has an explicit pass/fail result.
- Cross-cutting failures are tied to the earliest artifact that introduced the defect.
- Remediation is routed to the correct earlier artifact or workflow stage.
- No artifact is treated as downstream-ready while any required check is still failing.

## Failure Handling

- If evidence is insufficient to apply a check honestly, fail the check and identify the missing evidence rather than guessing.
- If the same failure appears in multiple downstream artifacts, rework the earliest source artifact instead of patching each symptom separately.
- If the planning chain passes primary checks but fails traceability or boundary checks, treat the chain as blocked until the cross-cutting defect is resolved.

## Notes

This workflow is the final definition-of-done audit after handoff exists. It is a quality gate and rework loop, not a second substantive planning review stage.

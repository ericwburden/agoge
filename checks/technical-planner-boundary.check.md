---
id: technical-planner-boundary
kind: check
title: Technical Planner Boundary Check
version: 1
summary: Validate that Technical Planner outputs stay inside planning role boundaries
  and do not drift into redoing business discovery, re-architecting the solution,
  delivery administration, or detailed implementation ownership.
mode: presence
severity: error
applies_to: []
---

# Technical Planner Boundary Check

## Purpose

Validate that Technical Planner outputs stay inside planning role boundaries and do not drift into redoing business discovery, re-architecting the solution, delivery administration, or detailed implementation ownership.

## Applies To

- All instantiated Technical Planner artifacts and Technical Planner workflows.
- Use whenever reviewing whether planning output stayed within the intended role.
- Do not use as a substitute for completeness or traceability checks.

## Criteria

- The output does not redefine business objectives or requirements without routing those issues upstream.
- The output does not silently re-architect the solution when the issue belongs in architecture.
- The output does not collapse into sprint administration, staffing management, or ongoing delivery-status ownership.
- The output does not turn into detailed implementation design or production code ownership.
- Planning assumptions are explicit rather than treated as confirmed facts.
- The work remains focused on execution structure, sequencing, dependencies, readiness, and downstream technical framing.
- Verification and rollout touchpoints are identified when relevant without drifting into final QA or release authority.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably Technical Planner work rather than a disguised BA artifact, architecture revision, project-management board, or implementation design.

## Evidence Required

- The Technical Planner artifact or workflow output being reviewed.
- The [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role definition when needed for role-boundary interpretation.
- Relevant upstream BA or architecture artifacts when needed to identify whether drift originated upstream or inside the plan.

If the output's role identity is ambiguous, fail the check and identify the drift explicitly.

## Supporting Skills

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when role drift began in the implementation strategy or sequencing structure.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the drift appears in downstream handoff packaging.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the planning drift is caused by unsynthesized local context or contradictory references.

## Failure Response

- Rework the output to remove role drift before treating it as a valid planning artifact.
- Route business or architecture ambiguity back to the Business Analyst or Solution Architect chain rather than leaving it embedded in planning output.

## Notes

This is the second cross-cutting Technical Planner check. It protects the planning role from absorbing too many adjacent jobs.

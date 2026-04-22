---
id: implementation-engineer-boundary
kind: check
title: Implementation Engineer Boundary Check
version: 1
summary: Validate that Implementation Engineer outputs stay inside implementation
  role boundaries and do not drift into redoing discovery, re-architecting, re-planning,
  formal QA authority, or release operations.
mode: presence
severity: error
applies_to: []
---

# Implementation Engineer Boundary Check

## Purpose

Validate that Implementation Engineer outputs stay inside implementation role boundaries and do not drift into redoing discovery, re-architecting, re-planning, formal QA authority, or release operations.

## Applies To

- All instantiated Implementation Engineer artifacts and Implementation Engineer workflows.
- Use whenever reviewing whether implementation output stayed within the intended role.
- Do not use as a substitute for completeness, evidence, or traceability checks.

## Criteria

- The output does not redefine business objectives or requirements without routing the issue upstream.
- The output does not silently re-architect the solution when the issue belongs in architecture.
- The output does not silently replace the reviewed implementation strategy with a new plan when the issue belongs in Technical Planner work.
- The output does not collapse into independent code review, verification authority, or release coordination.
- The work remains focused on implementing the approved slice, recording evidence, surfacing deviations, and preparing downstream-ready implementation context.
- Local validation evidence is explicit without being overstated as full downstream verification.
- Plan deviations, blockers, and evidence limits are explicit rather than hidden inside a success narrative.
- Human control points and trust-boundary-sensitive concerns are identified when relevant without drifting into governance or release ownership.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably Implementation Engineer work rather than a disguised BA artifact, architecture revision, replanning exercise, QA verdict, or release plan.

## Evidence Required

- The Implementation Engineer artifact or workflow output being reviewed.
- The [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role definition when needed for role-boundary interpretation.
- Relevant upstream requirements, architecture, or planning artifacts when needed to identify whether drift originated upstream or inside implementation output.

If the output's role identity is ambiguous, fail the check and identify the drift explicitly.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when drift is caused by unsynthesized local context or contradictory references.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when drift appears in downstream packaging.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when drift began because missing behavioral definition should have been routed upstream.

## Failure Response

- Rework the output to remove role drift before treating it as a valid implementation artifact.
- Route requirement, architecture, planning, or specification ambiguity back to the correct upstream role rather than leaving it embedded in implementation output.

## Notes

This is the second cross-cutting Implementation Engineer check. It protects the implementation role from absorbing adjacent jobs and hiding that drift in code changes.

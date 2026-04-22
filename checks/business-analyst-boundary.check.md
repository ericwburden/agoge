---
id: business-analyst-boundary
kind: check
title: Business Analyst Boundary Check
version: 1
summary: Validate that Business Analyst outputs stay inside BA role boundaries and
  do not drift into solution ownership, delivery management, technical design, or
  silent invention of business facts.
mode: presence
severity: error
applies_to: []
---

# Business Analyst Boundary Check

## Purpose

Validate that Business Analyst outputs stay inside BA role boundaries and do not drift into solution ownership, delivery management, technical design, or silent invention of business facts.

## Applies To

- All instantiated Business Analyst artifacts and Business Analyst workflows.
- Use whenever reviewing whether BA output stayed within the intended role.
- Do not use as a substitute for artifact completeness checks.

## Criteria

- The output does not drift into detailed technical design.
- Business rules and requirements are not silently invented without stakeholder confirmation or supporting evidence.
- Proposed solutions remain separate from business needs unless explicitly validated.
- Delivery-management, task decomposition, or implementation planning content is not presented as BA output.
- AI oversight concerns are identified when relevant without drifting into full lifecycle governance ownership.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably Business Analyst work rather than a disguised product, architecture, delivery, or implementation artifact.

## Evidence Required

- The BA artifact or workflow output being reviewed.
- The Business Analyst role definition in [`roles/business-analyst.md`](D:/Projects/agoge/roles/business-analyst.md) when needed for role-boundary interpretation.

If the output’s role identity is ambiguous, fail the check and identify the drift explicitly.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when requirement content is drifting into unsupported solutioning or unverified scope.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when a downstream handoff is mixing BA output with implementation-planning content.

## Failure Response

- Rework the output to remove role drift before treating it as a valid BA artifact.
- Route downstream planning, design, or delivery concerns into the appropriate role or workflow rather than leaving them embedded in BA output.

## Notes

This is the second cross-cutting BA check. It prevents quality erosion caused by BA artifacts trying to do too many adjacent jobs at once.

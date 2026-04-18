# Solution Architect Boundary Check

## Purpose

Validate that Solution Architect outputs stay inside architecture role boundaries and do not drift into redoing business discovery, delivery management, detailed implementation, or silent requirement invention.

## Applies To

- All instantiated Solution Architect artifacts and Solution Architect workflows.
- Use whenever reviewing whether architecture output stayed within the intended role.
- Do not use as a substitute for completeness or traceability checks.

## Criteria

- The output does not redefine business objectives or requirements without routing those issues upstream.
- The output does not collapse into implementation tasking, sprint planning, or delivery management.
- Architectural assumptions are explicit rather than treated as confirmed requirements.
- The work remains focused on system shape, decisions, constraints, and downstream technical framing.
- AI trust boundaries or control points are identified when relevant without drifting into full governance ownership.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably Solution Architect work rather than a disguised BA artifact, implementation plan, or delivery plan.

## Evidence Required

- The Solution Architect artifact or workflow output being reviewed.
- The [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role definition when needed for role-boundary interpretation.
- Relevant upstream BA artifacts when needed to identify whether requirement drift originated upstream or inside the architecture.

If the output's role identity is ambiguous, fail the check and identify the drift explicitly.

## Supporting Skills

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) when role drift began in the architecture shape or decision framing.
- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md) when the drift is being surfaced or reinforced during review.
- [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md) when the drift appears in downstream handoff packaging.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the architecture drift is caused by unsynthesized local context or technical references.

## Failure Response

- Rework the output to remove role drift before treating it as a valid architecture artifact.
- Route requirement ambiguity or business-scope questions back to the Business Analyst chain rather than leaving them embedded in architecture output.

## Notes

This is the second cross-cutting Solution Architect check. It protects the architecture role from absorbing too many adjacent jobs.

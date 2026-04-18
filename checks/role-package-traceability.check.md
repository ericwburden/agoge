# Role Package Traceability Check

## Purpose

Validate that the role package preserves traceability from the role definition to the support-system design to the package review and final adoption handoff.

## Applies To

- instantiated copies of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md)
- instantiated copies of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md)
- instantiated copies of [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md)
- instantiated copies of [`artifacts/role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md)

Use after more than one role-package artifact exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- The support-system design clearly derives from the role's job-to-be-done, outputs, and boundaries.
- The role's Allium relationship remains consistent from the role definition through the support system and final handoff when it is relevant.
- The package review reflects the actual role definition and support-system design rather than a different implicit role shape.
- The role-package handoff reflects the reviewed package honestly rather than inventing missing support, readiness, or adoption expectations.
- Missing workflows, checks, or skills are surfaced as gaps rather than silently skipped.
- Unsupported package elements are not presented as if they were required by the role definition.
- Contradictions or broken links across the package are explicit.

## Scoring Or Outcome

Pass/fail.

The package passes only if a reviewer can follow the logic from role definition to support system to review outcome without relying on hidden assumptions.

## Evidence Required

- The full role-package artifact chain.
- Any supporting notes needed to understand ambiguous derivations or design decisions.

If one or more links cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md) when the role-package chain breaks at the support-system layer and needs to be realigned to the role definition.
- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) when the review layer no longer reflects the actual role definition or support-system design.
- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md) when the adoption handoff overstates readiness, misses limitations, or breaks the traceability chain.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the role-package chain still needs synthesis or comparison across multiple local sources before the break can be resolved clearly.

## Failure Response

- Rework the earliest artifact that introduced the missing or broken connection.
- Do not treat the role package as coherent while support elements remain untraceable to the role definition.

## Notes

This is the core cross-cutting quality check for role-package design.

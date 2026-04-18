# Role Package Handoff Check

## Purpose

Validate that a role-package handoff is downstream-usable without forcing adopters to reconstruct the package from earlier artifacts or chat-only context.

## Applies To

- instantiated copies of [`artifacts/role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md)
- Use after the role definition, support system, and review outcome are stable enough to package for adoption.
- Do not use as a substitute for the readiness review itself.

## Criteria

- The handoff identifies the role in scope and summarizes what the role is meant to do.
- The reviewed inputs and review outcome are explicit.
- Adoption guidance is clear enough that a downstream consumer knows how to start.
- Recommended consumers are identified.
- Current limitations and required conventions are visible.
- Allium expectations are explicit enough that adopters know how the role should interact with specifications.
- Next adoption steps are concrete.
- The handoff does not invent missing readiness, support, or adoption confidence that was not established in the reviewed package.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream adopter could use it as the entry point to the role package without reopening every earlier artifact to figure out what matters.

## Evidence Required

- The role-package handoff artifact.
- The role definition brief, role support system, and role package review artifacts it refers to.
- Any supporting notes cited as part of adoption guidance.

If the handoff depends on missing or unstated package context, fail the check and identify the missing adoption-critical sections.

## Supporting Skills

- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md) when the handoff needs to be reworked into a clearer adoption-facing package.
- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) when the handoff overstates readiness or depends on an unclear review outcome.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the handoff still depends on synthesizing multiple local notes before adoption guidance can be stated clearly.

## Failure Response

- Rework the handoff until the package can be adopted without hidden context.
- Route unresolved readiness questions back to the review artifact rather than compensating with stronger packaging language.

## Notes

This check protects the downstream usability of the Role Builder package after review has passed.

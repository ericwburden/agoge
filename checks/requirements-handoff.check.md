# Requirements Handoff Check

## Purpose

Validate that the requirements handoff artifact is complete enough for downstream consumers to continue without rediscovering the business context.

## Applies To

- instantiated copies of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md)
- Use when BA work is being handed to product, architecture, delivery, or implementation roles.
- Do not use to validate implementation plans or task decomposition.

## Criteria

- The business problem and desired outcome are summarized clearly.
- Verified requirements are packaged for downstream use.
- Risks, dependencies, assumptions, and open questions are present.
- Downstream consumers are identified.
- Next decision points are included.
- The artifact does not collapse into implementation planning, task breakdown, or delivery management.
- Human oversight and AI notes are separated when relevant.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can continue the work without reconstructing the business problem from earlier discovery materials.

## Evidence Required

- The requirements handoff artifact.
- The requirements specification artifact and supporting BA artifacts when needed to confirm completeness.

If the handoff depends on implicit context not captured in the artifact, fail the check.

## Supporting Skills

- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the handoff needs to be repackaged for downstream consumers with stronger traceability, risks, and next decision points.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when the handoff is weak because the underlying requirements are still not cleanly verified.

## Failure Response

- Rework the handoff artifact before passing it downstream.
- Keep BA discovery open until missing risks, traceability, or unresolved questions are made explicit.

## Notes

This check protects the BA-to-downstream boundary by ensuring the handoff is self-sufficient without becoming an implementation plan.

# Requirements Specification Check

## Purpose

Validate that the requirements specification artifact contains verified, traceable, and business-grounded requirements rather than a mixed set of features, assumptions, and implementation ideas.

## Applies To

- instantiated copies of [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md)
- Use after BA discovery and process analysis have produced enough evidence to verify requirements.
- Do not use as a substitute for downstream handoff review.

## Criteria

- Verified requirements are stated clearly and are scoped.
- Each verified requirement has a business rationale or an explicit connection to a business objective or process need.
- Requirements are testable or verifiable in business terms.
- Non-requirements are separated from verified requirements.
- Assumptions and open questions are separated from verified requirements.
- Constraints are captured.
- Business constraints and AI or agent-specific constraints are separated when relevant.
- Acceptance considerations are present.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if confirmed requirements can be distinguished reliably from everything that is still unverified, speculative, or implementation-led.

## Evidence Required

- The requirements specification artifact.
- Supporting business objectives and process analysis artifacts.
- Any clarifying notes or research used to verify the requirements.

If traceability or evidence is missing, fail the check and identify which requirements are unsupported.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when the requirement set needs to be reclassified into verified requirements, non-requirements, assumptions, and open questions.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when additional local evidence or clarification is needed before requirements can pass the check.

## Failure Response

- Rework the requirement set before packaging a downstream handoff.
- Remove, reclassify, or clarify unsupported requirements instead of treating them as confirmed.

## Notes

This is the core quality gate for requirement quality. It should be applied before any implementation-leaning planning begins.

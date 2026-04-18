# Role Package Review Check

## Purpose

Validate that a role-package review records meaningful findings, remediation decisions, and a defensible readiness assessment for the role package.

## Applies To

- instantiated copies of [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md)
- Use when the role definition and support system have been reviewed together.
- Do not use to validate the role definition or support system in isolation.

## Criteria

- The review identifies the inputs that were actually examined.
- Findings about coherence, gaps, duplication, or role drift are explicit.
- Remediation decisions are recorded clearly.
- Readiness is stated as ready, conditionally ready, or not ready with reasoning.
- The review reflects whether the role's Allium relationship is clear and consistent when it is relevant.
- Remaining risks and next steps are visible.
- The review does not collapse into vague approval language without concrete findings.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what was reviewed, what still matters, and whether the role package can be adopted safely.

## Evidence Required

- The role package review artifact.
- The role definition brief and role support system artifacts it refers to.
- Any supporting notes or comparison material cited in the review.

If the review depends on unstated context or does not show what was assessed, fail the check.

## Supporting Skills

- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) when the review needs to be reworked into a clearer set of findings, remediation decisions, and readiness reasoning.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when review findings still require synthesis across multiple local documents or prior notes.

## Failure Response

- Rework the review until findings, remediation, and readiness are explicit.
- Do not treat the role package as approved based on implicit or chat-only confidence.

## Notes

This check protects the final handoff of a role package by requiring an explicit review record.

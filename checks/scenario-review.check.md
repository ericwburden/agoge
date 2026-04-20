# Scenario Review Check

## Purpose

Validate that the scenario review artifact records meaningful findings, remediation decisions, and a defensible readiness assessment for the scenario package.

## Applies To

- instantiated copies of [`artifacts/scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md)
- Use when the scenario definition and scenario integration map have been reviewed together.
- Do not use to validate the scenario definition or integration map in isolation.

## Criteria

- The review identifies the inputs that were actually examined.
- The overall assessment and decision status are explicit.
- Integration risks and failure modes are concrete.
- Conditions and remediation decisions are recorded clearly.
- Follow-up owners and revisit triggers are visible.
- The review does not collapse into vague approval language without concrete reasoning.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what was reviewed, what still matters, and whether the scenario package can be adopted safely.

## Evidence Required

- The scenario review artifact.
- The scenario definition and scenario integration map artifacts it refers to.
- Any supporting notes or role-package materials cited in the review.

If the review depends on unstated context or does not show what was assessed, fail the check.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when review findings still require synthesis across multiple local documents or role packages.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when scenario readiness depends on whether a borrowed public pattern was adapted cleanly.

## Failure Response

- Rework the review until findings, remediation, and readiness are explicit.
- Do not treat the scenario package as approved based on implicit or chat-only confidence.

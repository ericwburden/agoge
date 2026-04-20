# Security / Compliance Review Check

## Purpose

Validate that the security/compliance review artifact states the current posture, conditions, risks, gaps, and approval limits clearly enough for downstream consumers.

## Applies To

- Instantiated copies of [`security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md)
- Use before producing the downstream security/compliance handoff

## Criteria

- Review scope and reviewed inputs are explicit.
- The overall assessment is honest about both confidence and caution.
- Decision status is explicit rather than implied from tone.
- Any scope, environment, evidence, or approval limits are clear.
- The review makes it clear when downstream-readiness is not the same thing as legal, audit, policy, or operational authorization.
- Key risks, gaps, control weaknesses, and trust-boundary-sensitive concerns are visible.
- Conditions, required follow-up, and follow-up owners are explicit when the package is conditional or blocked.
- Re-review triggers and the recommended next step are explicit.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated security/compliance review artifact
- The scope artifact and controls/evidence matrix

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the decision logic depends on distributed local evidence.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when posture confidence depends on stronger grounding in validated constraints.

## Failure Response

- Rework the review artifact instead of expecting downstream roles to infer the real posture.
- Route missing evidence, obligation clarity, or upstream defects to the earliest artifact or role that introduced the gap.

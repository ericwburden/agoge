# Release Readiness Decision Check

## Purpose

Validate that the release readiness decision expresses a clear release posture, ties that posture back to the reviewed package, and makes conditions, owners, and re-approval triggers explicit.

## Applies To

- Instantiated copies of [`release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md)
- Use before routing a release package downstream as ready, conditional, or blocked

## Criteria

- The decision scope and reviewed inputs are explicit.
- The overall assessment is understandable in plain language.
- The current release status is explicit rather than implied.
- The decision clearly distinguishes release packaging readiness from final deployment authorization when those are not the same thing.
- Conditions, approval limits, and residual risks are separated clearly.
- Required follow-up and owners are explicit when the status is not a plain ready state.
- Environment, rollout, and monitoring watchouts are preserved when they materially affect downstream use.
- Re-review or re-approval triggers are explicit when the current decision should not be treated as blanket ongoing approval.
- The decision does not claim more readiness than the reviewed implementation, review, and verification packages support.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream consumer could understand the current release posture and the conditions on changing it.

## Evidence Required

- The instantiated release readiness decision artifact being reviewed
- The corresponding release candidate summary and rollout notes artifacts
- The corresponding implementation, review, and verification packages when needed to confirm grounding

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the decision still depends on unsynthesized local evidence or approvals.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the logic is present but not yet downstream-usable.

## Failure Response

- Rework the decision artifact before treating the release package as downstream-ready.
- If the decision ambiguity comes from weak candidate framing or weak upstream evidence, route remediation to the earlier artifact that introduced the defect.

# Review Handoff Check

## Purpose

Validate that the review handoff artifact packages the review outputs for downstream use without losing the approval posture, active findings, or follow-up ownership.

## Applies To

- Instantiated copies of [`review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md)
- Use whenever review output is being handed to implementation, verification, release-adjacent, or human decision-making consumers.

## Criteria

- The reviewed change summary is clear enough for a downstream reader to orient quickly.
- The handoff references the review package artifacts that define the current review posture.
- The current approval posture is explicit together with any revision, scope, or evidence limits on that posture.
- Key findings, follow-up work, and owners are preserved clearly.
- Re-review triggers are explicit when downstream trust depends on changes or refreshed evidence.
- Verification, release, and trust-boundary watchouts are included when relevant.
- Upstream routing notes are explicit when issues should be handled outside implementation.
- The handoff tells a downstream reader what to do next without turning into a patch queue or release script.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if the review package can move downstream without forcing the next role to reconstruct the review from raw comments.

## Evidence Required

- The instantiated review handoff artifact being reviewed
- The corresponding review decision artifact
- The corresponding review findings artifact when needed to confirm no material issue was lost in handoff

## Supporting Skills

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the handoff preserves the substance but not yet the downstream-ready framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the handoff still depends on unsynthesized context.

## Failure Response

- Rework the handoff artifact before routing it downstream.
- If information loss began in the decision or findings artifact, repair the earlier stage instead of only decorating the handoff.

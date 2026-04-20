# Release Handoff Check

## Purpose

Validate that the release handoff packages the release outputs for downstream use without losing the current release posture, active conditions, operational caveats, or next-step instructions.

## Applies To

- Instantiated copies of [`release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md)
- Use whenever release output is being handed to downstream release-adjacent, operational, or adoption consumers

## Criteria

- The candidate summary is clear enough for a downstream reader to orient quickly.
- The handoff references the release package artifacts that define the current release posture.
- The current release posture is explicit together with any scope, environment, evidence, or approval limits on that posture.
- Active conditions, follow-up work, and owners are preserved clearly.
- Rollout, monitoring, and trust-boundary watchouts are included when relevant.
- Re-review or re-approval triggers are explicit when downstream trust depends on later events or changed evidence.
- Upstream routing notes are explicit when issues should be handled outside release packaging.
- The handoff tells a downstream reader what to do next without turning into a deployment runbook or incident plan.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if the release package can move downstream without forcing the next role or team to reconstruct it from earlier artifacts.

## Evidence Required

- The instantiated release handoff artifact being reviewed
- The corresponding release readiness decision
- The corresponding release candidate summary and rollout notes when needed to confirm nothing material was lost in handoff

## Supporting Skills

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the handoff preserves the substance but not yet the downstream-ready framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the handoff still depends on unsynthesized context.

## Failure Response

- Rework the handoff artifact before routing it downstream.
- If information loss began in the decision or rollout notes artifact, repair the earlier stage instead of only decorating the handoff.

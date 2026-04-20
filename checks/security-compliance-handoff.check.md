# Security / Compliance Handoff Check

## Purpose

Validate that the security/compliance handoff packages the current scope, posture, controls, conditions, and next-step guidance clearly enough for downstream consumers.

## Applies To

- Instantiated copies of [`security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md)
- Use before routing Security / Compliance Specialist outputs downstream

## Criteria

- The current assessment scope and package contents are summarized clearly.
- The current posture is explicit and properly scoped.
- The handoff does not imply final legal, audit, policy, or operational approval when further authorization is still required.
- Active controls, conditions, waivers, and gaps are strong enough for downstream consumers to preserve the security/compliance intent.
- Follow-up owners and re-review triggers are explicit.
- The recommended next consumer is named and the routing logic is clear.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated security/compliance handoff artifact
- The scope artifact, controls/evidence matrix, and review artifact

## Supporting Skills

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is downstream packaging clarity.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when package context is spread across multiple local sources.

## Failure Response

- Rework the handoff before treating it as downstream-ready.
- If the defect began earlier in the chain, fix the earliest artifact instead of compensating only in the handoff.

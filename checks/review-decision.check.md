# Review Decision Check

## Purpose

Validate that the review decision artifact expresses a clear approval posture, ties that posture back to the findings, and makes required follow-up explicit.

## Applies To

- Instantiated copies of [`review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md)
- Use before routing a review package downstream as approved, conditional, or blocked.

## Criteria

- The decision scope and reviewed inputs are explicit.
- The decision is clearly scoped to the reviewed revision, scope boundary, or evidence window when that limit matters.
- The overall assessment is understandable in plain language.
- The current review status is explicit rather than implied.
- Blocking findings, non-blocking findings, and residual risks are separated clearly.
- Required remediation, conditions, and follow-up owners are explicit when the status is not a plain approval.
- Approval limits and re-review triggers are explicit when the decision should not be treated as blanket ongoing approval.
- Verification or release watchouts are preserved when they materially affect downstream use.
- The decision does not claim more assurance than the findings and evidence support.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream consumer could understand the current review posture and the conditions on changing it.

## Evidence Required

- The instantiated review decision artifact being reviewed
- The corresponding review findings artifact
- The corresponding code review scope artifact when needed to interpret the decision fairly

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the decision still depends on synthesizing multiple findings or evidence references.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the logic is present but the decision framing is not yet downstream-usable.

## Failure Response

- Rework the decision artifact before treating the review package as downstream-ready.
- If the decision ambiguity comes from weak findings or weak scope framing, route remediation to the earlier artifact that introduced the defect.

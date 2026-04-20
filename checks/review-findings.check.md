# Review Findings Check

## Purpose

Validate that the review findings artifact records concrete, actionable, severity-aware findings rather than vague commentary or ungrounded preference.

## Applies To

- Instantiated copies of [`review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- Use whenever a Code Reviewer findings package is being treated as durable review output.

## Criteria

- Each material finding is concrete enough to understand the problem, affected area, and why it matters.
- Each material finding identifies the affected location with enough precision for remediation, such as file, module, interface, query, prompt section, config key, or line range when practical.
- Blocking, non-blocking, and advisory concerns are distinguished clearly.
- Severity and confidence are explicit for material findings.
- Findings are tied to code, evidence, or an explicit upstream commitment rather than to generic preference alone.
- The evidence basis for each material finding is explicit enough that another reviewer could understand how the concern was reached.
- Requested remediation, clarification, or evidence is explicit enough for downstream action.
- Missing validation or weak evidence findings are recorded explicitly when they materially affect review confidence.
- Re-review triggers are explicit when the next review depends on changed code, refreshed evidence, or widened scope.
- If there are no material findings, the artifact explains the review coverage limits so silence is not mistaken for broader assurance than was actually provided.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another engineer or reviewer could act on the findings without reconstructing them from inline comments.

## Evidence Required

- The instantiated review findings artifact being reviewed
- The corresponding code review scope artifact
- The implementation package or supporting evidence when needed to confirm grounding

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when findings depend on unsynthesized local evidence.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when a suspected finding depends on targeted browser or application reproduction.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when a finding depends on spec-to-code drift.

## Failure Response

- Rework the findings artifact until each material concern is specific and grounded.
- If the issue is really missing review scope or missing implementation context, fix the earlier artifact rather than compensating inside the findings text.

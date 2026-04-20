# Code Review Scope Check

## Purpose

Validate that the code review scope artifact makes the review target, reviewed inputs, change boundary, hotspots, and limits explicit enough to support a defensible review pass.

## Applies To

- Instantiated copies of [`code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md)
- Use before treating a findings package as grounded review work rather than ad hoc commentary.

## Criteria

- The artifact clearly identifies the implementation package or change set under review.
- Reviewed inputs include the implementation package and relevant upstream artifacts when they materially constrain the change.
- The change boundary is explicit enough that silence cannot be mistaken for full approval of unrelated areas.
- Review hotspots and risk areas are named rather than implied.
- Review constraints and limits are explicit when the review is partial, evidence-constrained, or time-bounded.
- The scope distinguishes in-scope review work from explicitly out-of-scope areas.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what was reviewed and what was not.

## Evidence Required

- The instantiated code review scope artifact being reviewed
- The corresponding implementation package artifacts when needed to verify scope accuracy
- Relevant upstream handoffs when needed to confirm review anchors

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when scope context is spread across multiple local sources.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when scope anchors back to requirements or acceptance commitments are weak.

## Failure Response

- Rework the review scope before treating the findings package as review-complete.
- If scope uncertainty is caused by a weak implementation package, route remediation there instead of masking the gap in review language.

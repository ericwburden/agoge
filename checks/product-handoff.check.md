---
id: product-handoff
kind: check
title: Product Handoff Check
version: 1
summary: Validate that the product handoff artifact packages the current direction,
  priorities, conditions, and next-step guidance clearly enough for downstream consumers.
mode: headings
severity: error
applies_to:
- product-handoff
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Current Product Direction
- Product Package Included
- Current Priority Posture
- Priority And Acceptance Guidance
- Locked Decisions To Preserve
- Semantic Review Status
- Deferred Scope And Open Tradeoffs
- Follow-Up Owners
- Revisit Triggers
- Upstream Routing Notes
- Recommended Next Consumer
---

# Product Handoff Check

## Purpose

Validate that the product handoff artifact packages the current direction, priorities, conditions, and next-step guidance clearly enough for downstream consumers.

## Applies To

- Instantiated copies of [`product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md)
- Use before routing Product Owner outputs downstream

## Criteria

- The current product direction and package contents are summarized clearly.
- The current priority posture is explicit and properly scoped.
- The handoff does not imply implementation commitment, sprint commitment, or release approval when further downstream authorization is still required.
- Priority and acceptance guidance is strong enough for downstream consumers to preserve the product intent.
- Locked decisions to preserve are explicit.
- Semantic review status is visible when semantic review is required for scenario completion.
- Deferred scope, open tradeoffs, and follow-up owners are explicit.
- Revisit triggers are visible.
- The recommended next consumer is named and the routing logic is clear.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated product handoff artifact
- The product direction, backlog prioritization, and product decision review artifacts

## Supporting Skills

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is downstream packaging clarity.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when package context is spread across multiple local sources.

## Failure Response

- Rework the handoff before treating it as downstream-ready.
- If the defect began earlier in the artifact chain, fix the earliest artifact instead of compensating in the handoff.

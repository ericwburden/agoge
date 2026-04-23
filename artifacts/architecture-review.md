---
id: architecture-review
kind: artifact
title: Architecture Review
version: 1
summary: Capture the durable review record for a drafted architecture package, including
  findings, readiness, unresolved risks, and required remediation before the architecture
  is handed downstream.
template: true
default_output_path: docs/architecture/architecture-review.md
checks:
- architecture-review
- architecture-traceability
- solution-architect-boundary
---

# Architecture Review

## Purpose

Capture the durable review record for a drafted architecture package, including findings, readiness, unresolved risks, and required remediation before the architecture is handed downstream.

Use this artifact after the first architecture design pass and before producing the downstream architecture handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Solution Architect work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the architecture is ready to move forward, what findings still matter, what remediation is required, and which open risks or decisions still constrain the design.

## Related Checks

- Primary: [`architecture-review.check.md`](D:/Projects/agoge/checks/architecture-review.check.md)
- Cross-cutting: [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
- Cross-cutting: [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)

## Review Scope

State which architecture artifacts and architectural concerns are in scope for this review.

## Reviewed Inputs

Reference the solution architecture, architecture decisions, upstream BA artifacts, notes, or supporting constraints used in this review.

## Overall Assessment

Summarize the current architecture quality in plain language, including the strongest parts of the design and the most material concerns.

## Readiness Or Approval Status

State whether the architecture is:

- ready for downstream handoff
- ready with conditions
- blocked pending remediation

If relevant, note who or what must confirm the next step.

## Decision Owner Or Approver

Identify who owns the readiness decision or approval for this architecture package.

If the status is `ready with conditions` or `blocked pending remediation`, this section should be explicit rather than implied.

## Key Findings

Record the most important findings, grouped by artifact, design area, or defect type.

## Semantic Review Findings

Capture the most important semantic corrections surfaced by human review, especially when they changed the intended end state, package boundaries, or architectural framing.

## Decision Changes Since Draft

List the architecture decisions or locked constraints that changed during review and what downstream roles should now treat as current.

## Cross-Artifact Reconciliation

Record the cross-artifact updates required to keep the architecture package internally consistent after semantic review.

## Interface And Contract Observations

Call out the interface seams, ownership boundaries, or contract assumptions that deserve special attention before downstream work begins.

## Unresolved Risks And Questions

List unresolved risks, open technical questions, or architectural uncertainties that still materially affect the design.

## Required Remediation

Describe the changes needed before the architecture should be treated as downstream-ready, and point back to the earliest artifact that should be reworked.

## Condition Owners

If the architecture is conditionally ready or blocked, identify who owns each required follow-up, clarification, or approval condition.

## Recommended Next Step

Describe the immediate next architectural step, such as rework, targeted clarification, or downstream handoff.

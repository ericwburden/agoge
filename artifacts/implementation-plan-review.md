---
id: implementation-plan-review
kind: artifact
title: Implementation Plan Review
version: 1
summary: Capture the durable review record for a drafted implementation planning package,
  including findings, readiness, unresolved risks, and required remediation before
  the plan is handed downstream.
template: true
default_output_path: docs/planning/implementation-plan-review.md
checks:
- implementation-plan-review
- planning-traceability
- technical-planner-boundary
---

# Implementation Plan Review

## Purpose

Capture the durable review record for a drafted implementation planning package, including findings, readiness, unresolved risks, and required remediation before the plan is handed downstream.

Use this artifact after the first planning pass and before producing the downstream implementation handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Technical Planner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the implementation plan is ready to guide execution, what findings still matter, what remediation is required, and which open risks or decisions still constrain the plan.

## Related Checks

- Primary: [`implementation-plan-review.check.md`](D:/Projects/agoge/checks/implementation-plan-review.check.md)
- Cross-cutting: [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
- Cross-cutting: [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

## Review Scope

State which planning artifacts and planning concerns are in scope for this review.

## Reviewed Inputs

Reference the implementation strategy, sequencing and dependencies artifact, upstream architecture handoff, requirements handoff, notes, or supporting constraints used in this review.

## Overall Assessment

Summarize the current planning quality in plain language, including the strongest parts of the plan and the most material concerns.

## Readiness Or Approval Status

State whether the plan is:

- ready for downstream handoff
- ready with conditions
- blocked pending remediation

If relevant, note who or what must confirm the next step.

## Decision Owner Or Approver

Identify who owns the readiness decision or approval for this planning package.

If the status is `ready with conditions` or `blocked pending remediation`, this section should be explicit rather than implied.

## Key Findings

Record the most important findings, grouped by artifact, workstream, or defect type.

## Semantic Review Findings

Capture the most important semantic corrections surfaced by human review, especially when they changed slice boundaries, readiness framing, or execution assumptions.

## Decision Changes Since Draft

List the planning decisions or locked constraints that changed during review and what downstream roles should now treat as current.

## Cross-Artifact Reconciliation

Record the cross-artifact updates required to keep the planning package internally consistent after semantic review.

## Dependency And Sequencing Observations

Call out the dependency hotspots, weak sequencing assumptions, or critical-path concerns that deserve special attention before downstream work begins.

## Unresolved Risks And Questions

List unresolved planning risks, open technical questions, or execution uncertainties that still materially affect the plan.

## Required Remediation

Describe the changes needed before the plan should be treated as downstream-ready, and point back to the earliest artifact that should be reworked.

## Condition Owners

If the plan is conditionally ready or blocked, identify who owns each required follow-up, clarification, or approval condition.

## Recommended Next Step

Describe the immediate next planning step, such as rework, targeted clarification, or downstream handoff.

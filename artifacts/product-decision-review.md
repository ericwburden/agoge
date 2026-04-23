---
id: product-decision-review
kind: artifact
title: Product Decision Review
version: 1
summary: Capture the durable review outcome for a drafted Product Owner package, including
  the current decision posture, risks, tradeoffs, conditions, and what should happen
  next.
template: true
default_output_path: docs/product/product-decision-review.md
checks:
- product-decision-review
- product-traceability
- product-owner-boundary
---

# Product Decision Review

## Purpose

Capture the durable review outcome for a drafted Product Owner package, including the current decision posture, risks, tradeoffs, conditions, and what should happen next.

Use this artifact after product direction and backlog prioritization exist and before producing the downstream product handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Product Owner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the current product package is ready for downstream solutioning or planning, what evidence supports that posture, what conditions or risks still matter, and what must happen next.

## Related Checks

- Primary: [`product-decision-review.check.md`](D:/Projects/orpheum/checks/product-decision-review.check.md)
- Cross-cutting: [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md)
- Cross-cutting: [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md)

## Review Scope

State which product package, backlog window, initiative set, or decision horizon this review covers.

## Reviewed Inputs

Reference the product direction, backlog prioritization, upstream discovery artifacts, feedback inputs, and supporting evidence used in this review.

## Overall Assessment

Summarize the current product posture in plain language, including the strongest reasons for confidence and the most important reasons for caution.

## Decision Status

State whether the package is:

- ready for downstream solutioning or planning
- ready with conditions
- blocked pending clarification, evidence, or approval

State any scope, timing, evidence, or approval limits clearly if the decision is not universally applicable.

Make it explicit when this status means "ready for downstream solutioning or planning" rather than "already committed for implementation, sprint intake, or release."

## Decision Owner

Identify who owns the current product posture or who must explicitly approve the next step if the status is conditional or blocked.

If downstream implementation commitment, scheduling, or release still requires separate planning, delivery, or operational approval, record that here rather than leaving the distinction implied.

## Key Risks, Tradeoffs, And Tensions

Describe the material product risks, competing priorities, or stakeholder tensions that still matter.

## Semantic Review Findings

Capture the most important semantic corrections surfaced by human review, especially when they changed product framing, scope boundaries, or acceptance intent.

## Decision Changes Since Draft

List the decisions that changed during review and what downstream consumers should now treat as current.

## Cross-Artifact Reconciliation

Record the cross-artifact updates required to keep the product package internally consistent after semantic review.

## Conditions And Required Follow-Up

Record the clarification, approval, evidence gathering, discovery, or reprioritization work required before the product posture should change.

## Follow-Up Owners

If the package is conditional or blocked, identify who owns each required follow-up.

## Revisit Triggers

List what changes in evidence, customer feedback, strategy, release learnings, timing, or constraints should trigger another product pass.

## Recommended Next Step

Describe the immediate next step, such as downstream architecture handoff, further discovery, prioritization revision, or explicit business approval.

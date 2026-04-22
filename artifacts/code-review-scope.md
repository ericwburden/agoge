---
id: code-review-scope
kind: artifact
title: Code Review Scope
version: 1
summary: Capture the review target, reviewed inputs, change boundary, conformance
  anchors, risk hotspots, and explicit limits for a concrete code review pass.
template: true
default_output_path: docs/orpheum/code-review-scope.md
checks:
- code-review-scope
- review-traceability
- code-reviewer-boundary
---

# Code Review Scope

## Purpose

Capture the review target, reviewed inputs, change boundary, conformance anchors, risk hotspots, and explicit limits for a concrete code review pass.

Use this artifact to make the review frame explicit before or alongside findings so downstream readers do not have to infer what was actually reviewed from comments alone.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Code Reviewer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what implementation slice was reviewed, which upstream commitments constrained the review, which areas were treated as review hotspots, what evidence or context was consulted, and what review limits still matter.

## Related Checks

- Primary: [`code-review-scope.check.md`](D:/Projects/orpheum/checks/code-review-scope.check.md)
- Cross-cutting: [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md)
- Cross-cutting: [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md)

## Review Objective

Summarize what change set or implementation package this review pass is challenging and what the review is trying to determine.

## Reviewed Inputs

Reference the implementation record, implementation evidence, implementation readiness review, implementation package handoff, relevant requirements or architecture handoffs, reviewed planning artifacts, and any governing behavioral specifications used in this review.

## Change Boundary Summary

Describe the reviewed change boundary, including the main code, configuration, schema, contract, migration, prompt, or asset surfaces in scope.

## Upstream Conformance Anchors

Map the reviewed change back to the upstream commitments that matter for review.

For each major reviewed area, capture:

- the requirement or expected behavior it must satisfy or preserve
- the architecture or interface commitment it must respect
- the implementation-plan slice or dependency assumption it is fulfilling
- any governing specification or trust-boundary expectation that materially affects review treatment

## Review Hotspots And Risk Areas

List the major hotspots this review should pay attention to, such as:

- correctness or regression risk
- interface or contract drift
- migration or rollout hazards
- state handling or edge cases
- missing or weak validation
- trust-boundary-sensitive or human-control-point-sensitive behavior

## Evidence And Context Sources

List the evidence, notes, run outputs, diffs, logs, screenshots, or supporting context consulted during review.

## Review Constraints And Limits

Describe the limits that affect how strongly the review can conclude, such as:

- unavailable environments or evidence
- unusually large change surface
- weak implementation record quality
- unresolved upstream ambiguity
- time-bounded or partial review scope

## Explicitly Out Of Scope

List change areas or concerns that this review pass did not cover so later readers do not mistake silence for approval.

## Expected Follow-Up Consumers

Identify which downstream roles or human decision-makers are expected to use this review scope and findings set next.

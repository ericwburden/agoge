---
id: product-handoff
kind: artifact
title: Product Handoff
version: 1
summary: Package the completed Product Owner outputs into a downstream-ready handoff
  that lets solutioning, planning, delivery, or approval consumers use the package
  without reconstructing product intent, priorities, or tradeoffs from earlier artifacts.
template: true
default_output_path: docs/product/product-handoff.md
checks:
- product-handoff
- product-traceability
- product-owner-boundary
---

# Product Handoff

## Purpose

Package the completed Product Owner outputs into a downstream-ready handoff that lets solutioning, planning, delivery, or approval consumers use the package without reconstructing product intent, priorities, or tradeoffs from earlier artifacts.

Use this artifact after the product decision review is explicit and before routing the package downstream.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Product Owner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what product direction is being handed off, what work is prioritized, what conditions still matter, who owns follow-up, and what should happen next.

## Related Checks

- Primary: [`product-handoff.check.md`](D:/Projects/orpheum/checks/product-handoff.check.md)
- Cross-cutting: [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md)
- Cross-cutting: [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md)

## Current Product Direction

Summarize the current product goal, target value, and major scope boundary being handed off.

## Product Package Included

Reference the product direction, backlog prioritization, product decision review, and key supporting discovery or evidence artifacts that define the current package.

## Current Priority Posture

State what work is prioritized next and whether the package is ready, conditional, or blocked for the intended downstream consumer.

Make any scope, timing, evidence, or approval limits explicit so downstream consumers do not treat the handoff as blanket product approval.

If further planning, implementation commitment, scheduling, or release approval is still required, state that explicitly so the handoff is not misread as execution authorization.

## Priority And Acceptance Guidance

Describe the most important must-have outcomes, acceptance-oriented conditions, and guardrails that downstream roles should preserve.

## Locked Decisions To Preserve

Summarize the product decisions, constraints, or scope boundaries that downstream roles should not silently revisit.

## Semantic Review Status

State whether semantic artifact review is complete, what materially changed during that review, and whether any cross-artifact reconciliation is still pending.

## Deferred Scope And Open Tradeoffs

Describe what has been deferred, what tensions remain active, and what downstream roles should not silently expand.

## Follow-Up Owners

Identify who owns each required clarification, approval, or reprioritization follow-up.

## Revisit Triggers

List the changes in feedback, evidence, timing, constraints, or delivery outcomes that should cause the package to return to Product Owner work before downstream consumers rely on it further.

## Upstream Routing Notes

Call out any issues that should be routed to Business Analyst, specification, release, or other upstream or adjacent roles rather than treated purely as product cleanup.

## Recommended Next Consumer

Identify which downstream role, team, or human decision-maker should take the package next and why.

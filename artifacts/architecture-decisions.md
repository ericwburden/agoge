---
id: architecture-decisions
kind: artifact
title: Architecture Decisions
version: 1
summary: Capture the major architectural decisions, alternatives, rationale, and consequences
  that shape the solution.
template: true
default_output_path: docs/architecture/architecture-decisions.md
checks:
- architecture-decisions
- architecture-traceability
- solution-architect-boundary
---

# Architecture Decisions

## Purpose

Capture the major architectural decisions, alternatives, rationale, and consequences that shape the solution.

Use this artifact to make design reasoning explicit so downstream roles do not have to infer why a direction was chosen.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Solution Architect work.

## Completion Guidance

This artifact is complete when a downstream reader can understand which major decisions have been made, what alternatives were considered, why the chosen direction won, and what consequences or unresolved tradeoffs still matter.

## Related Checks

- Primary: [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md)
- Cross-cutting: [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
- Cross-cutting: [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)

## Decision Summary

Provide a short summary of the decision set this artifact covers.

## Major Decisions

For each major decision, capture:

- decision statement
- decision drivers
- alternatives considered
- chosen direction
- rationale
- consequences or follow-on impacts
- interface or contract implications when the decision materially affects a boundary

## Locked Decisions And Downstream Non-Negotiables

Summarize the decisions, constraints, or interface commitments that downstream roles should preserve unless a human explicitly reopens them.

Capture:

- what is locked
- why it is locked
- which downstream roles must preserve it
- what kind of change would justify reopening it

## Deferred Decisions

List meaningful architectural decisions that remain open and why they were deferred.

## Architecture Assumptions

List assumptions that materially shape the decisions but are not yet fully verified.

## Risks And Tradeoffs

Describe the key tradeoffs or risks introduced by the chosen directions.

---
id: business-objectives
kind: artifact
title: Business Objectives
version: 1
summary: Capture the business problem, goals, stakeholders, success criteria, scope
  boundaries, and business context for a proposed system or project.
template: true
default_output_path: docs/discovery/business-objectives.md
checks:
- business-objectives
- traceability
- business-analyst-boundary
---

# Business Objectives

## Purpose

Capture the business problem, goals, stakeholders, success criteria, scope boundaries, and business context for a proposed system or project.

Use this artifact early in discovery to separate the underlying business need from proposed solutions.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live BA work.

## Completion Guidance

This artifact is complete when the business problem, desired outcomes, affected stakeholders, and scope boundaries are clear enough to support process analysis and requirement discovery without relying on chat context.

## Related Checks

- Primary: [`business-objectives.check.md`](D:/Projects/agoge/checks/business-objectives.check.md)
- Cross-cutting: [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
- Cross-cutting: [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)

## Problem Statement

Describe the problem or opportunity in business terms.

## Business Context

Describe the organizational, operational, or project context that makes this work necessary now.

## Business Objectives

List the desired business outcomes. Keep outcomes separate from implementation ideas or preferred features.

## Success Criteria

Describe how success will be recognized or verified from a business perspective.

## Stakeholders

List the people, teams, or roles affected by the problem, the process, or the desired outcome.

For each stakeholder, capture:

- role or group
- relationship to the problem
- key concern or interest

## In Scope

Describe what this effort is intended to address.

## Out Of Scope

Describe what this effort is not intended to address at this stage.

## Constraints

Capture business, policy, operational, timing, or dependency constraints that materially shape the work.

If the project includes AI-enabled or agentic behavior, record business-facing constraints here and keep agent-specific constraints separate.

## Assumptions

List assumptions currently being relied on but not yet verified.

## Proposed Solutions Mentioned

Record solution ideas raised during discovery, but keep them separate from the business objective itself.

## Open Questions

List unresolved questions that block or weaken understanding of the business objective.

## Stakeholder Confirmation

Record whether the current framing has been restated to stakeholders and what confirmation status applies.

Capture:

- confirmation status such as confirmed, partially confirmed, or pending
- which stakeholders or stakeholder groups have responded
- any material disagreements, corrections, or unresolved concerns

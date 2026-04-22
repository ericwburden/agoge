---
id: process-analysis
kind: artifact
title: Process Analysis
version: 1
summary: Capture current-state and future-state process understanding, including actors,
  triggers, inputs, outputs, exceptions, business rules, and identified gaps.
template: true
default_output_path: docs/discovery/process-analysis.md
checks:
- process-analysis
- traceability
- business-analyst-boundary
---

# Process Analysis

## Purpose

Capture current-state and future-state process understanding, including actors, triggers, inputs, outputs, exceptions, business rules, and identified gaps.

Use this artifact when the business problem depends on how work currently flows and how that flow needs to change.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live BA work.

## Completion Guidance

This artifact is complete when a downstream reader can understand how the process works today, what should change, and where the major gaps, rules, and exceptions are without inferring missing business context.

## Related Checks

- Primary: [`process-analysis.check.md`](D:/Projects/agoge/checks/process-analysis.check.md)
- Cross-cutting: [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
- Cross-cutting: [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)

## Process In Scope

Name the process or workflow being analyzed.

## Objective Of The Process Change

Describe what the process should achieve from a business perspective.

## Process Needs

Describe the operational needs the process must satisfy in order to support the business objective.

Capture:

- the outcomes the process must enable
- the conditions the process must preserve or improve
- the operational needs that later requirements should trace back to

## Current-State ("As-Is") Process

Describe the current process in sequence.

Capture:

- major steps
- actors involved
- triggers that start the process
- inputs used
- outputs produced

## Current-State Pain Points

Describe where the current process breaks down, creates risk, causes delay, or fails to meet business needs.

## Future-State ("To-Be") Process

Describe the intended future process in sequence.

Capture:

- major steps
- actors involved
- triggers that start the process
- inputs used
- outputs produced

## Business Rules

List the business rules, decisions, or policy conditions that govern the process.

## Exceptions And Edge Cases

Describe known exceptions, special handling paths, failure cases, or escalation paths.

## Gaps

Describe the meaningful differences between the current-state and future-state process.

## Dependencies

List systems, teams, approvals, or external conditions the process depends on.

## Human Oversight Notes

If the process includes AI-enabled or agentic behavior, describe where human review, approval, intervention, or escalation is expected.

## Open Questions

List unresolved questions about the process, rules, or participants.

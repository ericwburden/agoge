# Scenario Definition

## Purpose

Capture the intent, lifecycle window, trigger conditions, participating roles, entry conditions, target outputs, exit conditions, and core sequence for a reusable multi-role scenario.

Use this artifact before detailed scenario integration, scenario review, or downstream scenario adoption work is treated as settled.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Scenario Designer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what repeatable multi-role activity the scenario is meant to support, when it should be used, which roles participate, what must be true before it starts, what should exist when it ends, and what broad sequence governs the work in between.

## Related Checks

- Primary: [`scenario-definition.check.md`](D:/Projects/orpheum/checks/scenario-definition.check.md)
- Cross-cutting: [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md)
- Cross-cutting: [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md)

## Scenario Name And Intent

State the stable name of the scenario and the repeatable multi-role activity it is meant to support.

## Lifecycle Window And Trigger Conditions

Describe where this scenario sits in the broader lifecycle and what events, decisions, or conditions should trigger its use.

## Participating Roles And Why

Identify which roles participate in the scenario and why their role-owned workflows belong in scope.

## Entry Conditions

Describe what artifacts, approvals, decisions, validated inputs, or environmental conditions must already exist before the scenario should start.

## Target Outputs And Exit Conditions

Describe the outputs, readiness state, or downstream posture that should exist when the scenario completes successfully.

## Core Sequence

Summarize the major phases or ordered steps of the scenario at the level needed to understand the multi-role chain before detailed integration mapping.

## Decision Gates And Human Checkpoints

List the major gates, approvals, or human-in-the-loop checkpoints that materially affect the scenario.

## Scenario Constraints And Non-Goals

Describe the constraints, exclusions, or limits that shape the scenario and what it is not supposed to absorb.

## Open Questions And Design Gaps

Record unresolved scenario questions, weak assumptions, or missing role-package support that still affect the design.

## Recommended Next Step

Describe the immediate next step, such as scenario integration mapping, role-package refinement, scenario review, or adoption handoff.

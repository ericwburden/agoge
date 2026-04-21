---
id: sequencing-and-dependencies
kind: artifact
title: Sequencing And Dependencies
version: 1
summary: Capture the intended execution order, dependency structure, and critical
  path for the planned implementation work.
template: true
default_output_path: docs/planning/sequencing-and-dependencies.md
checks:
- sequencing-and-dependencies
- planning-traceability
- technical-planner-boundary
---

# Sequencing And Dependencies

## Purpose

Capture the intended execution order, dependency structure, and critical path for the planned implementation work.

Use this artifact to make dependency handling and sequencing explicit so downstream roles do not have to reconstruct work order from architecture notes or hand-wavy plan summaries.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Technical Planner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what should happen first, what can proceed in parallel, what work is blocked on external conditions, where the critical path sits, and what decision gates or integration checkpoints still matter.

## Related Checks

- Primary: [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md)
- Cross-cutting: [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
- Cross-cutting: [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

## Sequencing Summary

Provide a short summary of the intended execution order and the main reason the sequence is shaped this way.

## Workstream Order

List the major workstreams or slices in their intended order, including the earliest useful implementation slice.

## Dependency Map

Describe the meaningful dependencies between workstreams, external systems, approvals, environments, data changes, or interface decisions.

For each material dependency, capture:

- what is depended on
- which workstream or slice is affected
- the owner of the dependency, if known
- whether it is blocking, sequencing-sensitive, or only rollout-sensitive
- the consequence if it does not land as assumed

## Critical Path

Identify the work that most strongly determines total delivery order or downstream readiness.

## Parallelization Opportunities

Describe which workstreams can proceed safely in parallel and what assumptions make that parallelization valid.

## Decision Gates And Spikes

List the checkpoints, architectural clarifications, technical spikes, or external confirmations that should change sequencing if they resolve differently than expected.

For each material decision gate or spike, capture:

- the decision, confirmation, or experiment outcome being waited on
- the owner responsible for deciding or confirming it
- the default sequencing assumption while it remains open
- how sequencing changes if it resolves in the expected direction
- how sequencing changes if it resolves in a materially different direction

## Integration, Migration, Or Rollout Checkpoints

Describe the integration milestones, migration boundaries, rollout-sensitive steps, or cutover concerns that materially affect sequencing.

## Verification Touchpoints

Identify the places where verification work should start early, run in parallel, or wait for a specific slice or dependency to land.

## Remaining Sequencing Risks And Assumptions

List sequencing assumptions or dependency risks that still need active monitoring.

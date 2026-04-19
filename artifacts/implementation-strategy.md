# Implementation Strategy

## Purpose

Capture the implementation approach that turns reviewed architecture and validated requirements into a coherent execution structure.

Use this artifact to make the planning intent explicit so downstream roles do not have to infer slice boundaries, enabling work, or readiness assumptions from scattered notes.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Technical Planner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what implementation approach is being proposed, why the slices are shaped that way, what assumptions or constraints matter, what enabling work is required, and where the main planning risks still sit.

## Related Checks

- Primary: [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md)
- Cross-cutting: [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
- Cross-cutting: [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

## Planning Scope And Objective

Summarize the implementation problem this plan is organizing and the part of the delivery effort this strategy covers.

## Input Context

Reference the architecture handoff, reviewed architecture artifacts, requirements artifacts, and any existing behavioral specifications this plan depends on.

## Traceability Map

Map the planned slices or workstreams back to the upstream sources that justify them.

For each major slice or workstream, capture:

- the validated requirements it implements or enables
- the architectural decisions, boundaries, or hotspots it depends on
- any architecture-handoff risks, conditions, or unresolved issues that materially shape the work
- any specification areas that materially constrain implementation behavior or verification treatment

## Planning Assumptions And Constraints

List the delivery assumptions, external constraints, operational limits, policy constraints, platform limits, or team constraints that materially shape the implementation strategy.

## Implementation Approach

Describe the overall implementation approach, such as phased rollout, vertical slices, enabling-platform-first work, migration-first work, or parallel workstream execution.

## Slice Strategy

Describe how implementation is being sliced and why that slicing approach was chosen.

## Workstream Overview

List the major workstreams, modules, services, or delivery tracks and what each one is responsible for.

## Enabling Work And Spikes

List the exploratory work, migrations, technical spikes, environment setup, or prerequisite changes that materially affect when committed implementation can begin.

## Slice Exit Criteria

Describe what must be true for each major slice or workstream to be treated as ready to hand off, integrate, or close.

For each major slice or workstream, capture:

- the minimum implementation outcome expected
- the verification evidence, test condition, or review signal required
- any interface, migration, or dependency condition that must be satisfied before the slice should be treated as complete
- whether the slice exits to another implementation slice, a verification stage, or a rollout checkpoint

## Readiness Conditions

Describe the conditions that should be true before downstream implementation should treat this strategy as ready, such as upstream approvals, dependency confirmation, data availability, contract clarification, or infrastructure readiness.

For each readiness condition, also capture:

- the owner responsible for clearing or confirming it
- whether it blocks all implementation, a specific workstream, or only downstream rollout
- the earliest point where the condition should be rechecked if it remains open

## Verification And Rollout Considerations

Describe the parts of the strategy that materially affect verification scope, rollout order, migration safety, or trust-boundary-sensitive behavior.

## Deferred Or Not Included

List the work that is intentionally out of scope for this implementation strategy, deferred to a later slice, or intentionally left for a downstream role or later planning pass.

Keep true deferrals separate from blocked work or unresolved dependencies.

## Risks And Open Questions

List the main planning risks, assumptions still under stress, and questions that remain unresolved.

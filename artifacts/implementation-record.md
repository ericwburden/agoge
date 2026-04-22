---
id: implementation-record
kind: artifact
title: Implementation Record
version: 1
summary: Capture the implementation scope, source context, traceability, changed areas,
  and material deviations for a concrete implementation slice.
template: true
default_output_path: docs/planning/implementation-record.md
checks:
- implementation-record
- implementation-traceability
- implementation-engineer-boundary
---

# Implementation Record

## Purpose

Capture the implementation scope, source context, traceability, changed areas, and material deviations for a concrete implementation slice.

Use this artifact to make the implementation intent and actual delivered change set explicit so downstream roles do not have to infer them only from diffs, commit messages, or chat history.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Implementation Engineer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what slice was being implemented, what upstream artifacts governed the work, what was actually changed, where the change boundary sits, and what plan deviations, blockers, or open questions still matter.

## Related Checks

- Primary: [`implementation-record.check.md`](D:/Projects/orpheum/checks/implementation-record.check.md)
- Cross-cutting: [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md)
- Cross-cutting: [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md)

## Implementation Scope And Objective

Summarize the implementation problem this change set is addressing and the slice boundary this record covers.

## Input Context

Reference the reviewed implementation handoff, reviewed planning artifacts, reviewed architecture artifacts, validated requirement artifacts, and any governing behavioral specifications this implementation depends on.

## Traceability Map

Map the implemented slice back to the upstream sources that justify it.

For each major change area, capture:

- the validated requirements it implements, enables, or preserves
- the architectural decisions, interfaces, or constraints it depends on
- the implementation-plan slices, dependencies, or readiness conditions it is fulfilling
- any specification areas that materially constrain expected behavior or validation treatment

## Target Slice Or Change Boundary

Describe the intended implementation boundary, including what this slice is supposed to change and what it is explicitly not supposed to absorb.

## Planned Versus Actual Scope

Describe how the actual implementation compared to the reviewed plan.

Capture:

- what was completed as planned
- what was partially completed or deferred
- what changed shape during implementation
- whether any scope expansion or contraction occurred and why

## Change Summary

Summarize the major code, configuration, data, contract, migration, or asset changes that were made.

## Change Inventory

List the concrete files, modules, services, scripts, prompts, configurations, schemas, or assets that were changed.

For each entry, capture:

- what was changed
- why that item had to change for this slice
- whether the item represents the main implementation path, enabling support, or incidental packaging support

## Changed Components And Affected Areas

List the major components, modules, services, interfaces, or data surfaces affected by the implementation.

For each affected area, capture:

- the role it plays in the implemented slice
- whether the change is behavioral, structural, configuration-only, migration-related, or support-only
- the main downstream concern, if one exists

## Interface, Schema, And Contract Effects

Describe any interface, schema, protocol, API, event, prompt-contract, or integration changes that downstream roles should know about.

## Deviations From Plan Or Specification

Record any material deviations from the reviewed implementation plan, reviewed architecture, or governing specification.

Keep true deviations separate from planned flexibility or harmless detail changes.

## Blockers, Risks, And Open Questions

List the implementation blockers, residual risks, and unresolved questions that still matter downstream.

## Deferred Or Intentionally Not Included

List work that was intentionally deferred, excluded, or left for a later slice rather than implemented here.

---
id: sequencing-and-dependencies
kind: check
title: Sequencing And Dependencies Check
version: 1
summary: Validate that sequencing, dependencies, critical path assumptions, and parallelization
  opportunities are explicit enough for downstream roles to execute against the plan
  without guessing at work order.
mode: headings
severity: error
applies_to:
- sequencing-and-dependencies
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Sequencing Summary
- Workstream Order
- Dependency Map
- Critical Path
- Parallelization Opportunities
- Decision Gates And Spikes
- Integration, Migration, Or Rollout Checkpoints
- Verification Touchpoints
- Remaining Sequencing Risks And Assumptions
---

# Sequencing And Dependencies Check

## Purpose

Validate that sequencing, dependencies, critical path assumptions, and parallelization opportunities are explicit enough for downstream roles to execute against the plan without guessing at work order.

## Applies To

- instantiated copies of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md)
- Use after sequencing has been drafted for the major workstreams.
- Do not use as a substitute for validating the overall planning strategy or downstream handoff.

## Criteria

- The sequencing summary is explicit.
- Workstream order is clear.
- Meaningful dependencies are identified together with affected workstreams, dependency ownership where known, and failure consequences.
- The critical path is visible.
- Parallelization opportunities are explicit and bounded by clear assumptions.
- Decision gates and spikes that can change sequencing are surfaced together with ownership, default assumptions, and branch outcomes.
- Integration, migration, or rollout checkpoints are visible when they materially affect order.
- Verification touchpoints are recorded where timing matters.
- Remaining sequencing risks and assumptions are explicit.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what should happen first, what can happen in parallel, and what could reorder the work.

## Evidence Required

- The sequencing and dependencies artifact.
- The implementation strategy artifact.
- The relevant architecture handoff or other upstream constraints needed to interpret the dependency model honestly.

If the sequencing depends on implied order or hidden dependencies, fail the check.

## Supporting Skills

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when workstream order, dependency structure, or slice breakdown needs stronger planning logic.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when constraints or dependency references are scattered across local notes.

## Failure Response

- Rework the sequencing artifact before treating the implementation plan as executable.
- Preserve uncertainty and reorder risk explicitly instead of retroactively pretending the path is settled.

## Notes

This check exists because good implementation strategy without explicit sequencing still leaves too much execution ambiguity downstream.

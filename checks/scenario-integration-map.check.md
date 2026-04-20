# Scenario Integration Map Check

## Purpose

Validate that the scenario integration map makes the cross-role choreography, handoff contracts, branching rules, synchronization points, and failure-routing expectations explicit enough to support downstream adoption or execution planning.

## Applies To

- Instantiated copies of [`scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md)
- Use after scenario definition and before scenario review or downstream handoff

## Criteria

- Participating role-owned workflows are identified clearly.
- Workflow inputs, outputs, and shared artifacts are explicit enough to understand what moves between steps.
- Handoff contracts are clear enough that receiving workflows do not have to reconstruct the missing context.
- Branching rules and decision logic are visible when the scenario is not strictly linear.
- Parallelism and synchronization points are explicit when they materially affect execution.
- Shared context, state assumptions, or dependency hotspots are surfaced.
- Failure handling and escalation routing are explicit.
- Coordination risks and watchouts are visible.
- The artifact does not collapse into role-local procedure details that belong in the owning role package.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated scenario integration map artifact
- The corresponding scenario definition artifact
- The role-owned workflows or supporting materials the map references

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the integration logic needs stronger synthesis across local role packages.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when public orchestration patterns need clearer translation into repo-native scenario logic.

## Failure Response

- Rework the scenario integration map before relying on it for review or downstream use.
- Route missing role-package support or broken handoff assumptions to the correct upstream role package rather than hiding them in scenario prose.

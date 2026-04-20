# Scenario Definition Check

## Purpose

Validate that the scenario definition artifact makes the scenario's purpose, lifecycle window, participating roles, entry conditions, exit conditions, and core sequence explicit enough to guide downstream scenario work.

## Applies To

- Instantiated copies of [`scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md)
- Use before treating a scenario as ready for integration mapping, review, or downstream handoff

## Criteria

- The scenario name and intent are explicit.
- The lifecycle window and trigger conditions are clear.
- Participating roles are identified with a credible reason for inclusion.
- Entry conditions and target outputs or exit conditions are explicit.
- The core sequence is clear enough to orient downstream consumers before detailed integration mapping.
- Major decision gates or human checkpoints are visible when they materially affect the scenario.
- Scenario constraints, non-goals, and open questions are surfaced rather than hidden.
- The artifact does not collapse into a live project plan, staffing schedule, or role-definition rewrite.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated scenario definition artifact
- The supporting role definitions, role-owned workflows, or source notes used to justify it

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the scenario definition needs stronger grounding in local role-package context.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when scenario structure was borrowed from a public orchestration framework and needs clearer repo-native adaptation.

## Failure Response

- Rework the scenario definition artifact before using it as a downstream scenario anchor.
- Route missing role support, lifecycle clarity, or scenario reuse intent upstream rather than inventing scenario certainty.

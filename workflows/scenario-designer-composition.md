# Scenario Designer Composition

## Purpose

Turn a repeatable multi-role activity into an explicit scenario definition and scenario integration map that downstream consumers can use without reconstructing the role choreography from scattered role packages.

## When To Use

- A repeatable multi-role activity, phase, or operating pattern needs durable design rather than only a meeting summary or ad hoc project plan.
- Existing role packages already exist, but teams still need a composition layer that explains how those role-owned workflows fit together.
- A phase such as project planning, sprint execution, release hardening, or another reusable lifecycle segment needs explicit sequencing, handoffs, and integration requirements.

## Inputs

- Required:
  - the scenario idea, scenario name if available, and intended job-to-be-done
  - the relevant role definitions and role-owned workflows that may participate in the scenario
- Expected supporting context:
  - relevant role-package notes, artifacts, checks, and any local conventions that materially constrain the scenario
- Optional:
  - workshop notes, external orchestration references, public framework examples, or existing project plans when they help clarify the scenario shape

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md)
  - an instantiated copy of [`artifacts/scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md)
- Secondary outputs: explicit role participation, scenario sequencing, handoff contracts, branching rules, synchronization points, and failure-routing expectations

## Skills And Tools

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) as the default direct-support skill for turning a repeatable multi-role activity plus the relevant role packages into explicit scenario-definition and integration-map outputs with clear handoffs, branching logic, synchronization points, and scenario limits.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default synthesis skill for combining local role-package context, scenario intent, and any external references before writing scenario artifacts.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when tool-coupled or public orchestration patterns need to be translated into repo-native scenario structure.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when the main inputs are workshop notes, planning sessions, or rough scenario discussions that need normalization first.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md) when the scenario shape depends materially on external orchestration or process references that should be sourced explicitly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), and [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when scenario sequencing or gating depends on behavioral specification clarity.

## Sequence

1. Read the scenario idea together with the relevant role definitions, role-owned workflows, and supporting notes that could participate in the scenario.
2. If the local context or public comparison material is spread across multiple sources, use `research-documentation` first to synthesize the scenario intent, lifecycle window, likely participants, and major constraints.
3. If the strongest source pattern is tool-coupled or framework-specific, use `pattern-adaptation` to extract the reusable orchestration method before shaping repo-native scenario outputs.
4. Instantiate [`artifacts/scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md) and [`artifacts/scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md) into the project workspace if working copies do not already exist.
5. Use `scenario-composition` to populate the scenario definition artifact with scenario intent, lifecycle window, trigger conditions, participating roles, entry conditions, target outputs, exit conditions, core sequence, major gates, constraints, and open questions.
6. Use `scenario-composition` to populate the scenario integration map artifact with participating role-owned workflows, inputs and outputs, handoff contracts, branching rules, synchronization points, shared dependencies, failure-routing expectations, and coordination watchouts.
7. Run [`scenario-definition.check.md`](D:/Projects/orpheum/checks/scenario-definition.check.md), [`scenario-integration-map.check.md`](D:/Projects/orpheum/checks/scenario-integration-map.check.md), [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md), and [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).

## Decision Points

- If the scenario cannot be described without inventing missing role responsibilities, route the gap back to Role Builder or the owning role package rather than compensating inside the scenario.
- If the scenario is really just a one-off project plan rather than a reusable multi-role pattern, say so directly instead of forcing it into Scenario Designer work.
- If the scenario exposes a missing lifecycle gate, human checkpoint, or approval limit, preserve that gap explicitly rather than smoothing it away.
- If scenario sequencing depends on unresolved behavioral definition, use the existing Allium skills or route specification work upstream rather than embedding policy in scenario prose.

## Validation

- [`scenario-definition.check.md`](D:/Projects/orpheum/checks/scenario-definition.check.md) passes.
- [`scenario-integration-map.check.md`](D:/Projects/orpheum/checks/scenario-integration-map.check.md) passes.
- [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md) passes.
- [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md) passes.
- The package is ready to feed [`scenario-designer-review.md`](D:/Projects/orpheum/workflows/scenario-designer-review.md).

## Failure Handling

- Stop and ask for clarification if the scenario's lifecycle window or intended repeatability cannot be identified honestly from the available inputs.
- If checks fail, rework the earliest artifact rather than expecting downstream consumers to infer missing scenario logic.
- If scenario work reveals a role-package, specification, or approval-gap defect, route remediation there before treating the scenario package as settled.

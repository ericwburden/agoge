# Scenario Designer Handoff

## Purpose

Package reviewed scenario outputs into a downstream-ready handoff that adopters, workflow authors, project leads, or execution-planning consumers can use without reconstructing the multi-role scenario from earlier artifacts.

## When To Use

- Scenario definition, scenario integration map, and scenario review artifacts already exist.
- A downstream consumer needs a durable summary of the scenario package and current readiness posture.
- The scenario is about to move into adoption, local tailoring, or execution planning work.

## Inputs

- Required:
  - instantiated copies of [`artifacts/scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md), [`artifacts/scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md), and [`artifacts/scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md)
- Expected supporting context:
  - the corresponding role definitions, role-owned workflows, and any scenario-relevant supporting notes
- Optional:
  - local adoption notes, approval notes, or tailoring constraints

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md) in the target project workspace
- Secondary outputs: downstream-ready scenario summary, participation and routing guidance, readiness posture, watchouts, and next-step routing

## Skills And Tools

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) when the handoff still needs sharper role-and-workflow routing guidance, scenario-readiness limits, or revisit-trigger wording before generic packaging can be honest.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default direct-support skill for turning reviewed scenario outputs into a downstream-ready handoff.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the current scenario context still needs synthesis before handoff.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when local adoption depends on clarifying how a public orchestration pattern was translated into repo-native scenario structure.

## Sequence

1. Read the reviewed scenario package together with the supporting role packages and any material adoption or tailoring notes.
2. If the current scenario context is still fragmented across multiple local sources, use `research-documentation` first.
3. Instantiate [`artifacts/scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md) into the project workspace if a working copy does not already exist.
4. If the handoff still needs sharper scenario-routing or readiness wording, use `scenario-composition` first to clarify those parts of the package.
5. Use `handoff-packaging` to populate the scenario handoff artifact with current scenario summary, included package contents, current readiness posture, role and workflow routing guidance, entry conditions for the next consumer, active conditions and watchouts, follow-up owners, revisit triggers, and recommended next consumer.
6. Run [`scenario-handoff.check.md`](D:/Projects/orpheum/checks/scenario-handoff.check.md), [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md), and [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).

## Decision Points

- If the handoff is primarily for adoption or tailoring work, preserve the scenario logic without drifting into project-instance planning.
- If the handoff is primarily for human approval, keep the conditions, risks, and readiness limits visible rather than collapsing them into a soft endorsement.
- If a downstream consumer needs missing role-package support or stronger specification clarity, route that gap explicitly rather than hiding it in handoff prose.

## Validation

- [`scenario-handoff.check.md`](D:/Projects/orpheum/checks/scenario-handoff.check.md) passes.
- [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md) passes.
- [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md) passes.
- The package is ready for downstream adoption, tailoring, or execution-planning use.

## Failure Handling

- Stop and ask for clarification if the next consumer cannot be identified honestly from the available package.
- If packaging clarity is the main weakness, rework the handoff artifact rather than assuming downstream consumers will infer the missing structure.
- If the defect began earlier in the chain, route remediation there before treating the handoff as complete.

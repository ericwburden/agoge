# Scenario Designer Review

## Purpose

Review the drafted scenario package, record the durable scenario posture, and decide whether the package is ready, conditional, or blocked for downstream adoption, tailoring, or execution use.

## When To Use

- Scenario definition and scenario integration map artifacts already exist.
- A downstream consumer needs an explicit scenario posture rather than only the design artifacts.
- The scenario package includes mixed role support, unresolved coordination risk, or conditional gates that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md)
  - an instantiated copy of [`artifacts/scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md)
- Expected supporting context:
  - the corresponding role definitions, role-owned workflows, and any local notes or references that materially constrain the scenario
- Optional:
  - workshop notes, approval notes, or public comparison material that informed the design

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md) in the target project workspace
- Secondary outputs: explicit scenario posture, grouped integration risks, remediation decisions, conditions, and downstream watchouts

## Skills And Tools

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) as the default direct-support skill for turning drafted scenario-definition and integration-map outputs into an explicit scenario posture with clear readiness, conditions, integration risks, and revisit triggers.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing role-package context, scenario risks, and supporting evidence before writing the review.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when the review needs to clarify whether a borrowed orchestration pattern was adapted cleanly into repo-native scenario form.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when review inputs include rough decision notes or workshop output that need normalization first.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when the review reveals a material specification gap that should be refined rather than improvised in scenario gating.

## Sequence

1. Read the scenario definition and scenario integration map together with the relevant role packages and any material supporting notes or references.
2. If role-package context, risks, or evidence references still need synthesis before the decision can be stated honestly, use `research-documentation` first.
3. Instantiate [`artifacts/scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md) into the project workspace if a working copy does not already exist.
4. Use `scenario-composition` to populate the scenario review artifact with review scope, reviewed inputs, overall assessment, explicit decision status, integration risks and failure modes, remediation decisions, follow-up owners, revisit triggers, and recommended next step.
5. Run [`scenario-review.check.md`](D:/Projects/orpheum/checks/scenario-review.check.md), [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md), and [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).

## Decision Points

- If the package is blocked because role support is materially incomplete, state that directly rather than softening it into scenario optimism.
- If the package is conditional, make the condition and owner explicit rather than leaving them implied in the integration map.
- If the issue is really a role-package gap, approval gap, or specification gap rather than a scenario-packaging defect, preserve that distinction in the review.
- If the scenario is reusable in one context but not another, state those limits explicitly rather than implying universal readiness.

## Validation

- [`scenario-review.check.md`](D:/Projects/orpheum/checks/scenario-review.check.md) passes.
- [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md) passes.
- [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md) passes.
- The package is ready to feed [`scenario-designer-handoff.md`](D:/Projects/orpheum/workflows/scenario-designer-handoff.md).

## Failure Handling

- Stop and ask for clarification if the scenario posture cannot be stated honestly from the available package.
- If the review check fails, rework the review instead of asking downstream consumers to infer the real readiness state.
- If traceability or boundary checks fail, route remediation to the earliest artifact, role package, or design decision that introduced the defect.

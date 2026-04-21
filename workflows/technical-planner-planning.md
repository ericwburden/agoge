# Technical Planner Planning

## Purpose

Turn reviewed architecture and validated requirements into an implementation strategy and explicit sequencing plan.

## When To Use

- Architecture direction is stable enough to support execution planning.
- Downstream implementation needs workstream structure, slice order, and dependency handling before coding can proceed safely.
- The work depends on making implementation slices, enabling work, sequencing, or decision gates explicit.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md)
  - an instantiated copy of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md)
- Optional:
  - instantiated copies of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md), [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md), and [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md)
  - instantiated copies of [`artifacts/security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md), [`artifacts/security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md), [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md), or [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md) when obligations, controls, or approval-sensitive constraints materially shape implementation planning
  - Allium specifications or other behavioral specifications when they already exist
  - planning workshop notes
  - dependency notes, rollout constraints, or integration references

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md) in the target project workspace
  - an instantiated copy of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md) in the target project workspace
- Secondary outputs: explicit slice strategy, workstream boundaries, critical path assumptions, dependency hotspots, enabling work, and planning risks

## Skills And Tools

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) as the default path for structuring the implementation strategy and execution order.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when planning workshop notes or transcripts need normalization.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when local planning context, dependency notes, or architecture references need synthesis before the plan can be stated cleanly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external platform constraints, migration patterns, or standards materially affect sequencing decisions.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), and [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when existing behavioral specifications materially constrain slicing or verification planning.

## Sequence

1. Read the architecture handoff and requirements handoff together, using reviewed architecture artifacts, requirements specification, optional security/compliance artifacts, business context, and existing behavioral specs as needed to clarify planning drivers.
2. If planning notes or workshop transcripts exist, normalize them with `meeting-notes-and-actions` before drafting.
3. If planning or dependency context is spread across multiple local files, use `research-documentation` to synthesize the relevant constraints and reference points.
4. Instantiate [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md) and [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md) into the project workspace if working copies do not already exist.
5. Use `spec-to-implementation` to populate the implementation strategy artifact with planning scope, input context, a slice-level traceability map, assumptions, implementation approach, slice strategy, workstream overview, enabling work, slice exit criteria, readiness conditions with owners, verification and rollout considerations, deferred or intentionally excluded work, and planning risks.
6. Use `spec-to-implementation` to populate the sequencing and dependencies artifact with workstream order, a dependency map that includes ownership and failure consequences where known, critical path, parallelization opportunities, and decision gates that record owner, default assumption, and branch outcomes, plus integration or migration checkpoints, verification touchpoints, and sequencing risks.
7. Run [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md), [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md), [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md), and [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md).

## Decision Points

- If upstream BA or architecture artifacts are still materially ambiguous, record the gap and route it upstream instead of solving it silently in the plan.
- If security/compliance constraints materially shape sequencing, readiness, or decision gates, preserve those inputs explicitly instead of letting planning absorb them as unnamed assumptions.
- If multiple plausible slice strategies exist, record the alternatives and planning drivers rather than defaulting to one without explanation.
- If important sequencing depends on external approvals, migration constraints, or unresolved interface decisions, make those dependencies explicit here instead of leaving them for downstream roles to infer.
- If important sequencing depends on unresolved security/compliance obligations, evidence expectations, or approval conditions, route that ambiguity back to Security / Compliance Specialist work rather than improvising a permanent answer inside the plan.
- If the system includes AI-enabled or agentic behavior, record trust-boundary-sensitive sequencing and human control points explicitly.

## Validation

- [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md) passes.
- [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md) passes.
- [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) passes.
- [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md) passes.
- The instantiated outputs are ready to feed [`technical-planner-review.md`](D:/Projects/agoge/workflows/technical-planner-review.md).

## Failure Handling

- Stop and ask for clarification if the plan cannot be stated without inventing missing requirements or architecture assumptions.
- Do not collapse unresolved sequencing tradeoffs into fake certainty.
- If a traceability or boundary check fails, route remediation to the earliest BA, architecture, or planning artifact that introduced the defect.

## Notes

This is the default entry workflow for Technical Planner work.

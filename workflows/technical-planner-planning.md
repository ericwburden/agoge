---
id: technical-planner-planning
kind: workflow
title: Technical Planner Planning
version: 1
summary: Turn reviewed architecture and validated requirements into an implementation
  strategy and explicit sequencing plan.
role: technical-planner
inputs:
- architecture-handoff
- requirements-handoff
- solution-architecture
- architecture-decisions
- requirements-specification
- security-compliance-handoff
- security-compliance-review
- security-compliance-scope
- controls-and-evidence-matrix
outputs:
- implementation-strategy
- definition-of-done
- sequencing-and-dependencies
skills:
- spec-to-implementation
- requirements-verification
- meeting-notes-and-actions
- research-documentation
- content-research-writer
checks:
- implementation-strategy
- definition-of-done
- sequencing-and-dependencies
- planning-traceability
- technical-planner-boundary
handoff_to: []
---

# Technical Planner Planning

## Purpose

Turn reviewed architecture and validated requirements into an implementation strategy, a project Definition of Done, and an explicit sequencing plan.

## When To Use

- Architecture direction is stable enough to support execution planning.
- Downstream implementation needs workstream structure, slice order, and dependency handling before coding can proceed safely.
- Downstream implementation, review, and verification need a standing project-level completion rule before slice execution begins.
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
  - an instantiated copy of [`artifacts/definition-of-done.md`](D:/Projects/orpheum/artifacts/definition-of-done.md) in the target project workspace
  - an instantiated copy of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md) in the target project workspace
- Secondary outputs: explicit slice strategy, project-level done rules, workstream boundaries, critical path assumptions, dependency hotspots, enabling work, and planning risks

## Skills And Tools

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) as the default path for structuring the implementation strategy and execution order.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when defining the Definition of Done from validated requirements, acceptance commitments, evidence expectations, or behavior-sensitive constraints.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when planning workshop notes or transcripts need normalization.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when local planning context, dependency notes, or architecture references need synthesis before the plan can be stated cleanly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external platform constraints, migration patterns, or standards materially affect sequencing decisions.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), and [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when existing behavioral specifications materially constrain slicing or verification planning.

## Sequence

1. Read the architecture handoff and requirements handoff together, using reviewed architecture artifacts, requirements specification, optional security/compliance artifacts, business context, and existing behavioral specs as needed to clarify planning drivers.
2. If planning notes or workshop transcripts exist, normalize them with `meeting-notes-and-actions` before drafting.
3. If planning or dependency context is spread across multiple local files, use `research-documentation` to synthesize the relevant constraints and reference points.
4. Instantiate [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md), [`artifacts/definition-of-done.md`](D:/Projects/orpheum/artifacts/definition-of-done.md), and [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md) into the project workspace if working copies do not already exist.
5. Treat broader product and architecture artifacts as inherited constraints, then use planning artifacts as the place where the current slice becomes operationally explicit while the Definition of Done preserves the standing project-level completion rule.
6. Use `requirements-verification` to derive and verify the Definition of Done from validated requirements, acceptance commitments, architecture constraints, expected review posture, verification expectations, documentation needs, and any materially constraining security/compliance guidance.
7. Populate the Definition of Done artifact with scope and applicability, source rationale, the project-level completion standard, required implementation, review, verification, and handoff conditions, conditional security/compliance expectations, release-readiness relationship, slice-level tailoring rules, explicit exclusions, and change-control expectations.
8. Use `spec-to-implementation` to populate the implementation strategy artifact with planning scope, input context, a slice-level traceability map, assumptions, implementation approach, slice strategy, workstream overview, enabling work, slice exit criteria, readiness conditions with owners, verification and rollout considerations, deferred or intentionally excluded work, and planning risks. Make slice exit criteria explicit as slice-local additions to the project Definition of Done rather than silent replacements for it.
9. Use `spec-to-implementation` to populate the sequencing and dependencies artifact with workstream order, a dependency map that includes ownership and failure consequences where known, critical path, parallelization opportunities, and decision gates that record owner, default assumption, and branch outcomes, plus integration or migration checkpoints, verification touchpoints, and sequencing risks.
10. Run [`definition-of-done.check.md`](D:/Projects/orpheum/checks/definition-of-done.check.md), [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md), [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md), [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md), and [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md).

## Decision Points

- If upstream BA or architecture artifacts are still materially ambiguous, record the gap and route it upstream instead of solving it silently in the plan.
- If the Definition of Done cannot be stated without inventing missing acceptance, evidence, or approval assumptions, route that ambiguity upstream instead of disguising it as planning certainty.
- If security/compliance constraints materially shape sequencing, readiness, or decision gates, preserve those inputs explicitly instead of letting planning absorb them as unnamed assumptions.
- If multiple plausible slice strategies exist, record the alternatives and planning drivers rather than defaulting to one without explanation.
- If a statement is only true of the current slice, keep it in implementation strategy or sequencing artifacts rather than pushing it back up into enduring product or architecture artifacts.
- If a completion rule should apply project-wide, keep it in the Definition of Done rather than scattering it across slice exit criteria, review notes, or verification guidance.
- If important sequencing depends on external approvals, migration constraints, or unresolved interface decisions, make those dependencies explicit here instead of leaving them for downstream roles to infer.
- If important sequencing depends on unresolved security/compliance obligations, evidence expectations, or approval conditions, route that ambiguity back to Security / Compliance Specialist work rather than improvising a permanent answer inside the plan.
- If the system includes AI-enabled or agentic behavior, record trust-boundary-sensitive sequencing and human control points explicitly.

## Validation

- [`definition-of-done.check.md`](D:/Projects/orpheum/checks/definition-of-done.check.md) passes.
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

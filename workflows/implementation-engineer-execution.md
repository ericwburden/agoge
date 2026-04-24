---
id: implementation-engineer-execution
kind: workflow
title: Implementation Engineer Execution
version: 1
summary: Turn reviewed planning, Definition of Done, and architecture handoff into
  a concrete implementation record and implementation evidence package.
role: implementation-engineer
inputs:
- implementation-handoff
- definition-of-done
- architecture-handoff
- requirements-handoff
- implementation-strategy
- sequencing-and-dependencies
- solution-architecture
- architecture-decisions
outputs:
- implementation-record
- implementation-evidence
skills:
- implementation-package-prep
- research-documentation
- meeting-notes-and-actions
- webapp-testing
- content-research-writer
checks:
- implementation-record
- implementation-evidence
- implementation-traceability
- implementation-engineer-boundary
handoff_to: []
---

# Implementation Engineer Execution

## Purpose

Turn reviewed planning and architecture handoff into a concrete implementation record and implementation evidence package.

## When To Use

- Reviewed implementation direction is stable enough to support coding work.
- Downstream review or verification will need more than a raw diff or commit summary to understand the implementation.
- The work benefits from making implementation scope, changed areas, deviations, and local validation evidence explicit.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-handoff.md`](D:/Projects/orpheum/artifacts/implementation-handoff.md)
  - an instantiated copy of [`artifacts/definition-of-done.md`](D:/Projects/orpheum/artifacts/definition-of-done.md)
  - an instantiated copy of [`artifacts/architecture-handoff.md`](D:/Projects/orpheum/artifacts/architecture-handoff.md)
  - an instantiated copy of [`artifacts/requirements-handoff.md`](D:/Projects/orpheum/artifacts/requirements-handoff.md)
- Optional:
  - instantiated copies of [`artifacts/implementation-strategy.md`](D:/Projects/orpheum/artifacts/implementation-strategy.md), [`artifacts/sequencing-and-dependencies.md`](D:/Projects/orpheum/artifacts/sequencing-and-dependencies.md), [`artifacts/solution-architecture.md`](D:/Projects/orpheum/artifacts/solution-architecture.md), and [`artifacts/architecture-decisions.md`](D:/Projects/orpheum/artifacts/architecture-decisions.md)
  - Allium specifications or other behavioral specifications when they already exist
  - implementation working notes, defect notes, environment notes, or local run outputs

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md) in the target project workspace
  - an instantiated copy of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md) in the target project workspace
- Secondary outputs: explicit implementation boundary, changed-area summary, deviation notes, and local validation evidence

## Skills And Tools

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) as the default direct-support skill for turning implemented work into an implementation record and evidence package.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing local planning, architecture, requirements, and implementation context before or after coding.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when implementation notes or working-session transcripts need normalization.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when the implemented slice includes browser-based or web-application behavior and local evidence needs to be collected.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md) when external platform references, standards, or integration constraints materially affect the implementation.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when existing behavioral specifications materially constrain the implementation or local drift checking.

## Sequence

1. Read the implementation handoff, Definition of Done, architecture handoff, and requirements handoff together, using reviewed planning and architecture artifacts plus existing behavioral specifications as needed to clarify the approved implementation boundary and the completion standard this slice must preserve.
2. If implementation notes or working-session transcripts exist, normalize them with `meeting-notes-and-actions` before drafting the implementation package.
3. If the local implementation context is spread across multiple artifacts, use `research-documentation` to synthesize the relevant scope, constraints, dependencies, and change rationale.
4. Instantiate [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md) and [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md) into the project workspace if working copies do not already exist.
5. Implement the approved slice in code, keeping actual scope, changed areas, deviations, and affected interfaces explicit as the change set takes shape.
6. Use `implementation-package-prep` to populate the implementation record artifact with implementation scope, input context, a change-level traceability map, target slice boundary, planned-versus-actual scope, Definition-of-Done alignment, change summary, concrete change inventory, affected areas, interface or contract effects, deviations from plan or specification, blockers, and deferred work.
7. Run local validation that is appropriate to the implemented slice, using `webapp-testing` when browser-based or web-application evidence is materially important and applicable.
8. Use `implementation-package-prep` to populate the implementation evidence artifact with evidence scope, provenance, validation activities including commands or manual procedures when relevant, observed results, known failures and skipped checks, manual verification notes, supporting references, evidence gaps, and revalidation watchouts.
10. Run [`implementation-record.check.md`](D:/Projects/orpheum/checks/implementation-record.check.md), [`implementation-evidence.check.md`](D:/Projects/orpheum/checks/implementation-evidence.check.md), [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md), and [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).

## Decision Points

- If requirement, architecture, planning, or specification ambiguity makes the implementation boundary unstable, record the gap and route it upstream instead of solving it silently in code.
- If the actual implementation must deviate materially from the reviewed plan, make that deviation explicit here rather than letting downstream roles infer it from diffs.
- If the slice cannot satisfy a standing Definition of Done condition, make that gap explicit rather than implying the slice is done because coding stopped.
- If local validation is unavailable, partial, or environment-bound, capture that limitation honestly instead of implying stronger confidence.
- If the system includes AI-enabled or agentic behavior, record trust-boundary-sensitive implementation details and human control points explicitly.

## Validation

- [`implementation-record.check.md`](D:/Projects/orpheum/checks/implementation-record.check.md) passes.
- [`implementation-evidence.check.md`](D:/Projects/orpheum/checks/implementation-evidence.check.md) passes.
- [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md) passes.
- [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md) passes.
- The instantiated outputs are ready to feed [`implementation-engineer-review.md`](D:/Projects/orpheum/workflows/implementation-engineer-review.md).

## Failure Handling

- Stop and ask for clarification if the implementation cannot proceed without inventing missing requirements, architecture decisions, or planning assumptions.
- Do not collapse skipped checks, mixed evidence, or material deviations into a fake-success implementation package.
- If a traceability or boundary check fails, route remediation to the earliest requirements, architecture, planning, specification, or implementation artifact that introduced the defect.

## Notes

This is the default entry workflow for Implementation Engineer work.

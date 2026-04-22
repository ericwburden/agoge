---
id: qa-verification-planning
kind: workflow
title: QA / Verification Planning
version: 1
summary: Turn reviewed requirements, architecture, planning, and implementation context
  into a verification strategy and explicit coverage matrix.
role: null
inputs:
- requirements-handoff
- architecture-handoff
- implementation-handoff
- requirements-specification
- solution-architecture
- implementation-strategy
- sequencing-and-dependencies
outputs:
- verification-strategy
- verification-matrix
skills:
- requirements-verification
- research-documentation
- meeting-notes-and-actions
- content-research-writer
checks:
- verification-strategy
- verification-matrix
- verification-traceability
- qa-verification-boundary
handoff_to: []
---

# QA / Verification Planning

## Purpose

Turn reviewed requirements, architecture, planning, and implementation context into a verification strategy and explicit coverage matrix.

## When To Use

- Downstream work needs an explicit verification posture before evidence review or release-adjacent decisions.
- Implementation progress exists, but confidence expectations, evidence requirements, or coverage expectations are still implicit.
- Verification needs to map requirements, architecture, planning hotspots, and implementation slices to expected or observed evidence.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md)
  - an instantiated copy of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md)
  - an instantiated copy of [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)
- Optional:
  - instantiated copies of [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md), [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md), [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md), or [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md)
  - implementation evidence summaries, test notes, walkthrough notes, defect notes, or environment notes
  - Allium specifications or other behavioral specifications when they already exist

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md) in the target project workspace
  - an instantiated copy of [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md) in the target project workspace
- Secondary outputs: explicit verification scope, confidence goals, risk focus, expected evidence signals, coverage state, and identified evidence gaps or hotspots

## Skills And Tools

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) as the default path for grounding verification scope, acceptance expectations, and coverage priorities in upstream evidence.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when requirements, architecture, planning, or implementation context is spread across multiple local files and needs synthesis before the verification package can be stated cleanly.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when walkthrough notes, test-session notes, or workshop transcripts need normalization.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external standards, compliance references, or platform verification expectations materially affect the verification approach.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md), and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when existing behavioral specifications materially constrain verification expectations or evidence interpretation.

## Sequence

1. Read the requirements handoff, architecture handoff, and implementation handoff together, using deeper upstream artifacts as needed to judge verification scope honestly.
2. If implementation evidence or walkthrough notes exist, normalize them with `meeting-notes-and-actions` when needed.
3. If verification context is spread across multiple local files, use `research-documentation` to synthesize the relevant requirements, architecture, planning, implementation, and evidence drivers.
4. Instantiate [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md) and [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md) into the project workspace if working copies do not already exist.
5. Use `requirements-verification` together with the verification artifacts to populate the verification strategy with scope, input context, verification drivers and risks, confidence goals, verification methods, evidence expectations, scope exclusions, constraints, watchouts, readiness framing, and open questions.
6. Use the same verified source context to populate the verification matrix with the coverage map, hotspot summary, contradictions and weak signals, deferred coverage, and upstream routing notes.
7. Run [`verification-strategy.check.md`](D:/Projects/agoge/checks/verification-strategy.check.md), [`verification-matrix.check.md`](D:/Projects/agoge/checks/verification-matrix.check.md), [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md), and [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md).

## Decision Points

- If the verification package cannot be stated without inventing missing requirement, architecture, planning, or implementation assumptions, route the gap upstream instead of hiding it in the verification plan.
- If evidence expectations materially exceed what the current environment or tooling can support, make that constraint explicit rather than pretending the coverage is already achievable.
- If trust-boundary or rollout-sensitive behavior exists, make the related verification watchouts explicit even if the rest of the package is straightforward.

## Validation

- [`verification-strategy.check.md`](D:/Projects/agoge/checks/verification-strategy.check.md) passes.
- [`verification-matrix.check.md`](D:/Projects/agoge/checks/verification-matrix.check.md) passes.
- [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) passes.
- [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md) passes.
- The instantiated outputs are ready to feed [`qa-verification-review.md`](D:/Projects/agoge/workflows/qa-verification-review.md).

## Failure Handling

- Stop and ask for clarification if the verification package cannot be stated honestly from the available artifacts and evidence.
- Do not collapse missing evidence or contradictory signals into fake coverage.
- If a traceability or boundary check fails, route remediation to the earliest requirements, architecture, planning, implementation, or verification artifact that introduced the defect.

## Notes

This is the default entry workflow for QA / Verification Lead work.

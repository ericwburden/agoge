---
id: implementation-engineer-handoff
kind: workflow
title: Implementation Engineer Handoff
version: 1
summary: Turn reviewed implementation outputs into a downstream-ready implementation
  package handoff for code review, verification, and release-adjacent roles.
role: implementation-engineer
inputs:
- implementation-record
- implementation-evidence
- implementation-readiness-review
outputs:
- implementation-package-handoff
skills:
- implementation-package-prep
- handoff-packaging
- research-documentation
checks:
- implementation-package-handoff
- implementation-traceability
- implementation-engineer-boundary
handoff_to: []
---

# Implementation Engineer Handoff

## Purpose

Turn reviewed implementation outputs into a downstream-ready implementation package handoff for code review, verification, and release-adjacent roles.

## When To Use

- Implementation direction and evidence are stable enough to package downstream.
- Downstream roles need a clean implementation handoff rather than scattered code notes and local run output.
- The implementation package includes context, evidence, and risks that should travel together.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
  - an instantiated copy of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
  - an instantiated copy of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
- Optional: upstream implementation, architecture, or requirements handoff artifacts; specification references; defect notes; and supporting review notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md) in the target project workspace
- Secondary outputs: highlighted change footprint, evidence posture, known issues, revalidation triggers, and downstream decision watchouts

## Skills And Tools

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) as the default direct-support skill for preparing the downstream implementation package handoff from reviewed implementation artifacts.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default path for packaging reviewed implementation outputs for downstream roles.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when implementation context, evidence posture, or cross-artifact links need synthesis before the handoff can be written cleanly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the handoff must explain how existing behavioral specifications constrain or qualify the implementation.

## Sequence

1. Read the implementation record, implementation evidence, and implementation readiness review artifacts together.
2. If supporting notes or implementation references still need synthesis, use `research-documentation` first.
3. Instantiate [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `implementation-package-prep` first and `handoff-packaging` as needed to populate the implementation package handoff artifact with the implemented scope summary, change footprint summary, evidence posture summary, review status and key findings, known issues and residual risks, specification relationship, explicit revalidation triggers, downstream consumers, and next decision points.
5. Run [`implementation-package-handoff.check.md`](D:/Projects/orpheum/checks/implementation-package-handoff.check.md), [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md), and [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).

## Decision Points

- If the readiness review status is blocked or materially conditional, keep implementation work open instead of packaging a misleading handoff.
- If unresolved requirement, architecture, planning, or specification ambiguity still shapes the implementation package materially, route that ambiguity upstream rather than hiding it in the handoff.
- If the current implementation package would become misleading under foreseeable code, environment, or dependency changes, make those revalidation triggers explicit rather than leaving them to downstream inference.
- If the handoff starts turning into a bug tracker, code review artifact, or release script, remove that drift and leave those concerns to downstream roles.

## Validation

- [`implementation-package-handoff.check.md`](D:/Projects/orpheum/checks/implementation-package-handoff.check.md) passes.
- [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md) passes.
- [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md) passes.
- The handoff is ready to feed downstream code review, verification, or release-adjacent work without rediscovery.

## Failure Handling

- Stop and ask for clarification if the implementation package cannot be handed off honestly from the available artifacts.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to reconstruct missing implementation context themselves.
- If a traceability or boundary check fails, route remediation to the earliest requirements, architecture, planning, specification, or implementation artifact that introduced the defect.

## Notes

This workflow packages implementation for downstream use without becoming a review or release artifact.

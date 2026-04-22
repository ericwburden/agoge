---
id: implementation-engineer-review
kind: workflow
title: Implementation Engineer Review
version: 1
summary: Review the drafted implementation package, record findings, and decide whether
  it is ready for downstream code review, verification, or release-adjacent use.
role: implementation-engineer
inputs:
- implementation-record
- implementation-evidence
- implementation-handoff
- architecture-handoff
- requirements-handoff
outputs:
- implementation-readiness-review
skills:
- implementation-package-prep
- research-documentation
- meeting-notes-and-actions
- handoff-packaging
checks:
- implementation-readiness-review
- implementation-traceability
- implementation-engineer-boundary
handoff_to: []
---

# Implementation Engineer Review

## Purpose

Review the drafted implementation package, record findings, and decide whether it is ready for downstream code review, verification, or release-adjacent use.

This workflow is an implementation-package self-review step, not a substitute for independent downstream review.

## When To Use

- An implementation record and implementation evidence artifact already exist.
- Downstream roles need an explicit readiness judgment rather than a raw implementation package.
- The implementation package includes mixed evidence, deviations, or upstream routing questions that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
  - an instantiated copy of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
- Optional:
  - instantiated copies of [`artifacts/implementation-handoff.md`](D:/Projects/orpheum/artifacts/implementation-handoff.md), [`artifacts/architecture-handoff.md`](D:/Projects/orpheum/artifacts/architecture-handoff.md), and [`artifacts/requirements-handoff.md`](D:/Projects/orpheum/artifacts/requirements-handoff.md)
  - supporting notes, run outputs, defect notes, or specification references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md) in the target project workspace
- Secondary outputs: explicit readiness decision, grouped findings, remediation conditions, upstream routing notes, and downstream cautions

## Skills And Tools

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) as the default direct-support skill for preparing the implementation-package self-review from the implementation record and evidence package.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing implementation context and evidence before writing the readiness review.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when review notes or implementation-session notes need normalization.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when findings and readiness conditions are clear but not yet expressed cleanly for downstream use.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when readiness depends materially on spec-to-code alignment concerns.

## Sequence

1. Read the implementation record and implementation evidence together, pulling in upstream handoffs and specification references as needed to interpret the package honestly.
2. If implementation context, run outputs, or review notes are spread across multiple sources, use `research-documentation` to synthesize the relevant findings before writing the readiness judgment.
3. Instantiate [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md) into the project workspace if a working copy does not already exist.
4. Use `implementation-package-prep` to populate the readiness review artifact with review scope, inputs reviewed, a clear implementation-package readiness decision, grouped findings, remediation and required conditions, residual risks, upstream routing notes, and a recommendation for downstream use.
5. Run [`implementation-readiness-review.check.md`](D:/Projects/orpheum/checks/implementation-readiness-review.check.md), [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md), and [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).

## Decision Points

- If the implementation package is materially incomplete or misleading, keep implementation work open instead of writing a soft-ready review.
- If a finding is really an upstream requirement, architecture, planning, or specification defect, route it explicitly rather than treating it as a downstream cleanup task.
- If the package is conditionally ready, make the conditions and owners explicit rather than leaving them implied.
- If the review starts turning into independent code review or QA verdict language, remove that drift and keep the artifact at implementation-package readiness level.

## Validation

- [`implementation-readiness-review.check.md`](D:/Projects/orpheum/checks/implementation-readiness-review.check.md) passes.
- [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md) passes.
- [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md) passes.
- The review is ready to feed [`implementation-engineer-handoff.md`](D:/Projects/orpheum/workflows/implementation-engineer-handoff.md).

## Failure Handling

- Stop and ask for clarification if the readiness judgment cannot be made honestly from the available implementation package.
- If the readiness-review check fails, rework the review rather than expecting downstream roles to infer the missing logic.
- If a traceability or boundary check fails, route remediation to the earliest requirements, architecture, planning, specification, or implementation artifact that introduced the defect.

## Notes

This workflow creates the implementation-package decision gate before downstream handoff.

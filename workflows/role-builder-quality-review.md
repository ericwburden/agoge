---
id: role-builder-quality-review
kind: workflow
title: Role Builder Quality Review
version: 1
summary: Review the full role package for coherence, record findings and remediation
  decisions, and determine whether the role is ready to move into adoption handoff
  or still needs rework.
role: role-builder
inputs:
- role-definition-brief
- role-support-system
outputs:
- role-package-review
skills:
- role-package-review
- research-documentation
checks:
- role-package-review
- role-package-traceability
- role-builder-boundary
handoff_to: []
---

# Role Builder Quality Review

## Purpose

Review the full role package for coherence, record findings and remediation decisions, and determine whether the role is ready to move into adoption handoff or still needs rework.

This workflow is the explicit implementation of the `Role Builder hardening pass`.

## When To Use

- A role definition and support system both exist and need a readiness review.
- A role package has been expanded and should be checked for coherence drift.
- A downstream adopter needs confidence that the role package is usable without hidden conventions.
- The user explicitly asks for a `Role Builder hardening pass`.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md)
  - an instantiated copy of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md)
- Optional: supporting notes, comparison material, and prior review findings

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md) in the target working location
- Secondary outputs: explicit findings, remediation routing, and role-package readiness status

## Skills And Tools

- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) as the preferred default path for reviewing role-package coherence, recording findings, and producing a defensible readiness assessment.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) only when findings still depend on synthesizing multiple local sources or prior notes before the review can be completed honestly.

## Sequence

1. Read the instantiated role definition brief and support-system artifact together.
2. Instantiate [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md) into the working location if a working copy does not already exist.
3. Use `role-package-review` to review the package for coherence, missing support, duplicated structure, broken traceability, and role drift.
4. Record findings, remediation decisions, readiness, remaining risks, and next steps in the instantiated review artifact.
5. Run [`role-package-review.check.md`](D:/Projects/agoge/checks/role-package-review.check.md), [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md), and [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md).
6. If the package is ready or conditionally ready with explicit limits, route it to [`role-builder-handoff.md`](D:/Projects/agoge/workflows/role-builder-handoff.md) for adoption packaging.

## Decision Points

- If the package is missing required support elements, mark it not ready rather than hand-waving the gap.
- If failures all trace back to the role definition, route remediation there instead of patching the review artifact.
- If the role package is coherent but still intentionally incomplete, mark it conditionally ready and state the remaining limits explicitly.

## Validation

- [`role-package-review.check.md`](D:/Projects/agoge/checks/role-package-review.check.md) passes.
- [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) passes.
- [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md) passes.
- The package's readiness state is explicit, defensible, and usable as the basis for adoption handoff.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly from the available artifacts.
- If the review check fails, rework the review with `role-package-review` and use `research-documentation` if findings still depend on unsynthesized local context.
- If traceability or boundary checks fail, route remediation to the earliest artifact or design decision that introduced the defect.

## Notes

This workflow is the final quality gate before adoption handoff for a reusable role package.

In this repository, `Role Builder hardening pass` is the short trigger phrase for invoking this workflow and its required checks.

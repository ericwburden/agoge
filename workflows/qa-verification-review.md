---
id: qa-verification-review
kind: workflow
title: QA / Verification Review
version: 1
summary: Review the drafted verification package and the available implementation
  evidence, record findings and remediation decisions, and determine whether the verification
  package is ready to move into downstream handoff.
role: null
inputs:
- verification-strategy
- verification-matrix
- implementation-handoff
outputs:
- evidence-review
skills:
- requirements-verification
- research-documentation
checks:
- evidence-review
- verification-traceability
- qa-verification-boundary
handoff_to: []
---

# QA / Verification Review

## Purpose

Review the drafted verification package and the available implementation evidence, record findings and remediation decisions, and determine whether the verification package is ready to move into downstream handoff.

## When To Use

- A verification strategy and verification matrix both exist and need a readiness review.
- Available implementation evidence needs a durable assessment before downstream release-adjacent work.
- A downstream consumer needs confidence that the verification package is usable without hidden conventions or missing caveats.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md)
  - an instantiated copy of [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md)
  - the corresponding instantiated [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)
  - at least one concrete evidence source, such as test results, walkthrough notes, logs, screenshots, defect summaries, or equivalent verification evidence
- Expected supporting context:
  - the corresponding instantiated requirements handoff and architecture handoff artifacts
- Optional: additional implementation evidence notes, prior review findings, and supporting local references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md) in the target working location
- Secondary outputs: explicit findings, evidence-strength judgments, remediation routing, readiness status, and identified residual risks

## Skills And Tools

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) as the preferred default path for tightening requirement-grounded verification logic when the review exposes weak acceptance framing or unsupported coverage claims.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when findings still depend on synthesizing multiple local sources or evidence references before the review can be completed honestly.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the review reveals specification-to-implementation drift that materially affects readiness.

## Sequence

1. Read the instantiated verification strategy, verification matrix, implementation handoff, and at least one concrete evidence source together with the requirements and architecture handoffs.
2. If local evidence or notes still need synthesis before the review can be written clearly, use `research-documentation` first.
3. Instantiate [`artifacts/evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md) into the working location if a working copy does not already exist.
4. Review the package for evidence strength, evidence provenance and freshness, missing support, contradictory signals, unsupported coverage claims, weak readiness framing, role drift, and upstream routing needs.
5. Record findings, evidence provenance, evidence strength and gaps, readiness, remaining risks, remediation routing, and next steps in the instantiated review artifact.
6. Run [`evidence-review.check.md`](D:/Projects/agoge/checks/evidence-review.check.md), [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md), and [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md).
7. If the package is ready or conditionally ready with explicit limits, route it to [`qa-verification-handoff.md`](D:/Projects/agoge/workflows/qa-verification-handoff.md) for downstream packaging.

## Decision Points

- If the package is missing required verification structure or honest evidence support, mark it not ready rather than smoothing over the gap.
- If no concrete evidence source exists for the scope under review, stop and route the gap to evidence generation or a narrower verification scope instead of producing a fake readiness judgment.
- If failures all trace back to the verification strategy or matrix, route remediation there instead of patching the review artifact.
- If the review reveals an upstream requirement, architecture, planning, or implementation defect, route remediation upstream rather than silently correcting it in the verification package.

## Validation

- [`evidence-review.check.md`](D:/Projects/agoge/checks/evidence-review.check.md) passes.
- [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) passes.
- [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md) passes.
- The package's readiness state is explicit, defensible, and usable as the basis for verification handoff.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly from the available artifacts or evidence.
- If the review check fails, rework the review and use `research-documentation` if findings still depend on unsynthesized local context.
- If traceability or boundary checks fail, route remediation to the earliest artifact, evidence source, or upstream role decision that introduced the defect.

## Notes

This workflow is the final substantive review before downstream verification handoff for the QA / Verification Lead package.

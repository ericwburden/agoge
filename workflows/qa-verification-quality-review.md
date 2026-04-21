---
id: qa-verification-quality-review
kind: workflow
title: QA / Verification Quality Review
version: 1
summary: Apply the QA / Verification Lead check chain to instantiated verification
  artifacts, identify failures by artifact and defect type, and route remediation
  before downstream release-adjacent or review work begins.
role: null
inputs:
- verification-handoff
- requirements-handoff
- architecture-handoff
- implementation-handoff
outputs: []
skills:
- requirements-verification
- handoff-packaging
- research-documentation
- meeting-notes-and-actions
checks:
- verification-traceability
- qa-verification-boundary
- verification-strategy
- verification-matrix
- evidence-review
- verification-handoff
handoff_to: []
---

# QA / Verification Quality Review

## Purpose

Apply the QA / Verification Lead check chain to instantiated verification artifacts, identify failures by artifact and defect type, and route remediation before downstream release-adjacent or review work begins.

## When To Use

- The verification handoff has been drafted and the full QA / Verification Lead artifact chain exists.
- A downstream consumer needs final definition-of-done confidence that the full verification chain is self-sufficient and within role boundaries.
- A check has failed and the work needs a standard rework path instead of ad hoc editing.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md)
  - the corresponding instantiated verification strategy, verification matrix, and evidence review artifacts
  - the upstream instantiated [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md), [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md), and [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md) artifacts needed to apply [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) honestly
- Optional: additional upstream requirement, architecture, planning, or implementation artifacts; evidence notes; defect notes; or prior check results

## Outputs

- Primary output: a pass/fail review result for each artifact in scope and each required cross-cutting check
- Secondary outputs: explicit remediation routing, identified defect types, rework priorities, and any blocked readiness status that should prevent downstream handoff

## Skills And Tools

- Use [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when failures trace back to weak requirement grounding, unstable acceptance expectations, or unsupported coverage claims.
- Use [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the verification handoff needs rework.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when missing evidence or contradictory local context prevents a clean pass.
- Use [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when verification evidence is still embedded in workshop notes, test-session notes, or transcripts.
- Use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when failures depend on specification-to-implementation misalignment.

## Sequence

1. Confirm that the verification handoff exists and identify the supporting QA / Verification Lead artifacts that feed it.
2. Run the primary checks for the supporting verification artifacts and the handoff artifact.
3. Run [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) across the full chain.
4. Run [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md) across all artifacts in scope.
5. Record failures by artifact and defect type: weak confidence framing, unsupported coverage claims, broken evidence links, missing coverage ownership, contradictory signals, hidden scope exclusions, broken traceability, over-claimed readiness, poor specification relationship, or role drift.
6. Route remediation to the appropriate earlier verification, implementation, planning, architecture, or requirements artifact rather than patching the latest artifact in isolation.
7. Re-run the failed checks until the full chain passes or until remaining gaps are explicitly recorded as unresolved and the work is intentionally held open.

## Required Check Set

- Verification strategy review:
  - [`verification-strategy.check.md`](D:/Projects/agoge/checks/verification-strategy.check.md)
  - [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
  - [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)
- Verification matrix review:
  - [`verification-matrix.check.md`](D:/Projects/agoge/checks/verification-matrix.check.md)
  - [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
  - [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)
- Evidence review:
  - [`evidence-review.check.md`](D:/Projects/agoge/checks/evidence-review.check.md)
  - [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
  - [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)
- Verification handoff review:
  - [`verification-handoff.check.md`](D:/Projects/agoge/checks/verification-handoff.check.md)
  - [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
  - [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)

## Validation

- Every artifact in scope has an explicit pass/fail result.
- Cross-cutting failures are tied to the earliest artifact that introduced the defect.
- Remediation is routed to the correct earlier artifact or workflow stage.
- No artifact is treated as downstream-ready while any required check is still failing.

## Failure Handling

- If evidence is insufficient to apply a check honestly, fail the check and identify the missing evidence rather than guessing.
- If the same failure appears in multiple downstream artifacts, rework the earliest source artifact instead of patching each symptom separately.
- If the verification chain passes primary checks but fails traceability or boundary checks, treat the chain as blocked until the cross-cutting defect is resolved.

## Notes

This workflow is the final definition-of-done audit after handoff exists. It is a quality gate and rework loop, not a second substantive verification review stage.

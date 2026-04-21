---
id: qa-verification-handoff
kind: workflow
title: QA / Verification Handoff
version: 1
summary: Turn completed verification outputs into a downstream-ready handoff for implementation,
  review, or release-adjacent roles.
role: null
inputs:
- verification-strategy
- verification-matrix
- evidence-review
outputs:
- verification-handoff
skills:
- handoff-packaging
- research-documentation
checks:
- verification-handoff
- verification-traceability
- qa-verification-boundary
handoff_to: []
---

# QA / Verification Handoff

## Purpose

Turn completed verification outputs into a downstream-ready handoff for implementation, review, or release-adjacent roles.

## When To Use

- Verification direction is stable enough to guide downstream decisions.
- Downstream roles need a clean verification handoff rather than scattered evidence notes.
- Downstream consumers need to know which gaps, conditions, and confidence limits still matter.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md)
  - an instantiated copy of [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md)
  - an instantiated copy of [`artifacts/evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md)
- Optional: upstream requirements, architecture, or implementation handoff artifacts; implementation evidence notes; defect notes; and supporting review notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md) in the target project workspace
- Secondary outputs: highlighted coverage hotspots, residual risks, readiness conditions, scope exclusions, and downstream verification watchouts

## Skills And Tools

- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) as the default path for packaging reviewed verification outputs for downstream roles.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when evidence context, rollout considerations, or cross-artifact links need synthesis before the handoff can be written cleanly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md), and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the handoff must explain how existing behavioral specifications constrain verification or expose drift.

## Sequence

1. Read the verification strategy, verification matrix, and evidence review artifacts together.
2. If supporting notes or evidence references still need synthesis, use `research-documentation` first.
3. Instantiate [`artifacts/verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `handoff-packaging` to populate the verification handoff artifact with the verification summary, review status and key findings, evidence provenance summary, readiness ownership and conditions, coverage and evidence hotspots, residual risks and weak evidence, specification relationship, scope exclusions and deferred coverage, explicit re-verification triggers, downstream consumers, and next decision points.
5. Run [`verification-handoff.check.md`](D:/Projects/agoge/checks/verification-handoff.check.md), [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md), and [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md).

## Decision Points

- If the evidence review status is blocked or materially conditional, keep verification work open instead of packaging a misleading handoff.
- If unresolved requirement, architecture, planning, implementation, or specification ambiguity still shapes verification confidence materially, route that ambiguity upstream rather than hiding it in the handoff.
- If the current verification judgment would become invalid under foreseeable code, environment, or rollout changes, make those re-verification triggers explicit rather than leaving them to downstream inference.
- If the handoff starts turning into release operations, deployment choreography, or a generic defect tracker, remove that content and leave those concerns to downstream roles.

## Validation

- [`verification-handoff.check.md`](D:/Projects/agoge/checks/verification-handoff.check.md) passes.
- [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) passes.
- [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md) passes.
- The handoff is ready to feed downstream implementation, review, approval, or release-adjacent work without rediscovery.

## Failure Handling

- Stop and ask for clarification if the verification package cannot be handed off honestly from the available artifacts.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to reconstruct the missing verification context themselves.
- If a traceability or boundary check fails, route remediation to the earliest verification, implementation, planning, architecture, or requirements artifact that introduced the defect.

## Notes

This workflow packages verification for downstream use without becoming a release checklist.

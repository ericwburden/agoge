---
id: release-handoff-manager-readiness-review
kind: workflow
title: Release / Handoff Manager Readiness Review
version: 1
summary: Review the drafted release package, record the durable release posture, and
  decide whether the candidate is ready, ready with conditions, or blocked pending
  remediation or approval.
role: release-handoff-manager
inputs:
- release-candidate-summary
- rollout-and-operations-notes
outputs:
- release-readiness-decision
skills:
- release-readiness-packaging
- research-documentation
- handoff-packaging
- requirements-verification
checks:
- release-readiness-decision
- release-traceability
- release-handoff-manager-boundary
handoff_to: []
---

# Release / Handoff Manager Readiness Review

## Purpose

Review the drafted release package, record the durable release posture, and decide whether the candidate is ready, ready with conditions, or blocked pending remediation or approval.

## When To Use

- A release candidate summary and rollout notes artifact already exist.
- Downstream roles need an explicit release posture rather than only a packaging summary.
- The release package includes mixed approvals, evidence gaps, or operational caveats that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md)
  - an instantiated copy of [`artifacts/rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md)
- Expected supporting context:
  - the corresponding implementation, review, and verification packages
- Optional:
  - approval notes, operational notes, environment notes, communication notes, or specification references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md) in the target project workspace
- Secondary outputs: explicit release posture, grouped conditions, decision owners, residual risks, and downstream watchouts

## Skills And Tools

- [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md) as the default direct-support skill for turning the release candidate and operational notes into an explicit release-readiness posture with scoped approval language and re-approval triggers.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing approvals, evidence, and operational constraints before writing the decision.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the logic is present but not yet expressed cleanly for downstream release use.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the release posture depends materially on validated behavior or acceptance commitments.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when release confidence depends materially on specification-sensitive alignment.

## Sequence

1. Read the release candidate summary and rollout notes together with the implementation, review, and verification packages and any relevant operational references.
2. If approvals, evidence references, or rollout constraints still need synthesis before the decision can be stated honestly, use `research-documentation` first.
3. Instantiate [`artifacts/release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md) into the project workspace if a working copy does not already exist.
4. Use `release-readiness-packaging` to populate the release readiness decision artifact with decision scope, reviewed inputs, overall assessment, explicit release status, any distinction between release packaging readiness and final deployment authorization, approval limits, required follow-up, condition owners, residual risks, environment or monitoring watchouts, and re-review or re-approval triggers.
5. Run [`release-readiness-decision.check.md`](D:/Projects/orpheum/checks/release-readiness-decision.check.md), [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md), and [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).

## Decision Points

- If the candidate is blocked, state the blocking condition directly rather than hiding it in general caution language.
- If the candidate is conditionally ready, make the conditions and owners explicit rather than leaving them implied.
- If the issue is really unresolved review, verification, or operational evidence rather than a packaging concern, preserve that distinction in the decision.
- If the decision only applies to a particular environment, rollout wave, evidence window, or approval state, state that explicitly rather than implying broader release permission.
- If the candidate is ready for downstream release handling but not yet authorized for immediate deployment, say so explicitly rather than allowing "ready" to carry both meanings at once.

## Validation

- [`release-readiness-decision.check.md`](D:/Projects/orpheum/checks/release-readiness-decision.check.md) passes.
- [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md) passes.
- [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md) passes.
- The package is ready to feed [`release-handoff-manager-handoff.md`](D:/Projects/orpheum/workflows/release-handoff-manager-handoff.md).

## Failure Handling

- Stop and ask for clarification if the release posture cannot be made honestly from the available package.
- If the decision check fails, rework the decision instead of asking downstream roles to infer the real posture.
- If traceability or boundary checks fail, route remediation to the earliest artifact, evidence source, or upstream role decision that introduced the defect.

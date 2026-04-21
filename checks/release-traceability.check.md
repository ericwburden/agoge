---
id: release-traceability
kind: check
title: Release Traceability Check
version: 1
summary: Validate that release candidate summary, readiness decision, rollout notes,
  and release handoff stay traceable to the reviewed implementation, code review,
  and verification packages that justify downstream release or adoption consideration.
mode: presence
severity: error
applies_to: []
---

# Release Traceability Check

## Purpose

Validate that release candidate summary, readiness decision, rollout notes, and release handoff stay traceable to the reviewed implementation, code review, and verification packages that justify downstream release or adoption consideration.

## Applies To

- All instantiated Release / Handoff Manager artifacts and Release / Handoff Manager workflows
- Use whenever checking whether a release package is grounded in the right reviewed inputs and current conditions

## Criteria

- The release candidate summary points back to the implementation, review, and verification packages being packaged.
- The release readiness decision is traceable to the reviewed package and the operational or approval constraints that justify its posture.
- The rollout and operations notes are traceable to actual environment, operational, or communication constraints rather than generic release boilerplate.
- The release handoff preserves the current decision, active conditions, and key rollout notes that matter downstream.
- Specification-sensitive, trust-boundary-sensitive, or approval-sensitive concerns are traceable to the relevant source when they materially drive release posture.
- If release packaging exposes an upstream defect, the routing target is explicit.

## Scoring Or Outcome

Pass/fail.

The package passes only if a downstream reader can follow the chain from reviewed inputs to release posture to downstream handoff without guesswork.

## Evidence Required

- The instantiated Release / Handoff Manager artifacts being reviewed
- The corresponding implementation, review, and verification packages
- Relevant upstream handoffs, specifications, or operational references when needed to validate anchors

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when traceability is obscured by fragmented local context.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when traceability back to validated behavior or acceptance commitments needs strengthening.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when traceability depends on specification-sensitive drift review.

## Failure Response

- Rework the earliest release artifact that failed to preserve the traceability chain.
- If the traceability break starts in the implementation, review, verification, or upstream artifacts, route remediation there explicitly.

---
id: review-traceability
kind: check
title: Review Traceability Check
version: 1
summary: Validate that review scope, findings, decision, and handoff stay traceable
  to the implementation package and the upstream commitments that materially constrain
  the reviewed change.
mode: presence
severity: error
applies_to: []
---

# Review Traceability Check

## Purpose

Validate that review scope, findings, decision, and handoff stay traceable to the implementation package and the upstream commitments that materially constrain the reviewed change.

## Applies To

- All instantiated Code Reviewer artifacts and Code Reviewer workflows
- Use whenever checking whether a review package is grounded in the right code and upstream commitments

## Criteria

- Review scope points back to the implementation package being reviewed.
- Material findings tie back to affected code, evidence, or explicit upstream commitments rather than floating as decontextualized opinions.
- The decision is traceable to the findings that justify it.
- The handoff preserves the current decision and the findings that matter downstream.
- Requirement, architecture, planning, or specification-sensitive concerns are traceable to the relevant upstream source when they materially drive review posture.
- If review exposes an upstream defect, the routing target is explicit.

## Scoring Or Outcome

Pass/fail.

The package passes only if a downstream reader can follow the chain from reviewed change to finding to decision to handoff without guesswork.

## Evidence Required

- The instantiated Code Reviewer artifacts being reviewed
- The corresponding implementation package artifacts
- Relevant upstream handoffs or specifications when needed to validate anchors

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when traceability is obscured by fragmented local context.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when traceability back to requirements or acceptance commitments needs strengthening.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when traceability depends on specification-sensitive drift review.

## Failure Response

- Rework the earliest review artifact that failed to preserve the traceability chain.
- If the traceability break starts in the implementation package or upstream artifacts, route remediation there explicitly.

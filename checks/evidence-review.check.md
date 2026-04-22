---
id: evidence-review
kind: check
title: Evidence Review Check
version: 1
summary: Validate that the evidence review artifact records a usable readiness judgment,
  the evidence basis for that judgment, and the remediation needed before downstream
  use.
mode: headings
severity: error
applies_to:
- evidence-review
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Review Scope
- Reviewed Inputs
- Evidence Provenance
- Overall Assessment
- Readiness Or Approval Status
- Decision Owner Or Approver
- Key Findings
- Evidence Strength And Gaps
- Requirement, Architecture, And Planning Observations
- Unresolved Risks And Questions
- Required Remediation
- Condition Owners
- Recommended Next Step
---

# Evidence Review Check

## Purpose

Validate that the evidence review artifact records a usable readiness judgment, the evidence basis for that judgment, and the remediation needed before downstream use.

## Applies To

- instantiated copies of [`artifacts/evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md)
- Use after the first evidence-review pass and before downstream verification handoff.
- Do not use as a substitute for validating the underlying strategy or matrix artifacts themselves.

## Criteria

- The review scope and reviewed inputs are identified clearly.
- Evidence provenance is explicit enough to tell which revision, environment, run, or artifact instance the review applies to.
- At least one concrete evidence source is identified and used as part of the readiness judgment.
- The overall assessment is explicit.
- Readiness or approval status is stated clearly.
- A decision owner or approver is explicit when readiness is not purely self-evident.
- Key findings are visible and materially relevant.
- Evidence strength and evidence gaps are surfaced clearly.
- Requirement, architecture, and planning observations are visible when they materially affect readiness.
- Unresolved risks and questions are explicit.
- Required remediation is routed to the earliest artifact, evidence source, or workflow stage that should be reworked.
- Condition owners are explicit when the review outcome is conditional or blocked.
- The recommended next step is consistent with the actual review outcome.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand whether the verification package is ready, why, and what still needs to change without reconstructing the review from chat context.

## Evidence Required

- The evidence review artifact.
- The reviewed verification strategy and verification matrix.
- The upstream implementation handoff and other supporting evidence needed to interpret the findings honestly.

If the review artifact does not show how it reached its readiness judgment, what evidence state the judgment applies to, or what concrete evidence sources were actually reviewed, fail the check.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when the review reveals weak grounding in upstream requirements or acceptance expectations.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when conflicting local context or evidence references need synthesis before a review judgment can be made honestly.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when readiness depends on resolving specification-to-implementation drift.

## Failure Response

- Rework the review artifact before relying on it as the decision point for downstream handoff.
- If the review reveals blocked verification, route remediation to the earliest strategy, matrix, implementation, planning, architecture, or requirement artifact that introduced the defect.

## Notes

This check protects the evidence review stage from becoming a vague approval ritual. It requires the review to be explicit, durable, and actionable.

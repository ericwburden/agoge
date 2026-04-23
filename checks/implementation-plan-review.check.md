---
id: implementation-plan-review
kind: check
title: Implementation Plan Review Check
version: 1
summary: Validate that the implementation plan review artifact records a usable readiness
  judgment, the material findings behind that judgment, and the remediation needed
  before implementation handoff.
mode: headings
severity: error
applies_to:
- implementation-plan-review
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Review Scope
- Reviewed Inputs
- Overall Assessment
- Readiness Or Approval Status
- Decision Owner Or Approver
- Key Findings
- Semantic Review Findings
- Decision Changes Since Draft
- Cross-Artifact Reconciliation
- Dependency And Sequencing Observations
- Unresolved Risks And Questions
- Required Remediation
- Condition Owners
- Recommended Next Step
---

# Implementation Plan Review Check

## Purpose

Validate that the implementation plan review artifact records a usable readiness judgment, the material findings behind that judgment, and the remediation needed before implementation handoff.

## Applies To

- instantiated copies of [`artifacts/implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md)
- Use after the first planning pass and before downstream implementation handoff.
- Do not use as a substitute for validating the underlying planning artifacts themselves.

## Criteria

- The review scope and reviewed inputs are identified clearly.
- The overall assessment is explicit.
- Readiness or approval status is stated clearly.
- A decision owner or approver is explicit when readiness is not purely self-evident.
- Key findings are visible and materially relevant.
- Semantic review findings and decision changes are explicit when review materially changed the package.
- Cross-artifact reconciliation is visible when semantic review required updates elsewhere in the package.
- Dependency and sequencing observations are surfaced when execution order is part of the review concern.
- Unresolved risks and questions are explicit.
- Required remediation is routed to the earliest artifact that should be reworked.
- Condition owners are explicit when the review outcome is conditional or blocked.
- The recommended next step is consistent with the actual review outcome.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand whether the planning package is ready, why, and what still needs to change without reconstructing the review from chat context.

## Evidence Required

- The implementation plan review artifact.
- The reviewed implementation strategy and sequencing artifacts.
- The upstream architecture handoff and other planning inputs needed to interpret the findings honestly.

If the review artifact does not show how it reached its readiness judgment, fail the check.

## Supporting Skills

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when the review reveals weak slice structure, unclear dependencies, or missing planning rationale that should be corrected in the underlying plan.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when conflicting local context needs synthesis before a review judgment can be made honestly.

## Failure Response

- Rework the review artifact before relying on it as the decision point for downstream handoff.
- If the review reveals blocked planning, route remediation to the earliest planning or upstream artifact rather than papering over the defect in the review summary.

## Notes

This check protects the planning review stage from becoming a vague approval ritual. It requires the review to be explicit, durable, and actionable.

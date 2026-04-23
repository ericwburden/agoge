---
id: product-decision-review
kind: check
title: Product Decision Review Check
version: 1
summary: Validate that the product decision review artifact states the current decision
  posture, conditions, risks, tradeoffs, and ownership clearly enough for downstream
  consumers.
mode: headings
severity: error
applies_to:
- product-decision-review
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Review Scope
- Reviewed Inputs
- Overall Assessment
- Decision Status
- Decision Owner
- Key Risks, Tradeoffs, And Tensions
- Semantic Review Findings
- Decision Changes Since Draft
- Cross-Artifact Reconciliation
- Conditions And Required Follow-Up
- Follow-Up Owners
- Revisit Triggers
- Recommended Next Step
---

# Product Decision Review Check

## Purpose

Validate that the product decision review artifact states the current decision posture, conditions, risks, tradeoffs, and ownership clearly enough for downstream consumers.

## Applies To

- Instantiated copies of [`product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md)
- Use before producing the downstream product handoff

## Criteria

- The review scope and reviewed inputs are explicit.
- The overall assessment is honest about both confidence and caution.
- The decision status is explicit rather than implied from tone.
- Any scope, timing, evidence, or approval limits are clear.
- The review makes it clear when product-priority readiness is not the same thing as implementation commitment, sprint commitment, or release approval.
- Key risks, tradeoffs, and stakeholder tensions are visible.
- Semantic review findings and decision changes are explicit when review materially changed the package.
- Cross-artifact reconciliation is visible when semantic review required updates elsewhere in the package.
- Conditions, required follow-up, and follow-up owners are explicit when the package is conditional or blocked.
- Revisit triggers and the recommended next step are explicit.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated product decision review artifact
- The product direction and backlog prioritization artifacts

## Supporting Skills

- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when decision confidence depends on stronger grounding in validated outcomes or acceptance commitments.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the decision logic depends on distributed local evidence.

## Failure Response

- Rework the review artifact instead of expecting downstream roles to infer the real product posture.
- Route missing evidence or unresolved discovery back to the earliest artifact or role that introduced the gap.

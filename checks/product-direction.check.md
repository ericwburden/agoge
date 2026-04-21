---
id: product-direction
kind: check
title: Product Direction Check
version: 1
summary: Validate that the product direction artifact makes the current outcome focus,
  target users or stakeholders, value frame, constraints, and scope boundaries explicit
  enough to guide downstream work.
mode: headings
severity: error
applies_to:
- product-direction
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Decision Scope
- Validated Inputs
- Product Goal Or Outcome Focus
- Target Users, Stakeholders, Or Beneficiaries
- Value Hypotheses And Success Signals
- Acceptance Intent And Behavioral Guardrails
- Scope Boundaries And Non-Goals
- Constraints And Decision Drivers
- Priority Themes
- Open Questions And Decision Needs
- Recommended Next Step
---

# Product Direction Check

## Purpose

Validate that the product direction artifact makes the current outcome focus, target users or stakeholders, value frame, constraints, and scope boundaries explicit enough to guide downstream work.

## Applies To

- Instantiated copies of [`product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md)
- Use before treating product direction as ready for backlog ordering, architecture work, or downstream handoff

## Criteria

- The decision scope is explicit.
- Validated inputs are referenced clearly enough to explain why this direction exists now.
- The product goal or outcome focus is clear and value-oriented.
- Target users, stakeholders, or beneficiaries are identified.
- Acceptance intent and behavioral guardrails are explicit when they materially affect downstream work.
- The artifact does not present product-level acceptance framing as if it were already a full specification or verification plan.
- Scope boundaries, non-goals, and material constraints are explicit.
- Priority themes are concrete enough to guide downstream choices.
- Open questions and decision needs are surfaced rather than hidden.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated product direction artifact
- The supporting business, feedback, or delivery inputs used to justify it

## Supporting Skills

- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the direction needs stronger grounding in validated needs or acceptance commitments.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the supporting context is spread across multiple local sources.

## Failure Response

- Rework the product direction artifact before using it as a downstream anchor.
- Route missing discovery or evidence upstream rather than inventing product certainty.

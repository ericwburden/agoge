---
id: definition-of-done
kind: check
title: Definition Of Done Check
version: 1
summary: Validate that the Definition of Done artifact clearly expresses the standing
  project-level completion standard for implementation-phase work and preserves the
  boundary between enduring done rules and slice-local execution details.
mode: headings
severity: error
applies_to:
- definition-of-done
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Scope And Applicability
- Source Inputs And Rationale
- Project-Level Completion Standard
- Required Implementation Conditions
- Required Review Conditions
- Required Verification Conditions
- Required Documentation And Handoff Conditions
- Security, Compliance, And Control Expectations
- Release-Readiness Relationship
- Slice-Level Tailoring Rules
- Explicit Non-Goals And Exclusions
- Change Control
---

# Definition Of Done Check

## Purpose

Validate that the Definition of Done artifact clearly expresses the standing project-level completion standard for implementation-phase work and preserves the boundary between enduring done rules and slice-local execution details.

## Applies To

- instantiated copies of [`artifacts/definition-of-done.md`](D:/Projects/orpheum/artifacts/definition-of-done.md)
- Use when a planning package is being prepared for downstream implementation, review, verification, or release-adjacent work.
- Do not use as a substitute for slice-local exit criteria, implementation evidence review, or downstream release decisions.

## Criteria

- The artifact states what delivery scope the Definition of Done governs.
- Source inputs and rationale are explicit enough that downstream roles can see where the standard came from.
- A project-level completion standard is stated clearly rather than implied.
- Required implementation, review, verification, and documentation or handoff conditions are explicit.
- Security, compliance, approval, or control expectations are explicit when they materially affect completion.
- The relationship between "done" and downstream release readiness is explicit.
- Slice-level tailoring rules are explicit.
- The artifact preserves the distinction between enduring completion rules and slice-local execution details.
- Non-goals and exclusions are explicit.
- Change control is explicit enough that downstream roles know when the Definition of Done should be revised rather than silently reinterpreted.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if downstream implementation, review, verification, and release-adjacent roles could apply the same completion standard without relying on hidden team norms.

## Evidence Required

- The Definition of Done artifact.
- The reviewed requirements, architecture, product, planning, and relevant security/compliance artifacts that shaped it.
- Supporting notes when project-level completion rules depend on local policy, delivery conventions, or approval constraints.

If the standing completion standard cannot be tied back to project evidence or policy, fail the check and identify the missing anchor.

## Supporting Skills

- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when completion rules depend on validated requirements, acceptance commitments, or behavioral expectations.
- [`spec-to-implementation`](D:/Projects/orpheum/skills/spec-to-implementation/SKILL.md) when the Definition of Done is not yet connected cleanly to execution planning.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when rationale or completion constraints are spread across multiple local sources.

## Failure Response

- Rework the Definition of Done artifact before treating the planning package as downstream-ready.
- Move slice-local completion details back into planning artifacts instead of overwriting the standing project-level completion standard.

## Notes

This check exists because implementation work drifts fastest when "done" is left implicit or is silently redefined slice by slice.

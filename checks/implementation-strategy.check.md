---
id: implementation-strategy
kind: check
title: Implementation Strategy Check
version: 1
summary: Validate that the implementation strategy artifact clearly expresses the
  intended execution approach, slice structure, enabling work, and readiness assumptions
  for the reviewed architecture.
mode: headings
severity: error
applies_to:
- implementation-strategy
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Planning Scope And Objective
- Input Context
- Traceability Map
- Planning Assumptions And Constraints
- Implementation Approach
- Slice Strategy
- Workstream Overview
- Enabling Work And Spikes
- Slice Exit Criteria
- Readiness Conditions
- Verification And Rollout Considerations
- Deferred Or Not Included
- Risks And Open Questions
---

# Implementation Strategy Check

## Purpose

Validate that the implementation strategy artifact clearly expresses the intended execution approach, slice structure, enabling work, and readiness assumptions for the reviewed architecture.

## Applies To

- instantiated copies of [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md)
- Use after the first planning pass.
- Do not use as a substitute for sequencing, review, or downstream handoff checks.

## Criteria

- The planning scope and objective are stated explicitly.
- Input context from upstream architecture and requirements artifacts is identified.
- An explicit traceability map connects major slices or workstreams back to validated requirements, architecture, and handoff hotspots.
- Planning assumptions and constraints are explicit.
- The implementation approach and slice strategy are clear.
- Major workstreams and their responsibilities are visible.
- Enabling work and spikes are surfaced rather than hidden inside committed slices.
- Slice exit criteria are explicit enough that downstream implementation or verification roles can tell when a slice is ready to hand off or close.
- Readiness conditions are explicit and include owner capture.
- Verification and rollout considerations are visible when they materially affect the plan.
- Deferred or intentionally excluded work is explicit.
- Risks and open questions are visible.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another technical role could understand the proposed execution approach without inferring missing planning structure from chat context.

## Evidence Required

- The implementation strategy artifact.
- The upstream architecture and requirements artifacts it depends on.
- Any supporting notes used to justify slice shape, workstream structure, or readiness conditions.

If planning reasoning depends on missing or unstated evidence, fail the check and identify the gap.

## Supporting Skills

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when the planning structure, slice strategy, or risk treatment needs strengthening.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when architecture or requirement context is spread across multiple local sources and needs synthesis.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when planning inputs are still embedded in workshop notes or transcript material.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when the implementation strategy depends on existing behavioral specifications that need interpretation or clarification upstream.

## Failure Response

- Rework the implementation strategy artifact before handing it to downstream implementation or verification roles.
- Make missing plan assumptions, slice logic, or enabling work explicit rather than implying them.

## Notes

This is the first Technical Planner quality gate. If it fails, the rest of the planning chain is likely to inherit weak execution framing.

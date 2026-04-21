---
id: implementation-record
kind: check
title: Implementation Record Check
version: 1
summary: Validate that the implementation record artifact clearly expresses what implementation
  slice was attempted, what upstream artifacts governed it, what changed, and where
  deviations or blockers still matter.
mode: headings
severity: error
applies_to:
- implementation-record
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Implementation Scope And Objective
- Input Context
- Traceability Map
- Target Slice Or Change Boundary
- Planned Versus Actual Scope
- Change Summary
- Change Inventory
- Changed Components And Affected Areas
- Interface, Schema, And Contract Effects
- Deviations From Plan Or Specification
- Blockers, Risks, And Open Questions
- Deferred Or Intentionally Not Included
---

# Implementation Record Check

## Purpose

Validate that the implementation record artifact clearly expresses what implementation slice was attempted, what upstream artifacts governed it, what changed, and where deviations or blockers still matter.

## Applies To

- instantiated copies of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
- Use after the first implementation pass.
- Do not use as a substitute for evidence, review, or downstream handoff checks.

## Criteria

- The implementation scope and objective are stated explicitly.
- Input context from reviewed planning, architecture, requirements, and relevant specifications is identified.
- An explicit traceability map connects major change areas back to validated requirements, reviewed architecture, reviewed planning, and governing specifications where relevant.
- The target slice or change boundary is clear.
- Planned versus actual scope is explained.
- Major changes and affected areas are visible.
- A concrete change inventory makes the implementation footprint auditable.
- Interface, schema, or contract effects are explicit when they materially affect downstream work.
- Deviations from plan or specification are explicit.
- Blockers, risks, and open questions are visible.
- Deferred or intentionally excluded work is explicit.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another technical role could understand what implementation happened, why it happened, and where it diverged from expectation without reconstructing the story from chat or code diffs alone.

## Evidence Required

- The implementation record artifact.
- The reviewed planning, architecture, requirements, and specification artifacts it depends on.
- Supporting notes when needed to explain deviations, blockers, or affected areas.

If implementation reasoning depends on missing or unstated evidence, fail the check and identify the gap.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the implementation context or change reasoning is spread across multiple local sources and needs synthesis.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when implementation inputs or deviations are still buried in working-session notes.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when implementation traceability depends on an existing behavioral specification.

## Failure Response

- Rework the implementation record before handing it to downstream review or verification roles.
- Make the implementation boundary, traceability, or deviation story explicit rather than assuming downstream readers will infer it.

## Notes

This is the first Implementation Engineer quality gate. If it fails, the rest of the implementation package will be hard to trust downstream.

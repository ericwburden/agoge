---
id: business-objectives
kind: check
title: Business Objectives Check
version: 1
summary: Validate that the business objectives artifact is complete enough to support
  process analysis and requirement discovery without relying on unstated chat context.
mode: headings
severity: error
applies_to:
- business-objectives
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Problem Statement
- Business Context
- Business Objectives
- Success Criteria
- Stakeholders
- In Scope
- Out Of Scope
- Constraints
- Assumptions
- Proposed Solutions Mentioned
- Open Questions
- Stakeholder Confirmation
---

# Business Objectives Check

## Purpose

Validate that the business objectives artifact is complete enough to support process analysis and requirement discovery without relying on unstated chat context.

## Applies To

- instantiated copies of [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
- Use after initial kickoff and discovery framing.
- Do not use as a substitute for process or requirements checks.

## Criteria

- A clear business problem or opportunity is stated explicitly.
- Business objectives are described separately from proposed solutions or preferred features.
- Stakeholders are identified with enough context to understand why they matter.
- Success criteria are present.
- In-scope and out-of-scope boundaries are stated.
- Assumptions and open questions are visible.
- Stakeholder confirmation status is explicit, including whether the framing has been confirmed or is still pending.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if all criteria are satisfied or any remaining gaps are explicitly recorded as open questions rather than silently omitted.

## Evidence Required

- The business objectives artifact itself.
- Any source notes or discovery material needed to verify that the artifact reflects the available evidence.

If evidence is missing, fail the check and note which sections cannot be trusted.

## Supporting Skills

- [`meeting-intelligence`](D:/Projects/agoge/skills/meeting-intelligence/SKILL.md) when the artifact needs stronger kickoff framing, clearer scope, or better meeting-prep inputs.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when the source material is still in transcript or rough-notes form.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when supporting context is spread across multiple local files and needs synthesis.

## Failure Response

- Rework the business objectives artifact before moving on to downstream BA artifacts.
- Do not treat feature requests as objectives until the underlying business need is explicit.

## Notes

This check is the first BA quality gate. If it fails, later BA artifacts are likely to inherit weak framing.

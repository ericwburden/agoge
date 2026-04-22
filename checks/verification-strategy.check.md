---
id: verification-strategy
kind: check
title: Verification Strategy Check
version: 1
summary: Validate that the verification strategy artifact clearly expresses the intended
  verification scope, confidence goals, evidence expectations, and major risk focus
  areas for the reviewed delivery package.
mode: headings
severity: error
applies_to:
- verification-strategy
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Verification Scope And Objective
- Input Context
- Verification Drivers And Risks
- Confidence Goals
- Verification Levels And Methods
- Evidence Expectations
- Scope Exclusions And Deferrals
- Verification Constraints And Assumptions
- Architecture, Planning, And Specification Watchouts
- Readiness Decision Framing
- Open Questions
---

# Verification Strategy Check

## Purpose

Validate that the verification strategy artifact clearly expresses the intended verification scope, confidence goals, evidence expectations, and major risk focus areas for the reviewed delivery package.

## Applies To

- instantiated copies of [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md)
- Use after the first verification-planning pass.
- Do not use as a substitute for the matrix, review, or downstream handoff checks.

## Criteria

- The verification scope and objective are stated explicitly.
- Input context from upstream requirements, architecture, planning, and implementation artifacts is identified.
- Verification drivers and risks are explicit.
- Confidence goals are visible enough that a reviewer can tell what evidence standard is being applied.
- Verification levels and methods are clear.
- Evidence expectations are explicit, including major limits on what each evidence type can prove.
- Scope exclusions and deferrals are explicit.
- Verification constraints and assumptions are visible.
- Architecture, planning, and specification watchouts are surfaced when they materially affect verification.
- Readiness decision framing is explicit and includes ownership when needed.
- Open questions are visible.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another technical role could understand what verification posture is being proposed and why without inferring missing verification logic from chat context.

## Evidence Required

- The verification strategy artifact.
- The upstream requirements, architecture, planning, and implementation artifacts it depends on.
- Any supporting notes or evidence references used to justify verification priorities or evidence expectations.

If verification reasoning depends on missing or unstated evidence, fail the check and identify the gap.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when verification priorities or acceptance concerns need stronger grounding in upstream requirements evidence.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when verification context is spread across multiple local sources and needs synthesis.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when verification inputs are still embedded in workshop notes, test-session notes, or transcript material.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md), or [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when existing behavioral specifications materially affect coverage expectations or evidence interpretation.

## Failure Response

- Rework the verification strategy artifact before handing it to downstream reviewers or release-adjacent roles.
- Make missing verification assumptions, evidence standards, or risk priorities explicit rather than implying them.

## Notes

This is the first QA / Verification Lead quality gate. If it fails, the rest of the verification chain is likely to inherit weak confidence framing.

---
id: verification-matrix
kind: check
title: Verification Matrix Check
version: 1
summary: Validate that the verification matrix artifact clearly maps important requirements,
  architectural commitments, planning hotspots, and implementation slices to the evidence
  expected or reviewed for each one.
mode: headings
severity: error
applies_to:
- verification-matrix
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Matrix Scope
- Source Inputs
- Coverage Map
- Hotspot Summary
- Contradictions And Weak Signals
- Deferred Coverage
- Upstream Routing Notes
---

# Verification Matrix Check

## Purpose

Validate that the verification matrix artifact clearly maps important requirements, architectural commitments, planning hotspots, and implementation slices to the evidence expected or reviewed for each one.

## Applies To

- instantiated copies of [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md)
- Use after the first coverage-mapping pass.
- Do not use as a substitute for the strategy, review, or downstream handoff checks.

## Criteria

- The matrix scope is stated explicitly.
- Source inputs from requirements, architecture, planning, implementation, and evidence artifacts are identified.
- Major in-scope requirements, architectural commitments, interfaces, implementation slices, or specification-sensitive behaviors are represented.
- Each mapped item has an expected evidence type or verification method.
- Observed evidence is recorded where available.
- Coverage state is explicit for each mapped item.
- Gate impact is explicit for each mapped item.
- Gaps, limitations, or contradictions are visible rather than implied.
- Next-action ownership is captured when a gap requires remediation rather than observation.
- Hotspot summary reflects the weakest or riskiest areas accurately.
- Contradictions and weak signals are surfaced explicitly.
- Deferred coverage is kept separate from currently blocked or missing coverage.
- Upstream routing notes distinguish verification gaps from upstream requirements, architecture, planning, or implementation defects.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a reviewer can tell what is covered, what is weak, and what still needs action without reconstructing the evidence chain from other notes.

## Evidence Required

- The verification matrix artifact.
- The requirements, architecture, planning, implementation, and evidence sources it depends on.
- Any supporting notes needed to interpret ambiguous coverage or conflicting signals.

If the coverage state, gate impact, or evidence chain cannot be demonstrated, fail the check and identify the broken link.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when the matrix is weak because upstream requirement framing or acceptance expectations are unstable.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the evidence chain or source context needs synthesis before coverage can be judged honestly.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when evidence or walkthrough findings are still embedded in rough notes.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when apparent coverage depends on specification-to-implementation alignment that may have drifted.

## Failure Response

- Rework the verification matrix artifact before relying on it as the coverage map for readiness decisions.
- Make missing or contradictory coverage explicit rather than allowing a soft assumption of completeness.

## Notes

This is the core coverage-quality gate for QA / Verification Lead work.

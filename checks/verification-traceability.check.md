---
id: verification-traceability
kind: check
title: Verification Traceability Check
version: 1
summary: Validate that the QA / Verification Lead artifact chain preserves traceability
  from upstream requirements, architecture, planning, and implementation outputs to
  the verification strategy, coverage matrix, evidence review, and downstream verification
  handoff.
mode: presence
severity: error
applies_to:
- verification-strategy
- verification-matrix
- evidence-review
- verification-handoff
---

# Verification Traceability Check

## Purpose

Validate that the QA / Verification Lead artifact chain preserves traceability from upstream requirements, architecture, planning, and implementation outputs to the verification strategy, coverage matrix, evidence review, and downstream verification handoff.

## Applies To

- instantiated copies of [`artifacts/verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md)
- instantiated copies of [`artifacts/verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md)
- instantiated copies of [`artifacts/evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md)
- instantiated copies of [`artifacts/verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md)

Use after more than one QA / Verification Lead artifact exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- Verification scope connects back to validated requirements, reviewed architecture, reviewed planning, or explicit delivery constraints.
- Major requirements, architectural commitments, interfaces, planning hotspots, or implementation slices are explicitly mapped to expected or reviewed evidence.
- Verification treatment of existing behavioral specifications is explicit when those specifications materially constrain evidence interpretation.
- The evidence review reflects the actual strengths, gaps, and unresolved issues shown in the strategy, matrix, and reviewed evidence rather than inventing a separate narrative.
- The verification handoff preserves the reasoning, readiness state, residual risks, and evidence limits already captured in earlier verification artifacts.
- Requirement, architecture, planning, or implementation ambiguity is surfaced explicitly rather than solved silently inside the verification package.
- Contradictions, missing links, or unsupported verification claims are explicit.

## Scoring Or Outcome

Pass/fail.

The artifact chain passes only if a reviewer can follow the logic from upstream requirements, architecture, planning, and implementation outputs to the verification judgment and downstream handoff without relying on hidden assumptions.

## Evidence Required

- The full QA / Verification Lead artifact chain.
- The relevant upstream requirements, architecture, planning, and implementation artifact chains.
- The implementation evidence or supporting notes needed to interpret ambiguous verification links.

If one or more links cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when the broken link originates in unstable requirement or acceptance framing.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when verification context or evidence needs synthesis before traceability can be judged honestly.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the broken link shows up primarily in the downstream verification handoff.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the broken link depends on specification-to-implementation alignment.

## Failure Response

- Rework the earliest artifact, evidence source, or upstream package that introduced the missing or broken connection.
- Do not hand the verification package downstream as if it were settled while upstream justification or evidence support remains unclear.

## Notes

This is the core cross-cutting QA / Verification Lead quality check. It exists because individually reasonable verification artifacts can still fail as a chain.

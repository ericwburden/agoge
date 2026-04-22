---
id: release-candidate-summary
kind: check
title: Release Candidate Summary Check
version: 1
summary: Validate that the release candidate summary makes the candidate scope, reviewed
  inputs, supported boundaries, and release-sensitive hotspots explicit enough to
  ground a defensible release package.
mode: headings
severity: error
applies_to:
- release-candidate-summary
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Release Or Adoption Objective
- Reviewed Inputs
- Candidate Scope Included
- Explicitly Excluded Or Deferred Scope
- Release Target And Consumers
- Upstream Decision Anchors
- Release-Sensitive Hotspots
- Candidate Limits And Assumptions
- Recommended Next Step
---

# Release Candidate Summary Check

## Purpose

Validate that the release candidate summary makes the candidate scope, reviewed inputs, supported boundaries, and release-sensitive hotspots explicit enough to ground a defensible release package.

## Applies To

- Instantiated copies of [`release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md)
- Use before treating a release package as grounded packaging rather than a generic summary

## Criteria

- The artifact clearly identifies the release or adoption target in scope.
- Reviewed inputs include the implementation, review, and verification packages that materially justify release consideration.
- Included, excluded, and deferred scope are explicit enough that silence cannot be mistaken for inclusion.
- Release-sensitive hotspots are named rather than implied.
- Candidate limits and assumptions are explicit when the package is scoped, conditional, or environment-limited.
- Upstream decision anchors are explicit enough for downstream readers to understand why this candidate is being considered for release.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what candidate is being packaged and what its boundaries are.

## Evidence Required

- The instantiated release candidate summary artifact being reviewed
- The corresponding implementation, review, and verification packages when needed to confirm candidate accuracy
- Relevant upstream handoffs when needed to verify anchors

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when candidate context is spread across multiple local sources.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the candidate is understood but not yet packaged clearly.

## Failure Response

- Rework the release candidate summary before treating the release package as grounded.
- If candidate ambiguity is caused by a weak upstream implementation, review, or verification package, route remediation there rather than masking the gap in release language.

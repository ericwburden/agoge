---
id: implementation-evidence
kind: check
title: Implementation Evidence Check
version: 1
summary: Validate that the implementation evidence artifact honestly records what
  local validation was attempted, what environments it applied to, what the observed
  results were, and where confidence limits remain.
mode: headings
severity: error
applies_to:
- implementation-evidence
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Evidence Scope Summary
- Revision And Environment Provenance
- Validation Activities
- Observed Results
- Known Failures And Skipped Checks
- Manual Verification Notes
- Logs, Artifacts, And Supporting References
- Evidence Gaps And Confidence Limits
- Revalidation Watchouts
---

# Implementation Evidence Check

## Purpose

Validate that the implementation evidence artifact honestly records what local validation was attempted, what environments it applied to, what the observed results were, and where confidence limits remain.

## Applies To

- instantiated copies of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
- Use after local validation or evidence capture has occurred.
- Do not use as a substitute for downstream verification or readiness review.

## Criteria

- The evidence scope summary identifies what implementation slice the evidence applies to.
- Revision and environment provenance are explicit enough to interpret the evidence honestly.
- Validation activities are listed with scope and status.
- Commands, procedures, or manual paths are explicit when needed to reproduce or interpret the evidence.
- Observed results distinguish passes, failures, partial results, and inconclusive outcomes.
- Known failures and skipped checks are explicit.
- Manual verification notes are captured when they materially affect downstream understanding.
- Supporting logs, artifacts, or references are identified when they materially strengthen the evidence.
- Evidence gaps and confidence limits are explicit.
- Revalidation watchouts are visible.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream role could understand what evidence exists and what it does not support without relying on optimistic assumptions.

## Evidence Required

- The implementation evidence artifact.
- The underlying run outputs, logs, screenshots, notes, or command results when they are referenced materially.
- The implementation record when evidence scope needs clarification.

If the artifact makes claims that cannot be tied back to observed evidence, fail the check and identify the unsupported claim.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when validation results or evidence references need synthesis.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when browser-based or web-application evidence needs to be collected or strengthened.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when evidence interpretation depends on checking implementation behavior against a governing specification.

## Failure Response

- Rework the implementation evidence artifact before treating the implementation package as review-ready.
- Remove misleading confidence claims and make weak or missing evidence explicit.

## Notes

This check does not certify correctness. It only certifies whether the implementation evidence has been captured honestly and usefully.

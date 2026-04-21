---
id: implementation-traceability
kind: check
title: Implementation Traceability Check
version: 1
summary: Validate that the Implementation Engineer artifact chain preserves traceability
  from upstream requirements, architecture, planning, and relevant specifications
  through the implementation record, evidence package, readiness review, and downstream
  handoff.
mode: presence
severity: error
applies_to:
- implementation-record
- implementation-evidence
- implementation-readiness-review
- implementation-package-handoff
---

# Implementation Traceability Check

## Purpose

Validate that the Implementation Engineer artifact chain preserves traceability from upstream requirements, architecture, planning, and relevant specifications through the implementation record, evidence package, readiness review, and downstream handoff.

## Applies To

- instantiated copies of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
- instantiated copies of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
- instantiated copies of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
- instantiated copies of [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)

Use after more than one Implementation Engineer artifact exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- The implemented scope connects back to reviewed requirements, reviewed architecture, reviewed implementation planning, or explicit delivery constraints.
- Major changed areas are explicitly mapped to the upstream requirement, architecture, planning, or specification sources that justified them.
- Implementation evidence applies to the same slice described in the implementation record rather than to a different or unstated scope.
- The readiness review reflects the actual strengths, gaps, deviations, and unresolved issues shown in the implementation record and implementation evidence rather than inventing a separate narrative.
- The implementation package handoff preserves the scope, readiness state, evidence limits, and routing logic already captured in earlier implementation artifacts.
- Requirement, architecture, planning, or specification ambiguity is surfaced explicitly rather than solved silently inside the implementation package.
- Contradictions, missing links, or unsupported implementation claims are explicit.

## Scoring Or Outcome

Pass/fail.

The artifact chain passes only if a reviewer can follow the logic from upstream requirements, architecture, planning, and relevant specifications through the implemented change set and into the downstream handoff without relying on hidden assumptions.

## Evidence Required

- The full Implementation Engineer artifact chain.
- The relevant upstream requirements, architecture, planning, and specification artifacts.
- Supporting implementation evidence needed to interpret ambiguous links.

If one or more links cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when implementation context or evidence needs synthesis before traceability can be judged honestly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the broken link appears primarily in the downstream implementation package handoff.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the broken link depends on specification-to-implementation alignment.

## Failure Response

- Rework the earliest artifact, evidence source, or upstream package that introduced the missing or broken connection.
- Do not hand the implementation package downstream as if it were settled while upstream justification or implementation support remains unclear.

## Notes

This is the core cross-cutting Implementation Engineer quality check. It exists because individually reasonable implementation artifacts can still fail as a chain.

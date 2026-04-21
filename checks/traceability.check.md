---
id: traceability
kind: check
title: Traceability Check
version: 1
summary: Validate that the Business Analyst artifact chain preserves traceability
  from business objectives to process needs to verified requirements and downstream
  handoff.
mode: presence
severity: error
applies_to:
- business-objectives
- process-analysis
- requirements-specification
- requirements-handoff
---

# Traceability Check

## Purpose

Validate that the Business Analyst artifact chain preserves traceability from business objectives to process needs to verified requirements and downstream handoff.

## Applies To

- instantiated copies of [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
- instantiated copies of [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md)
- instantiated copies of [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md)
- instantiated copies of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md)

Use after the artifact chain exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- Business objectives connect to process needs.
- Process needs connect to verified requirements.
- Process needs are recorded explicitly rather than inferred only from narrative process descriptions.
- The requirements handoff reflects that linkage clearly enough for downstream readers.
- Unsupported requirements are not presented as confirmed.
- Contradictions, missing links, or evidence gaps are surfaced explicitly rather than silently patched over.

## Scoring Or Outcome

Pass/fail.

The artifact chain passes only if a reviewer can follow the logic from objective to process to requirement to handoff without relying on unstated assumptions.

## Evidence Required

- The full BA artifact chain.
- Any supporting notes or clarifications needed to verify ambiguous links.

If one or more links in the chain cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when requirements are not clearly connected to business objectives or process needs.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the handoff artifact does not preserve the traceability already present in earlier BA artifacts.

## Failure Response

- Rework the affected artifact or artifacts until the missing linkage is explicit.
- Do not hand unsupported requirements to downstream teams as if they are settled.

## Notes

This is the core cross-cutting BA quality check. It exists because individually reasonable artifacts can still fail as a chain.

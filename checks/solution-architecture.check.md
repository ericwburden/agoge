---
id: solution-architecture
kind: check
title: Solution Architecture Check
version: 1
summary: Validate that the solution architecture artifact clearly expresses the intended
  system shape, drivers, boundaries, and risks for the validated problem space.
mode: headings
severity: error
applies_to:
- solution-architecture
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Problem And Scope
- Input Context
- Architectural Drivers
- System Boundary
- Major Components And Responsibilities
- Major Flows
- Interfaces And Contracts
- Integrations And External Dependencies
- Constraints
- Decisions Made
- Locked Constraints
- Specification Relationship
- Architecture Fitness Criteria
- Trust Boundaries And Human Control Points
- Risks And Open Questions
---

# Solution Architecture Check

## Purpose

Validate that the solution architecture artifact clearly expresses the intended system shape, drivers, boundaries, and risks for the validated problem space.

## Applies To

- instantiated copies of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md)
- Use after the first architectural design pass.
- Do not use as a substitute for decision-rationale or downstream-handoff checks.

## Criteria

- The problem and scope are stated explicitly.
- Input context from upstream BA artifacts is identified.
- Architectural drivers are explicit.
- The system boundary is explicit.
- Major components and responsibilities are clear.
- Major flows, interface seams, and integration points are described clearly enough for downstream use.
- Important contract assumptions or failure expectations are explicit when they materially affect the design.
- Constraints, risks, and open questions are visible.
- Decisions already made and locked constraints are explicit enough that downstream roles do not have to infer them from surrounding prose.
- The relationship to any existing behavioral specification is explicit when specification work materially constrains the architecture.
- Architecture fitness criteria are explicit enough for downstream verification or planning to use.
- Trust boundaries and human control points are recorded when AI-enabled or agentic behavior is relevant.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another technical role could understand the intended solution shape without inferring missing architecture from chat context.

## Evidence Required

- The solution architecture artifact.
- The upstream BA artifacts it depends on.
- Any supporting notes used to justify major architectural drivers or boundary choices.

If architectural reasoning depends on missing or unstated evidence, fail the check and identify the gap.

## Supporting Skills

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) when the architecture needs stronger structure, clearer boundaries, or more explicit interface framing.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when relevant technical or domain constraints are spread across multiple local sources and need synthesis.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when architecture inputs are still embedded in workshop notes or transcript material.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when the architecture depends on existing behavioral specifications that need to be interpreted, reconciled, or clarified.

## Failure Response

- Rework the solution architecture artifact before handing it to downstream planning or implementation roles.
- Make missing architectural drivers, boundaries, or risks explicit rather than implying them.

## Notes

This is the first Solution Architect quality gate. If it fails, the rest of the architecture chain is likely to inherit weak framing.

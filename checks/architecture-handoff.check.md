---
id: architecture-handoff
kind: check
title: Architecture Handoff Check
version: 1
summary: Validate that the architecture handoff is complete enough for downstream
  roles to continue without reconstructing the solution shape or design rationale
  from earlier artifacts.
mode: headings
severity: error
applies_to:
- architecture-handoff
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Handoff Summary
- Architecture Summary
- Review Status And Key Findings
- Locked Decisions To Preserve
- Semantic Review Status
- Readiness Ownership And Conditions
- Interface, Dependency, And Integration Hotspots
- Verification Focus Areas
- Architecture Fitness Criteria
- Specification Relationship
- Unresolved Decisions And Risks
- Recommended Downstream Consumers
- Next Decision Points
---

# Architecture Handoff Check

## Purpose

Validate that the architecture handoff is complete enough for downstream roles to continue without reconstructing the solution shape or design rationale from earlier artifacts.

## Applies To

- instantiated copies of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md)
- Use when architecture is being passed to planning, implementation, or verification roles.
- Do not use to validate implementation plans or sprint/task decomposition.

## Criteria

- The architecture being handed off is summarized clearly.
- Review status and material architecture findings are visible.
- Locked decisions to preserve are explicit.
- Semantic review status is visible when semantic review is required for scenario completion.
- Readiness ownership and any material handoff conditions are explicit when the architecture is not simply ready.
- Major interface, dependency, or integration hotspots are visible.
- Verification focus areas are explicit.
- Architecture fitness criteria are visible enough to guide downstream verification or planning.
- The relationship to any existing behavioral specification is explicit when specification work materially constrains the architecture.
- Unresolved decisions and risks are present.
- Downstream consumers are identified.
- Next decision points are included.
- The handoff does not collapse into implementation planning, task decomposition, or delivery management.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream technical role can proceed without rediscovering the architecture from earlier notes.

## Evidence Required

- The architecture handoff artifact.
- The solution architecture and architecture decisions artifacts when needed to confirm completeness.

If the handoff depends on implied context not captured in the artifact, fail the check.

## Supporting Skills

- [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md) when the handoff needs stronger packaging, better hotspot framing, or clearer downstream guidance.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when architecture context is still scattered across local notes.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when the handoff must explain how existing behavioral specifications constrain the architecture or where specification gaps remain downstream-relevant.

## Failure Response

- Rework the handoff artifact before passing architecture downstream.
- Keep architecture work open until major risks, decisions, and hotspots are explicit.

## Notes

This check protects the architect-to-downstream boundary by ensuring the handoff is self-sufficient without becoming an implementation plan.

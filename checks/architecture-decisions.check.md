---
id: architecture-decisions
kind: check
title: Architecture Decisions Check
version: 1
summary: Validate that architectural decisions are explicit, traceable, and supported
  by rationale rather than implied by the final design alone.
mode: headings
severity: error
applies_to:
- architecture-decisions
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Decision Summary
- Major Decisions
- Locked Decisions And Downstream Non-Negotiables
- Deferred Decisions
- Architecture Assumptions
- Risks And Tradeoffs
---

# Architecture Decisions Check

## Purpose

Validate that architectural decisions are explicit, traceable, and supported by rationale rather than implied by the final design alone.

## Applies To

- instantiated copies of [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md)
- Use after major architectural directions have been chosen or narrowed.
- Do not use as a substitute for validating the solution shape or downstream handoff.

## Criteria

- Major decisions are stated explicitly.
- Decision drivers are recorded.
- Meaningful alternatives are visible when the choice was non-trivial.
- The chosen direction and rationale are explicit.
- Consequences, tradeoffs, or follow-on impacts are visible.
- Locked downstream non-negotiables are surfaced when later roles are expected to preserve them.
- Interface or contract implications are visible when the decision materially affects a system boundary.
- Deferred decisions and assumptions are surfaced rather than hidden.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand why the architecture took its current shape rather than only what shape was chosen.

## Evidence Required

- The architecture decisions artifact.
- The solution architecture artifact.
- Any notes used to evaluate alternatives or decision drivers.

If rationale is missing or the choices read as implicit defaulting, fail the check.

## Supporting Skills

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) when the decision record needs stronger option framing, rationale, or consequence capture.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when decision context depends on synthesizing multiple local references or design notes.

## Failure Response

- Rework the decision record before treating the architecture as settled.
- Preserve unresolved or deferred choices explicitly instead of retroactively pretending they were fully decided.

## Notes

This check exists because architecture direction without visible reasoning is hard to validate and hard to hand off cleanly.

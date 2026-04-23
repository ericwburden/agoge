---
id: implementation-handoff
kind: check
title: Implementation Handoff Check
version: 1
summary: Validate that the implementation handoff is complete enough for downstream
  implementation and verification roles to continue without reconstructing the execution
  structure or planning rationale from earlier artifacts.
mode: headings
severity: error
applies_to:
- implementation-handoff
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Handoff Summary
- Planning Summary
- Review Status And Key Findings
- Locked Decisions To Preserve
- Semantic Review Status
- Readiness Ownership And Conditions
- Ordered Slices And Dependency Hotspots
- Slice Exit Criteria Summary
- Verification And Test Strategy Touchpoints
- Rollout, Migration, And Control-Point Watchouts
- Specification Relationship
- Unresolved Decisions And Risks
- Deferred Or Not Included
- Recommended Downstream Consumers
- Next Decision Points
---

# Implementation Handoff Check

## Purpose

Validate that the implementation handoff is complete enough for downstream implementation and verification roles to continue without reconstructing the execution structure or planning rationale from earlier artifacts.

## Applies To

- instantiated copies of [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)
- Use when planning is being passed to implementation, verification, or downstream review roles.
- Do not use to validate sprint administration or detailed ticket boards.

## Criteria

- The implementation plan being handed off is summarized clearly.
- Review status and material planning findings are visible.
- Locked decisions to preserve are explicit.
- Semantic review status is visible when semantic review is required for scenario completion.
- Readiness ownership and any material handoff conditions are explicit when the plan is not simply ready.
- Ordered slices and dependency hotspots are visible.
- Slice exit criteria or equivalent handoff-completion conditions are visible.
- Verification and test-strategy touchpoints are explicit.
- Rollout, migration, or control-point watchouts are visible when they materially affect downstream execution.
- The relationship to any existing behavioral specification is explicit when specification work materially constrains the plan.
- Unresolved decisions and risks are present.
- Deferred or intentionally excluded work is present.
- Downstream consumers are identified.
- Next decision points are included.
- The handoff does not collapse into sprint administration, ticket backlog management, or detailed implementation design.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream technical role can proceed without rediscovering the plan from earlier notes.

## Evidence Required

- The implementation handoff artifact.
- The implementation strategy, sequencing and dependencies, and implementation plan review artifacts when needed to confirm completeness.

If the handoff depends on implied context not captured in the artifact, fail the check.

## Supporting Skills

- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the handoff needs stronger packaging, clearer downstream guidance, or better treatment of risks and conditions.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when planning context is still scattered across local notes.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), or [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when the handoff must explain how existing behavioral specifications constrain implementation or verification planning.

## Failure Response

- Rework the handoff artifact before passing the plan downstream.
- Keep planning work open until major sequencing constraints, risks, and readiness conditions are explicit.

## Notes

This check protects the planner-to-downstream boundary by ensuring the handoff is self-sufficient without becoming a sprint board.

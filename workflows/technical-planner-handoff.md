# Technical Planner Handoff

## Purpose

Turn completed planning outputs into a downstream-ready handoff for implementation, verification, and downstream review roles.

## When To Use

- Planning direction is stable enough to guide downstream work.
- Implementation or verification needs a clean planning handoff rather than scattered execution notes.
- Downstream roles need to know which sequencing assumptions, readiness conditions, and risk hotspots still matter.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md)
  - an instantiated copy of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md)
  - an instantiated copy of [`artifacts/implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md)
- Optional: upstream architecture handoff artifacts, requirements handoff artifacts, relevant security/compliance artifacts, dependency notes, and supporting review notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md) in the target project workspace
- Secondary outputs: highlighted slice order, dependency hotspots, verification touchpoints, rollout watchouts, review status, and unresolved planning questions

## Skills And Tools

- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) as the default path for packaging reviewed planning outputs for downstream roles.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when planning context, dependency notes, or rollout considerations need synthesis before the handoff can be written cleanly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), and [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when the handoff must explain how existing behavioral specifications constrain implementation or verification planning.

## Sequence

1. Read the implementation strategy, sequencing and dependencies, and implementation plan review artifacts together, using relevant security/compliance artifacts as needed to preserve planning constraints honestly in the handoff.
2. If supporting notes or rollout references still need synthesis, use `research-documentation` first.
3. Instantiate [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `handoff-packaging` to populate the implementation handoff artifact with the planning summary, review status and key findings, readiness ownership and conditions, ordered slices and dependency hotspots, a slice exit criteria summary, verification touchpoints, rollout watchouts, specification relationship, unresolved decisions and risks, deferred or intentionally excluded work, downstream consumers, and next decision points.
5. Run [`implementation-handoff.check.md`](D:/Projects/agoge/checks/implementation-handoff.check.md), [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md), and [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md).

## Decision Points

- If the implementation plan review status is blocked or materially conditional, keep planning work open instead of packaging a misleading handoff.
- If unresolved business or architecture ambiguity is still shaping the plan materially, route that ambiguity upstream rather than hiding it in the handoff.
- If the handoff starts turning into sprint administration, remove that content and leave execution tracking to downstream roles.

## Validation

- [`implementation-handoff.check.md`](D:/Projects/agoge/checks/implementation-handoff.check.md) passes.
- [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) passes.
- [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md) passes.
- The handoff is ready to feed downstream implementation, verification, or review work without rediscovery.

## Failure Handling

- Stop and ask for clarification if the plan cannot be handed off honestly from the available artifacts.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to reconstruct the missing planning context themselves.
- If a traceability or boundary check fails, route remediation to the earliest planning, architecture, or BA artifact that introduced the defect.

## Notes

This workflow packages planning for downstream use without becoming a backlog tool.

# Solution Architect Handoff

## Purpose

Turn completed architecture outputs into a downstream-ready handoff for planning, implementation, and verification roles.

## When To Use

- Architecture direction is stable enough to guide downstream work.
- Technical planning or implementation needs a clean architectural handoff rather than scattered design notes.
- Verification roles need to know which assumptions, hotspots, and risks to pay attention to.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md)
  - an instantiated copy of [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md)
  - an instantiated copy of [`artifacts/architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md)
- Optional: upstream BA handoff artifacts, relevant Product Owner artifacts or product handoff, technical constraints, and supporting review notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md) in the target project workspace
- Secondary outputs: highlighted interface and dependency hotspots, verification focus areas, review status, and unresolved architecture questions

## Skills And Tools

- [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md) as the default path for packaging reviewed architecture for downstream roles.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when architecture context or dependency notes need synthesis before the handoff can be written cleanly.

## Sequence

1. Read the solution architecture, architecture decisions, and architecture review artifacts together, using upstream BA and Product Owner artifacts as needed to preserve the reasoning chain honestly.
2. If supporting notes or dependency references still need synthesis, use `research-documentation` first.
3. Instantiate [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `architecture-handoff-packaging` to populate the architecture handoff artifact with the architecture summary, review status and key findings, readiness ownership and conditions, interface or dependency hotspots, verification focus areas, architecture fitness criteria, specification relationship, unresolved decisions and risks, downstream consumers, and next decision points.
5. Run [`architecture-handoff.check.md`](D:/Projects/agoge/checks/architecture-handoff.check.md), [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md), and [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md).

## Decision Points

- If the architecture review status is blocked or materially conditional, keep architecture work open instead of packaging a misleading handoff.
- If unresolved business ambiguity is still shaping the architecture materially, route that ambiguity back to the BA chain.
- If unresolved or unstable behavioral specifications are still materially shaping the architecture, route that gap back to upstream specification work instead of hiding it in downstream planning.
- If the handoff starts turning into task decomposition, remove that content and leave implementation planning to downstream roles.

## Validation

- [`architecture-handoff.check.md`](D:/Projects/agoge/checks/architecture-handoff.check.md) passes.
- [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) passes.
- [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md) passes.
- The handoff is ready to feed downstream planning, implementation, or verification work without rediscovery.

## Failure Handling

- Stop and ask for clarification if the architecture cannot be handed off honestly from the available artifacts.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to reconstruct the missing design context themselves.
- If a traceability or boundary check fails, route remediation to the earliest architecture or BA artifact that introduced the defect.

## Notes

This workflow packages architecture for downstream use without becoming a build plan.

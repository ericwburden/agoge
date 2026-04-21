# Architecture Traceability Check

## Purpose

Validate that the Solution Architect artifact chain preserves traceability from upstream BA outputs and relevant Product Owner direction to the proposed solution shape, architectural decisions, and downstream architecture handoff.

## Applies To

- instantiated copies of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md)
- instantiated copies of [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md)
- instantiated copies of [`artifacts/architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md)
- instantiated copies of [`artifacts/architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md)

Use after more than one Solution Architect artifact exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- Architectural drivers connect back to business objectives, process needs, requirements, relevant Product Owner direction, or explicit technical constraints.
- Architectural treatment of existing behavioral specifications is explicit when those specifications materially constrain the design.
- Architecture decisions reflect the actual solution architecture rather than an unstated alternative.
- The architecture review reflects the actual strengths, risks, and unresolved issues in the architecture artifacts rather than inventing a separate narrative.
- The architecture handoff preserves the reasoning, risks, and review outcome already captured in earlier architecture artifacts.
- Requirement or business ambiguity is surfaced explicitly rather than solved silently inside the architecture.
- Contradictions, missing links, or unsupported architectural assumptions are explicit.

## Scoring Or Outcome

Pass/fail.

The artifact chain passes only if a reviewer can follow the logic from upstream BA outputs and relevant Product Owner direction to architecture to downstream handoff without relying on hidden assumptions.

## Evidence Required

- The full Solution Architect artifact chain.
- The relevant upstream BA artifact chain.
- Relevant upstream Product Owner artifacts when product posture materially shaped the architecture.
- Any supporting notes needed to interpret ambiguous architectural links.

If one or more links cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md) when broken traceability is showing up as inconsistent review conclusions or unclear readiness status.
- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) when the broken link originates in the underlying architecture shape or decision record.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when architectural context or upstream evidence needs synthesis before traceability can be judged honestly.

## Failure Response

- Rework the earliest artifact that introduced the missing or broken connection.
- Do not hand architecture downstream as if it were settled while upstream justification remains unclear.

## Notes

This is the core cross-cutting Solution Architect quality check. It exists because individually reasonable architecture artifacts can still fail as a chain.

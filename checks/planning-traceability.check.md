# Planning Traceability Check

## Purpose

Validate that the Technical Planner artifact chain preserves traceability from upstream BA and architecture outputs, plus relevant security/compliance guidance when it materially shapes planning, to the implementation strategy, sequencing plan, review outcome, and downstream implementation handoff.

## Applies To

- instantiated copies of [`artifacts/implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md)
- instantiated copies of [`artifacts/sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md)
- instantiated copies of [`artifacts/implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md)
- instantiated copies of [`artifacts/implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)

Use after more than one Technical Planner artifact exists. Do not apply this check to a single standalone artifact in isolation.

## Criteria

- Planning structure connects back to reviewed architecture, validated requirements, relevant security/compliance guidance when applicable, or explicit delivery constraints.
- Major slices or workstreams are explicitly mapped back to the upstream requirements, architecture decisions, or handoff hotspots that justify them.
- Planning treatment of existing behavioral specifications is explicit when those specifications materially constrain execution order or verification framing.
- The sequencing artifact reflects the actual implementation strategy rather than an unstated alternative.
- The plan review reflects the actual strengths, risks, and unresolved issues in the planning artifacts rather than inventing a separate narrative.
- The implementation handoff preserves the reasoning, readiness state, and hotspots already captured in earlier planning artifacts.
- Requirement or architecture ambiguity is surfaced explicitly rather than solved silently inside the plan.
- Contradictions, missing links, or unsupported planning assumptions are explicit.

## Scoring Or Outcome

Pass/fail.

The artifact chain passes only if a reviewer can follow the logic from upstream BA and architecture outputs, plus relevant security/compliance guidance when applicable, to the execution plan and downstream handoff without relying on hidden assumptions.

## Evidence Required

- The full Technical Planner artifact chain.
- The relevant upstream BA and architecture artifact chains.
- Relevant upstream security/compliance artifacts when obligations, controls, or approval-sensitive constraints materially shaped the planning package.
- Any supporting notes needed to interpret ambiguous planning links.

If one or more links cannot be demonstrated, fail the check and identify the broken connection.

## Supporting Skills

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) when the broken link originates in the underlying plan structure, slice strategy, or dependency logic.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when planning context or upstream evidence needs synthesis before traceability can be judged honestly.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the broken link shows up primarily in the downstream handoff packaging.

## Failure Response

- Rework the earliest artifact that introduced the missing or broken connection.
- Do not hand the plan downstream as if it were settled while upstream justification remains unclear.

## Notes

This is the core cross-cutting Technical Planner quality check. It exists because individually reasonable planning artifacts can still fail as a chain.

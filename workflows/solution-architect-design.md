# Solution Architect Design

## Purpose

Turn validated BA outputs and explicit product direction into a proposed solution architecture and explicit architecture decision record.

## When To Use

- BA discovery is stable enough to support architecture work.
- Downstream roles need a system shape before planning or implementation can proceed safely.
- The work depends on making boundaries, responsibilities, integrations, or architectural tradeoffs explicit.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md)
  - an instantiated copy of [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md)
- Optional:
  - instantiated copies of [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md) and [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md)
  - instantiated copies of [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md), [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md), or [`artifacts/product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md) when explicit product posture should shape architectural drivers, tradeoffs, or sequencing assumptions
  - Allium specifications or other behavioral specifications when they already exist
  - architecture workshop notes
  - technical constraints or domain references

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md) in the target project workspace
  - an instantiated copy of [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md) in the target project workspace
- Secondary outputs: explicit architectural drivers, interface or contract seams, dependency hotspots, open technical questions, and design tradeoffs

## Skills And Tools

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md) as the default path for structuring the solution shape and architecture decisions.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when architecture workshop notes or transcripts need normalization.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when technical constraints, local references, or design notes need synthesis before the architecture can be stated cleanly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external standards, patterns, or technology comparisons materially affect the architecture direction.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when existing behavioral specifications must be interpreted or refined as part of architecture framing.

## Sequence

1. Read the requirements handoff and requirements specification together, using business objectives, process analysis, Product Owner artifacts, and existing Allium or behavioral specs as needed to clarify architecture drivers.
2. If architecture notes or workshop transcripts exist, normalize them with `meeting-notes-and-actions` before drafting.
3. If technical or domain context is spread across multiple local files, use `research-documentation` to synthesize the relevant constraints and reference points.
4. Instantiate [`artifacts/solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md) and [`artifacts/architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md) into the project workspace if working copies do not already exist.
5. Use `architecture-design` to populate the solution architecture artifact with problem scope, input context, architectural drivers, system boundary, major components, major flows, interfaces and contracts, integrations, constraints, specification relationship, architecture fitness criteria, trust boundaries, and risks.
6. Use `architecture-design` to populate the architecture decisions artifact with major decisions, alternatives, rationale, consequences, interface implications, deferred decisions, assumptions, and tradeoffs.
7. Run [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md), [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md), [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md), and [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md).

## Decision Points

- If upstream BA artifacts are still ambiguous, record the gap and route it back upstream instead of solving it silently in architecture.
- If explicit product direction exists and materially affects architectural tradeoffs, preserve that input explicitly instead of forcing the architecture to infer product posture from BA artifacts alone.
- If Allium or other behavioral specifications are missing or unstable where architecture depends on them materially, route that gap back to discovery or specification work rather than inventing the behavior in architecture.
- If multiple plausible solution directions exist, record the alternatives and decision drivers rather than defaulting to one without explanation.
- If important boundaries depend on interface ownership or contract assumptions, make them explicit here instead of leaving them for downstream roles to infer.
- If the system includes AI-enabled or agentic behavior, record trust boundaries and human control points explicitly.

## Validation

- [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md) passes.
- [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md) passes.
- [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) passes.
- [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md) passes.
- The instantiated outputs are ready to feed [`solution-architect-review.md`](D:/Projects/agoge/workflows/solution-architect-review.md).

## Failure Handling

- Stop and ask for clarification if architecture cannot be stated without inventing missing requirements or business assumptions.
- Do not collapse unresolved tradeoffs into fake certainty.
- If a traceability or boundary check fails, route remediation to the earliest BA or architecture artifact that introduced the defect.

## Notes

This is the default entry workflow for Solution Architect work.

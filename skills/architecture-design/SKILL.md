---
name: architecture-design
description: Turn validated requirements and technical constraints into a solution architecture and explicit decision record; use when shaping system boundaries, components, flows, interfaces, and tradeoffs in a local Markdown workflow.
---

# Architecture Design

Turn validated BA outputs into an explicit solution shape and architecture decision record without drifting into implementation planning.

For this repository's Solution Architect workflows, this is the preferred default design path for local Markdown artifacts.

## Quick start
1) Read the validated BA handoff and requirements artifacts.
2) Identify the architectural drivers, constraints, and major boundary decisions.
3) Shape the system boundary, major components, flows, interfaces, and dependency assumptions.
4) Compare plausible options when the choice is non-trivial.
5) Populate or strengthen the local solution architecture and architecture decisions artifacts.

## Workflow

### 1) Gather architecture inputs
- Start with the local requirements handoff and requirements specification artifacts.
- Pull in business objectives, process analysis, and technical constraints only as needed to explain the architecture honestly.
- Treat validated requirements as the architectural source of truth.

### 2) Identify the design drivers
- Make the architectural drivers explicit before choosing a solution shape.
- Include scale, latency, reliability, operational complexity, trust boundaries, compliance, delivery constraints, and integration conditions when they matter materially.
- Do not let a preferred stack or pattern substitute for explicit decision drivers.

### 3) Shape the solution
- Define the system boundary and what remains outside it.
- Identify the major components or subsystems and their responsibilities.
- Describe the major data, control, or orchestration flows.
- Make important interfaces, ownership seams, and contract assumptions explicit where downstream work could otherwise drift.

### 4) Evaluate major choices
- Record the decisions that materially affect the architecture.
- Compare alternatives when the choice is non-obvious or high impact.
- Capture rationale, tradeoffs, and follow-on impacts rather than implying them from the final design alone.

### 5) Produce the architecture artifacts
- Populate or improve the local solution architecture artifact.
- Populate or improve the local architecture decisions artifact.
- Keep unresolved questions, assumptions, and risks explicit.
- Route requirement ambiguity back upstream instead of solving it silently inside the architecture.

## Notes

- This skill stops at solution shaping and decision framing.
- It should not create implementation tasks, sprint plans, or delivery-management outputs.
- For AI-enabled or agentic systems, keep trust boundaries and human control points explicit.

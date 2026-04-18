---
name: architecture-handoff-packaging
description: Package reviewed architecture into a downstream-ready handoff artifact; use when summarizing solution shape, decision status, interface hotspots, risks, and next design questions in a local Markdown workflow.
---

# Architecture Handoff Packaging

Package reviewed architecture so downstream planning, implementation, and verification roles can continue without reconstructing the design from earlier notes.

For this repository's Solution Architect workflows, this is the preferred default handoff path for local Markdown architecture artifacts.

## Quick start
1) Read the reviewed architecture artifacts together.
2) Summarize the architecture and the current review status.
3) Highlight interface, dependency, and verification hotspots.
4) Separate unresolved design issues from settled architecture.
5) Populate or strengthen the local architecture handoff artifact.

## Workflow

### 1) Gather the handoff inputs
- Start with the local solution architecture, architecture decisions, and architecture review artifacts.
- Pull in upstream BA context only when it improves downstream clarity materially.
- Prefer the reviewed architecture package over scattered notes.

### 2) Summarize the architecture
- State what is being handed off and why.
- Describe the solution shape, major components, and key architectural decisions clearly enough for downstream roles to work from.

### 3) Package the risk areas
- Highlight important interface seams, ownership boundaries, integrations, and dependency hotspots.
- State the architectural assumptions and verification focus areas that downstream roles should watch closely.

### 4) Preserve review status
- Carry forward the architecture review status honestly.
- Keep blocked items, conditions, unresolved decisions, and major risks separate from the settled architecture.

### 5) Produce the handoff artifact
- Populate or improve the local architecture handoff artifact.
- Make the downstream consumers and next decision points explicit.
- Do not turn the handoff into implementation planning or task decomposition.

## Notes

- This skill packages architecture; it does not redesign it.
- It should stop at the architect-to-downstream boundary.
- It is generic and reusable, but it is especially useful in this repository's Solution Architect handoff workflow.

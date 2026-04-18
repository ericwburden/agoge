# Solution Architecture

## Purpose

Capture the proposed solution shape for a validated problem space, including architectural drivers, system boundary, major components, interfaces, integrations, constraints, and risks.

Use this artifact after business objectives, process needs, and requirements are stable enough to support architectural design.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Solution Architect work.

## Completion Guidance

This artifact is complete when a downstream reader can understand the intended system shape, why it was chosen, what constraints it must respect, where the important interfaces and contract assumptions lie, and where the major architectural risks and boundaries sit without inferring them from chat context.

## Related Checks

- Primary: [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md)
- Cross-cutting: [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
- Cross-cutting: [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)

## Problem And Scope

Summarize the problem space and the part of the system this architecture is addressing.

## Input Context

Reference the business objectives, process analysis, requirements specification, and requirements handoff this architecture depends on.

## Architectural Drivers

List the decision drivers that shape the architecture, such as scale, reliability, compliance, latency, operational complexity, trust boundaries, or integration constraints.

## System Boundary

Describe what is inside the solution boundary and what remains outside it.

## Major Components And Responsibilities

List the major components, services, modules, or subsystems and what each one is responsible for.

## Major Flows

Describe the key data flows, control flows, or orchestration flows the system must support.

## Interfaces And Contracts

Describe the important interfaces or boundary seams the architecture depends on, including:

- who owns each boundary
- what crosses it
- contract expectations or assumptions
- important failure, retry, timeout, or fallback expectations when they matter materially

## Integrations And External Dependencies

List the major integrations, external systems, and dependency assumptions the architecture relies on.

## Constraints

Capture the technical, policy, platform, operational, or delivery constraints that materially shape the architecture.

## Trust Boundaries And Human Control Points

If the system includes AI-enabled or agentic behavior, describe the relevant trust boundaries, escalation points, approval steps, or human intervention expectations.

## Risks And Open Questions

List architectural risks, tradeoffs still under discussion, and questions that remain unresolved.

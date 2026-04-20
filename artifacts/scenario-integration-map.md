# Scenario Integration Map

## Purpose

Capture how a reusable scenario composes role-owned workflows into one coherent multi-role chain, including handoffs, dependencies, branching rules, synchronization points, and failure-routing expectations.

Use this artifact after the scenario definition exists and before the scenario is reviewed or handed off for adoption or execution.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Scenario Designer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand which role-owned workflows participate, what they consume and produce for one another, how the scenario branches or synchronizes, and where coordination risk or escalation is concentrated.

## Related Checks

- Primary: [`scenario-integration-map.check.md`](D:/Projects/orpheum/checks/scenario-integration-map.check.md)
- Cross-cutting: [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md)
- Cross-cutting: [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md)

## Scenario In Scope

Reference the corresponding scenario definition and state which scenario instance or lifecycle window this integration map covers.

## Participating Role-Owned Workflows

List the role-owned workflows that participate in the scenario and the role package each workflow belongs to.

## Workflow Inputs, Outputs, And Shared Artifacts

Describe what each participating workflow needs to start, what it is expected to produce, and which artifacts or shared context move between workflows.

## Handoff Contracts

Describe the explicit handoff expectations between workflows, including readiness conditions, information that must be preserved, and what the receiving workflow should not have to reconstruct.

## Branching Rules And Decision Logic

Describe the conditions that change scenario flow, including optional paths, conditional routes, approval outcomes, or escalation routes.

## Parallelism And Synchronization Points

Identify where work may proceed in parallel, where the scenario must reconverge, and what conditions must be satisfied before it can move forward.

## Shared Context, State, And Dependency Assumptions

Describe the scenario-wide context, state, or dependency assumptions that multiple workflows rely on.

## Failure Handling And Escalation Routing

Describe what should happen when a workflow fails, a gate is blocked, an artifact is missing, or a role boundary defect is discovered.

## Coordination Risks And Watchouts

Highlight the scenario-specific integration risks, ambiguity hotspots, or timing sensitivities that downstream consumers should watch closely.

## Recommended Next Step

Describe the immediate next step, such as scenario review, role-package remediation, or scenario handoff.

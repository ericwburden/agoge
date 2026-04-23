# Project Discovery Scenario Integration Map

## Purpose

Capture how the `Project Discovery` scenario composes role-owned workflows into one coherent upstream discovery phase, including handoffs, dependencies, optional branches, and downstream routing expectations.

## Scenario In Scope

This integration map applies to the reusable `Project Discovery` scenario defined in [project-discovery.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.definition.md).

## Participating Role-Owned Workflows

- [`business-analyst-kickoff.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-kickoff.md)
- [`business-analyst-process-analysis.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-process-analysis.md)
- [`business-analyst-requirements-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-requirements-handoff.md)
- optional [`product-owner-direction.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-direction.md)
- optional [`product-owner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-review.md)
- optional [`product-owner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Business Analyst workflows consume the kickoff request, rough notes, release-driven signal, or equivalent initiating context, then produce:
  - business objectives
  - process analysis
  - requirements specification
  - requirements handoff
- Optional Product Owner workflows consume the validated discovery package, then produce:
  - product direction
  - backlog prioritization
  - product decision review
  - product handoff when early product-readiness framing materially shapes whether the work should proceed into planning

Shared artifacts and context that move across the scenario:

- the initiating project or problem signal
- business objectives, scope boundaries, and stakeholder confirmation posture
- current-state and future-state process understanding
- verified requirements, assumptions, dependencies, and unresolved questions
- optional early product-readiness framing when downstream planning should not proceed on discovery alone
- semantic-review findings, changed decisions, and cross-artifact reconciliations completed before planning handoff

## Handoff Contracts

- initiating context -> Business Analyst
  - Business Analyst should receive enough raw context to identify the business problem honestly, but should not be expected to infer validated requirements from kickoff chatter alone
- Business Analyst kickoff -> Business Analyst process analysis
  - Process analysis should receive an explicit business-objectives baseline rather than reconstructing scope and actors from notes again
- Business Analyst process analysis -> Business Analyst requirements handoff
  - Requirements work should receive explicit process needs, rules, gaps, and future-state intent rather than treating requirements as detached feature wishes
- Business Analyst -> optional Product Owner
  - Product Owner should receive a validated discovery package when product-readiness or early priority framing is the unresolved question, rather than being asked to invent discovery from vague requirements language
- Business Analyst or optional Product Owner -> `Project Planning`
  - `Project Planning` should receive validated discovery and any material product-readiness framing rather than treating planning as the place where the business problem is first clarified
- discovery package -> semantic artifact review
  - semantic artifact review should happen artifact by artifact with the human in Planning Mode or the host environment's nearest equivalent, and should capture decision changes durably before planning handoff

## Branching Rules And Decision Logic

- If the initiating request is still too weak to describe a business problem honestly, keep the work inside Business Analyst kickoff rather than pretending discovery is complete.
- If process understanding remains too weak to support verified requirements, keep the work inside discovery rather than routing vague assumptions into planning.
- If the main unresolved issue is early product value or priority framing rather than discovery completeness, route through Product Owner before treating the package as planning-ready.
- If semantic artifact review exposes wrong architecture, package-boundary drift, or missing locked decisions, route remediation back to the earliest affected discovery artifact before planning handoff.
- If the issue revealed by discovery is actually a product reprioritization decision driven by recent release evidence, `Release Feedback To Reprioritization` may be the better entry path than new-project discovery.
- If the discovery package is stable and validated, route it downstream into `Project Planning`.

## Parallelism And Synchronization Points

- Product Owner framing may begin once Business Analyst requirements posture is explicit enough to support an honest product-readiness judgment, but the scenario should not treat product framing as a substitute for validated discovery.
- The scenario must reconverge at:
  - explicit business objectives before process analysis relies on discovery scope
  - explicit process analysis before requirements are treated as verified
  - explicit requirements handoff before downstream planning relies on the discovery package
  - optional explicit product-readiness posture before the package is treated as ready for `Project Planning` when product framing materially affects readiness
  - completed semantic artifact review and cross-artifact reconciliation before downstream planning consumes the package

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes discovery is distinct from downstream planning because the business problem, process context, and requirement basis are still being validated here.
- The scenario assumes `Project Planning` is the normal downstream consumer once validated discovery is ready.
- The scenario assumes the current Business Analyst package is the core capability for this work, with Product Owner remaining an optional branch rather than the default owner of discovery.
- The scenario assumes the current Business Analyst and optional Product Owner packages are usable here as-is, with no additional scenario-specific hardening currently required.

## Failure Handling And Escalation Routing

- If discovery reveals the issue is really an implementation, review, verification, or release defect on an existing bounded slice, route to the appropriate downstream scenario rather than disguising it as new-project discovery.
- If discovery reveals a stable release-driven learning that mainly requires product reprioritization rather than fresh business analysis, route to `Release Feedback To Reprioritization`.
- If the package is not getting materially stronger, stop and make the missing discovery evidence explicit rather than handing weak discovery downstream.
- If stakeholder confirmation is still pending, preserve that uncertainty explicitly rather than implying planning-ready certainty.
- If semantic review changes decisions in one artifact, reconcile the affected discovery artifacts before treating the package as closed.

## Coordination Risks And Watchouts

- Business Analyst and Product Owner boundaries can blur if product-priority language starts replacing actual discovery validation.
- This scenario is easy to underuse, with teams jumping straight from a request into planning; the explicit discovery handoff should remain visible.
- This scenario is also easy to overread as indefinite research; once the discovery package is validated enough for planning, it should route forward rather than staying open by habit.
- A structurally complete discovery package can still be directionally wrong; semantic artifact review must stay visible as a separate closeout requirement.

## Recommended Next Step

Use the Project Discovery review artifact to make readiness, participant fit, optional product-readiness use, and downstream routing into `Project Planning` explicit before treating this scenario as adoption-ready.

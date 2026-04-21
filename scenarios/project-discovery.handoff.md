# Project Discovery Scenario Handoff

## Purpose

Package the reusable `Project Discovery` scenario for downstream adopters, workflow authors, or project leads so they can use it without reconstructing the discovery path from kickoff notes, scattered stakeholder context, or role-local BA artifacts alone.

## Current Scenario Summary

`Project Discovery` is a reusable multi-role scenario that starts from a project idea, kickoff request, release-driven follow-up need, or other early signal, turns that context into validated discovery through Business Analyst work, optionally adds early Product Owner framing when product-readiness is the unresolved issue, and produces a downstream-ready discovery package for `Project Planning`.

Primary participating roles:

- `Business Analyst`
- optional `Product Owner`

## Scenario Package Included

- [project-discovery.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.definition.md)
- [project-discovery.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.integration-map.md)
- [project-discovery.review.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.review.md)
- supporting rationale in [scenario-recommendations.md](C:/Users/ericw/Projects/orpheum/notes/scenario-recommendations.md)
- participant-package rationale in [business-analyst-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/business-analyst-skill-sourcing.md) and [product-owner-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/product-owner-skill-sourcing.md)
- the explicit participant-fit judgment that Business Analyst is the core owner of this scenario and Product Owner remains an optional readiness branch

## Current Readiness Posture

`ready`

This scenario is ready for downstream adoption and local tailoring as a reusable upstream discovery phase.

Limits:

- it is not a substitute for downstream planning
- it does not replace role-local workflows
- it does not include architecture design or implementation planning
- it is not a sprint board, delivery-management layer, or implementation-intake shortcut
- Product Owner participation should remain trigger-based rather than automatic

## Role And Workflow Routing Guidance

Use the scenario in this broad order:

1. Business Analyst kickoff to normalize the initiating request into explicit business objectives
2. Business Analyst process analysis to ground the discovery package in current-state and future-state process understanding
3. Business Analyst requirements handoff to produce verified requirements and a downstream-ready discovery package
4. optional Product Owner direction, review, and handoff when early product-readiness framing materially shapes whether the work should move into planning
5. hand the completed discovery package downstream into `Project Planning`

Downstream consumers should preserve:

- the rule that Business Analyst remains the core owner of validated discovery here
- the rule that Product Owner is optional unless product-readiness framing materially affects downstream planning readiness
- the distinction between validated discovery and downstream product direction, architecture, and planning
- the rule that `Project Planning` is the normal downstream consumer of a successful discovery package
- the rule that release-driven follow-up should route to `Release Feedback To Reprioritization` when the real issue is reprioritization rather than fresh discovery
- the participant-fit judgment that the current Business Analyst and optional Product Owner packages are usable here as-is unless repeated usage proves otherwise

## Entry Conditions For The Next Consumer

Before using this scenario, the next consumer should confirm:

- a real project, problem, or change signal exists
- validated discovery does not yet exist or is not yet strong enough for downstream planning
- the role packages referenced in the scenario are available
- the local context does not already belong in a downstream planning, remediation, or release-feedback scenario instead

## Active Conditions, Risks, And Watchouts

- avoid treating a stakeholder request as if it were already validated discovery
- avoid using Product Owner framing to bypass actual discovery validation
- avoid letting discovery drift into architecture, implementation planning, or release reprioritization work
- avoid keeping the scenario open once the package is already honest enough for `Project Planning`
- avoid routing release-driven learning into discovery when the real next step is product reprioritization

## Follow-Up Owners

- Scenario Designer for future scenario-level hardening
- Role Builder if repeated usage exposes gaps in the participating role packages

## Revisit Triggers

- repeated usage shows discovery packages still arrive too weak for `Project Planning`
- repeated usage shows Product Owner participation is too vague or too frequent
- repeated usage shows the repository needs a narrower request-triage scenario before discovery
- repeated usage shows one of the participating role packages no longer supports this scenario cleanly without hardening

## Recommended Next Consumer

- `Project Planning`
  - when validated discovery is complete enough to support product direction, architecture, and implementation planning
- `Release Feedback To Reprioritization`
  - when the real issue is release-driven reprioritization rather than fresh discovery
- `Scenario Designer`
  - when tailoring this reusable scenario for another context
- `Role Builder`
  - when repeated usage reveals a missing role-package capability

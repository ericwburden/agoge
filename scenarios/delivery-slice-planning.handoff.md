# Delivery Slice Planning Scenario Handoff

## Purpose

Package the reusable `Delivery Slice Planning` scenario for downstream adopters, workflow authors, or project leads so they can use it without reconstructing the scenario from the Scenario Designer package and role-local workflows.

## Current Scenario Summary

`Delivery Slice Planning` is a reusable multi-role scenario that starts from a broader reviewed planning package, selects the next priority candidate, turns that candidate into one bounded delivery slice while preserving architecture and dependency guardrails, optionally adds early verification or security/compliance framing when those concerns materially constrain the slice, and hands one slice-sized package downstream into `Implementation and Release Prep`.

Primary participating roles:

- `Product Owner`
- `Solution Architect`
- `Technical Planner`
- optional `QA / Verification Lead`
- optional `Security / Compliance Specialist`

## Scenario Package Included

- [delivery-slice-planning.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.definition.md)
- [delivery-slice-planning.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.integration-map.md)
- [delivery-slice-planning.review.md](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.review.md)
- supporting rationale in [scenario-recommendations.md](C:/Users/ericw/Projects/orpheum/notes/scenario-recommendations.md)
- participant-package rationale in [product-owner-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/product-owner-skill-sourcing.md), [solution-architect-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/solution-architect-skill-sourcing.md), [technical-planner-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/technical-planner-skill-sourcing.md), [qa-verification-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/qa-verification-skill-sourcing.md), and [security-compliance-specialist-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/security-compliance-specialist-skill-sourcing.md)
- the explicit participant-fit judgment that the current core role packages are usable in this scenario as-is, with QA / Verification Lead and Security / Compliance Specialist remaining optional trigger-based branches

## Current Readiness Posture

`ready`

This scenario is ready for downstream adoption and local tailoring as a reusable bounded-slice planning phase.

Limits:

- it is not a replacement for broader project planning
- it is not the downstream implementation scenario itself
- it is not sprint administration or progress tracking
- it does not replace role-local workflows
- optional QA / Verification Lead and Security / Compliance Specialist participation should remain trigger-based rather than automatic

## Role And Workflow Routing Guidance

Use the scenario in this broad order:

1. Product Owner direction, review, and handoff to select the next priority candidate and preserve current product guardrails
2. Solution Architect design, review, and handoff when architecture-sensitive seams or constraints need explicit confirmation before that candidate becomes the next slice
3. Technical Planner planning, review, and handoff to turn the selected candidate into a bounded implementation package
4. optional QA / Verification Lead planning when verification expectations materially constrain the slice before implementation begins
5. optional Security / Compliance Specialist scoping, review, and handoff when obligations, controls, or trust boundaries materially constrain the slice before implementation begins
6. hand the completed slice package downstream into `Implementation and Release Prep`

Downstream consumers should preserve:

- the rule that the scenario outputs one bounded slice package
- the distinction between Product Owner priority selection and the later architecture-and-planning work that makes the final slice boundary honest
- the distinction between broader project planning and slice-sized downstream work
- explicit slice exclusions, deferred scope, and dependency hotspots
- the rule that architecture-sensitive seams should stay visible when the slice is shaped
- the optional nature of QA / Verification Lead and Security / Compliance Specialist unless real slice-shaping triggers require them
- the participant-fit judgment that the current core role packages are usable here as-is unless repeated usage proves otherwise

## Entry Conditions For The Next Consumer

Before using this scenario, the next consumer should confirm:

- a broader reviewed planning package already exists
- the current target is too broad to serve as one honest implementation boundary without further slice shaping
- the next slice can be expressed in bounded in-scope and out-of-scope terms
- the role packages referenced in the scenario are available
- the local context does not actually require a broader delivery-management role that the repository does not yet define

## Active Conditions, Risks, And Watchouts

- avoid treating the broader project plan as if it were already a bounded implementation slice
- avoid letting Product Owner prioritization masquerade as technical decomposition without architectural or planning confirmation
- avoid letting Technical Planner absorb unresolved architecture or product ambiguity into slice prose
- avoid treating this scenario as sprint administration or team-capacity management
- avoid invoking QA / Verification Lead or Security / Compliance Specialist without a real slice-shaping trigger

## Follow-Up Owners

- Scenario Designer for future scenario-level hardening
- Role Builder if repeated usage exposes gaps in the participating role packages

## Revisit Triggers

- repeated usage shows the scenario still produces slices that are too broad downstream
- repeated usage shows one of the participating role packages no longer supports this scenario cleanly without hardening
- repeated usage shows stronger QA / Verification Lead trigger guidance is needed
- repeated usage shows stronger security/compliance trigger guidance is needed
- a future delivery-manager role is added that changes the right scenario boundary

## Recommended Next Consumer

- `Scenario Designer`
  - when tailoring this reusable scenario for another context
- `Role Builder`
  - when repeated usage reveals a missing role-package capability
- project leads or workflow authors
  - when choosing and applying a reusable slice-shaping scenario before bounded downstream delivery work

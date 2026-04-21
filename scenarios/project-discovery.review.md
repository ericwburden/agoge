# Project Discovery Scenario Review

## Purpose

Capture the current review posture for the reusable `Project Discovery` scenario before it is treated as ready for downstream adoption or tailoring.

## Review Scope

Scenario in scope:

- [project-discovery.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.definition.md)
- [project-discovery.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.integration-map.md)

Lifecycle window:

- initiating project or problem signal through validated discovery and downstream handoff into `Project Planning`

## Reviewed Inputs

- the Scenario Designer role package
- the participating role definitions for Business Analyst and optional Product Owner
- the role-owned workflows referenced in the integration map
- the adjacent downstream scenarios in [project-planning.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md) and [release-feedback-to-reprioritization.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.definition.md)
- the scenario recommendations note in [scenario-recommendations.md](C:/Users/ericw/Projects/orpheum/notes/scenario-recommendations.md)
- participant-package rationale in [business-analyst-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/business-analyst-skill-sourcing.md) and [product-owner-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/product-owner-skill-sourcing.md)
- participant-fit judgment that the current Business Analyst package is the core capability for this scenario and Product Owner is an optional branch when early product-readiness framing materially shapes whether the discovery package should proceed into planning

## Overall Assessment

The scenario closes the repository's main remaining upstream SDLC gap by turning discovery from an implicit role-covered precondition into a reusable scenario with explicit downstream routing into `Project Planning`.

The current package is coherent and should be treated as ready for downstream adoption and tailoring with explicit limits.

## Decision Status

`ready`

Basis for judgment:

- the scenario fills the previously explicit lifecycle gap before `Project Planning`
- the role participation is explicit and aligns with current role boundaries
- the current Business Analyst workflows already form a real discovery chain without additional scenario-specific hardening
- optional Product Owner participation is explicit and conditioned on genuine product-readiness needs rather than treated as automatic overhead
- the scenario keeps discovery separate from downstream product planning, architecture, and implementation work
- the scenario makes the downstream handoff into `Project Planning` explicit instead of leaving validated discovery as an implied precondition

## Integration Risks And Failure Modes

- Teams may still overuse Product Owner language too early and treat prioritization as a substitute for validated discovery.
- Teams may still bypass the scenario and jump directly from a request into `Project Planning` if intake discipline is weak.
- Release-driven follow-up work may sometimes be misrouted into discovery when it really belongs in `Release Feedback To Reprioritization`.
- The scenario may stay open too long if teams mistake ongoing research appetite for an actual missing discovery requirement.

## Conditions And Remediation Decisions

- Preserve the rule that Business Analyst remains the core owner of validated discovery in this scenario.
- Preserve the optional nature of Product Owner unless early product-readiness framing materially affects whether the work should proceed.
- Preserve the distinction between validated discovery and downstream planning.
- Preserve the rule that a validated discovery package should normally hand off into `Project Planning`.
- Preserve the rule that release-driven learning should route to `Release Feedback To Reprioritization` when the real next step is priority change rather than fresh discovery.

## Follow-Up Owners

- Scenario Designer
  - owns future strengthening of scenario-specific intake routing, optional Product Owner entry rules, and downstream planning handoff clarity
- Role Builder
  - owns any future changes that require underlying role-package refinement rather than scenario-only edits

## Revisit Triggers

- repeated usage shows the current Business Analyst package no longer supports this scenario cleanly without hardening
- repeated usage shows Product Owner is joining too early or too often in place of discovery validation
- repeated usage shows the repository needs a narrower request-triage scenario before full discovery work begins
- repeated usage shows discovery outputs still arrive too weak for `Project Planning`

## Recommended Next Step

Use the Project Discovery handoff artifact to package this scenario for downstream adoption in `scenarios/`.

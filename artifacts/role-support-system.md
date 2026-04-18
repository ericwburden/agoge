# Role Support System

## Purpose

Capture the operational support package for a reusable agent role: artifacts, workflows, checks, skills, and tool-preference decisions.

Use this artifact after the role definition is stable enough to design the supporting system that makes the role repeatable.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live role-design work.

## Completion Guidance

This artifact is complete when the role's artifact chain, workflow chain, quality gates, skill support, and handoff path are explicit enough that another agent or designer could implement or use the role consistently.

## Related Checks

- Primary: [`role-support-system.check.md`](D:/Projects/agoge/checks/role-support-system.check.md)
- Cross-cutting: [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md)
- Cross-cutting: [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md)

## Role In Scope

Name the role this support system is being designed for.

## Artifact Set

List the artifacts the role should produce by default and what each one is for.

## Workflow Set

List the workflows that should define, operationalize, review, and hand off the role's outputs.

## Check Set

List the primary and cross-cutting checks that act as definition-of-done quality gates.

## Skill Support Map

List the supporting skills, grouped as:

- direct support
- local adaptations
- adjacent or downstream only

## Tooling Preferences

Capture any explicit tool-preference decisions, such as local-first patterns or adaptation rules for tool-coupled methods.

## Allium Touchpoints

Describe how the role interacts with Allium across its lifecycle, including:

- which workflows should consume existing specifications
- when the role should produce, refine, or defer Allium work
- which installed Allium skills are relevant
- where specification gaps should be routed instead of improvised

If the role has no meaningful Allium relationship, state that explicitly.

## Lifecycle And Handoffs

Describe how the role moves from role idea or workshop intake through definition, support-system design, quality review, and downstream adoption handoff.

## Gaps And Risks

Describe missing support, weak links, or role-package risks that still need to be addressed.

## Open Questions

List unresolved questions about the supporting system.

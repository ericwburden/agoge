---
id: role-package-handoff
kind: artifact
title: Role Package Handoff
version: 1
summary: Package a reviewed reusable role so another repo, team, or downstream designer
  can adopt it without reconstructing the role package from earlier artifacts or chat
  context.
template: true
default_output_path: docs/orpheum/role-package-handoff.md
checks:
- role-package-handoff
- role-package-traceability
- role-builder-boundary
---

# Role Package Handoff

## Purpose

Package a reviewed reusable role so another repo, team, or downstream designer can adopt it without reconstructing the role package from earlier artifacts or chat context.

Use this artifact after the role definition, support system, and role package review are complete enough to support downstream adoption.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live role-package handoff work.

## Completion Guidance

This artifact is complete when a downstream adopter can understand what the role does, what support package comes with it, what limitations still apply, and what next steps are required without reopening the entire design history.

## Related Checks

- Primary: [`role-package-handoff.check.md`](D:/Projects/agoge/checks/role-package-handoff.check.md)
- Cross-cutting: [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md)
- Cross-cutting: [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md)

## Role In Scope

Name the role package being handed off.

## Package Summary

Summarize the role's job-to-be-done, lifecycle position, and why the package is worth adopting.

## Reviewed Inputs

List the role definition, support-system design, review artifact, and any other materials this handoff depends on.

## Review Outcome

State the package's readiness status and the review conclusions that matter for adoption.

## Adoption Guidance

Explain how adopters should use or adapt the role package, including where to start and what to preserve.

## Recommended Consumers

List the teams, repos, or downstream designers who are the intended consumers of this handoff.

## Current Limitations

List the limits, exclusions, or known weak areas adopters should understand before using the role package.

## Required Conventions And Preferences

Capture the conventions, tool preferences, or local-first assumptions adopters should preserve.

## Allium Expectations

Describe the role's expected relationship to Allium that adopters should preserve, including:

- whether the role consumes existing specifications
- whether it helps identify, create, refine, or defer specification work
- which installed Allium skills are part of the role's support model
- any boundaries on when the role should not improvise behavior outside the specification chain

## Next Adoption Steps

List the concrete next steps needed to adopt, adapt, or extend the role package downstream.

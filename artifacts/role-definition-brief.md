---
id: role-definition-brief
kind: artifact
title: Role Definition Brief
version: 1
summary: Capture the core definition of a reusable agent role before building its
  supporting artifacts, workflows, checks, and skill map.
template: true
default_output_path: docs/orpheum/role-definition-brief.md
checks:
- role-definition-brief
- role-package-traceability
- role-builder-boundary
---

# Role Definition Brief

## Purpose

Capture the core definition of a reusable agent role before building its supporting artifacts, workflows, checks, and skill map.

Use this artifact to define what job the role performs, where it fits in the lifecycle, what good looks like, and where the role should stop.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live role-design work.

## Completion Guidance

This artifact is complete when a downstream designer can understand the role's job, audience, boundaries, and major outputs without inferring missing intent from chat context.

## Related Checks

- Primary: [`role-definition-brief.check.md`](D:/Projects/agoge/checks/role-definition-brief.check.md)
- Cross-cutting: [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md)
- Cross-cutting: [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md)

## Role Name

State the stable name of the role being designed.

## Job To Be Done

Describe what job this role is meant to perform and why it should exist.

## Intended Audience And Consumers

Describe who will use or depend on this role, such as agent designers, project leads, or downstream delivery roles.

## Lifecycle Position

Describe where the role sits in the broader delivery lifecycle and what stages it should hand off to.

## Success Criteria

Describe what a good outcome looks like when this role is working correctly.

## Primary Responsibilities

List the responsibilities this role should own directly.

## Out Of Scope

List the responsibilities this role should explicitly not own.

## Default Outputs

Describe the major artifacts, decisions, or deliverables the role should produce by default.

## Allium Relationship

State whether this role should:

- consume existing Allium specifications
- help identify behavior that is ready to become or update an Allium specification
- refine or maintain Allium specifications directly
- route specification work to another role or skill

If Allium is not relevant to the role, say that explicitly.

## Source Material And References

Capture the references, examples, or observed practices this role is derived from.

## Assumptions

List assumptions currently shaping the role design.

## Open Questions

List unresolved questions that materially affect the role definition.

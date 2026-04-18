# Artifacts

This directory stores reusable scaffolding for common project artifacts used in AI-assisted delivery work.

Use this area for:

- kickoff templates
- plan templates
- review templates
- retrospective templates
- handoff templates

Templates here should reduce variance, make expectations explicit, and make good process cheaper to follow.

Treat the checked-in files in this directory as canonical artifact definitions. Do not use them as the live working copy for a specific project. Instead, copy the relevant artifact into a project-specific working location and fill out that instantiated copy there.

Start from [`artifact.template.md`](D:/Projects/agoge/artifacts/artifact.template.md) when creating a new reusable artifact template.

## Business Analyst Artifacts

The first concrete artifact set in this directory is aligned to the [`Business Analyst`](D:/Projects/agoge/roles/business-analyst.md) role.

- [`business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md) defines the reusable structure for capturing the business problem, goals, stakeholders, context, and scope boundaries.
- [`process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md) defines the reusable structure for capturing as-is and to-be process understanding, business rules, actors, triggers, and gaps.
- [`requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md) defines the reusable structure for capturing verified business requirements with rationale, scope, and open questions.
- [`requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) defines the reusable structure for capturing downstream-ready traceability, risks, unresolved questions, and handoff guidance.

Use this set when a project needs structured kickoff and discovery outputs without inventing a custom artifact library from scratch. Instantiate the needed artifact files into the project workspace before filling them out.

Each Business Analyst artifact links to its primary definition-of-done check and the two cross-cutting BA checks for traceability and role-boundary discipline. The intended lifecycle is: select template, instantiate project copy, populate it, run checks, remediate failures, then hand it downstream.

## Role Builder Artifacts

The next concrete artifact set in this directory is aligned to the [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) role.

- [`role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md) defines the reusable structure for capturing a role's job-to-be-done, lifecycle position, boundaries, success criteria, and default outputs.
- [`role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md) defines the reusable structure for capturing a role's artifact set, workflow set, check set, skill support, and tooling preferences.
- [`role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md) defines the reusable structure for reviewing the coherence and readiness of a role package.
- [`role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md) defines the reusable structure for packaging a reviewed role for downstream adoption, including guidance, limitations, consumers, and next steps.

Use this set when defining or tightening a reusable role package rather than producing project-specific delivery artifacts. Instantiate the needed artifact files into the working location before filling them out.

The intended Role Builder lifecycle is: select template, instantiate working copy, populate the role definition, derive the support system, review the package, package it for adoption, and remediate any failed checks before treating the role as reusable downstream.

## Solution Architect Artifacts

The next concrete artifact set in this directory is aligned to the [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role.

- [`solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md) defines the reusable structure for capturing architectural drivers, system boundaries, major components, flows, interfaces, integrations, constraints, and risks.
- [`architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md) defines the reusable structure for capturing major architectural choices, alternatives, rationale, and consequences.
- [`architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md) defines the reusable structure for capturing durable architecture review findings, readiness, remediation, and unresolved concerns before handoff.
- [`architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md) defines the reusable structure for packaging reviewed architecture for downstream planning, implementation, and verification roles.

Use this set when validated BA outputs need to be turned into a technical solution shape and downstream architectural handoff. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Solution Architect lifecycle is: select template, instantiate working copy, define the solution architecture, record major decisions, review the architecture explicitly, package the architecture handoff, run checks, remediate failures, and only then move the architecture downstream.

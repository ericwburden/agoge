# Checks

This directory stores evaluation and validation patterns for AI-assisted workflows and project practices.

Use this area for:

- checklist-based reviews
- workflow evaluation rubrics
- definition-of-done style quality gates
- repeatable validation criteria for agent outputs

Checks here should be concrete enough that an agent or reviewer can apply them consistently across projects.

Start from [`workflow-check.template.md`](D:/Projects/agoge/checks/workflow-check.template.md) when defining a new evaluation or validation pattern.

## Business Analyst Check Set

The first concrete check set in this directory supports the [`Business Analyst`](D:/Projects/agoge/roles/business-analyst.md) role and its artifact library.

- [`business-objectives.check.md`](D:/Projects/agoge/checks/business-objectives.check.md) validates the business objectives artifact.
- [`process-analysis.check.md`](D:/Projects/agoge/checks/process-analysis.check.md) validates the process analysis artifact.
- [`requirements-specification.check.md`](D:/Projects/agoge/checks/requirements-specification.check.md) validates the requirements specification artifact.
- [`requirements-handoff.check.md`](D:/Projects/agoge/checks/requirements-handoff.check.md) validates the requirements handoff artifact.
- [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md) validates linkage across the BA artifact chain.
- [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) validates that BA outputs stay within role boundaries.

These checks are definition-of-done quality gates rather than advisory rubrics. A failing check means the artifact or artifact chain needs rework before it should be treated as ready.

Apply these checks to instantiated project artifacts derived from the templates in [`artifacts/`](D:/Projects/agoge/artifacts), not to the checked-in template files themselves.

Where a check depends on a specific repo skill to remediate or complete the work, that skill is referenced directly inside the check definition. The current BA check set is fully supported by the existing BA-oriented skills, so no additional check-only skill was required for this pass.

The intended BA lifecycle is: instantiate the artifact template, populate the working copy, run the required primary and cross-cutting checks, remediate failures with the linked skills, and only then move the artifact downstream.

## Role Builder Check Set

The next concrete check set in this directory supports the [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) role and its artifact library.

- [`role-definition-brief.check.md`](D:/Projects/agoge/checks/role-definition-brief.check.md) validates the role definition brief.
- [`role-support-system.check.md`](D:/Projects/agoge/checks/role-support-system.check.md) validates the role support system artifact.
- [`role-package-review.check.md`](D:/Projects/agoge/checks/role-package-review.check.md) validates the role package review artifact.
- [`role-package-handoff.check.md`](D:/Projects/agoge/checks/role-package-handoff.check.md) validates the adoption-facing role package handoff artifact.
- [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) validates linkage across the Role Builder artifact chain.
- [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md) validates that role-design outputs stay inside the intended role boundary.

These checks are definition-of-done quality gates for the Role Builder lifecycle: define the role, design the support system, review the package, package it for adoption, and remediate failures at the earliest stage that introduced the defect.

## Solution Architect Check Set

The next concrete check set in this directory supports the [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role and its artifact library.

- [`solution-architecture.check.md`](D:/Projects/agoge/checks/solution-architecture.check.md) validates the solution architecture artifact.
- [`architecture-decisions.check.md`](D:/Projects/agoge/checks/architecture-decisions.check.md) validates the architecture decisions artifact.
- [`architecture-review.check.md`](D:/Projects/agoge/checks/architecture-review.check.md) validates the durable architecture review artifact.
- [`architecture-handoff.check.md`](D:/Projects/agoge/checks/architecture-handoff.check.md) validates the architecture handoff artifact.
- [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md) validates linkage from upstream BA artifacts through the architecture chain.
- [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md) validates that architecture outputs stay within role boundaries.

These checks are definition-of-done quality gates for the Solution Architect lifecycle: define the solution shape, record the major decisions, review the architecture explicitly, package the architecture handoff, and remediate failures at the earliest stage that introduced the defect.

## Technical Planner Check Set

The next concrete check set in this directory supports the [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role and its artifact library.

- [`implementation-strategy.check.md`](D:/Projects/agoge/checks/implementation-strategy.check.md) validates the implementation strategy artifact.
- [`sequencing-and-dependencies.check.md`](D:/Projects/agoge/checks/sequencing-and-dependencies.check.md) validates the sequencing and dependencies artifact.
- [`implementation-plan-review.check.md`](D:/Projects/agoge/checks/implementation-plan-review.check.md) validates the durable planning review artifact.
- [`implementation-handoff.check.md`](D:/Projects/agoge/checks/implementation-handoff.check.md) validates the implementation handoff artifact.
- [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md) validates linkage from upstream BA and architecture artifacts through the planning chain.
- [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md) validates that planning outputs stay within role boundaries.

These checks are definition-of-done quality gates for the Technical Planner lifecycle: define the implementation strategy, define sequencing and dependencies, review the plan explicitly, package the implementation handoff, and remediate failures at the earliest stage that introduced the defect.

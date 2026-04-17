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

The first concrete check set in this directory supports the [`Business Analyst`](D:/Projects/agoge/agents/business-analyst.md) role and its artifact library.

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

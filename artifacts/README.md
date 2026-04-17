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

The first concrete artifact set in this directory is aligned to the [`Business Analyst`](D:/Projects/agoge/agents/business-analyst.md) role.

- [`business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md) defines the reusable structure for capturing the business problem, goals, stakeholders, context, and scope boundaries.
- [`process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md) defines the reusable structure for capturing as-is and to-be process understanding, business rules, actors, triggers, and gaps.
- [`requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md) defines the reusable structure for capturing verified business requirements with rationale, scope, and open questions.
- [`requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) defines the reusable structure for capturing downstream-ready traceability, risks, unresolved questions, and handoff guidance.

Use this set when a project needs structured kickoff and discovery outputs without inventing a custom artifact library from scratch. Instantiate the needed artifact files into the project workspace before filling them out.

Each Business Analyst artifact links to its primary definition-of-done check and the two cross-cutting BA checks for traceability and role-boundary discipline. The intended lifecycle is: select template, instantiate project copy, populate it, run checks, remediate failures, then hand it downstream.

# Roles

This directory stores reusable role templates and related role-guidance patterns.

Use this area for:

- baseline role templates for new projects
- variants tuned for specific repository types or delivery contexts
- experiments in role operating guidance that may later be standardized

Templates here should favor clarity, reuse, and explicit tradeoffs over project-specific assumptions.

Start from [`role.template.md`](D:/Projects/agoge/roles/role.template.md) when creating a new reusable role definition.

## Concrete Roles

- [`business-analyst.md`](D:/Projects/agoge/roles/business-analyst.md) defines a structured interviewer role for early project discovery, business requirements, and process analysis.
- [`role-builder.md`](D:/Projects/agoge/roles/role-builder.md) defines a meta-role for designing reusable agent roles together with their artifacts, workflows, checks, supporting skills, and adoption handoff.
- [`solution-architect.md`](D:/Projects/agoge/roles/solution-architect.md) defines the architecture role that turns validated requirements into a solution shape, decision record, review outcome, and downstream technical handoff.
- [`technical-planner.md`](D:/Projects/agoge/roles/technical-planner.md) defines the execution-planning role that turns reviewed architecture and validated requirements into an implementation strategy, sequencing plan, reviewed planning package, and downstream implementation handoff.

Use the Business Analyst role when a project needs help clarifying business goals, documenting as-is and to-be processes, verifying requirements, maintaining traceability, and preparing downstream roles for solutioning without mixing in implementation ownership too early.

Use the Role Builder role when a project needs to define a new reusable agent role or tighten the coherence of an existing role package across its supporting artifacts, workflows, checks, skills, and downstream adoption guidance.

Use the Solution Architect role when validated BA outputs need to be turned into an explicit system shape, architectural decisions, integration framing, reviewed readiness, and a downstream technical handoff without collapsing into implementation planning.

Use the Technical Planner role when reviewed architecture and validated requirements need to be turned into an execution-ready implementation strategy, explicit sequencing and dependency handling, reviewed readiness, and a downstream implementation handoff without collapsing into sprint administration or code ownership.

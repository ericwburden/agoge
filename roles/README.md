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
- [`qa-verification-lead.md`](D:/Projects/agoge/roles/qa-verification-lead.md) defines the verification role that turns reviewed requirements, architecture, planning, and implementation context into a verification strategy, traceable evidence matrix, reviewed readiness judgment, and downstream verification handoff.
- [`implementation-engineer.md`](D:/Projects/orpheum/roles/implementation-engineer.md) defines the implementation role that turns reviewed planning and architecture direction into a concrete change set, explicit implementation evidence, a reviewed implementation package, and downstream review-ready handoff.
- [`code-reviewer.md`](D:/Projects/orpheum/roles/code-reviewer.md) defines the review role that turns a completed implementation package into explicit review scope, concrete findings, an independent review decision, and downstream handoff guidance.
- [`release-handoff-manager.md`](D:/Projects/orpheum/roles/release-handoff-manager.md) defines the release packaging role that turns reviewed implementation, review, and verification outputs into explicit release posture, rollout notes, and downstream release or adoption handoff guidance.
- [`product-owner.md`](D:/Projects/orpheum/roles/product-owner.md) defines the product-governance role that turns validated requirements, product feedback, and delivery learnings into explicit product direction, backlog prioritization, reviewed product decisions, and downstream product handoff guidance.

Use the Business Analyst role when a project needs help clarifying business goals, documenting as-is and to-be processes, verifying requirements, maintaining traceability, and preparing downstream roles for solutioning without mixing in implementation ownership too early.

Use the Role Builder role when a project needs to define a new reusable agent role or tighten the coherence of an existing role package across its supporting artifacts, workflows, checks, skills, and downstream adoption guidance.

In this repository, `Role Builder hardening pass` is the short trigger phrase for the standard role-package quality review.

Use the Solution Architect role when validated BA outputs need to be turned into an explicit system shape, architectural decisions, integration framing, reviewed readiness, and a downstream technical handoff without collapsing into implementation planning.

Use the Technical Planner role when reviewed architecture and validated requirements need to be turned into an execution-ready implementation strategy, explicit sequencing and dependency handling, reviewed readiness, and a downstream implementation handoff without collapsing into sprint administration or code ownership.

Use the QA / Verification Lead role when reviewed requirements, architecture, planning, and implementation context need to be turned into an evidence-based verification strategy, explicit coverage and gap framing, reviewed readiness, and a downstream verification handoff without collapsing into implementation ownership or release operations.

Use the Implementation Engineer role when reviewed implementation direction needs to be turned into an explicit code change set, durable implementation evidence, a reviewed implementation package, and a downstream handoff without collapsing into planning, formal review ownership, or release operations.

Use the Code Reviewer role when a completed implementation package needs independent defect-finding, explicit approval posture, and downstream-ready review context without collapsing into implementation ownership, full QA authority, or release approval.

Use the Release / Handoff Manager role when reviewed implementation, review, and verification outputs need to be turned into explicit release posture, rollout caveats, and downstream-ready handoff guidance without collapsing into deployment execution, engineering ownership, or QA authority.

Use the Product Owner role when validated requirements, feedback, and delivery learnings need to be turned into explicit product direction, accountable prioritization, reviewed product decisions, and downstream-ready handoff guidance without collapsing into broad discovery, technical design, sprint administration, or release operations.

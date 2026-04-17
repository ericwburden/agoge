# Workflows

This directory stores higher-level workflow instructions that combine multiple skills into more capable delivery patterns.

Use this area for:

- chained skill workflows for common delivery tasks
- repeatable multi-step operating procedures
- orchestration guidance for moving from specification to implementation to validation

Workflows should describe when to use the chain, which skills are involved, expected inputs and outputs, and how success is evaluated.

Start from [`workflow.template.md`](D:/Projects/agoge/workflows/workflow.template.md) when creating a new reusable workflow.

## Business Analyst Workflow Set

The first concrete workflow set in this directory is aligned to the [`Business Analyst`](D:/Projects/agoge/agents/business-analyst.md) role and the Business Analyst artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`business-analyst-kickoff.md`](D:/Projects/agoge/workflows/business-analyst-kickoff.md) drives kickoff and discovery into the business objectives artifact.
- [`business-analyst-process-analysis.md`](D:/Projects/agoge/workflows/business-analyst-process-analysis.md) turns discovery outputs into as-is/to-be process analysis.
- [`business-analyst-requirements-handoff.md`](D:/Projects/agoge/workflows/business-analyst-requirements-handoff.md) turns verified discovery into requirements specification and downstream handoff artifacts.

## Business Analyst Skill Review

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): transcript and rough-notes normalization into structured summaries, decisions, risks, and actions.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): optional external research and citation support when BA work needs domain or market context.

### Adapted For Local Markdown

- [`meeting-intelligence`](D:/Projects/agoge/skills/meeting-intelligence/SKILL.md): local-Markdown adaptation of meeting prep and contextual agenda building.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): local-Markdown adaptation of research synthesis and structured documentation.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md): verifies requirements against BA discovery evidence and strengthens the requirements specification artifact.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md): packages verified requirements into a downstream-ready handoff artifact with traceability and decision framing.
- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md): local-Markdown adaptation of spec parsing and downstream planning translation.

### Downstream Or Adjacent Only

- [`create-plan`](D:/Projects/agoge/skills/create-plan/SKILL.md): useful after BA handoff, but not part of core BA discovery.
- [`meeting-insights-analyzer`](D:/Projects/agoge/skills/meeting-insights-analyzer/SKILL.md): communication coaching rather than requirements or process analysis.
- [`linear`](D:/Projects/agoge/skills/linear/SKILL.md), [`connect`](D:/Projects/agoge/skills/connect/SKILL.md), and [`connect-apps`](D:/Projects/agoge/skills/connect-apps/SKILL.md): optional operational integrations rather than default BA workflow dependencies.
- [`internal-comms`](D:/Projects/agoge/skills/internal-comms/SKILL.md): optional packaging for stakeholder communications, not a default BA skill.

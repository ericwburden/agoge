# Workflows

This directory stores higher-level workflow instructions that combine multiple skills into more capable delivery patterns.

Use this area for:

- chained skill workflows for common delivery tasks
- repeatable multi-step operating procedures
- orchestration guidance for moving from specification to implementation to validation

Workflows should describe when to use the chain, which skills are involved, expected inputs and outputs, and how success is evaluated.

For this repository, workflows should treat checked-in artifact files as reusable definitions. Live project work should operate on instantiated artifact copies in a project workspace, not on the files in [`artifacts/`](D:/Projects/agoge/artifacts) directly.

Start from [`workflow.template.md`](D:/Projects/agoge/workflows/workflow.template.md) when creating a new reusable workflow.

## Business Analyst Workflow Set

The first concrete workflow set in this directory is aligned to the [`Business Analyst`](D:/Projects/agoge/roles/business-analyst.md) role and the Business Analyst artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`business-analyst-kickoff.md`](D:/Projects/agoge/workflows/business-analyst-kickoff.md) drives kickoff and discovery into the business objectives artifact.
- [`business-analyst-process-analysis.md`](D:/Projects/agoge/workflows/business-analyst-process-analysis.md) turns discovery outputs into as-is/to-be process analysis.
- [`business-analyst-requirements-handoff.md`](D:/Projects/agoge/workflows/business-analyst-requirements-handoff.md) turns verified discovery into requirements specification and downstream handoff artifacts.
- [`business-analyst-quality-review.md`](D:/Projects/agoge/workflows/business-analyst-quality-review.md) runs the BA check chain and routes remediation before downstream handoff.

## Business Analyst Skill Review

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): transcript and rough-notes normalization into structured summaries, decisions, risks, and actions.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): optional external research and citation support when BA work needs domain or market context.

### Adapted For Local Markdown

- [`meeting-intelligence`](D:/Projects/agoge/skills/meeting-intelligence/SKILL.md): preferred local-Markdown meeting-prep path for this repo's BA workflows.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): preferred local-Markdown research and synthesis path for this repo's BA workflows.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md): preferred local-Markdown path for verifying requirements against BA discovery evidence and strengthening the requirements specification artifact.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging verified requirements into a downstream-ready handoff artifact with traceability and decision framing.
- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md): preferred local-Markdown downstream bridge from mature BA outputs into implementation planning.

The local-Markdown skills above are the default operating path for this repository. Treat the `notion-*` skills as reference implementations or optional adaptation sources, not as equal defaults for BA work here.

Business Analyst work should also be Allium-aware: consume existing Allium specs when they already define relevant behavior, and use the installed Allium skills only when mature verified behavior is ready to become or update a specification.

### Downstream Or Adjacent Only

- [`create-plan`](D:/Projects/agoge/skills/create-plan/SKILL.md): useful after BA handoff, but not part of core BA discovery.
- [`meeting-insights-analyzer`](D:/Projects/agoge/skills/meeting-insights-analyzer/SKILL.md): communication coaching rather than requirements or process analysis.
- [`linear`](D:/Projects/agoge/skills/linear/SKILL.md), [`connect`](D:/Projects/agoge/skills/connect/SKILL.md), and [`connect-apps`](D:/Projects/agoge/skills/connect-apps/SKILL.md): optional operational integrations rather than default BA workflow dependencies.
- [`internal-comms`](D:/Projects/agoge/skills/internal-comms/SKILL.md): optional packaging for stakeholder communications, not a default BA skill.

## Role Builder Workflow Set

The next concrete workflow set in this directory is aligned to the [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) role and the Role Builder artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`role-builder-role-definition.md`](D:/Projects/agoge/workflows/role-builder-role-definition.md) turns a role idea and source references into a reusable role definition brief.
- [`role-builder-support-system.md`](D:/Projects/agoge/workflows/role-builder-support-system.md) turns the role definition into an explicit support package of artifacts, workflows, checks, and skill support.
- [`role-builder-quality-review.md`](D:/Projects/agoge/workflows/role-builder-quality-review.md) reviews the role package for coherence, readiness, and remediation needs.
- [`role-builder-handoff.md`](D:/Projects/agoge/workflows/role-builder-handoff.md) packages a reviewed role for adoption by another repo, team, or downstream designer.

The intended Role Builder lifecycle in this repo is: role idea or workshop intake -> role definition -> support-system design -> quality review -> adoption handoff.

## Role Builder Skill Review

### Direct Support

- [`role-definition`](D:/Projects/agoge/skills/role-definition/SKILL.md): preferred local-Markdown path for turning a role idea and source references into a reusable role-definition brief.
- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md): preferred local-Markdown path for deriving the minimum viable support system from a role definition.
- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md): preferred local-Markdown path for reviewing role-package coherence, recording findings, and producing a readiness assessment.
- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md): preferred local-Markdown path for packaging a reviewed role for downstream adoption without requiring reconstruction from earlier artifacts.
- [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md): preferred local-first path for translating tool-coupled or external patterns into reusable repo-native methods.

### Workshop Intake Support

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): explicit intake path when role-definition work starts from workshop notes, interview notes, or meeting transcripts that need to be normalized before the dedicated role-definition workflow can proceed.

### Auxiliary Synthesis

- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): conditional synthesis helper when local precedents or notes need to be combined before the dedicated `role-*` skills can operate cleanly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): conditional external-reference helper when comparison material needs to be summarized before role-definition or adaptation work.

### Adjacent Or Future Support

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md): plausible model for future role-package verification or traceability support if Role Builder needs stronger dedicated validation skills.

The dedicated `role-*` skills and `pattern-adaptation` are the primary operating path for this repository's Role Builder workflows. Generic synthesis skills remain available, but they should be used only as supporting input steps rather than as the main Role Builder path.

Role Builder work should make each role's relationship to Allium explicit whenever the role touches product behavior, specification refinement, test propagation, or spec-code alignment.

## Solution Architect Workflow Set

The next concrete workflow set in this directory is aligned to the [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role and the Solution Architect artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`solution-architect-design.md`](D:/Projects/agoge/workflows/solution-architect-design.md) turns validated BA outputs into a solution architecture and architecture decision record.
- [`solution-architect-review.md`](D:/Projects/agoge/workflows/solution-architect-review.md) reviews drafted architecture, records findings, and decides whether the package is ready for downstream use.
- [`solution-architect-handoff.md`](D:/Projects/agoge/workflows/solution-architect-handoff.md) packages completed architecture for downstream planning, implementation, and verification roles.
- [`solution-architect-quality-review.md`](D:/Projects/agoge/workflows/solution-architect-quality-review.md) runs the Solution Architect check chain and routes remediation before downstream use.

The intended Solution Architect lifecycle in this repo is: validated BA handoff -> solution design -> architecture review -> architecture handoff -> final quality review and remediation -> downstream technical work.

## Solution Architect Skill Review

### Direct Support

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md): preferred local-Markdown path for turning validated BA outputs into a solution shape, explicit interfaces and contracts, and a decision record.
- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md): preferred local-Markdown path for reviewing architecture readiness, recording findings, and deciding whether the design should move downstream.
- [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed architecture into a downstream-ready handoff.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when technical constraints, local references, or architectural context need to be combined before architecture work can proceed cleanly.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): normalize architecture workshop notes or technical design-session transcripts.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): optional external research and citation support when architecture direction depends on external patterns, standards, or market context.

### Downstream Or Adjacent Only

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md): downstream bridge from mature architecture and requirements into implementation planning.
- [`create-plan`](D:/Projects/agoge/skills/create-plan/SKILL.md): useful after architecture handoff, but not part of core Solution Architect work.

### Remaining Design Choice

- No dedicated interface-framing or integration-contracts skill exists yet. For now, interface and contract coverage is embedded directly in `architecture-design`, the architecture artifacts, and the Solution Architect checks.

Solution Architect work should treat existing Allium specifications as behavioral input constraints, but architecture artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Technical Planner Workflow Set

The next concrete workflow set in this directory is aligned to the [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role and the Technical Planner artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`technical-planner-planning.md`](D:/Projects/agoge/workflows/technical-planner-planning.md) turns reviewed architecture and validated requirements into an implementation strategy and sequencing plan.
- [`technical-planner-review.md`](D:/Projects/agoge/workflows/technical-planner-review.md) reviews drafted planning outputs, records findings, and decides whether the package is ready for downstream use.
- [`technical-planner-handoff.md`](D:/Projects/agoge/workflows/technical-planner-handoff.md) packages completed planning outputs for downstream implementation and verification roles.
- [`technical-planner-quality-review.md`](D:/Projects/agoge/workflows/technical-planner-quality-review.md) runs the Technical Planner check chain and routes remediation before downstream use.

The intended Technical Planner lifecycle in this repo is: reviewed architecture handoff -> implementation strategy and sequencing -> planning review -> implementation handoff -> final quality review and remediation -> downstream technical work.

## Technical Planner Skill Review

### Direct Support

- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md): preferred local-Markdown path for turning reviewed architecture and validated requirements into implementation strategy, sequencing, and downstream planning structure.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed planning outputs into a downstream-ready implementation handoff.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when planning context, dependency notes, or rollout constraints need to be combined before execution planning can proceed cleanly.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): normalize planning workshops, dependency reviews, or implementation working-session transcripts.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): optional external research and citation support when execution order depends on external standards, migration patterns, or platform constraints.

### Downstream Or Adjacent Only

- [`create-plan`](D:/Projects/agoge/skills/create-plan/SKILL.md): useful for ad hoc planning requests in chat, but not the default artifact-chain path for this repository.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md): upstream quality support rather than a core Technical Planner dependency.
- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md): upstream architecture-readiness support rather than a default Technical Planner skill.

### Remaining Design Choice

- No dedicated sequencing-specific or planning-review skill exists yet. For now, sequencing and review coverage is embedded directly in `spec-to-implementation`, the planning artifacts, and the Technical Planner checks.

Technical Planner work should treat existing Allium specifications as behavioral scope and verification constraints, but planning artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

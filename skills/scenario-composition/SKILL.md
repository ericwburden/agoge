---
name: scenario-composition
description: Compose reusable multi-role scenarios from existing role packages and workflow chains; use when Scenario Designer work needs explicit scenario boundary framing, role participation logic, handoff contracts, branching rules, synchronization points, readiness posture, and re-review triggers without drifting into role definition, project management, or live execution control.
---

# Scenario Composition

Turn a repeatable multi-role activity plus the relevant role packages into a coherent Scenario Designer package that downstream adopters, workflow authors, or execution planners can use without reconstructing the choreography from scattered role definitions and workflow files.

For this repository, this is the preferred direct-support skill for the Scenario Designer role when the work needs explicit scenario-definition, integration-mapping, and scenario-posture discipline rather than only general synthesis, adaptation, or handoff packaging.

## Quick start

1. Read the scenario idea together with the participating role packages.
2. Define the actual scenario boundary, lifecycle window, and intended reuse horizon.
3. Make role participation, handoffs, branching rules, and synchronization points explicit.
4. Distinguish reusable scenario structure from role design, project management, and live execution control.
5. Make conditions, approval limits, and re-review triggers explicit so downstream consumers know when the current scenario package no longer applies.

## Workflow

### 1) Read the scenario context

- Start with the scenario idea, target lifecycle phase or activity, relevant role definitions, role-owned workflows, and any role-package notes that materially constrain the scenario.
- Pull in workshop notes, public orchestration references, tailoring notes, or approval context only as needed to understand the scenario honestly.
- Keep the scenario boundary explicit instead of silently broadening it into a general operating model or a live project plan.

### 2) Frame the scenario boundary clearly

- State what repeatable multi-role activity, lifecycle window, or phase is actually in scope.
- Distinguish participating roles, optional participants, excluded work, and explicitly deferred scenario concerns.
- Preserve scenario-sensitive constraints such as approvals, trust-boundary-sensitive transitions, artifact readiness conditions, or synchronization hotspots.

### 3) Author the scenario choreography cleanly

For the current scenario package, capture:

- the scenario intent and reuse target
- the participating roles and role-owned workflows
- the entry conditions and expected downstream-ready outputs
- the broad step sequence and major gates
- the handoff contracts between workflows
- the branching rules, synchronization points, and failure-routing expectations
- the shared dependencies, context assumptions, and coordination watchouts

- Do not let scenario logic stay implicit in role names or phase labels.
- Do not let a scenario depend on role responsibilities that do not actually exist in the underlying role packages.

### 4) Preserve readiness posture without overclaiming

- Record whether the scenario package is ready for downstream adoption, ready with conditions, or blocked.
- Keep role-package gaps, evidence gaps, approval limits, and tailoring needs explicit rather than smoothing them into generic readiness language.
- Distinguish reusable scenario readiness from live project execution authority.

### 5) Make re-review and revisit conditions explicit

- State what changes in lifecycle scope, participating role packages, approvals, specifications, or environment assumptions should trigger another scenario pass.
- Do not let scenario validity silently extend beyond the reviewed context, role package set, or intended adoption target.
- Preserve separation between scenario composition, role design, technical design, and project-instance execution management.

## Guardrails

- Do not redefine roles that should remain owned by Role Builder or the underlying role packages.
- Do not turn reusable scenario design into project management, sprint administration, staffing, or status reporting.
- Do not present a reusable scenario package as if it were already a live execution plan or authorization.
- Do not hide missing handoff logic, weak role support, or unresolved approval dependencies behind a polished phase narrative.
- Do not let branching rules, synchronization points, or failure-routing expectations stay implicit.

## Supporting skills

- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the scenario context or supporting role-package evidence is spread across multiple local sources.
- Use [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when the strongest source pattern is tool-coupled or framework-specific and needs cleaner repo-native translation.
- Use [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is downstream packaging clarity after the scenario posture is already clear.
- Use [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when the inputs are mainly workshops, planning sessions, or rough scenario notes that need normalization first.
- Use Allium skills only when governing behavioral specifications materially constrain the scenario or when scenario gating reveals a real specification gap.

## Outputs

This skill should strengthen or help populate:

- [`scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md)
- [`scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md)
- [`scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md)
- the scenario-routing and readiness-related portions of [`scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md)

## Notes

- This skill is intentionally narrow. It supports Scenario Designer composition and review work after the underlying role packages already exist.
- It is not a role-definition skill, not a project-management method, not a technical-design skill, and not a runtime orchestration engine.

# Orpheum CLI Proposal and Specification

## Purpose

This document proposes a full specification for the **Orpheum CLI**, a utility for applying reusable agentic scenarios, workflows, roles, skills, artifact definitions, and validation checks from the Orpheum repository into a target project in a controlled, minimally invasive way.

This proposal is based on the design direction established in discussion:

- Orpheum is a **catalog** of reusable agentic operating context.
- Codex or another running agent is expected to be the **entry point**.
- Orpheum is expected to be a **CLI utility invoked from within a project**.
- Orpheum should materialize only the minimum project-local orchestration context needed for an active session.
- Orpheum should avoid polluting the user’s workspace.
- Orpheum should support **cleanup** and graceful exit when a scenario is complete.
- A hidden local control directory inside the project is preferred over worktrees as the default behavior.

---

## Executive Summary

Orpheum should be implemented as a **thin scenario-resolution and orchestration CLI** with three major responsibilities:

1. **Resolve** a selected scenario into its dependent roles, workflows, skills, checks, and artifact expectations.
2. **Materialize** a minimal transient control surface inside the target project under `.orpheum/`.
3. **Support lifecycle management** for active sessions, including status inspection, validation, finalization, suspension, resumption, and cleanup.

Orpheum should **not** be the primary project entry point and should **not** be responsible for owning the entire execution loop of an agent. Instead, Orpheum should act as a **local scenario/state service and scaffolding layer** that a running agent may invoke.

---

## Design Goals

### Goals

- Make Orpheum scenarios reusable across multiple agentic projects.
- Keep scenario state close enough to project work to be accessible inside agent sandboxes.
- Minimize pollution of the target workspace.
- Distinguish clearly between **ephemeral orchestration state** and **durable project outputs**.
- Allow a running agent such as Codex to invoke Orpheum programmatically.
- Make sessions resumable when useful, but disposable when finished.
- Keep the CLI tool simple enough to remain maintainable.

### Non-Goals

- Orpheum is not intended to become a full autonomous orchestrator runtime.
- Orpheum is not intended to permanently own project documentation or deliverables.
- Orpheum is not intended to replace project-specific process, project management systems, or repository conventions.
- Orpheum is not intended to require a Git worktree for standard usage.

---

## Core Conceptual Model

Orpheum should be understood as having three layers:

### 1. Catalog Layer
The Orpheum repository remains the canonical source of truth for:

- Roles
- Workflows
- Scenarios
- Checks
- Artifact definitions
- Skills

### 2. Session Layer
When a scenario is applied to a project, Orpheum creates a lightweight, hidden, ephemeral session directory within the target project:

- `.orpheum/`

This directory contains only the active orchestration state needed for the current scenario.

### 3. Project Output Layer
Durable work product belongs in normal project locations, not in `.orpheum/`.

Examples:

- accepted requirements documents
- ADRs
- tickets
- implementation plans
- test strategies
- generated specifications intended for retention

---

## Explicit Decisions

This section calls out decisions made in this proposal. These are the main points to review.

### Decision 1: Orpheum is a CLI utility invoked by a running agent, not the default entry point
**Decision:** Codex or another agent should call Orpheum. Orpheum should not assume responsibility for launching or hosting the agent.

**Reasoning:** This matches the desired operational model and reduces coupling to a specific agent runtime.

### Decision 2: The default local control surface is `.orpheum/` inside the target repository
**Decision:** Orpheum should create a hidden control directory inside the active project rather than mounting or materializing files outside the repo.

**Reasoning:** This avoids permission and sandbox issues and keeps all active state visible to the agent in the same workspace.

### Decision 3: `.orpheum/` is ephemeral orchestration state only
**Decision:** `.orpheum/` should not contain durable project outputs except temporarily before promotion.

**Reasoning:** This preserves a clean boundary between session control state and long-lived project artifacts.

### Decision 4: Git worktrees are optional advanced isolation mode, not default behavior
**Decision:** Orpheum should not require a worktree for standard operation.

**Reasoning:** Worktrees add lifecycle complexity and path indirection that are unnecessary for most scenario applications.

### Decision 5: Orpheum should support explicit session lifecycle commands
**Decision:** The CLI should include commands for apply, status, suspend, resume, finalize, validate/check, and cleanup.

**Reasoning:** Sessions need explicit lifecycle control if Orpheum is to remain lightweight and avoid lingering project pollution.

### Decision 6: Orpheum should prefer writing durable outputs directly to final project locations when appropriate
**Decision:** Durable artifacts should generally be written directly where they belong, or staged briefly and promoted explicitly.

**Reasoning:** This reduces duplication and avoids treating `.orpheum/` as a storage area.

### Decision 7: Orpheum definitions should include machine-readable metadata
**Decision:** Roles, workflows, scenarios, artifacts, checks, and possibly skills should include structured metadata, preferably frontmatter.

**Reasoning:** The CLI needs a deterministic way to resolve dependencies and construct sessions without parsing prose heuristically.

### Decision 8: The CLI should expose both human-readable and machine-readable outputs
**Decision:** Most commands should support normal terminal output and a structured output mode such as `--json`.

**Reasoning:** This makes the CLI usable both by humans and by invoking agents.

---

## High-Level Architecture

The Orpheum CLI should consist of the following logical components:

### 1. Catalog Loader
Responsible for locating and reading the Orpheum catalog.

Responsibilities:
- identify the Orpheum repository path or configured source
- read definitions for roles, workflows, scenarios, checks, artifacts, and skills
- parse structured metadata and associated markdown content

### 2. Dependency Resolver
Responsible for computing the transitive scenario dependency graph.

Responsibilities:
- resolve scenario → workflows
- resolve scenario → roles
- resolve workflows → artifact expectations
- resolve scenario/workflows → checks
- resolve workflows/roles → required skills

### 3. Session Manager
Responsible for project-local session state.

Responsibilities:
- create `.orpheum/`
- create and update session manifest/state
- track current phase and active tasks
- track resumable checkpoints
- maintain cleanup readiness state

### 4. Prompt/Instruction Generator
Responsible for producing the working instructions that an agent will consume.

Responsibilities:
- generate current scenario briefing
- generate next-step prompts
- generate role handoff prompts
- summarize expected outputs and checks

### 5. Validation Runner
Responsible for applying Orpheum-defined checks to local outputs.

Responsibilities:
- execute file/content presence checks
- execute structural or semantic validation hooks when supported
- record pass/fail status
- expose unresolved issues to the invoking agent or user

### 6. Promotion/Cleanup Manager
Responsible for finalization and graceful exit.

Responsibilities:
- promote staged durable artifacts when needed
- remove transient state
- preserve only resumable state when suspending
- remove `.orpheum/` entirely when cleanup is complete

---

## Repository Metadata Requirements

To support deterministic resolution, each definition file should include frontmatter.

### Proposed frontmatter fields by entity type

#### Scenario
```yaml
---
id: project-planning
kind: scenario
title: Project Planning
version: 1
summary: Multi-role scenario for initiating a software project through planning and design.
roles:
  - business-analyst
  - solution-architect
  - technical-planner
  - qa-verification-lead
workflows:
  - business-analyst-kickoff
  - business-analyst-process-analysis
  - business-analyst-requirements-handoff
  - solution-architect-design
  - technical-planner-planning
  - qa-verification-planning
artifacts:
  - business-objectives
  - process-analysis
  - requirements-specification
  - architectural-decision-record
  - implementation-plan
  - verification-strategy
checks:
  - project-planning-readiness
entry_conditions:
  - project-scope-known
  - stakeholder-access-available
exit_conditions:
  - required-artifacts-complete
  - required-checks-passing
---
```

#### Workflow
```yaml
---
id: solution-architect-design
kind: workflow
title: Solution Architect Design
version: 1
role: solution-architect
summary: Produce an architecture proposal and handoff based on validated requirements.
inputs:
  - requirements-specification
outputs:
  - architectural-decision-record
  - solution-handoff
skills:
  - architecture-review
  - tradeoff-analysis
checks:
  - architecture-completeness
handoff_to:
  - technical-planner
---
```

#### Role
```yaml
---
id: solution-architect
kind: role
title: Solution Architect
version: 1
summary: Designs technical solutions and produces technical handoff artifacts.
default_workflows:
  - solution-architect-design
skills:
  - architecture-review
  - tradeoff-analysis
---
```

#### Artifact
```yaml
---
id: implementation-plan
kind: artifact
title: Implementation Plan
version: 1
summary: Structured implementation roadmap suitable for development execution.
template: true
default_output_path: docs/planning/implementation-plan.md
checks:
  - implementation-plan-structure
---
```

#### Check
```yaml
---
id: implementation-plan-structure
kind: check
title: Implementation Plan Structure
version: 1
summary: Confirms the implementation plan contains required sections and handoff criteria.
mode: content
severity: error
applies_to:
  - implementation-plan
---
```

#### Skill
```yaml
---
id: architecture-review
kind: skill
title: Architecture Review
version: 1
summary: Provides architecture-focused reasoning and review instructions.
source: local
---
```

### Decision 9: Frontmatter should be the primary structured metadata format
**Decision:** Use YAML frontmatter embedded in human-readable markdown definitions.

**Reasoning:** This preserves author ergonomics while adding machine readability.

---

## Project-Local Directory Contract

### Proposed `.orpheum/` layout

```text
.orpheum/
  ACTIVE.md
  session.json
  scenario.json
  state.json
  prompts/
    kickoff.md
    next.md
    handoff.md
  logs/
    checks.json
```

### File purposes

#### `ACTIVE.md`
Human-readable session summary for a running agent.

Contents may include:
- scenario name
- session ID
- current phase
- expected outputs
- current blocking issues
- next recommended action
- completion criteria

#### `session.json`
Stable machine-readable metadata for the active session.

Suggested fields:
- session_id
- scenario_id
- scenario_version
- applied_at
- orpheum_source
- source_revision
- target_project_root
- mode
- cleanup_policy

#### `scenario.json`
Resolved dependency snapshot for the active scenario.

Suggested fields:
- roles
- workflows
- artifacts
- checks
- skills
- entry_conditions
- exit_conditions

#### `state.json`
Mutable progress record.

Suggested fields:
- current_phase
- completed_workflows
- pending_workflows
- artifact_status
- check_status
- suspended
- resumable

#### `prompts/`
Short-lived generated instructions for the invoking agent.

#### `logs/checks.json`
Current and prior validation status for the active session.

### Decision 10: `.orpheum/` should have a minimal stable file contract
**Decision:** Keep the local control directory small and predictable.

**Reasoning:** A small contract reduces drift and improves interoperability with invoking agents.

---

## CLI Command Set

## Command Overview

```bash
orpheum scenario list
orpheum scenario show <scenario>
orpheum scenario apply <scenario>
orpheum status
orpheum prompt current
orpheum check run
orpheum finalize
orpheum suspend
orpheum resume
orpheum cleanup
orpheum doctor
```

---

## Command Specifications

### `orpheum scenario list`
List available scenarios from the catalog.

#### Purpose
Allow users or agents to discover reusable scenarios.

#### Behavior
- reads all scenario definitions
- returns id, title, summary, and version
- supports filtering by tags, role participation, or keyword in future

#### Example
```bash
orpheum scenario list
```

#### Machine-readable mode
```bash
orpheum scenario list --json
```

---

### `orpheum scenario show <scenario>`
Display scenario details.

#### Purpose
Provide enough information to evaluate whether a scenario is applicable before applying it.

#### Behavior
- resolves the named scenario
- displays participating roles, workflows, expected artifacts, checks, entry conditions, and exit conditions

#### Example
```bash
orpheum scenario show project-planning
```

---

### `orpheum scenario apply <scenario>`
Apply a scenario to the current or specified project.

#### Purpose
Create a local active session under `.orpheum/`.

#### Behavior
- validates that the target path is a project directory
- validates no conflicting active session exists unless overridden
- resolves scenario dependencies
- creates `.orpheum/`
- writes `ACTIVE.md`, `session.json`, `scenario.json`, `state.json`
- generates initial prompts
- optionally stages templates or writes durable outputs as directed by scenario rules

#### Example
```bash
orpheum scenario apply project-planning
orpheum scenario apply project-planning --project .
orpheum scenario apply project-planning --json
```

#### Suggested options
- `--project <path>`
- `--force`
- `--session-id <id>`
- `--json`
- `--mode default|isolated-worktree`

### Decision 11: only one active default session per project root
**Decision:** By default, only one active session should exist per project root, unless a future multi-session mode is explicitly enabled.

**Reasoning:** This keeps state management simple and reduces agent confusion.

---

### `orpheum status`
Show current active session state.

#### Purpose
Give the user or invoking agent a concise current-state view.

#### Behavior
- reads `.orpheum/session.json` and `.orpheum/state.json`
- shows current phase, remaining workflows, artifact status, check status, and cleanup readiness

#### Example
```bash
orpheum status
orpheum status --json
```

---

### `orpheum prompt current`
Print the current recommended prompt/instruction block.

#### Purpose
Provide the next working instruction for the invoking agent.

#### Behavior
- reads current scenario and state
- generates or retrieves the next recommended instruction set
- may provide different prompt types in future: kickoff, handoff, remediation, validation

#### Example
```bash
orpheum prompt current
orpheum prompt current --json
```

### Decision 12: prompts are derived outputs, not authoritative state
**Decision:** Generated prompts should be treated as disposable renderings of session state.

**Reasoning:** The authoritative state should remain in structured files, not in prompt text.

---

### `orpheum check run`
Run scenario-associated validation checks.

#### Purpose
Confirm that required outputs are present and meet scenario validation expectations.

#### Behavior
- resolves applicable checks for the active session
- evaluates them against relevant project files
- records status in `.orpheum/logs/checks.json`
- updates `state.json`
- prints concise results

#### Example
```bash
orpheum check run
orpheum check run --json
```

#### Future capability
Checks may eventually include:
- file presence checks
- section/heading checks
- structured schema checks
- semantic review hooks
- custom script/plugin checks

### Decision 13: initial check support should be intentionally narrow
**Decision:** Start with structural/content checks before introducing arbitrary executable validation hooks.

**Reasoning:** This reduces security and complexity risk.

---

### `orpheum finalize`
Finalize an active session.

#### Purpose
Conclude the active scenario and preserve only durable project outputs.

#### Behavior
- confirms exit conditions are met or requires override
- promotes any staged outputs if needed
- updates final session status
- removes transient prompts and logs if configured
- prepares for cleanup or performs it automatically depending on policy

#### Example
```bash
orpheum finalize
orpheum finalize --force
```

### Decision 14: finalize should be distinct from cleanup
**Decision:** Finalization and cleanup should be separate commands.

**Reasoning:** Some workflows may require a finalized-but-reviewable state before removing transient control data.

---

### `orpheum cleanup`
Remove transient Orpheum control files.

#### Purpose
Gracefully exit the project and eliminate workspace pollution.

#### Behavior
- validates that no important staged output would be lost
- removes `.orpheum/` entirely when safe
- may preserve minimal tombstone metadata in future if desired, but default is full removal

#### Example
```bash
orpheum cleanup
orpheum cleanup --force
```

### Decision 15: cleanup should default to full removal of `.orpheum/`
**Decision:** When safe, cleanup removes the directory entirely.

**Reasoning:** The accepted direction is that `.orpheum/` should not persist unnecessarily.

---

### `orpheum suspend`
Suspend an active session.

#### Purpose
Preserve resumable state without continuing work.

#### Behavior
- marks state as suspended
- preserves resumable checkpoint data
- may prune disposable prompt files

#### Example
```bash
orpheum suspend
```

---

### `orpheum resume`
Resume a suspended session.

#### Purpose
Return an active scenario to working state.

#### Behavior
- validates suspended session presence
- regenerates `ACTIVE.md` and current prompts if necessary
- updates state to active

#### Example
```bash
orpheum resume
```

---

### `orpheum doctor`
Validate environment and catalog health.

#### Purpose
Confirm that Orpheum is available and configured correctly.

#### Behavior
- verifies catalog location
- checks metadata validity
- checks project compatibility
- checks whether `.orpheum/` is in `.gitignore`
- reports common misconfigurations

#### Example
```bash
orpheum doctor
```

---

## Session Lifecycle Specification

### Phase 1: Apply
A scenario is selected and resolved.

Outputs:
- `.orpheum/` created
- session metadata written
- active scenario summary generated
- initial prompt generated

### Phase 2: Work
A user or agent performs the scenario.

Outputs:
- progress state updated
- checks run as needed
- prompts regenerated as state changes

### Phase 3: Handoff
The current workflow phase or role hands off to another.

Outputs:
- handoff prompt may be generated
- completed/pending workflow statuses updated

### Phase 4: Validate
Checks confirm required artifacts and structure.

Outputs:
- check status updated
- unresolved issues surfaced

### Phase 5: Finalize
The scenario is declared complete.

Outputs:
- staged durable outputs promoted if needed
- state marked finalized

### Phase 6: Cleanup
Transient orchestration state is removed.

Outputs:
- `.orpheum/` deleted

---

## Output Placement Policy

### Default policy
Durable artifacts should be written directly to their final project locations whenever feasible.

Examples:
- `docs/requirements/requirements-spec.md`
- `docs/architecture/adr-001-project-architecture.md`
- `docs/planning/implementation-plan.md`

### Staging policy
Temporary staging under `.orpheum/` may be used only when:
- the scenario intentionally requires review before promotion
- the final destination cannot be determined yet
- a generated file is clearly draft/transient

### Decision 16: durable outputs should default to direct placement
**Decision:** Write retained artifacts directly to intended locations by default.

**Reasoning:** This keeps `.orpheum/` from becoming shadow storage.

---

## Agent Invocation Contract

Orpheum should support being invoked by Codex or another agent with minimal ambiguity.

### Required contract characteristics
- stable exit codes
- machine-readable `--json` output
- predictable file paths
- idempotent reads for status and prompt retrieval

### Suggested machine-readable output example for `scenario apply --json`
```json
{
  "session_id": "sess_20260420_001",
  "scenario_id": "project-planning",
  "project_root": "/workspace/project",
  "control_dir": "/workspace/project/.orpheum",
  "active_file": "/workspace/project/.orpheum/ACTIVE.md",
  "current_phase": "apply",
  "next_command": "orpheum prompt current",
  "cleanup_policy": "explicit"
}
```

### Suggested machine-readable output example for `status --json`
```json
{
  "session_id": "sess_20260420_001",
  "scenario_id": "project-planning",
  "state": "active",
  "current_phase": "solution-architect-design",
  "pending_workflows": ["technical-planner-planning", "qa-verification-planning"],
  "artifact_status": {
    "requirements-specification": "complete",
    "architectural-decision-record": "in_progress",
    "implementation-plan": "pending"
  },
  "check_status": {
    "project-planning-readiness": "pending"
  },
  "cleanup_ready": false
}
```

### Decision 17: machine-readable output should be a first-class feature
**Decision:** JSON output is not optional polish; it is a required integration surface.

**Reasoning:** The tool is explicitly intended to be invoked by a running agent.

---

## Error Model

Orpheum should use stable exit codes and concise structured errors.

### Suggested error classes
- `CATALOG_NOT_FOUND`
- `INVALID_METADATA`
- `SCENARIO_NOT_FOUND`
- `SESSION_ALREADY_ACTIVE`
- `NO_ACTIVE_SESSION`
- `SESSION_NOT_SUSPENDED`
- `CHECK_FAILED`
- `FINALIZE_BLOCKED`
- `CLEANUP_BLOCKED`

### Suggested JSON error shape
```json
{
  "error": {
    "code": "SESSION_ALREADY_ACTIVE",
    "message": "An active Orpheum session already exists for this project.",
    "details": {
      "session_id": "sess_20260420_001"
    }
  }
}
```

---

## Git Behavior

### Default expectations
- `.orpheum/` should be added to `.gitignore`
- Orpheum should warn if it is not ignored
- Orpheum should not create branches or worktrees by default

### Optional future mode
An isolated worktree mode may later be supported for heavy exploratory scenarios.

### Decision 18: no Git topology manipulation in v1 default path
**Decision:** Do not require branch/worktree creation in the primary implementation.

**Reasoning:** The accepted design favors minimal local control state inside the project.

---

## Security and Safety Considerations

- Avoid arbitrary code execution in checks in v1.
- Avoid automatic project mutations beyond explicit scenario application and lifecycle commands.
- Avoid copying large amounts of vendored skill content into the project when references or minimal projections suffice.
- Avoid hidden retention of sensitive prompts or project details after cleanup.

### Decision 19: validation extensibility should be constrained in v1
**Decision:** Defer arbitrary plugin/script execution until the trust and extension model is clearly defined.

**Reasoning:** Agent-invoked tooling should remain predictable and auditable.

---

## Proposed v1 Scope

### Included in v1
- catalog metadata parsing
- scenario listing and inspection
- scenario application
- local `.orpheum/` session creation
- status inspection
- current prompt generation
- basic structural/content checks
- finalize
- cleanup
- suspend/resume
- JSON output mode
- doctor command

### Deferred beyond v1
- multi-session support in one project
- arbitrary executable checks/plugins
- remote catalog registries
- interactive TUI
- Git worktree isolation mode
- agent auto-execution hooks
- networked orchestration services

### Decision 20: keep v1 intentionally narrow
**Decision:** Optimize first for a reliable thin integration layer rather than a feature-rich orchestration platform.

**Reasoning:** The core value is reusable scenario application with clean lifecycle management.

---

## Recommended Implementation Sequence

### Phase A: Metadata normalization
- add frontmatter to all supported Orpheum entities
- validate schema consistency

### Phase B: Resolver library
- load catalog
- resolve scenario dependencies
- build a dependency snapshot

### Phase C: Session manager
- create `.orpheum/`
- create core session/state files
- implement apply/status/suspend/resume/cleanup lifecycle

### Phase D: Prompt generator
- generate `ACTIVE.md`
- generate current prompt blocks

### Phase E: Validation runner
- add basic content/structure checks
- persist check status

### Phase F: Finalization logic
- add exit condition enforcement
- add promotion rules where needed

---

## Resolved Questions

### Resolved Question 1: Durable output staging
**Resolution:** v1 will not stage durable outputs under `.orpheum/outputs/`.

- Durable outputs must be written directly to their intended final project locations.
- `.orpheum/` remains orchestration state only.

### Resolved Question 2: Finalize vs cleanup
**Resolution:** `finalize` and `cleanup` remain strictly separate.

- `finalize` never removes `.orpheum/`.
- `cleanup` must always be called explicitly.

### Resolved Question 3: Preferred output path conventions
**Resolution:** artifact or scenario definitions may include recommended output paths in v1.

- these paths are defaults, not mandates
- the caller or project may override them

### Resolved Question 4: Suspended session compatibility
**Resolution:** suspended sessions are resumable only when the session state format is compatible.

- Orpheum should perform format/version checks
- Orpheum should fail clearly when a suspended session is incompatible with the current CLI
- v1 does not promise broad migration across incompatible session format changes

### Resolved Question 5: Skill materialization
**Resolution:** skills are referenced from the Orpheum catalog by default and materialized only when explicitly needed.

- full skill content is not copied into `.orpheum/` by default
- Orpheum may materialize reduced or derived instructions only when required for the active session

### Resolved Question 6: Scenario-first vs role-first v1 scope
**Resolution:** v1 remains scenario-first.

- `orpheum role apply <role>` is deferred beyond v1
- v1 centers on scenario application, lifecycle management, and validation

## Final Recommendation

Orpheum should be built as a **thin, project-local, lifecycle-managed CLI** that lets a running agent apply reusable scenarios into a target repository without permanently polluting it.

The core design center should be:

- **catalog in Orpheum**
- **ephemeral orchestration state in `.orpheum/`**
- **durable outputs in the project proper**
- **explicit finalize and cleanup behavior**
- **agent-friendly machine-readable command outputs**

That model best aligns with the accepted direction and provides a strong foundation for future growth without overcommitting to premature orchestration complexity.


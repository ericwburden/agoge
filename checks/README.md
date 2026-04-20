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

## QA / Verification Lead Check Set

The next concrete check set in this directory supports the [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role and its artifact library.

- [`verification-strategy.check.md`](D:/Projects/agoge/checks/verification-strategy.check.md) validates the verification strategy artifact.
- [`verification-matrix.check.md`](D:/Projects/agoge/checks/verification-matrix.check.md) validates the verification matrix artifact.
- [`evidence-review.check.md`](D:/Projects/agoge/checks/evidence-review.check.md) validates the durable evidence review artifact.
- [`verification-handoff.check.md`](D:/Projects/agoge/checks/verification-handoff.check.md) validates the verification handoff artifact.
- [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md) validates linkage from upstream requirements, architecture, planning, and implementation artifacts through the verification chain.
- [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md) validates that verification outputs stay within role boundaries.

These checks are definition-of-done quality gates for the QA / Verification Lead lifecycle: define the verification strategy, map coverage and evidence, review the verification package explicitly, package the verification handoff, and remediate failures at the earliest stage that introduced the defect.

## Implementation Engineer Check Set

The next concrete check set in this directory supports the [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role and its artifact library.

- [`implementation-record.check.md`](D:/Projects/orpheum/checks/implementation-record.check.md) validates the implementation record artifact.
- [`implementation-evidence.check.md`](D:/Projects/orpheum/checks/implementation-evidence.check.md) validates the implementation evidence artifact.
- [`implementation-readiness-review.check.md`](D:/Projects/orpheum/checks/implementation-readiness-review.check.md) validates the durable implementation readiness review artifact.
- [`implementation-package-handoff.check.md`](D:/Projects/orpheum/checks/implementation-package-handoff.check.md) validates the implementation package handoff artifact.
- [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md) validates linkage from upstream requirements, architecture, planning, and specifications through the implementation chain.
- [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md) validates that implementation outputs stay within role boundaries.

These checks are definition-of-done quality gates for the Implementation Engineer lifecycle: capture the implementation record, capture implementation evidence, review the implementation package explicitly, package the downstream handoff, and remediate failures at the earliest stage that introduced the defect.

## Code Reviewer Check Set

The next concrete check set in this directory supports the [`Code Reviewer`](D:/Projects/orpheum/roles/code-reviewer.md) role and its artifact library.

- [`code-review-scope.check.md`](D:/Projects/orpheum/checks/code-review-scope.check.md) validates the code review scope artifact.
- [`review-findings.check.md`](D:/Projects/orpheum/checks/review-findings.check.md) validates the durable review findings artifact.
- [`review-decision.check.md`](D:/Projects/orpheum/checks/review-decision.check.md) validates the review decision artifact.
- [`review-handoff.check.md`](D:/Projects/orpheum/checks/review-handoff.check.md) validates the review handoff artifact.
- [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md) validates linkage from the implementation package and upstream commitments through the review chain.
- [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md) validates that review outputs stay within independent review role boundaries.

These checks are definition-of-done quality gates for the Code Reviewer lifecycle: frame review scope, record findings explicitly, make the review decision explicit, package the review handoff, and remediate failures at the earliest stage that introduced the defect.

## Release / Handoff Manager Check Set

The next concrete check set in this directory supports the [`Release / Handoff Manager`](D:/Projects/orpheum/roles/release-handoff-manager.md) role and its artifact library.

- [`release-candidate-summary.check.md`](D:/Projects/orpheum/checks/release-candidate-summary.check.md) validates the release candidate summary artifact.
- [`release-readiness-decision.check.md`](D:/Projects/orpheum/checks/release-readiness-decision.check.md) validates the release readiness decision artifact.
- [`rollout-and-operations-notes.check.md`](D:/Projects/orpheum/checks/rollout-and-operations-notes.check.md) validates the rollout and operations notes artifact.
- [`release-handoff.check.md`](D:/Projects/orpheum/checks/release-handoff.check.md) validates the release handoff artifact.
- [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md) validates linkage from implementation, review, and verification packages through the release chain.
- [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md) validates that release outputs stay within release packaging role boundaries.

These checks are definition-of-done quality gates for the Release / Handoff Manager lifecycle: frame the release candidate, make the release posture explicit, preserve rollout caveats, package the release handoff, and remediate failures at the earliest stage that introduced the defect.

## Product Owner Check Set

The next concrete check set in this directory supports the [`Product Owner`](D:/Projects/orpheum/roles/product-owner.md) role and its artifact library.

- [`product-direction.check.md`](D:/Projects/orpheum/checks/product-direction.check.md) validates the product direction artifact.
- [`backlog-prioritization.check.md`](D:/Projects/orpheum/checks/backlog-prioritization.check.md) validates the backlog prioritization artifact.
- [`product-decision-review.check.md`](D:/Projects/orpheum/checks/product-decision-review.check.md) validates the product decision review artifact.
- [`product-handoff.check.md`](D:/Projects/orpheum/checks/product-handoff.check.md) validates the product handoff artifact.
- [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md) validates linkage from validated discovery and supporting evidence through the product chain.
- [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md) validates that product outputs stay within product-ownership role boundaries.

These checks are definition-of-done quality gates for the Product Owner lifecycle: define product direction, make priorities explicit, review the product posture, package the product handoff, and remediate failures at the earliest stage that introduced the defect.

## Security / Compliance Specialist Check Set

The next concrete check set in this directory supports the [`Security / Compliance Specialist`](D:/Projects/orpheum/roles/security-compliance-specialist.md) role and its artifact library.

- [`security-compliance-scope.check.md`](D:/Projects/orpheum/checks/security-compliance-scope.check.md) validates the security/compliance scope artifact.
- [`controls-and-evidence-matrix.check.md`](D:/Projects/orpheum/checks/controls-and-evidence-matrix.check.md) validates the controls/evidence matrix artifact.
- [`security-compliance-review.check.md`](D:/Projects/orpheum/checks/security-compliance-review.check.md) validates the security/compliance review artifact.
- [`security-compliance-handoff.check.md`](D:/Projects/orpheum/checks/security-compliance-handoff.check.md) validates the security/compliance handoff artifact.
- [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md) validates linkage from reviewed delivery and obligation inputs through the security/compliance chain.
- [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md) validates that security/compliance outputs stay within role boundaries.

These checks are definition-of-done quality gates for the Security / Compliance Specialist lifecycle: frame the security/compliance scope, map controls and evidence, review the posture explicitly, package the downstream handoff, and remediate failures at the earliest stage that introduced the defect.

## Scenario Designer Check Set

The next concrete check set in this directory supports the [`Scenario Designer`](D:/Projects/orpheum/roles/scenario-designer.md) role and its artifact library.

- [`scenario-definition.check.md`](D:/Projects/orpheum/checks/scenario-definition.check.md) validates the scenario definition artifact.
- [`scenario-integration-map.check.md`](D:/Projects/orpheum/checks/scenario-integration-map.check.md) validates the scenario integration map artifact.
- [`scenario-review.check.md`](D:/Projects/orpheum/checks/scenario-review.check.md) validates the durable scenario review artifact.
- [`scenario-handoff.check.md`](D:/Projects/orpheum/checks/scenario-handoff.check.md) validates the scenario handoff artifact.
- [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md) validates linkage from role-owned workflows, artifact contracts, and gating assumptions through the scenario chain.
- [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md) validates that scenario outputs stay within Scenario Designer role boundaries.

These checks are definition-of-done quality gates for the Scenario Designer lifecycle: define the scenario boundary, map the integration logic, review the scenario posture explicitly, package the scenario handoff, and remediate failures at the earliest stage that introduced the defect.

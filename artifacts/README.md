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

The first concrete artifact set in this directory is aligned to the [`Business Analyst`](D:/Projects/agoge/roles/business-analyst.md) role.

- [`business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md) defines the reusable structure for capturing the business problem, goals, stakeholders, context, and scope boundaries.
- [`process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md) defines the reusable structure for capturing as-is and to-be process understanding, business rules, actors, triggers, and gaps.
- [`requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md) defines the reusable structure for capturing verified business requirements with rationale, scope, and open questions.
- [`requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) defines the reusable structure for capturing downstream-ready traceability, risks, unresolved questions, and handoff guidance.

Use this set when a project needs structured kickoff and discovery outputs without inventing a custom artifact library from scratch. Instantiate the needed artifact files into the project workspace before filling them out.

Each Business Analyst artifact links to its primary definition-of-done check and the two cross-cutting BA checks for traceability and role-boundary discipline. The intended lifecycle is: select template, instantiate project copy, populate it, run checks, remediate failures, then hand it downstream.

## Role Builder Artifacts

The next concrete artifact set in this directory is aligned to the [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) role.

- [`role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md) defines the reusable structure for capturing a role's job-to-be-done, lifecycle position, boundaries, success criteria, and default outputs.
- [`role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md) defines the reusable structure for capturing a role's artifact set, workflow set, check set, skill support, and tooling preferences.
- [`role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md) defines the reusable structure for reviewing the coherence and readiness of a role package.
- [`role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md) defines the reusable structure for packaging a reviewed role for downstream adoption, including guidance, limitations, consumers, and next steps.

Use this set when defining or tightening a reusable role package rather than producing project-specific delivery artifacts. Instantiate the needed artifact files into the working location before filling them out.

The intended Role Builder lifecycle is: select template, instantiate working copy, populate the role definition, derive the support system, review the package, package it for adoption, and remediate any failed checks before treating the role as reusable downstream.

## Solution Architect Artifacts

The next concrete artifact set in this directory is aligned to the [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role.

- [`solution-architecture.md`](D:/Projects/agoge/artifacts/solution-architecture.md) defines the reusable structure for capturing architectural drivers, system boundaries, major components, flows, interfaces, integrations, constraints, and risks.
- [`architecture-decisions.md`](D:/Projects/agoge/artifacts/architecture-decisions.md) defines the reusable structure for capturing major architectural choices, alternatives, rationale, and consequences.
- [`architecture-review.md`](D:/Projects/agoge/artifacts/architecture-review.md) defines the reusable structure for capturing durable architecture review findings, readiness, remediation, and unresolved concerns before handoff.
- [`architecture-handoff.md`](D:/Projects/agoge/artifacts/architecture-handoff.md) defines the reusable structure for packaging reviewed architecture for downstream planning, implementation, and verification roles.

Use this set when validated BA outputs need to be turned into a technical solution shape and downstream architectural handoff. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Solution Architect lifecycle is: select template, instantiate working copy, define the solution architecture, record major decisions, review the architecture explicitly, package the architecture handoff, run checks, remediate failures, and only then move the architecture downstream.

## Technical Planner Artifacts

The next concrete artifact set in this directory is aligned to the [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role.

- [`implementation-strategy.md`](D:/Projects/agoge/artifacts/implementation-strategy.md) defines the reusable structure for capturing the implementation approach, slice strategy, workstream framing, enabling work, readiness conditions, and planning risks.
- [`sequencing-and-dependencies.md`](D:/Projects/agoge/artifacts/sequencing-and-dependencies.md) defines the reusable structure for capturing execution order, dependency structure, critical path assumptions, and parallelization opportunities.
- [`implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md) defines the reusable structure for capturing durable planning review findings, readiness, remediation, and unresolved concerns before handoff.
- [`implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md) defines the reusable structure for packaging reviewed planning outputs for downstream implementation and verification roles.

Use this set when reviewed architecture and validated requirements need to be turned into a downstream-ready execution plan. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Technical Planner lifecycle is: select template, instantiate working copy, define the implementation strategy, make sequencing and dependencies explicit, review the planning package, package the implementation handoff, run checks, remediate failures, and only then move the plan downstream.

## QA / Verification Lead Artifacts

The next concrete artifact set in this directory is aligned to the [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role.

- [`verification-strategy.md`](D:/Projects/agoge/artifacts/verification-strategy.md) defines the reusable structure for capturing verification scope, confidence goals, evidence expectations, risk focus, and readiness framing.
- [`verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md) defines the reusable structure for mapping requirements, architecture, planning hotspots, and implementation slices to expected or observed verification evidence.
- [`evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md) defines the reusable structure for capturing durable evidence review findings, readiness, remediation, and unresolved concerns before handoff.
- [`verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md) defines the reusable structure for packaging reviewed verification outputs for downstream implementation, review, or release-adjacent roles.

Use this set when reviewed delivery artifacts and implementation evidence need to be turned into a downstream-ready verification package. Instantiate the needed artifact files into the project workspace before filling them out.

The intended QA / Verification Lead lifecycle is: select template, instantiate working copy, define the verification strategy, map verification coverage, review the evidence package explicitly, package the verification handoff, run checks, remediate failures, and only then move the verification package downstream.

## Implementation Engineer Artifacts

The next concrete artifact set in this directory is aligned to the [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role.

- [`implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md) defines the reusable structure for capturing implementation scope, upstream traceability, changed areas, deviations, blockers, and deferred work.
- [`implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md) defines the reusable structure for capturing local validation activities, observed results, provenance, skipped checks, and evidence gaps.
- [`implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md) defines the reusable structure for capturing durable implementation-package findings, readiness, remediation, and upstream routing before handoff.
- [`implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md) defines the reusable structure for packaging reviewed implementation outputs for downstream code review, verification, and release-adjacent roles.

Use this set when reviewed planning and architecture direction need to be turned into a downstream-ready implementation package rather than only a raw code diff. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Implementation Engineer lifecycle is: select template, instantiate working copy, record the implemented slice and evidence, review implementation readiness explicitly, package the implementation handoff, run checks, remediate failures, and only then move the implementation package downstream.

## Code Reviewer Artifacts

The next concrete artifact set in this directory is aligned to the [`Code Reviewer`](D:/Projects/orpheum/roles/code-reviewer.md) role.

- [`code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md) defines the reusable structure for capturing the reviewed change boundary, reviewed inputs, upstream conformance anchors, hotspots, and review limits.
- [`review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md) defines the reusable structure for capturing durable review findings, severity, confidence, affected areas, and requested remediation or evidence.
- [`review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md) defines the reusable structure for capturing the review posture, blocking versus non-blocking concerns, required remediation, and unresolved risks before handoff.
- [`review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md) defines the reusable structure for packaging reviewed code-review outputs for downstream implementation, verification, release-adjacent, or human approval consumers.

Use this set when a completed implementation package needs to be turned into a downstream-ready independent review package rather than only a collection of comments or an approval label. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Code Reviewer lifecycle is: select template, instantiate working copy, define review scope, record findings explicitly, decide the review posture, package the review handoff, run checks, remediate failures, and only then move the review package downstream.

## Release / Handoff Manager Artifacts

The next concrete artifact set in this directory is aligned to the [`Release / Handoff Manager`](D:/Projects/orpheum/roles/release-handoff-manager.md) role.

- [`release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md) defines the reusable structure for capturing the target release scope, reviewed inputs, included and excluded scope, release-sensitive hotspots, and candidate limits.
- [`release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md) defines the reusable structure for capturing the release posture, approval limits, required conditions, unresolved risks, and decision owners before handoff.
- [`rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md) defines the reusable structure for capturing environment assumptions, protection rules, sequencing, monitoring, rollback triggers, communication notes, and operational caveats.
- [`release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md) defines the reusable structure for packaging reviewed release outputs for downstream release-adjacent, operational, or adoption consumers.

Use this set when reviewed implementation, review, and verification outputs need to be turned into a downstream-ready release or adoption package rather than only a summary or approval label. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Release / Handoff Manager lifecycle is: select template, instantiate working copy, define the release candidate, make rollout notes explicit, review the release posture, package the release handoff, run checks, remediate failures, and only then move the release package downstream.

## Product Owner Artifacts

The next concrete artifact set in this directory is aligned to the [`Product Owner`](D:/Projects/orpheum/roles/product-owner.md) role.

- [`product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md) defines the reusable structure for capturing current product goal, target users or stakeholders, value hypotheses, constraints, scope boundaries, and priority themes.
- [`backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md) defines the reusable structure for capturing ordered work, ordering rationale, acceptance-oriented conditions, deferred scope, sequencing notes, and reprioritization triggers.
- [`product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md) defines the reusable structure for capturing the product decision posture, risks, tradeoffs, conditions, and decision ownership before handoff.
- [`product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md) defines the reusable structure for packaging reviewed product outputs for downstream solutioning, planning, delivery, release-feedback, or approval consumers.

Use this set when validated requirements, feedback, and delivery learnings need to be turned into a downstream-ready product direction package rather than only backlog edits or meeting notes. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Product Owner lifecycle is: select template, instantiate working copy, define product direction, make backlog priorities explicit, review the current product posture, package the product handoff, run checks, remediate failures, and only then move the product package downstream.

## Security / Compliance Specialist Artifacts

The next concrete artifact set in this directory is aligned to the [`Security / Compliance Specialist`](D:/Projects/orpheum/roles/security-compliance-specialist.md) role.

- [`security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md) defines the reusable structure for capturing in-scope systems, assets, data types, obligations, trust boundaries, threat surfaces, assumptions, and open questions.
- [`controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md) defines the reusable structure for capturing required controls, expected evidence, control owners, compensating controls, waivers, and unresolved gaps.
- [`security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md) defines the reusable structure for capturing the current security/compliance posture, decision limits, residual risks, and required follow-up before handoff.
- [`security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md) defines the reusable structure for packaging reviewed security/compliance outputs for downstream architecture, planning, implementation, verification, release, or approval consumers.

Use this set when reviewed delivery context and governing obligations need to be turned into a downstream-ready security/compliance package rather than only ad hoc risk notes or checklist comments. Instantiate the needed artifact files into the project workspace before filling them out.

The intended Security / Compliance Specialist lifecycle is: select template, instantiate working copy, define security/compliance scope, make controls and evidence expectations explicit, review the current posture, package the downstream handoff, run checks, remediate failures, and only then move the package downstream.

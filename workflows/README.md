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

`Role Builder hardening pass` is the explicit short trigger for invoking [`role-builder-quality-review.md`](D:/Projects/agoge/workflows/role-builder-quality-review.md).

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

## QA / Verification Lead Workflow Set

The next concrete workflow set in this directory is aligned to the [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role and the QA / Verification Lead artifact library in [`artifacts/`](D:/Projects/agoge/artifacts).

- [`qa-verification-planning.md`](D:/Projects/agoge/workflows/qa-verification-planning.md) turns reviewed requirements, architecture, planning, and implementation context into a verification strategy and coverage matrix.
- [`qa-verification-review.md`](D:/Projects/agoge/workflows/qa-verification-review.md) reviews drafted verification outputs together with the available evidence, records findings, and decides whether the package is ready for downstream use.
- [`qa-verification-handoff.md`](D:/Projects/agoge/workflows/qa-verification-handoff.md) packages completed verification outputs for downstream implementation, review, or release-adjacent roles.
- [`qa-verification-quality-review.md`](D:/Projects/agoge/workflows/qa-verification-quality-review.md) runs the QA / Verification Lead check chain and routes remediation before downstream use.

The intended QA / Verification Lead lifecycle in this repo is: reviewed requirements, architecture, and implementation handoffs -> verification strategy and coverage mapping -> evidence review -> verification handoff -> final quality review and remediation -> downstream release-adjacent or review work.

## QA / Verification Lead Skill Review

### Direct Support

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md): preferred local-Markdown path for grounding verification scope, acceptance expectations, and coverage in upstream evidence.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed verification outputs into a downstream-ready verification handoff.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when requirements, architecture, planning, implementation, or evidence context needs to be combined before verification can proceed cleanly.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md): normalize verification workshops, walkthrough notes, test-session notes, or evidence-review transcripts.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md): optional external research and citation support when verification direction depends on external standards, compliance expectations, or platform guidance.

### Allium-Aware Support

- [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md): useful when mature behavioral specifications should shape verification expectations or coverage direction.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md): useful when verification confidence depends on checking whether implementation behavior has drifted from a governing specification.

### Remaining Design Choice

- No dedicated evidence-review or coverage-mapping skill exists yet. For now, coverage and review discipline is embedded directly in `requirements-verification`, the verification artifacts, and the QA / Verification Lead checks.

QA / Verification Lead work should treat existing Allium specifications as behavioral verification anchors, but verification artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Implementation Engineer Workflow Set

The next concrete workflow set in this directory is aligned to the [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role and the Implementation Engineer artifact library in [`artifacts/`](D:/Projects/orpheum/artifacts).

- [`implementation-engineer-execution.md`](D:/Projects/orpheum/workflows/implementation-engineer-execution.md) turns reviewed planning and architecture handoff into an implementation record and implementation evidence package.
- [`implementation-engineer-review.md`](D:/Projects/orpheum/workflows/implementation-engineer-review.md) reviews drafted implementation outputs, records findings, and decides whether the package is ready for downstream use.
- [`implementation-engineer-handoff.md`](D:/Projects/orpheum/workflows/implementation-engineer-handoff.md) packages completed implementation outputs for downstream review, verification, and release-adjacent roles.
- [`implementation-engineer-quality-review.md`](D:/Projects/orpheum/workflows/implementation-engineer-quality-review.md) runs the Implementation Engineer check chain and routes remediation before downstream use.

The intended Implementation Engineer lifecycle in this repo is: reviewed implementation handoff -> implementation record and evidence -> implementation readiness review -> implementation package handoff -> final quality review and remediation -> downstream review and verification work.

## Implementation Engineer Skill Review

### Direct Support

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md): preferred direct-support skill for turning implemented work into an explicit implementation record, evidence package, implementation-package self-review, and downstream handoff.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when planning, architecture, requirement, code, and evidence context need to be combined before implementation packaging can proceed cleanly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed implementation outputs into a downstream-ready handoff.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md): normalize implementation working-session notes, defect notes, or debug-session transcripts.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md): optional external research and citation support when implementation depends on external standards, platform guidance, or integration references.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md): conditional implementation-evidence support when the slice includes browser-based or web-application behavior.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md): useful when existing behavioral specifications materially constrain the implementation.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md): useful when implementation confidence depends on checking for specification-to-code drift.

### Downstream Or Adjacent Only

- [`spec-to-implementation`](D:/Projects/orpheum/skills/spec-to-implementation/SKILL.md): upstream planning support rather than a default Implementation Engineer dependency.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md): downstream verification support rather than a core implementation skill.
- [`gh-address-comments`](D:/Projects/orpheum/skills/gh-address-comments/SKILL.md) and [`gh-fix-ci`](D:/Projects/orpheum/skills/gh-fix-ci/SKILL.md): useful in repository-specific GitHub workflows, but not core to the repo-neutral role package.

### Remaining Design Choice

- A dedicated repo-native packaging skill now exists for this role, but the package still intentionally does not define a generic implementation-execution coding skill.
- The package intentionally treats implementation-package readiness review as an internal self-review step, not as a replacement for future Code Reviewer or QA / Verification Lead work.
- [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) is the explicit mechanism for deciding when that "future skill" should stop being deferred and what narrow shape it should take.
- The standard trigger point for that decision is repeated failure or remediation patterns observed during [`implementation-engineer-quality-review.md`](D:/Projects/orpheum/workflows/implementation-engineer-quality-review.md).

Implementation Engineer work should treat existing Allium specifications as behavioral implementation constraints, but implementation artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Code Reviewer Workflow Set

The next concrete workflow set in this directory is aligned to the [`Code Reviewer`](D:/Projects/orpheum/roles/code-reviewer.md) role and the Code Reviewer artifact library in [`artifacts/`](D:/Projects/orpheum/artifacts).

- [`code-reviewer-analysis.md`](D:/Projects/orpheum/workflows/code-reviewer-analysis.md) turns a completed implementation package into explicit review scope and durable review findings.
- [`code-reviewer-decision.md`](D:/Projects/orpheum/workflows/code-reviewer-decision.md) reviews the drafted findings package, records the review posture, and decides whether the change is approved, conditional, or blocked.
- [`code-reviewer-handoff.md`](D:/Projects/orpheum/workflows/code-reviewer-handoff.md) packages reviewed code-review outputs for downstream implementation, verification, release-adjacent, or human approval consumers.
- [`code-reviewer-quality-review.md`](D:/Projects/orpheum/workflows/code-reviewer-quality-review.md) runs the Code Reviewer check chain and routes remediation before downstream use.

The intended Code Reviewer lifecycle in this repo is: reviewed implementation package -> code review scope and findings -> review decision -> review handoff -> final quality review and remediation -> downstream implementation, verification, or approval work.

## Code Reviewer Skill Review

### Direct Support

- [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md): preferred direct-support skill for turning implementation context into explicit Code Reviewer findings with severity, confidence, location, evidence-basis, and re-review discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when implementation context, upstream commitments, code signals, and evidence notes need to be combined before review artifacts can be written honestly.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md): preferred local-Markdown grounding path when review findings hinge on requirement conformance, acceptance commitments, or unsupported behavior claims.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed code-review outputs into a downstream-ready handoff.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md): normalize review notes, implementation walkthrough notes, or defect-discussion transcripts.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md): optional external research and citation support when review depends on external standards, platform guidance, or integration references.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md): conditional support when a suspected finding depends on targeted browser or application reproduction.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md): useful when existing behavioral specifications materially constrain review judgment.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md): useful when review confidence depends on checking for specification-to-code drift.

### Downstream Or Adjacent Only

- [`gh-address-comments`](D:/Projects/orpheum/skills/gh-address-comments/SKILL.md): useful after review when implementation owners need to address submitted feedback, but not a core Code Reviewer authoring skill.
- [`gh-fix-ci`](D:/Projects/orpheum/skills/gh-fix-ci/SKILL.md): useful when failing CI materially affects review posture, but still adjacent rather than core.

### Remaining Design Choice

- A dedicated repo-native findings-authoring skill now exists for this role, but the package still intentionally does not define a generic repository-specific review-rubric or approval skill.
- The package intentionally keeps code review separate from implementation ownership and from broader QA or verification authority.
- If repeated usage reveals consistent friction around repository-specific review rubrics or broader approval conventions, that should trigger a later hardening pass rather than being hidden as informal reviewer intuition.

Code Reviewer work should treat existing Allium specifications as behavioral review anchors, but review artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Release / Handoff Manager Workflow Set

The next concrete workflow set in this directory is aligned to the [`Release / Handoff Manager`](D:/Projects/orpheum/roles/release-handoff-manager.md) role and the Release / Handoff Manager artifact library in [`artifacts/`](D:/Projects/orpheum/artifacts).

- [`release-handoff-manager-packaging.md`](D:/Projects/orpheum/workflows/release-handoff-manager-packaging.md) turns reviewed implementation, review, and verification packages into an explicit release candidate summary and rollout notes package.
- [`release-handoff-manager-readiness-review.md`](D:/Projects/orpheum/workflows/release-handoff-manager-readiness-review.md) reviews the drafted release package, records the release posture, and decides whether the candidate is ready, conditional, or blocked.
- [`release-handoff-manager-handoff.md`](D:/Projects/orpheum/workflows/release-handoff-manager-handoff.md) packages reviewed release outputs for downstream release-adjacent, operational, or adoption consumers.
- [`release-handoff-manager-quality-review.md`](D:/Projects/orpheum/workflows/release-handoff-manager-quality-review.md) runs the Release / Handoff Manager check chain and routes remediation before downstream use.

The intended Release / Handoff Manager lifecycle in this repo is: reviewed implementation, review, and verification packages -> release candidate summary and rollout notes -> release readiness decision -> release handoff -> final quality review and remediation -> downstream release-adjacent or adoption work.

## Release / Handoff Manager Skill Review

### Direct Support

- [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md): preferred direct-support skill for turning reviewed implementation, review, and verification packages into explicit release candidate framing, release posture, rollout caveats, and re-approval discipline.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for turning reviewed delivery artifacts into a downstream-ready release or adoption package.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when implementation, review, verification, operational, and approval context need to be combined before release packaging can proceed cleanly.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md): preferred local-Markdown grounding path when release framing depends heavily on validated behavior or acceptance commitments.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md): normalize release coordination notes, operational review notes, approval notes, or adoption notes.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md): optional external research and citation support when release packaging depends on external platform guidance, compliance notes, or downstream communication expectations.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md): conditional support when release posture depends on targeted validation or demonstration evidence beyond the current package summary.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md): useful when existing behavioral specifications materially constrain rollout or downstream release communication.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md): useful when release confidence depends on checking for specification-to-code drift.

### Downstream Or Adjacent Only

- deployment tools and environment-specific operational systems remain downstream or adjacent concerns rather than core repo-neutral role-package support.
- [`gh-fix-ci`](D:/Projects/orpheum/skills/gh-fix-ci/SKILL.md): useful when failing CI materially affects release posture, but still adjacent rather than core.

### Remaining Design Choice

- A dedicated repo-native release-packaging skill now exists for this role, but the package still intentionally does not define a generic repository-specific release-rubric or deployment-authorization skill.
- The package intentionally keeps release packaging separate from deployment execution, implementation ownership, and broader QA authority.
- If repeated usage reveals consistent friction around repository-specific release rubrics, cross-project approval conventions, or deployment-authorization phrasing, that should trigger a later hardening pass rather than being hidden as informal operator intuition.

Release / Handoff Manager work should treat existing Allium specifications as behavioral release anchors, but release artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Product Owner Workflow Set

The next concrete workflow set in this directory is aligned to the [`Product Owner`](D:/Projects/orpheum/roles/product-owner.md) role and the Product Owner artifact library in [`artifacts/`](D:/Projects/orpheum/artifacts).

- [`product-owner-direction.md`](D:/Projects/orpheum/workflows/product-owner-direction.md) turns validated requirements, product feedback, and delivery learnings into explicit product direction and backlog prioritization artifacts.
- [`product-owner-review.md`](D:/Projects/orpheum/workflows/product-owner-review.md) reviews the drafted product package, records the product posture, and decides whether it is ready, conditional, or blocked.
- [`product-owner-handoff.md`](D:/Projects/orpheum/workflows/product-owner-handoff.md) packages reviewed product outputs for downstream solutioning, planning, delivery, release-feedback, or approval consumers.
- [`product-owner-quality-review.md`](D:/Projects/orpheum/workflows/product-owner-quality-review.md) runs the Product Owner check chain and routes remediation before downstream use.

The intended Product Owner lifecycle in this repo is: validated discovery and feedback context -> product direction and backlog prioritization -> product decision review -> product handoff -> final quality review and remediation -> downstream solutioning, planning, delivery, or approval work.

## Product Owner Skill Review

### Direct Support

- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md): preferred direct-support skill for turning validated requirements, feedback, and delivery learnings into explicit product direction, tradeoff framing, acceptance guardrails, deferred-scope discipline, and reprioritization triggers.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md): preferred local-Markdown grounding path for turning validated needs, commitments, and acceptance-sensitive constraints into explicit product direction and prioritization.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path when discovery, feedback, release learnings, and decision notes need to be combined before product work can proceed cleanly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed product outputs into a downstream-ready handoff.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md): normalize prioritization workshops, stakeholder sessions, release-learnings reviews, or product decision meetings.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md): optional external research and citation support when product direction depends on market, platform, or standards context.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md): useful when existing behavioral specifications materially constrain product direction.
- [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md): useful when mature product intent should be sharpened into clearer behavioral commitments.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md): useful when prioritization or product review reveals a real specification gap that should be refined upstream.

### Remaining Design Choice

- A dedicated repo-native product-priority skill now exists for this role, but the package still intentionally does not define a broader repository-specific roadmap-governance or delivery-commitment skill.
- The package intentionally keeps product ownership separate from broad discovery execution, technical design, sprint administration, and release operations.
- If repeated usage reveals consistent friction around broader repository-specific roadmap governance, cross-team delivery-commitment conventions, or release-feedback intake into prioritization, that should trigger a later hardening pass rather than being hidden as informal product judgment.

Product Owner work should treat existing Allium specifications as behavioral product anchors, but product artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

## Security / Compliance Specialist Workflow Set

The next concrete workflow set in this directory is aligned to the [`Security / Compliance Specialist`](D:/Projects/orpheum/roles/security-compliance-specialist.md) role and the Security / Compliance Specialist artifact library in [`artifacts/`](D:/Projects/orpheum/artifacts).

- [`security-compliance-specialist-scoping.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-scoping.md) turns reviewed delivery context and governing obligations into explicit security/compliance scope and controls/evidence artifacts.
- [`security-compliance-specialist-review.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-review.md) reviews the drafted security/compliance package, records the posture, and decides whether it is ready, conditional, or blocked.
- [`security-compliance-specialist-handoff.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-handoff.md) packages reviewed security/compliance outputs for downstream architecture, planning, implementation, verification, release, or approval consumers.
- [`security-compliance-specialist-quality-review.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-quality-review.md) runs the Security / Compliance Specialist check chain and routes remediation before downstream use.

The intended Security / Compliance Specialist lifecycle in this repo is: reviewed delivery and obligation context -> security/compliance scope and controls/evidence mapping -> security/compliance review -> security/compliance handoff -> final quality review and remediation -> downstream architecture, planning, implementation, verification, release, or approval work.

## Security / Compliance Specialist Skill Review

### Direct Support

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md): preferred direct-support skill for turning reviewed delivery and obligation context into explicit controls, evidence expectations, compensating controls, exception posture, approval limits, and re-review discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md): preferred local-Markdown synthesis path for turning reviewed delivery, policy, vendor, obligation, and operational context into explicit security/compliance framing.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md): preferred local-Markdown grounding path when risk or obligation posture depends heavily on validated needs, commitments, or acceptance-sensitive constraints.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md): preferred local-Markdown path for packaging reviewed security/compliance outputs into a downstream-ready handoff.

### Use As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md): normalize security review sessions, audit notes, stakeholder meetings, or obligation workshops.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md): optional external research and citation support when obligation framing depends on platform, standards, or regulatory context.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md): useful when existing behavioral specifications materially constrain security/compliance posture.
- [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md): useful when mature policy-sensitive intent should be sharpened into clearer behavioral commitments.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md): useful when review reveals a real policy or approval-behavior gap that should be refined upstream.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md): useful when confidence depends on checking for specification-to-implementation drift in a security-sensitive area.

### Remaining Design Choice

- A dedicated repo-native security/compliance framing skill now exists for this role, but the package still intentionally does not define a broader legal-interpretation, control-implementation, or deployment-authorization skill.
- The package intentionally keeps security/compliance framing separate from legal sign-off, technical implementation, QA authority, and release execution.
- If repeated usage still reveals consistent friction around stronger exception specialization or stronger architecture/release bridges, that should trigger a later hardening pass rather than being hidden as informal specialist judgment.

Security / Compliance Specialist work should treat existing Allium specifications as behavioral security/compliance anchors, but the artifacts themselves remain Markdown-first in this repository unless a later repo convention says otherwise.

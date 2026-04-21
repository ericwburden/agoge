# Secure Delivery / Secure Feature Lifecycle Scenario Integration Map

## Purpose

Capture how the `Secure Delivery / Secure Feature Lifecycle` scenario composes role-owned workflows into one coherent downstream delivery lifecycle for a bounded feature, change set, or release candidate, including handoffs, dependencies, parallel work, reconvergence points, control-sensitive routing, and downstream handoff expectations.

## Scenario In Scope

This integration map applies to the reusable `Secure Delivery / Secure Feature Lifecycle` scenario defined in [secure-delivery-feature-lifecycle.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.definition.md).

## Participating Role-Owned Workflows

- [`business-analyst-kickoff.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-kickoff.md) or [`product-owner-direction.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-direction.md)
- [`solution-architect-design.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-design.md)
- [`security-compliance-specialist-scoping.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
- [`security-compliance-specialist-review.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-review.md)
- [`security-compliance-specialist-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-handoff.md)
- [`technical-planner-planning.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-planning.md)
- [`technical-planner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-review.md)
- [`technical-planner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-handoff.md)
- [`implementation-engineer-execution.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-execution.md)
- [`implementation-engineer-review.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-review.md)
- [`implementation-engineer-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-handoff.md)
- [`code-reviewer-analysis.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-analysis.md)
- [`code-reviewer-decision.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-decision.md)
- [`code-reviewer-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-handoff.md)
- [`qa-verification-planning.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-planning.md)
- [`qa-verification-review.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-review.md)
- [`qa-verification-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-handoff.md)
- [`release-handoff-manager-packaging.md`](C:/Users/ericw/Projects/orpheum/workflows/release-handoff-manager-packaging.md)
- [`release-handoff-manager-readiness-review.md`](C:/Users/ericw/Projects/orpheum/workflows/release-handoff-manager-readiness-review.md)
- [`release-handoff-manager-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/release-handoff-manager-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Business Analyst or Product Owner workflows consume the upstream business need and produce:
  - validated feature intent
  - priority or acceptance framing
  - any product-direction notes that materially constrain the secure slice
- Solution Architect workflows consume the reviewed business direction and produce:
  - reviewed architecture
  - trust-boundary framing
  - solution-shape constraints that security-sensitive implementation must honor
- Security / Compliance Specialist workflows consume the reviewed delivery context and produce:
  - security/compliance scope
  - controls and evidence matrix
  - security/compliance review
  - security/compliance handoff
- Technical Planner workflows consume reviewed architecture and security/compliance framing, then produce:
  - implementation-shaped plan
  - reviewed planning posture
  - handoff that preserves the secure slice boundary and control constraints
- Implementation Engineer workflows consume reviewed planning and control posture, then produce:
  - implementation record
  - implementation evidence
  - implementation readiness review
  - implementation handoff
- Code Reviewer workflows consume the implementation package, then produce:
  - review scope
  - review findings
  - review decision
  - review handoff
- QA / Verification Lead workflows consume reviewed requirements, architecture, implementation evidence, and control posture, then produce:
  - verification strategy
  - verification matrix
  - evidence review
  - verification handoff
- Release / Handoff Manager workflows consume the reviewed implementation, review, verification, and security/compliance packages, then produce:
  - release candidate summary
  - rollout and operations notes
  - release readiness decision
  - release handoff

Shared artifacts and context that move across the scenario:

- the reviewed feature intent and its bounded delivery slice
- the trust boundary, control expectations, and human control points that shape the secure feature
- the security and compliance obligations, evidence expectations, waiver posture, and unresolved gaps
- the implementation scope, changed-area summary, deviations, and local evidence posture
- the independent review findings, approval posture, and re-review triggers
- the verification coverage expectations, evidence strength, residual gaps, and re-verification triggers
- the release scope for the same bounded slice or release candidate, active conditions, rollout watchouts, and approval-sensitive constraints
- participant-role fit and skill-sufficiency evidence when the scenario package needs to stay honest about whether the current role set is usable as-is

## Handoff Contracts

- upstream planning roles -> Security / Compliance Specialist
  - Security / Compliance Specialist should receive reviewed business intent, architecture, and planning context so risks and obligations can be framed against the actual feature boundary instead of a generic compliance checklist
- Security / Compliance Specialist -> Technical Planner and Implementation Engineer
  - downstream delivery roles should receive explicit control expectations, evidence needs, trust-boundary notes, and human checkpoints rather than discovering them from scattered comments later
- Technical Planner -> Implementation Engineer
  - Implementation Engineer should receive a bounded execution shape that preserves the security-sensitive constraints and does not re-plan the slice in code by default
- Implementation Engineer -> Code Reviewer
  - Code Reviewer should receive a reviewed implementation package with explicit scope, evidence posture, and control-sensitive watchouts rather than a raw diff alone
- Implementation Engineer -> QA / Verification Lead
  - QA / Verification Lead should receive reviewed implementation context plus at least one concrete evidence source rather than being forced to infer verification scope from commits or chat history
- Code Reviewer or QA / Verification Lead -> Security / Compliance Specialist
  - when review or verification reveals a security, compliance, or control-posture gap, the gap should be packaged explicitly rather than left as an informal comment thread
- Code Reviewer -> Release / Handoff Manager
  - Release / Handoff Manager should receive an explicit review posture, active findings, and re-review triggers rather than only a yes-or-no approval summary
- QA / Verification Lead -> Release / Handoff Manager
  - Release / Handoff Manager should receive an explicit verification posture, evidence strength summary, and active conditions rather than a vague QA pass signal
- Security / Compliance Specialist -> Release / Handoff Manager
  - Release / Handoff Manager should receive an explicit security/compliance posture, control expectations, unresolved gaps, and human-approval watchouts rather than a soft "security aware" label
- Code Reviewer or QA / Verification Lead -> `Review Remediation Loop`
  - when review or verification surfaces concrete blocking or conditional findings whose honest next step is bounded remediation inside the current slice, those findings should be packaged explicitly as the normal upstream input to `Review Remediation Loop`

## Branching Rules And Decision Logic

- If reviewed planning inputs are unstable, route upstream before implementation starts rather than letting implementation invent missing direction.
- If the input package still represents too much of the overall project to serve as one honest secure-delivery boundary, route upstream to narrower slice planning rather than forcing the whole project through this scenario at once.
- If security/compliance scope cannot be made explicit, block the scenario until the control posture is framed honestly.
- If implementation-package review is blocked, keep the work inside the Implementation Engineer loop rather than pushing a soft-ready handoff downstream.
- If code review is blocked, route remediation back to Implementation Engineer and require re-review before release preparation proceeds.
- If QA / Verification Lead lacks at least one concrete evidence source for the verified scope, route remediation to evidence generation or a narrower verification claim rather than producing a fake verification posture.
- If security/compliance review reveals blocked obligations, unresolved waivers, or missing approval limits, route remediation upstream or to the correct human approver rather than smoothing the issue into the release package.
- If review or verification findings show that the candidate still belongs inside the same bounded slice but needs another explicit remediation pass, route that candidate into `Review Remediation Loop` with the concrete findings package preserved.
- If release-readiness is blocked or materially conditional, preserve that posture explicitly instead of smoothing it into adoption-ready language.

## Parallelism And Synchronization Points

- Security / Compliance Specialist scoping may overlap with architecture review once the feature boundary and trust boundaries are stable enough to frame honestly.
- Technical Planner work may overlap with security/compliance scoping once the control posture is visible enough to shape the slice.
- Code Reviewer analysis and QA / Verification planning may overlap once the implementation package handoff exists and concrete evidence is beginning to accumulate.
- Security / Compliance review may continue while review findings are being finalized, but release preparation must not treat the candidate as settled until security posture, review posture, and verification posture are all explicit.
- The scenario must reconverge at:
  - explicit security/compliance scope before implementation relies on the control posture
  - reviewed implementation-package readiness before independent review and verification rely on the package
  - explicit code-review decision before release preparation relies on independent challenge
  - explicit verification review before release preparation relies on evidence strength
  - explicit security/compliance review before release preparation relies on control posture
  - explicit release-readiness decision before downstream consumers treat the release-preparation package as actionable

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes reviewed upstream planning already exists for the current bounded feature and is stable enough to support secure implementation work.
- The scenario assumes implementation, independent review, verification, security/compliance framing, and release preparation are distinct downstream functions even when they happen close together in time.
- The scenario assumes release preparation packages a candidate for downstream handling and does not itself authorize or perform deployment by default.
- The scenario assumes the current Business Analyst or Product Owner, Solution Architect, Security / Compliance Specialist, Technical Planner, Implementation Engineer, Code Reviewer, QA / Verification Lead, and Release / Handoff Manager packages are usable here without additional scenario-specific hardening.
- The scenario assumes existing Allium behavior remains the source of truth when a mature behavioral specification materially constrains security-sensitive or trust-boundary-sensitive work.
- The scenario assumes the current feature slice is narrow enough that security and compliance work can remain explicit without turning the scenario into a whole-program governance model.

## Failure Handling And Escalation Routing

- If implementation exposes requirement, architecture, planning, or security/compliance instability, route upstream rather than freezing an invented local workaround into the scenario.
- If review reveals an upstream design, planning, or control-posture defect, route that defect to the earliest accountable role rather than disguising it as a purely local code smell.
- If verification reveals weak evidence, contradictory signals, or unsupported acceptance claims, route remediation to implementation, evidence generation, or upstream scope correction rather than averaging the weakness into a vague pass.
- If security/compliance review reveals unowned approvals, missing control evidence, or unresolved obligations, route those gaps to the correct downstream owner or human approver rather than hiding them in upbeat release language.
- If release preparation reveals unresolved approval, evidence, or operational constraints, route remediation upstream rather than treating the package as release-ready.

## Coordination Risks And Watchouts

- Implementation Engineer and Code Reviewer boundaries can blur if implementation-package self-review starts sounding like independent approval.
- Code Reviewer and QA / Verification Lead boundaries can blur if review findings about weak evidence are mistaken for the full verification verdict.
- Security / Compliance Specialist and Release / Handoff Manager boundaries can blur if control posture is mistaken for release authority.
- QA / Verification Lead and Release / Handoff Manager boundaries can blur if verification artifacts drift into rollout instructions or release authority.
- This scenario is easy to overread as deployment execution; actual deployment and operational control must stay outside the scenario boundary unless a future scenario explicitly adds them.
- This scenario is also easy to overread as "implement the whole secure feature program"; the bounded slice boundary must stay explicit from entry through release packaging.
- Teams may skip reconvergence and jump straight from implementation to release packaging if review, verification, and security/compliance gates are not kept explicit.

## Recommended Next Step

Use the Secure Delivery / Secure Feature Lifecycle review artifact to make readiness, limits, participant fit, and remediation routing explicit before treating this scenario as adoption-ready.


# Implementation and Release Prep Scenario Integration Map

## Purpose

Capture how the `Implementation and Release Prep` scenario composes role-owned workflows into one coherent downstream delivery phase for a bounded delivery slice, including handoffs, dependencies, parallelism, reconvergence points, and downstream routing expectations.

## Scenario In Scope

This integration map applies to the reusable `Implementation and Release Prep` scenario defined in [implementation-and-release-prep.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.definition.md).

## Participating Role-Owned Workflows

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

- Implementation Engineer workflows consume reviewed planning inputs, then produce:
  - implementation record
  - implementation evidence
  - implementation readiness review
  - implementation package handoff
- Code Reviewer workflows consume the implementation package, then produce:
  - code review scope
  - review findings
  - review decision
  - review handoff
- QA / Verification Lead workflows consume reviewed requirements, architecture, implementation planning, implementation handoff, and concrete evidence sources, then produce:
  - verification strategy
  - verification matrix
  - evidence review
  - verification handoff
- Release / Handoff Manager workflows consume the reviewed implementation, review, and verification packages, then produce:
  - release candidate summary
  - rollout and operations notes
  - release readiness decision
  - release handoff

Shared artifacts and context that move across the scenario:

- reviewed requirements, architecture, and implementation-planning handoffs for the current bounded delivery slice from upstream planning work
- implementation scope, changed-area summary, deviations, and evidence posture
- independent review findings, decision scope, and re-review triggers
- verification coverage expectations, evidence strength, residual gaps, and re-verification triggers
- release scope for the same bounded slice or release candidate, active conditions, rollout watchouts, and approval-sensitive constraints
- participant-role fit and skill-sufficiency evidence when role-package review materially explains why the scenario is ready without additional hardening

## Handoff Contracts

- upstream planning package -> Implementation Engineer
  - Implementation Engineer should receive reviewed requirements, architecture, and implementation-planning inputs for one bounded delivery slice without re-planning the work in code by default
- Implementation Engineer -> Code Reviewer
  - Code Reviewer should receive a reviewed implementation package with explicit scope, evidence posture, readiness, and revalidation watchouts rather than a raw diff alone
- Implementation Engineer -> QA / Verification Lead
  - QA / Verification Lead should receive reviewed implementation context plus at least some concrete evidence source rather than being forced to infer verification scope from commits or chat history
- Code Reviewer -> Release / Handoff Manager
  - Release / Handoff Manager should receive an explicit review posture, active findings, and re-review triggers rather than only a yes-or-no approval summary
- QA / Verification Lead -> Release / Handoff Manager
  - Release / Handoff Manager should receive an explicit verification posture, evidence strength summary, and active conditions rather than a vague QA pass signal
- Release / Handoff Manager -> `Verification And Release Gate`
  - When the candidate still needs a distinct final downstream gate after implementation-side review, verification, and release preparation are complete, `Verification And Release Gate` should receive the reviewed candidate package rather than reconstructing it from branch history or approval chatter
- Code Reviewer or QA / Verification Lead -> `Review Remediation Loop`
  - When review or verification surfaces concrete blocking or conditional findings whose honest next step is bounded remediation inside the current slice, those findings should be packaged explicitly as the normal upstream input to `Review Remediation Loop` rather than left as informal comments, chat context, or vague iteration pressure

## Branching Rules And Decision Logic

- If reviewed planning inputs are unstable, route upstream before implementation starts rather than letting implementation invent missing direction.
- If the input package still represents too much of the overall project to serve as one honest implementation boundary, route upstream to narrower slice planning rather than forcing the whole project through this scenario at once.
- If implementation-package review is blocked, keep the work inside the Implementation Engineer loop rather than pushing a soft-ready handoff downstream.
- If code review is blocked, route remediation back to Implementation Engineer and require re-review before release preparation proceeds.
- If QA / Verification Lead lacks at least one concrete evidence source for the verified scope, route remediation to evidence generation or a narrower verification claim rather than producing a fake verification posture.
- If review or verification findings show that the candidate still belongs inside the same bounded slice but needs another explicit remediation pass, route that candidate into `Review Remediation Loop` with the concrete findings package preserved.
- If the candidate is implementation-complete but still requires a distinct final downstream gate because approval structure, trust-boundary sensitivity, or operational separation demands it, route the package into `Verification And Release Gate` rather than stretching this scenario into deployment authority.
- If release-readiness is blocked or materially conditional, preserve that posture explicitly instead of smoothing it into adoption-ready language.

## Parallelism And Synchronization Points

- Code Reviewer analysis and QA / Verification planning may overlap once the implementation package handoff exists and concrete evidence is beginning to accumulate.
- QA / Verification evidence review may continue while review findings are being finalized, but release preparation must not treat the candidate as settled until both review and verification postures are explicit.
- The scenario must reconverge at:
  - reviewed implementation-package readiness before independent review and verification rely on the package
  - explicit code-review decision before release preparation relies on independent challenge
  - explicit verification review before release preparation relies on evidence strength
  - explicit release-readiness decision before downstream consumers treat the release-preparation package as actionable

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes reviewed upstream planning already exists for the current bounded slice and is stable enough to support coding work.
- The scenario assumes implementation, independent review, verification, and release preparation are distinct downstream functions even when they happen close together in time.
- The scenario assumes release preparation packages a candidate for downstream handling and does not itself authorize or perform deployment by default.
- The scenario assumes it may be repeated many times across one larger project, roadmap, or initiative rather than consuming the entire project scope in one pass by default.
- The scenario assumes the current Implementation Engineer, Code Reviewer, QA / Verification Lead, and Release / Handoff Manager packages are usable here without additional scenario-specific hardening, and that this as-is fit should remain visible in the scenario-readiness story.

## Failure Handling And Escalation Routing

- If implementation exposes requirement, architecture, or planning instability, route upstream rather than freezing an invented local workaround into the scenario.
- If review reveals an upstream design or planning defect, route that defect to the earliest accountable role rather than disguising it as a purely local code smell.
- If verification reveals weak evidence, contradictory signals, or unsupported acceptance claims, route remediation to implementation, evidence generation, or upstream scope correction rather than averaging the weakness into a vague pass.
- If release preparation reveals unresolved approval, evidence, or operational constraints, route remediation upstream rather than hiding the problem in upbeat release language.

## Coordination Risks And Watchouts

- Implementation Engineer and Code Reviewer boundaries can blur if implementation-package self-review starts sounding like independent approval.
- Code Reviewer and QA / Verification Lead boundaries can blur if review findings about weak evidence are mistaken for the full verification verdict.
- QA / Verification Lead and Release / Handoff Manager boundaries can blur if verification artifacts drift into rollout instructions or release authority.
- This scenario is easy to overread as deployment execution; actual deployment and operational control must stay outside the scenario boundary unless a future scenario explicitly adds them.
- This scenario is also easy to overread as “implement the whole project”; the bounded slice boundary must stay explicit from entry through release packaging.
- Teams may skip reconvergence and jump straight from implementation to release packaging if review and verification gates are not kept explicit.

## Recommended Next Step

Use the Implementation and Release Prep review artifact to make readiness, limits, and remediation routing explicit before treating this scenario as adoption-ready.

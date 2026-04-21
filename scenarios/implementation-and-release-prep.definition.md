# Implementation and Release Prep Scenario Definition

## Purpose

Capture the reusable `Implementation and Release Prep` scenario that turns reviewed upstream planning into a reviewed implementation package, an explicit independent review posture, an evidence-based verification package, and a downstream-ready release-preparation package before actual deployment, adoption handling, or a distinct final downstream gate begins.

Use this scenario when a team needs a clear multi-role downstream delivery phase rather than a loose transition from planning into coding, review, QA, and release packaging.

## Scenario Name And Intent

`Implementation and Release Prep`

This scenario exists to compose the repository's downstream delivery roles into one reusable post-planning phase that reduces ambiguity between "the work is planned" and "the candidate is packaged honestly for release-adjacent handling."

## Lifecycle Window And Trigger Conditions

This scenario sits between reviewed project planning or slice planning and actual release, deployment, adoption handling, or any distinct final downstream gate.

Trigger it when:

- reviewed requirements, architecture, and implementation planning already exist for a bounded delivery slice or release candidate scope
- implementation needs to produce more than a raw diff or commit summary
- independent review, explicit verification, and release-adjacent packaging need a durable handoff chain
- a project slice, milestone slice, feature slice, or release wave is ready to move from approved planning into coding, review, verification, and release preparation

## Participating Roles And Why

- [`Implementation Engineer`](C:/Users/ericw/Projects/orpheum/roles/implementation-engineer.md)
  - turns reviewed requirements, architecture, and implementation planning into a code change set plus a reviewed implementation package
- [`Code Reviewer`](C:/Users/ericw/Projects/orpheum/roles/code-reviewer.md)
  - turns the implementation package into an explicit independent review posture with concrete findings and decision scope
- [`QA / Verification Lead`](C:/Users/ericw/Projects/orpheum/roles/qa-verification-lead.md)
  - turns reviewed planning and implementation evidence into an explicit verification posture, evidence review, and downstream verification handoff
- [`Release / Handoff Manager`](C:/Users/ericw/Projects/orpheum/roles/release-handoff-manager.md)
  - turns reviewed implementation, review, and verification outputs into a release-preparation package with explicit readiness posture, conditions, and rollout watchouts

## Entry Conditions

- reviewed requirements, architecture, and implementation-planning handoffs already exist for the current bounded delivery slice, or equivalent reviewed upstream planning inputs are available
- the current delivery slice is stable enough to begin coding without inventing missing upstream direction in code
- the participating role packages are available and treated as the source of truth for role-local workflows
- the scenario is being used as a reusable downstream delivery phase, not as a deployment runbook or incident-management framework

## Target Outputs And Exit Conditions

The scenario completes successfully when the downstream delivery package includes:

- a reviewed implementation package for the current bounded slice, with explicit evidence posture and implementation-package readiness
- an independent code-review package with explicit findings, approval posture, and re-review triggers
- a reviewed verification package with explicit evidence strength, gaps, readiness, and re-verification triggers
- a release-preparation package for the same bounded slice or release candidate, with explicit candidate scope, current release posture, conditions, rollout watchouts, and downstream handoff guidance

Exit condition:

- downstream release-adjacent, final-gate, or human approval consumers can evaluate the current bounded slice or release candidate from durable implementation, review, verification, and release-preparation artifacts rather than reconstructing intent from commits, chat history, or scattered notes

## Core Sequence

1. Consume the reviewed planning package for the current bounded slice and execute that approved slice through Implementation Engineer outputs.
2. Review the resulting implementation package for implementation-package completeness and downstream readiness before handoff.
3. Package the reviewed implementation outputs for independent review and verification work.
4. Turn the implementation package into an explicit Code Reviewer findings and decision package.
5. Turn the reviewed planning inputs plus concrete implementation evidence into an explicit QA / Verification Lead strategy, evidence review, and verification handoff package.
6. Reconverge the reviewed implementation, review, and verification packages before release-preparation packaging begins.
7. Turn the reconverged downstream package into a release candidate summary, release-readiness decision, and release handoff package.
8. Hand the completed release-preparation package for that bounded slice downstream for human approval, deployment handling, adoption handling, or `Verification And Release Gate` when a distinct final downstream gate is required.

## Decision Gates And Human Checkpoints

- implementation-package review must be explicit before downstream review or verification trusts the implementation package
- code-review decision must be explicit before release preparation treats the candidate as independently challenged
- verification review must be explicit before release preparation treats the candidate as evidence-supported
- release-readiness decision must be explicit before downstream consumers treat the package as ready, conditional, or blocked for release-adjacent handling
- human approval remains visible when blocked findings, conditional verification, rollout-sensitive risk, or environment-specific authorization exceed the scenario's default authority

## Scenario Constraints And Non-Goals

- This scenario does not replace role-local workflows; it composes them.
- This scenario does not replace upstream discovery, product direction, architecture, or implementation planning.
- This scenario is not the default mechanism for executing an entire project backlog or roadmap in one pass.
- This scenario does not absorb actual deployment execution, incident response, or long-running operational ownership.
- This scenario does not replace `Verification And Release Gate` when a distinct downstream gate owned mainly by verification, security/compliance, and release-handoff roles is still required after implementation work is complete.
- This scenario should stay reusable across projects and should not be overfit to a single CI/CD stack, release ceremony, or environment model.

## Open Questions And Design Gaps

- Repeated usage may show a need for a dedicated slice-planning scenario that turns larger project plans into bounded slices before this downstream scenario starts.
- Repeated usage may show a need for a narrower remediation-loop scenario when review or verification findings force multiple implementation iterations.
- Repeated usage may show a need for a stronger rule about when verification planning should start relative to late-stage code review in fast-moving delivery slices.

## Recommended Next Step

Use the Implementation and Release Prep integration map to make handoffs, dependencies, parallelism, reconvergence points, and downstream routing explicit.

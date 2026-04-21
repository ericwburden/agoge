# Review Remediation Loop Scenario Integration Map

## Purpose

Capture how the `Review Remediation Loop` scenario composes role-owned workflows into one coherent remediation-and-re-review loop for a bounded delivery slice.

## Scenario In Scope

This integration map applies to the reusable `Review Remediation Loop` scenario defined in [review-remediation-loop.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.definition.md).

## Participating Role-Owned Workflows

- [`code-reviewer-analysis.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-analysis.md)
- [`code-reviewer-decision.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-decision.md)
- [`code-reviewer-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/code-reviewer-handoff.md)
- [`implementation-engineer-execution.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-execution.md)
- [`implementation-engineer-review.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-review.md)
- [`implementation-engineer-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/implementation-engineer-handoff.md)
- optional [`qa-verification-review.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-review.md)
- optional [`qa-verification-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Code Reviewer workflows consume the current implementation package, then produce:
  - review findings
  - review decision
  - review handoff
- Implementation Engineer workflows consume the current bounded-slice implementation context together with review findings and remediation needs, then produce:
  - refreshed implementation record
  - refreshed implementation evidence
  - refreshed implementation readiness review
  - refreshed implementation package handoff
- Optional QA / Verification Lead workflows consume the refreshed implementation context and any relevant evidence sources, then produce:
  - refreshed evidence review
  - optional refreshed verification handoff when verification-sensitive concerns still shape downstream trust

Shared artifacts and context that move across the scenario:

- the current bounded-slice implementation package
- the current review findings, review decision, review handoff, and any verification findings or conditions that materially shape whether remediation can clear the candidate
- remediation targets, re-review triggers, and unresolved conditions
- refreshed implementation scope, evidence, and residual risks after remediation
- optional verification findings and re-verification triggers when evidence-sensitive concerns remain in play

## Handoff Contracts

- `Implementation and Release Prep` -> Review Remediation Loop
  - `Review Remediation Loop` should normally receive explicit blocked or conditional findings from Code Reviewer or QA / Verification Lead outputs produced during `Implementation and Release Prep`, rather than relying on informal comments or vague awareness that the candidate "still has issues"
- Code Reviewer -> Implementation Engineer
  - Implementation Engineer should receive explicit findings, current review posture, and re-review triggers rather than a vague request to "fix the PR"
- Implementation Engineer -> Code Reviewer
  - Code Reviewer should receive a refreshed implementation package with explicit remediation changes and evidence updates rather than a bare updated diff
- Implementation Engineer -> optional QA / Verification Lead
  - QA / Verification Lead should receive refreshed implementation context and evidence when review findings exposed verification-sensitive weaknesses that remain material after remediation
- optional QA / Verification Lead -> Code Reviewer or downstream delivery
  - When verification re-enters the loop, its refreshed posture should preserve whether review can continue, whether downstream trust is still conditional, and what must be rechecked next

## Branching Rules And Decision Logic

- If the current review findings are really exposing an upstream requirement, architecture, planning, or slice-boundary defect, route out of this loop rather than pretending the issue is local remediation only.
- If Implementation Engineer cannot remediate the findings without expanding or redefining the approved slice, stop and route upstream instead of silently turning remediation into replanning.
- If the refreshed implementation package is still weak or misleading, keep the work inside the Implementation Engineer branch rather than rushing back to review.
- If Code Reviewer still finds concrete defects after remediation, repeat the loop while the candidate is still converging and the issue remains local to the current slice.
- If the main unresolved issue is now evidence strength rather than code correctness, invoke QA / Verification Lead rather than forcing code review to stand in for verification.

## Parallelism And Synchronization Points

- Implementation remediation and local evidence refresh may overlap, but the scenario must reconverge at a refreshed implementation package before re-review is treated as honest.
- Optional QA / Verification Lead review may overlap with late Code Reviewer work when both are responding to the same refreshed candidate, but the scenario must keep their judgments distinct.
- The scenario must reconverge at:
  - explicit review findings before remediation begins
  - a refreshed implementation package before re-review begins
  - a refreshed review decision before downstream trust increases
  - optional refreshed verification posture before the candidate is treated as settled when evidence-sensitive concerns materially remain

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes the current unit of work is still one bounded slice rather than a broad project scope.
- The scenario assumes remediation is distinct from first-pass implementation because it is anchored to explicit review findings and re-review discipline.
- The scenario assumes `Implementation and Release Prep` is the normal upstream scenario that surfaces and packages the blocked or conditional findings consumed here.
- The scenario assumes the loop may repeat, but only while each pass is still yielding a materially stronger candidate or a clearer unresolved posture.
- The scenario assumes the current Implementation Engineer, Code Reviewer, and optional QA / Verification Lead packages are usable here as-is, with no additional scenario-specific hardening currently required.

## Failure Handling And Escalation Routing

- If the findings reveal a slice-boundary defect, route to `Delivery Slice Planning` rather than stretching remediation to absorb scope reshaping.
- If the findings reveal a planning or architecture defect, route upstream rather than hiding the problem in repeated implementation churn.
- If the candidate is not getting materially stronger from pass to pass, stop and make the stalled posture explicit instead of running the loop indefinitely.
- If verification confidence is the real blocker, route to QA / Verification Lead rather than treating repeated code review as equivalent to stronger evidence.

## Coordination Risks And Watchouts

- Implementation Engineer and Code Reviewer boundaries can blur if remediation notes start replacing explicit refreshed implementation artifacts.
- Code Reviewer and QA / Verification Lead boundaries can blur if evidence-sensitive concerns are treated as just more review findings instead of optional verification work.
- This scenario is easy to misuse as a general "keep iterating until someone feels good" loop; explicit findings, refreshed artifacts, and convergence judgment must stay visible.
- This scenario is also easy to underuse, with teams pushing blocked candidates forward without an explicit remediation path.

## Recommended Next Step

Use the Review Remediation Loop review artifact to make readiness, participant fit, optional verification use, and convergence limits explicit before treating this scenario as adoption-ready.

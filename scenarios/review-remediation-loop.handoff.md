# Review Remediation Loop Scenario Handoff

## Purpose

Package the reusable `Review Remediation Loop` scenario for downstream adopters, workflow authors, or project leads so they can use it without reconstructing the remediation path from implementation notes, review comments, and chat history.

## Current Scenario Summary

`Review Remediation Loop` is a reusable multi-role scenario that starts from a bounded slice with concrete review findings, turns those findings into scoped remediation work, refreshes the implementation package, re-runs review, optionally re-engages QA / Verification Lead when evidence-sensitive concerns remain material, and either converges to a stronger downstream posture or routes the blocker out of the loop honestly.

Primary participating roles:

- `Code Reviewer`
- `Implementation Engineer`
- optional `QA / Verification Lead`

## Scenario Package Included

- [review-remediation-loop.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.definition.md)
- [review-remediation-loop.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.integration-map.md)
- [review-remediation-loop.review.md](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.review.md)
- supporting rationale in [scenario-recommendations.md](C:/Users/ericw/Projects/orpheum/notes/scenario-recommendations.md)
- participant-package rationale in [implementation-engineer-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/implementation-engineer-skill-sourcing.md), [code-reviewer-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/code-reviewer-skill-sourcing.md), and [qa-verification-skill-sourcing.md](C:/Users/ericw/Projects/orpheum/notes/qa-verification-skill-sourcing.md)
- the explicit participant-fit judgment that the current core role packages are usable in this scenario as-is, with QA / Verification Lead remaining an optional branch

## Current Readiness Posture

`ready`

This scenario is ready for downstream adoption and local tailoring as a reusable remediation-and-re-review loop.

Limits:

- it is not a replacement for broader replanning, re-slicing, or architecture revision
- it is not the first-pass implementation scenario itself
- it does not replace role-local workflows
- optional QA / Verification Lead participation should remain trigger-based rather than automatic

## Role And Workflow Routing Guidance

Use the scenario in this broad order:

1. Code Reviewer analysis, decision, and handoff to make the remediation target explicit
2. Implementation Engineer execution, review, and handoff to remediate the findings and refresh the implementation package
3. Code Reviewer analysis, decision, and handoff again to re-evaluate the remediated candidate
4. optional QA / Verification Lead review and handoff when evidence-sensitive concerns remain material after remediation
5. either repeat the loop while the candidate is still getting materially stronger, or route the blocker elsewhere when the issue is no longer honest local remediation

Downstream consumers should preserve:

- the rule that the scenario operates on one bounded slice
- the rule that `Implementation and Release Prep` is the normal upstream producer of the concrete review or verification findings consumed here
- the rule that remediation is anchored to explicit review findings, not vague iteration pressure
- the distinction between refreshed implementation packaging and independent re-review
- the optional nature of QA / Verification Lead unless evidence-sensitive blockers require it
- the convergence rule that the loop should stop when the candidate is no longer materially improving or the blocker is no longer local to the current slice
- the participant-fit judgment that the current core role packages are usable here as-is unless repeated usage proves otherwise
- the rule that a successfully remediated candidate should normally re-enter `Implementation And Release Prep` or `Verification And Release Gate`, depending on what downstream work remains

## Entry Conditions For The Next Consumer

Before using this scenario, the next consumer should confirm:

- a bounded slice already has an implementation package and a concrete blocked or conditional findings package, normally from `Implementation and Release Prep`
- the current blocker is concrete review findings, conditions, or evidence gaps
- the remediation target still fits inside the current bounded slice
- the role packages referenced in the scenario are available
- the local context does not actually require upstream replanning instead of remediation

## Active Conditions, Risks, And Watchouts

- avoid using this scenario when the real defect is a slice-boundary, planning, or architecture problem
- avoid letting remediation live only in comments or diffs instead of refreshed implementation artifacts
- avoid using repeated code review as a substitute for verification when evidence strength is the real blocker
- avoid letting the loop continue indefinitely once it has stopped producing a materially stronger candidate

## Follow-Up Owners

- Scenario Designer for future scenario-level hardening
- Role Builder if repeated usage exposes gaps in the participating role packages

## Revisit Triggers

- repeated usage shows the loop is hiding upstream defects instead of routing them
- repeated usage shows one of the participating role packages no longer supports this scenario cleanly without hardening
- repeated usage shows stronger QA / Verification Lead trigger guidance is needed
- repeated usage shows stronger convergence guidance is needed

## Recommended Next Consumer

- `Scenario Designer`
  - when tailoring this reusable scenario for another context
- `Implementation And Release Prep` or `Verification And Release Gate`
  - when remediation has converged and the candidate needs to re-enter the normal downstream delivery path
- `Role Builder`
  - when repeated usage reveals a missing role-package capability
- project leads or workflow authors
  - when choosing and applying a reusable remediation loop for bounded downstream delivery work

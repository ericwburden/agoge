# Review Remediation Loop Scenario Definition

## Purpose

Capture the reusable `Review Remediation Loop` scenario that turns review findings on a bounded delivery slice into explicit remediation work, refreshed implementation evidence, and a renewed downstream review posture without forcing teams to improvise the loop from scattered artifacts.

Use this scenario when a bounded slice has already entered downstream delivery work and code review or verification-sensitive concerns require another implementation-and-review pass before the candidate can move forward honestly.

## Scenario Name And Intent

`Review Remediation Loop`

This scenario exists to compose the repository's implementation, review, and optional verification roles into one reusable remediation loop that reduces ambiguity between "review found issues" and "we know how the candidate should be reworked, re-evidenced, and re-reviewed."

## Lifecycle Window And Trigger Conditions

This scenario sits inside or immediately after bounded downstream delivery work, typically after a slice has entered `Implementation and Release Prep` and review has produced findings that still block or condition downstream trust.

Trigger it when:

- a bounded delivery slice already has an implementation package and a concrete review or verification findings package, normally produced by `Implementation and Release Prep`
- `Code Reviewer` findings require implementation changes, stronger evidence, or clearer scope discipline before review can honestly pass
- the next correct step is remediation and re-review, not broader replanning of the whole project
- optional verification-sensitive concerns may need refreshed evidence or re-verification after remediation

## Participating Roles And Why

- [`Code Reviewer`](C:/Users/ericw/Projects/orpheum/roles/code-reviewer.md)
  - identifies concrete findings, preserves what must change or be clarified, and determines whether re-review can clear the candidate
- [`Implementation Engineer`](C:/Users/ericw/Projects/orpheum/roles/implementation-engineer.md)
  - turns the findings into scoped remediation work, refreshed implementation artifacts, and updated implementation evidence
- optional [`QA / Verification Lead`](C:/Users/ericw/Projects/orpheum/roles/qa-verification-lead.md)
  - participates when the review findings expose evidence weaknesses or verification-sensitive issues that must be rechecked before the candidate can move forward honestly

## Entry Conditions

- the candidate already has a reviewed implementation package and a concrete findings package, normally from `Implementation and Release Prep`
- at least one concrete review finding, condition, or evidence gap exists that requires remediation before downstream trust can increase
- the remediation target is still one bounded slice rather than a broad project replanning effort
- the participating role packages are available and treated as the source of truth for role-local workflows

## Target Outputs And Exit Conditions

The scenario completes successfully when the remediation effort yields:

- a refreshed implementation package that clearly reflects the remediation work performed
- an updated review posture that explicitly says whether the candidate is now approved, still conditional, or still blocked
- optional refreshed verification posture when evidence-sensitive findings required verification re-engagement
- explicit re-review or re-verification triggers if downstream trust still depends on further change

Exit condition:

- the candidate either converges back into the normal downstream path through `Implementation And Release Prep` or `Verification And Release Gate`, or is explicitly routed out of the loop because the blocker is not honest remediation work inside the current slice

## Core Sequence

1. Consume the current review package and identify the concrete findings, conditions, and re-review triggers that define the remediation target.
2. Turn those findings into scoped remediation work through refreshed Implementation Engineer artifacts and evidence.
3. Re-run implementation-package self-review and downstream implementation handoff so the changed candidate is explicit again, not implied by a diff alone.
4. Re-run Code Reviewer analysis, decision, and handoff against the remediated package.
5. Optionally re-engage QA / Verification Lead when the review findings exposed evidence weakness, verification-sensitive regressions, or confidence limits that cannot be resolved by code review alone.
6. Repeat the loop while each pass is still yielding materially stronger implementation, clearer evidence, or a narrower unresolved review posture.
7. Stop the loop when the candidate either clears review honestly, needs explicit verification-led judgment, or should be routed upstream because the real blocker is not local remediation inside the current slice.

## Decision Gates And Human Checkpoints

- the remediation target must stay explicit enough that Implementation Engineer is fixing named findings rather than reopening the whole slice
- refreshed implementation review must be explicit before re-review treats the candidate as stable again
- refreshed code-review decision must be explicit before downstream trust is extended further
- optional verification review should become an explicit gate when evidence-sensitive findings cannot be cleared by implementation and code review alone
- human approval remains visible when the loop exposes a risk, tradeoff, or escalation path that exceeds the scenario's default authority

## Scenario Constraints And Non-Goals

- This scenario does not replace role-local workflows; it composes them.
- This scenario does not replace broader project replanning, slice planning, or architecture revision when the review findings reveal an upstream defect.
- This scenario does not treat repeated remediation as a license to expand scope beyond the current bounded slice.
- This scenario does not absorb release handling or deployment authority.
- This scenario should stay reusable across projects and should not be overfit to one review tool or PR ceremony.

## Open Questions And Design Gaps

- Repeated usage may show a need for stronger default rules about when verification should re-enter the loop instead of remaining optional.
- Repeated usage may show a need for a separate scenario when review findings expose an upstream planning or architecture defect rather than a local remediation problem.
- Repeated usage may show a need for sharper guidance on when the loop has converged enough versus when it has stalled.

## Recommended Next Step

Use the Review Remediation Loop integration map to make remediation routing, re-review sequencing, optional verification participation, and convergence logic explicit.

# Review Decision

## Purpose

Capture the durable decision record for a drafted code review package, including the findings considered, the current review status, remediation conditions, unresolved risks, and what should happen next.

Use this artifact after the first findings pass and before producing the downstream review handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Code Reviewer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the reviewed implementation is approved, conditionally acceptable, or blocked, what findings drove that decision, what remediation is required, and which residual risks or unanswered questions still constrain downstream use.

## Related Checks

- Primary: [`review-decision.check.md`](D:/Projects/orpheum/checks/review-decision.check.md)
- Cross-cutting: [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md)
- Cross-cutting: [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md)

## Decision Scope

State which review scope and findings set this decision covers.

Make the reviewed revision, commit, branch state, or implementation package version explicit when that detail matters for interpreting approval correctly.

## Reviewed Inputs

Reference the code review scope, review findings, implementation package artifacts, upstream handoffs, and supporting evidence used in this decision.

## Overall Assessment

Summarize the review posture in plain language, including the strongest reasons for confidence and the most important reasons for caution.

## Review Status

State whether the reviewed implementation is:

- approved for downstream use
- approved with conditions
- blocked pending remediation

If relevant, note whether a human approver or downstream role still needs to confirm the next step.

State any approval limits clearly if the decision applies only to a specific reviewed revision, narrow scope slice, or evidence window.

## Decision Owner Or Approver

Identify who owns the current decision or who must explicitly accept the conditions if the status is not a plain approval.

## Blocking Findings Driving The Decision

List the findings that most materially justify a blocked or conditional decision.

## Non-Blocking Findings And Watchouts

List the concerns that remain important but do not by themselves block the current status.

## Required Remediation And Conditions

Describe the code changes, evidence generation, clarification, or upstream rework required before the review posture should change.

## Condition Owners

If the implementation is conditionally acceptable or blocked, identify who owns each follow-up, waiver, clarification, or re-review condition.

## Residual Risks And Open Questions

List unresolved risks, uncertainties, or decision-sensitive questions that still matter downstream.

## Verification And Release Watchouts

Call out what QA, verification, release-adjacent, or human-approval consumers should pay special attention to next.

## Re-Review Triggers

List what changes, evidence updates, scope expansions, or remediation outcomes should trigger another review decision rather than being treated as covered by the current one.

## Recommended Next Step

Describe the immediate next step, such as remediation, targeted re-review, downstream verification, or human approval.

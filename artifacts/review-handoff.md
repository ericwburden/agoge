---
id: review-handoff
kind: artifact
title: Review Handoff
version: 1
summary: Package the completed code review outputs into a downstream-ready handoff
  that lets implementation, verification, release-adjacent, or human decision-making
  roles use the review without reconstructing it from raw comments.
template: true
default_output_path: docs/verification/review-handoff.md
checks:
- review-handoff
- review-traceability
- code-reviewer-boundary
---

# Review Handoff

## Purpose

Package the completed code review outputs into a downstream-ready handoff that lets implementation, verification, release-adjacent, or human decision-making roles use the review without reconstructing it from raw comments.

Use this artifact after the review decision is explicit and before routing the review package downstream.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Code Reviewer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what change was reviewed, what the current approval posture is, which findings still matter, what follow-up is required, who owns it, and what downstream roles should do next.

## Related Checks

- Primary: [`review-handoff.check.md`](D:/Projects/orpheum/checks/review-handoff.check.md)
- Cross-cutting: [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md)
- Cross-cutting: [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md)

## Reviewed Change Summary

Summarize what implementation slice was reviewed and what major surfaces or change areas were in scope.

## Review Package Included

Reference the code review scope, review findings, review decision, and any supporting implementation artifacts or evidence that downstream readers should treat as part of the current review package.

## Current Approval Posture

State the current review status clearly, including whether the package is approved, approved with conditions, or blocked.

Make any revision, scope, or evidence limits on that posture explicit so downstream roles do not treat it as a blanket approval.

## Key Findings To Carry Forward

List the findings, watchouts, or evidence concerns that downstream roles should not lose sight of.

## Required Follow-Up

Describe the immediate follow-up required, such as remediation, clarification, re-review, verification focus, or explicit human approval.

## Follow-Up Owners

Identify who owns each required follow-up, waiver, or confirmation.

## Verification, Release, And Trust-Boundary Watchouts

Describe what downstream verification, release-adjacent, or human-approval consumers should pay special attention to.

## Re-Review Triggers

List the changes, evidence updates, waivers, or scope expansions that should cause the package to return to Code Reviewer work before downstream consumers rely on it further.

## Upstream Routing Notes

Call out any issues that should be routed to requirements, architecture, planning, or specification owners rather than treated purely as implementation cleanup.

## Recommended Next Consumer

Identify which downstream role or human decision-maker should take the package next and why.

---
id: implementation-readiness-review
kind: artifact
title: Implementation Readiness Review
version: 1
summary: Capture the explicit self-review of an implementation package before it is
  handed to downstream code review, verification, or release-adjacent roles.
template: true
default_output_path: docs/planning/implementation-readiness-review.md
checks:
- implementation-readiness-review
- implementation-traceability
- implementation-engineer-boundary
---

# Implementation Readiness Review

## Purpose

Capture the explicit self-review of an implementation package before it is handed to downstream code review, verification, or release-adjacent roles.

Use this artifact to record whether the implementation package is actually ready for downstream use, what findings still matter, and what remediation or upstream routing is still required.

This artifact is not a substitute for independent code review or downstream verification.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Implementation Engineer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what implementation package was self-reviewed, what the readiness judgment is, what findings and unresolved conditions still matter, and what remediation or routing should happen next.

## Related Checks

- Primary: [`implementation-readiness-review.check.md`](D:/Projects/orpheum/checks/implementation-readiness-review.check.md)
- Cross-cutting: [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md)
- Cross-cutting: [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md)

## Review Scope

Summarize which implementation package, revision, or slice this readiness review covers.

## Inputs Reviewed

Reference the implementation record, implementation evidence, relevant upstream handoff artifacts, and any supporting notes reviewed as part of this readiness judgment.

## Readiness Decision

State the current readiness state clearly, such as ready, ready with conditions, blocked, or routed upstream for rework.

## Findings

List the most important implementation findings that materially affect downstream use.

Separate:

- implementation defects
- weak or missing evidence
- upstream ambiguity
- downstream watchouts

## Remediation And Required Conditions

Describe the remediation, clarifications, approvals, or follow-up work required before downstream roles should treat the package as fully ready.

For each condition, capture:

- the owner when known
- whether it blocks all downstream use or only a specific path
- whether the issue belongs in implementation rework or upstream routing

## Residual Risks

List the major unresolved risks or confidence limits that remain even if the package moves downstream.

## Upstream Routing Notes

Record any requirement, architecture, planning, or specification issues that should be routed upstream rather than solved inside downstream review or verification.

## Recommendation For Downstream Use

Summarize what downstream roles can do next and what cautions should travel with the package.

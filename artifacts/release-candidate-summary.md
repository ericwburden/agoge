---
id: release-candidate-summary
kind: artifact
title: Release Candidate Summary
version: 1
summary: Capture the release or adoption target, included and excluded scope, reviewed
  upstream inputs, and release-sensitive hotspots for a concrete release candidate.
template: true
default_output_path: docs/release/release-candidate-summary.md
checks:
- release-candidate-summary
- release-traceability
- release-handoff-manager-boundary
---

# Release Candidate Summary

## Purpose

Capture the release or adoption target, included and excluded scope, reviewed upstream inputs, and release-sensitive hotspots for a concrete release candidate.

Use this artifact to make the candidate and its boundaries explicit before or alongside a release decision.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Release / Handoff Manager work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what candidate or downstream handoff target is in scope, what reviewed packages support it, what is included or deferred, and what release-sensitive hotspots still deserve attention.

## Related Checks

- Primary: [`release-candidate-summary.check.md`](D:/Projects/orpheum/checks/release-candidate-summary.check.md)
- Cross-cutting: [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md)
- Cross-cutting: [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md)

## Release Or Adoption Objective

Summarize what release, deployment wave, downstream adoption step, or handoff target this candidate covers.

## Reviewed Inputs

Reference the implementation package, code review package, verification package, relevant upstream handoffs, and any governing specifications or operational references that materially constrain this release candidate.

## Candidate Scope Included

Describe the implementation slices, change areas, features, fixes, migrations, prompts, assets, or operational changes included in this candidate.

## Explicitly Excluded Or Deferred Scope

List work that is not part of this candidate so later readers do not mistake silence for inclusion.

## Release Target And Consumers

Identify the target environment set, operational destination, release audience, or receiving team that this package is intended for.

## Upstream Decision Anchors

Map the candidate back to the upstream decisions that support release consideration.

For each major included area, capture:

- the implementation artifacts that describe it
- the review posture that currently applies to it
- the verification or evidence posture that currently applies to it
- any approval, trust-boundary, or specification-sensitive consideration that materially affects release treatment

## Release-Sensitive Hotspots

List the major hotspots this release candidate should keep visible, such as:

- migration or rollback risk
- environment or secret dependencies
- monitoring-sensitive behavior
- verification-sensitive or trust-boundary-sensitive areas
- staged rollout or communication-sensitive changes

## Candidate Limits And Assumptions

Describe the limits that affect how broadly the candidate can be trusted, such as:

- narrow evidence window
- scoped environment coverage
- pending approvals
- unresolved operational assumptions
- partial rollout intent

## Recommended Next Step

Describe the immediate next packaging or readiness step, such as release readiness review, operational review, approval collection, or downstream handoff.

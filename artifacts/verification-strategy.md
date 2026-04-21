---
id: verification-strategy
kind: artifact
title: Verification Strategy
version: 1
summary: Capture the verification approach that turns reviewed requirements, architecture,
  planning, and implementation context into a coherent evidence strategy.
template: true
default_output_path: docs/verification/verification-strategy.md
checks:
- verification-strategy
- verification-traceability
- qa-verification-boundary
---

# Verification Strategy

## Purpose

Capture the verification approach that turns reviewed requirements, architecture, planning, and implementation context into a coherent evidence strategy.

Use this artifact to make verification intent explicit so downstream roles do not have to infer what confidence signals matter, what evidence is expected, or where the major verification risks concentrate.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live QA / Verification Lead work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what verification scope is being covered, what kinds of evidence are expected, which risks matter most, what confidence standard is being applied, and what constraints or assumptions still shape the verification plan.

## Related Checks

- Primary: [`verification-strategy.check.md`](D:/Projects/agoge/checks/verification-strategy.check.md)
- Cross-cutting: [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
- Cross-cutting: [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)

## Verification Scope And Objective

Summarize what product, release, implementation slice, or delivery stage this verification work is covering and what confidence question it is trying to answer.

## Input Context

Reference the requirements handoff, architecture handoff, implementation handoff, reviewed planning artifacts, implementation evidence, and any existing behavioral specifications this verification work depends on.

## Verification Drivers And Risks

Describe the main drivers that shape verification priority, such as high-risk requirements, fragile interfaces, migration risk, compliance constraints, trust boundaries, or rollout sensitivity.

## Confidence Goals

Describe what level of confidence is required and for which parts of the scope.

For each major goal, capture:

- the concern being reduced
- the minimum evidence signal expected
- the consequence of weak or missing evidence

## Verification Levels And Methods

Describe the verification layers in scope, such as specification review, test evidence review, integration checks, manual validation, rollout safeguards, or human approval checkpoints.

## Evidence Expectations

List the kinds of evidence that should exist or be reviewed, such as automated test results, logs, screenshots, walkthrough notes, specification checks, defect history, interface validation, or production-like trial evidence.

For each evidence type, capture:

- when it is required versus optional
- which risks or scope areas it is intended to cover
- any known limits on what the evidence can prove

## Scope Exclusions And Deferrals

List the work that is intentionally outside this verification pass, deferred to a later stage, or left for a downstream role or approval step.

Keep true exclusions separate from evidence gaps or blocked verification.

## Verification Constraints And Assumptions

List environment limits, data limits, access issues, tooling gaps, timing limits, or policy constraints that materially affect how verification can be performed or judged.

## Architecture, Planning, And Specification Watchouts

Describe the architectural assumptions, planning hotspots, or specification-sensitive behaviors that deserve focused verification attention.

## Readiness Decision Framing

Describe how readiness will be judged, including whether outcomes are expected to be ready, ready with conditions, or blocked pending remediation.

Capture:

- who owns the readiness decision if it is not purely self-evident
- what kinds of conditions would prevent a clean ready state
- what evidence shortfalls would force escalation or upstream routing

## Open Questions

List the verification questions or uncertainties that remain unresolved before evidence review begins.

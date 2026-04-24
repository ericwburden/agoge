---
id: definition-of-done
kind: artifact
title: Definition Of Done
version: 1
summary: Capture the project-wide completion rules that implementation, review, verification,
  and release-adjacent work should use when deciding whether a bounded slice can
  honestly be treated as done.
template: true
default_output_path: docs/planning/definition-of-done.md
checks:
- definition-of-done
- planning-traceability
- technical-planner-boundary
---

# Definition Of Done

## Purpose

Capture the project-wide completion rules that implementation, review, verification, and release-adjacent work should use when deciding whether a bounded slice can honestly be treated as done.

Use this artifact to make the standing completion standard explicit so downstream roles do not have to infer "done" from team habit, chat history, or the most optimistic artifact in the chain.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Technical Planner work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what completion standard applies across implementation phases, which completion signals are mandatory versus situational, what evidence or approvals are expected before a slice is treated as done, and who can change that standard.

This artifact should preserve a project-level delivery standard. Do not collapse it into a single current slice unless the project-wide completion rule itself has changed.

## Related Checks

- Primary: [`definition-of-done.check.md`](D:/Projects/orpheum/checks/definition-of-done.check.md)
- Cross-cutting: [`planning-traceability.check.md`](D:/Projects/orpheum/checks/planning-traceability.check.md)
- Cross-cutting: [`technical-planner-boundary.check.md`](D:/Projects/orpheum/checks/technical-planner-boundary.check.md)

## Scope And Applicability

State what delivery scope this Definition of Done governs, such as the whole project, a major workstream, or a bounded product area.

Make explicit whether it applies to every delivery slice by default and what kinds of work, if any, are intentionally exempt.

## Source Inputs And Rationale

Reference the validated requirements, architecture handoff, product priorities, security/compliance posture, and any behavioral specifications or acceptance commitments that materially shaped this Definition of Done.

## Project-Level Completion Standard

Summarize what it means for work in this project to be honestly treated as done.

This should read like the standing delivery bar for the project, not a retrospective summary of a single slice.

## Required Implementation Conditions

List the implementation conditions that must be true before a slice can be treated as done.

Capture expectations such as:

- implementation scope completed to the intended slice boundary
- material deviations from plan made explicit
- required migrations, configuration changes, or integration updates handled
- documentation or operational changes preserved when the slice depends on them

## Required Review Conditions

List the review conditions that must be true before a slice can be treated as done.

Capture expectations such as:

- implementation-package self-review completed
- independent code review completed when required by project policy or risk level
- blocking findings remediated or explicitly routed

## Required Verification Conditions

List the verification and evidence conditions that must be true before a slice can be treated as done.

Capture expectations such as:

- required local validation completed
- required verification evidence exists for the slice type
- known evidence gaps are below the project’s acceptance threshold or explicitly routed
- requirement-, trust-boundary-, or rollout-sensitive behavior is verified at the expected level

## Required Documentation And Handoff Conditions

List the documentation, traceability, and handoff conditions that must be true before a slice can be treated as done.

Capture expectations such as:

- implementation artifacts updated
- downstream handoff artifacts are complete enough for review, verification, or release consumers
- changed decisions or constraints are reconciled across the affected artifact chain

## Security, Compliance, And Control Expectations

List the security, compliance, approval, or human-control-point expectations that must be preserved when relevant.

If these expectations are only conditional, state the triggering conditions explicitly.

## Release-Readiness Relationship

Explain how this Definition of Done relates to release readiness.

Make clear what completion standard is satisfied before release packaging begins versus what remains the responsibility of downstream release or final-gate workflows.

## Slice-Level Tailoring Rules

Explain how current slices should inherit this Definition of Done and what kinds of slice-local additions are allowed.

Preserve the distinction between:

- project-level completion rules that stay here
- slice exit criteria or current-slice details that belong in implementation strategy, sequencing, or downstream artifacts

## Explicit Non-Goals And Exclusions

List what this Definition of Done does not certify, such as broad release approval, production incident absence, or permanent support completeness.

## Change Control

State who can change this Definition of Done, when it should be revisited, and what kinds of triggers justify updating it.


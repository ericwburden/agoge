---
id: implementation-package-handoff
kind: check
title: Implementation Package Handoff Check
version: 1
summary: Validate that the implementation package handoff artifact preserves the implementation
  scope, change footprint, evidence posture, readiness state, and revalidation logic
  needed by downstream review and verification roles.
mode: headings
severity: error
applies_to:
- implementation-package-handoff
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Handoff Summary
- Implemented Scope Summary
- Change Footprint Summary
- Evidence Posture Summary
- Review Status And Key Findings
- Definition Of Done Status
- Known Issues And Residual Risks
- Specification Relationship
- Revalidation Triggers
- Recommended Downstream Consumers
- Next Decision Points
---

# Implementation Package Handoff Check

## Purpose

Validate that the implementation package handoff artifact preserves the implementation scope, change footprint, evidence posture, readiness state, and revalidation logic needed by downstream review and verification roles.

## Applies To

- instantiated copies of [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)
- Use after the implementation readiness review exists.
- Do not use as a substitute for earlier implementation completeness or evidence checks.

## Criteria

- The handoff summary and implemented scope summary are explicit.
- The change footprint summary highlights the code or interface areas that matter most downstream.
- The evidence posture summary reflects the real strengths and weaknesses already captured in the implementation evidence artifact.
- The review status and key findings match the implementation-package readiness review rather than inventing a cleaner narrative.
- Definition-of-Done status is explicit and does not confuse implemented scope with a broader project-level claim that the slice is fully done.
- The handoff does not blur implementation-package readiness with future independent code review or downstream verification.
- Known issues and residual risks are explicit.
- Specification relationship is clear when existing behavioral specifications materially constrain the implementation.
- Revalidation triggers are explicit.
- Recommended downstream consumers and next decision points are clear.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if downstream roles can continue the work without reconstructing the implementation package from earlier artifacts or code history.

## Evidence Required

- The implementation package handoff artifact.
- The implementation record, implementation evidence, and implementation readiness review artifacts it summarizes.
- Relevant upstream handoffs when traceability or routing depends on them.

If the handoff drops or distorts material implementation context, fail the check and identify what was lost.

## Supporting Skills

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the implementation package needs clearer downstream framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when cross-artifact synthesis is needed before the handoff can be written cleanly.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the handoff must explain implementation behavior relative to a governing specification.

## Failure Response

- Rework the handoff before expecting downstream roles to use it.
- Restore missing context rather than assuming downstream reviewers will infer it from diffs or local validation notes.

## Notes

This is the final implementation-package quality gate before downstream code review, verification, or release-adjacent work.

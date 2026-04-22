---
id: implementation-readiness-review
kind: check
title: Implementation Readiness Review Check
version: 1
summary: Validate that the implementation readiness review artifact records a clear
  implementation-package readiness judgment, meaningful findings, required remediation,
  and correct routing for upstream issues before downstream use.
mode: headings
severity: error
applies_to:
- implementation-readiness-review
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Review Scope
- Inputs Reviewed
- Readiness Decision
- Findings
- Remediation And Required Conditions
- Residual Risks
- Upstream Routing Notes
- Recommendation For Downstream Use
---

# Implementation Readiness Review Check

## Purpose

Validate that the implementation readiness review artifact records a clear implementation-package readiness judgment, meaningful findings, required remediation, and correct routing for upstream issues before downstream use.

## Applies To

- instantiated copies of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
- Use after the implementation record and implementation evidence artifacts both exist.
- Do not use as a substitute for independent downstream code review or QA verdicts.

## Criteria

- The review scope is explicit.
- Inputs reviewed are identified.
- A clear readiness decision is stated.
- Findings distinguish implementation defects, weak evidence, upstream ambiguity, and downstream watchouts.
- Remediation and required conditions are explicit and include ownership when known.
- Residual risks are visible.
- Upstream routing notes are present when requirement, architecture, planning, or specification defects were exposed.
- The artifact stays at implementation-package self-review level rather than drifting into independent code review language.
- The recommendation for downstream use is actionable without pretending the package is cleaner than it is.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if another role could tell whether the implementation package is ready, conditionally ready, or blocked, and why.

## Evidence Required

- The implementation readiness review artifact.
- The implementation record and implementation evidence artifacts it reviews.
- Relevant upstream planning, architecture, requirement, or specification artifacts when routing decisions depend on them.

If the readiness judgment cannot be justified from the reviewed package, fail the check and identify the unsupported or missing reasoning.

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the reviewed implementation context or evidence needs synthesis before readiness can be judged clearly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when readiness findings are strong but not yet expressed clearly enough for downstream use.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) or [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when readiness depends on known spec-to-code alignment concerns.

## Failure Response

- Rework the readiness review before treating the implementation package as downstream-ready.
- Route upstream defects to the earliest responsible artifact or role instead of hiding them in downstream cautions.

## Notes

This is the implementation-package self-review decision gate before handoff. It exists to keep weak or ambiguous implementation packages from moving downstream with false certainty.

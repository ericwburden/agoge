---
id: verification-handoff
kind: check
title: Verification Handoff Check
version: 1
summary: Validate that the verification handoff artifact packages the reviewed verification
  state accurately for downstream implementation, review, or release-adjacent use.
mode: headings
severity: error
applies_to:
- verification-handoff
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Handoff Summary
- Verification Summary
- Review Status And Key Findings
- Evidence Provenance Summary
- Readiness Ownership And Conditions
- Coverage And Evidence Hotspots
- Residual Risks And Weak Evidence
- Specification Relationship
- Scope Exclusions And Deferred Coverage
- Reverification Triggers
- Recommended Downstream Consumers
- Next Decision Points
---

# Verification Handoff Check

## Purpose

Validate that the verification handoff artifact packages the reviewed verification state accurately for downstream implementation, review, or release-adjacent use.

## Applies To

- instantiated copies of [`artifacts/verification-handoff.md`](D:/Projects/agoge/artifacts/verification-handoff.md)
- Use after the verification review artifact exists.
- Do not use as a substitute for the underlying evidence review or traceability checks.

## Criteria

- The handoff summary states what verification package is being handed off and why.
- The verification summary reflects the actual strategy, coverage state, evidence signals, and confidence limits already recorded upstream.
- Review status and key findings are preserved accurately.
- Evidence provenance summary is explicit enough that downstream roles can tell which implementation state and environments the judgment applies to.
- Readiness ownership and conditions are explicit when the package is not simply ready.
- Coverage and evidence hotspots are visible.
- Residual risks and weak evidence are summarized honestly.
- Specification relationship is explicit when existing behavioral specifications materially constrain verification interpretation.
- Scope exclusions and deferred coverage are visible and are not misrepresented as completed verification.
- Reverification triggers are explicit enough that downstream roles can tell when the current verification judgment has gone stale.
- Recommended downstream consumers are appropriate.
- Next decision points are clear without turning the artifact into a release runbook or bug tracker.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if a downstream reader can understand what has been verified, how strong the support is, and what still matters next without reopening the entire verification chain.

## Evidence Required

- The verification handoff artifact.
- The verification strategy, verification matrix, and evidence review artifacts.
- Any implementation, planning, architecture, or requirements handoffs needed to verify that the downstream framing is honest.

If the handoff overstates confidence or hides major evidence limits, fail the check.

## Supporting Skills

- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the downstream framing or packaging needs strengthening.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when supporting context still needs synthesis before the handoff can be stated cleanly.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when specification-sensitive verification issues still need to be explained accurately in the handoff.

## Failure Response

- Rework the verification handoff artifact before expecting downstream roles to rely on it.
- If the handoff is weak because upstream evidence or readiness is weak, rework the earliest source artifact rather than patching the handoff summary alone.

## Notes

This is the packaging-quality gate for the QA / Verification Lead chain.

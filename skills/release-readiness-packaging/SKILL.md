---
name: release-readiness-packaging
description: Package reviewed implementation, review, and verification outputs into a durable release candidate and release-readiness package; use when release or downstream adoption needs explicit candidate framing, release posture, approval-limit clarity, rollout caveats, and re-approval triggers without drifting into deployment execution, QA authority, or engineering ownership.
---

# Release Readiness Packaging

Turn reviewed implementation, review, verification, and operational context into a concrete Release / Handoff Manager package that downstream operators, adopters, and approvers can use without reconstructing rollout intent or current release posture from scattered artifacts.

For this repository, this is the preferred direct-support skill for the Release / Handoff Manager role when the work needs explicit release candidate and release-readiness discipline rather than only general handoff packaging.

## Quick start

1. Read the reviewed implementation, review, and verification packages together with the operational context.
2. Define the actual release candidate boundary and downstream target.
3. Make the current release posture explicit, including approval limits and condition owners.
4. Preserve rollout, monitoring, rollback, and communication caveats clearly.
5. Make re-review or re-approval triggers explicit so downstream consumers know when the current package no longer applies.

## Workflow

### 1) Read the release context

- Start with the implementation package, code review package, verification package, and relevant operational or environment references.
- Pull in requirements, architecture, planning, specifications, release notes, communication drafts, or approval notes only as needed to understand the candidate honestly.
- Keep candidate limits explicit instead of silently expanding release scope.

### 2) Frame the release candidate clearly

- State what release, rollout wave, environment set, or downstream adoption target is actually in scope.
- Distinguish included scope, excluded scope, and explicitly deferred scope.
- Preserve release-sensitive hotspots such as migrations, trust boundaries, environment assumptions, or communication-sensitive changes.

### 3) Author the release-readiness posture cleanly

For the current candidate, capture:

- the release or adoption target
- the reviewed evidence and upstream packages that currently support release consideration
- whether the candidate is ready for downstream release handling, ready with conditions, or blocked
- any scope, environment, evidence, or approval limits on that posture
- whether further operator, environment-owner, or change-approval authorization is still required before actual deployment
- the required follow-up, conditions, and owners
- residual risks and open questions

### 4) Preserve rollout and operational caveats

- Record environment assumptions, protection rules, sequencing or staged rollout notes, monitoring watchouts, rollback or escalation triggers, and communication needs.
- Keep operational unknowns explicit rather than smoothing them into generic readiness language.
- Preserve trust-boundary-sensitive and human-control-point-sensitive rollout concerns when relevant.

### 5) Make re-review and re-approval conditions explicit

- State what changes, evidence updates, approval events, environment changes, or scope expansions should trigger another release-readiness pass.
- Do not let "ready" language silently extend beyond the reviewed candidate, current approval state, or intended downstream target.
- Preserve separation between release packaging, deployment execution, and final operational control.

## Guardrails

- Do not turn release packaging into deployment execution or incident command.
- Do not override blocked review or verification posture through optimistic packaging language.
- Do not present release packaging as blanket deploy-now authority when further approvals or environment controls are still required.
- Do not hide weak evidence, blocked approvals, or operational unknowns behind confident release notes.
- Do not let candidate scope or rollout assumptions stay implicit.

## Supporting skills

- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the release context or operational evidence is spread across multiple local sources.
- Use [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is downstream packaging clarity after the release posture is already clear.
- Use [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when release framing depends heavily on validated behavior or acceptance commitments.
- Use [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when release posture depends on targeted validation or demonstration evidence.
- Use Allium skills only when governing behavioral specifications materially constrain the release package or drift checking.

## Outputs

This skill should strengthen or help populate:

- [`release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md)
- [`release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md)
- [`rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md)
- the packaging-related portions of [`release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md)

## Notes

- This skill is intentionally narrow. It supports Release / Handoff Manager packaging work after implementation, review, and verification already exist.
- It is not a deployment skill, not a QA replacement, not a code-review replacement, and not a release-approval override.

---
name: review-findings-authoring
description: Author durable Code Reviewer findings from an implementation package; use when independent review needs concrete findings with severity, confidence, affected-location discipline, evidence basis, blocking versus non-blocking distinction, and explicit re-review triggers without drifting into implementation ownership, QA authority, or release approval.
---

# Review Findings Authoring

Turn a reviewed implementation package plus local code and evidence context into a concrete Code Reviewer findings package that downstream implementation, verification, and approval consumers can act on without reconstructing the review from inline comments.

For this repository, this is the preferred direct-support skill for the Code Reviewer role when the work needs explicit findings discipline rather than only general synthesis.

## Quick start

1. Read the implementation package and clarify the review boundary.
2. Identify concrete review-worthy concerns rather than broad commentary.
3. Record each material finding with severity, confidence, affected location, and evidence basis.
4. Distinguish blocking, non-blocking, evidence-gap, and upstream-routing findings clearly.
5. Make re-review triggers explicit so downstream roles know when the current review posture no longer applies.

## Workflow

### 1) Read the review context

- Start with the implementation record, implementation evidence, implementation readiness review, and implementation package handoff.
- Pull in requirements, architecture, planning, specifications, diffs, logs, screenshots, or reproduction notes only as needed to understand the reviewed slice honestly.
- Keep review limits explicit instead of silently expanding the review boundary.

### 2) Decide what is actually finding-worthy

- Prefer concrete correctness, regression, drift, validation, or risk concerns.
- Avoid turning style preferences or implementation alternatives into findings unless they materially affect correctness, maintainability, safety, or downstream confidence.
- If the concern is real but under-evidenced, treat it as a lower-confidence or evidence-gap finding rather than overclaiming it.

### 3) Author each material finding cleanly

For each material finding, capture:

- a short finding title
- whether it is blocking, non-blocking, advisory, or an open question
- severity and confidence
- affected location detail, such as file, module, interface, endpoint, query, prompt section, config key, or line range when practical
- why it matters
- the evidence basis, such as code inspection, reproduction notes, logs, tests, screenshots, or specification comparison
- the upstream requirement, architecture, planning, evidence, or specification anchor it touches
- the requested remediation, clarification, or additional evidence

### 4) Separate finding classes explicitly

- Keep blocking findings separate from non-blocking concerns.
- Keep evidence-gap findings distinct from direct code-defect findings.
- Keep upstream-routing findings distinct from local implementation defects.
- Keep residual risk notes separate from concrete findings when the concern is broader than one localized issue.

### 5) Make re-review conditions explicit

- State what changes, clarifications, evidence updates, or scope expansions should trigger another review pass.
- Do not let approval language silently extend beyond the reviewed revision or boundary.
- Preserve review independence from implementation, QA, and release authority.

## Guardrails

- Do not rewrite the code as the main output of the review.
- Do not present code review as though it were full verification or release approval.
- Do not hide weak evidence behind high-confidence wording.
- Do not leave findings so vague that downstream consumers must rediscover them from comments or diffs.
- Do not treat no findings as blanket assurance without an explicit coverage note.

## Supporting skills

- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the review context or evidence is spread across multiple local sources.
- Use [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when findings depend heavily on requirement conformance or acceptance commitments.
- Use [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when a suspected finding depends on targeted browser or application reproduction.
- Use [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) only after the findings are already solid and the main weakness is downstream packaging clarity.
- Use Allium skills only when governing behavioral specifications materially constrain the review or drift checking.

## Outputs

This skill should strengthen or help populate:

- [`code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md)
- [`review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- the findings-related portions of [`review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md)

## Notes

- This skill is intentionally narrow. It supports Code Reviewer findings authoring after implementation work already exists.
- It is not a generic "do code review" skill, not a coding skill, not a QA replacement, and not a release-approval method.

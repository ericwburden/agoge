---
id: evidence-review
kind: artifact
title: Evidence Review
version: 1
summary: Capture the durable review record for a drafted verification package, including
  the evidence reviewed, the findings that matter, the current readiness state, and
  the remediation needed before downstream use.
template: true
default_output_path: docs/verification/evidence-review.md
checks:
- evidence-review
- verification-traceability
- qa-verification-boundary
---

# Evidence Review

## Purpose

Capture the durable review record for a drafted verification package, including the evidence reviewed, the findings that matter, the current readiness state, and the remediation needed before downstream use.

Use this artifact after the first verification pass and before producing the downstream verification handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live QA / Verification Lead work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the verification package is ready to support downstream decisions, what evidence was actually reviewed, what findings still matter, what remediation is required, and which residual risks or unanswered questions still constrain readiness.

## Related Checks

- Primary: [`evidence-review.check.md`](D:/Projects/agoge/checks/evidence-review.check.md)
- Cross-cutting: [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
- Cross-cutting: [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)

## Review Scope

State which verification artifacts, implementation evidence, and verification concerns are in scope for this review.

## Reviewed Inputs

Reference the verification strategy, verification matrix, reviewed planning artifacts, implementation handoff, implementation evidence, upstream requirements or architecture artifacts, and supporting notes used in this review.

## Evidence Provenance

For each major evidence source reviewed, capture:

- the implementation revision, commit, branch, build, or artifact version the evidence applies to
- the environment or test context where the evidence was produced
- the run identifier, timestamp, or date window when the evidence was captured
- any freshness, representativeness, or reuse limits that affect how strongly the evidence should be trusted

## Overall Assessment

Summarize the current verification quality in plain language, including the strongest evidence and the most material confidence gaps.

## Readiness Or Approval Status

State whether the verification package is:

- ready for downstream use
- ready with conditions
- blocked pending remediation

If relevant, note who or what must confirm the next step.

## Decision Owner Or Approver

Identify who owns the readiness decision or approval for this verification package.

If the status is `ready with conditions` or `blocked pending remediation`, this section should be explicit rather than implied.

## Key Findings

Record the most important findings, grouped by scope area, evidence type, or defect pattern.

## Evidence Strength And Gaps

Call out which evidence is strong enough to support confidence, which is only partial, and where critical evidence is missing or contradictory.

## Requirement, Architecture, And Planning Observations

Call out the requirement, architecture, interface, planning, or implementation assumptions that deserve special attention before downstream work continues.

## Unresolved Risks And Questions

List unresolved verification risks, open questions, or uncertainties that still materially affect readiness.

## Required Remediation

Describe the changes needed before the verification package should be treated as downstream-ready, and point back to the earliest artifact, evidence source, or workflow stage that should be reworked.

## Condition Owners

If the verification package is conditionally ready or blocked, identify who owns each required follow-up, clarification, approval, or evidence-generation condition.

## Recommended Next Step

Describe the immediate next verification or remediation step, such as evidence collection, upstream rework, targeted implementation fix, or downstream handoff.

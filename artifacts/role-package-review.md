---
id: role-package-review
kind: artifact
title: Role Package Review
version: 1
summary: Capture the coherence review of a reusable role package before it is treated
  as ready for adoption or reuse.
template: true
default_output_path: docs/orpheum/role-package-review.md
checks:
- role-package-review
- role-package-traceability
- role-builder-boundary
---

# Role Package Review

## Purpose

Capture the coherence review of a reusable role package before it is treated as ready for adoption or reuse.

Use this artifact to record what was reviewed, what failed, what was remediated, and whether the role package is ready for downstream use.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live role-design work.

## Completion Guidance

This artifact is complete when the role definition, support system, workflows, checks, and supporting skills have been reviewed together and the package's readiness or remaining gaps are explicit.

## Related Checks

- Primary: [`role-package-review.check.md`](D:/Projects/agoge/checks/role-package-review.check.md)
- Cross-cutting: [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md)
- Cross-cutting: [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md)

## Role In Scope

Name the role package being reviewed.

## Reviewed Inputs

List the role definition, support-system design, workflows, checks, and other materials included in this review.

## Review Findings

Record the major findings about coherence, gaps, duplication, missing validation, or role drift.

## Remediation Decisions

Describe what was fixed, what should be fixed next, and what was intentionally deferred.

## Readiness Assessment

State whether the role package is ready, conditionally ready, or not ready, and why.

## Remaining Risks

List the risks or weak areas that remain after review.

## Recommended Next Steps

List the next improvements or adoption steps.

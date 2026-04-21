---
id: verification-matrix
kind: artifact
title: Verification Matrix
version: 1
summary: Capture the traceable map from requirements, architecture, and implementation-plan
  commitments to the evidence expected or observed for each verification area.
template: true
default_output_path: docs/verification/verification-matrix.md
checks:
- verification-matrix
- verification-traceability
- qa-verification-boundary
---

# Verification Matrix

## Purpose

Capture the traceable map from requirements, architecture, and implementation-plan commitments to the evidence expected or observed for each verification area.

Use this artifact to make coverage, gaps, and weak links visible so downstream roles do not have to infer what has or has not been verified.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live QA / Verification Lead work.

## Completion Guidance

This artifact is complete when a downstream reader can understand which requirements, architectural commitments, interfaces, and implementation slices are covered, what evidence supports them, where coverage is missing or weak, and which gaps still matter most.

## Related Checks

- Primary: [`verification-matrix.check.md`](D:/Projects/agoge/checks/verification-matrix.check.md)
- Cross-cutting: [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
- Cross-cutting: [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)

## Matrix Scope

State what verification scope, release slice, or artifact chain this matrix covers.

## Source Inputs

Reference the requirements, architecture, planning, implementation, and evidence sources used to build the matrix.

## Coverage Map

For each major requirement, architectural commitment, interface hotspot, implementation slice, or specification-sensitive behavior in scope, capture:

- the source item being verified
- why it matters
- the expected evidence type or verification method
- the evidence actually reviewed, if available
- the current coverage state: covered, partially covered, not yet covered, or blocked
- the gate impact: blocking, conditional, or informational
- the main gap, limitation, or contradiction if coverage is weak
- who should act next if the gap is not merely informational

## Hotspot Summary

Summarize the areas with the weakest evidence, highest residual risk, or most concentrated downstream impact.

## Contradictions And Weak Signals

List places where the evidence conflicts, where results are ambiguous, or where apparent coverage is too weak to support a strong readiness judgment.

## Deferred Coverage

List the verification areas intentionally left for later phases, later environments, or later approvals.

## Upstream Routing Notes

Identify any gaps that should route back to requirements, architecture, planning, implementation, or specification work instead of being treated as downstream verification cleanup.

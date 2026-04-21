---
id: business-analyst-quality-review
kind: workflow
title: Business Analyst Quality Review
version: 1
summary: Apply the Business Analyst check chain to instantiated BA artifacts, identify
  failures by artifact and defect type, and route remediation before downstream handoff
  or Allium promotion.
role: business-analyst
inputs: []
outputs: []
skills:
- requirements-verification
- handoff-packaging
- research-documentation
- meeting-intelligence
- meeting-notes-and-actions
checks:
- traceability
- business-analyst-boundary
- business-objectives
- process-analysis
- requirements-specification
- requirements-handoff
handoff_to: []
---

# Business Analyst Quality Review

## Purpose

Apply the Business Analyst check chain to instantiated BA artifacts, identify failures by artifact and defect type, and route remediation before downstream handoff or Allium promotion.

## When To Use

- A BA artifact or artifact chain has been drafted and needs definition-of-done validation.
- A downstream consumer needs confidence that BA outputs are self-sufficient and within role boundaries.
- A check has failed and the work needs a standard rework path instead of ad hoc editing.

## Inputs

- Required: one or more instantiated BA artifacts derived from the templates in [`artifacts/`](D:/Projects/agoge/artifacts)
- Optional: supporting notes, transcripts, research outputs, policy references, or prior check results

## Outputs

- Primary output: a pass/fail review result for each artifact in scope and each required cross-cutting check
- Secondary outputs: explicit remediation routing, identified defect types, and rework priorities

## Skills And Tools

- Use the primary artifact-specific skill when the relevant primary check fails.
- Use [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when traceability breaks at the requirement layer or unsupported requirements are present.
- Use [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the downstream handoff is incomplete or poorly packaged.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when missing evidence or contradictory local context prevents a clean pass.
- Use [`meeting-intelligence`](D:/Projects/agoge/skills/meeting-intelligence/SKILL.md) or [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when the defect originates in weak kickoff framing or unsynthesized source notes.

## Sequence

1. Determine which instantiated BA artifacts are in scope.
2. Run the primary check for each artifact in scope.
3. Run [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md) when enough of the BA chain exists to evaluate linkage across artifacts.
4. Run [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) across all artifacts in scope.
5. Record failures by artifact and defect type: missing evidence, weak framing, process ambiguity, unsupported requirements, broken traceability, missing specification relationship, missing confirmation status, or role drift.
6. Route remediation to the appropriate BA skill or earlier BA workflow stage rather than patching the latest artifact in isolation.
7. Re-run the failed checks until the artifact chain passes or until the remaining gaps are explicitly recorded as unresolved and the work is intentionally held open.

## Required Check Set

- Business objectives review:
  - [`business-objectives.check.md`](D:/Projects/agoge/checks/business-objectives.check.md)
  - [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md) when downstream artifacts exist
  - [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)
- Process analysis review:
  - [`process-analysis.check.md`](D:/Projects/agoge/checks/process-analysis.check.md)
  - [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
  - [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)
- Requirements specification review:
  - [`requirements-specification.check.md`](D:/Projects/agoge/checks/requirements-specification.check.md)
  - [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
  - [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)
- Requirements handoff review:
  - [`requirements-handoff.check.md`](D:/Projects/agoge/checks/requirements-handoff.check.md)
  - [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
  - [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)

## Validation

- Every artifact in scope has an explicit pass/fail result.
- Cross-cutting failures are tied to the earliest artifact that introduced the defect.
- Remediation is routed to the correct BA skill or prior workflow stage.
- No artifact is treated as downstream-ready while any required check is still failing.

## Failure Handling

- If evidence is insufficient to apply a check honestly, fail the check and identify the missing evidence rather than guessing.
- If the same failure appears in multiple downstream artifacts, rework the earliest source artifact instead of patching each symptom separately.
- If the artifact chain passes the primary checks but fails traceability or boundary checks, treat the chain as blocked until the cross-cutting defect is resolved.

## Notes

This workflow is intentionally lightweight. It is a quality gate and rework loop, not a second full BA lifecycle.

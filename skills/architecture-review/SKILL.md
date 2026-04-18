---
name: architecture-review
description: Review solution architecture, architecture decisions, and related evidence for coherence, risk, interface clarity, and downstream readiness; use when producing a durable architecture review in a local Markdown workflow.
---

# Architecture Review

Review drafted architecture artifacts, make readiness explicit, and produce a durable architecture review artifact without drifting into implementation review or delivery approval theater.

For this repository's Solution Architect workflows, this is the preferred default review path for local Markdown architecture artifacts.

## Quick start
1) Read the drafted architecture artifacts and the upstream evidence they depend on.
2) Assess whether the architecture is coherent, traceable, and explicit enough for downstream use.
3) Identify the material findings, risks, and interface or contract concerns.
4) Decide whether the architecture is ready, conditionally ready, or blocked.
5) Populate or strengthen the local architecture review artifact.

## Workflow

### 1) Gather the review inputs
- Start with the local solution architecture and architecture decisions artifacts.
- Pull in upstream BA artifacts when traceability or requirement fit is in doubt.
- Read supporting technical notes only when they materially affect the review judgment.

### 2) Assess architectural quality
- Check whether the architecture expresses:
  - clear drivers
  - explicit system boundary
  - understandable component responsibilities
  - explicit interface seams and contract assumptions where they matter
  - visible tradeoffs, risks, and unresolved questions
- Fail the review if these elements cannot be judged honestly from the available artifacts.

### 3) Classify findings
- Separate strengths, material concerns, blocked issues, and unresolved decisions.
- Distinguish architecture defects from upstream requirement ambiguity.
- Route defects to the earliest artifact that should be reworked.

### 4) Decide readiness
- Use one of:
  - ready for downstream handoff
  - ready with conditions
  - blocked pending remediation
- Do not convert uncertainty into implied approval.

### 5) Produce the review artifact
- Populate or improve the local architecture review artifact.
- Include the review scope, inputs, overall assessment, readiness status, key findings, interface or contract observations, unresolved issues, remediation, and the recommended next step.

## Notes

- This skill is for architecture review, not code review.
- It should not silently redesign the architecture while pretending to review it.
- It is generic and reusable, but it is especially useful in this repository's Solution Architect review and quality workflows.

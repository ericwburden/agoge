---
name: requirements-verification
description: Verify requirements against available business objectives, process analysis, and supporting evidence; use when separating confirmed requirements from non-requirements, assumptions, and open questions in a local Markdown workflow.
metadata:
  short-description: Verify requirements against source evidence
---

# Requirements Verification

Verify which requirements are actually supported by the available discovery evidence and strengthen the requirements specification artifact without drifting into implementation planning.

For this repository's Business Analyst workflows, this is the preferred default verification path for local Markdown artifacts and check remediation.

## Quick start
1) Read the available source artifacts and supporting evidence.
2) Identify candidate requirements and test whether each one is supported.
3) Separate verified requirements from non-requirements, assumptions, and open questions.
4) Check requirement quality for clarity, scope, traceability, and verifiability.
5) Populate or strengthen the local requirements specification artifact.

## Workflow

### 1) Gather source evidence
- Read the local business objective and process analysis artifacts first.
- Pull in supporting notes, research summaries, policies, or stakeholder clarifications only as needed.
- Prefer evidence already captured in local Markdown files.

### 2) Identify candidate requirements
- Extract statements that appear to describe expected business behavior, constraints, or outcomes.
- Do not assume every requested feature or proposed solution is a confirmed requirement.

### 3) Classify each candidate
- **Verified requirement**: clearly supported by business objectives, process needs, or validated source evidence.
- **Non-requirement**: a proposed solution, preference, idea, or out-of-scope request that is not yet confirmed.
- **Assumption**: something being relied on but not yet verified.
- **Open question**: a gap whose answer materially affects the requirement set.

### 4) Test requirement quality
- Check whether each verified requirement is:
  - clear
  - scoped
  - traceable to a business need or process need
  - testable or verifiable
  - separated from implementation detail unless the constraint is explicitly part of the requirement
- Flag contradictions, unsupported requirements, or solution drift instead of smoothing them over.

### 5) Produce the strengthened output
- Populate or improve the local requirements specification artifact.
- Ensure the artifact includes:
  - verified requirements
  - non-requirements
  - assumptions
  - open questions
  - constraints
  - acceptance considerations
- If appropriate, note candidate behaviors or business rules that may be ready for Allium later, but do not make that the default output.

## Notes

- This skill stops at requirement verification and specification quality.
- It should not create implementation plans, tasks, or delivery coordination outputs.
- It is generic and reusable, but it is especially useful in the Business Analyst requirements handoff workflow.

---
id: business-analyst-process-analysis
kind: workflow
title: Business Analyst Process Analysis
version: 1
summary: Turn discovery outputs into a clear analysis of current-state and future-state
  process behavior, including rules, participants, exceptions, and operational gaps.
role: business-analyst
inputs:
- business-objectives
outputs:
- process-analysis
skills:
- meeting-notes-and-actions
- research-documentation
checks:
- process-analysis
- traceability
- business-analyst-boundary
handoff_to: []
---

# Business Analyst Process Analysis

## Purpose

Turn discovery outputs into a clear analysis of current-state and future-state process behavior, including rules, participants, exceptions, and operational gaps.

## When To Use

- The business objective is understood well enough to analyze how work currently flows and how it should change.
- Interviews, notes, or process-session transcripts exist and need to be converted into a reusable process artifact.
- The project depends on understanding actors, triggers, inputs, outputs, rules, or edge cases before writing requirements.

## Inputs

- Required: an instantiated copy of [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
- Optional: interview notes, process workshop transcripts, operational documents, policies, SOPs, and existing system notes.

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md) in the target project workspace
- Secondary outputs: clarified business rules, exception paths, gap statements, dependencies, and human oversight notes when relevant.

## Skills And Tools

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) for process-session transcripts or interview notes.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) for synthesizing policy documents, SOPs, and multi-source process references.
- Local Markdown artifacts in [`artifacts/`](D:/Projects/agoge/artifacts) as the canonical output format.

## Sequence

1. Read the instantiated business objectives artifact and confirm the process being analyzed is actually in scope for the business objective.
2. Normalize any interview notes or transcripts with `meeting-notes-and-actions` so actors, decisions, risks, and ambiguities are explicit.
3. If current process knowledge is spread across multiple documents, use `research-documentation` to synthesize the relevant sources before drafting.
4. Instantiate [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md) into the project workspace if a working copy does not already exist.
5. Populate the current-state section of the instantiated process analysis artifact, including actors, triggers, inputs, outputs, and pain points.
6. Populate the future-state section, capturing intended changes, required outcomes, and the process conditions needed to support the business objective.
7. Record the explicit process needs that later requirements should trace back to rather than leaving them implied inside the broader process narrative.
8. Record business rules, exceptions, gaps, dependencies, and human oversight notes where the process involves AI-enabled or agentic behavior.
9. Run [`process-analysis.check.md`](D:/Projects/agoge/checks/process-analysis.check.md), [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md), and [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) against the instantiated artifact and the existing BA chain.

## Decision Points

- If the current-state process is unknown, do not infer it from the desired future state; record the gap and ask for clarification.
- If multiple process variants exist, capture them explicitly instead of flattening them into one path.
- If policy or rule conflicts appear, preserve them as unresolved questions or competing constraints.
- If process needs are only implied by the narrative, restate them explicitly before treating the artifact as ready for requirements work.
- If AI-enabled steps appear in the future-state process, record human review, approval, intervention, or escalation expectations.

## Validation

- [`process-analysis.check.md`](D:/Projects/agoge/checks/process-analysis.check.md) passes.
- [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md) passes.
- [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) passes.
- The instantiated output is ready to feed [`business-analyst-requirements-handoff.md`](D:/Projects/agoge/workflows/business-analyst-requirements-handoff.md).

## Failure Handling

- Stop and ask for clarification if the process cannot be described without inventing steps or actors.
- Do not rewrite process ambiguity into false certainty; record open questions instead.
- If the future-state process is still aspirational and underspecified, describe the intended outcomes and unresolved design areas rather than pretending the process is settled.
- If the primary check fails, rework transcripts or interview notes with `meeting-notes-and-actions` or synthesize missing operational context with `research-documentation`.
- If a traceability or boundary check fails, route remediation to the earlier BA artifact or process section that introduced the break instead of patching the handoff stage later.

## Notes

This workflow is process-heavy by design. It should strengthen requirement quality by grounding them in actual operational behavior.

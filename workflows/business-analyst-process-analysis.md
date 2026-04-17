# Business Analyst Process Analysis

## Purpose

Turn discovery outputs into a clear analysis of current-state and future-state process behavior, including rules, participants, exceptions, and operational gaps.

## When To Use

- The business objective is understood well enough to analyze how work currently flows and how it should change.
- Interviews, notes, or process-session transcripts exist and need to be converted into a reusable process artifact.
- The project depends on understanding actors, triggers, inputs, outputs, rules, or edge cases before writing requirements.

## Inputs

- Required: [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
- Optional: interview notes, process workshop transcripts, operational documents, policies, SOPs, and existing system notes.

## Outputs

- Primary artifact: [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md)
- Secondary outputs: clarified business rules, exception paths, gap statements, dependencies, and human oversight notes when relevant.

## Skills And Tools

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) for process-session transcripts or interview notes.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) for synthesizing policy documents, SOPs, and multi-source process references.
- Local Markdown artifacts in [`artifacts/`](D:/Projects/agoge/artifacts) as the canonical output format.

## Sequence

1. Read [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md) and confirm the process being analyzed is actually in scope for the business objective.
2. Normalize any interview notes or transcripts with `meeting-notes-and-actions` so actors, decisions, risks, and ambiguities are explicit.
3. If current process knowledge is spread across multiple documents, use `research-documentation` to synthesize the relevant sources before drafting.
4. Populate the current-state section of [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md), including actors, triggers, inputs, outputs, and pain points.
5. Populate the future-state section, capturing intended changes, required outcomes, and the process conditions needed to support the business objective.
6. Record business rules, exceptions, gaps, dependencies, and human oversight notes where the process involves AI-enabled or agentic behavior.

## Decision Points

- If the current-state process is unknown, do not infer it from the desired future state; record the gap and ask for clarification.
- If multiple process variants exist, capture them explicitly instead of flattening them into one path.
- If policy or rule conflicts appear, preserve them as unresolved questions or competing constraints.
- If AI-enabled steps appear in the future-state process, record human review, approval, intervention, or escalation expectations.

## Validation

- The current-state and future-state process can each be read as coherent flows.
- Actors, triggers, inputs, outputs, business rules, and exceptions are represented clearly.
- The gap between current-state and future-state is explicit.
- Dependencies and oversight expectations are visible where they matter.
- The output is ready to feed [`business-analyst-requirements-handoff.md`](D:/Projects/agoge/workflows/business-analyst-requirements-handoff.md).

## Failure Handling

- Stop and ask for clarification if the process cannot be described without inventing steps or actors.
- Do not rewrite process ambiguity into false certainty; record open questions instead.
- If the future-state process is still aspirational and underspecified, describe the intended outcomes and unresolved design areas rather than pretending the process is settled.

## Notes

This workflow is process-heavy by design. It should strengthen requirement quality by grounding them in actual operational behavior.

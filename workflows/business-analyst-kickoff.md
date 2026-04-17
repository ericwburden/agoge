# Business Analyst Kickoff

## Purpose

Turn a project kickoff prompt, rough notes, transcript, or prior documentation into a clear discovery baseline centered on the business problem and desired outcome.

## When To Use

- A project is starting and the business problem is not yet crisply defined.
- Stakeholders have offered goals, features, or concerns, but they are not yet organized into a usable discovery artifact.
- Prior notes or transcripts exist and need to be normalized into a reusable kickoff artifact.

## Inputs

- Required: kickoff request, notes, transcript, or existing project context.
- Optional: prior documentation, stakeholder lists, meeting agendas, research sources, and existing local Markdown files.

## Outputs

- Primary artifact: [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
- Secondary outputs: structured notes, clarified scope boundaries, assumptions, and open questions.

## Skills And Tools

- [`meeting-intelligence`](D:/Projects/agoge/skills/meeting-intelligence/SKILL.md) for structuring discovery meetings, agendas, and pre-reads.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when raw notes or transcripts need to be normalized.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when prior documents or external context need synthesis.
- Local Markdown files in [`artifacts/`](D:/Projects/agoge/artifacts) and project notes as the default storage/output model.

## Sequence

1. Gather the starting material: kickoff request, notes, transcript, or prior docs, and identify whether the work begins from a live discovery need or existing artifacts.
2. If a discovery meeting needs to be prepared, use `meeting-intelligence` to structure the meeting goal, decision needs, agenda, and pre-read context.
3. If rough notes or a transcript already exist, use `meeting-notes-and-actions` to extract the business problem, decisions, risks, action items, and ambiguities.
4. If multiple existing documents or research sources are involved, use `research-documentation` to synthesize the relevant context into a coherent discovery summary.
5. Populate [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md), keeping business objectives, scope boundaries, assumptions, proposed solutions, and open questions clearly separated.
6. Confirm that the resulting artifact is sufficient to support process analysis without inventing missing business facts.

## Decision Points

- If only a prompt exists, structure discovery around open questions and stakeholder gaps before drafting conclusions.
- If a transcript exists, normalize it first rather than drafting the artifact directly from noisy raw material.
- If prior docs conflict, preserve the contradiction as an open question instead of reconciling it silently.
- If the project includes AI-enabled or agentic behavior, record business-facing constraints separately from AI or agent-specific constraints.

## Validation

- The business problem, business objectives, success criteria, and scope boundaries are explicit.
- Stakeholders are identified with their relationship to the problem or desired outcome.
- Proposed solutions are recorded separately from the underlying business need.
- Assumptions and open questions are visible rather than implied.
- The output is ready to feed [`business-analyst-process-analysis.md`](D:/Projects/agoge/workflows/business-analyst-process-analysis.md).

## Failure Handling

- Stop and ask for more information if the business problem cannot be stated without inventing facts.
- Do not collapse feature requests into confirmed objectives when the underlying need is still unclear.
- If no meaningful context exists, produce a discovery-ready skeleton with explicit gaps instead of a false summary.

## Notes

This is the default entry workflow for Business Analyst work. Prefer local Markdown artifacts even when external systems exist.

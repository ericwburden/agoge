# Process Analysis Check

## Purpose

Validate that the process analysis artifact accurately captures current-state and future-state process understanding, including the operational details needed for strong requirements work.

## Applies To

- instantiated copies of [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md)
- Use after process analysis work has been drafted from discovery, interviews, or process-session notes.
- Do not use as a replacement for business-objective framing or requirement verification.

## Criteria

- The process in scope is identified explicitly.
- Current-state and future-state process descriptions are both present.
- Actors, triggers, inputs, and outputs are described clearly enough to follow the process.
- Business rules are recorded.
- Exceptions, edge cases, or escalation paths are recorded when known.
- Gaps between current-state and future-state are explicit.
- Process ambiguity is surfaced as an open question rather than invented away.
- Human oversight notes are present when AI-enabled or agentic behavior is relevant to the process.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if the process can be understood without inferring missing operational structure from chat context.

## Evidence Required

- The process analysis artifact.
- Supporting interview notes, transcripts, policies, SOPs, or other process references when available.

If process evidence is weak or contradictory, fail the check and identify the unresolved areas.

## Supporting Skills

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when process evidence is still embedded in transcripts or rough notes.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when policies, SOPs, or other local sources need to be synthesized into a coherent process view.

## Failure Response

- Rework the process analysis artifact before using it as the basis for verified requirements.
- Preserve uncertainty explicitly instead of flattening multiple process variants into one invented flow.

## Notes

This check ensures that requirements are grounded in actual business flow rather than abstract solution ideas.

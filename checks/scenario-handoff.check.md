# Scenario Handoff Check

## Purpose

Validate that the scenario handoff artifact packages the scenario in a way that downstream adopters, workflow authors, or execution-planning consumers can use without reconstructing the multi-role logic from earlier artifacts.

## Applies To

- Instantiated copies of [`scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md)
- Use after scenario review and before treating the scenario as ready for downstream adoption or execution-planning use

## Criteria

- The current scenario summary is explicit.
- The included scenario package is identified clearly.
- Current readiness posture is explicit and does not overstate what the package authorizes.
- Role and workflow routing guidance is clear enough for the next consumer to act.
- Entry conditions for the next consumer are explicit.
- Active conditions, risks, and watchouts are visible.
- Follow-up owners and revisit triggers are identified.
- The recommended next consumer is explicit.
- The handoff does not collapse into a live project plan, staffing board, or deployment authorization.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated scenario handoff artifact
- The scenario definition, scenario integration map, and scenario review artifacts it packages

## Supporting Skills

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) when the main weakness is still scenario-routing clarity, readiness-limit wording, or revisit-trigger discipline before packaging.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the scenario package needs stronger downstream packaging clarity.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the context feeding the handoff is still fragmented.

## Failure Response

- Rework the scenario handoff before using it as the downstream scenario package.
- Route upstream defects back to the earliest scenario artifact or role package that introduced them.

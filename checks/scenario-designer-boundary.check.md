# Scenario Designer Boundary Check

## Purpose

Validate that Scenario Designer outputs stay inside reusable scenario composition, integration mapping, review, and downstream handoff role boundaries and do not drift into role-definition ownership, project management, architecture design, or project-instance execution control.

## Applies To

- All instantiated Scenario Designer artifacts and Scenario Designer workflow outputs
- Use whenever reviewing whether scenario outputs stayed within the intended role

## Criteria

- The output does not redefine role-local responsibilities that belong in Role Builder or the owning role package.
- The output does not collapse into project management, sprint administration, staffing, or status reporting.
- The output does not design the technical solution or implementation approach when those issues belong in role-local architecture or planning work.
- The output does not imply that a reusable scenario package is already a live project plan or execution authorization.
- The output remains focused on scenario intent, participation, sequencing, handoffs, integration requirements, readiness, and downstream scenario packaging.
- Missing role support, unresolved approvals, and weak assumptions are explicit without being overstated as solved.
- Human checkpoints and trust-boundary-sensitive transitions are identified when relevant without drifting into governance command or operational control.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The Scenario Designer artifact or workflow output being reviewed
- The [`Scenario Designer`](D:/Projects/orpheum/roles/scenario-designer.md) role definition when needed for role-boundary interpretation
- Relevant role packages when needed to identify where drift began

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when role drift is being caused by unsynthesized local context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when drift appears in downstream packaging.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when drift began because a public orchestration pattern was imported too literally.

## Failure Response

- Rework the output to remove role drift before treating it as valid Scenario Designer work.
- Route role-definition, architecture, planning, execution, or operational ambiguity back to the correct role rather than leaving it embedded in scenario output.

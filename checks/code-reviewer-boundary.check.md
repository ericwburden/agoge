# Code Reviewer Boundary Check

## Purpose

Validate that Code Reviewer outputs stay inside independent review role boundaries and do not drift into re-implementation, formal QA ownership, release operations, or upstream role redefinition.

## Applies To

- All instantiated Code Reviewer artifacts and Code Reviewer workflows
- Use whenever reviewing whether review output stayed within the intended role
- Do not use as a substitute for findings quality, traceability, or decision clarity checks

## Criteria

- The output does not rewrite or implement the change instead of reviewing it.
- The output does not redefine business objectives or requirements without routing the issue upstream.
- The output does not silently re-architect the solution when the issue belongs in architecture.
- The output does not silently replace the reviewed implementation strategy with a new plan when the issue belongs in Technical Planner work.
- The output does not collapse into full QA authority, merge authority, release approval, or release coordination.
- The work remains focused on independent review scope, findings, decision posture, and downstream-ready review context.
- Missing validation and weak evidence are explicit without being overstated as a full verification verdict.
- Approval language stays scoped to the reviewed revision and review boundary rather than implying permanent or blanket approval.
- Human control points and trust-boundary-sensitive concerns are identified when relevant without drifting into governance or release ownership.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably Code Reviewer work rather than a disguised implementation patch, QA package, release checklist, or upstream redesign.

## Evidence Required

- The Code Reviewer artifact or workflow output being reviewed
- The [`Code Reviewer`](D:/Projects/orpheum/roles/code-reviewer.md) role definition when needed for role-boundary interpretation
- Relevant implementation, requirements, architecture, or planning artifacts when needed to identify whether drift originated upstream or inside review output

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when drift is caused by unsynthesized local context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when drift appears in downstream packaging.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when drift began because missing behavioral definition should have been routed upstream.

## Failure Response

- Rework the output to remove role drift before treating it as a valid review artifact.
- Route requirement, architecture, planning, specification, or verification ambiguity back to the correct upstream or adjacent role rather than leaving it embedded in review output.

# Security / Compliance Specialist Boundary Check

## Purpose

Validate that Security / Compliance Specialist outputs stay inside security/compliance framing, control mapping, review, and downstream handoff role boundaries and do not drift into legal sign-off, architecture design, implementation ownership, QA authority, or release execution.

## Applies To

- All instantiated Security / Compliance Specialist artifacts and workflow outputs
- Use whenever reviewing whether security/compliance outputs stayed within the intended role

## Criteria

- The output does not replace formal legal counsel, external audit authority, or final compliance sign-off by default.
- The output does not design the technical solution or implement the controls as its main output.
- The output does not collapse into broad QA ownership, deployment execution, or release operations.
- The output does not treat security/compliance guidance as equivalent to blanket product, implementation, or release approval.
- The output does not treat downstream-ready security/compliance packaging as equivalent to final policy approval, deployment approval, or operational authorization.
- The output remains focused on in-scope risks, obligations, controls, evidence expectations, review posture, and downstream decision framing.
- Missing evidence, unresolved obligations, waivers, and residual risks are explicit without being overstated as solved.
- Human control points and trust-boundary-sensitive concerns are identified when relevant without drifting into governance theater or operational command.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The Security / Compliance Specialist artifact or workflow output being reviewed
- The [`Security / Compliance Specialist`](D:/Projects/orpheum/roles/security-compliance-specialist.md) role definition when needed for role-boundary interpretation
- Relevant upstream or downstream artifacts when needed to identify where drift began

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when role drift is being caused by unsynthesized local context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when drift appears in downstream packaging.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when drift began because missing behavioral definition should have been routed into specification refinement instead.

## Failure Response

- Rework the output to remove role drift before treating it as valid Security / Compliance Specialist work.
- Route legal, architecture, planning, implementation, verification, release, or policy ambiguity back to the correct role rather than leaving it embedded in security/compliance output.

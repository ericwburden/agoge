# Controls And Evidence Matrix Check

## Purpose

Validate that the controls/evidence matrix makes required safeguards, expected evidence, control owners, compensating controls, and unresolved gaps explicit enough for downstream consumers.

## Applies To

- Instantiated copies of [`controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md)
- Use before treating the current package as security-ready or compliance-ready

## Criteria

- Matrix scope is explicit.
- Required controls or obligations are identified clearly enough to guide downstream work.
- Evidence expectations are visible where they materially affect confidence or approval.
- Control owners are explicit enough to support follow-up.
- Compensating controls, exceptions, or waivers are recorded honestly.
- Material compensating controls, exceptions, or waivers include rationale, owner, and re-review or expiration posture when that detail matters.
- Unresolved gaps and re-review triggers are explicit.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated controls/evidence matrix
- The security/compliance scope artifact and supporting inputs that justify the matrix

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when control or evidence context is fragmented across local sources.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) only when the control logic is already solid and the main weakness is packaging clarity.

## Failure Response

- Rework the matrix before treating it as downstream-ready.
- Route missing control ownership, evidence, obligation clarity, or exception governance to the earliest source that can resolve it.

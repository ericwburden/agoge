# Security / Compliance Specialist Handoff

## Purpose

Package reviewed security/compliance outputs into a downstream-ready handoff that architecture, planning, implementation, verification, release, or approval consumers can use without reconstructing the risk and obligation posture from earlier artifacts.

## When To Use

- Security/compliance scope, controls/evidence matrix, and review artifacts already exist.
- A downstream role or human approver needs a durable summary of current security/compliance posture.
- The package is about to move into architecture revision, planning, implementation, verification, release, or formal approval work.

## Inputs

- Required:
  - instantiated copies of [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md), [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md), and [`artifacts/security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md)
- Expected supporting context:
  - the corresponding reviewed delivery and obligation inputs
- Optional:
  - related waiver notes, audit notes, or approval notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md) in the target project workspace
- Secondary outputs: downstream-ready risk summary, active controls and gaps, approval watchouts, and next-step routing

## Skills And Tools

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) when the handoff needs stronger control posture, exception framing, approval-limit wording, or re-review discipline before packaging.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default direct-support skill for turning reviewed security/compliance outputs into a downstream-ready handoff.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the current security/compliance context still needs synthesis before handoff.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the handoff needs stronger grounding in validated constraints or acceptance-sensitive commitments.

## Sequence

1. Read the reviewed security/compliance package together with the supporting reviewed delivery and obligation inputs.
2. If the current context is still fragmented across multiple local sources, use `research-documentation` first.
3. Instantiate [`artifacts/security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `security-controls-and-exceptions` to make the control posture, exception framing, approval limits, and re-review logic explicit, then use `handoff-packaging` to package the security/compliance handoff artifact with current scope, package contents, posture, active controls and gaps, follow-up owners, re-review triggers, upstream routing notes, recommended next consumer, and any distinction between downstream usability and final approval authority.
5. Run [`security-compliance-handoff.check.md`](D:/Projects/orpheum/checks/security-compliance-handoff.check.md), [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md), and [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).

## Decision Points

- If the handoff is primarily for architecture or planning work, preserve the risk and control intent without drifting into detailed solution design.
- If the handoff is primarily for human approval, keep the controls, waivers, and unresolved gaps visible rather than collapsing them into a soft endorsement.
- If the handoff is only fit for approval-routing, release-planning, or conditional downstream preparation, say that directly rather than implying implementation or deployment can proceed.
- If a downstream consumer needs missing evidence or stronger obligation clarity, route that gap explicitly rather than hiding it in handoff prose.

## Validation

- [`security-compliance-handoff.check.md`](D:/Projects/orpheum/checks/security-compliance-handoff.check.md) passes.
- [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md) passes.
- [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md) passes.
- The package is ready for the explicitly named downstream use and does not overstate legal, policy, deployment, or operational authorization.

## Failure Handling

- Stop and ask for clarification if the next consumer cannot be identified honestly from the available package.
- If packaging clarity is the main weakness, rework the handoff artifact rather than assuming downstream consumers will infer the missing structure.
- If the defect began earlier in the chain, route remediation there before treating the handoff as complete.

# Security / Compliance Specialist Review

## Purpose

Review the drafted security/compliance package, record the durable posture, and decide whether the package is ready, conditional, or blocked for downstream design, planning, implementation, verification, or release-adjacent use.

## When To Use

- Security/compliance scope and controls/evidence matrix artifacts already exist.
- Downstream roles need an explicit security/compliance posture rather than only scope and control notes.
- The package includes mixed evidence, unresolved obligations, waivers, or conditional decisions that should be clarified before handoff.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md)
  - an instantiated copy of [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md)
- Expected supporting context:
  - the corresponding reviewed delivery artifacts and relevant policy, vendor, regulatory, or operational references
- Optional:
  - waiver notes, audit notes, incident learnings, or approval notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md) in the target project workspace
- Secondary outputs: explicit posture, grouped risks and gaps, follow-up owners, and downstream watchouts

## Skills And Tools

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) as the preferred direct-support skill when review quality depends on explicit control posture, exception framing, approval limits, and re-review discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing obligations, evidence, and delivery context before writing the decision.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the package needs stronger grounding in validated constraints or acceptance-sensitive commitments.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when review inputs include rough decision notes, audit notes, or stakeholder meetings that need normalization first.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md), and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when the review reveals a material gap or drift in policy-sensitive behavioral definition.

## Sequence

1. Read the security/compliance scope and controls/evidence matrix together with the reviewed delivery context and any material policy, vendor, regulatory, or operational references.
2. If evidence, approvals, or obligation references still need synthesis before the decision can be stated honestly, use `research-documentation` first.
3. Instantiate [`artifacts/security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md) into the project workspace if a working copy does not already exist.
4. Use `security-controls-and-exceptions` to populate the security/compliance review artifact with review scope, reviewed inputs, overall assessment, explicit decision status, any distinction between downstream-readiness and final legal, audit, policy, deployment, or operational authorization, decision owner or required approver, key risks and gaps, required follow-up, follow-up owners, re-review triggers, and recommended next step.
5. Run [`security-compliance-review.check.md`](D:/Projects/orpheum/checks/security-compliance-review.check.md), [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md), and [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).

## Decision Points

- If the package is blocked, state the blocking condition directly rather than softening it into broad caution language.
- If the package is conditional, make the condition and owner explicit rather than leaving it implied in the matrix.
- If the issue is really unresolved discovery, architecture, implementation, verification, legal, or policy clarity rather than security packaging, preserve that distinction in the review.
- If the decision only applies to a particular environment, scope, evidence window, or approval state, state that explicitly rather than implying broader authorization.
- If the package is suitable only for downstream approval-routing or release-planning use, say that directly instead of implying it is already approved to proceed.

## Validation

- [`security-compliance-review.check.md`](D:/Projects/orpheum/checks/security-compliance-review.check.md) passes.
- [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md) passes.
- [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md) passes.
- The package is ready to feed [`security-compliance-specialist-handoff.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-handoff.md).

## Failure Handling

- Stop and ask for clarification if the posture cannot be made honestly from the available package.
- If the review check fails, rework the decision instead of asking downstream roles to infer the real posture.
- If traceability or boundary checks fail, route remediation to the earliest artifact, evidence source, or role decision that introduced the defect.

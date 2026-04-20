# Security / Compliance Specialist Scoping

## Purpose

Turn reviewed product, architecture, implementation, release, and obligation context into an explicit security/compliance scope artifact and controls/evidence matrix that downstream roles can use without reconstructing risk and obligation posture.

## When To Use

- Reviewed requirements, architecture, or delivery context already exist.
- A system or release needs explicit security/compliance framing before architecture, planning, implementation, verification, or release work proceeds.
- Security or compliance decisions need durable packaging rather than only comments or meeting notes.

## Inputs

- Required:
  - the corresponding reviewed requirements, architecture, planning, implementation, or release artifacts that define the current scope
- Expected supporting context:
  - relevant policy, contractual, regulatory, vendor, or operational references when they materially constrain the work
- Optional:
  - incident learnings, audit notes, customer assurance requests, or release learnings

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md)
  - an instantiated copy of [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md)
- Secondary outputs: explicit risk framing, obligation scope, control expectations, control ownership, waivers, and re-review triggers

## Skills And Tools

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) as the preferred direct-support skill for turning reviewed delivery and obligation context into explicit controls, evidence expectations, compensating controls, exceptions, and re-review logic.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing delivery, obligation, policy, vendor, and operational context before writing security/compliance artifacts.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the package needs stronger grounding in validated needs, commitments, or acceptance-sensitive constraints.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when the main inputs are review sessions, stakeholder meetings, audit notes, or security workshops that need normalization first.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md) when obligation framing depends materially on external platform, standards, or regulatory research that should be sourced explicitly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md), [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md), and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when mature policy-sensitive behavior should be sharpened into clearer behavioral commitments or checked for drift.

## Sequence

1. Read the reviewed delivery context together with any relevant policy, vendor, contractual, regulatory, or operational references.
2. If the context is spread across multiple local sources, use `research-documentation` first to synthesize the current system boundary, data sensitivities, obligations, and control drivers.
3. Instantiate [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md) and [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md) into the project workspace if working copies do not already exist.
4. Use `security-controls-and-exceptions` to populate the scope artifact with reviewed inputs, assets and data surfaces, trust boundaries, applicable obligations, assumptions, and open questions.
5. Use `security-controls-and-exceptions` to populate the controls/evidence matrix with required controls, evidence expectations, control owners, compensating controls, exception or waiver rationale and ownership where relevant, unresolved gaps, and re-review triggers.
6. Run [`security-compliance-scope.check.md`](D:/Projects/orpheum/checks/security-compliance-scope.check.md), [`controls-and-evidence-matrix.check.md`](D:/Projects/orpheum/checks/controls-and-evidence-matrix.check.md), [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md), and [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).

## Decision Points

- If the current choice depends on obligations or policies that are not actually identifiable, route that gap explicitly rather than disguising it as a security decision.
- If the most important downstream need is control ownership or evidence clarity, preserve that emphasis instead of collapsing everything into generic risk prose.
- If the package relies on a compensating control, exception, or waiver, make its rationale, owner, and re-review posture explicit rather than letting it read like a silent permanent approval.
- If policy-sensitive or trust-boundary-sensitive behavior is mature enough that it should become or refine a behavioral specification, use the existing Allium skills rather than burying it in control prose.
- If the issue becomes detailed solution design or implementation planning, route that work downstream instead of absorbing it here.

## Validation

- [`security-compliance-scope.check.md`](D:/Projects/orpheum/checks/security-compliance-scope.check.md) passes.
- [`controls-and-evidence-matrix.check.md`](D:/Projects/orpheum/checks/controls-and-evidence-matrix.check.md) passes.
- [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md) passes.
- [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md) passes.
- The package is ready to feed [`security-compliance-specialist-review.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-review.md).

## Failure Handling

- Stop and ask for clarification if the system scope or obligation horizon cannot be identified honestly from the available inputs.
- If checks fail, rework the earliest artifact rather than expecting downstream roles to infer missing security/compliance logic.
- If scoping work reveals a discovery, architecture, specification, or policy gap, route remediation there before treating the package as settled.

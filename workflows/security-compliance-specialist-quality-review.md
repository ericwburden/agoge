# Security / Compliance Specialist Quality Review

## Purpose

Run the Security / Compliance Specialist check chain across the package, confirm whether it is coherent and downstream-usable, and route remediation to the earliest artifact that introduced any defect.

## When To Use

- The security/compliance scope, controls/evidence matrix, review, and handoff artifacts all exist.
- The package is about to be treated as a reusable security/compliance output.
- A Role Builder or downstream consumer wants to know whether the package is actually coherent and security/compliance-complete.

## Inputs

- Required:
  - instantiated copies of [`artifacts/security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md), [`artifacts/controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md), [`artifacts/security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md), and [`artifacts/security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md)
- Expected supporting context:
  - the corresponding reviewed delivery, policy, vendor, regulatory, or operational references that materially constrain the current package
- Optional:
  - waiver notes, approval notes, incident learnings, or audit notes

## Outputs

- Primary outputs: pass or fail results for the Security / Compliance Specialist check chain and explicit remediation routing
- Secondary outputs: strengthened artifacts, narrowed open questions, and clearer downstream readiness

## Skills And Tools

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) when repeated quality failures are concentrated in control mapping, exception handling, approval-limit clarity, or re-review discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when repeated quality failures are concentrated in unsynthesized obligation, control, or evidence context.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when repeated failures are concentrated in validated-constraint grounding or acceptance-sensitive context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when failures are concentrated in downstream packaging clarity.

## Sequence

1. Read the full security/compliance package together with the reviewed delivery and obligation inputs that materially constrain the current assessment.
2. Run [`security-compliance-scope.check.md`](D:/Projects/orpheum/checks/security-compliance-scope.check.md), [`controls-and-evidence-matrix.check.md`](D:/Projects/orpheum/checks/controls-and-evidence-matrix.check.md), [`security-compliance-review.check.md`](D:/Projects/orpheum/checks/security-compliance-review.check.md), [`security-compliance-handoff.check.md`](D:/Projects/orpheum/checks/security-compliance-handoff.check.md), [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md), and [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).
3. If failures cluster around fragmented local context, use `research-documentation` to synthesize the missing context before rewriting the affected artifact.
4. Route each failure to the earliest artifact that introduced it:
   - scope defects go back to [`security-compliance-specialist-scoping.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
   - control-matrix defects go back to [`security-compliance-specialist-scoping.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
   - review defects go back to [`security-compliance-specialist-review.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-review.md)
   - handoff defects go back to [`security-compliance-specialist-handoff.md`](D:/Projects/orpheum/workflows/security-compliance-specialist-handoff.md)
5. Re-run the full check chain after remediation before treating the package as downstream-ready.

## Decision Points

- If the package fails because reviewed delivery context is materially weak, route that defect upstream instead of hiding it inside security/compliance prose.
- If the package fails because controls, waivers, or approval limits are being overstated or understated, preserve that distinction explicitly rather than smoothing it into generic readiness.
- If the package passes with explicit conditions, keep those conditions visible instead of rewriting the package into a soft-ready summary.
- If repeated failures show weak control authoring, exception posture, or approval-limit wording, use `security-controls-and-exceptions` before rewriting the affected artifact.
- If repeated failures show the current repo skill set is not enough to support this role cleanly, record that explicitly in the skill-sourcing note instead of leaving it as implicit friction.

## Validation

- All six Security / Compliance Specialist checks pass.
- The package is coherent, explicit, and downstream-usable.
- The package preserves the distinction between risk/obligation framing, technical implementation, legal or audit approval, and operational ownership.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly because the scope or obligation horizon is not identifiable.
- If failures continue after local remediation, record the residual gap explicitly and route it to the earliest artifact or role package that needs strengthening.
- If repeated quality-review failures point to a repo-support gap rather than a one-off artifact defect, record that in the skill-sourcing note for future hardening work.

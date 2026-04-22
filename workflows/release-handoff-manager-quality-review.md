---
id: release-handoff-manager-quality-review
kind: workflow
title: Release / Handoff Manager Quality Review
version: 1
summary: Run the Release / Handoff Manager check chain across the release package,
  confirm whether the package is coherent and downstream-usable, and route remediation
  to the earliest artifact that introduced any defect.
role: release-handoff-manager
inputs:
- release-candidate-summary
- release-readiness-decision
- rollout-and-operations-notes
- release-handoff
outputs: []
skills:
- release-readiness-packaging
- research-documentation
- handoff-packaging
- requirements-verification
- webapp-testing
checks:
- release-candidate-summary
- release-readiness-decision
- rollout-and-operations-notes
- release-handoff
- release-traceability
- release-handoff-manager-boundary
handoff_to: []
---

# Release / Handoff Manager Quality Review

## Purpose

Run the Release / Handoff Manager check chain across the release package, confirm whether the package is coherent and downstream-usable, and route remediation to the earliest artifact that introduced any defect.

## When To Use

- The release candidate summary, release readiness decision, rollout notes, and release handoff artifacts all exist.
- The package is about to be treated as a reusable release or downstream adoption output.
- A Role Builder or downstream consumer wants to know whether the package is actually coherent and release-complete.

## Inputs

- Required:
  - instantiated copies of [`artifacts/release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md), [`artifacts/release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md), [`artifacts/rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md), and [`artifacts/release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md)
- Expected supporting context:
  - the corresponding implementation, review, and verification packages and relevant upstream or operational references
- Optional:
  - additional approval notes, communication notes, or specification references

## Outputs

- Primary outputs: pass or fail results for the Release / Handoff Manager check chain and explicit remediation routing
- Secondary outputs: strengthened release artifacts, narrowed open questions, and clearer downstream readiness

## Skills And Tools

- [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md) when repeated quality failures are concentrated in candidate framing, release posture wording, approval-limit clarity, rollout caveats, or re-approval framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when failures still depend on unsynthesized implementation, review, verification, or operational context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when failures are concentrated in downstream packaging clarity.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when traceability back to validated behavior or acceptance commitments is the main weakness.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when repeated release packaging defects depend on missing validation or application evidence.

## Sequence

1. Read the full release package together with the corresponding implementation, review, and verification packages and the operational references that materially constrain the release target.
2. Run [`release-candidate-summary.check.md`](D:/Projects/orpheum/checks/release-candidate-summary.check.md), [`release-readiness-decision.check.md`](D:/Projects/orpheum/checks/release-readiness-decision.check.md), [`rollout-and-operations-notes.check.md`](D:/Projects/orpheum/checks/rollout-and-operations-notes.check.md), [`release-handoff.check.md`](D:/Projects/orpheum/checks/release-handoff.check.md), [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md), and [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).
3. If failures cluster around fragmented local context, use `research-documentation` to synthesize the missing context before rewriting the affected artifact.
4. Route each failure to the earliest artifact that introduced it:
   - candidate framing defects go back to [`release-handoff-manager-packaging.md`](D:/Projects/orpheum/workflows/release-handoff-manager-packaging.md)
   - rollout-notes defects go back to [`release-handoff-manager-packaging.md`](D:/Projects/orpheum/workflows/release-handoff-manager-packaging.md)
   - decision defects go back to [`release-handoff-manager-readiness-review.md`](D:/Projects/orpheum/workflows/release-handoff-manager-readiness-review.md)
   - handoff defects go back to [`release-handoff-manager-handoff.md`](D:/Projects/orpheum/workflows/release-handoff-manager-handoff.md)
5. If failures cluster around candidate framing, release posture wording, approval limits, rollout caveats, or re-approval discipline, use `release-readiness-packaging` before rewriting the affected release artifact.
6. Re-run the full check chain after remediation before treating the package as downstream-ready.

## Decision Points

- If the package fails because the implementation, review, or verification package is materially weak, route that defect upstream instead of trying to hide it inside release prose.
- If the package fails because operational or approval constraints are being overstated or understated, preserve the distinction between release packaging and operational control rather than collapsing them together.
- If the package passes with explicit conditions, keep those conditions visible instead of rewriting the package into a soft-ready summary.
- If repeated failures show the current repo skill set is not enough to support release packaging cleanly, record that explicitly in the skill-sourcing note instead of leaving it as implicit friction.

## Validation

- All six Release / Handoff Manager checks pass.
- The release package is coherent, explicit, and downstream-usable.
- The package preserves the distinction between validated release packaging, deployment execution, and upstream engineering authority.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly because the release target or upstream commitments are not identifiable.
- If failures continue after local remediation, record the residual gap explicitly and route it to the earliest artifact or role package that needs strengthening.
- If repeated quality-review failures point to a repo-support gap rather than a one-off artifact defect, record that in the Release / Handoff Manager skill-sourcing note for future hardening work.

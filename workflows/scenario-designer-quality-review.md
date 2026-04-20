# Scenario Designer Quality Review

## Purpose

Run the Scenario Designer check chain across the scenario package, confirm whether the package is coherent and downstream-usable, and route remediation to the earliest artifact or role-package decision that introduced any defect.

## When To Use

- The scenario definition, scenario integration map, scenario review, and scenario handoff artifacts all exist.
- The package is about to be treated as a reusable scenario output.
- A Role Builder or downstream consumer wants to know whether the package is actually coherent and scenario-complete.

## Inputs

- Required:
  - instantiated copies of [`artifacts/scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md), [`artifacts/scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md), [`artifacts/scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md), and [`artifacts/scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md)
- Expected supporting context:
  - the corresponding role definitions, role-owned workflows, and supporting notes that materially constrain the scenario
- Optional:
  - adoption notes, approval notes, or public comparison references

## Outputs

- Primary outputs: pass or fail results for the Scenario Designer check chain and explicit remediation routing
- Secondary outputs: strengthened scenario artifacts, narrowed open questions, and clearer downstream readiness

## Skills And Tools

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) when repeated quality failures are concentrated in scenario boundary framing, role participation logic, handoff contracts, branching rules, synchronization points, or readiness-posture wording.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when repeated quality failures are concentrated in unsynthesized role-package context or scattered scenario evidence.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when repeated quality failures are concentrated in weak translation of public orchestration patterns into repo-native scenario form.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when failures are concentrated in downstream packaging clarity.

## Sequence

1. Read the full scenario package together with the relevant role packages and any supporting adoption context that materially constrains the scenario.
2. Run [`scenario-definition.check.md`](D:/Projects/orpheum/checks/scenario-definition.check.md), [`scenario-integration-map.check.md`](D:/Projects/orpheum/checks/scenario-integration-map.check.md), [`scenario-review.check.md`](D:/Projects/orpheum/checks/scenario-review.check.md), [`scenario-handoff.check.md`](D:/Projects/orpheum/checks/scenario-handoff.check.md), [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md), and [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).
3. If failures cluster around fragmented local context, use `research-documentation` to synthesize the missing context before rewriting the affected artifact.
4. Route each failure to the earliest artifact that introduced it:
   - definition defects go back to [`scenario-designer-composition.md`](D:/Projects/orpheum/workflows/scenario-designer-composition.md)
   - integration-map defects go back to [`scenario-designer-composition.md`](D:/Projects/orpheum/workflows/scenario-designer-composition.md)
   - review defects go back to [`scenario-designer-review.md`](D:/Projects/orpheum/workflows/scenario-designer-review.md)
   - handoff defects go back to [`scenario-designer-handoff.md`](D:/Projects/orpheum/workflows/scenario-designer-handoff.md)
5. If failures cluster around scenario boundary framing, role participation logic, handoff contracts, branching rules, synchronization points, or readiness-posture wording, use `scenario-composition` before rewriting the affected artifact.
6. If failures cluster around weak adaptation of public orchestration ideas, use `pattern-adaptation` before rewriting the affected artifact.
7. Re-run the full check chain after remediation before treating the package as downstream-ready.

## Decision Points

- If the package fails because a participating role package is materially incomplete, route that defect upstream instead of hiding it inside scenario prose.
- If the package fails because scenario logic is trying to absorb project management or role design, preserve that drift explicitly and remove it.
- If the package passes with explicit conditions, keep those conditions visible instead of rewriting the package into a soft-ready summary.
- If repeated failures show the current repo skill set is not enough to support Scenario Designer work cleanly, record that explicitly in the skill-sourcing note instead of leaving it as implicit friction.

## Validation

- All six Scenario Designer checks pass.
- The scenario package is coherent, explicit, and downstream-usable.
- The package preserves the distinction between scenario orchestration, role-local work, and project-instance execution management.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly because the scenario's lifecycle window or reuse intent is not identifiable.
- If failures continue after local remediation, record the residual gap explicitly and route it to the earliest artifact or role package that needs strengthening.
- If repeated quality-review failures point to a repo-support gap rather than a one-off artifact defect, record that in the Scenario Designer skill-sourcing note for future hardening work.

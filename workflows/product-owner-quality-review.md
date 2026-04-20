# Product Owner Quality Review

## Purpose

Run the Product Owner check chain across the product package, confirm whether the package is coherent and downstream-usable, and route remediation to the earliest artifact that introduced any defect.

## When To Use

- The product direction, backlog prioritization, product decision review, and product handoff artifacts all exist.
- The package is about to be treated as a reusable product-direction output.
- A Role Builder or downstream consumer wants to know whether the package is actually coherent and product-complete.

## Inputs

- Required:
  - instantiated copies of [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md), [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md), [`artifacts/product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md), and [`artifacts/product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md)
- Expected supporting context:
  - the corresponding validated discovery, feedback, release, or specification references that materially constrain the current product decision
- Optional:
  - stakeholder approval notes, roadmap notes, or KPI reviews

## Outputs

- Primary outputs: pass or fail results for the Product Owner check chain and explicit remediation routing
- Secondary outputs: strengthened product artifacts, narrowed open questions, and clearer downstream readiness

## Skills And Tools

- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md) when repeated quality failures are concentrated in tradeoff framing, acceptance guardrails, deferred-scope discipline, or reprioritization logic.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when repeated quality failures are concentrated in validated-need grounding, acceptance conditions, or value framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when failures still depend on unsynthesized discovery, feedback, or delivery context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when failures are concentrated in downstream packaging clarity.

## Sequence

1. Read the full product package together with the validated discovery and other supporting evidence that materially constrain the current product decision.
2. Run [`product-direction.check.md`](D:/Projects/orpheum/checks/product-direction.check.md), [`backlog-prioritization.check.md`](D:/Projects/orpheum/checks/backlog-prioritization.check.md), [`product-decision-review.check.md`](D:/Projects/orpheum/checks/product-decision-review.check.md), [`product-handoff.check.md`](D:/Projects/orpheum/checks/product-handoff.check.md), [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md), and [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).
3. If failures cluster around fragmented local context, use `research-documentation` to synthesize the missing context before rewriting the affected artifact.
4. Route each failure to the earliest artifact that introduced it:
   - direction defects go back to [`product-owner-direction.md`](D:/Projects/orpheum/workflows/product-owner-direction.md)
   - prioritization defects go back to [`product-owner-direction.md`](D:/Projects/orpheum/workflows/product-owner-direction.md)
   - decision defects go back to [`product-owner-review.md`](D:/Projects/orpheum/workflows/product-owner-review.md)
   - handoff defects go back to [`product-owner-handoff.md`](D:/Projects/orpheum/workflows/product-owner-handoff.md)
5. If failures cluster around tradeoff framing, acceptance guardrails, deferred-scope discipline, or reprioritization logic, use `product-priority-framing` before rewriting the affected artifact.
6. Re-run the full check chain after remediation before treating the package as downstream-ready.

## Decision Points

- If the package fails because validated discovery is materially weak, route that defect upstream instead of hiding it inside product prose.
- If the package fails because tradeoffs or deferred scope are being understated, preserve that tension explicitly rather than smoothing it into simple roadmap confidence.
- If the package passes with explicit conditions, keep those conditions visible instead of rewriting the package into a soft-ready summary.
- If repeated failures show the current repo skill set is not enough to support Product Owner work cleanly, record that explicitly in the skill-sourcing note instead of leaving it as implicit friction.

## Validation

- All six Product Owner checks pass.
- The product package is coherent, explicit, and downstream-usable.
- The package preserves the distinction between validated product judgment, technical design, execution planning, and operational ownership.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly because the product scope or decision horizon is not identifiable.
- If failures continue after local remediation, record the residual gap explicitly and route it to the earliest artifact or role package that needs strengthening.
- If repeated quality-review failures point to a repo-support gap rather than a one-off artifact defect, record that in the Product Owner skill-sourcing note for future hardening work.

# Product Owner Handoff

## Purpose

Package reviewed product-direction outputs into a downstream-ready handoff that solutioning, planning, delivery, or approval consumers can use without reconstructing product intent, priorities, and tradeoffs from earlier artifacts.

## When To Use

- Product direction, backlog prioritization, and product decision review artifacts already exist.
- A downstream role or human approver needs a durable summary of current product posture.
- The product package is about to move into solutioning, planning, delivery, release feedback, or business approval work.

## Inputs

- Required:
  - instantiated copies of [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md), [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md), and [`artifacts/product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md)
- Expected supporting context:
  - the corresponding validated discovery or evidence artifacts
- Optional:
  - related roadmap notes, release learnings, or approval notes

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md) in the target project workspace
- Secondary outputs: downstream-ready product summary, priority posture, acceptance guidance, deferred scope, and next-step routing

## Skills And Tools

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default direct-support skill for turning reviewed product outputs into a downstream-ready handoff.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the current product context still needs synthesis before handoff.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the handoff needs stronger grounding in validated requirements or acceptance-sensitive constraints.

## Sequence

1. Read the reviewed product package together with the supporting validated discovery or evidence artifacts.
2. If the current product context is still fragmented across multiple local sources, use `research-documentation` first.
3. Instantiate [`artifacts/product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `handoff-packaging` to populate the product handoff artifact with current direction, package contents, priority posture, acceptance guidance, deferred scope, follow-up owners, revisit triggers, upstream routing notes, and recommended next consumer.
5. Run [`product-handoff.check.md`](D:/Projects/orpheum/checks/product-handoff.check.md), [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md), and [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).

## Decision Points

- If the handoff is primarily for Solution Architect or Technical Planner work, preserve product intent and priority guardrails without drifting into technical design.
- If the handoff is primarily for human approval, keep the tradeoffs and conditions visible rather than collapsing them into a soft endorsement.
- If a downstream consumer needs missing discovery or stronger evidence, route that gap explicitly rather than hiding it in handoff prose.

## Validation

- [`product-handoff.check.md`](D:/Projects/orpheum/checks/product-handoff.check.md) passes.
- [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md) passes.
- [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md) passes.
- The package is ready for downstream solutioning, planning, delivery, release-feedback, or approval use.

## Failure Handling

- Stop and ask for clarification if the next consumer cannot be identified honestly from the available package.
- If packaging clarity is the main weakness, rework the handoff artifact rather than assuming downstream consumers will infer the missing structure.
- If the defect began earlier in the chain, route remediation there before treating the handoff as complete.

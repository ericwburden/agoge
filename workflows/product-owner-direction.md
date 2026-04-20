# Product Owner Direction

## Purpose

Turn validated requirements, product feedback, and delivery learnings into an explicit product direction and backlog prioritization package that downstream solutioning and planning roles can use without reconstructing product intent.

## When To Use

- Validated discovery or requirements already exist.
- A product area needs explicit priority direction before architecture, planning, or renewed delivery work proceeds.
- Product choices need durable packaging rather than only meeting notes or backlog edits.

## Inputs

- Required:
  - the corresponding instantiated requirements or validated discovery artifacts
- Expected supporting context:
  - relevant release learnings, verification findings, stakeholder input, customer feedback, market context, or existing specifications when they materially constrain product choices
- Optional:
  - roadmap notes, KPI reviews, support trends, or operational learnings

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md)
  - an instantiated copy of [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md)
- Secondary outputs: explicit product goal framing, ordered work, deferred scope, tradeoffs, and reprioritization triggers

## Skills And Tools

- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md) as the default direct-support skill for turning validated discovery, feedback, and delivery learnings into explicit product direction, tradeoff framing, acceptance guardrails, deferred scope, and reprioritization discipline.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) as the default grounding skill when product direction or prioritization still needs stronger linkage to validated needs, commitments, and acceptance-sensitive constraints.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing discovery, feedback, delivery learnings, and constraint context before writing product artifacts.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when the main inputs are workshop notes, stakeholder calls, or rough prioritization discussions that need normalization first.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md) when product choices depend materially on external platform, market, or standards research that should be sourced explicitly.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md) when mature product intent should be sharpened into clearer behavioral commitments.

## Sequence

1. Read the validated discovery or requirements together with the product feedback, delivery learnings, and other evidence that materially shape the current decision.
2. If the context is spread across multiple local sources, use `research-documentation` first to synthesize the current product problem, value signals, and constraints.
3. Instantiate [`artifacts/product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md) and [`artifacts/backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md) into the project workspace if working copies do not already exist.
4. Use `product-priority-framing` to populate the product direction artifact with decision scope, validated inputs, product goal, target beneficiaries, value hypotheses, acceptance intent, behavioral guardrails, constraints, priority themes, and open questions.
5. Use `product-priority-framing` to populate the backlog prioritization artifact with ordered work, rationale, acceptance-oriented conditions, deferred scope, sequencing notes, stakeholder tensions, and reprioritization triggers.
6. Run [`product-direction.check.md`](D:/Projects/orpheum/checks/product-direction.check.md), [`backlog-prioritization.check.md`](D:/Projects/orpheum/checks/backlog-prioritization.check.md), [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md), and [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).

## Decision Points

- If the current choice depends on discovery that is not actually validated, route that gap upstream rather than disguising it as a product decision.
- If the most important product choice is about what not to do now, preserve that deferred scope explicitly.
- If acceptance intent is strong enough that it should become or refine a behavioral specification, use the existing Allium skills rather than burying it in backlog prose.
- If acceptance-oriented guidance starts turning into full behavioral definition or test design, route that work to specification, architecture, or verification instead of absorbing it here.
- If sequencing needs exceed product-level ordering and become technical execution structure, route that work to downstream planning instead of absorbing it here.

## Validation

- [`product-direction.check.md`](D:/Projects/orpheum/checks/product-direction.check.md) passes.
- [`backlog-prioritization.check.md`](D:/Projects/orpheum/checks/backlog-prioritization.check.md) passes.
- [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md) passes.
- [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md) passes.
- The package is ready to feed [`product-owner-review.md`](D:/Projects/orpheum/workflows/product-owner-review.md).

## Failure Handling

- Stop and ask for clarification if the product scope or current decision horizon cannot be identified honestly from the available inputs.
- If checks fail, rework the earliest artifact rather than expecting downstream roles to infer missing product logic.
- If product-direction work reveals a discovery, specification, or evidence gap, route remediation there before treating the package as settled.

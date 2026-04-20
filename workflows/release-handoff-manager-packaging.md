# Release / Handoff Manager Packaging

## Purpose

Turn reviewed implementation, code review, and verification outputs into an explicit release candidate summary and rollout notes package that downstream release-adjacent consumers can use without reconstructing the candidate from upstream artifacts.

## When To Use

- Implementation, review, and verification packages already exist.
- A downstream release, adoption, or operational consumer needs more than raw upstream artifacts.
- The release or downstream handoff needs durable packaging rather than only a chat summary.

## Inputs

- Required:
  - the corresponding instantiated implementation package artifacts
  - the corresponding instantiated code review package artifacts
  - the corresponding instantiated verification package artifacts
- Expected supporting context:
  - relevant requirements, architecture, planning, specification, and operational references when they materially constrain the release target
- Optional:
  - release notes, deployment notes, environment notes, approval notes, support notes, or communication drafts

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md)
  - an instantiated copy of [`artifacts/rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md)
- Secondary outputs: explicit candidate scope, release-sensitive hotspots, rollout caveats, operational assumptions, and next-step framing

## Skills And Tools

- [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md) as the default direct-support skill for turning reviewed implementation, review, and verification packages into explicit release candidate framing, release posture context, rollout caveats, and re-approval discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing implementation, review, verification, and operational context before writing release artifacts.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default direct-support skill for turning reviewed outputs into downstream-ready release packaging once the source package is understood.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when release framing depends heavily on validated behavior or acceptance commitments.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when release coordination notes, operational review notes, or adoption notes need normalization.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when release posture depends materially on specification-sensitive drift or constraints.

## Sequence

1. Read the implementation, review, and verification packages together with the relevant upstream and operational references to understand what candidate is actually being prepared for downstream use.
2. If context is spread across multiple local sources, use `research-documentation` first to synthesize the candidate and its constraints.
3. Instantiate [`artifacts/release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md) and [`artifacts/rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md) into the project workspace if working copies do not already exist.
4. Use `release-readiness-packaging` to populate the release candidate summary with release scope, reviewed inputs, included and excluded scope, target consumers, upstream decision anchors, release-sensitive hotspots, and candidate limits.
5. Use `release-readiness-packaging` to populate the rollout and operations notes with environment assumptions, protection rules, sequencing notes, monitoring watchouts, rollback or escalation triggers, communication notes, and trust-boundary-sensitive concerns.
6. Run [`release-candidate-summary.check.md`](D:/Projects/orpheum/checks/release-candidate-summary.check.md), [`rollout-and-operations-notes.check.md`](D:/Projects/orpheum/checks/rollout-and-operations-notes.check.md), [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md), and [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).

## Decision Points

- If the candidate cannot be framed honestly because upstream review or verification posture is unclear, record that limit explicitly instead of inventing release confidence.
- If the most important downstream concern is operational or environment-sensitive rather than product-facing, preserve that emphasis in the package.
- If the issue belongs upstream in implementation, review, verification, or specification work, route it explicitly rather than disguising it as release packaging cleanup.
- If the package is intentionally scoped to a narrow environment, rollout wave, or downstream consumer, state that boundary clearly so later readers do not assume broader coverage.

## Validation

- [`release-candidate-summary.check.md`](D:/Projects/orpheum/checks/release-candidate-summary.check.md) passes.
- [`rollout-and-operations-notes.check.md`](D:/Projects/orpheum/checks/rollout-and-operations-notes.check.md) passes.
- [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md) passes.
- [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md) passes.
- The package is ready to feed [`release-handoff-manager-readiness-review.md`](D:/Projects/orpheum/workflows/release-handoff-manager-readiness-review.md).

## Failure Handling

- Stop and ask for clarification if the release target or downstream consumer cannot be identified honestly from the available package.
- If packaging or traceability checks fail, rework the earlier artifact rather than expecting downstream consumers to infer the missing logic.
- If release packaging reveals the upstream package is materially incomplete or misleading, route remediation there before escalating release language.

# Release / Handoff Manager Handoff

## Purpose

Package the reviewed release outputs into a downstream-ready handoff for release-adjacent, operational, or adoption consumers.

## When To Use

- A release readiness decision already exists and the package needs a downstream-facing summary.
- Downstream consumers need to know what candidate is in scope, what the current release posture is, and what should happen next.
- The release package needs to move beyond raw upstream artifacts or a single readiness label.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md)
  - the corresponding instantiated copies of [`artifacts/release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md) and [`artifacts/rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md)
- Expected supporting context:
  - the corresponding implementation, review, and verification packages
- Optional:
  - approval notes, communication drafts, support notes, or additional release references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md) in the target project workspace
- Secondary outputs: downstream-ready summary of release posture, active conditions, rollout watchouts, owners, and next consumers

## Skills And Tools

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default path for downstream-ready release packaging once the release posture is explicit.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the handoff still depends on synthesizing multiple upstream or operational sources.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when downstream consumers need stronger requirement-grounded framing for release conditions.

## Sequence

1. Read the release readiness decision together with the release candidate summary, rollout notes, and the corresponding implementation, review, and verification packages.
2. If the handoff still depends on unsynthesized context, use `research-documentation` first.
3. Instantiate [`artifacts/release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md) into the project workspace if a working copy does not already exist.
4. Use `handoff-packaging` to populate the release handoff artifact with the candidate summary, release package included, current release posture, active conditions, owners, rollout and monitoring watchouts, re-review or re-approval triggers, upstream routing notes, and recommended next consumer.
5. Run [`release-handoff.check.md`](D:/Projects/orpheum/checks/release-handoff.check.md), [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md), and [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).

## Decision Points

- If the candidate is blocked, do not package the handoff as though downstream work can proceed normally.
- If follow-up ownership is unclear, make that ambiguity explicit rather than smoothing it over.
- If the most important downstream concern is operational or environment-sensitive rather than product-facing, preserve that distinction in the handoff.
- If release packaging uncovered an upstream defect, route it explicitly rather than burying it in the follow-up list.
- If downstream trust depends on a fresh approval, environment change, or evidence update, say so explicitly rather than leaving downstream consumers to infer that re-review is required.

## Validation

- [`release-handoff.check.md`](D:/Projects/orpheum/checks/release-handoff.check.md) passes.
- [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md) passes.
- [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md) passes.
- The package is ready for downstream release-adjacent, operational, adoption, or human approval handling.

## Failure Handling

- Stop and ask for clarification if the current release posture or next consumer cannot be identified honestly.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to infer what matters.
- If the real defect is earlier in the candidate, rollout, or decision artifact, repair that earlier stage instead of decorating the handoff.

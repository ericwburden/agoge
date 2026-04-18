# Role Builder Handoff

## Purpose

Turn a reviewed reusable role package into an adoption-facing handoff that another repo, team, or downstream designer can use without reconstructing the package from earlier artifacts or chat history.

## When To Use

- A role package has passed quality review and is ready or conditionally ready for downstream adoption.
- Another repo, team, or designer needs a clean entry point into the role package.
- A reusable role should be handed off with explicit guidance, limits, and next steps instead of only with raw package files.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md)
  - an instantiated copy of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md)
  - an instantiated copy of [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md)
- Optional: supporting notes and adopter-specific context when it materially affects packaging guidance

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md) in the target working location
- Secondary outputs: adoption guidance, preserved conventions, limitations, and next-step routing

## Skills And Tools

- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md) as the preferred default path for packaging a reviewed role for adoption.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) only when multiple local notes still need synthesis before adoption guidance can be written clearly.

## Sequence

1. Read the instantiated role definition brief, role support system, and role package review together.
2. Confirm the review outcome marks the package ready or conditionally ready with explicit limits.
3. If supporting notes still need synthesis before adoption guidance can be written honestly, use `research-documentation` first.
4. Instantiate [`artifacts/role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md) into the working location if a working copy does not already exist.
5. Use `role-handoff-packaging` to populate the instantiated artifact with the package summary, reviewed inputs, review outcome, adoption guidance, recommended consumers, current limitations, required conventions, Allium expectations, and next adoption steps.
6. Run [`role-package-handoff.check.md`](D:/Projects/agoge/checks/role-package-handoff.check.md), [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md), and [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md).

## Decision Points

- If the review outcome is not ready, stop and route remediation back through the review or earlier artifacts rather than packaging an unready role.
- If the package is conditionally ready, keep the limits explicit in the handoff instead of smoothing them over.
- If the role has an Allium relationship, make that contract explicit for adopters instead of assuming they will infer it from earlier artifacts.
- If adopter-specific context would materially change the handoff, state the adaptation need explicitly rather than pretending the current package is universal.

## Validation

- [`role-package-handoff.check.md`](D:/Projects/agoge/checks/role-package-handoff.check.md) passes.
- [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) passes.
- [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md) passes.
- The instantiated handoff is usable as the downstream entry point to the role package.

## Failure Handling

- Stop and ask for clarification if the reviewed package does not provide enough evidence to package adoption guidance honestly.
- If the primary check fails, rework the handoff with `role-handoff-packaging` and use `research-documentation` only when unsynthesized local notes are still blocking a clean adoption summary.
- If a traceability or boundary check fails, route remediation to the earliest artifact or adoption claim that introduced the defect.

## Notes

This workflow is the final operational stage of the current Role Builder lifecycle.

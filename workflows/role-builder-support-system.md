# Role Builder Support System

## Purpose

Turn a stable role definition into an operational support package of artifacts, workflows, checks, skills, and tooling preferences.

## When To Use

- A role definition is stable enough to derive a support package from it.
- A role exists but lacks explicit workflows, checks, or skill support.
- A role package needs its artifact chain and operational lifecycle documented clearly.

## Inputs

- Required: an instantiated copy of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md)
- Optional: prior workflows, prior checks, supporting notes, and relevant skill references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md) in the target working location
- Secondary outputs: workflow recommendations, check recommendations, skill classifications, and surfaced support gaps

## Skills And Tools

- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md) as the preferred default path for deriving the role's artifact set, workflows, checks, skill support, and tooling preferences from the role definition.
- [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md) when a useful pattern is tool-coupled and needs to be translated into a local-first or Markdown-first form.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) only when multiple local precedents or role-package examples still need synthesis before the support system can be designed cleanly.

## Sequence

1. Read the instantiated role definition brief and identify the role's default outputs, boundaries, and lifecycle position.
2. Review existing local artifacts, workflows, checks, and skills that could support the role.
3. If local precedents still need synthesis before design decisions can be made honestly, use `research-documentation` first rather than folding unsynthesized references into the support-system artifact.
4. If a candidate support pattern is useful but tool-coupled, use `pattern-adaptation` to determine what should be preserved, stripped, adapted, or rejected before building it into the role package.
5. Instantiate [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md) into the working location if a working copy does not already exist.
6. Use `role-support-system-design` to populate the instantiated artifact with the role's artifact set, workflow set, check set, skill support map, tooling preferences, Allium touchpoints, lifecycle, and known gaps.
7. Run [`role-support-system.check.md`](D:/Projects/agoge/checks/role-support-system.check.md), [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md), and [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md).

## Decision Points

- If the role's outputs are too vague to derive artifacts, return to the role-definition workflow first.
- If the support-system design starts leaning on generic synthesis instead of the dedicated `role-*` skills, simplify the chain and restore the role-specific primary path.
- If an existing skill is useful but tool-coupled, use `pattern-adaptation` to preserve the method explicitly and document the local adaptation rather than importing the whole tool assumption.
- If the role touches product behavior, make its expected relationship to Allium explicit instead of leaving specification interaction implicit.
- If the support system would add overhead without improving repeatability, simplify it before proceeding.

## Validation

- [`role-support-system.check.md`](D:/Projects/agoge/checks/role-support-system.check.md) passes.
- [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) passes.
- [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md) passes.
- The instantiated output is ready to feed [`role-builder-quality-review.md`](D:/Projects/agoge/workflows/role-builder-quality-review.md).

## Failure Handling

- Stop and ask for clarification if the support system cannot be derived honestly from the role definition.
- If the primary check fails, rework the support-system artifact with `role-support-system-design` and use `research-documentation` when the design still depends on unsynthesized local precedents.
- If the failure is caused by a weak or implicit adaptation decision, rework that decision with `pattern-adaptation` before changing the role package further.
- If a traceability or boundary check fails, route remediation to the earlier role-definition or support-system decision that introduced the defect.

## Notes

This workflow is where a role becomes operational rather than merely described.

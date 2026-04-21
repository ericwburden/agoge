---
id: role-builder-role-definition
kind: workflow
title: Role Builder Role Definition
version: 1
summary: Turn a role idea, source references, and observed practice into a clear reusable
  role definition that can support design of the role's artifact, workflow, and check
  package.
role: role-builder
inputs: []
outputs:
- role-definition-brief
skills:
- role-definition
- meeting-notes-and-actions
- research-documentation
- content-research-writer
checks:
- role-definition-brief
- role-package-traceability
- role-builder-boundary
handoff_to: []
---

# Role Builder Role Definition

## Purpose

Turn a role idea, source references, and observed practice into a clear reusable role definition that can support design of the role's artifact, workflow, and check package.

## When To Use

- A new reusable agent role needs to be defined from scratch.
- An existing role idea is still too vague to support workflow or artifact design.
- External reference material or observed practice needs to be translated into a repo-native role definition.

## Inputs

- Required: role idea, target job-to-be-done, or source references
- Optional: prior role definitions, comparison notes, raw workshop notes, interview notes, meeting transcripts, and external role references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md) in the target working location
- Secondary outputs: clarified scope boundaries, source notes, assumptions, and open questions

## Skills And Tools

- [`role-definition`](D:/Projects/agoge/skills/role-definition/SKILL.md) as the preferred default path for turning a role idea and references into a reusable role-definition brief.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) when the starting point is raw workshop notes, interview notes, or meeting transcripts that need to be normalized before role-definition work can proceed cleanly.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) only when multiple local sources still need to be synthesized before the role can be defined clearly.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) only when external references need to be summarized or compared before the role can be defined clearly.

## Sequence

1. Gather the role idea, source references, intended job-to-be-done, and any raw workshop notes or transcripts.
2. If the starting point is raw workshop notes, interview notes, or transcripts, use `meeting-notes-and-actions` first to normalize candidate responsibilities, out-of-scope items, open questions, assumptions, and boundary decisions.
3. If the role still depends on multiple local or external references, use `research-documentation` or `content-research-writer` to synthesize that material before drafting.
4. Instantiate [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md) into the working location if a working copy does not already exist.
5. Use `role-definition` to populate the instantiated artifact with the role's job-to-be-done, audience, lifecycle position, responsibilities, boundaries, success criteria, outputs, Allium relationship, source references, assumptions, and open questions.
6. Run [`role-definition-brief.check.md`](D:/Projects/agoge/checks/role-definition-brief.check.md), [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) when enough of the package exists, and [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md).

## Decision Points

- If the role idea is still mostly personality language, convert it into a job-to-be-done before drafting.
- If the inputs are mostly raw workshop notes or discussion fragments, normalize them before drafting instead of treating them as an already-stable role definition.
- If the role touches product behavior, specification refinement, or spec-code alignment, make its Allium relationship explicit instead of leaving that dependency implicit.
- If multiple plausible role boundaries exist, preserve the ambiguity as an open question instead of silently picking one.
- If the role depends on a tool-specific pattern, describe the underlying method separately from the tool assumption.

## Validation

- [`role-definition-brief.check.md`](D:/Projects/agoge/checks/role-definition-brief.check.md) passes.
- [`role-package-traceability.check.md`](D:/Projects/agoge/checks/role-package-traceability.check.md) passes when enough of the role package exists for it to apply.
- [`role-builder-boundary.check.md`](D:/Projects/agoge/checks/role-builder-boundary.check.md) passes.
- The instantiated output is ready to feed [`role-builder-support-system.md`](D:/Projects/agoge/workflows/role-builder-support-system.md).

## Failure Handling

- Stop and ask for more information if the role's job-to-be-done cannot be stated without inventing purpose or boundaries.
- If the primary check fails, rework the role definition with `role-definition`, normalize workshop inputs with `meeting-notes-and-actions` when needed, add stronger source synthesis with `research-documentation`, or use `content-research-writer` for better comparison inputs.
- If a traceability or boundary check fails, route remediation to the earliest role-definition section or source assumption that introduced the defect.

## Notes

This is the default entry workflow for Role Builder work, whether it starts from a role idea or from normalized workshop intake.

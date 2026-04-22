---
id: role-builder-boundary
kind: check
title: Role Builder Boundary Check
version: 1
summary: Validate that role-design work stays inside the boundaries of reusable role-package
  design and does not drift into project-instance delivery work, implementation ownership,
  or unsupported framework overhead.
mode: presence
severity: error
applies_to: []
---

# Role Builder Boundary Check

## Purpose

Validate that role-design work stays inside the boundaries of reusable role-package design and does not drift into project-instance delivery work, implementation ownership, or unsupported framework overhead.

## Applies To

- All instantiated Role Builder artifacts and Role Builder workflows.
- Use whenever reviewing whether the role package stayed inside the intended role-design boundary.
- Do not use as a substitute for completeness or traceability checks.

## Criteria

- The work defines reusable role structure rather than project-instance deliverables.
- Role responsibilities, artifacts, and checks are grounded in the intended job-to-be-done rather than invented framework language.
- Workflow and check design are treated as operational support, not optional decoration.
- Tool preferences are explicit when they matter, without hard-coding unnecessary tool lock-in.
- The package does not expand into implementation planning, product delivery, or unrelated governance work.
- The role package remains lightweight enough to be adopted without unnecessary overhead.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably role-package design rather than a disguised project plan or framework exercise.

## Evidence Required

- The instantiated Role Builder artifact or workflow output being reviewed.
- The [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) role definition when needed for interpreting boundary drift.

If the work's purpose is ambiguous or over-expanded, fail the check and identify the drift explicitly.

## Supporting Skills

- [`role-definition`](D:/Projects/agoge/skills/role-definition/SKILL.md) when the package has drifted because the role's core job or boundaries were not defined clearly.
- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md) when the drift comes from overbuilt or misplaced support structure.
- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) when the package needs a clearer readiness review and remediation route instead of hand-waving drift away.
- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md) when the adoption handoff starts drifting into project-instance delivery work or unsupported claims.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the drift is caused by unsynthesized source references or role precedents.

## Failure Response

- Rework the output to remove project-instance delivery content, unsupported framework overhead, or role drift.
- Route project-specific implementation or product work into the appropriate role rather than leaving it embedded in role-design artifacts.

## Notes

This is the second cross-cutting quality check for role-package design.

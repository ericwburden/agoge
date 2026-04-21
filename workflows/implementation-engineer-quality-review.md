---
id: implementation-engineer-quality-review
kind: workflow
title: Implementation Engineer Quality Review
version: 1
summary: Run the full Implementation Engineer check chain against an implementation
  package and route remediation before downstream use.
role: implementation-engineer
inputs:
- implementation-record
- implementation-evidence
- implementation-readiness-review
- implementation-package-handoff
outputs: []
skills:
- implementation-package-prep
- research-documentation
- handoff-packaging
- implementation-skill-discovery
checks:
- implementation-record
- implementation-evidence
- implementation-readiness-review
- implementation-package-handoff
- implementation-traceability
- implementation-engineer-boundary
handoff_to: []
---

# Implementation Engineer Quality Review

## Purpose

Run the full Implementation Engineer check chain against an implementation package and route remediation before downstream use.

## When To Use

- The implementation record, implementation evidence, implementation readiness review, and implementation package handoff artifacts all exist.
- The package is about to move to code review, verification, or release-adjacent work.
- A user asks for a package-hardening or quality-review pass on implementation outputs.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
  - an instantiated copy of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
  - an instantiated copy of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
  - an instantiated copy of [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)
- Optional: upstream requirements, architecture, planning, and specification artifacts; logs; screenshots; run outputs; and supporting notes

## Outputs

- Primary outcome: a pass/fail quality judgment on the implementation package
- Secondary outputs: explicit remediation targets, broken traceability links, role-boundary findings, and packaging gaps

## Skills And Tools

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) when failures show that the implementation package itself needs stronger structure before downstream use.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when evidence or context needs synthesis before the check chain can be applied honestly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main failures are in downstream implementation packaging.
- [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) when the same Implementation Engineer failure patterns recur often enough that a dedicated repo-native implementation skill may now be justified.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when traceability or readiness depends on specification-to-implementation alignment.

## Sequence

1. Read the full implementation artifact chain together with the relevant upstream requirements, architecture, planning, and specification context.
2. If the implementation package depends on scattered notes, logs, or evidence references, use `research-documentation` to synthesize the context needed to judge it honestly.
3. Run [`implementation-record.check.md`](D:/Projects/orpheum/checks/implementation-record.check.md).
4. Run [`implementation-evidence.check.md`](D:/Projects/orpheum/checks/implementation-evidence.check.md).
5. Run [`implementation-readiness-review.check.md`](D:/Projects/orpheum/checks/implementation-readiness-review.check.md).
6. Run [`implementation-package-handoff.check.md`](D:/Projects/orpheum/checks/implementation-package-handoff.check.md).
7. Run [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md).
8. Run [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).
9. Route remediation to the earliest artifact or upstream package that introduced the defect instead of patching only the last artifact in the chain.

## Decision Points

- If a failure is local to implementation evidence or packaging, remediate it inside the implementation package, using [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) as the default strengthening path.
- If a failure exposes an upstream requirement, architecture, planning, or specification defect, route it upstream rather than treating it as only an implementation-edit problem.
- If the package technically passes individual artifact checks but fails the traceability or boundary checks, treat the package as not ready.
- If the same failure classes recur across multiple Implementation Engineer passes, invoke [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) to decide whether the repo now needs a dedicated implementation skill instead of continued artifact-only remediation.
- If evidence is too weak to judge honestly, stop and identify the missing evidence rather than guessing a pass.

## Validation

- All six Implementation Engineer checks pass.
- The resulting implementation package is ready to move downstream without requiring code-review or QA roles to reconstruct implementation intent, evidence, or scope boundaries.

## Failure Handling

- Stop downstream handoff until failed checks are remediated.
- Make remediation targets explicit and attach them to the earliest responsible artifact or upstream role.
- Do not mark the implementation package ready while traceability, evidence integrity, or role-boundary failures remain open.

## Notes

This is the standard final quality gate for the first-pass Implementation Engineer package.

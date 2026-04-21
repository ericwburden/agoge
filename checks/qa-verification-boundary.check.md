---
id: qa-verification-boundary
kind: check
title: QA / Verification Lead Boundary Check
version: 1
summary: Validate that QA / Verification Lead outputs stay inside verification role
  boundaries and do not drift into redoing business discovery, re-architecting the
  solution, re-planning execution, owning implementation changes, or taking over release
  operations.
mode: presence
severity: error
applies_to: []
---

# QA / Verification Lead Boundary Check

## Purpose

Validate that QA / Verification Lead outputs stay inside verification role boundaries and do not drift into redoing business discovery, re-architecting the solution, re-planning execution, owning implementation changes, or taking over release operations.

## Applies To

- All instantiated QA / Verification Lead artifacts and QA / Verification Lead workflows.
- Use whenever reviewing whether verification output stayed within the intended role.
- Do not use as a substitute for completeness or traceability checks.

## Criteria

- The output does not redefine business objectives or requirements without routing those issues upstream.
- The output does not silently re-architect the solution or rewrite the execution plan when the issue belongs in architecture or planning.
- The output does not collapse into test implementation ownership, production code ownership, or detailed defect fixing.
- The output does not turn into release operations, deployment choreography, or standing delivery-status ownership.
- Evidence strength, evidence weakness, and verification assumptions are explicit rather than treated as confirmed facts.
- The work remains focused on verification strategy, coverage, evidence review, readiness, and downstream confidence framing.
- Verification findings are routed to the correct earlier artifact or role rather than hidden inside a soft approval.
- Trust boundaries, rollout-sensitive behavior, and human control points are identified when relevant without drifting into governance bureaucracy.

## Scoring Or Outcome

Pass/fail.

The output passes only if it remains recognizably QA / Verification Lead work rather than a disguised BA artifact, architecture rewrite, implementation task list, or release runbook.

## Evidence Required

- The QA / Verification Lead artifact or workflow output being reviewed.
- The [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role definition when needed for role-boundary interpretation.
- Relevant upstream requirements, architecture, planning, or implementation artifacts when needed to identify whether drift originated upstream or inside the verification work.

If the output's role identity is ambiguous, fail the check and identify the drift explicitly.

## Supporting Skills

- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) when role drift began in unstable requirement or acceptance framing.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) when the drift appears in downstream verification packaging.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the verification drift is caused by unsynthesized local context or contradictory references.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when apparent verification drift is actually specification-to-implementation misalignment that should be routed separately.

## Failure Response

- Rework the output to remove role drift before treating it as a valid verification artifact.
- Route requirement, architecture, planning, implementation, or release ambiguity back to the correct role rather than leaving it embedded in verification output.

## Notes

This is the second cross-cutting QA / Verification Lead check. It protects the verification role from absorbing too many adjacent jobs.

---
id: role-support-system
kind: check
title: Role Support System Check
version: 1
summary: Validate that the proposed support system for a role makes the role operational
  through an explicit artifact chain, workflow chain, check set, and skill map.
mode: headings
severity: error
applies_to:
- role-support-system
required_headings:
- Purpose
- Completion Guidance
- Related Checks
- Role In Scope
- Artifact Set
- Workflow Set
- Check Set
- Skill Support Map
- Tooling Preferences
- Allium Touchpoints
- Lifecycle And Handoffs
- Gaps And Risks
- Open Questions
---

# Role Support System Check

## Purpose

Validate that the proposed support system for a role makes the role operational through an explicit artifact chain, workflow chain, check set, and skill map.

## Applies To

- instantiated copies of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md)
- Use after the role definition is stable enough to design the support package.
- Do not use as a substitute for the final coherence review.

## Criteria

- The artifact set is explicit and tied to the role's default outputs.
- The workflow set is explicit and sufficient to produce and validate those outputs.
- The check set includes both primary and cross-cutting quality gates where needed.
- Supporting skills are classified clearly as direct support, local adaptations, or adjacent/downstream only.
- Tooling preferences are explicit when multiple operating models are possible.
- Allium touchpoints are explicit when the role interacts with specifications or behavioral definition.
- Lifecycle and handoff expectations are clear.
- Gaps and risks are surfaced rather than ignored.

## Scoring Or Outcome

Pass/fail.

The artifact passes only if the role could be executed consistently from the documented support system without requiring hidden conventions.

## Evidence Required

- The role support system artifact.
- The role definition brief used to derive it.
- Any relevant workflow, check, or skill references that the support-system design relies on.

If the support system depends on implied structure not captured in the artifact, fail the check.

## Supporting Skills

- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md) when the support system needs to be reworked into a clearer artifact/workflow/check/skill package with explicit tooling preferences.
- [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md) when tool-coupled patterns need to be translated into explicit local support decisions rather than copied directly.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the support-system design depends on synthesizing multiple local patterns or prior role materials.

## Failure Response

- Rework the support-system design before treating the role as operational.
- Add or refine missing artifacts, workflows, checks, or skill classifications instead of compensating with chat-only guidance.

## Notes

This is the main operational quality gate for role-package design.

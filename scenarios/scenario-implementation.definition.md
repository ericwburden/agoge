---
id: scenario-implementation
kind: scenario
title: Scenario Implementation Definition
version: 1
summary: Capture the reusable `Scenario Implementation` scenario that turns a candidate
  scenario idea into a reviewed, adoption-ready scenario package, together with any
  participant-role hardening needed for that scenario to work operationally.
roles:
- scenario-designer
- role-builder
- security-compliance-specialist
workflows:
- scenario-designer-composition
- scenario-designer-review
- scenario-designer-quality-review
- scenario-designer-handoff
- role-builder-quality-review
- role-builder-handoff
- role-builder-support-system
- role-builder-role-definition
- security-compliance-specialist-scoping
- security-compliance-specialist-review
- security-compliance-specialist-handoff
artifacts:
- scenario-definition
- scenario-integration-map
- scenario-review
- scenario-handoff
- role-package-review
- role-package-handoff
- role-support-system
- role-definition-brief
- security-compliance-scope
- controls-and-evidence-matrix
- security-compliance-review
- security-compliance-handoff
checks:
- scenario-definition
- scenario-integration-map
- scenario-traceability
- scenario-designer-boundary
- scenario-review
- scenario-handoff
- role-package-review
- role-package-traceability
- role-builder-boundary
- role-package-handoff
- role-support-system
- role-definition-brief
- security-compliance-scope
- controls-and-evidence-matrix
- security-compliance-traceability
- security-compliance-specialist-boundary
- security-compliance-review
- security-compliance-handoff
entry_conditions:
- a-candidate-scenario-idea-recommendation-or-explicit-implementation-request-already-exists
- the-likely-participating-role-packages-are-already-available-or-the-scenario-can-honestly-be-blocked-on-missing-role-support
- the-work-is-to-create-a-reusable-scenario-package-rather-than-a-one-off-project-plan
- the-repository-is-prepared-to-treat-participant-role-hardening-as-part-of-scenario-implementation-rather-than-as-unrelated-follow-up-work
exit_conditions:
- a-scenario-definition-artifact-for-the-new-scenario
- a-scenario-integration-map-artifact-for-the-new-scenario
- a-reviewed-scenario-posture-and-scenario-handoff
- explicit-participant-role-fit-decisions
- durable-participant-package-review-or-hardening-outcomes-where-those-decisions-materially-shaped-scenario-readiness
- any-required-role-package-hardening-for-scenario-relevant-workflows-checks-artifacts-or-boundaries
- explicit-skills-review-outcomes-including-new-skill-implementation-when-needed-for-scenario-critical-participant-work
- downstream-adopters-can-use-the-new-scenario-package-without-reconstructing-its-choreography-and-without-discovering-only-at-execution-time-that-participant-roles-lack-the-support-needed-to-perform-their-scenario-relevant-work
- the-package-may-be-marked-ready-only-after-the-full-scenario-designer-quality-review-check-chain-passes-across-the-completed-scenario-definition-integration-map-review-and-handoff-artifacts-with-no-unresolved-defect-that-still-requires-upstream-remediation-before-downstream-reuse
---

# Scenario Implementation Definition

## Purpose

Capture the reusable `Scenario Implementation` scenario that turns a candidate scenario idea into a reviewed, adoption-ready scenario package, together with any participant-role hardening needed for that scenario to work operationally.

Use this scenario when a new reusable scenario should be implemented in the repository rather than discussed informally or left as a one-off branch workflow.

## Scenario Name And Intent

`Scenario Implementation`

This scenario exists to dogfood the repository's own role and scenario system by making scenario implementation itself a reusable multi-role activity instead of an ad hoc design exercise.

## Lifecycle Window And Trigger Conditions

This scenario sits between scenario recommendation or selection and downstream adoption of a new reusable scenario package.

Trigger it when:

- a new scenario idea has been selected for implementation
- the scenario should be expressed as a first-class reusable package in `scenarios/`
- participating role packages need to be checked for scenario fit before the scenario is declared ready
- scenario-relevant skills may need review, addition, or implementation before the resulting scenario can be treated as operational

## Participating Roles And Why

- [`Scenario Designer`](C:/Users/ericw/Projects/orpheum/roles/scenario-designer.md)
  - defines the scenario package, integration logic, readiness posture, and downstream handoff
- [`Role Builder`](C:/Users/ericw/Projects/orpheum/roles/role-builder.md)
  - verifies that participating role packages can support the scenario, reviews scenario-relevant skills, and hardens role packages when workflows, checks, artifacts, or skills are missing
- optional [`Security / Compliance Specialist`](C:/Users/ericw/Projects/orpheum/roles/security-compliance-specialist.md)
  - participates only when the scenario being implemented introduces trust-boundary-sensitive, approval-sensitive, or compliance-sensitive concerns that must shape the package before it is treated as ready

## Entry Conditions

- a candidate scenario idea, recommendation, or explicit implementation request already exists
- the likely participating role packages are already available, or the scenario can honestly be blocked on missing role support
- the work is to create a reusable scenario package rather than a one-off project plan
- the repository is prepared to treat participant-role hardening as part of scenario implementation rather than as unrelated follow-up work

## Target Outputs And Exit Conditions

The scenario completes successfully when the implementation effort yields:

- a scenario definition artifact for the new scenario
- a scenario integration map artifact for the new scenario
- a reviewed scenario posture and scenario handoff
- explicit participant-role fit decisions
- durable participant-package review or hardening outcomes where those decisions materially shaped scenario readiness
- any required role-package hardening for scenario-relevant workflows, checks, artifacts, or boundaries
- explicit skills review outcomes, including new skill implementation when needed for scenario-critical participant work

Exit condition:

- downstream adopters can use the new scenario package without reconstructing its choreography, and without discovering only at execution time that participant roles lack the support needed to perform their scenario-relevant work
- the package may be marked `ready` only after the full Scenario Designer quality-review check chain passes across the completed scenario definition, integration map, review, and handoff artifacts, with no unresolved defect that still requires upstream remediation before downstream reuse

## Core Sequence

1. Select or confirm the scenario idea and its intended lifecycle window.
2. Draft the scenario definition and scenario integration map through Scenario Designer work.
3. Review each named participant role package for scenario fit through Role Builder work.
4. If a participant package lacks the role-definition or support-system artifacts needed for an honest hardening pass, instantiate or refresh those artifacts for that package first.
5. Review scenario-relevant skills for each participant package and decide whether existing skills are sufficient, should be rewired, or require new implementation.
6. Implement any required participant-role hardening per affected package, including scenario-relevant workflow, check, artifact, or skill additions.
7. Re-review the scenario package against the hardened participant-role state.
8. Repeat the hardening-and-re-review loop only while each pass is still surfacing concrete structural defects, clearer routing, or materially stronger readiness evidence.
9. Stop the loop when a new pass no longer produces a substantive package improvement, then draft the scenario handoff so the full scenario package exists as a reviewable unit.
10. Run a final scenario-quality pass across the completed package, including the handoff artifact, before downstream adoption.

## Decision Gates And Human Checkpoints

- the scenario idea must be specific enough to describe a reusable multi-role activity rather than a one-off branch plan
- the participant-role fit review must be explicit before the scenario is treated as ready
- the skills review must be explicit before the scenario is treated as operational
- new skills should be implemented only when existing skills and workflow wiring do not honestly cover the scenario-critical method
- repeated hardening is acceptable when it is still producing concrete package improvements, but the scenario should converge once additional passes become mostly restatement or cosmetic cleanup
- the package should transition from `conditional` to `ready` only when the final Scenario Designer quality review confirms the scenario is coherent, downstream-usable, and free of unresolved blocking defects
- human approval remains visible when the scenario introduces material control points, authority questions, or trust-boundary-sensitive transitions

## Scenario Constraints And Non-Goals

- This scenario does not replace Scenario Designer or Role Builder role-local workflows; it composes them.
- This scenario does not act as a generic meta-process for designing anything in the repository.
- This scenario does not absorb implementation execution, release handling, or project-instance management.
- This scenario does not silently invent missing roles; if a scenario depends on a role that does not exist, that gap should be made explicit and routed appropriately.
- This scenario should stay lightweight enough that dogfooding improves repository quality rather than creating recursive process overhead.
- This scenario should not require endless self-application; repeated hardening is a means of convergence, not the product itself.
- The hardening-loop stop rule belongs to this implementation scenario itself and should not be copied into target scenarios by default unless their own ordinary operating posture genuinely requires it.

## Open Questions And Design Gaps

- If repeated use shows that scenario implementation frequently depends on creating brand-new roles, the repository may need a sharper rule about when scenario work should block and route into separate role-creation work.
- If repeated use shows that skills review becomes too implicit inside Role Builder hardening, the repository may eventually want a more explicit skill-sourcing artifact for scenario implementation work.
- If repeated self-application remains the only reliable way to reach `ready`, the repository may need a more explicit convergence or acceptance rubric for scenario hardening.

## Recommended Next Step

Run the final `Scenario Designer` quality review across the completed Scenario Implementation package, then update the package posture to `ready` only if the full Scenario Designer check chain passes.

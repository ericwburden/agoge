---
id: project-discovery
kind: scenario
title: Project Discovery Scenario Definition
version: 1
summary: Capture the reusable `Project Discovery` scenario that turns a kickoff request,
  vague initiative idea, release-driven follow-up need, or early stakeholder context
  into a validated discovery package with explicit business objectives, process understanding,
  requirements posture, and downstream readiness for planning.
roles:
- business-analyst
- product-owner
workflows:
- business-analyst-kickoff
- business-analyst-process-analysis
- business-analyst-requirements-handoff
- product-owner-direction
- product-owner-review
- product-owner-handoff
artifacts:
- business-objectives
- process-analysis
- requirements-specification
- requirements-handoff
- product-direction
- backlog-prioritization
- product-decision-review
- product-handoff
checks:
- business-objectives
- traceability
- business-analyst-boundary
- process-analysis
- requirements-specification
- requirements-handoff
- product-direction
- backlog-prioritization
- product-traceability
- product-owner-boundary
- product-decision-review
- product-handoff
entry_conditions:
- a-project-idea-problem-signal-stakeholder-request-or-equivalent-initiating-context-exists
- validated-discovery-does-not-yet-exist-or-is-not-yet-strong-enough-to-support-downstream-planning-honestly
- the-participating-role-packages-are-available-and-treated-as-the-source-of-truth-for-role-local-workflows
- the-scenario-is-being-used-as-a-reusable-upstream-discovery-phase-not-as-architecture-design-sprint-planning-or-implementation-work
exit_conditions:
- explicit-business-objectives-with-scope-boundaries-stakeholders-success-criteria-and-confirmation-posture
- explicit-current-state-and-future-state-process-understanding-with-process-needs-rules-exceptions-and-operational-gaps
- a-verified-requirements-posture-with-clear-traceability-to-business-goals-and-process-needs
- a-downstream-ready-requirements-handoff-that-names-risks-dependencies-unresolved-questions-and-confirmation-limits
- optional-product-readiness-or-early-priority-framing-when-discovery-alone-is-not-enough-to-decide-whether-the-work-should-proceed-into-planning
- semantic-artifact-review-completed-in-planning-mode-or-the-host-equivalent
- changed-decisions-captured-explicitly-and-reconciled-across-the-discovery-package
- downstream-planning-roles-can-begin-project-planning-with-validated-discovery-rather-than-reconstructing-the-business-problem-process-context-or-requirement-basis-from-kickoff-chatter
---

# Project Discovery Scenario Definition

## Purpose

Capture the reusable `Project Discovery` scenario that turns a kickoff request, vague initiative idea, release-driven follow-up need, or early stakeholder context into a validated discovery package with explicit business objectives, process understanding, requirements posture, and downstream readiness for planning.

Use this scenario when a team needs a disciplined multi-role discovery phase rather than jumping directly from an idea, request, or feedback signal into product planning or technical solutioning.

## Scenario Name And Intent

`Project Discovery`

This scenario exists to compose the repository's discovery-oriented roles into one reusable upstream phase that reduces ambiguity between "we have a project idea or problem signal" and "we have validated discovery that is honest enough to feed `Project Planning`."

## Lifecycle Window And Trigger Conditions

This scenario sits between a raw project or change signal and downstream planning work.

Trigger it when:

- a kickoff request, stakeholder ask, release-driven follow-up, or other project signal exists but validated discovery does not yet
- the business problem, desired outcome, current-state process, or requirements posture is still too weak for `Project Planning`
- downstream product, architecture, or planning roles would otherwise need to infer the business problem from scattered notes, meetings, or assumptions
- the next correct step is structured discovery and requirement validation rather than architecture, implementation planning, or sprint administration

## Participating Roles And Why

- [`Business Analyst`](C:/Users/ericw/Projects/orpheum/roles/business-analyst.md)
  - leads discovery, clarifies the business problem, maps current and future process understanding, verifies requirements, and produces a downstream-ready requirements handoff
- optional [`Product Owner`](C:/Users/ericw/Projects/orpheum/roles/product-owner.md)
  - participates when discovery outputs already need an explicit product-readiness judgment, early value framing, or priority signal before the work should advance into `Project Planning`

## Entry Conditions

- a project idea, problem signal, stakeholder request, or equivalent initiating context exists
- validated discovery does not yet exist or is not yet strong enough to support downstream planning honestly
- the participating role packages are available and treated as the source of truth for role-local workflows
- the scenario is being used as a reusable upstream discovery phase, not as architecture design, sprint planning, or implementation work

## Target Outputs And Exit Conditions

The scenario completes successfully when the downstream discovery package includes:

- explicit business objectives with scope boundaries, stakeholders, success criteria, and confirmation posture
- explicit current-state and future-state process understanding with process needs, rules, exceptions, and operational gaps
- a verified requirements posture with clear traceability to business goals and process needs
- a downstream-ready requirements handoff that names risks, dependencies, unresolved questions, and confirmation limits
- optional product-readiness or early priority framing when discovery alone is not enough to decide whether the work should proceed into planning

Exit condition:

- downstream planning roles can begin `Project Planning` with validated discovery rather than reconstructing the business problem, process context, or requirement basis from kickoff chatter

## Core Sequence

1. Consume the initiating request, notes, transcript, release-driven signal, or other early project context through Business Analyst kickoff work.
2. Clarify the business problem, scope boundaries, stakeholders, success criteria, and confirmation posture.
3. Turn the validated discovery baseline into explicit current-state and future-state process understanding through Business Analyst process-analysis work.
4. Turn the discovery and process context into a verified requirements posture and downstream-ready requirements handoff through Business Analyst requirements-handoff work.
5. Optionally bring in Product Owner framing when discovery outputs already need a clear product-readiness, value, or priority judgment before the work should proceed into planning.
6. Run a required semantic artifact review with the human, artifact by artifact, using Planning Mode or the host environment's nearest equivalent, and stay in that mode until semantic questions, decision changes, and cross-artifact reconciliation are complete.
7. Review the resulting discovery package and decide whether it is ready to feed `Project Planning`, should stay in discovery, or should route elsewhere.
8. Hand the completed discovery package downstream to `Project Planning`.

## Decision Gates And Human Checkpoints

- the business problem must be explicit enough that downstream roles are not forced to guess what need is actually being solved
- process understanding must be explicit enough that requirements are not floating free of operational context
- requirements verification must be explicit before the package is treated as planning-ready
- stakeholder confirmation status must stay visible rather than being implied
- optional Product Owner participation should become explicit when the main unresolved issue is whether the discovered work deserves downstream planning priority
- semantic artifact review is a required checkpoint before closure; the review should happen in Planning Mode or the host environment's nearest equivalent
- human approval remains visible when discovery exposes sensitive tradeoffs, unresolved scope disputes, or trust-boundary-sensitive behavior

## Scenario Constraints And Non-Goals

- This scenario does not replace role-local workflows; it composes them.
- This scenario does not replace `Project Planning`; it feeds it.
- This scenario does not absorb solution architecture, implementation planning, implementation execution, or release handling.
- This scenario does not act as a sprint board, delivery-management framework, or staffing mechanism.
- This scenario should stay reusable across projects and should not be overfit to one discovery meeting style or intake channel.

## Open Questions And Design Gaps

- Repeated usage may show a need for a narrower discovery-triage scenario when many requests are too weak even for full BA kickoff work.
- Repeated usage may show a need for a stronger default rule about when Product Owner should join before discovery is treated as ready for planning.
- Repeated usage may show a need for a sharper route from release-driven feedback into discovery when reprioritization reveals missing business understanding rather than a simple priority change.

## Recommended Next Step

Use the Project Discovery integration map to make kickoff intake, BA artifact flow, optional product-readiness participation, and downstream routing into `Project Planning` explicit.

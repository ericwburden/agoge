# Business Analyst Requirements Handoff

## Purpose

Turn discovery and process analysis into verified business requirements and a downstream-ready handoff package for product, architecture, delivery, or implementation roles.

## When To Use

- The business objective and process context are stable enough to express verified requirements.
- Downstream teams need a clean handoff that preserves business rationale, traceability, risks, and unresolved questions.
- The work is ready to move from BA discovery into planning or implementation preparation.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/business-objectives.md`](D:/Projects/agoge/artifacts/business-objectives.md)
  - an instantiated copy of [`artifacts/process-analysis.md`](D:/Projects/agoge/artifacts/process-analysis.md)
- Optional: additional research, stakeholder clarifications, policy references, and prior specs.

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md) in the target project workspace
  - an instantiated copy of [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) in the target project workspace
- Secondary outputs: clarified acceptance considerations, candidate Allium promotion notes, and downstream decision framing.

## Skills And Tools

- Repo-native synthesis from the Business Analyst artifacts as the default path.
- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when additional evidence or clarification is needed before finalizing requirements.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md) to determine which requirements are actually supported and strengthen the requirements specification artifact.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) to package verified requirements into a downstream-ready handoff with traceability, risks, and decision points.
- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) only after the BA handoff is mature enough to support downstream planning translation.

## Sequence

1. Read the business objectives and process analysis artifacts together and identify which requirements are truly supported by the current discovery evidence.
2. If gaps remain, use `research-documentation` to synthesize any additional local documentation or structured notes needed to clarify the requirement set.
3. Instantiate [`artifacts/requirements-specification.md`](D:/Projects/agoge/artifacts/requirements-specification.md) and [`artifacts/requirements-handoff.md`](D:/Projects/agoge/artifacts/requirements-handoff.md) into the project workspace if working copies do not already exist.
4. Use `requirements-verification` to determine which requirements are confirmed, separate non-requirements and unresolved items, and populate the instantiated requirements specification artifact.
5. Use `handoff-packaging` to build the instantiated requirements handoff artifact with traceability, risks, dependencies, unresolved questions, and downstream decision points.
6. Run [`requirements-specification.check.md`](D:/Projects/agoge/checks/requirements-specification.check.md), [`requirements-handoff.check.md`](D:/Projects/agoge/checks/requirements-handoff.check.md), [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md), and [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) against the instantiated BA chain.
7. If the handoff is mature and the next step is implementation planning, use `spec-to-implementation` as a downstream bridge rather than extending BA discovery further.

## Decision Points

- If a requirement does not trace back to a business objective or process need, treat it as unverified.
- If proposed solutions were mentioned during discovery, keep them separate from confirmed requirements unless they were explicitly validated as requirements.
- If AI-enabled behavior is involved, separate business objectives, acceptance expectations for agent outputs, and AI or agent-specific constraints.
- If the requirement set is stable and behaviorally clear, identify which parts may be ready for Allium promotion.

## Validation

- [`requirements-specification.check.md`](D:/Projects/agoge/checks/requirements-specification.check.md) passes.
- [`requirements-handoff.check.md`](D:/Projects/agoge/checks/requirements-handoff.check.md) passes.
- [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md) passes.
- [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md) passes.
- The workflow does not drift into delivery management or implementation execution.

## Failure Handling

- Stop and ask for clarification if the requirements cannot be verified from the available discovery evidence.
- Do not treat feature suggestions as confirmed requirements without traceability.
- If the handoff would still require downstream roles to rediscover the business problem, keep the BA workflow open and fill the gaps first.
- If the requirements specification check fails, rework the requirement set with `requirements-verification` and use `research-documentation` if additional evidence is needed.
- If the requirements handoff check fails, repackage the artifact with `handoff-packaging` and route back to `requirements-verification` if the underlying requirement set is still unstable.
- If a traceability or boundary check fails, route remediation to the prior BA artifact that introduced the defect rather than hiding it inside the handoff.

## Notes

This workflow is the bridge between Business Analyst discovery and downstream planning. Use [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md) only after the BA artifacts are mature.

# Code Reviewer Analysis

## Purpose

Turn a completed implementation package into an explicit review scope and a durable findings set that independently challenges the change for defects, regressions, architectural drift, and missing validation.

## When To Use

- An implementation package already exists and needs independent review.
- Downstream roles need more than an implementation self-review before trusting the change.
- The review needs to be durable enough that findings do not live only in inline comments or chat.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
  - an instantiated copy of [`artifacts/implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
  - an instantiated copy of [`artifacts/implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
  - an instantiated copy of [`artifacts/implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)
- Expected supporting context:
  - corresponding requirements, architecture, and implementation-planning handoffs when they materially constrain the change
- Optional:
  - diffs, local run outputs, screenshots, logs, defect notes, or specification references

## Outputs

- Primary artifact types:
  - an instantiated copy of [`artifacts/code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md)
  - an instantiated copy of [`artifacts/review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- Secondary outputs: concrete findings, severity and confidence calls, review coverage notes, and explicit requests for remediation or evidence

## Skills And Tools

- [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md) as the default direct-support skill for turning the implementation package into explicit Code Reviewer findings with severity, confidence, location, evidence-basis, and re-review discipline.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) as the default path for synthesizing implementation context, upstream commitments, and local evidence before writing review artifacts.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when review concerns hinge on requirement conformance, acceptance commitments, or unsupported behavioral claims.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when a suspected finding depends on targeted browser or web-application reproduction.
- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when review notes or implementation-session notes need normalization.
- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md) and [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when review depends materially on specification-to-code alignment.

## Sequence

1. Read the implementation package together with the relevant upstream handoffs to understand what was changed and what commitments constrain the review.
2. If local evidence, notes, or upstream context are spread across multiple sources, use `research-documentation` first to synthesize the relevant review frame.
3. Instantiate [`artifacts/code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md) and [`artifacts/review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md) into the project workspace if working copies do not already exist.
4. Populate the review scope artifact with the change boundary, reviewed inputs, conformance anchors, hotspots, evidence sources, and explicit review limits.
5. Review the change for concrete correctness issues, regressions, edge cases, architectural or contract drift, migration risk, trust-boundary-sensitive behavior, and missing or weak validation.
6. Use `review-findings-authoring` to record concrete findings, severity, confidence, affected areas, affected locations when practical, evidence basis, requested remediation, explicit no-finding coverage notes, and re-review triggers in the review findings artifact.
7. Run [`code-review-scope.check.md`](D:/Projects/orpheum/checks/code-review-scope.check.md), [`review-findings.check.md`](D:/Projects/orpheum/checks/review-findings.check.md), [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md), and [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).

## Decision Points

- If review confidence depends on missing implementation context, weak evidence, or unresolved upstream ambiguity, record the limit explicitly instead of inventing certainty.
- If a suspected issue is really a verification gap rather than a code defect, preserve it as a review concern without pretending the review has become the QA verdict.
- If the issue belongs upstream in requirements, architecture, planning, or specification work, route it explicitly rather than disguising it as a purely local code complaint.
- If the review is intentionally partial, state that boundary clearly so later readers do not mistake silence for approval.
- If the location or evidence basis for a finding is too weak to explain clearly, either strengthen it or downgrade the concern rather than leaving a hand-wavy finding in place.

## Validation

- [`code-review-scope.check.md`](D:/Projects/orpheum/checks/code-review-scope.check.md) passes.
- [`review-findings.check.md`](D:/Projects/orpheum/checks/review-findings.check.md) passes.
- [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md) passes.
- [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md) passes.
- The package is ready to feed [`code-reviewer-decision.md`](D:/Projects/orpheum/workflows/code-reviewer-decision.md).

## Failure Handling

- Stop and ask for clarification if the reviewed change or upstream commitments cannot be identified honestly from the available package.
- If a findings or traceability check fails, rework the earlier artifact rather than expecting downstream consumers to infer the missing logic.
- If the review reveals the implementation package is materially incomplete or misleading, route remediation there before escalating review language.

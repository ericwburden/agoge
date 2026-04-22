---
id: code-reviewer-quality-review
kind: workflow
title: Code Reviewer Quality Review
version: 1
summary: Run the Code Reviewer check chain across the review package, confirm whether
  the package is coherent and downstream-usable, and route remediation to the earliest
  artifact that introduced any defect.
role: code-reviewer
inputs:
- code-review-scope
- review-findings
- review-decision
- review-handoff
outputs: []
skills:
- review-findings-authoring
- research-documentation
- handoff-packaging
- requirements-verification
- webapp-testing
- implementation-skill-discovery
checks:
- code-review-scope
- review-findings
- review-decision
- review-handoff
- review-traceability
- code-reviewer-boundary
handoff_to: []
---

# Code Reviewer Quality Review

## Purpose

Run the Code Reviewer check chain across the review package, confirm whether the package is coherent and downstream-usable, and route remediation to the earliest artifact that introduced any defect.

## When To Use

- The review scope, review findings, review decision, and review handoff artifacts all exist.
- The package is about to be treated as a reusable review output for downstream roles.
- A Role Builder or downstream consumer wants to know whether the package is actually coherent and review-complete.

## Inputs

- Required:
  - instantiated copies of [`artifacts/code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md), [`artifacts/review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md), [`artifacts/review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md), and [`artifacts/review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md)
- Expected supporting context:
  - the corresponding implementation package artifacts and relevant upstream handoffs
- Optional:
  - additional evidence sources, review notes, or specification references

## Outputs

- Primary outputs: pass or fail results for the Code Reviewer check chain and explicit remediation routing
- Secondary outputs: strengthened review artifacts, narrowed open questions, and clearer downstream readiness

## Skills And Tools

- [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md) when repeated quality failures are concentrated in finding phrasing, severity or confidence calibration, location precision, evidence basis, or re-review framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when failures still depend on unsynthesized review, implementation, or upstream context.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when failures are concentrated in downstream packaging clarity.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when traceability back to requirements or acceptance commitments is the main weakness.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when repeated findings depend on missing reproduction or application evidence.
- [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) only if recurring implementation-package weakness is preventing review quality and the repo needs to decide whether a stronger upstream implementation skill is now warranted.

## Sequence

1. Read the full review package together with the implementation package and the upstream handoffs that materially constrain the reviewed change.
2. Run [`code-review-scope.check.md`](D:/Projects/orpheum/checks/code-review-scope.check.md), [`review-findings.check.md`](D:/Projects/orpheum/checks/review-findings.check.md), [`review-decision.check.md`](D:/Projects/orpheum/checks/review-decision.check.md), [`review-handoff.check.md`](D:/Projects/orpheum/checks/review-handoff.check.md), [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md), and [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).
3. If failures cluster around fragmented local context, use `research-documentation` to synthesize the missing context before rewriting the affected artifact.
4. Route each failure to the earliest artifact that introduced it:
   - review-scope defects go back to [`code-reviewer-analysis.md`](D:/Projects/orpheum/workflows/code-reviewer-analysis.md)
   - findings defects go back to [`code-reviewer-analysis.md`](D:/Projects/orpheum/workflows/code-reviewer-analysis.md)
   - decision defects go back to [`code-reviewer-decision.md`](D:/Projects/orpheum/workflows/code-reviewer-decision.md)
   - handoff defects go back to [`code-reviewer-handoff.md`](D:/Projects/orpheum/workflows/code-reviewer-handoff.md)
5. If findings defects cluster around severity calibration, location precision, evidence basis, or re-review framing, use `review-findings-authoring` before rewriting the findings artifact.
6. Re-run the full check chain after remediation before treating the package as downstream-ready.

## Decision Points

- If the package fails because the implementation package is materially weak, route that defect upstream instead of trying to hide it inside review prose.
- If the package fails because verification-sensitive concerns are being overstated or understated, preserve the distinction between code review and QA rather than collapsing them together.
- If the package passes with explicit conditions, keep those conditions visible instead of rewriting the package into a soft approval.
- If repeated failures show the current repo skill set is not enough to support review quality cleanly, record that explicitly in the skill-sourcing note instead of leaving it as implicit friction.

## Validation

- All six Code Reviewer checks pass.
- The review package is coherent, explicit, and downstream-usable.
- The package preserves the distinction between implementation review, broader verification, and release-adjacent work.

## Failure Handling

- Stop and ask for clarification if the package cannot be reviewed honestly because the reviewed change or upstream commitments are not identifiable.
- If failures continue after local remediation, record the residual gap explicitly and route it to the earliest artifact or role package that needs strengthening.
- If repeated quality-review failures point to a repo-support gap rather than a one-off artifact defect, record that in the Code Reviewer skill-sourcing note for future hardening work.

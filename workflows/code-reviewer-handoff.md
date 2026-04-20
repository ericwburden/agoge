# Code Reviewer Handoff

## Purpose

Package the reviewed code-review outputs into a downstream-ready handoff for implementation, verification, release-adjacent, or human approval consumers.

## When To Use

- A review decision already exists and the review package needs a downstream-facing summary.
- Downstream roles need to know which findings still matter, what the approval posture is, and what should happen next.
- The review package needs to move beyond raw findings or a single approval label.

## Inputs

- Required:
  - an instantiated copy of [`artifacts/review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md)
  - the corresponding instantiated copies of [`artifacts/code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md) and [`artifacts/review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- Expected supporting context:
  - the corresponding implementation package artifacts
- Optional:
  - verification notes, release notes, approval notes, or additional review references

## Outputs

- Primary artifact type: an instantiated copy of [`artifacts/review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md) in the target project workspace
- Secondary outputs: downstream-ready summary of review posture, active findings, follow-up owners, and next consumers

## Skills And Tools

- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) as the default path for downstream-ready packaging once the review decision is explicit.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the handoff still depends on synthesizing multiple review or implementation sources.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when downstream consumers need stronger requirement-grounded framing for review conclusions.

## Sequence

1. Read the review decision together with the review scope, review findings, and implementation package.
2. If the handoff still depends on unsynthesized context, use `research-documentation` first.
3. Instantiate [`artifacts/review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md) into the project workspace if a working copy does not already exist.
4. Populate the review handoff artifact with the reviewed change summary, review package included, current approval posture, any revision or scope limits on that posture, key findings to carry forward, required follow-up, owners, verification or release watchouts, re-review triggers, upstream routing notes, and recommended next consumer.
5. Run [`review-handoff.check.md`](D:/Projects/orpheum/checks/review-handoff.check.md), [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md), and [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).

## Decision Points

- If the review is blocked, do not package the handoff as though downstream work can proceed normally.
- If follow-up ownership is unclear, make that ambiguity explicit rather than smoothing it over.
- If the most important downstream concern is verification-sensitive rather than implementation-sensitive, preserve that distinction in the handoff.
- If the review uncovered an upstream defect, route it explicitly rather than burying it in the follow-up list.
- If downstream trust depends on a fresh revision or evidence update, say so explicitly rather than leaving downstream consumers to infer that re-review is required.

## Validation

- [`review-handoff.check.md`](D:/Projects/orpheum/checks/review-handoff.check.md) passes.
- [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md) passes.
- [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md) passes.
- The package is ready for downstream implementation remediation, QA or verification, release-adjacent handling, or human approval.

## Failure Handling

- Stop and ask for clarification if the current review posture or next consumer cannot be identified honestly.
- If the handoff check fails, rework the handoff rather than expecting downstream roles to infer what matters.
- If the real defect is earlier in the findings or decision artifact, repair that earlier stage instead of decorating the handoff.

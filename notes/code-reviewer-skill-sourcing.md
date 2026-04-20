# Code Reviewer Skill Sourcing

## Purpose

Capture which local skills already support the [`Code Reviewer`](D:/Projects/orpheum/roles/code-reviewer.md) role, which gaps are intentionally being handled inside the first package shape, and which external patterns informed the first-pass support decisions.

## External Pattern Summary

The first-pass Code Reviewer package is shaped by four recurring external patterns:

- code review is a distinct quality function from implementation and from broader QA or system-level verification
- durable review output needs more than a raw approval label or scattered inline comments
- review quality improves when comments are concrete, line- or area-specific, and severity-aware
- repository-specific review guidance often matters, but the core reusable method is still scope framing, findings discipline, decision clarity, and downstream handoff

These patterns are visible in:

- GitHub's pull request review model, which separates comment, approve, and request-changes outcomes and anchors review around concrete review feedback rather than generic status language
- OpenHands' automated code-review guidance, which emphasizes repository-specific review instructions, changed-file context, and concrete inline findings rather than broad summaries alone
- ChatDev's separation of reviewer and tester responsibilities, which treats static review and broader testing as distinct stages in the development chain

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing review notes, implementation walkthrough notes, or defect-discussion notes before review artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when review depends on external standards, platform guidance, security notes, or integration references that should be cited explicitly.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md)
  - Useful when a suspected review finding depends on targeted browser or application reproduction rather than code inspection alone.

### Useful With Existing Local-Markdown Positioning

- [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md)
  - This is now the dedicated repo-native direct-support skill for Code Reviewer findings work.
  - It owns the most role-specific repeatable method in the package: severity and confidence calibration, affected-location discipline, evidence-basis discipline, blocking versus non-blocking distinction, and re-review trigger framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Useful as the core local-Markdown synthesis skill because review work often depends on combining implementation context, upstream commitments, local code signals, and evidence notes into one durable record.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md)
  - Useful when review concerns hinge on requirement conformance, acceptance commitments, or unsupported behavior claims and the reviewer needs a stronger grounding path than code commentary alone.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Useful for packaging the completed review decision into a downstream-ready handoff without inventing a new handoff-specific skill too early.

### GitHub-Adjacent Support

- [`gh-address-comments`](D:/Projects/orpheum/skills/gh-address-comments/SKILL.md)
  - Useful after review, when implementation owners need to address submitted feedback.
  - This is downstream of the Code Reviewer role, not the role's main authoring path.
- [`gh-fix-ci`](D:/Projects/orpheum/skills/gh-fix-ci/SKILL.md)
  - Useful when review posture is materially affected by failing CI evidence, but it is still adjacent rather than core support for the review package itself.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when an existing behavioral specification materially constrains review judgment.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when review confidence depends on checking whether the code has drifted from a governing specification.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful only when review uncovers an upstream behavioral gap that needs clarification rather than local reviewer improvisation.

## Why Existing Skills Are Enough For V1

### The missing problem is review packaging and discipline, not "how to comment"

This repository already operates inside a coding-capable agent environment that can inspect code, diffs, tests, and local evidence.

The main missing need for a Code Reviewer role is not a generic "review the code" capability. It is a repeatable way to:

- define what was actually reviewed
- record findings in a concrete and severity-aware way
- separate approval posture from the findings list
- preserve review independence from implementation ownership
- hand review output downstream without forcing other roles to reconstruct it from comments

The package therefore spends its reusable structure on review scope, findings discipline, decision clarity, and downstream packaging.

### Durable review output can live in artifacts and checks first

The major new requirement is not merely "produce comments." It is:

- frame review scope honestly
- preserve code-to-finding traceability
- distinguish blocking and non-blocking concerns
- make approval posture explicit
- hand the review downstream in a durable way

Those concerns are being handled in v1 through:

- [`code-review-scope.md`](D:/Projects/orpheum/artifacts/code-review-scope.md)
- [`review-findings.md`](D:/Projects/orpheum/artifacts/review-findings.md)
- [`review-decision.md`](D:/Projects/orpheum/artifacts/review-decision.md)
- [`review-handoff.md`](D:/Projects/orpheum/artifacts/review-handoff.md)
- [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md)
- [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md)

That keeps the package operational without prematurely inventing a custom review skill that might bundle the wrong concerns together.

The hardening pass on this package also tightened two common failure modes directly in the artifacts and checks:

- findings now require stronger affected-location and evidence-basis discipline
- approval posture now has to stay explicitly scoped to the reviewed revision or review boundary so "approved" does not silently expand into blanket merge or release permission

### A generic repo-native review skill would still be premature

External patterns suggest a future review-specific skill may eventually be useful, especially if repeated work shows consistent friction around:

- repository-specific review rubrics
- broader approval conventions across repositories

But the repo does not yet have enough repeated Code Reviewer usage to know whether that future skill should be:

- review-decision support
- repository-specific review rubric support
- or a tighter bridge between code review and downstream verification

The main direct-support gap has now been closed by adding [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md).

The remaining deferred question is not "how should the reviewer author findings?" but "does this repo need a stronger repository-specific review rubric or decision-support skill?"

## Role-Specific Design Decisions

### Keep Code Reviewer separate from Implementation Engineer

The package intentionally keeps implementation ownership and independent review ownership separate.

The Code Reviewer is allowed to:

- review the implemented slice
- record concrete findings
- decide whether the review posture is approved, conditional, or blocked
- package the review for downstream use

The Code Reviewer is not supposed to:

- implement the fix as the default response
- convert review artifacts into implementation notes
- absorb ownership of the reviewed change

This boundary is enforced in the role definition and in [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).

### Keep Code Reviewer separate from QA / Verification Lead

The package intentionally keeps code review distinct from broader evidence-based verification.

The Code Reviewer is allowed to:

- identify missing validation or weak evidence as review concerns
- call out regression risk, drift, or unsupported behavior claims
- tell downstream verification where extra attention is needed

The Code Reviewer is not supposed to:

- act as the final verification authority
- replace the verification strategy or evidence review package
- present review approval as though it were release readiness

This boundary is enforced in the role definition and in [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- code review scope
- review findings
- review decision
- review handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by the existing SDLC roles and keeps the role easier to validate and hand off.

## Allium Relationship

The Code Reviewer role should treat Allium as an upstream behavioral review anchor, not as the format for review artifacts.

That means:

- consume existing Allium when it materially constrains behavior under review
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when review confidence depends on checking for specification-to-code drift
- route unstable behavior back upstream rather than inventing it in review language

The review artifacts themselves remain Markdown-first in this repository.

## External Sources

- [GitHub Docs: About pull request reviews](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-request-reviews)
- [OpenHands docs: Automated code review by file-based agents](https://docs.openhands.dev/sdk/guides/agent-file-based)
- [ChatDev ACL paper](https://aclanthology.org/2024.acl-long.810.pdf)
- [ChatDev README](https://raw.githubusercontent.com/OpenBMB/ChatDev/main/README.md)

## Implementation Decision

For this repository, the right move is:

- keep the Code Reviewer role repo-neutral
- use [`review-findings-authoring`](D:/Projects/orpheum/skills/review-findings-authoring/SKILL.md) as the direct-support skill for the most role-specific part of review execution
- reuse the existing synthesis, requirement-grounding, testing, and handoff skills where they already express the right method
- embed scope discipline, findings discipline, decision clarity, traceability, and boundary control directly in the artifacts, workflows, and checks
- keep the first pass independent from implementation ownership and broader QA authority
- defer any additional repo-native review skills until repeated usage shows which narrower remaining review support pattern is actually missing

## Follow-Up Considerations

If future Code Reviewer work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a repository-specific review-rubric skill for recurring local risk patterns
- a tighter bridge skill between review findings and downstream verification focus areas
- a review-decision support skill if cross-project approval-limit or condition framing remains inconsistent

The standard trigger for evaluating those additions should be repeated failure or remediation patterns observed during [`code-reviewer-quality-review.md`](D:/Projects/orpheum/workflows/code-reviewer-quality-review.md).

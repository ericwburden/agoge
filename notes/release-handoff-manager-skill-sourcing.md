# Release / Handoff Manager Skill Sourcing

## Purpose

Capture which local skills already support the [`Release / Handoff Manager`](D:/Projects/orpheum/roles/release-handoff-manager.md) role, which gaps are intentionally being handled inside the first package shape, and which external patterns informed the first-pass support decisions.

## External Pattern Summary

The first-pass Release / Handoff Manager package is shaped by three recurring external patterns:

- release work is downstream of reviewed implementation, review, and verification outputs rather than a substitute for them
- effective release or handoff packaging depends on explicit conditions, approvals, and environment constraints rather than on a generic "ready" label
- multi-agent and operational systems benefit from explicit handoff routing so downstream humans or teams know what they own next

These patterns are visible in:

- GitHub's release and deployment model, which separates release notes from deployment protection rules and allows release packaging, required reviewers, wait timers, and environment constraints to remain explicit rather than implicit
- AutoGen's handoff design pattern, which treats handoff as a first-class routing mechanism with explicit downstream ownership rather than a hidden continuation of the same task
- broader agentic SDLC patterns, where release or adoption work follows implementation, review, and verification rather than replacing them

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing release coordination notes, operational review notes, approval notes, or adoption notes before release artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when release packaging depends on external platform guidance, compliance notes, environment constraints, or customer-facing release-note expectations that should be sourced explicitly.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md)
  - Useful when release posture depends on targeted validation or demonstration evidence that should be understood alongside the existing review and verification package.

### Useful With Existing Local-Markdown Positioning

- [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md)
  - This is now the dedicated repo-native direct-support skill for Release / Handoff Manager packaging work.
  - It owns the most role-specific repeatable method in the package: candidate framing, release posture wording, approval-limit clarity, rollout caveat packaging, and re-approval trigger framing.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Useful as the core local-Markdown synthesis skill because release packaging often depends on combining implementation, review, verification, operational, and approval context into one durable package.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Useful as the main direct-support skill for packaging validated delivery outputs into a downstream-ready release or adoption handoff.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md)
  - Useful when release framing depends heavily on validated behavior, acceptance commitments, or requirement-sensitive rollout constraints.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when existing behavioral specifications materially constrain rollout posture or downstream release communication.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when release confidence depends on checking whether the delivered system may have drifted from a governing specification.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful only when release packaging uncovers an upstream behavioral gap that needs clarification rather than workaround packaging language.

## Why Existing Skills Are Enough For V1

### The missing problem is release packaging discipline, not deployment execution

This repository already has roles for implementation, review, and verification. The main missing need for a Release / Handoff Manager role is not a generic deployment skill. It is a repeatable way to:

- define the release candidate clearly
- make current release posture explicit
- preserve rollout and operational caveats
- package the current state for downstream operators, adopters, or approvers without rewriting the upstream engineering package

The package therefore spends its reusable structure on candidate framing, decision clarity, rollout notes, and downstream handoff rather than on deployment execution.

### Release packaging can be expressed through artifacts and checks first

The major new requirement is not merely "write release notes." It is:

- frame release scope honestly
- preserve traceability back to implementation, review, and verification
- make approval limits and conditions explicit
- preserve operational and rollout caveats
- hand the package downstream in a durable way

Those concerns are being handled in v1 through:

- [`release-candidate-summary.md`](D:/Projects/orpheum/artifacts/release-candidate-summary.md)
- [`release-readiness-decision.md`](D:/Projects/orpheum/artifacts/release-readiness-decision.md)
- [`rollout-and-operations-notes.md`](D:/Projects/orpheum/artifacts/rollout-and-operations-notes.md)
- [`release-handoff.md`](D:/Projects/orpheum/artifacts/release-handoff.md)
- [`release-traceability.check.md`](D:/Projects/orpheum/checks/release-traceability.check.md)
- [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md)

That keeps the package operational while avoiding broader release-specific specialization before the repo has enough repeated usage to justify it.

The hardening pass on this package also tightened a common failure mode directly in the artifacts and checks:

- release posture now has to distinguish between "ready for downstream release handling" and "already authorized for immediate deployment" when those are not the same thing

### A generic repo-native release skill would still be premature

External patterns suggest a future release-specific skill may eventually be useful, especially if repeated work shows consistent friction around:

- repository-specific release-rubric support
- decision-support for approvals and waivers across environments
- broader deployment-authorization conventions across repositories

But the repo does not yet have enough repeated Release / Handoff Manager usage to know whether that future skill should be:

- release-decision support
- repository-specific release-rubric support
- or a stronger bridge between verification posture and deployment-authorization guidance

The main direct-support gap has now been closed by adding [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md).

The remaining deferred question is not "how should the release package be authored?" but "does this repo need a stronger repository-specific release rubric or release-decision support skill?"

## Role-Specific Design Decisions

### Keep Release / Handoff Manager separate from Implementation Engineer and Code Reviewer

The package intentionally keeps engineering ownership and release packaging ownership separate.

The Release / Handoff Manager is allowed to:

- package the reviewed candidate
- record the current release posture
- preserve rollout and operational caveats
- package the release handoff for downstream use

The Release / Handoff Manager is not supposed to:

- implement fixes as the default response
- convert release artifacts into a new code review or implementation package
- absorb ownership of the underlying change set

This boundary is enforced in the role definition and in [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).

### Keep Release / Handoff Manager separate from QA / Verification Lead

The package intentionally keeps release packaging distinct from broader evidence-based verification.

The Release / Handoff Manager is allowed to:

- preserve the current verification posture and its conditions
- call out where downstream release confidence still depends on verification-sensitive areas
- tell downstream operators or approvers what evidence limits still matter

The Release / Handoff Manager is not supposed to:

- act as the final verification authority
- replace the verification strategy or evidence review package
- invent release confidence that the upstream evidence does not support

This boundary is enforced in the role definition and in [`release-handoff-manager-boundary.check.md`](D:/Projects/orpheum/checks/release-handoff-manager-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- release candidate summary
- release readiness decision
- rollout and operations notes
- release handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by the existing SDLC roles and keeps the role easier to validate and hand off.

## Allium Relationship

The Release / Handoff Manager role should treat Allium as an upstream behavioral release anchor, not as the format for release artifacts.

That means:

- consume existing Allium when it materially constrains rollout or downstream release communication
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when release confidence depends on checking for specification-to-code drift
- route unstable behavior back upstream rather than inventing it in release language

The release artifacts themselves remain Markdown-first in this repository.

## External Sources

- [GitHub Docs: Automatically generated release notes](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes)
- [GitHub Docs: Reviewing deployments](https://docs.github.com/en/actions/managing-workflow-runs-and-deployments/managing-deployments/reviewing-deployments)
- [GitHub Docs: Deployments and environments](https://docs.github.com/en/actions/reference/deployments-and-environments)
- [AutoGen handoffs pattern](https://microsoft.github.io/autogen/dev/user-guide/core-user-guide/design-patterns/handoffs.html)

## Implementation Decision

For this repository, the right move is:

- keep the Release / Handoff Manager role repo-neutral
- use [`release-readiness-packaging`](D:/Projects/orpheum/skills/release-readiness-packaging/SKILL.md) as the direct-support skill for the most role-specific part of release execution
- reuse the existing synthesis, handoff, requirement-grounding, and testing skills where they already express the right method
- embed candidate framing, release posture, rollout caveats, traceability, and boundary control directly in the artifacts, workflows, and checks
- keep the first pass focused on release packaging rather than deployment execution
- defer any additional repo-native release skills until repeated usage shows which narrower remaining release support pattern is actually missing

## Follow-Up Considerations

If future Release / Handoff Manager work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a repository-specific release-rubric skill for recurring local release or approval patterns
- a release-decision support skill if cross-project approval-limit and waiver framing remain inconsistent
- a stronger bridge skill if release packaging repeatedly needs to connect verification posture to deployment-authorization decisions

The standard trigger for evaluating those additions should be repeated failure or remediation patterns observed during [`release-handoff-manager-quality-review.md`](D:/Projects/orpheum/workflows/release-handoff-manager-quality-review.md).

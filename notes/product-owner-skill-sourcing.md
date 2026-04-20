# Product Owner Skill Sourcing

## Purpose

Capture which local skills already support the [`Product Owner`](D:/Projects/orpheum/roles/product-owner.md) role, which gaps are intentionally being handled inside the first package shape, and which external patterns informed the first-pass support decisions.

## External Pattern Summary

The first-pass Product Owner package is shaped by three recurring external patterns:

- product ownership is accountable for maximizing value rather than merely administering tickets
- effective product ownership requires one explicit decision posture for backlog and goal direction rather than committee-style priority drift
- product ownership must connect validated customer and business needs to team-facing execution without absorbing all of product management, project management, or technical design

These patterns are visible in:

- the [Scrum Guide](https://scrumguides.org/docs/scrumguide/v2020/2020-Scrum-Guide-US.pdf), which makes the Product Owner accountable for maximizing product value, developing and communicating the Product Goal, and ordering and maintaining a transparent Product Backlog
- Atlassian's explanation of [Product Owner responsibilities](https://www.atlassian.com/agile/product-management/product-owner) and [Scrum roles](https://www.atlassian.com/agile/scrum/roles), which emphasizes backlog management, clear direction, stakeholder management, and the distinction between product ownership and broader product management or Scrum Master work
- the [Scaled Agile Framework Product Owner page](https://framework.scaledagile.com/product-owner/), which reinforces the Product Owner as the voice of the customer on the team and highlights the alignment boundary between Product Owner and broader product management

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing stakeholder sessions, prioritization workshops, release-learnings reviews, and product decision meetings before product artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when product direction depends on external market, platform, or standards context that should be sourced explicitly.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Useful for synthesizing validated discovery, product feedback, delivery evidence, roadmap context, and decision notes before product-direction or review work begins.
- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md)
  - This is now the dedicated repo-native direct-support skill for Product Owner direction and prioritization work.
  - It owns the most role-specific repeatable method in the package: tradeoff framing, acceptance guardrails, deferred-scope discipline, reprioritization triggers, and clear separation between product-priority readiness and downstream commitment.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md)
  - Useful as the core grounding skill because Product Owner work still depends heavily on separating validated needs and acceptance-sensitive constraints from assumptions, pressure, or incomplete discovery.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Useful for packaging reviewed product outputs into a downstream-ready handoff without inventing a new handoff skill too early.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when existing behavioral specifications materially constrain product direction or acceptance framing.
- [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md)
  - Useful when mature product direction or acceptance intent should be sharpened into clearer behavioral commitments.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful when prioritization or review work reveals that an existing specification needs refinement rather than workaround language in product artifacts.

## New Repo-Native Skills Added

- [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md)
  - Added because the role needed a dedicated default path for product tradeoff framing, acceptance-guardrail discipline, deferred-scope handling, and reprioritization logic.

The current package is using the existing local-Markdown grounding, synthesis, and handoff skills first, with role-specific behavior embedded in:

- the [`Product Owner`](D:/Projects/orpheum/roles/product-owner.md) role definition
- the Product Owner artifact set in [`artifacts/`](D:/Projects/orpheum/artifacts)
- the Product Owner workflow set in [`workflows/`](D:/Projects/orpheum/workflows)
- the Product Owner check set in [`checks/`](D:/Projects/orpheum/checks)

## Why Existing Skills Were Enough For V1

### `requirements-verification` already covers the main grounding method

The Product Owner role needed a repeatable way to turn validated discovery, feedback, and delivery learnings into:

- product direction
- explicit priorities
- acceptance-oriented guidance
- tradeoff framing
- conditions and downstream decision routing

`requirements-verification` already provides most of the underlying grounding method for separating supported product commitments from assumptions and open questions in local Markdown form.

The role package therefore keeps using that grounding skill, but now pairs it with a dedicated Product Owner authoring skill for the most role-specific method.

### Product review can be expressed through artifacts and checks first

The main new requirement was not merely "manage a backlog." It was:

- define product direction explicitly
- preserve traceability back to validated needs and evidence
- make tradeoffs and deferred scope visible
- review the current product posture explicitly
- package the result for downstream roles

Those concerns are being handled in v1 through:

- [`product-direction.md`](D:/Projects/orpheum/artifacts/product-direction.md)
- [`backlog-prioritization.md`](D:/Projects/orpheum/artifacts/backlog-prioritization.md)
- [`product-decision-review.md`](D:/Projects/orpheum/artifacts/product-decision-review.md)
- [`product-handoff.md`](D:/Projects/orpheum/artifacts/product-handoff.md)
- [`product-traceability.check.md`](D:/Projects/orpheum/checks/product-traceability.check.md)
- [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md)

That keeps the package operational while avoiding broader product-governance specialization before usage patterns are clearer.

The hardening pass on this package also tightened a common failure mode directly in the artifacts and checks:

- product-priority readiness now has to stay distinct from implementation commitment, sprint commitment, or release approval when those are not the same thing
- product-level acceptance framing now has to stay distinct from full behavioral specification or verification planning when those are not the same thing

### A broader product-governance or backlog-admin skill would still be premature duplication

The existing [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture.

The existing [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) skill already expresses the right grounding posture.

The new [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md) skill closes the main direct-support gap.

The remaining deferred question is not "how should Product Owner direction be authored?" but "does this repo need a stronger repository-specific roadmap-governance or delivery-commitment skill?"

## Role-Specific Design Decisions

### Keep Product Owner separate from Business Analyst

The package intentionally keeps discovery validation and ongoing product ownership separate.

The Product Owner is allowed to:

- decide what should be prioritized next
- synthesize stakeholder and delivery inputs into one product posture
- define acceptance-oriented guardrails
- package product direction for downstream solutioning and planning

The Product Owner is not supposed to:

- replace discovery interviews or process analysis as the default response
- treat unvalidated stakeholder requests as equivalent to requirements
- absorb the full Business Analyst lifecycle

This boundary is enforced in the role definition and in [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).

### Keep Product Owner separate from Solution Architect and Technical Planner

The package intentionally keeps product judgment distinct from technical design and execution planning.

The Product Owner is allowed to:

- decide what matters most
- decide what outcomes and guardrails should shape the work
- decide what can wait

The Product Owner is not supposed to:

- decide the technical solution structure
- define implementation slices or sprint commitments
- absorb day-to-day delivery administration

This boundary is enforced in the role definition and in [`product-owner-boundary.check.md`](D:/Projects/orpheum/checks/product-owner-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- product direction
- backlog prioritization
- product decision review
- product handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by the existing SDLC roles and keeps the role easier to validate and hand off.

## Allium Relationship

The Product Owner role should treat Allium as a behavioral product anchor, not as the format for product artifacts.

That means:

- consume existing Allium when it materially constrains direction or acceptance framing
- use [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md) when mature product intent should become clearer behavioral commitments
- use [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when prioritization or review reveals a real specification gap
- route immature or contradictory behavior back upstream rather than inventing policy in product prose

The product artifacts themselves remain Markdown-first in this repository.

## Implementation Decision

For this repository, the right move is:

- keep the Product Owner role repo-neutral
- use [`product-priority-framing`](D:/Projects/orpheum/skills/product-priority-framing/SKILL.md) as the direct-support skill for the most role-specific part of Product Owner execution
- reuse the existing grounding, synthesis, and handoff skills where they already express the right method
- embed product direction, priority posture, review discipline, traceability, and boundary control directly in the artifacts, workflows, and checks
- keep the role focused on accountable product judgment rather than project administration, technical design, or release operations

## Follow-Up Considerations

If future Product Owner work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a repository-specific roadmap-governance skill if recurring local portfolio or release-horizon conventions emerge
- a delivery-commitment support skill if repeated packages need stronger downstream commitment framing across teams
- a stronger bridge skill if repeated release learnings need more structured intake into product reprioritization

For now, those concerns are handled adequately by the current artifact chain and existing skills.

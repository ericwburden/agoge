# Security / Compliance Specialist Skill Sourcing

## Purpose

Capture which local skills support the [`Security / Compliance Specialist`](D:/Projects/orpheum/roles/security-compliance-specialist.md) role, which gaps are still intentionally deferred, and which external patterns informed the package's support decisions.

## External Pattern Summary

The Security / Compliance Specialist package is shaped by three recurring external patterns:

- secure delivery work needs explicit risk, control, and evidence framing rather than generic "security matters" language
- compliance-sensitive and high-assurance work depends on traceable obligations, defined controls, and clear approval limits rather than on informal good intentions
- security and compliance work should constrain downstream design, implementation, verification, and release without silently replacing those roles

These patterns are visible in:

- NIST's [Secure Software Development Framework (SSDF)](https://csrc.nist.gov/projects/ssdf), which organizes secure software work around preparing the organization, protecting software, producing well-secured software, and responding to vulnerabilities through explicit practices rather than ad hoc reminders
- CISA's [Secure by Design](https://www.cisa.gov/securebydesign) guidance, which emphasizes secure defaults, burden reduction for customers, and explicit responsibility for reducing exploitable risk in software systems
- OWASP's [Software Assurance Maturity Model (SAMM)](https://owaspsamm.org/model/), which reinforces that governance, design, implementation, verification, and operations all need structured security practices and measurable expectations rather than one late-stage review

## Local Skill Support

### Direct Support

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md)
  - Preferred direct-support skill for turning reviewed delivery and obligation context into explicit controls, evidence expectations, compensating controls, exception posture, approval limits, and re-review discipline.

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing audit notes, security review sessions, stakeholder meetings, and obligation-review workshops before artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when obligation framing depends on external platform guidance, standards, or regulatory context that should be sourced explicitly.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Useful for synthesizing delivery, policy, vendor, obligation, and operational context before scope or review work begins.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md)
  - Useful when security/compliance posture depends heavily on validated requirements, acceptance-sensitive constraints, or product claims that should be grounded explicitly.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Useful for packaging reviewed security/compliance outputs into a downstream-ready handoff without inventing a new handoff skill too early.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when existing behavioral specifications materially constrain risk, policy, or compliance posture.
- [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md)
  - Useful when mature policy-sensitive intent should become clearer behavioral commitments.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful when security or compliance review reveals that an existing specification needs refinement.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when confidence depends on checking for specification-to-implementation drift in a security-sensitive area.

## New Repo-Native Skills Added

- [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md)
  - Added as the direct-support skill for the role's most specific repeated method: control mapping, evidence framing, compensating-control discipline, exception handling, approval-limit wording, and re-review logic.

The package still reuses the existing local-Markdown synthesis, grounding, and handoff skills alongside that new role-specific skill.

## Why Existing Skills Were Enough For V1

### `research-documentation` already covers the main synthesis method

The Security / Compliance Specialist role needed a repeatable way to turn reviewed delivery context, obligations, and operational assumptions into:

- assessment scope
- control expectations
- evidence needs
- explicit review posture
- downstream handoff guidance

`research-documentation` already provides most of the underlying method for combining distributed local context into a durable Markdown package.

The role package therefore adapts that skill into a tighter security/compliance operating model instead of adding a new specialist skill before real usage proves the gap.

### Security/compliance review can be expressed through artifacts and checks first

The main new requirement was not merely "do a security review." It was:

- define scope and obligations explicitly
- preserve traceability back to reviewed delivery context
- map controls and evidence expectations
- review the posture explicitly
- package the result for downstream roles

Those concerns are being handled in v1 through:

- [`security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md)
- [`controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md)
- [`security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md)
- [`security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md)
- [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md)
- [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md)

That keeps the package operational without prematurely inventing a broader security/compliance skill before usage patterns are clearer.

### The main repo-native seam is now explicit, while broader security specialization still stays deferred

The new [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) skill captures the part of the role that was most likely to drift in repeated usage:

- explicit control mapping
- evidence expectation authoring
- compensating-control and exception discipline
- approval-limit wording
- re-review posture

That closes the biggest support gap behind the earlier `conditionally ready` judgment without overreaching into legal analysis, technical implementation, or release operations.

### Broader security/compliance specialization would still be premature duplication

The existing [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) skill already expresses the right synthesis posture.

The existing [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture.

The package still does not need a broad policy-writing, legal-interpretation, control-implementation, or deployment-authorization skill.

## Role-Specific Design Decisions

### Keep Security / Compliance Specialist separate from architecture, implementation, and release roles

The package intentionally keeps risk and obligation framing separate from technical implementation and operational execution.

The Security / Compliance Specialist is allowed to:

- define relevant risks and obligations
- make control and evidence expectations explicit
- review the current posture
- package downstream guidance
- make compensating controls, exceptions, and waiver posture explicit without pretending those are self-resolving approvals

The Security / Compliance Specialist is not supposed to:

- implement the controls as the default response
- own architecture or implementation decisions directly
- absorb release execution or operational authorization
- let downstream-ready security/compliance packaging be mistaken for final policy, deployment, or operational approval

This boundary is enforced in the role definition and in [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).

### Keep Security / Compliance Specialist separate from legal or audit authority

The package intentionally stops short of formal legal sign-off, formal audit sign-off, or blanket approval authority.

The goal is explicit risk, obligation, and control framing, not external counsel substitution.

This boundary is enforced in the role definition and in [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- security/compliance scope
- controls and evidence matrix
- security/compliance review
- security/compliance handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by the existing SDLC roles and keeps the role easier to validate and hand off.

## Allium Relationship

The Security / Compliance Specialist role should treat Allium as a behavioral security/compliance anchor, not as the format for its artifacts.

That means:

- consume existing Allium when it materially constrains policy-sensitive or trust-boundary-sensitive behavior
- use [`distill`](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md) when mature security/compliance intent should become clearer behavioral commitments
- use [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when review reveals a real policy or approval-behavior gap
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when confidence depends on checking for behavioral drift

The artifacts themselves remain Markdown-first in this repository.

## Implementation Decision

For this repository, the right move is:

- keep the Security / Compliance Specialist role repo-neutral
- use [`security-controls-and-exceptions`](D:/Projects/orpheum/skills/security-controls-and-exceptions/SKILL.md) as the direct-support method for explicit control and exception framing
- reuse the existing synthesis, grounding, and handoff skills around that new direct-support skill
- keep scope framing, review discipline, traceability, and boundary control embedded directly in the artifacts, workflows, and checks
- keep the role focused on explicit risk and obligation framing rather than legal sign-off, control implementation, or release operations

## Follow-Up Considerations

If future Security / Compliance Specialist work becomes repetitive or exposes consistent friction beyond the new direct-support skill, the next likely additions would be:

- a stronger waiver-and-exception specialization if repeated work still shows weak approval-limit or exception handling beyond the current skill
- a stronger bridge skill if repeated security/compliance findings need more structured intake into architecture or release decisions

For now, the main role-specific support gap is closed.

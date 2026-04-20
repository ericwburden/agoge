---
name: security-controls-and-exceptions
description: Frame Security / Compliance Specialist control expectations, evidence needs, compensating controls, exceptions, waivers, approval limits, and re-review triggers from reviewed delivery and obligation context; use when security/compliance work needs explicit control mapping and exception discipline without drifting into legal sign-off, technical implementation, QA authority, or release execution.
---

# Security Controls And Exceptions

Turn reviewed delivery context, obligations, and risk posture into a coherent Security / Compliance Specialist package that downstream roles can use without reconstructing control logic, exception rationale, or approval limits from scattered notes.

For this repository, this is the preferred direct-support skill for the Security / Compliance Specialist role when the work needs explicit control mapping, evidence framing, compensating-control discipline, exception or waiver handling, and re-review logic rather than only general synthesis or handoff packaging.

## Quick start

1. Read the reviewed delivery and obligation context together.
2. Define the actual assessment boundary, control horizon, and approval limits.
3. Make required controls, evidence expectations, and ownership explicit.
4. Make compensating controls, exceptions, and waivers explicit with rationale, limits, and follow-up.
5. Preserve re-review triggers so downstream consumers know when the package no longer applies.

## Workflow

### 1) Read the security/compliance context

- Start with the reviewed requirements, architecture, planning, implementation, release, policy, contractual, regulatory, vendor, and operational context that materially constrains the package.
- Pull in incident notes, audit notes, assurance requests, or release learnings only as needed to state the current posture honestly.
- Keep scope, environment, and approval limits explicit instead of silently broadening the package.

### 2) Frame controls and evidence clearly

- State which obligations, risks, trust boundaries, or sensitive flows are actually driving the control expectations.
- Identify the controls, safeguards, reviews, approvals, or process requirements that materially apply.
- For each material control, make evidence expectations explicit where they affect confidence or downstream use.
- Name the relevant implementation, verification, approval, or evidence owners instead of leaving ownership implied.

### 3) Handle compensating controls and exceptions honestly

For each material compensating control, exception, or waiver, capture:

- what is being relied on
- why it is being relied on
- who owns it
- what time horizon or approval window applies
- what follow-up or stronger control is still expected
- what change should trigger re-review or expiry

- Do not let temporary exceptions read like permanent resolutions.
- Do not let compensating controls masquerade as full equivalence when they are only partial risk reduction.

### 4) Preserve approval limits without overclaiming

- Distinguish "ready for downstream work under stated conditions" from final legal, audit, policy, deployment, or operational authorization.
- Keep approval-routing needs explicit when the package is not itself the final decision.
- Separate control framing from technical solution design, implementation ownership, and release execution.

### 5) Keep re-review posture visible

- State what scope changes, architecture changes, vendor changes, data handling changes, evidence updates, or exception changes should trigger another security/compliance pass.
- Keep unresolved gaps visible rather than smoothing them into generic readiness language.
- Preserve traceability from obligations and risk drivers through controls, exceptions, review posture, and handoff language.

## Guardrails

- Do not replace legal counsel, external audit authority, or final compliance sign-off.
- Do not drift into detailed technical solution design or direct control implementation.
- Do not present compensating controls, waivers, or exceptions as self-justifying approvals.
- Do not present downstream-ready packaging as final deployment or operational authorization.
- Do not hide weak evidence, unresolved obligations, or follow-up ownership gaps.

## Supporting skills

- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when security/compliance context is fragmented across multiple local sources.
- Use [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the package depends on stronger grounding in validated requirements or acceptance-sensitive constraints.
- Use [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is downstream packaging clarity after control and exception posture is already clear.
- Use [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md) when the inputs are mainly security reviews, audits, or stakeholder sessions that need normalization first.
- Use Allium skills only when governing behavioral specifications materially constrain the security/compliance package or when review reveals a real policy-sensitive behavioral gap.

## Outputs

This skill should strengthen or help populate:

- [`security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md)
- [`controls-and-evidence-matrix.md`](D:/Projects/orpheum/artifacts/controls-and-evidence-matrix.md)
- the posture, approval-limit, and follow-up portions of [`security-compliance-review.md`](D:/Projects/orpheum/artifacts/security-compliance-review.md)
- the control, exception, and approval-limit portions of [`security-compliance-handoff.md`](D:/Projects/orpheum/artifacts/security-compliance-handoff.md)

## Notes

- This skill is intentionally narrow. It supports Security / Compliance Specialist control and exception framing after reviewed delivery context already exists.
- It is not a legal-sign-off skill, not a technical implementation method, and not a deployment-authorization override.

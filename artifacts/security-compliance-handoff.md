# Security / Compliance Handoff

## Purpose

Package the completed security/compliance outputs into a downstream-ready handoff that architecture, planning, implementation, verification, release, or approval consumers can use without reconstructing risks, obligations, or control expectations from earlier artifacts.

Use this artifact after the security/compliance review is explicit and before routing the package downstream.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Security / Compliance Specialist work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what scope is being handed off, what obligations and controls still matter, what posture currently applies, who owns follow-up, what kind of downstream use the package is actually fit for, and what should happen next.

## Related Checks

- Primary: [`security-compliance-handoff.check.md`](D:/Projects/orpheum/checks/security-compliance-handoff.check.md)
- Cross-cutting: [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md)
- Cross-cutting: [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md)

## Current Assessment Scope

Summarize the system or delivery scope being handed off and why it matters.

## Security / Compliance Package Included

Reference the scope artifact, controls/evidence matrix, security/compliance review, and key supporting inputs that define the current package.

## Current Posture

State whether the package is ready, conditional, or blocked for the intended downstream consumer.

Make any scope, environment, evidence, consumer, or approval limits explicit so downstream consumers do not treat the handoff as blanket security or compliance approval.

If further legal, audit, policy, deployment, or operational approval is still required, state that explicitly so the handoff is not misread as final authorization.

## Active Controls, Conditions, And Gaps

Describe the most important controls, conditions, unresolved gaps, waivers, or compensating controls that downstream roles should preserve.

## Follow-Up Owners

Identify who owns each required remediation, evidence update, waiver, or approval follow-up.

## Re-Review Or Re-Approval Triggers

List the changes in scope, architecture, data handling, vendors, evidence, or environment that should cause the package to return to security/compliance work before downstream consumers rely on it further.

## Upstream Routing Notes

Call out any issues that should be routed to product, architecture, planning, implementation, verification, release, legal, or policy owners rather than treated purely as security/compliance cleanup.

## Recommended Next Consumer

Identify which downstream role, team, or human decision-maker should take the package next, why, and whether that next step is implementation-oriented, verification-oriented, release-oriented, or approval-oriented.

# Security / Compliance Review

## Purpose

Capture the durable review outcome for a drafted Security / Compliance Specialist package, including the current posture, blocking versus conditional issues, residual risks, and what should happen next.

Use this artifact after the scope artifact and controls/evidence matrix exist and before producing the downstream handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Security / Compliance Specialist work.

## Completion Guidance

This artifact is complete when a downstream reader can understand whether the current package is ready for downstream design, planning, implementation, verification, release-related, or approval-routing use under stated limits, what evidence supports that posture, what controls or approvals still matter, and what must happen next.

## Related Checks

- Primary: [`security-compliance-review.check.md`](D:/Projects/orpheum/checks/security-compliance-review.check.md)
- Cross-cutting: [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md)
- Cross-cutting: [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md)

## Review Scope

State which system, release horizon, environment set, or assessment window this review covers.

## Reviewed Inputs

Reference the scope artifact, controls/evidence matrix, and the supporting delivery, policy, or regulatory inputs used in this review.

## Overall Assessment

Summarize the current security/compliance posture in plain language, including the strongest reasons for confidence and the most important reasons for caution.

## Decision Status

State whether the package is:

- ready for downstream design, planning, implementation, verification, or release-adjacent work
- ready with conditions
- blocked pending remediation, evidence, waiver, or approval

State any scope, environment, evidence, consumer, or approval limits clearly if the decision is not universally applicable.

Make it explicit when this status means "ready for downstream work under stated conditions" rather than "already legally approved, audit-approved, policy-approved, deployment-approved, or operationally authorized."

## Decision Owner Or Required Approver

Identify who owns the current posture or who must explicitly approve the next step if the status is conditional or blocked.

If downstream legal, audit, policy, or operational approval is still required, record that here rather than leaving the distinction implied.

## Key Risks, Gaps, And Control Watchouts

Describe the material risks, unresolved gaps, control weaknesses, or trust-boundary-sensitive concerns that still matter.

## Conditions And Required Follow-Up

Record the remediation, evidence gathering, control implementation, waiver, approval, or architecture/product clarification required before the posture should change.

## Follow-Up Owners

If the package is conditional or blocked, identify who owns each required follow-up.

## Re-Review Or Re-Approval Triggers

List the changes in architecture, data handling, scope, vendors, evidence, environment, or obligations that should trigger another security/compliance review.

## Recommended Next Step

Describe the immediate next step, such as implementation follow-up, verification, release planning, approval routing, or formal approval request.

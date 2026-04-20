# Controls And Evidence Matrix

## Purpose

Capture the required controls, expected evidence, control owners, compensating controls, waivers, and unresolved gaps that shape the current security and compliance posture.

Use this artifact after scope is explicit and before downstream consumers treat the package as security-ready or compliance-ready.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Security / Compliance Specialist work.

## Completion Guidance

This artifact is complete when a downstream reader can understand which controls are required, what evidence is expected, who owns each control or follow-up, what compensating measures or exceptions exist, why they are being relied on, and where the current package is still weak.

## Related Checks

- Primary: [`controls-and-evidence-matrix.check.md`](D:/Projects/orpheum/checks/controls-and-evidence-matrix.check.md)
- Cross-cutting: [`security-compliance-traceability.check.md`](D:/Projects/orpheum/checks/security-compliance-traceability.check.md)
- Cross-cutting: [`security-compliance-specialist-boundary.check.md`](D:/Projects/orpheum/checks/security-compliance-specialist-boundary.check.md)

## Matrix Scope

State which assessment scope, release horizon, or environment set this matrix covers.

## Required Controls

List the controls, safeguards, reviews, approvals, or process requirements that materially apply.

## Evidence Expectations

Describe what evidence, documentation, tests, attestations, or operational proof is expected for each control where that detail matters.

## Control Owners

Identify who owns implementation, verification, approval, or evidence collection for each material control.

## Compensating Controls Or Exceptions

Describe any compensating controls, temporary exceptions, waivers, or alternative safeguards currently being relied on.

For each material exception, waiver, or compensating control, make the rationale, owner, time horizon, and any required approval or follow-up explicit enough that downstream consumers do not treat it as a permanent or self-justifying resolution.

## Unresolved Gaps

Identify where required controls, evidence, approvals, or exception handling are still incomplete, weak, or blocked.

## Re-Review Or Re-Approval Triggers

List the changes in architecture, data handling, vendors, threat exposure, evidence posture, or exception posture that should trigger another security/compliance pass.

## Recommended Next Step

Describe the immediate next step, such as architecture change, implementation follow-up, verification activity, or explicit approval request.

# Scenarios

This directory stores reusable scenario definitions and related scenario-guidance patterns.

Use this area for:

- baseline scenario templates for new projects
- variants tuned for specific delivery contexts or lifecycle phases
- experiments in multi-role scenario composition that may later be standardized

Start from the Scenario Designer package when creating or refining reusable multi-role scenarios.

## Lifecycle Guidance

The current default SDLC path is:

- validated discovery as a role-covered upstream precondition
- [`project-planning.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md)
- [`delivery-slice-planning.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.definition.md)
- [`implementation-and-release-prep.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.definition.md)
- [`review-remediation-loop.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.definition.md) when bounded remediation is needed
- [`verification-and-release-gate.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.definition.md) when a distinct final downstream gate is needed
- [`release-feedback-to-reprioritization.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.definition.md)

Use [`secure-delivery-feature-lifecycle.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.definition.md) instead of that standard chain when security/compliance, trust-boundary, or approval-sensitive concerns must shape the work end to end rather than appearing only as optional branches or a final gate.

The repository does not yet have a first-class discovery scenario. Validated discovery is still an upstream role-covered precondition rather than a reusable scenario package.

## Current Scenarios

- [`project-planning.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md) defines the reusable Project Planning scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs.
- [`project-planning.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.integration-map.md) captures the role-owned workflow chain, handoffs, dependencies, branching rules, and coordination watchouts for Project Planning.
- [`project-planning.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.review.md) records the current readiness and limits of the Project Planning scenario package.
- [`project-planning.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.handoff.md) packages the Project Planning scenario for downstream adoption and tailoring.
- [`delivery-slice-planning.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.definition.md) defines the reusable Delivery Slice Planning scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs for one bounded delivery slice package.
- [`delivery-slice-planning.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.integration-map.md) captures the role-owned workflow chain, handoffs, optional branches, and downstream routing from broader planning into one bounded slice.
- [`delivery-slice-planning.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.review.md) records the current readiness, participant-fit basis, and bounded-slice limits of the Delivery Slice Planning scenario package.
- [`delivery-slice-planning.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.handoff.md) packages the Delivery Slice Planning scenario for downstream adoption and tailoring.
- [`implementation-and-release-prep.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.definition.md) defines the reusable Implementation and Release Prep scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs for one bounded delivery slice or release candidate.
- [`implementation-and-release-prep.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.integration-map.md) captures the role-owned workflow chain, handoffs, dependencies, reconvergence points, and coordination watchouts for bounded-slice Implementation and Release Prep.
- [`implementation-and-release-prep.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.review.md) records the current readiness, participant-fit basis, and bounded-scope limits of the Implementation and Release Prep scenario package.
- [`implementation-and-release-prep.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.handoff.md) packages the bounded-slice Implementation and Release Prep scenario for downstream adoption and tailoring.
- [`secure-delivery-feature-lifecycle.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.definition.md) defines the reusable Secure Delivery / Secure Feature Lifecycle scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs for one bounded secure feature, control-sensitive change, or release candidate.
- [`secure-delivery-feature-lifecycle.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.integration-map.md) captures the role-owned workflow chain, security/compliance handoffs, dependencies, reconvergence points, and coordination watchouts for secure feature delivery.
- [`secure-delivery-feature-lifecycle.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.review.md) records the current readiness, participant-fit basis, and bounded secure-delivery limits of the Secure Delivery / Secure Feature Lifecycle scenario package.
- [`secure-delivery-feature-lifecycle.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.handoff.md) packages the Secure Delivery / Secure Feature Lifecycle scenario for downstream adoption and tailoring.
- [`review-remediation-loop.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.definition.md) defines the reusable Review Remediation Loop scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs for bounded remediation and re-review.
- [`review-remediation-loop.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.integration-map.md) captures the role-owned workflow chain, handoffs, optional verification participation, and convergence logic for Review Remediation Loop.
- [`review-remediation-loop.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.review.md) records the current readiness, participant-fit basis, and loop-specific limits of the Review Remediation Loop scenario package.
- [`review-remediation-loop.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.handoff.md) packages the Review Remediation Loop scenario for downstream adoption and tailoring.
- [`release-feedback-to-reprioritization.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.definition.md) defines the reusable Release Feedback To Reprioritization scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs for turning release or adoption learnings into updated product direction.
- [`release-feedback-to-reprioritization.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.integration-map.md) captures the role-owned workflow chain, feedback-routing branches, handoffs, and downstream guardrails for Release Feedback To Reprioritization.
- [`release-feedback-to-reprioritization.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.review.md) records the current readiness, participant-fit basis, and feedback-to-priority limits of the Release Feedback To Reprioritization scenario package.
- [`release-feedback-to-reprioritization.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.handoff.md) packages the Release Feedback To Reprioritization scenario for downstream adoption and tailoring.
- [`scenario-implementation.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.definition.md) defines the reusable Scenario Implementation scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs.
- [`scenario-implementation.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.integration-map.md) captures the role-owned workflow chain, participant-hardening loop, skills-review flow, branching rules, and coordination watchouts for Scenario Implementation.
- [`scenario-implementation.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.review.md) records the current readiness and limits of the Scenario Implementation scenario package.
- [`scenario-implementation.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.handoff.md) packages the Scenario Implementation scenario for downstream adoption and tailoring.
- [`verification-and-release-gate.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.definition.md) defines the reusable Verification And Release Gate scenario boundary, lifecycle window, participating roles, entry conditions, and target outputs.
- [`verification-and-release-gate.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.integration-map.md) captures the verification, security/compliance, and release-handoff workflow chain, handoff contracts, branching rules, and coordination watchouts for Verification And Release Gate.
- [`verification-and-release-gate.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.review.md) records the current readiness, participant-fit basis, and downstream-gate limits of the Verification And Release Gate scenario package.
- [`verification-and-release-gate.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.handoff.md) packages the Verification And Release Gate scenario for downstream adoption and tailoring.

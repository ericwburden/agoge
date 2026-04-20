# Review Findings

## Purpose

Capture the durable finding record for a code review pass, including the concrete concerns, affected areas, severity, confidence, rationale, and requested remediation or evidence.

Use this artifact after the review scope is clear and before producing the downstream review decision and handoff.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Code Reviewer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what findings were raised, why they matter, how severe or confident they are, what surfaces they affect, and what remediation or clarification is requested without reconstructing the concern from raw comments.

## Related Checks

- Primary: [`review-findings.check.md`](D:/Projects/orpheum/checks/review-findings.check.md)
- Cross-cutting: [`review-traceability.check.md`](D:/Projects/orpheum/checks/review-traceability.check.md)
- Cross-cutting: [`code-reviewer-boundary.check.md`](D:/Projects/orpheum/checks/code-reviewer-boundary.check.md)

## Findings Summary

Summarize the overall findings posture in plain language, including whether the review found blocking defects, non-blocking concerns, or no material findings.

## Review Coverage Note

State whether the absence of findings reflects a full review pass, a narrow review slice, or a constrained review with important limits.

## Finding Log

For each material finding, capture:

- finding identifier or short title
- status such as blocking, non-blocking, advisory, or open question
- severity and confidence
- affected files, modules, interfaces, migrations, prompts, or assets
- affected location detail, such as file path, function, endpoint, query, prompt section, config key, or line range when practical
- why the finding matters
- the concrete behavior, risk, or drift involved
- the evidence basis for the finding, such as direct code inspection, observed behavior, reproduction notes, logs, tests, screenshots, or specification comparison
- the upstream requirement, architecture, planning, evidence, or specification anchor it relates to
- requested remediation, clarification, or additional evidence

## Blocking Findings

List the findings that should prevent approval or downstream use until remediated or explicitly waived.

## Non-Blocking Findings

List findings that should be addressed or tracked but do not by themselves block the current review decision.

## Missing Validation Or Weak Evidence Findings

Call out findings that primarily concern missing, weak, stale, or contradictory evidence rather than code defects alone.

## Upstream Routing Findings

List findings that actually point to requirement, architecture, planning, or specification defects and should be routed upstream rather than treated as only local code issues.

## Reviewer Notes On Residual Risk

Describe any residual risk patterns that still matter even if no single finding fully captures them.

## Re-Review Triggers

List the changes, clarifications, evidence updates, or scope expansions that should trigger another code-review pass.

## Suggested Next Action

Describe the immediate next step for the findings package, such as implementation remediation, targeted evidence collection, upstream rework, or move to review decision.

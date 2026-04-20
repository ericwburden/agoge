# Security / Compliance Scope Check

## Purpose

Validate that the security/compliance scope artifact makes the in-scope systems, obligations, threat surfaces, trust boundaries, and assumptions explicit enough to guide downstream work.

## Applies To

- Instantiated copies of [`security-compliance-scope.md`](D:/Projects/orpheum/artifacts/security-compliance-scope.md)
- Use before treating the assessment scope as ready for control mapping, review, or downstream handoff

## Criteria

- The assessment scope is explicit.
- Reviewed inputs are referenced clearly enough to explain why this scope exists now.
- Sensitive assets, data classes, interfaces, or vendor surfaces are identified when they materially affect risk or obligations.
- Relevant trust boundaries and abuse or threat surfaces are visible.
- Applicable obligations and control drivers are explicit.
- Assumptions, exclusions, and open questions are surfaced rather than hidden.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The instantiated security/compliance scope artifact
- The supporting delivery, policy, or regulatory inputs used to justify it

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when supporting context is spread across multiple local sources.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md) when the scope needs stronger grounding in validated needs or acceptance-sensitive constraints.

## Failure Response

- Rework the scope artifact before using it as a downstream anchor.
- Route missing discovery, architecture, or obligation clarity upstream rather than inventing certainty.

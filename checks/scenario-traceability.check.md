# Scenario Traceability Check

## Purpose

Validate that scenario outputs stay traceable to the role-owned workflows, artifact contracts, and gating assumptions they compose rather than inventing scenario logic that cannot be grounded in the underlying role packages.

## Applies To

- All instantiated Scenario Designer artifacts and Scenario Designer workflow outputs
- Use whenever reviewing whether a scenario package stays grounded in the role packages it composes

## Criteria

- Participating roles and role-owned workflows are identified explicitly.
- Material scenario steps trace back to role-owned workflows, artifacts, or explicit gates rather than chat-only assumptions.
- Handoff expectations and dependency assumptions are traceable to actual role-package outputs where those outputs are expected to exist.
- Scenario conditions, risks, or watchouts are grounded in identifiable workflow, artifact, approval, or specification realities.
- Missing support is called out explicitly rather than hidden behind scenario language.
- The package makes the relationship to existing behavioral specifications explicit when scenario gating materially depends on them.

## Scoring Or Outcome

Pass/fail.

## Evidence Required

- The Scenario Designer artifact or workflow output being reviewed
- The relevant role definitions, workflows, artifacts, or supporting notes it depends on
- Any cited Allium or behavioral specification references when they materially constrain the scenario

## Supporting Skills

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when traceability defects are being caused by unsynthesized local context.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) when traceability defects come from loosely imported public patterns that were not adapted cleanly.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when traceability broke because scenario gating depended on specification clarity that should have been refined upstream.

## Failure Response

- Rework the output until scenario logic can be traced back to the role packages, artifact contracts, and gating assumptions it depends on.
- Route unsupported scenario expectations to the correct role package, specification, or approval source rather than leaving them implied.

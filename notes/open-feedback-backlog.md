# Open Feedback Backlog

## Purpose

Track the remaining product-feedback items that still appear unimplemented or only partially implemented after the recent Orpheum minor releases.

This note is intentionally short and backlog-oriented. Implemented feedback should be removed from the active notes set rather than preserved here as duplicate history.

## Current Open Feedback

### 1. Host-environment skill registry parity

Local project skills installed by Orpheum should appear in the host environment's advertised skill registry, not only on disk.

Why this remains open:

- the adoption problem was captured in earlier onboarding feedback
- the repo now installs and refreshes the local Orpheum skill correctly
- there is still no repo-local implementation that makes host environments surface those local project skills automatically

Likely ownership:

- host environment coordination
- Orpheum follow-up only where local metadata or integration hooks are required

## 2. Dedicated locked-decisions artifact decision

Orpheum now has stronger `Decisions Made` and `Locked Constraints` patterns in existing artifacts, but it has not yet decided whether that is sufficient or whether the repo still needs a dedicated artifact such as `decision-register`, `locked-decisions`, or `architecture-constraints`.

Why this remains open:

- the lightweight section-based path has landed
- the stronger "new artifact versus stronger existing contract" choice is still unresolved

Decision to make:

- keep the current embedded-section approach as the long-term standard
- add a dedicated artifact
- support both, with a narrow default and a fuller optional pattern

### 3. CLI lifecycle commands beyond the current v1 surface

The broader CLI proposal still includes lifecycle commands and compatibility handling that have not yet landed.

Most visible gaps:

- `orpheum finalize`
- `orpheum cleanup`
- `orpheum suspend`
- `orpheum resume`
- stronger compatibility handling for suspended-session format changes

Current status:

- version-aware refresh warnings and `orpheum update` are implemented
- the rest of the lifecycle model remains proposal-level rather than shipping behavior

### 4. CLI maintainability hardening backlog

The Rust CLI maintainability issue set still appears mostly open as a backlog note rather than as completed repo work.

Most likely remaining items:

- further splitting of large core modules into smaller focused units
- standardizing all persisted `.orpheum/` file creation behind typed serialization paths
- any remaining runtime/catalog decoupling or structural cleanup that was proposed but not yet implemented

## Recommended Next Step

Use this note as the default destination for future product-feedback review until new feedback arrives.

When an item here is implemented:

1. remove it from this backlog
2. update `notes/README.md`
3. avoid recreating a new long-form note unless the feedback introduces a genuinely new design question

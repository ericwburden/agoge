# Orpheum CLI Maintainability Issue Set

## Purpose

Capture the GitHub issue set produced by applying the repository's planning scenarios to the recent Rust CLI maintainability review.

This note turns review findings into a sequenced, developer-attractive backlog rather than leaving them as isolated observations.

## Scenario Basis

This issue set was derived by applying the current scenario model in a lightweight planning sequence:

- `Project Discovery`
  - clarified the real problem: the CLI works, but the current Rust structure still reads more like a repo-local prototype than a polished tool other developers will want to extend
- `Project Planning`
  - turned that problem into explicit product and architecture direction for the next CLI hardening phase
- `Delivery Slice Planning`
  - turned the resulting improvement space into bounded implementation slices suitable for GitHub issues

The resulting issue set is intentionally shaped for incremental implementation rather than one large refactor.

## Product Direction

The current direction for the CLI codebase should be:

- keep the command surface narrow and explicit
- make runtime behavior less coupled to the source checkout
- increase type safety in the domain model
- split large modules along durable boundaries
- keep persisted file contracts strongly typed and easy to evolve

The goal is not "more features." The goal is a Rust project structure that feels intentional, trustworthy, and pleasant for human maintainers.

## Backlog Ordering

Recommended implementation order:

1. decouple runtime catalog discovery from the build checkout
2. stop loading the catalog for commands that do not need it
3. replace stringly typed domain fields with enums
4. split `catalog.rs` and `session.rs` into smaller focused modules
5. unify persisted file formats behind typed serialization

This order reduces architectural risk first, then improves ergonomics and long-term maintainability.

## Proposed GitHub Issues

### Issue 1: Decouple catalog discovery from build-time source layout

**Why this exists**

The current binary derives the catalog root from `CARGO_MANIFEST_DIR`, which ties runtime behavior to the checkout it was built from. That is acceptable for a local prototype but weak for a maintainable CLI tool.

**Primary files**

- [crates/orpheum-core/src/catalog.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/catalog.rs)
- [crates/orpheum-cli/src/main.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-cli/src/main.rs)

**Suggested GitHub title**

`Decouple Orpheum catalog discovery from build-time source paths`

**Issue body draft**

The CLI currently resolves its default catalog root from `CARGO_MANIFEST_DIR`, which makes the runtime behavior depend on where the crate was built from rather than on an explicit runtime contract. We should move catalog discovery behind a maintainable runtime mechanism such as a CLI flag, environment variable, or adjacent installed catalog layout.

Acceptance criteria:

- the CLI no longer requires `CARGO_MANIFEST_DIR`-derived source checkout assumptions at runtime
- catalog location can be supplied through an explicit runtime contract
- the default resolution strategy is documented and test-covered
- existing development workflows remain straightforward for local contributors

### Issue 2: Load the catalog only for commands that actually need it

**Why this exists**

`init`, `status`, and `prompt current` should not fail just because catalog loading fails. Today the CLI loads the catalog before dispatching any command, which creates unnecessary coupling and makes the command architecture less clean.

**Primary files**

- [crates/orpheum-cli/src/main.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-cli/src/main.rs)

**Suggested GitHub title**

`Refactor CLI dispatch so catalog loading is command-specific`

**Issue body draft**

The CLI currently loads the catalog before subcommand dispatch, even for commands whose logic is entirely project-local. We should refactor dispatch so catalog loading happens only in commands that actually require catalog access.

Acceptance criteria:

- `init`, `status`, and `prompt current` can run without preloading the catalog
- catalog-dependent commands continue to validate catalog state clearly
- command dispatch becomes easier to read and reason about
- integration tests cover at least one no-catalog-needed path

### Issue 3: Replace stringly typed state with serde-backed enums

**Why this exists**

The core model currently uses plain strings for fields like check mode, severity, session mode, cleanup policy, and session state. That gives up a lot of Rust's value and makes future refactors riskier than they need to be.

**Primary files**

- [crates/orpheum-core/src/catalog.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/catalog.rs)
- [crates/orpheum-core/src/session.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/session.rs)
- [crates/orpheum-core/src/checks.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/checks.rs)

**Suggested GitHub title**

`Introduce typed enums for CLI metadata, session state, and check status`

**Issue body draft**

Several important parts of the CLI domain model are still represented as ad hoc strings. We should replace those with typed enums that preserve the current serialized contract via `serde`.

Acceptance criteria:

- check mode and severity are typed enums
- session state, mode, and cleanup policy are typed enums
- persisted JSON/YAML formats remain compatible or are migrated intentionally
- command and check logic no longer relies on free-form string comparisons for these fields

### Issue 4: Split large core modules into smaller focused units

**Why this exists**

`catalog.rs` and `session.rs` are already carrying multiple concerns at once: model definitions, filesystem behavior, validation, rendering, and helper logic. That is a maintainability warning sign even if the code still works.

**Primary files**

- [crates/orpheum-core/src/catalog.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/catalog.rs)
- [crates/orpheum-core/src/session.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/session.rs)
- [crates/orpheum-core/src/lib.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/lib.rs)

**Suggested GitHub title**

`Restructure orpheum-core into smaller catalog and session modules`

**Issue body draft**

The core crate needs stronger internal boundaries so future contributors can navigate it more easily. We should split the large modules into smaller focused units such as model, load, validate, files, and render layers.

Acceptance criteria:

- `catalog.rs` is split into smaller modules with clear responsibilities
- `session.rs` is split into smaller modules with clear responsibilities
- public exports remain easy to discover from `lib.rs`
- the resulting structure is documented or obvious enough that maintainers can find catalog, session, and rendering logic quickly

### Issue 5: Use typed structs for every persisted file contract

**Why this exists**

The current implementation still writes some persisted state through raw JSON strings instead of through the same typed Rust models used elsewhere. That invites format drift and makes evolution harder.

**Primary files**

- [crates/orpheum-core/src/session.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/session.rs)
- [crates/orpheum-core/src/checks.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/src/checks.rs)

**Suggested GitHub title**

`Standardize persisted .orpheum file formats on typed serialization`

**Issue body draft**

Persisted `.orpheum/` files should be produced from one typed Rust struct per file shape rather than from handwritten JSON fragments. This keeps the file contract explicit, safer to evolve, and easier to test.

Acceptance criteria:

- no persisted `.orpheum/` files are initialized from handwritten JSON strings
- file creation and refresh logic share typed models where appropriate
- tests cover the initial shape of each persisted control file that the CLI owns directly
- future schema changes can be made from typed models rather than scattered string literals

## Optional Meta Issue

If you want a parent tracking issue, use:

**Suggested GitHub title**

`Harden the Orpheum Rust CLI for long-term maintainability`

**Scope**

Track the five issues above as one maintainability epic focused on structure, type safety, runtime decoupling, and durable developer ergonomics.

## Recommended Next Step

Create the five concrete GitHub issues first, then optionally add the meta issue if you want one parent thread for tracking the hardening phase.

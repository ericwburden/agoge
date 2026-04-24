use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::catalog_runtime::runtime_catalog_resolution;
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::project_config::{CatalogSource, ProjectState, write_local_config};
use crate::session::refresh_session_cli_version;

const ORPHEUM_SKILL_BODY: &str = r#"---
name: orpheum
description: Use the local Orpheum CLI and session files to discover scenarios, apply them, inspect state, and run checks without inferring workflow state from prose.
---

# Orpheum

Use this skill when working in a project that has the `orpheum` CLI available and may use Orpheum scenarios for AI-assisted SDLC work.

This file is installed by `orpheum init` and represents the local agent contract for Orpheum usage in a consumer project.

## Purpose

This skill teaches the local operating contract for Orpheum in a consumer project.

Use Orpheum to:

- discover available scenarios
- inspect scenario structure before applying it
- apply one scenario into the current project
- read authoritative session state from `.orpheum/`
- generate the current recommended prompt
- run scenario checks

## Authoritative Sources

Prefer these sources in order:

1. `orpheum ... --json`
2. `.orpheum/session.json`
3. `.orpheum/scenario.json`
4. `.orpheum/state.json`
5. `.orpheum/logs/checks.json`

Treat these as derived views rather than source of truth:

- `.orpheum/ACTIVE.md`
- `.orpheum/prompts/current.md`
- surrounding catalog prose
- stale chat context

## Core Rules

- Prefer `orpheum ... --json` when you need authoritative machine-readable state.
- Treat `.orpheum/session.json`, `.orpheum/scenario.json`, and `.orpheum/state.json` as authoritative state.
- Treat `.orpheum/prompts/current.md` and `orpheum prompt current` as derived guidance, not source of truth.
- Do not infer scenario dependencies from prose files when the CLI or session JSON can tell you directly.
- Do not create `.orpheum/` by hand. Use `orpheum scenario apply <id>`.
- Run `orpheum check run` before claiming scenario-associated outputs are ready.

## What `orpheum init` Does

`orpheum init` is the project-onboarding step for agents.

It:

- installs this local skill at `.codex/skills/orpheum/SKILL.md`
- refreshes the project to use the embedded Orpheum catalog by default
- persists an external catalog override in `.codex/orpheum/config.json` only when one is explicitly selected or discovered
- writes a repo-root `ORPHEUM.md` onboarding file
- appends `.orpheum/` to an existing `.gitignore` if that line is missing

It does not:

- apply a scenario
- create `.orpheum/` session files
- infer or select an active scenario
- create a new `.gitignore` when none exists

## Normal Command Loop

1. Discover scenarios:
   - `orpheum scenario list --json`
2. Inspect one scenario:
   - `orpheum scenario show <id> --json`
3. Apply a scenario to the current project:
   - `orpheum scenario apply <id> --json`
4. Inspect current session state:
   - `orpheum status --json`
5. Get the current recommended prompt:
   - `orpheum prompt current --json`
6. Run validation checks:
   - `orpheum check run --json`
7. Close a finalized session safely when ready:
   - `orpheum session close --json`
8. Diagnose catalog or project setup:
   - `orpheum doctor --json`

## Session Files

When a scenario is active, expect these files under `.orpheum/`:

- `ACTIVE.md`
- `session.json`
- `scenario.json`
- `state.json`
- `prompts/current.md`
- `logs/checks.json`

Use them like this:

- `session.json`: session identity and project-local control metadata
- `scenario.json`: resolved snapshot of the applied scenario
- `state.json`: mutable progress, workflow state, artifact state, and check state
- `logs/checks.json`: latest check report

## Decision Guidance

- If there is no active session, use `orpheum scenario list` or `orpheum scenario show`, then apply a scenario if appropriate.
- If a session exists, start with `orpheum status --json`.
- If check status is unclear or stale, run `orpheum check run --json`.
- If a session is finalized and `cleanup_ready` is true, run `orpheum session close --json` before applying a new scenario.
- If the environment looks misconfigured, run `orpheum doctor --json`.
- During semantic artifact review for discovery or planning scenarios, use Planning Mode or the host environment's nearest equivalent before changing the artifact set.

## What Not To Do

- Do not guess the active scenario from nearby docs when `.orpheum/scenario.json` exists.
- Do not treat stale chat context as authoritative when current session files disagree.
- Do not silently skip scenario checks.
- Do not treat `.orpheum/` as the durable home for finished project artifacts.
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitResult {
    pub project_root: Utf8PathBuf,
    pub project_state: ProjectState,
    pub skill_dir: Utf8PathBuf,
    pub skill_file: Utf8PathBuf,
    pub skill_installed: bool,
    pub gitignore_file: Option<Utf8PathBuf>,
    pub gitignore_updated: bool,
    pub local_config_file: Utf8PathBuf,
    pub onboarding_file: Utf8PathBuf,
    pub catalog_source: CatalogSource,
    pub catalog_root: Option<Utf8PathBuf>,
}

pub fn init_project(
    project_root: &Utf8Path,
    explicit_catalog_root: Option<&Utf8Path>,
) -> Result<InitResult, OrpheumError> {
    let skill_dir = project_root.join(".codex").join("skills").join("orpheum");
    let skill_file = skill_dir.join("SKILL.md");
    fs::create_dir_all(&skill_dir)?;

    let skill_installed = match fs::read_to_string(&skill_file) {
        Ok(existing) if existing == ORPHEUM_SKILL_BODY => false,
        _ => {
            fs::write(&skill_file, ORPHEUM_SKILL_BODY)?;
            true
        }
    };

    let resolution =
        runtime_catalog_resolution(explicit_catalog_root, project_root).map_err(|_| {
            OrpheumError::coded(
                OrpheumErrorCode::CatalogNotFound,
                "unable to resolve an Orpheum catalog for this project",
            )
        })?;

    let local_config_file = project_root
        .join(".codex")
        .join("orpheum")
        .join("config.json");
    match resolution.root.as_deref() {
        Some(root) => {
            write_local_config(project_root, root)?;
        }
        None => {
            let config_dir = local_config_file.parent().expect("config dir");
            if config_dir.exists() {
                fs::create_dir_all(config_dir)?;
            }
            if local_config_file.exists() {
                fs::remove_file(&local_config_file)?;
            }
        }
    }
    let onboarding_file = project_root.join("ORPHEUM.md");
    let project_state = if project_root.join(".orpheum").exists() {
        ProjectState::Active
    } else {
        ProjectState::Initialized
    };
    let _ = refresh_session_cli_version(project_root)?;

    let gitignore_file = project_root.join(".gitignore");
    let (gitignore_path, gitignore_updated) = if gitignore_file.exists() {
        let existing = fs::read_to_string(&gitignore_file)?;
        if existing.lines().any(|line| line.trim() == ".orpheum/") {
            (Some(gitignore_file), false)
        } else {
            let mut updated = existing;
            if !updated.is_empty() && !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push_str(".orpheum/\n");
            fs::write(&gitignore_file, updated)?;
            (Some(gitignore_file), true)
        }
    } else {
        (None, false)
    };

    fs::write(
        &onboarding_file,
        build_onboarding_markdown(
            project_root,
            project_state,
            &skill_file,
            &local_config_file,
            resolution.root.as_deref(),
            resolution.source,
        ),
    )?;

    Ok(InitResult {
        project_root: project_root.to_path_buf(),
        project_state,
        skill_dir,
        skill_file,
        skill_installed,
        gitignore_file: gitignore_path,
        gitignore_updated,
        local_config_file,
        onboarding_file,
        catalog_source: resolution.source,
        catalog_root: resolution.root,
    })
}

fn build_onboarding_markdown(
    project_root: &Utf8Path,
    project_state: ProjectState,
    skill_file: &Utf8Path,
    local_config_file: &Utf8Path,
    catalog_root: Option<&Utf8Path>,
    catalog_source: CatalogSource,
) -> String {
    let next_commands = if matches!(project_state, ProjectState::Active) {
        vec![
            "- `orpheum status --json`",
            "- `orpheum prompt current --json`",
            "- `orpheum check run --json`",
        ]
    } else {
        vec![
            "- `orpheum scenario list --json`",
            "- `orpheum scenario show <id> --json`",
            "- `orpheum scenario apply <id> --json`",
        ]
    };

    let catalog_root_display = catalog_root
        .map(|root| root.as_str().to_string())
        .unwrap_or_else(|| "embedded-catalog".into());
    let local_config_note = if catalog_root.is_some() {
        format!(
            "- External catalog override is recorded in `{}` and will take precedence over the embedded catalog.\n- Compatibility fallback: `ORPHEUM_CATALOG={}`\n",
            local_config_file, catalog_root_display
        )
    } else {
        format!(
            "- No external catalog override is recorded in `{}`; Orpheum will use the embedded catalog by default.\n- Use `orpheum update --catalog <path>` if you want this project to follow a local development catalog instead.\n",
            local_config_file
        )
    };

    format!(
        "# ORPHEUM\n\n- Project root: `{}`\n- Project state: `{}`\n- Local skill: `{}`\n- Local catalog config: `{}`\n- Catalog root: `{}`\n- Catalog source: `{}`\n- Active session present: `{}`\n\n## Next Commands\n\n{}\n\n## Notes\n\n- `orpheum init` makes the project Orpheum-capable, but does not apply a scenario.\n- Catalog-dependent commands prefer explicit `--catalog`, then repo-local config, then `ORPHEUM_CATALOG`, then the embedded catalog.\n{}\n",
        project_root,
        project_state.as_str(),
        skill_file,
        local_config_file,
        catalog_root_display,
        catalog_source.as_str(),
        project_root.join(".orpheum").exists(),
        next_commands.join("\n"),
        local_config_note,
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use camino::Utf8PathBuf;
    use tempfile::tempdir;

    use crate::catalog_loading::load_catalog_for_root;
    use crate::project_config::read_local_config;

    use super::{ORPHEUM_SKILL_BODY, init_project};

    fn repo_root() -> Utf8PathBuf {
        Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir has parent")
            .parent()
            .expect("workspace root")
            .to_path_buf()
    }

    #[test]
    fn init_installs_skill_updates_gitignore_and_persists_local_config() {
        let project = tempdir().expect("tempdir");
        let project_root =
            Utf8PathBuf::from_path_buf(project.path().to_path_buf()).expect("utf8 temp path");
        fs::write(project_root.join(".gitignore"), "node_modules/\n").expect("gitignore");

        let result =
            init_project(&project_root, Some(repo_root().as_ref())).expect("init should succeed");

        assert!(result.skill_file.exists());
        assert!(result.skill_installed);
        assert!(result.gitignore_updated);
        assert!(result.onboarding_file.exists());
        assert!(result.local_config_file.exists());
        let gitignore = fs::read_to_string(project_root.join(".gitignore")).expect("gitignore");
        assert!(gitignore.contains(".orpheum/"));
        let config = read_local_config(&project_root)
            .expect("config read")
            .expect("config");
        load_catalog_for_root(&config.catalog_root).expect("catalog should load");
    }

    #[test]
    fn init_is_idempotent_when_skill_and_gitignore_are_already_present() {
        let project = tempdir().expect("tempdir");
        let project_root =
            Utf8PathBuf::from_path_buf(project.path().to_path_buf()).expect("utf8 temp path");
        fs::create_dir_all(project_root.join(".codex").join("skills").join("orpheum"))
            .expect("skill dir");
        fs::write(
            project_root
                .join(".codex")
                .join("skills")
                .join("orpheum")
                .join("SKILL.md"),
            ORPHEUM_SKILL_BODY,
        )
        .expect("skill body");
        fs::write(project_root.join(".gitignore"), ".orpheum/\n").expect("gitignore");

        let result =
            init_project(&project_root, Some(repo_root().as_ref())).expect("init should succeed");

        assert!(!result.skill_installed);
        assert!(!result.gitignore_updated);
    }

    #[test]
    fn init_refreshes_outdated_skill_content() {
        let project = tempdir().expect("tempdir");
        let project_root =
            Utf8PathBuf::from_path_buf(project.path().to_path_buf()).expect("utf8 temp path");
        fs::create_dir_all(project_root.join(".codex").join("skills").join("orpheum"))
            .expect("skill dir");
        fs::write(
            project_root
                .join(".codex")
                .join("skills")
                .join("orpheum")
                .join("SKILL.md"),
            "# old skill\n",
        )
        .expect("stale skill body");

        let result =
            init_project(&project_root, Some(repo_root().as_ref())).expect("init should succeed");
        let installed = fs::read_to_string(result.skill_file).expect("skill file");

        assert!(result.skill_installed);
        assert_eq!(installed, ORPHEUM_SKILL_BODY);
    }
}

use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::error::OrpheumError;

const ORPHEUM_SKILL_BODY: &str = r#"---
name: orpheum
description: Use the local Orpheum CLI and session files to discover scenarios, apply them, inspect state, and run checks without inferring workflow state from prose.
---

# Orpheum

Use this skill when working in a project that has the `orpheum` CLI available and may use Orpheum scenarios for AI-assisted SDLC work.

## Purpose

This skill teaches the local operating contract for Orpheum in a consumer project.

Use Orpheum to:

- discover available scenarios
- inspect scenario structure before applying it
- apply one scenario into the current project
- read authoritative session state from `.orpheum/`
- generate the current recommended prompt
- run scenario checks

## Core Rules

- Prefer `orpheum ... --json` when you need authoritative machine-readable state.
- Treat `.orpheum/session.json`, `.orpheum/scenario.json`, and `.orpheum/state.json` as authoritative state.
- Treat `.orpheum/prompts/current.md` and `orpheum prompt current` as derived guidance, not source of truth.
- Do not infer scenario dependencies from prose files when the CLI or session JSON can tell you directly.
- Do not create `.orpheum/` by hand. Use `orpheum scenario apply <id>`.
- Run `orpheum check run` before claiming scenario-associated outputs are ready.

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
7. Diagnose catalog or project setup:
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
- If the environment looks misconfigured, run `orpheum doctor --json`.

## What Not To Do

- Do not guess the active scenario from nearby docs when `.orpheum/scenario.json` exists.
- Do not treat stale chat context as authoritative when current session files disagree.
- Do not silently skip scenario checks.
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitResult {
    pub project_root: Utf8PathBuf,
    pub skill_dir: Utf8PathBuf,
    pub skill_file: Utf8PathBuf,
    pub skill_installed: bool,
    pub gitignore_file: Option<Utf8PathBuf>,
    pub gitignore_updated: bool,
}

pub fn init_project(project_root: &Utf8Path) -> Result<InitResult, OrpheumError> {
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

    Ok(InitResult {
        project_root: project_root.to_path_buf(),
        skill_dir,
        skill_file,
        skill_installed,
        gitignore_file: gitignore_path,
        gitignore_updated,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use camino::Utf8PathBuf;
    use tempfile::tempdir;

    use super::{ORPHEUM_SKILL_BODY, init_project};

    #[test]
    fn init_installs_skill_and_updates_existing_gitignore() {
        let project = tempdir().expect("tempdir");
        let project_root =
            Utf8PathBuf::from_path_buf(project.path().to_path_buf()).expect("utf8 temp path");
        fs::write(project_root.join(".gitignore"), "node_modules/\n").expect("gitignore");

        let result = init_project(&project_root).expect("init should succeed");

        assert!(result.skill_file.exists());
        assert!(result.skill_installed);
        assert!(result.gitignore_updated);
        let gitignore = fs::read_to_string(project_root.join(".gitignore")).expect("gitignore");
        assert!(gitignore.contains(".orpheum/"));
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

        let result = init_project(&project_root).expect("init should succeed");

        assert!(!result.skill_installed);
        assert!(!result.gitignore_updated);
    }
}

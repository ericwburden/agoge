use std::fs;

use camino::Utf8Path;

use crate::catalog::OutputMode;
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::session_render::{build_active_markdown, build_prompt};
use crate::session_types::{SessionFiles, SessionManifest, SessionScenarioSnapshot, SessionState};

pub(crate) fn session_files(project_root: &Utf8Path) -> SessionFiles {
    let control_dir = project_root.join(".orpheum");
    SessionFiles {
        active_file: control_dir.join("ACTIVE.md"),
        session_file: control_dir.join("session.json"),
        scenario_file: control_dir.join("scenario.json"),
        state_file: control_dir.join("state.json"),
        prompt_file: control_dir.join("prompts").join("current.md"),
        check_log_file: control_dir.join("logs").join("checks.json"),
        control_dir,
    }
}

pub fn read_session_files(
    project_root: &Utf8Path,
) -> Result<
    (
        SessionManifest,
        SessionScenarioSnapshot,
        SessionState,
        SessionFiles,
    ),
    OrpheumError,
> {
    let files = session_files(project_root);
    if !files.session_file.exists() {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::NoActiveSession,
            format!("no active Orpheum session found in {}", files.control_dir),
        ));
    }

    let manifest =
        serde_json::from_str::<SessionManifest>(&fs::read_to_string(&files.session_file)?)?;
    let snapshot = serde_json::from_str::<SessionScenarioSnapshot>(&fs::read_to_string(
        &files.scenario_file,
    )?)?;
    let state = serde_json::from_str::<SessionState>(&fs::read_to_string(&files.state_file)?)?;
    Ok((manifest, snapshot, state, files))
}

pub fn read_active_summary(project_root: &Utf8Path) -> Result<String, OrpheumError> {
    let (_, _, _, files) = read_session_files(project_root)?;
    Ok(fs::read_to_string(files.active_file)?)
}

pub fn refresh_state_files(
    project_root: &Utf8Path,
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
    manifest: &SessionManifest,
) -> Result<(), OrpheumError> {
    let files = session_files(project_root);
    fs::write(&files.state_file, serde_json::to_string_pretty(state)?)?;
    let prompt = build_prompt(snapshot, state, OutputMode::Human);
    fs::write(&files.prompt_file, &prompt)?;
    fs::write(
        &files.active_file,
        build_active_markdown(manifest, snapshot, state),
    )?;
    Ok(())
}

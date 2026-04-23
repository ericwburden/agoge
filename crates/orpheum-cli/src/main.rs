use std::process::ExitCode;

use camino::{Utf8Path, Utf8PathBuf};
use clap::{Args, Parser, Subcommand};
use orpheum_core::{
    Catalog, CheckRunReport, CheckStatusValue, DoctorReport, OrpheumError, ResolvedScenario,
    ScenarioListItem, apply_scenario, generate_current_prompt, init_project, read_session_files,
    run_doctor,
};

#[derive(Debug, Parser)]
#[command(name = "orpheum")]
#[command(about = "Resolve Orpheum scenarios into project-local orchestration state")]
struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    catalog: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init(InitArgs),
    Scenario(ScenarioCommand),
    Status(OutputArgs),
    Prompt(PromptCommand),
    Check(CheckCommand),
    Doctor(OutputArgs),
}

#[derive(Debug, Args)]
struct OutputArgs {
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct InitArgs {
    #[arg(long)]
    catalog: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum ScenarioSubcommand {
    List(OutputArgs),
    Show(ScenarioShowArgs),
    Apply(ScenarioApplyArgs),
}

#[derive(Debug, Args)]
struct ScenarioCommand {
    #[command(subcommand)]
    command: ScenarioSubcommand,
}

#[derive(Debug, Args)]
struct ScenarioShowArgs {
    scenario: String,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ScenarioApplyArgs {
    scenario: String,
    #[arg(long)]
    project: Option<String>,
    #[arg(long)]
    force: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct PromptCommand {
    #[command(subcommand)]
    command: PromptSubcommand,
}

#[derive(Debug, Subcommand)]
enum PromptSubcommand {
    Current(OutputArgs),
}

#[derive(Debug, Args)]
struct CheckCommand {
    #[command(subcommand)]
    command: CheckSubcommand,
}

#[derive(Debug, Subcommand)]
enum CheckSubcommand {
    Run(OutputArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}: {}", err.code().as_str(), err);
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<(), OrpheumError> {
    let cwd = current_dir_utf8()?;
    let catalog_arg = cli.catalog.as_deref();

    match cli.command {
        Commands::Init(args) => {
            let explicit_catalog = args.catalog.as_deref().map(Utf8Path::new);
            let result = init_project(&cwd, explicit_catalog)?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("Initialized Orpheum guidance for {}", result.project_root);
                println!("Project state: {}", result.project_state.as_str());
                println!("Local skill: {}", result.skill_file);
                println!("Catalog source: {}", result.catalog_source.as_str());
                if let Some(catalog_root) = &result.catalog_root {
                    println!("Catalog root: {}", catalog_root);
                }
                println!("Local config: {}", result.local_config_file);
                println!("Onboarding file: {}", result.onboarding_file);
                match &result.gitignore_file {
                    Some(path) if result.gitignore_updated => {
                        println!("Updated .gitignore: {}", path);
                    }
                    Some(path) => {
                        println!(".gitignore already covered: {}", path);
                    }
                    None => {
                        println!("No .gitignore found; skipped .orpheum/ ignore update");
                    }
                }
            }
        }
        Commands::Scenario(cmd) => match cmd.command {
            ScenarioSubcommand::List(args) => {
                let catalog = load_catalog(catalog_arg, &cwd)?;
                let scenarios = catalog.list_scenarios();
                if args.json {
                    println!("{}", serde_json::to_string_pretty(&scenarios)?);
                } else {
                    print_scenario_list(&scenarios);
                }
            }
            ScenarioSubcommand::Show(args) => {
                let catalog = load_catalog(catalog_arg, &cwd)?;
                let resolved = catalog.resolve_scenario(&args.scenario)?;
                if args.json {
                    println!("{}", serde_json::to_string_pretty(&resolved)?);
                } else {
                    print_scenario_show(&resolved);
                }
            }
            ScenarioSubcommand::Apply(args) => {
                let catalog = load_catalog(catalog_arg, &cwd)?;
                let project = args
                    .project
                    .map(Utf8PathBuf::from)
                    .unwrap_or_else(|| cwd.clone());
                let result = apply_scenario(&catalog, &project, &args.scenario, args.force)?;
                if args.json {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                } else {
                    println!("Applied scenario `{}`", result.scenario_id);
                    println!("Session ID: {}", result.session_id);
                    println!("Control dir: {}", result.control_dir);
                    println!("Current phase: {}", result.current_phase);
                    println!("Next command: {}", result.next_command);
                }
            }
        },
        Commands::Status(args) => {
            let (manifest, snapshot, state, _) = read_session_files(&cwd)?;
            let value = serde_json::json!({
                "session_id": manifest.session_id,
                "scenario_id": snapshot.scenario.id,
                "state": state.state,
                "current_phase": state.current_phase,
                "pending_workflows": state.pending_workflows,
                "artifact_status": state.artifact_status,
                "check_status": state.check_status,
                "cleanup_ready": false
            });
            if args.json {
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("Scenario: {}", snapshot.scenario.title);
                println!("Current phase: {}", state.current_phase);
                println!("Pending workflows: {}", state.pending_workflows.len());
                println!("Artifacts: {}", state.artifact_status.len());
                println!(
                    "Failed checks: {}",
                    state
                        .check_status
                        .values()
                        .filter(|status| matches!(status, CheckStatusValue::Failed))
                        .count()
                );
            }
        }
        Commands::Prompt(cmd) => match cmd.command {
            PromptSubcommand::Current(args) => {
                let prompt = generate_current_prompt(&cwd)?;
                if args.json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({ "prompt": prompt }))?
                    );
                } else {
                    print!("{prompt}");
                }
            }
        },
        Commands::Check(cmd) => match cmd.command {
            CheckSubcommand::Run(args) => {
                let catalog = load_catalog(catalog_arg, &cwd)?;
                match orpheum_core::checks::run_checks(&catalog, &cwd) {
                    Ok(report) => {
                        emit_check_report(&report, args.json)?;
                    }
                    Err(err) if err.code() == orpheum_core::OrpheumErrorCode::CheckFailed => {
                        let log_path = cwd.join(".orpheum").join("logs").join("checks.json");
                        let report: CheckRunReport =
                            serde_json::from_str(&std::fs::read_to_string(log_path)?)?;
                        emit_check_report(&report, args.json)?;
                        return Err(err);
                    }
                    Err(err) => return Err(err),
                }
            }
        },
        Commands::Doctor(args) => {
            let explicit_catalog = catalog_arg.map(Utf8Path::new);
            let report = run_doctor(explicit_catalog, &cwd)?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print_doctor(&report);
            }
        }
    }

    Ok(())
}

fn current_dir_utf8() -> Result<Utf8PathBuf, OrpheumError> {
    let cwd = std::env::current_dir().map_err(OrpheumError::from)?;
    Utf8PathBuf::from_path_buf(cwd).map_err(|_| {
        OrpheumError::coded(
            orpheum_core::OrpheumErrorCode::Io,
            "current directory must be UTF-8",
        )
    })
}

fn load_catalog(catalog_arg: Option<&str>, cwd: &Utf8Path) -> Result<Catalog, OrpheumError> {
    let explicit = catalog_arg.map(Utf8Path::new);
    Catalog::load_runtime(explicit, cwd)
}

fn print_scenario_list(scenarios: &[ScenarioListItem]) {
    for scenario in scenarios {
        println!(
            "{} (v{}): {}",
            scenario.id, scenario.version, scenario.summary
        );
    }
}

fn print_scenario_show(resolved: &ResolvedScenario) {
    println!("Scenario: {}", resolved.scenario.title);
    println!("Summary: {}", resolved.scenario.summary);
    println!("Roles:");
    for role in &resolved.roles {
        println!("  - {}: {}", role.id, role.summary);
    }
    println!("Workflows:");
    for workflow in &resolved.workflows {
        println!("  - {}: {}", workflow.id, workflow.summary);
    }
    println!("Artifacts:");
    for artifact in &resolved.artifacts {
        println!("  - {} -> {}", artifact.id, artifact.default_output_path);
    }
    println!("Checks:");
    for check in &resolved.checks {
        println!("  - {} ({:?})", check.id, check.mode);
    }
}

fn emit_check_report(report: &CheckRunReport, json: bool) -> Result<(), OrpheumError> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
    } else {
        println!("Scenario: {}", report.scenario_id);
        for result in &report.results {
            match &result.artifact_id {
                Some(artifact) => println!(
                    "- {} / {} => {} ({})",
                    result.check_id,
                    artifact,
                    result.status.as_str(),
                    result.message
                ),
                None => println!(
                    "- {} => {} ({})",
                    result.check_id,
                    result.status.as_str(),
                    result.message
                ),
            }
        }
    }
    Ok(())
}

fn print_doctor(report: &DoctorReport) {
    println!("Project state: {}", report.project_state.as_str());
    println!("Catalog source: {}", report.catalog_source.as_str());
    match &report.catalog_root {
        Some(root) => println!("Catalog root: {}", root),
        None => println!("Catalog root: unresolved"),
    }
    println!("Project root: {}", report.project_root);
    println!("Local config: {}", report.local_config.file);
    println!("Local config valid: {}", report.local_config.valid);
    println!(
        "Counts: scenarios={} workflows={} roles={} artifacts={} checks={} skills={}",
        report.counts.scenarios,
        report.counts.workflows,
        report.counts.roles,
        report.counts.artifacts,
        report.counts.checks,
        report.counts.skills
    );
    println!("Active session present: {}", report.active_session_present);
    println!(".orpheum ignored: {}", report.orpheum_gitignored);
    if !report.warnings.is_empty() {
        println!("Warnings:");
        for warning in &report.warnings {
            println!("  - {}: {}", warning.code, warning.message);
        }
    }
    if !report.recovery_commands.is_empty() {
        println!("Recovery commands:");
        for command in &report.recovery_commands {
            println!("  - {command}");
        }
    }
}

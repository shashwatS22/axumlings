mod app_state;
mod cmd;
mod embedded;
mod exercise;
mod info_file;
mod init;
mod run;
mod term;
mod watch;

use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::io::{self, IsTerminal};
use std::path::Path;

#[derive(Parser)]
#[command(name = "axumlings")]
#[command(about = "Small exercises to get you used to Axum, Tokio, and Rust web development!")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Run the current exercise
    Run {
        #[arg(help = "Exercise name")]
        name: Option<String>,
    },
    /// Show hint for the current exercise
    Hint {
        #[arg(help = "Exercise name")]
        name: Option<String>,
    },
    /// Verify all exercises according to the recommended order
    Verify,
    /// Run axumlings in watch mode
    Watch,
    /// List all exercises
    List,
    /// Reset an exercise to its original state
    Reset {
        #[arg(help = "Exercise name")]
        name: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !Path::new("exercises").is_dir() {
        println!("Exercises directory not found. Initializing...");
        init::init()?;
        println!("\nInitialization complete! Run `axumlings` again to start.");
        return Ok(());
    }

    let info = info_file::InfoFile::parse_from_disk()
        .or_else(|_| info_file::InfoFile::parse_from_embedded())?;

    let final_message = info
        .final_message
        .unwrap_or_else(|| "Congratulations! You've completed all exercises.".to_string());

    let (mut app_state, state_file_status) =
        app_state::AppState::new(info.exercises, final_message)?;

    match args.command {
        None | Some(Subcommand::Watch) => {
            if let Some(welcome) = info.welcome_message.as_ref() {
                if matches!(state_file_status, app_state::StateFileStatus::NotRead) {
                    let mut stdout = io::stdout().lock();
                    term::clear_terminal(&mut stdout)?;
                    println!("{}", welcome.trim());
                    println!("\nPress ENTER to continue...");
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    term::clear_terminal(&mut stdout)?;
                }
            }
            if !io::stdout().is_terminal() {
                anyhow::bail!("axumlings watch mode requires a terminal");
            }
            watch::watch(&mut app_state)?;
        }
        Some(Subcommand::Run { name }) => {
            if let Some(n) = name {
                app_state.set_current_exercise_by_name(&n)?;
            }
            run::run(&mut app_state)?;
        }
        Some(Subcommand::Hint { name }) => {
            if let Some(n) = name {
                app_state.set_current_exercise_by_name(&n)?;
            }
            println!("{}", app_state.current_exercise().hint);
        }
        Some(Subcommand::Verify) => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "Verifying all exercises...")?;
            let cmd_runner = cmd::CmdRunner::build()?;
            let mut first_pending = None;
            for i in 0..app_state.exercises().len() {
                let success = app_state
                    .exercises()
                    .get(i)
                    .unwrap()
                    .run_exercise(None, &cmd_runner)?;
                if success {
                    app_state.set_done(i)?;
                } else {
                    app_state.set_pending(i)?;
                    if first_pending.is_none() {
                        first_pending = Some(i);
                    }
                }
            }
            if let Some(ind) = first_pending {
                app_state.set_current_exercise_ind(ind)?;
                writeln!(
                    stdout,
                    "\nFirst pending: {}",
                    app_state.current_exercise().path
                )?;
            } else {
                writeln!(
                    stdout,
                    "\n{} exercises passed!",
                    app_state.exercises().len()
                )?;
            }
        }
        Some(Subcommand::List) => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "Exercises:")?;
            for ex in app_state.exercises() {
                let status = if ex.done { "Done" } else { "Pending" };
                writeln!(stdout, "{:<20} {:<30} [{}]", ex.name, ex.path, status)?;
            }
        }
        Some(Subcommand::Reset { name }) => {
            app_state.set_current_exercise_by_name(&name)?;
            let path = app_state.reset_current_exercise()?;
            println!("Reset {}", path);
        }
    }

    Ok(())
}

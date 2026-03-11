use anyhow::Result;
use crossterm::{style::{Color, ResetColor, SetForegroundColor}, QueueableCommand};
use std::io::{self, Write};

use crate::app_state::AppState;

pub fn run(app_state: &mut AppState) -> Result<()> {
    let mut output = Vec::with_capacity(crate::exercise::OUTPUT_CAPACITY);
    let success = app_state
        .current_exercise()
        .run_exercise(Some(&mut output), &crate::cmd::CmdRunner::build()?)?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output)?;

    if !success {
        app_state.set_pending(app_state.current_exercise_ind())?;
        stdout.write_all(b"Ran ")?;
        stdout.write_all(app_state.current_exercise().path.as_bytes())?;
        stdout.write_all(b" with errors\n")?;
        return Ok(());
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all(b"Successfully ran ")?;
    stdout.write_all(app_state.current_exercise().path.as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n")?;

    if let Some(solution_path) = app_state.current_solution_path()? {
        stdout.write_all(b"\nSolution for comparison: ")?;
        stdout.write_all(solution_path.as_bytes())?;
        stdout.write_all(b"\n")?;
    }

    app_state.set_done(app_state.current_exercise_ind())?;
    if let Some(next_ind) = app_state.next_pending_exercise_ind() {
        app_state.set_current_exercise_ind(next_ind)?;
        stdout.write_all(b"Next exercise: ")?;
        stdout.write_all(app_state.current_exercise().path.as_bytes())?;
        stdout.write_all(b"\n")?;
    }

    Ok(())
}

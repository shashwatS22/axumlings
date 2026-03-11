use anyhow::Result;
use crossterm::{style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor}, QueueableCommand};
use std::io::{self, Read, StdoutLock, Write};

use crate::app_state::AppState;
use crate::exercise;
use crate::term::{self, progress_bar};


#[derive(PartialEq, Eq)]
enum DoneStatus {
    DoneWithSolution(String),
    DoneWithoutSolution,
    Pending,
}

pub struct WatchState<'a> {
    app_state: &'a mut AppState,
    output: Vec<u8>,
    show_hint: bool,
    done_status: DoneStatus,
    term_width: u16,
}

impl<'a> WatchState<'a> {
    pub fn new(app_state: &'a mut AppState, term_width: u16) -> Self {
        Self {
            app_state,
            output: Vec::with_capacity(exercise::OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            term_width,
        }
    }

    pub fn run_current_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        self.show_hint = false;

        writeln!(
            stdout,
            "\nChecking the exercise `{}`. Please wait…",
            self.app_state.current_exercise().name,
        )?;

        let success = self
            .app_state
            .current_exercise()
            .run_exercise(Some(&mut self.output), &crate::cmd::CmdRunner::build()?)?;
        self.output.push(b'\n');

        if success {
            self.done_status = match self.app_state.current_solution_path()? {
                Some(path) => DoneStatus::DoneWithSolution(path),
                None => DoneStatus::DoneWithoutSolution,
            };
            self.app_state.set_done(self.app_state.current_exercise_ind())?;
        } else {
            self.app_state.set_pending(self.app_state.current_exercise_ind())?;
            self.done_status = DoneStatus::Pending;
        }

        self.render(stdout)?;
        Ok(())
    }

    pub fn reset_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        term::clear_terminal(stdout)?;
        write!(stdout, "Resetting {}. Reset (y/n)? ", self.app_state.current_exercise().path)?;
        stdout.flush()?;

        let mut buf = [0u8; 1];
        loop {
            io::stdin().read_exact(&mut buf)?;
            match buf[0] {
                b'y' | b'Y' => {
                    let path = self.app_state.reset_current_exercise()?;
                    writeln!(stdout, "Reset {}", path)?;
                    self.done_status = DoneStatus::Pending;
                    self.run_current_exercise(stdout)?;
                    break;
                }
                b'n' | b'N' => {
                    self.render(stdout)?;
                    break;
                }
                _ => continue,
            }
        }
        Ok(())
    }

    pub fn next_exercise(&mut self, stdout: &mut StdoutLock) -> Result<bool> {
        if self.done_status == DoneStatus::Pending {
            return Ok(false);
        }

        if let Some(next_ind) = self.app_state.next_pending_exercise_ind() {
            self.app_state.set_current_exercise_ind(next_ind)?;
            self.done_status = DoneStatus::Pending;
            self.run_current_exercise(stdout)?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn show_hint(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        if !self.show_hint {
            self.show_hint = true;
            self.render(stdout)?;
        }
        Ok(())
    }

    pub fn check_all_exercises(&mut self, stdout: &mut StdoutLock) -> Result<bool> {
        writeln!(stdout, "\nChecking all exercises...")?;
        stdout.flush()?;

        let cmd_runner = crate::cmd::CmdRunner::build()?;
        let mut first_pending = None;
        let exercise_count = self.app_state.exercises().len();

        for i in 0..exercise_count {
            let success = self.app_state.exercises().get(i).unwrap().run_exercise(None, &cmd_runner)?;
            if success {
                self.app_state.set_done(i)?;
            } else {
                self.app_state.set_pending(i)?;
                if first_pending.is_none() {
                    first_pending = Some(i);
                }
            }
        }

        if let Some(ind) = first_pending {
            self.app_state.set_current_exercise_ind(ind)?;
            self.done_status = DoneStatus::Pending;
            self.run_current_exercise(stdout)?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn show_prompt(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.done_status != DoneStatus::Pending {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            write!(stdout, "n")?;
            stdout.queue(ResetColor)?;
            write!(stdout, ":next / ")?;
        }
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "h")?;
        stdout.queue(ResetColor)?;
        write!(stdout, ":hint / ")?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "c")?;
        stdout.queue(ResetColor)?;
        write!(stdout, ":check all / ")?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "x")?;
        stdout.queue(ResetColor)?;
        write!(stdout, ":reset / ")?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "q")?;
        stdout.queue(ResetColor)?;
        writeln!(stdout, ":quit ?")?;
        stdout.flush()
    }

    pub fn update_term_width(&mut self, width: u16) {
        self.term_width = width;
    }

    pub fn render(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        stdout.write_all(b"\n")?;
        term::clear_terminal(stdout)?;

        stdout.write_all(&self.output)?;

        if self.show_hint {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.queue(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Hint\n")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(self.app_state.current_exercise().hint.as_bytes())?;
            stdout.write_all(b"\n\n")?;
        }

        if self.done_status != DoneStatus::Pending {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.queue(SetForegroundColor(Color::Green))?;
            stdout.write_all(b"Exercise done!\n")?;
            stdout.queue(ResetColor)?;
            if let DoneStatus::DoneWithSolution(ref path) = self.done_status {
                write!(stdout, "Solution for comparison: {}\n", path)?;
            }
            stdout.write_all(b"When done, press n to move to the next exercise.\n\n")?;
        }

        progress_bar(
            stdout,
            self.app_state.n_done(),
            self.app_state.exercises().len() as u16,
            self.term_width,
        )?;
        writeln!(stdout)?;
        write!(stdout, "Current exercise: ")?;
        stdout.write_all(self.app_state.current_exercise().path.as_bytes())?;
        writeln!(stdout)?;
        self.show_prompt(stdout)?;

        Ok(())
    }
}

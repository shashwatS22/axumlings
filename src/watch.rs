mod state;
mod terminal_event;

use anyhow::Result;
use crossterm::terminal;
use std::io::{self, Write};
use std::sync::mpsc;

use crate::app_state::AppState;

pub enum WatchEvent {
    Input(terminal_event::InputEvent),
    FileChange { exercise_ind: usize },
    TerminalResize { width: u16 },
}

pub fn watch(app_state: &mut AppState) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let term_width = terminal::size().unwrap_or((80, 24)).0;

    std::thread::spawn(move || terminal_event::terminal_event_handler(tx));

    let mut watch_state = state::WatchState::new(app_state, term_width);
    let mut stdout = io::stdout().lock();

    watch_state.run_current_exercise(&mut stdout)?;

    while let Ok(event) = rx.recv() {
        match event {
            WatchEvent::Input(terminal_event::InputEvent::Next) => {
                if watch_state.next_exercise(&mut stdout)? {
                    break;
                }
            }
            WatchEvent::Input(terminal_event::InputEvent::Hint) => {
                watch_state.show_hint(&mut stdout)?;
            }
            WatchEvent::Input(terminal_event::InputEvent::CheckAll) => {
                if watch_state.check_all_exercises(&mut stdout)? {
                    break;
                }
            }
            WatchEvent::Input(terminal_event::InputEvent::Reset) => {
                watch_state.reset_exercise(&mut stdout)?;
            }
            WatchEvent::Input(terminal_event::InputEvent::Quit) => {
                stdout.write_all(b"\n\nHappy learning! Run axumlings again to continue.\n")?;
                break;
            }
            WatchEvent::TerminalResize { width } => {
                watch_state.update_term_width(width);
                watch_state.render(&mut stdout)?;
            }
            _ => {}
        }
    }

    Ok(())
}

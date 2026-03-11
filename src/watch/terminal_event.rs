use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::mpsc::Sender;

pub enum InputEvent {
    Next,
    Hint,
    CheckAll,
    Reset,
    Quit,
}

pub fn terminal_event_handler(sender: Sender<super::WatchEvent>) {
    loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                if matches!(key.kind, KeyEventKind::Release | KeyEventKind::Repeat) {
                    continue;
                }
                let input = match key.code {
                    KeyCode::Char('n') => InputEvent::Next,
                    KeyCode::Char('h') => InputEvent::Hint,
                    KeyCode::Char('c') => InputEvent::CheckAll,
                    KeyCode::Char('x') => InputEvent::Reset,
                    KeyCode::Char('q') => InputEvent::Quit,
                    _ => continue,
                };
                let is_quit = matches!(input, InputEvent::Quit);
                if sender.send(super::WatchEvent::Input(input)).is_err() {
                    break;
                }
                if is_quit {
                    break;
                }
            }
            Ok(Event::Resize(width, _)) => {
                let _ = sender.send(super::WatchEvent::TerminalResize { width });
            }
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, StdoutLock, Write};

pub fn progress_bar(
    writer: &mut StdoutLock,
    progress: u16,
    total: u16,
    term_width: u16,
) -> io::Result<()> {
    const PREFIX: &[u8] = b"Progress: [";
    const PREFIX_WIDTH: u16 = 11;
    const POSTFIX_WIDTH: u16 = 10; // "] xxx/xxx"
    const MIN_LINE_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH + 4;

    if term_width < MIN_LINE_WIDTH {
        write!(writer, "Progress: {}/{}", progress, total)?;
        return Ok(());
    }

    let width = term_width - PREFIX_WIDTH - POSTFIX_WIDTH;
    let filled = (width as u32 * progress as u32 / total as u32).min(width as u32) as u16;

    writer.write_all(PREFIX)?;
    writer.queue(SetForegroundColor(Color::Green))?;
    for _ in 0..filled {
        writer.write_all(b"#")?;
    }
    if filled < width {
        writer.write_all(b">")?;
    }
    let remaining = width.saturating_sub(filled).saturating_sub(1);
    if remaining > 0 {
        writer.queue(SetForegroundColor(Color::Red))?;
        for _ in 0..remaining {
            writer.write_all(b"-")?;
        }
    }
    writer.queue(ResetColor)?;
    write!(writer, "] {:>3}/{}", progress, total)?;
    Ok(())
}

pub fn clear_terminal(stdout: &mut StdoutLock) -> io::Result<()> {
    use crossterm::terminal::{Clear, ClearType};
    use crossterm::cursor::MoveTo;
    stdout.queue(MoveTo(0, 0))?;
    stdout.queue(Clear(ClearType::All))?;
    Ok(())
}

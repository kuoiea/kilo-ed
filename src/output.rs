use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::style::Print;
use crossterm::terminal::ClearType;

/// 向每一行的行首增加 ～
pub(crate) fn editor_draw_rows(stdout: &mut Stdout) -> io::Result<()> {
    for row in 0..24 {
        stdout
            .queue(cursor::MoveTo( 0, row))?
            .queue(Print("~".to_string()))?;
    }
    Ok(())
}

/// 清空屏幕
pub(crate) fn clear_screen(stdout: &mut Stdout) -> io::Result<()> {
    stdout
        .queue(terminal::Clear(ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;

    Ok(())
}

pub(crate) fn editor_refresh_screen() -> io::Result<()> {
    let mut stdout = stdout();
    clear_screen(&mut stdout)?;
    editor_draw_rows(&mut stdout)?;

    stdout.queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}
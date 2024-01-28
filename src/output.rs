use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::terminal::ClearType;


pub(crate) fn clear_screen(stdout: &mut Stdout) -> io::Result<()>{
    stdout
        .queue(terminal::Clear(ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;

    Ok(())
}
pub(crate) fn editor_refresh_screen() -> io::Result<()> {
    let mut stdout = stdout();
    clear_screen(&mut stdout)?;
    Ok(())
}
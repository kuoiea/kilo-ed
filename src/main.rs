use std::io::Result;

use crossterm::terminal;
use errno::errno;

use crate::input::editor_process_keypress;

mod keyboard;
mod input;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_process_keypress() {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}


fn die<S: Into<String>>(message: S) {
    terminal::disable_raw_mode().expect("disable raw error");
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
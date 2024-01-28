use std::io::{Result, stdout};

use crossterm::terminal;
use errno::errno;

use crate::input::editor_process_keypress;
use crate::output::{clear_screen, editor_refresh_screen};

mod keyboard;
mod input;
mod output;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_refresh_screen().is_err(){
            die("unable to refresh screen")
        }
        if editor_process_keypress() {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}


fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    clear_screen(&mut stdout)?;
    terminal::disable_raw_mode().expect("disable raw error");
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
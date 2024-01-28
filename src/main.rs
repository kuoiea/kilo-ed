use std::io::Result;

use crossterm::terminal;

use crate::editor::Editor;

mod editor;

fn main() -> Result<()> {
    // Enables raw mode.
    terminal::enable_raw_mode()?;

    // 初始化一个窗口
    let editor = Editor::new()?;
    loop {
        if editor.refresh_screen().is_err() {
            editor.die("unable to refresh screen")
        }
        if editor.process_keypress() {
            break;
        }
    }

    // Disables raw mode.
    terminal::disable_raw_mode()?;
    Ok(())
}



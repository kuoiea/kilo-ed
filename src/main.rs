use std::io::Result;

use crate::editor::Editor;

mod editor;
mod keyboard;
mod my_lib;
mod screen;
fn main() -> Result<()> {
    // 初始化一个窗口
    let mut editor = Editor::new()?;
    editor.start()?;

    Ok(())
}

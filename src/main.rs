use std::io::Result;

use crate::editor::Editor;

mod editor;
mod keyboard;
mod my_lib;
mod screen;
fn main() -> Result<()> {
    let mut args = std::env::args();
    // 初始化一个窗口
    let mut editor = if args.len() >= 2 {
        // 使用窗口打开一个文件
        Editor::with_file(args.nth(1).unwrap())?
    } else {
        // 初始化一个新窗口
        Editor::new()?
    };
    editor.start()?;

    Ok(())
}

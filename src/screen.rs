use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::style::Print;
use crossterm::terminal::ClearType;

#[derive(Debug)]
pub(crate) struct Screen {
    pub(crate) stdout: Stdout,
    width: u16,
    height: u16,
}

impl Screen {
    pub(crate) fn new() -> io::Result<Self> {
        let (columns, rows) = terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
            stdout: stdout(),
        })
    }

    /// 向每一行的行首增加 ～
    pub(crate) fn draw_rows(&mut self) -> io::Result<()> {
        for row in 0..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
        self.stdout.flush()
    }


    /// 清空屏幕
    pub(crate) fn clear(&mut self) -> io::Result<()> {
        self.stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()
    }
}

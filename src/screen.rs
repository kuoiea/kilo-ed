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


    pub(crate) fn draw_rows(&mut self) -> io::Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        // 向每一行的行首增加 ～
        for row in 0..self.height {
            if row == self.height / 3 {
                let mut welcome = format!("Kuoiea's editor --version {VERSION}");
                // 将此 String 缩短至指定长度
                welcome.truncate(self.width as usize);

                if welcome.len() < self.width as usize {
                    // 将欢迎语居中显示到屏幕上
                    let leftmost=( (self.width as usize - welcome.len()) / 2 ) as u16;
                    self.stdout
                        .queue(cursor::MoveTo(0, row))?
                        .queue(Print("~".to_string()))?
                        .queue(cursor::MoveTo(leftmost, row))?
                        .queue(Print(welcome))?;
                } else {
                    // 直接打印到屏幕上
                    self.stdout
                        .queue(cursor::MoveTo(0, row))?
                        .queue(Print(welcome))?;
                }
            } else {
                self.stdout
                    .queue(cursor::MoveTo(0, row))?
                    .queue(Print("~".to_string()))?;
            }
        };

        // 将光标定位到首行
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        // self.stdout.flush()
        Ok(())
    }


    /// 清空屏幕
    pub(crate) fn clear(&mut self) -> io::Result<()> {
        self.stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub(crate) fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
    pub(crate) fn cursor_position(&self) -> io::Result<(u16, u16)> {
        cursor::position()
    }
}

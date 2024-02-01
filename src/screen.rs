use std::io::{self, stdout, Stdout, Write};

use crate::my_lib::Position;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, QueueableCommand};

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

    pub(crate) fn draw_rows(
        &mut self,
        rows: &[String],
        rowoff: u16,
        coloff: u16,
    ) -> io::Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        // 向每一行的行首增加 ～
        for row in 0..self.height {
            let filerow = (row + rowoff) as usize;
            if filerow >= rows.len() {
                if rows.is_empty() && row == self.height / 3 {
                    let mut welcome = format!("Kuoiea's editor --version {VERSION}");
                    // 将此 String 缩短至指定长度
                    welcome.truncate(self.width as usize);

                    if welcome.len() < self.width as usize {
                        // 将欢迎语居中显示到屏幕上
                        let leftmost = ((self.width as usize - welcome.len()) / 2) as u16;
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
            } else {
                // 如果屏幕可以放下当前行，就在屏幕上将当前行打印出来
                let mut len = rows[filerow].len() as u16;
                if len < coloff {
                    continue;
                }

                len -= coloff;
                let start = coloff as usize;
                let end = start
                    + if len >= self.width {
                        self.width as usize
                    } else {
                        len as usize
                    };
                self.stdout
                    .queue(MoveTo(0, row))?
                    // 这里需要限制，如果屏幕宽度不足以放下所有字符，需要将多余的字符截取下来，不在屏幕上进行显示。
                    .queue(Print(rows[filerow][start..end].to_string()))?;
            }
        }

        // 将光标定位到首行

        // self.stdout.flush()
        Ok(())
    }

    /// 清空屏幕
    pub(crate) fn clear(&mut self) -> io::Result<()> {
        self.stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        Ok(())
    }

    pub(crate) fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    // pub(crate) fn cursor_position(&self) -> io::Result<(u16, u16)> {
    //     cursor::position()
    // }

    pub(crate) fn move_to(&mut self, pos: Position, rowoff: u16, coloff: u16) -> io::Result<()> {
        self.stdout.queue(MoveTo(pos.x - coloff, pos.y - rowoff))?;
        Ok(())
    }

    pub(crate) fn bounds(&self) -> Position {
        Position {
            x: self.width,
            y: self.height,
        }
    }
}

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
        // 初始化屏幕信息
        // 获取Terminal 尺寸
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
        // 从环境变量中获取软件版本号，并将其存到常量VERSION
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        // 遍历屏幕可展示的所有行
        for row in 0..self.height {
            // 计算实际应该展示的文件行数（组合上滚动偏移）
            let filerow = (row + rowoff) as usize;
            // 如果实际展示的文件行数超出了文件的总行数
            if filerow >= rows.len() {
                // 如果rows是空的数组并且当前行是屏幕高度的三分之一，
                // 也就是屏幕的中间行
                if rows.is_empty() && row == self.height / 3 {
                    // 生成欢迎信息，并且使用软件版本号
                    let mut welcome = format!("Kuoiea's editor --version {VERSION}");
                    // 如果欢迎信息长度超出屏幕宽度，那就截断它
                    welcome.truncate(self.width as usize);
                    // 若欢迎信息长度小于屏幕宽度，使欢迎信息居中显示
                    if welcome.len() < self.width as usize {
                        // 计算左侧空白填充长度，用于让文字居中
                        let leftmost = ((self.width as usize - welcome.len()) / 2) as u16;
                        // 将文本光标移动到行首
                        self.stdout
                            .queue(cursor::MoveTo(0, row))?
                            // 在行首打印波浪符~
                            .queue(Print("~".to_string()))?
                            // 将光标移动到中间开始位置
                            .queue(cursor::MoveTo(leftmost, row))?
                            // 打印居中的欢迎信息
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
        // 清空屏幕，并将光标移动到 0，0
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

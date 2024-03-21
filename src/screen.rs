use std::io::{self, stdout, Stdout, Write};

use crate::my_lib::Position;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crossterm::{terminal, QueueableCommand};

const KILO_TABLE_STOP: usize = 4;
const TABLE_CHAT: char = '\t';
pub(crate) struct Row {
    chars: String,
    render: String,
}

impl Row {
    pub(crate) fn new(chars: String) -> Self {
        let mut render = String::new();
        let mut idx = 0;
        for c in chars.chars() {
            match c {
                TABLE_CHAT => {
                    render.push(' ');
                    idx += 1;
                    while idx % KILO_TABLE_STOP != 0 {
                        render.push(' ');
                        idx += 1;
                    }
                }
                _ => {
                    render.push(c);
                    idx += 1;
                }
            }
        }
        Self { chars, render }
    }

    pub(crate) fn len(&self) -> usize {
        self.chars.len()
    }
    pub(crate) fn render_len(&self) -> usize {
        self.render.len()
    }

    pub(crate) fn cx_to_rx(&self, cursor_x: u16) -> usize {
        let mut rx = 0;
        for c in self.chars.chars().take(cursor_x as usize) {
            if c == TABLE_CHAT {
                rx += (KILO_TABLE_STOP - 1) - (rx % KILO_TABLE_STOP);
            }
            rx += 1;
        }

        rx
    }
}

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

    pub(crate) fn draw_rows(&mut self, rows: &[Row], rowoff: u16, coloff: u16) -> io::Result<()> {
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
                        // 将文本光标移动到 行首
                        self.stdout
                            .queue(MoveTo(0, row))?
                            // 在行首打印波浪符~
                            .queue(Print("~".to_string()))?
                            // 将光标移动到中间开始位置
                            .queue(MoveTo(leftmost, row))?
                            // 打印居中的欢迎信息
                            .queue(Print(welcome))?;
                    } else {
                        // 直接打印到屏幕上
                        self.stdout.queue(MoveTo(0, row))?.queue(Print(welcome))?;
                    }
                } else {
                    self.stdout
                        .queue(MoveTo(0, row))?
                        .queue(Print("~".to_string()))?;
                }
            } else {
                // 如果屏幕可以放下当前行，就在屏幕上将当前行打印出来
                // 获取当前行的文本长度
                let chars: Vec<char> = rows[filerow].render.chars().collect(); // 将字符串转换为字符向量
                let mut len = chars.len() as u16;
                // 如果文本长度小于屏幕偏移量，那么表示当前行没有文本可以在屏幕上进行展示，跳过此行。
                if len < coloff {
                    continue;
                }
                // 文本长度减去偏移量，等于剩下的文本长度
                len -= coloff;
                // 下面这段代码计算需要截取的字符串的长度
                // 需要截取字符串的开始长度，也就是屏幕的偏移量
                let start = coloff as usize;
                let end = start
                    // 如果文本长度大于 屏幕宽度，那么表示屏幕无法完全展示， 所以要以屏幕宽度为准， 否则以剩下的文本长度为准
                    + if len >= self.width {
                    self.width as usize
                } else {
                    len as usize
                };
                self.stdout
                    .queue(MoveTo(0, row))?
                    // 这里需要限制，如果屏幕宽度不足以放下所有字符，需要将多余的字符截取下来，不在屏幕上进行显示。
                    .queue(Print(chars[start..end].iter().collect::<String>()))?;
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

    pub(crate) fn move_to(
        &mut self,
        pos: Position,
        rowoff: u16,
        coloff: u16,
        render_x: u16,
    ) -> io::Result<()> {
        self.stdout
            .queue(MoveTo(render_x - coloff, pos.y - rowoff))?;
        Ok(())
    }

    pub(crate) fn bounds(&self) -> Position {
        Position {
            x: self.width,
            y: self.height,
        }
    }
}

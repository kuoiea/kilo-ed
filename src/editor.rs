use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use errno::errno;
use std::collections::HashMap;
use std::io::{self};
use std::path::Path;

use crate::keyboard::Keyboard;
use crate::my_lib::Position;
use crate::screen::Screen;

#[derive(Copy, Clone)]
pub(crate) enum EditorKey {
    // a
    Left,
    // d
    Right,
    // w
    Up,
    // s
    Down,
}

pub(crate) struct Editor {
    screen: Screen, // 屏幕信息
    keyboard: Keyboard,
    cursor: Position,                 // 光标在屏幕上的位置
    keymap: HashMap<char, EditorKey>, // 键盘映射关系
    rows: Vec<String>,                // 屏幕展示上的内容
    rowoff: u16,                      // 垂直滚动
    coloff: u16,                      // 水平滚动
}

impl Editor {
    pub(crate) fn with_file(file_name: impl AsRef<Path>) -> io::Result<Self> {
        // 打开文件路径，并使用\n进行分割，按行读取
        let lines = std::fs::read_to_string(file_name)
            .expect("不能打开文件")
            .split('\n')
            .map(|x| x.into())
            .collect::<Vec<String>>();
        // 读取文件中的行数剧， 并传递给编辑器
        Editor::build(&lines)
    }

    pub(crate) fn new() -> io::Result<Self> {
        // 创建一个新的窗口， 传递空数组给编辑器
        Editor::build(&[])
    }
    pub(crate) fn build(data: &[String]) -> io::Result<Self> {
        // 创建一个映射表，将按键映射为枚举类型
        let mut keymap: HashMap<char, EditorKey> = HashMap::new();
        keymap.insert('w', EditorKey::Up);
        keymap.insert('a', EditorKey::Left);
        keymap.insert('s', EditorKey::Down);
        keymap.insert('d', EditorKey::Right);
        // 初始化窗口信息
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            keymap,
            rows: if data.is_empty() {
                Vec::new()
            } else {
                Vec::from(data)
            },
            rowoff: 0,
            coloff: 0,
        })
    }

    pub(crate) fn start(&mut self) -> io::Result<()> {
        // 启用原生模式
        terminal::enable_raw_mode()?;
        // 循环监听用户按键输入
        loop {
            // 如果刷新屏幕的时候报错
            if self.refresh_screen().is_err() {
                // 退出进程
                self.die("unable to refresh screen")
            }

            // 移动光标
            self.screen.move_to(self.cursor, self.rowoff, self.coloff)?;
            // 刷新屏幕
            self.screen.flush()?;

            if self.process_keypress()? {
                break;
            }
        }
        // 禁用原生模式
        terminal::disable_raw_mode()
    }
    // 公有但受限制的函数，用来处理用户的按键输入。
    // 返回一个结果，指明是否因特定按键组合而退出程序。
    pub(crate) fn process_keypress(&mut self) -> io::Result<bool> {
        // 尝试从键盘读入字符，如果读入成功则进入匹配流程。
        if let Ok(c) = self.keyboard.read() {
            // 根据读入的按键事件匹配对应的操作。
            match c {
                // 匹配按下了'q'键，并且附带CONTROL修饰符（即Ctrl+q）,
                // 如果匹配成功，则返回Ok(true)，表示需要退出程序。
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => return Ok(true),
                // 匹配按下了其他字符键，进一步确定是哪个字符。
                KeyEvent {
                    code: KeyCode::Char(key_code),
                    ..
                } => match key_code {
                    // 如果是 'w'、'a'、's' 或 'd'，查询键位映射表，处理光标移动。
                    'w' | 'a' | 's' | 'd' => {
                        let c = self.keymap.get(&key_code).unwrap();
                        self.move_cursor(*c)
                    }
                    // 对于其他字符，不执行任何操作。
                    _ => {}
                },
                // 匹配非字符类按键，进行相应的光标移动处理。
                KeyEvent { code, .. } => match code {
                    // Home键被按下，光标移动到当前行的起始位置。
                    KeyCode::Home => self.cursor.x = 0,
                    // End键被按下，光标移动到当前屏幕宽度的末尾（减去1是因为索引是从0开始的）。
                    KeyCode::End => self.cursor.x = self.screen.bounds().x - 1,
                    // 箭头键被按下时，根据按键调用move_cursor函数来移动光标位置。
                    KeyCode::Up => self.move_cursor(EditorKey::Up),
                    KeyCode::Left => self.move_cursor(EditorKey::Left),
                    KeyCode::Right => self.move_cursor(EditorKey::Right),
                    KeyCode::Down => self.move_cursor(EditorKey::Down),
                    // PageDown和PageUp键被按下时，对当前屏幕高度的每一行都进行光标移动操作。
                    KeyCode::PageDown | KeyCode::PageUp => {
                        let bounds = self.screen.bounds();
                        for _ in 0..bounds.y {
                            self.move_cursor(if code == KeyCode::PageUp {
                                EditorKey::Up
                            } else {
                                EditorKey::Down
                            })
                        }
                    }
                    KeyCode::Delete => {}
                    _ => {}
                },
            }
        } else {
            self.die("Unable to read keyboard");
        }
        Ok(false)
    }

    /// 刷新屏幕
    pub(crate) fn refresh_screen(&mut self) -> io::Result<()> {
        self.scroll();
        // 清除屏幕信息
        self.screen.clear()?;
        // 在屏幕上绘制信息
        self.screen.draw_rows(&self.rows, self.rowoff, self.coloff)
    }

    pub(crate) fn die<S: Into<String>>(&mut self, message: S) {
        // 清除屏幕
        let _ = self.screen.clear();
        // 禁用原始模式
        terminal::disable_raw_mode().expect("disable raw error");
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    /// 匹配键位， 移动光标
    /// move_cursor函数：根据传入的EditorKey枚举值匹配键位，并移动光标位置
    pub(crate) fn move_cursor(&mut self, key: EditorKey) {
        // 获取当前光标所在的行数，如果超过文本行数的长度，则返回None
        let row_index = if self.cursor.y as usize >= self.rows.len() {
            None
        } else {
            Some(self.cursor.y as usize)
        };
        // 这一行是被注释掉的，原本可以用来获取屏幕的大小，可能不需要了因为有其他限制条件
        // let bounds = self.screen.bounds();

        // 这个match语句匹配key参数所表示的方向键，然后相应地移动光标
        match key {
            // 如果按下的是向上的键，使用saturating_sub防止减去1后变为负数导致的越界问题
            EditorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            // 如果按下的是向左的键，同样使用saturating_sub来移动光标，避免越界
            EditorKey::Left => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            // 如果按下的是向下的键并且光标不在最后一行，
            // 这里通过条件判断确保光标不会移动到文本行数之外
            EditorKey::Down if (self.cursor.y as usize) < self.rows.len() => {
                self.cursor.y = self.cursor.y.saturating_add(1);
            }
            // 如果按下的是向右的键，使用saturating_add来移动光标，以避免数值溢出
            EditorKey::Right => {
                if let Some(idx) = row_index {
                    // 获取当前行的文本长度， 并且和光标所在x轴位置做比较，如果光标所在位置小于文本长度，则可以向右继续移动，否则，光标不动。
                    if (self.rows[idx].len() as u16) > self.cursor.x {
                        self.cursor.x = self.cursor.x.saturating_add(1);
                    }
                }
            }
            // 其他键位不进行操作
            _ => {}
        };
        // 匹配操作结束

        // 下面这几行代码是为了防止用户按键向下移动时，从比较长地行移动到短地行，光标没有紧贴行尾的问题。
        // 如果用户按键向下移动，这里需要重新计算选中行文本长度信息
        let row_length = if self.cursor.y as usize >= self.rows.len() {
            0
        } else {
            self.rows[self.cursor.y as usize].len() as u16
        };
        // 计算文本长度和光标位置的最小值， 并将最小值重新赋值给光标的X坐标
        self.cursor.x = self.cursor.x.min(row_length);
    }

    pub(crate) fn scroll(&mut self) {
        // 获取屏幕边界信息（Terminal 大小）
        let bounds: Position = self.screen.bounds();
        // 检测如果光标的垂直位置（y坐标）小于屏幕滚动的起始行，我们需要更新屏幕滚动的起始行
        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y;
        }
        // 如果光标的y坐标大于或等于可视区域的最后一行，更新屏幕滚动的起始行以使光标在可视区域
        if self.cursor.y >= (self.rowoff + bounds.y) {
            self.rowoff = self.cursor.y - bounds.y
        }

        // 计算水平滚动
        // 检测如果光标的水平位置（x坐标）小于屏幕滚动的起始列，我们需要更新屏幕滚动的起始列
        if self.cursor.x < self.coloff {
            self.coloff = self.cursor.x;
        }
        // 如果光标的x坐标大于或等于可视区域的最右列，更新屏幕滚动的起始列以使光标在可视区域
        if self.cursor.x >= self.coloff + bounds.x {
            self.coloff = self.cursor.x - bounds.x + 1;
        }
    }
}

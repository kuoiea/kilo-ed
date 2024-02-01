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
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    keymap: HashMap<char, EditorKey>,
    rows: Vec<String>,
}

impl Editor {
    pub(crate) fn with_file(file_name: impl AsRef<Path>) -> io::Result<Self> {
        let lines = std::fs::read_to_string(file_name)
            .expect("不能打开文件")
            .split('\n')
            .map(|x| x.into())
            .collect::<Vec<String>>();

        Editor::build(&lines)
    }

    pub(crate) fn new() -> io::Result<Self> {
        Editor::build(&[])
    }
    pub(crate) fn build(data: &[String]) -> io::Result<Self> {
        let mut keymap: HashMap<char, EditorKey> = HashMap::new();
        keymap.insert('w', EditorKey::Up);
        keymap.insert('a', EditorKey::Left);
        keymap.insert('s', EditorKey::Down);
        keymap.insert('d', EditorKey::Right);
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
        })
    }

    pub(crate) fn start(&mut self) -> io::Result<()> {
        // Enables raw mode.
        terminal::enable_raw_mode()?;
        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen")
            }
            self.screen.move_to(self.cursor)?;
            self.screen.flush()?;

            if self.process_keypress()? {
                break;
            }
        }
        // Disables raw mode.
        terminal::disable_raw_mode()
    }
    /// 处理按键
    pub(crate) fn process_keypress(&mut self) -> io::Result<bool> {
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => return Ok(true),
                KeyEvent {
                    code: KeyCode::Char(key_code),
                    ..
                } => match key_code {
                    'w' | 'a' | 's' | 'd' => {
                        let c = self.keymap.get(&key_code).unwrap();
                        self.move_cursor(*c)
                    }
                    _ => {}
                },
                KeyEvent { code, .. } => match code {
                    KeyCode::Home => self.cursor.x = 0,
                    KeyCode::End => self.cursor.x = self.screen.bounds().x - 1,
                    KeyCode::Up => self.move_cursor(EditorKey::Up),
                    KeyCode::Left => self.move_cursor(EditorKey::Left),
                    KeyCode::Right => self.move_cursor(EditorKey::Right),
                    KeyCode::Down => self.move_cursor(EditorKey::Down),
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
        self.screen.clear()?;
        self.screen.draw_rows(&self.rows)
    }

    pub(crate) fn die<S: Into<String>>(&mut self, message: S) {
        let _ = self.screen.clear();
        terminal::disable_raw_mode().expect("disable raw error");
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    /// 匹配键位， 移动光标
    pub(crate) fn move_cursor(&mut self, key: EditorKey) {
        let bounds = self.screen.bounds();

        //saturating_sub/saturating_add 它将其限制在有效范围内,防止越界
        match key {
            EditorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            EditorKey::Left => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            // 添加条件判断，不允许光标超出屏幕范围
            EditorKey::Down if self.cursor.y <= bounds.y => {
                self.cursor.y = self.cursor.y.saturating_add(1);
            }
            EditorKey::Right if self.cursor.x <= bounds.x => {
                self.cursor.x = self.cursor.x.saturating_add(1);
            }
            _ => {}
        };
    }
}

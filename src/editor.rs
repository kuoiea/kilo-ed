use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::event::Event::Key;
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use errno::errno;

pub(crate) struct Editor {
    width: u16,
    height: u16,
}


impl Editor {
    pub(crate) fn new() -> io::Result<Self> {
        let (columns, rows) = terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
        })
    }

    pub(crate) fn process_keypress(&self) -> bool {
        let c = self.read_key();
        match c {
            Ok(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. }) => true,
            _ => false
        }
    }

    /// 向每一行的行首增加 ～
    pub(crate) fn draw_rows(&self, stdout: &mut Stdout) -> io::Result<()> {
        for row in 0..24 {
            stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
        Ok(())
    }

    /// 清空屏幕
    pub(crate) fn clear_screen(&self, stdout: &mut Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;

        Ok(())
    }

    pub(crate) fn refresh_screen(&self) -> io::Result<()> {
        let mut stdout = stdout();
        self.clear_screen(&mut stdout)?;
        self.draw_rows(&mut stdout)?;

        stdout.queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub(crate) fn die<S: Into<String>>(&self, message: S) {
        let mut stdout = stdout();
        let _ = self.clear_screen(&mut stdout);
        terminal::disable_raw_mode().expect("disable raw error");
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    /// 监听键入的值
    pub(crate) fn read_key(&self) -> Result<KeyEvent, ()> {
        loop {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                self.die("read");
                break;
            }
        }
        unreachable!()
    }
}


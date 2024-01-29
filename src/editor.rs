use std::io::{self, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use errno::errno;
use crate::keyboard::Keyboard;
use crate::my_lib::{Position, ResultCode};
use crate::screen::Screen;


pub(crate) struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
}

impl Editor {
    pub(crate) fn new() -> io::Result<Self> {
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
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
    pub(crate) fn process_keypress(&mut self) -> io::Result<bool> {
        self.screen.move_to(self.cursor)?;
        let c = self.keyboard.read();
        match c {
            Ok(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. }) => Ok(true),
            Err(ResultCode::KeyReadFail) => {
                self.die("Unable to read keyboard");
                Ok(false)
            }
            _ => Ok(false)
        }
    }

    pub(crate) fn refresh_screen(&mut self) -> io::Result<()> {
        self.screen.clear()?;
        self.screen.draw_rows()?;
        Ok(())
    }

    pub(crate) fn die<S: Into<String>>(&mut self, message: S) {
        let _ = self.screen.clear();
        terminal::disable_raw_mode().expect("disable raw error");
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }
}


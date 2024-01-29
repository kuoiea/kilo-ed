use std::io::{self, Write};

use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use errno::errno;
use crate::keyboard::Keyboard;
use crate::my_lib::ResultCode;
use crate::screen::Screen;

pub(crate) struct Editor {
    screen: Screen,
    keyboard: Keyboard,
}

impl Editor {
    pub(crate) fn new() -> io::Result<Self> {
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
        })
    }

    pub(crate) fn start(&mut self) -> io::Result<()> {
        // Enables raw mode.
        terminal::enable_raw_mode()?;
        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen")
            }

            self.screen.flush()?;

            if self.process_keypress() {
                break;
            }
        }
        // Disables raw mode.
        terminal::disable_raw_mode()
    }
    pub(crate) fn process_keypress(&mut self) -> bool {
        let c = self.keyboard.read();
        match c {
            Ok(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. }) => true,
            Err(ResultCode::KeyReadFail) => {
                self.die("Unable to read keyboard");
                false
            }
            _ => false
        }
    }

    pub(crate) fn refresh_screen(&mut self) -> io::Result<()> {
        self.screen.clear()?;
        self.screen.draw_rows()?;

        self.screen.stdout.queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub(crate) fn die<S: Into<String>>(&mut self, message: S) {
        let _ = self.screen.clear();
        terminal::disable_raw_mode().expect("disable raw error");
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }
}


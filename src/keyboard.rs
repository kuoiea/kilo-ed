use crate::my_lib::{EditorResult, ResultCode};
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyEvent};

pub(crate) struct Keyboard;

impl Keyboard {
    /// 监听键入的值
    pub(crate) fn read(&mut self) -> EditorResult<KeyEvent, ResultCode> {
        loop {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                return Err(ResultCode::KeyReadFail);
            }
        }
    }
}

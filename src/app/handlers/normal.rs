use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{App, InputMode};

pub enum NormalModeAction {
    Continue,
    Exit,
}

pub fn normal_mode_handler(app: &mut App, key: KeyEvent) -> io::Result<NormalModeAction> {
    match key.code {
        KeyCode::Char('e') => {
            app.input_mode = InputMode::Editing;
            return Ok(NormalModeAction::Continue);
        }
        KeyCode::Char('q') => {
            return Ok(NormalModeAction::Exit);
        }
        _ => return Ok(NormalModeAction::Continue),
    }
}

pub mod edit;
pub mod normal;

use std::io;

use crossterm::event::{self, Event};

use crate::app::handlers::normal::NormalModeAction;
use crate::{App, InputMode};

pub enum KeyEventAction {
    Continue,
    Exit,
}

pub fn key_event_handler(app: &mut App) -> io::Result<KeyEventAction> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => {
                let result = normal::normal_mode_handler(app, key);
                match result {
                    Ok(NormalModeAction::Continue) => return Ok(KeyEventAction::Continue),
                    Ok(NormalModeAction::Exit) => return Ok(KeyEventAction::Exit),
                    Err(_) => {
                        todo!("Figure this out")
                    }
                }
            }
            InputMode::Editing => {
                edit::edit_mode_handler(app, key);
                Ok(KeyEventAction::Continue)
            }
        }
    } else {
        return Ok(KeyEventAction::Continue);
    }
}

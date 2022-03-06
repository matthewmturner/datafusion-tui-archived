pub mod app;

use std::io;
use tui::{backend::Backend, Terminal};

use crate::app::handlers::{key_event_handler, KeyEventAction};
use crate::app::ui;

enum InputMode {
    Normal,
    Editing,
}

struct Editor {
    /// Current line in editor
    current_row: u16,
    /// Current column in editor
    current_column: u16,
    /// Vec of line lengths
    line_lengths: Vec<u16>,
    /// Flag if SQL statement was terminated with ';'
    sql_terminated: bool,
}
impl Default for Editor {
    fn default() -> Editor {
        let mut line_lengths = Vec::new();
        line_lengths.push(0);
        Editor {
            current_row: 1,
            current_column: 1,
            line_lengths,
            sql_terminated: false,
        }
    }
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    sql_history: Vec<String>,
    /// Editor
    editor: Editor,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            sql_history: Vec::new(),
            editor: Editor::default(),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::generate_ui(f, app))?;

        let result = key_event_handler(app);
        match result {
            Ok(KeyEventAction::Continue) => {}
            Ok(KeyEventAction::Exit) => return Ok(()),
            Err(_) => return Ok(()),
        }
    }
}

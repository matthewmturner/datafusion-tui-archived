pub mod app;

use crossterm::event::{self, Event, KeyCode};
use std::cmp;
use std::io;
use tui::{backend::Backend, Terminal};

use crate::app::ui;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    /// Current line in editor
    editor_line: u16,
    /// Current column in editor
    editor_column: u16,
    /// SQL statement terminated with ';'
    sql_terminated: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            editor_line: 1,
            editor_column: 1,
            sql_terminated: false,
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::generate_ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => match app.sql_terminated {
                        false => {
                            app.input.push('\n');
                            app.editor_line += 1;
                            app.editor_column = 1;
                        }
                        true => {
                            app.messages.push(app.input.drain(..).collect());
                            app.editor_line = 1;
                            app.editor_column = 1;
                        }
                    },
                    KeyCode::Char(c) => {
                        match c {
                            ';' => {
                                app.input.push(c);
                                app.sql_terminated = true;
                            }
                            _ => {
                                app.input.push(c);
                            }
                        }
                        app.editor_column += 1;
                    }
                    KeyCode::Backspace => {
                        // TODO: Handle backspace with newlines
                        app.input.pop();
                        let column = cmp::max(1, app.editor_column - 1);
                        app.editor_column = column;
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

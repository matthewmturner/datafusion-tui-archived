use std::cmp;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{App, InputMode};

pub fn edit_mode_handler(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => match app.editor.sql_terminated {
            false => {
                app.input.push('\n');
                app.editor.current_row += 1;
                app.editor.line_lengths.push(app.editor.current_column);
                app.editor.current_column = 1;
            }
            true => {
                app.sql_history.push(app.input.drain(..).collect());
                app.editor.current_row = 1;
                app.editor.current_column = 1;
                app.editor.sql_terminated = false
            }
        },
        KeyCode::Char(c) => {
            match c {
                ';' => {
                    app.input.push(c);
                    app.editor.sql_terminated = true;
                }
                _ => {
                    app.input.push(c);
                }
            }
            app.editor.current_column += 1;
        }
        KeyCode::Backspace => {
            // TODO: Handle backspace with multiple newlines and blanks
            let last = app.input.pop();
            if let Some(char) = last {
                match char {
                    '\n' => {
                        let previous_row = app.editor.current_row - 1;
                        app.editor.current_column = app.editor.line_lengths[previous_row as usize];
                        app.editor.current_row -= 1;
                    }
                    _ => {
                        let column = cmp::max(1, app.editor.current_column - 1);
                        app.editor.current_column = column;
                    }
                }
            }
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

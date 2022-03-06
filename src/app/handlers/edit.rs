// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::cmp;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{App, InputMode};

pub async fn edit_mode_handler(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => enter_handler(app).await,
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
        KeyCode::Up => {
            let sql = app.sql_history.last();
            match sql {
                Some(sql) => app.input = sql.clone(),
                None => {}
            }
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

async fn enter_handler(app: &mut App) {
    match app.editor.sql_terminated {
        false => {
            app.input.push('\n');
            app.editor.current_row += 1;
            app.editor.line_lengths.push(app.editor.current_column);
            app.editor.current_column = 1;
        }
        true => {
            let sql: String = app.input.drain(..).collect();
            app.sql_history.push(sql.clone());
            app.editor.current_row = 1;
            app.editor.current_column = 1;
            app.editor.sql_terminated = false;
            // TODO: Remove unwrap and add result / action
            let df = app.context.sql(&sql).await;
            match df {
                Ok(df) => {}
                Err(e) => {
                    let err_msg = format!("{}", e);
                    app.sql_history.push(err_msg)
                }
            }
        }
    }
}

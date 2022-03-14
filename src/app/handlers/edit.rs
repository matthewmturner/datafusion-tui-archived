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

use std::io;
use std::time::Instant;

use crate::app::datafusion::context::QueryResults;
use crate::app::ui::Scroll;
use crate::app::{App, AppReturn, InputMode};
use crate::events::Key;

pub async fn edit_mode_handler(app: &mut App, key: Key) -> io::Result<AppReturn> {
    // TODO: Move cursor with arrow keys
    match key {
        Key::Enter => enter_handler(app).await,
        Key::Char(c) => match c {
            ';' => {
                app.editor.input.append_char(c);
                app.editor.sql_terminated = true;
            }
            _ => {
                app.editor.input.append_char(c);
            }
        },
        Key::Tab => app.editor.input.tab(),
        Key::Backspace => {
            app.editor.input.backspace();
        }
        Key::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    };
    Ok(AppReturn::Continue)
}

async fn enter_handler(app: &mut App) {
    match app.editor.sql_terminated {
        false => {
            app.editor.input.append_char('\n');
        }
        true => {
            let sql: String = app.editor.input.combine_lines();
            app.editor.history.push(sql.clone());
            app.editor.sql_terminated = false;

            let now = Instant::now();
            let df = app.context.sql(&sql).await;
            match df {
                Ok(df) => {
                    let batches = df.collect().await.unwrap();
                    let query_duration = now.elapsed().as_secs_f64();
                    let rows: usize = batches.iter().map(|b| b.num_rows()).sum();
                    app.query_results = Some(QueryResults {
                        // TODO: Remove unwrap and add result / action
                        batches,
                        rows,
                        query_duration,
                        scroll: Scroll { x: 0, y: 0 },
                    });
                }
                Err(e) => {
                    let err_msg = format!("{}", e);
                    app.editor.history.push(err_msg)
                }
            }
        }
    }
}

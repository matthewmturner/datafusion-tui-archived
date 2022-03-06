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

pub mod app;

use std::io;

use datafusion::prelude::*;
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
    /// Rows to scroll the editor
    scroll: u16,
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
            scroll: 0,
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
    /// DataFusion `ExecutionContext`
    context: ExecutionContext,
}

impl Default for App {
    fn default() -> App {
        let config = ExecutionConfig::new().with_information_schema(true);
        let ctx = ExecutionContext::with_config(config);

        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            sql_history: Vec::new(),
            editor: Editor::default(),
            context: ctx,
        }
    }
}

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::generate_ui(f, app))?;

        let result = key_event_handler(app).await;
        match result {
            Ok(KeyEventAction::Continue) => {}
            Ok(KeyEventAction::Exit) => return Ok(()),
            Err(_) => return Ok(()),
        }
    }
}

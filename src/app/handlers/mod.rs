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

pub async fn key_event_handler(app: &mut App) -> io::Result<KeyEventAction> {
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
                edit::edit_mode_handler(app, key).await;
                Ok(KeyEventAction::Continue)
            }
        }
    } else {
        return Ok(KeyEventAction::Continue);
    }
}

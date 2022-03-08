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

use arrow::record_batch::RecordBatch;
use arrow::util::display::array_value_to_string;
use tui::{
    style::{Color, Style},
    widgets::{Cell, Row, Table, TableState},
};

pub fn create_table(batches: &[RecordBatch]) {
    let schema = batches[0].schema();
    let header_cells = Vec::new();
    for field in schema.fields() {
        let cell = Cell::from(field.name().as_str()).style(Style::default().fg(Color::Red));
        header_cells.push(cell)
    }
    let header = Row::new(header_cells).height(1).bottom_margin(1);
}

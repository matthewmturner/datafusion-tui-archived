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
use tui::widgets::TableState;
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub struct QueryResultTable<'a> {
    pub state: TableState,
    pub header: Row<'a>,
    pub rows: Vec<Row<'a>>,
    pub columns: Vec<Constraint>,
    pub table: Table<'a>,
}

impl<'a> QueryResultTable<'a> {
    pub fn new(
        state: TableState,
        header: Row<'a>,
        rows: Vec<Row<'a>>,
        columns: Vec<Constraint>,
    ) -> QueryResultTable<'a> {
        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .widths(&columns);
        QueryResultTable {
            state,
            header,
            rows,
            columns,
            table,
        }
    }
}

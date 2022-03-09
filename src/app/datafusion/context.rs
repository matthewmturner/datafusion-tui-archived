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

//! Context (remote or local)

use arrow::record_batch::RecordBatch;
use arrow::util::display::array_value_to_string;
use datafusion::dataframe::DataFrame;
use datafusion::error::DataFusionError;
use datafusion::execution::context::{ExecutionConfig, ExecutionContext};
use std::fmt::Display;
use std::sync::Arc;
use tui::layout::Constraint;
use tui::widgets::TableState;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use super::display::QueryResultTable;

/// The CLI supports using a local DataFusion context or a distributed BallistaContext
// pub enum Context {
/// In-process execution with DataFusion
// Local(ExecutionContext),
/// Distributed execution with Ballista (if available)
// Remote(BallistaContext),
// }

// impl Context {
//     /// create a new remote context with given host and port
//     pub fn new_remote(host: &str, port: u16) -> Result<Context> {
//         Ok(Context::Remote(BallistaContext::try_new(host, port)?))
//     }

//     /// create a local context using the given config
//     pub fn new_local(config: &ExecutionConfig) -> Context {
//         Context::Local(ExecutionContext::with_config(config.clone()))
//     }

//     /// execute an SQL statement against the context
//     pub async fn sql(&mut self, sql: &str) -> Result<Arc<dyn DataFrame>> {
//         match self {
//             Context::Local(datafusion) => datafusion.sql(sql).await,
//             Context::Remote(ballista) => ballista.sql(sql).await,
//         }
//     }
// }

pub struct QueryResults<'a> {
    pub batches: Vec<RecordBatch>,
    pub table: QueryResultTable<'a>,
}

impl<'a> QueryResults<'a> {
    pub fn new(batches: Vec<RecordBatch>) -> QueryResults<'a> {
        let table = QueryResults::create_table(&batches);
        QueryResults { batches, table }
    }

    fn create_table(batches: &[RecordBatch]) -> QueryResultTable {
        let schema = batches[0].schema();
        let mut header_cells = Vec::new();
        let mut columns = Vec::new();
        for field in schema.fields() {
            let cell = Cell::from(field.name().to_owned()).style(Style::default().fg(Color::Red));
            header_cells.push(cell);
            columns.push(Constraint::Length(10))
        }
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(1)
            .style(Style::default().add_modifier(Modifier::BOLD));

        let mut rows = Vec::new();
        for batch in batches {
            for row in 0..batch.num_rows() {
                let mut cells = Vec::new();
                for col in 0..batch.num_columns() {
                    let column = batch.column(col);
                    cells.push(Cell::from(array_value_to_string(column, row).unwrap()))
                }
                rows.push(Row::new(cells).height(1).bottom_margin(1))
            }
        }
        QueryResultTable::new(TableState::default(), header, rows, columns)
    }
}

// implement wrappers around the BallistaContext to support running without ballista

#[cfg(feature = "ballista")]
pub struct BallistaContext(ballista::context::BallistaContext);
#[cfg(feature = "ballista")]
impl BallistaContext {
    pub fn try_new(host: &str, port: u16) -> Result<Self> {
        use ballista::context::BallistaContext;
        use ballista::prelude::BallistaConfig;
        let config: BallistaConfig =
            BallistaConfig::new().map_err(|e| DataFusionError::Execution(format!("{:?}", e)))?;
        Ok(Self(BallistaContext::remote(host, port, &config)))
    }
    pub async fn sql(&mut self, sql: &str) -> Result<Arc<dyn DataFrame>> {
        self.0.sql(sql).await
    }
}

// #[cfg(not(feature = "ballista"))]
// pub struct BallistaContext();
// #[cfg(not(feature = "ballista"))]
// impl BallistaContext {
//     pub fn try_new(_host: &str, _port: u16) -> Result<Self> {
//         Err(DataFusionError::NotImplemented(
//             "Remote execution not supported. Compile with feature 'ballista' to enable".to_string(),
//         ))
//     }
//     pub async fn sql(&mut self, _sql: &str) -> Result<Arc<dyn DataFrame>> {
//         unreachable!()
//     }
// }
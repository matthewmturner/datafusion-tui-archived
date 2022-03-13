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

use arrow::util::pretty::pretty_format_batches;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use tui_logger::TuiLoggerWidget;

use crate::app::{App, InputMode};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.tabs.index {
        // SQL Editor
        0 => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(30),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let help_message = draw_help(app);
            f.render_widget(help_message, chunks[0]);

            let tabs = draw_tabs(app);
            f.render_widget(tabs, chunks[1]);
            let editor = draw_editor(app);
            f.render_widget(editor, chunks[2]);
            draw_cursor(app, f, &chunks);
            let query_results = draw_query_results(app);
            f.render_widget(query_results, chunks[3]);
        }
        // Query History
        1 => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let help_message = draw_help(app);
            f.render_widget(help_message, chunks[0]);

            let tabs = draw_tabs(app);
            f.render_widget(tabs, chunks[1]);
            let query_history = draw_query_history(app);
            f.render_widget(query_history, chunks[2])
        }
        2 => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let help_message = draw_help(app);
            f.render_widget(help_message, chunks[0]);

            let tabs = draw_tabs(app);
            f.render_widget(tabs, chunks[1]);
            let logs = draw_logs();
            f.render_widget(logs, chunks[2])
        }
        _ => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let help_message = draw_help(app);
            f.render_widget(help_message, chunks[0]);

            let tabs = draw_tabs(app);
            f.render_widget(tabs, chunks[1]);
        }
    }
}

fn draw_help<'a>(app: &mut App) -> Paragraph<'a> {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

fn draw_editor<'a>(app: &mut App) -> Paragraph<'a> {
    Paragraph::new(app.editor.input.combine_lines())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("SQL Editor"))
}

fn draw_cursor<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.editor.get_cursor_column() + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + app.editor.get_cursor_row() + 1,
            )
        }
    };
}

fn draw_query_results<'a>(app: &'a mut App) -> Paragraph<'a> {
    let query_results = match &app.query_results {
        Some(results) => {
            let query = app.editor.history.last().unwrap();
            if query.starts_with("CREATE") {
                Paragraph::new(String::from("Table created"))
            } else {
                let table = pretty_format_batches(&results.batches).unwrap().to_string();
                Paragraph::new(table)
            }
        }
        None => {
            let last_query = app.editor.history.last();
            match last_query {
                Some(query) => Paragraph::new(query.as_str()),
                None => Paragraph::new("No queries yet"),
            }
        }
    };
    query_results.block(
        Block::default()
            .borders(Borders::TOP)
            .title("Query Results"),
    )
}

fn draw_tabs<'a>(app: &mut App) -> Tabs<'a> {
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(vec![Span::styled(*t, Style::default())]))
        .collect();

    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tabs.index)
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

fn draw_query_history<'a>(app: &mut App) -> List<'a> {
    let messages: Vec<ListItem> = app
        .editor
        .history
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Query History"),
    )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    // TODO: Figure out how to scroll logs
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default())
                .borders(Borders::ALL),
        )
}

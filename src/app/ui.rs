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
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use tui_logger::TuiLoggerWidget;

use crate::app::{App, InputMode};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(30),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = draw_help(app);
    f.render_widget(help_message, chunks[0]);

    let editor = draw_editor(app);
    f.render_widget(editor, chunks[1]);
    draw_cursor(app, f, &chunks);

    let messages: Vec<ListItem> = app
        .sql_history
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Query Output"));
    if let Some(res) = &app.query_results {
        let query = app.sql_history.last().unwrap();
        if query.starts_with("CREATE") {
            f.render_widget(messages, chunks[2]);
        } else {
            let table = pretty_format_batches(&res.batches).unwrap().to_string();
            let p = Paragraph::new(table).block(
                Block::default()
                    .borders(Borders::TOP)
                    .title("Query Results"),
            );
            f.render_widget(p, chunks[2]);
        }
    } else {
        f.render_widget(messages, chunks[2]);
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
                chunks[1].x + app.editor.get_cursor_column() + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + app.editor.get_cursor_row() + 1,
            )
        }
    };
}

// fn draw_query_results<'a>(app: &mut App) -> Paragraph<'a> {}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

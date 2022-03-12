use datafusion::prelude::{ExecutionConfig, ExecutionContext};

use crate::app::datafusion::context::QueryResults;
use crate::app::editor::Editor;
use crate::app::handlers::key_event_handler;
use crate::events::Key;

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
pub enum AppReturn {
    Continue,
    Exit,
}

/// App holds the state of the application
pub struct App {
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub sql_history: Vec<String>,
    /// Editor
    pub editor: Editor,
    /// DataFusion `ExecutionContext`
    pub context: ExecutionContext,
    /// Results from DataFusion query
    pub query_results: Option<QueryResults>,
}

impl App {
    pub fn new() -> App {
        let config = ExecutionConfig::new().with_information_schema(true);
        let ctx = ExecutionContext::with_config(config);

        App {
            input_mode: InputMode::Normal,
            sql_history: Vec::new(),
            editor: Editor::default(),
            context: ctx,
            query_results: None,
        }
    }

    pub async fn key_handler(&mut self, key: Key) -> AppReturn {
        key_event_handler(self, key).await.unwrap()
    }

    pub fn update_on_tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }
}

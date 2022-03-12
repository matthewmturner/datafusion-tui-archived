use datafusion::prelude::{ExecutionConfig, ExecutionContext};

use crate::app::datafusion::context::QueryResults;
use crate::app::editor::Editor;
use crate::app::handlers::key_event_handler;
use crate::events::Key;

pub struct Tabs {
    pub titles: Vec<&'static str>,
    pub index: usize,
}

impl Tabs {
    fn new() -> Self {
        Tabs {
            titles: vec!["SQL Editor", "Logs"],
            index: 0,
        }
    }
}

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
    /// Application tabs
    pub tabs: Tabs,
    /// Current input mode
    pub input_mode: InputMode,
    /// SQL Editor and it's state
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
            tabs: Tabs::new(),
            input_mode: InputMode::Normal,
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

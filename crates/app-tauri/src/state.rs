use poker_eval::LookupTableEvaluator;
use std::sync::Arc;

/// Shared application state managed by Tauri.
/// Holds the hand evaluator (expensive to create, shared across commands).
pub struct AppState {
    pub evaluator: Arc<LookupTableEvaluator>,
    pub data_dir: std::path::PathBuf,
}

impl AppState {
    pub fn new(data_dir: std::path::PathBuf) -> Self {
        Self {
            evaluator: Arc::new(LookupTableEvaluator::new()),
            data_dir,
        }
    }
}

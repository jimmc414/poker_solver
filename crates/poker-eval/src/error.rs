use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Evaluator errors.
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum EvalError {
    #[error("not enough cards for evaluation: got {0}, need at least 5")]
    NotEnoughCards(usize),

    #[error("too many cards: got {0}, max 7")]
    TooManyCards(usize),

    #[error("duplicate card in hand")]
    DuplicateCard,

    #[error("evaluation table not loaded")]
    TableNotLoaded,

    #[error("core error: {0}")]
    Core(#[from] poker_core::CoreError),
}

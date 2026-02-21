use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Core domain errors.
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CoreError {
    #[error("invalid rank index: {0}")]
    InvalidRank(u8),

    #[error("invalid rank character: '{0}'")]
    InvalidRankChar(char),

    #[error("invalid suit index: {0}")]
    InvalidSuit(u8),

    #[error("invalid suit character: '{0}'")]
    InvalidSuitChar(char),

    #[error("invalid card index: {0}")]
    InvalidCard(u8),

    #[error("invalid card string: '{0}'")]
    InvalidCardString(String),

    #[error("invalid board size: {0} cards (must be 0, 3, 4, or 5)")]
    InvalidBoardSize(usize),

    #[error("board is full (5 cards)")]
    BoardFull,
}

use poker_core::Card;
use poker_eval::equity_heads_up;
use serde::Serialize;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HandResult {
    pub rank: u16,
    pub category: String,
    pub description: String,
}

/// Evaluate a poker hand (5 or 7 cards).
#[tauri::command]
pub fn evaluate_hand(
    cards: Vec<Card>,
    state: State<'_, AppState>,
) -> Result<HandResult, AppError> {
    match cards.len() {
        5 => {
            let hand: [Card; 5] = cards
                .try_into()
                .map_err(|_| AppError::Eval("Expected exactly 5 cards".into()))?;
            let rank = state.evaluator.evaluate_5(&hand);
            Ok(HandResult {
                rank: rank.value(),
                category: rank.category().to_string(),
                description: rank.to_string(),
            })
        }
        7 => {
            let hand: [Card; 7] = cards
                .try_into()
                .map_err(|_| AppError::Eval("Expected exactly 7 cards".into()))?;
            let rank = state.evaluator.evaluate_7(&hand);
            Ok(HandResult {
                rank: rank.value(),
                category: rank.category().to_string(),
                description: rank.to_string(),
            })
        }
        n => Err(AppError::Eval(format!(
            "Invalid card count: {n}. Expected 5 or 7."
        ))),
    }
}

#[derive(Serialize)]
pub struct EquityResult {
    pub equity: f64,
    pub win: f64,
    pub tie: f64,
}

/// Calculate equity between two hands on a board.
#[tauri::command]
pub fn equity_calculation(
    hand: Vec<Card>,
    villain: Vec<Card>,
    board: Vec<Card>,
    state: State<'_, AppState>,
) -> Result<EquityResult, AppError> {
    if hand.len() != 2 || villain.len() != 2 {
        return Err(AppError::Eval("Each hand must have exactly 2 cards".into()));
    }
    if board.len() > 5 {
        return Err(AppError::Eval("Board cannot have more than 5 cards".into()));
    }

    let hand_arr: [Card; 2] = hand
        .try_into()
        .map_err(|_| AppError::Eval("Hand conversion failed".into()))?;
    let villain_arr: [Card; 2] = villain
        .try_into()
        .map_err(|_| AppError::Eval("Villain hand conversion failed".into()))?;

    let (eq_hero, eq_villain) =
        equity_heads_up(&state.evaluator, hand_arr, villain_arr, &board);

    Ok(EquityResult {
        equity: eq_hero,
        win: eq_hero,
        tie: 1.0 - eq_hero - eq_villain,
    })
}

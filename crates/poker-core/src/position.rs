use serde::{Deserialize, Serialize};
use std::fmt;

/// Player position at the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    // 6-max positions
    UTG,
    MP,
    CO,
    BTN,
    SB,
    BB,
    // Additional 9-max positions
    UTG1,
    UTG2,
    LJ,
}

impl Position {
    /// Standard 6-max positions in order.
    pub const SIX_MAX: [Position; 6] = [
        Position::UTG,
        Position::MP,
        Position::CO,
        Position::BTN,
        Position::SB,
        Position::BB,
    ];

    /// 9-max positions in order.
    pub const NINE_MAX: [Position; 9] = [
        Position::UTG,
        Position::UTG1,
        Position::UTG2,
        Position::LJ,
        Position::MP,
        Position::CO,
        Position::BTN,
        Position::SB,
        Position::BB,
    ];

    /// Heads-up positions.
    pub const HEADS_UP: [Position; 2] = [Position::BTN, Position::BB];

    pub fn label(self) -> &'static str {
        match self {
            Position::UTG => "UTG",
            Position::UTG1 => "UTG+1",
            Position::UTG2 => "UTG+2",
            Position::LJ => "LJ",
            Position::MP => "MP",
            Position::CO => "CO",
            Position::BTN => "BTN",
            Position::SB => "SB",
            Position::BB => "BB",
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_labels() {
        assert_eq!(Position::UTG.label(), "UTG");
        assert_eq!(Position::BB.label(), "BB");
        assert_eq!(Position::BTN.label(), "BTN");
        assert_eq!(Position::SIX_MAX.len(), 6);
        assert_eq!(Position::NINE_MAX.len(), 9);
        assert_eq!(Position::HEADS_UP.len(), 2);
    }
}

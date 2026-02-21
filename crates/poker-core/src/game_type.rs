use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of poker game/format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameType {
    /// Cash game No-Limit Hold'em.
    CashNLH,
    /// Multi-table tournament.
    MTT,
    /// Sit & Go.
    SNG,
    /// Spin & Go (hyper-turbo).
    SpinAndGo,
    /// Heads-up.
    HeadsUp,
}

impl GameType {
    pub const ALL: [GameType; 5] = [
        GameType::CashNLH,
        GameType::MTT,
        GameType::SNG,
        GameType::SpinAndGo,
        GameType::HeadsUp,
    ];

    pub fn label(self) -> &'static str {
        match self {
            GameType::CashNLH => "Cash NLH",
            GameType::MTT => "MTT",
            GameType::SNG => "SNG",
            GameType::SpinAndGo => "Spin & Go",
            GameType::HeadsUp => "Heads Up",
        }
    }

    pub fn uses_icm(self) -> bool {
        matches!(self, GameType::MTT | GameType::SNG | GameType::SpinAndGo)
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_type_enum() {
        assert_eq!(GameType::ALL.len(), 5);
        assert_eq!(GameType::CashNLH.label(), "Cash NLH");
        assert!(GameType::MTT.uses_icm());
        assert!(!GameType::CashNLH.uses_icm());
    }
}

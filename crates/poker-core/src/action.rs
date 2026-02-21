use serde::{Deserialize, Serialize};
use std::fmt;

/// A poker action.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Check,
    Call,
    /// Bet a specific amount (opening bet).
    Bet(f64),
    /// Raise to a specific amount.
    Raise(f64),
    /// All-in for remaining stack.
    AllIn(f64),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Fold => write!(f, "Fold"),
            Action::Check => write!(f, "Check"),
            Action::Call => write!(f, "Call"),
            Action::Bet(amt) => write!(f, "Bet {:.2}", amt),
            Action::Raise(amt) => write!(f, "Raise {:.2}", amt),
            Action::AllIn(amt) => write!(f, "All-In {:.2}", amt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_enum() {
        let actions = vec![
            Action::Fold,
            Action::Check,
            Action::Call,
            Action::Bet(100.0),
            Action::Raise(250.0),
            Action::AllIn(1000.0),
        ];
        assert_eq!(actions.len(), 6);
        assert_eq!(Action::Fold, Action::Fold);
        assert_ne!(Action::Fold, Action::Check);
        assert_eq!(format!("{}", Action::Bet(50.0)), "Bet 50.00");
    }
}

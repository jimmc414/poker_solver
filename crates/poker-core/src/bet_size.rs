use serde::{Deserialize, Serialize};
use std::fmt;

/// Bet sizing specification.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BetSize {
    /// Fraction of the pot (e.g., 0.5 = half pot, 1.0 = pot).
    PotFraction(f64),
    /// Absolute chip amount.
    Absolute(f64),
    /// All-in (entire remaining stack).
    AllIn,
}

impl BetSize {
    /// Calculate the actual bet amount given pot size and stack.
    pub fn resolve(self, pot: f64, stack: f64) -> f64 {
        match self {
            BetSize::PotFraction(frac) => (pot * frac).min(stack),
            BetSize::Absolute(amt) => amt.min(stack),
            BetSize::AllIn => stack,
        }
    }
}

impl fmt::Display for BetSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetSize::PotFraction(frac) => write!(f, "{:.0}%", frac * 100.0),
            BetSize::Absolute(amt) => write!(f, "{:.2}", amt),
            BetSize::AllIn => write!(f, "All-In"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bet_size_variants() {
        let half_pot = BetSize::PotFraction(0.5);
        assert_eq!(half_pot.resolve(100.0, 500.0), 50.0);

        let pot = BetSize::PotFraction(1.0);
        assert_eq!(pot.resolve(100.0, 500.0), 100.0);

        // Capped at stack
        let two_pot = BetSize::PotFraction(2.0);
        assert_eq!(two_pot.resolve(300.0, 500.0), 500.0);

        let absolute = BetSize::Absolute(75.0);
        assert_eq!(absolute.resolve(100.0, 500.0), 75.0);

        let all_in = BetSize::AllIn;
        assert_eq!(all_in.resolve(100.0, 500.0), 500.0);
    }
}

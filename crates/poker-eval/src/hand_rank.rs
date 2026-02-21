use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Numeric hand rank. Lower values are stronger hands.
/// Range: 1 (Royal Flush) to 7462 (worst high card: 7-5-4-3-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandRank(pub u16);

impl HandRank {
    /// Stronger hand has LOWER numeric rank.
    #[inline]
    pub fn value(self) -> u16 {
        self.0
    }

    /// Compare two hand ranks. Returns Ordering from perspective of self.
    /// Less means self is stronger (lower rank number = better hand).
    #[inline]
    pub fn compare(self, other: HandRank) -> Ordering {
        // Lower value = better hand, so reverse the comparison
        other.0.cmp(&self.0)
    }

    #[inline]
    pub fn category(self) -> HandCategory {
        HandCategory::from_rank(self)
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower rank value = better = Greater in ordering
        other.0.cmp(&self.0)
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.category(), self.0)
    }
}

/// Hand category (high card through straight flush).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

// Rank boundaries (inclusive). Rank 1 is the best (royal flush).
// These define the ranges for the 7,462 distinct hand classes.
const STRAIGHT_FLUSH_MAX: u16 = 10;
const FOUR_OF_A_KIND_MAX: u16 = 166;
const FULL_HOUSE_MAX: u16 = 322;
const FLUSH_MAX: u16 = 1599;
const STRAIGHT_MAX: u16 = 1609;
const THREE_OF_A_KIND_MAX: u16 = 2467;
const TWO_PAIR_MAX: u16 = 3325;
const ONE_PAIR_MAX: u16 = 6185;
// Everything above 6185 up to 7462 is high card.

impl HandCategory {
    pub fn from_rank(rank: HandRank) -> Self {
        let v = rank.0;
        if v <= STRAIGHT_FLUSH_MAX {
            HandCategory::StraightFlush
        } else if v <= FOUR_OF_A_KIND_MAX {
            HandCategory::FourOfAKind
        } else if v <= FULL_HOUSE_MAX {
            HandCategory::FullHouse
        } else if v <= FLUSH_MAX {
            HandCategory::Flush
        } else if v <= STRAIGHT_MAX {
            HandCategory::Straight
        } else if v <= THREE_OF_A_KIND_MAX {
            HandCategory::ThreeOfAKind
        } else if v <= TWO_PAIR_MAX {
            HandCategory::TwoPair
        } else if v <= ONE_PAIR_MAX {
            HandCategory::OnePair
        } else {
            HandCategory::HighCard
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            HandCategory::StraightFlush => "Straight Flush",
            HandCategory::FourOfAKind => "Four of a Kind",
            HandCategory::FullHouse => "Full House",
            HandCategory::Flush => "Flush",
            HandCategory::Straight => "Straight",
            HandCategory::ThreeOfAKind => "Three of a Kind",
            HandCategory::TwoPair => "Two Pair",
            HandCategory::OnePair => "One Pair",
            HandCategory::HighCard => "High Card",
        }
    }
}

impl fmt::Display for HandCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

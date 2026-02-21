use serde::{Deserialize, Serialize};
use std::fmt;

use crate::card::{Card, Rank};
#[cfg(test)]
use crate::card::Suit;

/// A two-card poker hand (hole cards).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hand {
    cards: [Card; 2],
}

impl Hand {
    /// Create a hand from two cards. Cards are stored high-card first.
    pub fn new(c1: Card, c2: Card) -> Self {
        if c1.rank() >= c2.rank() {
            Hand { cards: [c1, c2] }
        } else {
            Hand { cards: [c2, c1] }
        }
    }

    pub fn card1(self) -> Card {
        self.cards[0]
    }

    pub fn card2(self) -> Card {
        self.cards[1]
    }

    pub fn cards(self) -> [Card; 2] {
        self.cards
    }

    /// Whether the two cards share the same suit.
    pub fn is_suited(self) -> bool {
        self.cards[0].suit() == self.cards[1].suit()
    }

    /// Whether the two cards have the same rank (pocket pair).
    pub fn is_pair(self) -> bool {
        self.cards[0].rank() == self.cards[1].rank()
    }

    /// Short notation like "AKs", "QJo", "TT".
    pub fn notation(self) -> String {
        let r1 = self.cards[0].rank().to_char();
        let r2 = self.cards[1].rank().to_char();
        if self.is_pair() {
            format!("{}{}", r1, r2)
        } else if self.is_suited() {
            format!("{}{}s", r1, r2)
        } else {
            format!("{}{}o", r1, r2)
        }
    }

    /// Canonical hand group index (0..168).
    /// Layout: 13x13 matrix where row=high rank, col=low rank.
    /// Suited hands: matrix[hi][lo] where hi > lo (upper triangle).
    /// Offsuit hands: matrix[lo][hi] where lo < hi (lower triangle).
    /// Pairs: diagonal matrix[r][r].
    /// Index = row * 13 + col.
    pub fn canonical_index(self) -> usize {
        let hi = self.cards[0].rank().index() as usize;
        let lo = self.cards[1].rank().index() as usize;
        if self.is_pair() {
            // Diagonal: use (12 - hi) for row so Aces are at top-left
            let r = 12 - hi;
            r * 13 + r
        } else if self.is_suited() {
            // Upper triangle: row = 12 - hi, col = 12 - lo
            let row = 12 - hi;
            let col = 12 - lo;
            row * 13 + col
        } else {
            // Lower triangle: swap row/col
            let row = 12 - lo;
            let col = 12 - hi;
            row * 13 + col
        }
    }

    /// Bit mask covering both cards.
    pub fn mask(self) -> u64 {
        self.cards[0].mask() | self.cards[1].mask()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.cards[0], self.cards[1])
    }
}

/// Returns the notation label for a canonical index position in the 13x13 matrix.
/// Row and col are 0-indexed from top-left where AA is (0,0).
pub fn notation_for_matrix_cell(row: usize, col: usize) -> String {
    if row >= 13 || col >= 13 {
        return String::new();
    }
    let rank1 = Rank::ALL[12 - row];
    let rank2 = Rank::ALL[12 - col];
    let c1 = rank1.to_char();
    let c2 = rank2.to_char();
    if row == col {
        format!("{}{}", c1, c2)
    } else if row < col {
        // Upper triangle = suited
        format!("{}{}s", c1, c2)
    } else {
        // Lower triangle = offsuit
        format!("{}{}o", c2, c1)
    }
}

/// Count the number of specific combos for a hand group.
pub fn combo_count(row: usize, col: usize) -> u32 {
    if row == col {
        6 // Pairs: C(4,2) = 6
    } else if row < col {
        4 // Suited: 4 suit combos
    } else {
        12 // Offsuit: 4*3 = 12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_notation() {
        // Suited
        let aks = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
        );
        assert_eq!(aks.notation(), "AKs");
        assert!(aks.is_suited());
        assert!(!aks.is_pair());

        // Offsuit
        let qjo = Hand::new(
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Clubs),
        );
        assert_eq!(qjo.notation(), "QJo");
        assert!(!qjo.is_suited());

        // Pair
        let tt = Hand::new(
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Ten, Suit::Hearts),
        );
        assert_eq!(tt.notation(), "TT");
        assert!(tt.is_pair());

        // Low card first still works
        let hand = Hand::new(
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        );
        assert_eq!(hand.notation(), "A2s");
    }

    #[test]
    fn test_canonical_index_unique() {
        // All 169 groups should have unique indices
        let mut seen = std::collections::HashSet::new();
        for row in 0..13 {
            for col in 0..13 {
                let idx = row * 13 + col;
                assert!(seen.insert(idx), "duplicate index {}", idx);
            }
        }
        assert_eq!(seen.len(), 169);
    }

    #[test]
    fn test_canonical_index_consistency() {
        // Same hand group → same canonical index
        let aks1 = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
        );
        let akh = Hand::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
        );
        assert_eq!(aks1.canonical_index(), akh.canonical_index());

        // Different groups → different index
        let ako = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        );
        assert_ne!(aks1.canonical_index(), ako.canonical_index());
    }

    #[test]
    fn test_notation_for_matrix_cell() {
        assert_eq!(notation_for_matrix_cell(0, 0), "AA");
        assert_eq!(notation_for_matrix_cell(0, 1), "AKs");
        assert_eq!(notation_for_matrix_cell(1, 0), "AKo");
        assert_eq!(notation_for_matrix_cell(12, 12), "22");
    }

    #[test]
    fn test_combo_count() {
        assert_eq!(combo_count(0, 0), 6);   // Pair
        assert_eq!(combo_count(0, 1), 4);   // Suited
        assert_eq!(combo_count(1, 0), 12);  // Offsuit
    }

    #[test]
    fn test_hand_mask() {
        let hand = Hand::new(
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        );
        let mask = hand.mask();
        assert_ne!(mask, 0);
        assert_eq!(mask.count_ones(), 2);
    }
}

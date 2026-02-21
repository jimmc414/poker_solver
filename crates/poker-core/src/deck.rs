use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::card::{Card, Rank, Suit};

/// A standard 52-card deck with Fisher-Yates shuffle.
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
    position: usize,
}

impl Deck {
    /// Create a new ordered deck.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &rank in &Rank::ALL {
            for &suit in &Suit::ALL {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck {
            cards,
            position: 0,
        }
    }

    /// Shuffle with a random seed.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        self.position = 0;
    }

    /// Shuffle with a deterministic seed (for reproducible tests/sims).
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        self.cards.shuffle(&mut rng);
        self.position = 0;
    }

    /// Deal the next card, or None if exhausted.
    pub fn deal(&mut self) -> Option<Card> {
        if self.position < self.cards.len() {
            let card = self.cards[self.position];
            self.position += 1;
            Some(card)
        } else {
            None
        }
    }

    /// Number of remaining cards.
    pub fn remaining(&self) -> usize {
        self.cards.len() - self.position
    }

    /// Reset position without re-shuffling.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Remove specific cards from the deck (for dead card removal).
    pub fn remove_cards(&mut self, dead: &[Card]) {
        let dead_mask: u64 = dead.iter().fold(0u64, |acc, c| acc | c.mask());
        self.cards.retain(|c| c.mask() & dead_mask == 0);
        self.position = 0;
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_new() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deck_shuffle_deterministic() {
        let mut d1 = Deck::new();
        let mut d2 = Deck::new();
        d1.shuffle_with_seed(42);
        d2.shuffle_with_seed(42);

        for _ in 0..52 {
            assert_eq!(d1.deal(), d2.deal());
        }
    }

    #[test]
    fn test_deck_deal() {
        let mut deck = Deck::new();
        deck.shuffle_with_seed(123);
        let mut dealt = Vec::new();
        while let Some(card) = deck.deal() {
            dealt.push(card);
        }
        assert_eq!(dealt.len(), 52);
        // All unique
        let mut seen = std::collections::HashSet::new();
        for c in &dealt {
            assert!(seen.insert(c.as_u8()));
        }
        assert!(deck.deal().is_none());
    }

    #[test]
    fn test_deck_remove_cards() {
        let mut deck = Deck::new();
        let ah = Card::new(Rank::Ace, Suit::Hearts);
        let ks = Card::new(Rank::King, Suit::Spades);
        deck.remove_cards(&[ah, ks]);
        assert_eq!(deck.remaining(), 50);
    }
}

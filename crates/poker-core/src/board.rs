use serde::{Deserialize, Serialize};
use std::fmt;

use crate::card::Card;
use crate::error::CoreError;

/// The community board (0 to 5 cards).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    cards: Vec<Card>,
}

/// Board street/stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl Board {
    /// Empty board (preflop).
    pub fn new() -> Self {
        Board {
            cards: Vec::with_capacity(5),
        }
    }

    /// Create from a slice of cards.
    pub fn from_cards(cards: &[Card]) -> Result<Self, CoreError> {
        if cards.len() > 5 {
            return Err(CoreError::InvalidBoardSize(cards.len()));
        }
        // Check for flop: must be 0, 3, 4, or 5 cards
        if cards.len() == 1 || cards.len() == 2 {
            return Err(CoreError::InvalidBoardSize(cards.len()));
        }
        Ok(Board {
            cards: cards.to_vec(),
        })
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn street(&self) -> Street {
        match self.cards.len() {
            0 => Street::Preflop,
            3 => Street::Flop,
            4 => Street::Turn,
            5 => Street::River,
            _ => Street::Preflop, // Should not happen if constructed via from_cards
        }
    }

    /// Add a card to the board (deal next street).
    pub fn add_card(&mut self, card: Card) -> Result<(), CoreError> {
        if self.cards.len() >= 5 {
            return Err(CoreError::BoardFull);
        }
        // Can't go from 0 to 1 card (must deal flop of 3)
        self.cards.push(card);
        Ok(())
    }

    /// Deal a flop (exactly 3 cards to empty board).
    pub fn deal_flop(&mut self, c1: Card, c2: Card, c3: Card) -> Result<(), CoreError> {
        if !self.cards.is_empty() {
            return Err(CoreError::InvalidBoardSize(self.cards.len()));
        }
        self.cards.push(c1);
        self.cards.push(c2);
        self.cards.push(c3);
        Ok(())
    }

    /// Bit mask covering all board cards.
    pub fn mask(&self) -> u64 {
        self.cards.iter().fold(0u64, |acc, c| acc | c.mask())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strs: Vec<String> = self.cards.iter().map(|c| c.to_string()).collect();
        write!(f, "[{}]", strs.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_board_progression() {
        let mut board = Board::new();
        assert_eq!(board.street(), Street::Preflop);
        assert_eq!(board.len(), 0);

        board
            .deal_flop(
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Queen, Suit::Diamonds),
            )
            .expect("deal flop");
        assert_eq!(board.street(), Street::Flop);
        assert_eq!(board.len(), 3);

        board
            .add_card(Card::new(Rank::Jack, Suit::Clubs))
            .expect("deal turn");
        assert_eq!(board.street(), Street::Turn);
        assert_eq!(board.len(), 4);

        board
            .add_card(Card::new(Rank::Ten, Suit::Spades))
            .expect("deal river");
        assert_eq!(board.street(), Street::River);
        assert_eq!(board.len(), 5);

        // Can't add more
        assert!(board.add_card(Card::new(Rank::Nine, Suit::Spades)).is_err());
    }

    #[test]
    fn test_board_from_cards() {
        let cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
        ];
        let board = Board::from_cards(&cards).expect("valid");
        assert_eq!(board.street(), Street::Flop);

        // Invalid: 2 cards
        assert!(Board::from_cards(&cards[..2]).is_err());
        // Invalid: 1 card
        assert!(Board::from_cards(&cards[..1]).is_err());
    }

    #[test]
    fn test_board_mask() {
        let board = Board::from_cards(&[
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
        ])
        .expect("valid");
        let mask = board.mask();
        assert_eq!(mask.count_ones(), 3);
    }
}

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::CoreError;

/// Rank of a playing card, ordered Deuce (lowest) to Ace (highest).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Rank {
    pub const ALL: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    /// Convert from u8 (0=Two .. 12=Ace).
    pub fn from_index(index: u8) -> Result<Self, CoreError> {
        match index {
            0 => Ok(Rank::Two),
            1 => Ok(Rank::Three),
            2 => Ok(Rank::Four),
            3 => Ok(Rank::Five),
            4 => Ok(Rank::Six),
            5 => Ok(Rank::Seven),
            6 => Ok(Rank::Eight),
            7 => Ok(Rank::Nine),
            8 => Ok(Rank::Ten),
            9 => Ok(Rank::Jack),
            10 => Ok(Rank::Queen),
            11 => Ok(Rank::King),
            12 => Ok(Rank::Ace),
            _ => Err(CoreError::InvalidRank(index)),
        }
    }

    /// Parse from character: '2'-'9', 'T', 'J', 'Q', 'K', 'A'.
    pub fn from_char(c: char) -> Result<Self, CoreError> {
        match c {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' | 't' => Ok(Rank::Ten),
            'J' | 'j' => Ok(Rank::Jack),
            'Q' | 'q' => Ok(Rank::Queen),
            'K' | 'k' => Ok(Rank::King),
            'A' | 'a' => Ok(Rank::Ace),
            _ => Err(CoreError::InvalidRankChar(c)),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }

    pub fn index(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Suit of a playing card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    pub fn from_index(index: u8) -> Result<Self, CoreError> {
        match index {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(CoreError::InvalidSuit(index)),
        }
    }

    pub fn from_char(c: char) -> Result<Self, CoreError> {
        match c {
            'c' | 'C' => Ok(Suit::Clubs),
            'd' | 'D' => Ok(Suit::Diamonds),
            'h' | 'H' => Ok(Suit::Hearts),
            's' | 'S' => Ok(Suit::Spades),
            _ => Err(CoreError::InvalidSuitChar(c)),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Suit::Clubs => '\u{2663}',
            Suit::Diamonds => '\u{2666}',
            Suit::Hearts => '\u{2665}',
            Suit::Spades => '\u{2660}',
        }
    }

    pub fn index(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// A playing card packed into a single u8.
/// Encoding: `rank * 4 + suit` (0..51).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Card(u8);

impl Card {
    /// Create a card from rank and suit.
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card(rank.index() * 4 + suit.index())
    }

    /// Create from packed u8 (0..51).
    pub fn from_u8(value: u8) -> Result<Self, CoreError> {
        if value >= 52 {
            return Err(CoreError::InvalidCard(value));
        }
        Ok(Card(value))
    }

    /// Parse from 2-character string like "Ah", "Ts", "2c".
    pub fn from_str_notation(s: &str) -> Result<Self, CoreError> {
        let mut chars = s.chars();
        let rank_ch = chars.next().ok_or(CoreError::InvalidCardString(s.to_string()))?;
        let suit_ch = chars.next().ok_or(CoreError::InvalidCardString(s.to_string()))?;
        if chars.next().is_some() {
            return Err(CoreError::InvalidCardString(s.to_string()));
        }
        let rank = Rank::from_char(rank_ch)?;
        let suit = Suit::from_char(suit_ch)?;
        Ok(Card::new(rank, suit))
    }

    /// Get the packed u8 value.
    #[inline]
    pub fn as_u8(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn rank(self) -> Rank {
        // SAFETY equivalent: value always 0..51 by construction, rank index 0..12
        // No unsafe needed — from_index handles bounds
        Rank::ALL[(self.0 / 4) as usize]
    }

    #[inline]
    pub fn suit(self) -> Suit {
        Suit::ALL[(self.0 % 4) as usize]
    }

    /// Bit mask for this card (1 << card_index).
    #[inline]
    pub fn mask(self) -> u64 {
        1u64 << self.0
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card({}{})", self.rank().to_char(), self.suit().to_char())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank().to_char(), self.suit().to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_construction() {
        // All 52 cards can be constructed
        for &rank in &Rank::ALL {
            for &suit in &Suit::ALL {
                let card = Card::new(rank, suit);
                assert_eq!(card.rank(), rank);
                assert_eq!(card.suit(), suit);
                assert!(card.as_u8() < 52);
            }
        }
    }

    #[test]
    fn test_card_packed_repr() {
        // Round-trip through u8
        for i in 0..52u8 {
            let card = Card::from_u8(i).expect("valid card index");
            assert_eq!(card.as_u8(), i);
            // Re-construct from components
            let rebuilt = Card::new(card.rank(), card.suit());
            assert_eq!(rebuilt, card);
        }
        // Invalid indices
        assert!(Card::from_u8(52).is_err());
        assert!(Card::from_u8(255).is_err());
    }

    #[test]
    fn test_card_from_string() {
        let ah = Card::from_str_notation("Ah").expect("valid");
        assert_eq!(ah.rank(), Rank::Ace);
        assert_eq!(ah.suit(), Suit::Hearts);

        let tc = Card::from_str_notation("Tc").expect("valid");
        assert_eq!(tc.rank(), Rank::Ten);
        assert_eq!(tc.suit(), Suit::Clubs);

        let two_s = Card::from_str_notation("2s").expect("valid");
        assert_eq!(two_s.rank(), Rank::Two);
        assert_eq!(two_s.suit(), Suit::Spades);

        // Invalid
        assert!(Card::from_str_notation("Xx").is_err());
        assert!(Card::from_str_notation("A").is_err());
        assert!(Card::from_str_notation("Ahh").is_err());
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.to_string(), "As");
        let card = Card::new(Rank::Ten, Suit::Hearts);
        assert_eq!(card.to_string(), "Th");
    }

    #[test]
    fn test_card_mask() {
        let c0 = Card::from_u8(0).expect("valid");
        assert_eq!(c0.mask(), 1u64);
        let c51 = Card::from_u8(51).expect("valid");
        assert_eq!(c51.mask(), 1u64 << 51);
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Two < Rank::Three);
    }

    #[test]
    fn test_suit_symbols() {
        assert_eq!(Suit::Spades.symbol(), '\u{2660}');
        assert_eq!(Suit::Hearts.symbol(), '\u{2665}');
        assert_eq!(Suit::Diamonds.symbol(), '\u{2666}');
        assert_eq!(Suit::Clubs.symbol(), '\u{2663}');
    }
}

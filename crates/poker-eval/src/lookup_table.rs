use std::collections::HashMap;

use poker_core::Card;

use crate::fast_hash::FastLookup;
use crate::hand_rank::HandRank;
use crate::table_gen;

/// The 21 combinations of choosing 5 cards from 7.
const COMBOS_7_5: [[usize; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

/// Lookup-table based hand evaluator.
/// Uses a fast custom hash table for O(1) prime product lookups.
pub struct LookupTableEvaluator {
    flush_table: Vec<u16>,
    unique5_fast: FastLookup,
    // Keep HashMap for compatibility with tests
    unique5_table: HashMap<u32, u16>,
}

impl LookupTableEvaluator {
    /// Create a new evaluator by generating lookup tables.
    pub fn new() -> Self {
        let flush_table = table_gen::generate_flush_table();
        let unique5_table = table_gen::generate_unique5_table();
        let entries = table_gen::unique5_entries(&unique5_table);
        let unique5_fast = FastLookup::from_entries(&entries);

        LookupTableEvaluator {
            flush_table,
            unique5_fast,
            unique5_table,
        }
    }

    /// Evaluate a 5-card hand.
    #[inline]
    pub fn evaluate_5(&self, cards: &[Card; 5]) -> HandRank {
        table_gen::evaluate_5cards_fast(
            &self.flush_table,
            &self.unique5_fast,
            cards[0].as_u8(),
            cards[1].as_u8(),
            cards[2].as_u8(),
            cards[3].as_u8(),
            cards[4].as_u8(),
        )
    }

    /// Evaluate the best 5-card hand from 7 cards.
    #[inline]
    pub fn evaluate_7(&self, cards: &[Card; 7]) -> HandRank {
        let c: [u8; 7] = [
            cards[0].as_u8(),
            cards[1].as_u8(),
            cards[2].as_u8(),
            cards[3].as_u8(),
            cards[4].as_u8(),
            cards[5].as_u8(),
            cards[6].as_u8(),
        ];

        let mut best = 7463u16;

        for combo in &COMBOS_7_5 {
            let rank = table_gen::evaluate_5cards_fast(
                &self.flush_table,
                &self.unique5_fast,
                c[combo[0]],
                c[combo[1]],
                c[combo[2]],
                c[combo[3]],
                c[combo[4]],
            );
            let v = rank.value();
            if v < best {
                best = v;
            }
        }

        HandRank(best)
    }

    /// Evaluate any number of cards (5, 6, or 7).
    pub fn evaluate(&self, cards: &[Card]) -> HandRank {
        match cards.len() {
            5 => {
                let arr: [Card; 5] = [cards[0], cards[1], cards[2], cards[3], cards[4]];
                self.evaluate_5(&arr)
            }
            6 => {
                let mut best = HandRank(7463);
                for skip in 0..6 {
                    let mut hand = [Card::new(poker_core::Rank::Two, poker_core::Suit::Clubs); 5];
                    let mut j = 0;
                    for (i, &card) in cards.iter().enumerate() {
                        if i != skip {
                            hand[j] = card;
                            j += 1;
                        }
                    }
                    let rank = self.evaluate_5(&hand);
                    if rank.value() < best.value() {
                        best = rank;
                    }
                }
                best
            }
            7 => {
                let arr: [Card; 7] = [
                    cards[0], cards[1], cards[2], cards[3], cards[4], cards[5], cards[6],
                ];
                self.evaluate_7(&arr)
            }
            _ => HandRank(7462),
        }
    }

    /// Access the HashMap table (for tests that need exact lookup).
    pub fn unique5_map(&self) -> &HashMap<u32, u16> {
        &self.unique5_table
    }
}

impl Default for LookupTableEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::{Card, Rank, Suit};

    fn card(r: Rank, s: Suit) -> Card {
        Card::new(r, s)
    }

    #[test]
    fn test_royal_flush() {
        let eval = LookupTableEvaluator::new();
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let rank = eval.evaluate_5(&hand);
        assert_eq!(rank.value(), 1, "Royal flush should be rank 1");
    }

    #[test]
    fn test_wheel_straight_flush() {
        let eval = LookupTableEvaluator::new();
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
        ];
        let rank = eval.evaluate_5(&hand);
        assert_eq!(rank.value(), 10, "Wheel straight flush should be rank 10");
    }

    #[test]
    fn test_four_aces() {
        let eval = LookupTableEvaluator::new();
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ];
        let rank = eval.evaluate_5(&hand);
        assert_eq!(rank.value(), 11, "Four aces with king should be rank 11");
    }

    #[test]
    fn test_7card_evaluation() {
        let eval = LookupTableEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Clubs),
        ];
        let rank = eval.evaluate_7(&cards);
        assert_eq!(rank.value(), 1, "Should find royal flush in 7 cards");
    }

    #[test]
    fn test_wheel_straight_less_than_six_high() {
        let eval = LookupTableEvaluator::new();
        let wheel = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Diamonds),
            card(Rank::Four, Suit::Clubs),
            card(Rank::Five, Suit::Spades),
        ];
        let six_high = [
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Diamonds),
            card(Rank::Five, Suit::Clubs),
            card(Rank::Six, Suit::Spades),
        ];
        let wheel_rank = eval.evaluate_5(&wheel);
        let six_rank = eval.evaluate_5(&six_high);
        assert!(wheel_rank < six_rank);
    }

    #[test]
    fn test_kicker_comparison() {
        let eval = LookupTableEvaluator::new();
        let hand1 = [
            card(Rank::Queen, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Seven, Suit::Clubs),
            card(Rank::Ace, Suit::Spades),
        ];
        let hand2 = [
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        assert!(eval.evaluate_5(&hand1) > eval.evaluate_5(&hand2));
    }

    #[test]
    fn test_split_pot() {
        let eval = LookupTableEvaluator::new();
        let hand1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ];
        let hand2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Nine, Suit::Hearts),
        ];
        assert_eq!(eval.evaluate_5(&hand1), eval.evaluate_5(&hand2));
    }

    #[test]
    fn test_hand_categories() {
        use crate::hand_rank::HandCategory;
        let eval = LookupTableEvaluator::new();

        let sf = eval.evaluate_5(&[
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
        ]);
        assert_eq!(sf.category(), HandCategory::StraightFlush);

        let foak = eval.evaluate_5(&[
            card(Rank::King, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Two, Suit::Spades),
        ]);
        assert_eq!(foak.category(), HandCategory::FourOfAKind);

        let fh = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ]);
        assert_eq!(fh.category(), HandCategory::FullHouse);

        let fl = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Eight, Suit::Spades),
            card(Rank::Four, Suit::Spades),
            card(Rank::Two, Suit::Spades),
        ]);
        assert_eq!(fl.category(), HandCategory::Flush);

        let st = eval.evaluate_5(&[
            card(Rank::Ten, Suit::Spades),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Eight, Suit::Diamonds),
            card(Rank::Seven, Suit::Clubs),
            card(Rank::Six, Suit::Spades),
        ]);
        assert_eq!(st.category(), HandCategory::Straight);

        let toak = eval.evaluate_5(&[
            card(Rank::Jack, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Jack, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Two, Suit::Spades),
        ]);
        assert_eq!(toak.category(), HandCategory::ThreeOfAKind);

        let tp = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Queen, Suit::Spades),
        ]);
        assert_eq!(tp.category(), HandCategory::TwoPair);

        let op = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
        ]);
        assert_eq!(op.category(), HandCategory::OnePair);

        let hc = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ]);
        assert_eq!(hc.category(), HandCategory::HighCard);
    }

    #[test]
    fn test_transitivity() {
        let eval = LookupTableEvaluator::new();
        let royal = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ]);
        let quads = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ]);
        let full_house = eval.evaluate_5(&[
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ]);
        assert!(royal > quads);
        assert!(quads > full_house);
        assert!(royal > full_house);
    }

    #[test]
    fn test_all_7462_hand_classes() {
        let eval = LookupTableEvaluator::new();
        let mut seen_ranks = std::collections::HashSet::new();

        for a in 0..52u8 {
            for b in (a + 1)..52 {
                for c in (b + 1)..52 {
                    for d in (c + 1)..52 {
                        for e in (d + 1)..52 {
                            let cards = [
                                Card::from_u8(a).expect("valid"),
                                Card::from_u8(b).expect("valid"),
                                Card::from_u8(c).expect("valid"),
                                Card::from_u8(d).expect("valid"),
                                Card::from_u8(e).expect("valid"),
                            ];
                            let rank = eval.evaluate_5(&cards);
                            assert!(
                                rank.value() >= 1 && rank.value() <= 7462,
                                "Rank {} out of range",
                                rank.value(),
                            );
                            seen_ranks.insert(rank.value());
                        }
                    }
                }
            }
        }

        assert_eq!(seen_ranks.len(), 7462);
    }
}

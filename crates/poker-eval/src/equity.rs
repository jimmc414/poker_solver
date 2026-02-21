use poker_core::{Card, Rank, Suit};

use crate::lookup_table::LookupTableEvaluator;

/// Calculate equity of hand1 vs hand2 on a given board by enumeration.
/// Returns (equity_hand1, equity_hand2) as fractions summing to 1.0.
pub fn equity_heads_up(
    eval: &LookupTableEvaluator,
    hand1: [Card; 2],
    hand2: [Card; 2],
    board: &[Card],
) -> (f64, f64) {
    let dead_mask = hand1[0].mask()
        | hand1[1].mask()
        | hand2[0].mask()
        | hand2[1].mask()
        | board.iter().fold(0u64, |acc, c| acc | c.mask());

    let remaining_to_deal = 5 - board.len();

    // Collect available cards
    let mut available = Vec::with_capacity(52);
    for i in 0..52u8 {
        if let Ok(c) = Card::from_u8(i) {
            if c.mask() & dead_mask == 0 {
                available.push(c);
            }
        }
    }

    let mut wins1 = 0u64;
    let mut wins2 = 0u64;
    let mut ties = 0u64;

    match remaining_to_deal {
        0 => {
            // River - just evaluate
            let mut cards1 = [Card::new(Rank::Two, Suit::Clubs); 7];
            let mut cards2 = [Card::new(Rank::Two, Suit::Clubs); 7];
            cards1[0] = hand1[0];
            cards1[1] = hand1[1];
            cards2[0] = hand2[0];
            cards2[1] = hand2[1];
            for (i, &bc) in board.iter().enumerate() {
                cards1[2 + i] = bc;
                cards2[2 + i] = bc;
            }
            let rank1 = eval.evaluate_7(&cards1);
            let rank2 = eval.evaluate_7(&cards2);
            match rank1.compare(rank2) {
                std::cmp::Ordering::Greater => wins1 = 1,
                std::cmp::Ordering::Less => wins2 = 1,
                std::cmp::Ordering::Equal => ties = 1,
            }
        }
        1 => {
            // Turn dealt, enumerate river
            for &river in &available {
                let mut c1 = [Card::new(Rank::Two, Suit::Clubs); 7];
                let mut c2 = [Card::new(Rank::Two, Suit::Clubs); 7];
                c1[0] = hand1[0];
                c1[1] = hand1[1];
                c2[0] = hand2[0];
                c2[1] = hand2[1];
                for (i, &bc) in board.iter().enumerate() {
                    c1[2 + i] = bc;
                    c2[2 + i] = bc;
                }
                c1[6] = river;
                c2[6] = river;
                let r1 = eval.evaluate_7(&c1);
                let r2 = eval.evaluate_7(&c2);
                match r1.compare(r2) {
                    std::cmp::Ordering::Greater => wins1 += 1,
                    std::cmp::Ordering::Less => wins2 += 1,
                    std::cmp::Ordering::Equal => ties += 1,
                }
            }
        }
        2 => {
            // Flop dealt, enumerate turn + river
            let n = available.len();
            for i in 0..n {
                for j in (i + 1)..n {
                    let turn = available[i];
                    let river = available[j];
                    let mut c1 = [Card::new(Rank::Two, Suit::Clubs); 7];
                    let mut c2 = [Card::new(Rank::Two, Suit::Clubs); 7];
                    c1[0] = hand1[0];
                    c1[1] = hand1[1];
                    c2[0] = hand2[0];
                    c2[1] = hand2[1];
                    for (k, &bc) in board.iter().enumerate() {
                        c1[2 + k] = bc;
                        c2[2 + k] = bc;
                    }
                    c1[5] = turn;
                    c1[6] = river;
                    c2[5] = turn;
                    c2[6] = river;
                    let r1 = eval.evaluate_7(&c1);
                    let r2 = eval.evaluate_7(&c2);
                    match r1.compare(r2) {
                        std::cmp::Ordering::Greater => wins1 += 1,
                        std::cmp::Ordering::Less => wins2 += 1,
                        std::cmp::Ordering::Equal => ties += 1,
                    }
                }
            }
        }
        5 => {
            // Preflop - enumerate all 5-card boards (C(48,5) is huge, use sampling or
            // enumerate C(48,5) which is ~1.7M -- feasible but slow)
            // For preflop, we'll enumerate all possible 5-card boards
            let n = available.len();
            for a in 0..n {
                for b in (a + 1)..n {
                    for c in (b + 1)..n {
                        for d in (c + 1)..n {
                            for e in (d + 1)..n {
                                let mut c1 = [Card::new(Rank::Two, Suit::Clubs); 7];
                                let mut c2 = [Card::new(Rank::Two, Suit::Clubs); 7];
                                c1[0] = hand1[0];
                                c1[1] = hand1[1];
                                c1[2] = available[a];
                                c1[3] = available[b];
                                c1[4] = available[c];
                                c1[5] = available[d];
                                c1[6] = available[e];
                                c2[0] = hand2[0];
                                c2[1] = hand2[1];
                                c2[2] = available[a];
                                c2[3] = available[b];
                                c2[4] = available[c];
                                c2[5] = available[d];
                                c2[6] = available[e];
                                let r1 = eval.evaluate_7(&c1);
                                let r2 = eval.evaluate_7(&c2);
                                match r1.compare(r2) {
                                    std::cmp::Ordering::Greater => wins1 += 1,
                                    std::cmp::Ordering::Less => wins2 += 1,
                                    std::cmp::Ordering::Equal => ties += 1,
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            // 3 or 4 remaining cards - enumerate similarly
            // For now, handle the turn (3 remaining = flop missing turn+river... not standard)
            // Standard: board 0 = preflop (5 remaining), board 3 = flop (2 remaining),
            // board 4 = turn (1 remaining), board 5 = river (0 remaining)
        }
    }

    let total = (wins1 + wins2 + ties) as f64;
    if total == 0.0 {
        return (0.5, 0.5);
    }
    let eq1 = (wins1 as f64 + ties as f64 / 2.0) / total;
    let eq2 = (wins2 as f64 + ties as f64 / 2.0) / total;
    (eq1, eq2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::{Rank, Suit};

    fn card(r: Rank, s: Suit) -> Card {
        Card::new(r, s)
    }

    #[test]
    fn test_equity_aa_vs_kk_river() {
        // On a dry board, AA should beat KK
        let eval = LookupTableEvaluator::new();
        let aa = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        ];
        let kk = [
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
        ];
        let board = [
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Nine, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
        ];
        let (eq1, eq2) = equity_heads_up(&eval, aa, kk, &board);
        assert!(
            (eq1 - 1.0).abs() < 0.001,
            "AA should have ~100% equity vs KK on this board, got {eq1}"
        );
        assert!(eq2 < 0.001);
    }

    #[test]
    fn test_equity_known_matchup_turn() {
        let eval = LookupTableEvaluator::new();
        // AKs vs QQ on a flop that doesn't help either
        let aks = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        ];
        let qq = [
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let board = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Nine, Suit::Clubs),
            card(Rank::Four, Suit::Spades),
        ];
        let (eq1, _eq2) = equity_heads_up(&eval, aks, qq, &board);
        // AKs has roughly 15-25% equity vs QQ on a blank board with turn dealt
        assert!(
            eq1 > 0.05 && eq1 < 0.50,
            "AKs equity vs QQ on blank board should be reasonable, got {eq1}"
        );
    }

    #[test]
    fn test_equity_flopped_set() {
        let eval = LookupTableEvaluator::new();
        // Set of aces vs overpair on river
        let aa = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        ];
        let kk = [
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
        ];
        let board = [
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Five, Suit::Hearts),
            card(Rank::Eight, Suit::Clubs),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let (eq1, _) = equity_heads_up(&eval, aa, kk, &board);
        assert!(
            (eq1 - 1.0).abs() < 0.001,
            "Set of aces should beat pair of kings on this river"
        );
    }
}

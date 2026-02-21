use poker_core::Card;
use serde::{Deserialize, Serialize};

/// Information about how a player's cards block opponent ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerInfo {
    /// How many opponent combos are removed by this hand's cards.
    pub combos_removed: u32,
    /// Whether this hand blocks the nut flush draw.
    pub blocks_nut_flush: bool,
    /// Whether this hand blocks a set (holds a card matching a board card rank).
    pub blocks_set: bool,
    /// Whether this hand blocks top pair.
    pub blocks_top_pair: bool,
    /// Suit that is blocked for flush (if any).
    pub blocked_flush_suit: Option<u8>,
}

/// Analyze blocker effects of a hand on a given board.
pub fn analyze_blockers(hand: &[Card; 2], board: &[Card]) -> BlockerInfo {
    let mut info = BlockerInfo {
        combos_removed: 0,
        blocks_nut_flush: false,
        blocks_set: false,
        blocks_top_pair: false,
        blocked_flush_suit: None,
    };

    // Count suits on board
    let mut board_suit_counts = [0u8; 4];
    for c in board {
        board_suit_counts[c.suit() as usize] += 1;
    }

    // Find if there's a potential flush suit (2+ on board)
    let flush_suit = board_suit_counts
        .iter()
        .enumerate()
        .find(|(_, &count)| count >= 2)
        .map(|(suit, _)| suit as u8);

    // Check if hand blocks nut flush (holds ace of flush suit)
    if let Some(fs) = flush_suit {
        for c in hand {
            if c.suit() as u8 == fs && c.rank() == poker_core::Rank::Ace {
                info.blocks_nut_flush = true;
                info.blocked_flush_suit = Some(fs);
            }
        }
    }

    // Check if hand blocks a set (holds a card with same rank as board card)
    let board_ranks: Vec<poker_core::Rank> = board.iter().map(|c| c.rank()).collect();
    for c in hand {
        if board_ranks.contains(&c.rank()) {
            info.blocks_set = true;
        }
    }

    // Check if hand blocks top pair
    if let Some(&top_rank) = board_ranks.iter().max() {
        for c in hand {
            if c.rank() == top_rank {
                info.blocks_top_pair = true;
            }
        }
    }

    // Calculate combos removed: each card in hand removes combos from opponent's range
    // Each card removes: 3 pairs + board_overlaps for pairs, reduces combo availability
    // Simplified: each hand card removes at least 2-3 combos per remaining rank
    let hand_mask = hand[0].mask() | hand[1].mask();
    let board_mask = board.iter().fold(0u64, |acc, c| acc | c.mask());
    let dead = hand_mask | board_mask;

    // Count how many of the 1326 combos are blocked
    let mut removed = 0u32;
    for i in 0..52u8 {
        if (1u64 << i) & dead != 0 {
            continue;
        }
        for j in (i + 1)..52 {
            if (1u64 << j) & dead != 0 {
                continue;
            }
            // This combo (i,j) is available. Check if either card matches hand
            // If removing hand cards means this combo couldn't exist if the hand didn't exist... no.
            // Actually, we count combos that include at least one of the hand's RANKS
            let ri = i / 4;
            let rj = j / 4;
            let hr0 = hand[0].rank() as u8;
            let hr1 = hand[1].rank() as u8;
            if ri == hr0 || ri == hr1 || rj == hr0 || rj == hr1 {
                removed += 1;
            }
        }
    }
    // The number above counts how many of opponent's potential combos share a rank with our hand.
    // This is a simplification; real blocker analysis is per-range.
    info.combos_removed = removed;

    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::{Card, Rank, Suit};

    fn card(r: Rank, s: Suit) -> Card {
        Card::new(r, s)
    }

    #[test]
    fn test_blocker_nut_flush() {
        // As on a board with 2 spades should block nut flush
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ];
        let board = [
            card(Rank::Two, Suit::Spades),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
        ];
        let info = analyze_blockers(&hand, &board);
        assert!(info.blocks_nut_flush, "As should block nut flush on 2-spade board");
    }

    #[test]
    fn test_blocker_no_nut_flush() {
        let hand = [
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        ];
        let board = [
            card(Rank::Two, Suit::Spades),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
        ];
        let info = analyze_blockers(&hand, &board);
        assert!(
            !info.blocks_nut_flush,
            "Ks should not block nut flush (need As)"
        );
    }

    #[test]
    fn test_blocker_blocks_set() {
        let hand = [
            card(Rank::Jack, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        ];
        let board = [
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Two, Suit::Diamonds),
        ];
        let info = analyze_blockers(&hand, &board);
        assert!(info.blocks_set, "Holding a jack should block set of jacks");
    }

    #[test]
    fn test_blocker_blocks_top_pair() {
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ];
        let board = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Two, Suit::Diamonds),
        ];
        let info = analyze_blockers(&hand, &board);
        assert!(info.blocks_top_pair, "Holding an ace should block top pair");
        assert!(info.blocks_set, "Holding an ace should also block set");
    }

    #[test]
    fn test_blocker_combo_reduction() {
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        ];
        let board = [
            card(Rank::King, Suit::Diamonds),
            card(Rank::Seven, Suit::Clubs),
            card(Rank::Two, Suit::Spades),
        ];
        let info = analyze_blockers(&hand, &board);
        // With AA, we block many combos that include an ace
        assert!(
            info.combos_removed > 0,
            "AA should block opponent combos containing aces"
        );
    }
}

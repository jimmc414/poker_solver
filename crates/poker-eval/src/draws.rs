use poker_core::Card;
use serde::{Deserialize, Serialize};

/// Types of draws detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrawType {
    FlushDraw,
    BackdoorFlushDraw,
    OpenEndedStraightDraw,
    Gutshot,
    ComboDraw,
    BackdoorStraightDraw,
}

/// Detect draws for a player's hand on a given board.
pub fn detect_draws(hand: &[Card; 2], board: &[Card]) -> Vec<DrawType> {
    let mut draws = Vec::new();

    let all_cards: Vec<Card> = hand.iter().chain(board.iter()).copied().collect();

    // Count suits
    let mut suit_counts = [0u8; 4];
    for c in &all_cards {
        suit_counts[c.suit() as usize] += 1;
    }

    // Count suits on board only
    let mut board_suit_counts = [0u8; 4];
    for c in board {
        board_suit_counts[c.suit() as usize] += 1;
    }

    // Count suits in hand only
    let mut hand_suit_counts = [0u8; 4];
    for c in hand {
        hand_suit_counts[c.suit() as usize] += 1;
    }

    // Flush draw: 4 cards of same suit (need 1 more), at least one from hand
    let has_flush_draw = (0..4).any(|s| suit_counts[s] == 4 && hand_suit_counts[s] >= 1);

    // Backdoor flush draw: 3 cards of same suit (need 2 more), at least one from hand, only on flop
    let has_backdoor_flush =
        board.len() == 3 && (0..4).any(|s| suit_counts[s] == 3 && hand_suit_counts[s] >= 1);

    // Straight draw detection: build rank bitmask
    let mut rank_bits = 0u16;
    for c in &all_cards {
        rank_bits |= 1 << c.rank() as u16;
    }

    // Check for OESD and gutshot
    let has_oesd = detect_oesd(rank_bits);
    let has_gutshot = detect_gutshot(rank_bits);

    // Detect made straight for exclusion (if already have straight, not a draw)
    let has_straight = detect_made_straight(rank_bits);

    if has_flush_draw && (has_oesd || has_gutshot) && !has_straight {
        draws.push(DrawType::ComboDraw);
    } else {
        if has_flush_draw {
            draws.push(DrawType::FlushDraw);
        }
        if has_oesd && !has_straight {
            draws.push(DrawType::OpenEndedStraightDraw);
        } else if has_gutshot && !has_straight {
            draws.push(DrawType::Gutshot);
        }
    }

    if has_backdoor_flush && !has_flush_draw {
        draws.push(DrawType::BackdoorFlushDraw);
    }

    draws
}

/// Check for open-ended straight draw (4 consecutive ranks, not already a straight).
fn detect_oesd(rank_bits: u16) -> bool {
    // Check all 4-consecutive-rank windows that can extend to 5 in both directions
    // OESD: 4 in a row where both ends can complete
    // Must have exactly 4 consecutive (not 5 which is a made straight)

    // First check if we have a made straight (5 consecutive)
    if detect_made_straight(rank_bits) {
        return false;
    }

    // Check 4-card consecutive sequences
    // Pattern: xxxx where both ends can complete (not at the edges of A-2 or A-K)
    for bottom in 1..10 {
        // bottom can be 1 (Three) to 9 (Jack)
        let mask = 0xF << bottom; // 4 consecutive bits
        if rank_bits & mask == mask {
            return true;
        }
    }
    // Special: A-2-3-4 (bottom = 0, but top can extend to 5)
    let low_mask = 0b1000000001111u16; // A,2,3,4 with bits 0,1,2,3 and 12(Ace)
    if (rank_bits & 0xF) == 0xF {
        // 2-3-4-5 already handled above
        return false;
    }
    // Check A-2-3-4
    let a_low = (rank_bits & 0b111) == 0b111 && (rank_bits & (1 << 12)) != 0;
    if a_low {
        // A-2-3-4: can complete with 5 (one end only = gutshot from this perspective)
        // Actually this is a gutshot, not OESD
        return false;
    }
    let _ = low_mask; // suppress warning

    false
}

/// Check for gutshot straight draw.
fn detect_gutshot(rank_bits: u16) -> bool {
    if detect_made_straight(rank_bits) {
        return false;
    }
    if detect_oesd(rank_bits) {
        return false; // OESD takes priority
    }

    // Check 5-card windows where exactly 4 of 5 ranks are present
    for bottom in 0..9 {
        let window = 0x1Fu16 << bottom; // 5 consecutive bits
        let overlap = rank_bits & window;
        if overlap.count_ones() == 4 {
            return true;
        }
    }
    // Ace-low window: A-2-3-4-5
    let ace_low = (1u16 << 12) | 0xF; // A,2,3,4,5
    let overlap = rank_bits & ace_low;
    if overlap.count_ones() == 4 {
        return true;
    }

    false
}

/// Check if rank_bits contains a made straight (5 consecutive ranks).
fn detect_made_straight(rank_bits: u16) -> bool {
    for bottom in 0..9 {
        let mask = 0x1Fu16 << bottom;
        if rank_bits & mask == mask {
            return true;
        }
    }
    // Ace-low: A-2-3-4-5
    let ace_low = (1u16 << 12) | 0xF;
    rank_bits & ace_low == ace_low
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::{Rank, Suit};

    fn card(r: Rank, s: Suit) -> Card {
        Card::new(r, s)
    }

    #[test]
    fn test_flush_draw_detection() {
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        ];
        let board = [
            card(Rank::Two, Suit::Spades),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
        ];
        let draws = detect_draws(&hand, &board);
        assert!(
            draws.contains(&DrawType::FlushDraw),
            "Should detect flush draw with 4 spades"
        );
    }

    #[test]
    fn test_oesd_detection() {
        // Hand: 8h 9h, Board: Ts 7c 2d = 7-8-9-T straight draw
        let hand = [
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Nine, Suit::Hearts),
        ];
        let board = [
            card(Rank::Ten, Suit::Spades),
            card(Rank::Seven, Suit::Clubs),
            card(Rank::Two, Suit::Diamonds),
        ];
        let draws = detect_draws(&hand, &board);
        assert!(
            draws.contains(&DrawType::OpenEndedStraightDraw),
            "Should detect OESD: 7-8-9-T, draws: {:?}",
            draws
        );
    }

    #[test]
    fn test_gutshot_detection() {
        // Hand: Ah Jh, Board: Ks Tc 2d = A-K-?-J-T needs Q for straight
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
        ];
        let board = [
            card(Rank::King, Suit::Spades),
            card(Rank::Ten, Suit::Clubs),
            card(Rank::Two, Suit::Diamonds),
        ];
        let draws = detect_draws(&hand, &board);
        assert!(
            draws.contains(&DrawType::Gutshot),
            "Should detect gutshot: A-K-?-J-T, draws: {:?}",
            draws
        );
    }

    #[test]
    fn test_combo_draw() {
        // Flush draw + straight draw
        let hand = [
            card(Rank::Eight, Suit::Spades),
            card(Rank::Nine, Suit::Spades),
        ];
        let board = [
            card(Rank::Ten, Suit::Spades),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Two, Suit::Diamonds),
        ];
        let draws = detect_draws(&hand, &board);
        assert!(
            draws.contains(&DrawType::ComboDraw),
            "Should detect combo draw (flush + straight): {:?}",
            draws
        );
    }

    #[test]
    fn test_no_draw_on_made_hand() {
        // Already have a straight
        let hand = [
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Nine, Suit::Clubs),
        ];
        let board = [
            card(Rank::Ten, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
            card(Rank::Seven, Suit::Hearts),
        ];
        let draws = detect_draws(&hand, &board);
        assert!(
            !draws.contains(&DrawType::OpenEndedStraightDraw),
            "Should not detect OESD when straight is made: {:?}",
            draws
        );
        assert!(
            !draws.contains(&DrawType::Gutshot),
            "Should not detect gutshot when straight is made: {:?}",
            draws
        );
    }
}

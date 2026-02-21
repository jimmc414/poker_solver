use poker_core::{Card, Suit};

/// A canonical board representation after suit isomorphism reduction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalBoard {
    pub cards: Vec<Card>,
    /// The suit permutation applied: permutation[original_suit] = canonical_suit
    pub suit_mapping: [u8; 4],
}

/// Canonicalize a flop by applying suit isomorphism.
/// Maps 22,100 possible flops to 1,755 canonical forms.
///
/// Algorithm: Assign suits in order of first appearance.
/// The first suit seen becomes suit 0, second becomes suit 1, etc.
pub fn canonicalize_flop(cards: &[Card]) -> CanonicalBoard {
    canonicalize_board(cards)
}

/// Canonicalize a board of any length using suit isomorphism.
pub fn canonicalize_board(cards: &[Card]) -> CanonicalBoard {
    // Sort cards by rank (descending), then by suit for determinism
    let mut sorted: Vec<Card> = cards.to_vec();
    sorted.sort_by(|a, b| {
        b.rank()
            .cmp(&a.rank())
            .then_with(|| a.suit().cmp(&b.suit()))
    });

    // Find the canonical suit mapping by trying all 24 permutations
    // and picking the lexicographically smallest result
    let perms = all_suit_permutations();
    let mut best_cards: Option<Vec<Card>> = None;
    let mut best_mapping = [0u8; 4];

    for perm in &perms {
        let mapped: Vec<Card> = sorted
            .iter()
            .map(|c| {
                let new_suit = Suit::from_index(perm[c.suit() as usize])
                    .unwrap_or(Suit::Clubs);
                Card::new(c.rank(), new_suit)
            })
            .collect();

        // Sort the mapped cards for consistent comparison
        let mut sorted_mapped = mapped;
        sorted_mapped.sort_by(|a, b| {
            b.rank()
                .cmp(&a.rank())
                .then_with(|| a.suit().cmp(&b.suit()))
        });

        let is_better = match &best_cards {
            None => true,
            Some(best) => card_vec_less(&sorted_mapped, best),
        };

        if is_better {
            best_cards = Some(sorted_mapped);
            best_mapping = *perm;
        }
    }

    CanonicalBoard {
        cards: best_cards.unwrap_or_default(),
        suit_mapping: best_mapping,
    }
}

/// Compare two card vectors lexicographically.
fn card_vec_less(a: &[Card], b: &[Card]) -> bool {
    for (ca, cb) in a.iter().zip(b.iter()) {
        if ca.as_u8() != cb.as_u8() {
            return ca.as_u8() < cb.as_u8();
        }
    }
    false
}

/// Generate all 24 permutations of 4 suits.
fn all_suit_permutations() -> Vec<[u8; 4]> {
    let mut perms = Vec::with_capacity(24);
    let suits = [0u8, 1, 2, 3];
    for &a in &suits {
        for &b in &suits {
            if b == a {
                continue;
            }
            for &c in &suits {
                if c == a || c == b {
                    continue;
                }
                for &d in &suits {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    perms.push([a, b, c, d]);
                }
            }
        }
    }
    perms
}

/// Count the number of canonical flops (should be 1,755).
pub fn count_canonical_flops() -> usize {
    let mut canonical_set = std::collections::HashSet::new();

    for a in 0..52u8 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                let cards = [
                    Card::from_u8(a).expect("valid card"),
                    Card::from_u8(b).expect("valid card"),
                    Card::from_u8(c).expect("valid card"),
                ];
                let canonical = canonicalize_flop(&cards);
                let key: Vec<u8> = canonical.cards.iter().map(|c| c.as_u8()).collect();
                canonical_set.insert(key);
            }
        }
    }

    canonical_set.len()
}

/// Remap a player's hole cards using the same suit mapping applied to the board.
pub fn remap_hand(hand: &[Card; 2], suit_mapping: &[u8; 4]) -> [Card; 2] {
    let remap = |c: Card| {
        let new_suit = Suit::from_index(suit_mapping[c.suit() as usize])
            .unwrap_or(Suit::Clubs);
        Card::new(c.rank(), new_suit)
    };
    [remap(hand[0]), remap(hand[1])]
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::{Card, Rank, Suit};

    fn card(r: Rank, s: Suit) -> Card {
        Card::new(r, s)
    }

    #[test]
    fn test_suit_isomorphism_flop_count() {
        let count = count_canonical_flops();
        assert_eq!(
            count, 1755,
            "There should be exactly 1,755 canonical flops, got {}",
            count
        );
    }

    #[test]
    fn test_suit_isomorphism_idempotent() {
        let flop = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let canon1 = canonicalize_flop(&flop);
        let canon2 = canonicalize_flop(&canon1.cards);
        assert_eq!(
            canon1.cards, canon2.cards,
            "Canonicalization should be idempotent"
        );
    }

    #[test]
    fn test_suit_isomorphism_equivalent() {
        // These two flops differ only in suits, should map to same canonical form
        let flop1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let flop2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
        ];
        let canon1 = canonicalize_flop(&flop1);
        let canon2 = canonicalize_flop(&flop2);
        assert_eq!(
            canon1.cards, canon2.cards,
            "Suit-swapped flops should have same canonical form"
        );
    }

    #[test]
    fn test_monotone_flop() {
        // All same suit flops should be equivalent regardless of which suit
        let flop_spades = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
        ];
        let flop_hearts = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
        ];
        let canon1 = canonicalize_flop(&flop_spades);
        let canon2 = canonicalize_flop(&flop_hearts);
        assert_eq!(canon1.cards, canon2.cards);
    }

    #[test]
    fn test_remap_hand() {
        let mapping = [1, 0, 2, 3]; // Swap clubs<->diamonds
        let hand = [
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Diamonds),
        ];
        let remapped = remap_hand(&hand, &mapping);
        assert_eq!(remapped[0].suit(), Suit::Diamonds);
        assert_eq!(remapped[1].suit(), Suit::Clubs);
    }
}

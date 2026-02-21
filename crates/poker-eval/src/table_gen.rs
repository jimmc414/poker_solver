use std::collections::HashMap;

use crate::hand_rank::HandRank;

/// Prime numbers mapped to each rank (2=Two .. 41=Ace).
pub const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Generate the flush lookup table.
/// Index: bit pattern of 5 ranks (13 bits, exactly 5 set).
/// Value: HandRank for the flush hand with those ranks.
pub fn generate_flush_table() -> Vec<u16> {
    let mut table = vec![0u16; 8192]; // 2^13

    // Straight flush patterns
    let straights: [u16; 10] = [
        0b1_1111_0000_0000, // A-K-Q-J-T (royal)
        0b0_1111_1000_0000, // K-Q-J-T-9
        0b0_0111_1100_0000, // Q-J-T-9-8
        0b0_0011_1110_0000, // J-T-9-8-7
        0b0_0001_1111_0000, // T-9-8-7-6
        0b0_0000_1111_1000, // 9-8-7-6-5
        0b0_0000_0111_1100, // 8-7-6-5-4
        0b0_0000_0011_1110, // 7-6-5-4-3
        0b0_0000_0001_1111, // 6-5-4-3-2
        0b1_0000_0000_1111, // 5-4-3-2-A (wheel)
    ];

    for (i, &bits) in straights.iter().enumerate() {
        table[bits as usize] = (i + 1) as u16;
    }

    // Collect non-straight flush hands and sort by strength
    let mut flush_hands: Vec<u16> = Vec::new();
    for a in 0..13u16 {
        for b in (a + 1)..13 {
            for c in (b + 1)..13 {
                for d in (c + 1)..13 {
                    for e in (d + 1)..13 {
                        let bits = (1 << a) | (1 << b) | (1 << c) | (1 << d) | (1 << e);
                        if table[bits as usize] == 0 {
                            flush_hands.push(bits);
                        }
                    }
                }
            }
        }
    }

    // Sort descending by bit pattern value (higher bits = higher ranks = better)
    flush_hands.sort_unstable_by(|a, b| b.cmp(a));

    // Flush (non-straight) ranks: 323..1599 (1277 hands)
    for (i, &bits) in flush_hands.iter().enumerate() {
        table[bits as usize] = 323 + i as u16;
    }

    table
}

/// Generate lookup HashMap for non-flush 5-card hands using prime products.
pub fn generate_unique5_table() -> HashMap<u32, u16> {
    let mut table = HashMap::new();

    // Straights (non-flush): ranks 1600..1609
    let straight_combos: [(u16, u16); 10] = [
        (0b1_1111_0000_0000, 1600), // A-high
        (0b0_1111_1000_0000, 1601),
        (0b0_0111_1100_0000, 1602),
        (0b0_0011_1110_0000, 1603),
        (0b0_0001_1111_0000, 1604),
        (0b0_0000_1111_1000, 1605),
        (0b0_0000_0111_1100, 1606),
        (0b0_0000_0011_1110, 1607),
        (0b0_0000_0001_1111, 1608),
        (0b1_0000_0000_1111, 1609), // Wheel
    ];

    let mut straight_prods = std::collections::HashSet::new();
    for &(bits, rank) in &straight_combos {
        let mut ranks_in = Vec::new();
        for i in 0..13u8 {
            if bits & (1 << i) != 0 {
                ranks_in.push(i);
            }
        }
        let prod: u32 = ranks_in.iter().map(|&r| PRIMES[r as usize]).product();
        table.insert(prod, rank);
        straight_prods.insert(prod);
    }

    // Four of a kind: ranks 11..166 (156 hands = 13 quads * 12 kickers)
    let mut rank_val = 11u16;
    for quad_rank in (0..13u8).rev() {
        for kicker in (0..13u8).rev() {
            if kicker == quad_rank {
                continue;
            }
            let prod = PRIMES[quad_rank as usize].pow(4) * PRIMES[kicker as usize];
            table.insert(prod, rank_val);
            rank_val += 1;
        }
    }

    // Full house: ranks 167..322 (156 hands = 13 trips * 12 pairs)
    rank_val = 167;
    for trips in (0..13u8).rev() {
        for pair in (0..13u8).rev() {
            if pair == trips {
                continue;
            }
            let prod = PRIMES[trips as usize].pow(3) * PRIMES[pair as usize].pow(2);
            table.insert(prod, rank_val);
            rank_val += 1;
        }
    }

    // Three of a kind: ranks 1610..2467 (858 hands = 13*C(12,2))
    rank_val = 1610;
    for trips in (0..13u8).rev() {
        for k1 in (0..13u8).rev() {
            if k1 == trips {
                continue;
            }
            for k2 in (0..k1).rev() {
                if k2 == trips {
                    continue;
                }
                let prod = PRIMES[trips as usize].pow(3)
                    * PRIMES[k1 as usize]
                    * PRIMES[k2 as usize];
                table.insert(prod, rank_val);
                rank_val += 1;
            }
        }
    }

    // Two pair: ranks 2468..3325 (858 hands = C(13,2)*11)
    rank_val = 2468;
    for hi_pair in (0..13u8).rev() {
        for lo_pair in (0..hi_pair).rev() {
            for kicker in (0..13u8).rev() {
                if kicker == hi_pair || kicker == lo_pair {
                    continue;
                }
                let prod = PRIMES[hi_pair as usize].pow(2)
                    * PRIMES[lo_pair as usize].pow(2)
                    * PRIMES[kicker as usize];
                table.insert(prod, rank_val);
                rank_val += 1;
            }
        }
    }

    // One pair: ranks 3326..6185 (2860 hands = 13*C(12,3))
    rank_val = 3326;
    for pair in (0..13u8).rev() {
        for k1 in (0..13u8).rev() {
            if k1 == pair {
                continue;
            }
            for k2 in (0..k1).rev() {
                if k2 == pair {
                    continue;
                }
                for k3 in (0..k2).rev() {
                    if k3 == pair {
                        continue;
                    }
                    let prod = PRIMES[pair as usize].pow(2)
                        * PRIMES[k1 as usize]
                        * PRIMES[k2 as usize]
                        * PRIMES[k3 as usize];
                    table.insert(prod, rank_val);
                    rank_val += 1;
                }
            }
        }
    }

    // High card (non-flush, non-straight): ranks 6186..7462 (1277 hands)
    rank_val = 6186;
    for a in (0..13u8).rev() {
        for b in (0..a).rev() {
            for c in (0..b).rev() {
                for d in (0..c).rev() {
                    for e in (0..d).rev() {
                        let prod = PRIMES[a as usize]
                            * PRIMES[b as usize]
                            * PRIMES[c as usize]
                            * PRIMES[d as usize]
                            * PRIMES[e as usize];
                        if straight_prods.contains(&prod) {
                            continue;
                        }
                        table.insert(prod, rank_val);
                        rank_val += 1;
                    }
                }
            }
        }
    }

    table
}

/// Evaluate a 5-card hand using precomputed tables (HashMap version for tests).
#[inline]
pub fn evaluate_5cards(
    flush_table: &[u16],
    unique5_table: &HashMap<u32, u16>,
    c0: u8,
    c1: u8,
    c2: u8,
    c3: u8,
    c4: u8,
) -> HandRank {
    let r0 = (c0 / 4) as usize;
    let r1 = (c1 / 4) as usize;
    let r2 = (c2 / 4) as usize;
    let r3 = (c3 / 4) as usize;
    let r4 = (c4 / 4) as usize;

    let s0 = c0 % 4;
    let s1 = c1 % 4;
    let s2 = c2 % 4;
    let s3 = c3 % 4;
    let s4 = c4 % 4;

    if s0 == s1 && s1 == s2 && s2 == s3 && s3 == s4 {
        let bits = (1u16 << r0) | (1u16 << r1) | (1u16 << r2) | (1u16 << r3) | (1u16 << r4);
        return HandRank(flush_table[bits as usize]);
    }

    let prod = PRIMES[r0] * PRIMES[r1] * PRIMES[r2] * PRIMES[r3] * PRIMES[r4];
    HandRank(*unique5_table.get(&prod).unwrap_or(&7462))
}

/// Fast 5-card evaluation using the FastLookup hash table.
#[inline(always)]
pub fn evaluate_5cards_fast(
    flush_table: &[u16],
    unique5_fast: &crate::fast_hash::FastLookup,
    c0: u8,
    c1: u8,
    c2: u8,
    c3: u8,
    c4: u8,
) -> HandRank {
    let r0 = (c0 >> 2) as usize;
    let r1 = (c1 >> 2) as usize;
    let r2 = (c2 >> 2) as usize;
    let r3 = (c3 >> 2) as usize;
    let r4 = (c4 >> 2) as usize;

    let s0 = c0 & 3;
    let s1 = c1 & 3;
    let s2 = c2 & 3;
    let s3 = c3 & 3;
    let s4 = c4 & 3;

    if s0 == s1 && s1 == s2 && s2 == s3 && s3 == s4 {
        let bits = (1u16 << r0) | (1u16 << r1) | (1u16 << r2) | (1u16 << r3) | (1u16 << r4);
        return HandRank(flush_table[bits as usize]);
    }

    let prod = PRIMES[r0] * PRIMES[r1] * PRIMES[r2] * PRIMES[r3] * PRIMES[r4];
    HandRank(unique5_fast.get(prod))
}

/// Convert the HashMap table to Vec of entries for FastLookup construction.
pub fn unique5_entries(table: &HashMap<u32, u16>) -> Vec<(u32, u16)> {
    table.iter().map(|(&k, &v)| (k, v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flush_table_generation() {
        let table = generate_flush_table();
        // Royal flush = rank 1
        let royal = (1u16 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8);
        assert_eq!(table[royal as usize], 1);
        // Wheel straight flush = rank 10
        let wheel = (1u16 << 12) | (1 << 3) | (1 << 2) | (1 << 1) | (1 << 0);
        assert_eq!(table[wheel as usize], 10);
    }

    #[test]
    fn test_unique5_table_generation() {
        let table = generate_unique5_table();
        // Four aces with king kicker
        let prod = PRIMES[12].pow(4) * PRIMES[11]; // A^4 * K
        assert_eq!(table[&prod], 11, "Four aces + king should be rank 11");
    }

    #[test]
    fn test_hand_class_count() {
        let flush_table = generate_flush_table();
        let unique5 = generate_unique5_table();

        // Count distinct non-zero flush ranks
        let flush_count = flush_table.iter().filter(|&&v| v > 0).count();
        // 10 straight flushes + 1277 flushes = 1287
        assert_eq!(flush_count, 1287, "Should have 1287 flush hand classes");

        // Count unique5 entries
        let unique5_count = unique5.len();
        // 10 straights + 156 four-of-a-kind + 156 full houses + 858 three-of-a-kind
        // + 858 two-pair + 2860 one-pair + 1277 high-card = 6175
        assert_eq!(
            unique5_count, 6175,
            "Should have 6175 non-flush hand classes"
        );

        // Total: 1287 + 6175 = 7462
        assert_eq!(flush_count + unique5_count, 7462);
    }
}

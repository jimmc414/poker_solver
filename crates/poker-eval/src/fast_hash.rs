/// A fast open-addressing hash table optimized for the hand evaluator.
/// Uses Fibonacci hashing for O(1) amortized lookups with minimal collisions.
pub struct FastLookup {
    keys: Vec<u32>,
    values: Vec<u16>,
    shift: u32,
    mask: usize,
}

const GOLDEN_RATIO: u32 = 2654435769; // 2^32 / phi
const EMPTY: u32 = u32::MAX;

impl FastLookup {
    /// Build from a set of (product, rank) pairs.
    pub fn from_entries(entries: &[(u32, u16)]) -> Self {
        // Use 2^14 = 16384 slots for ~6175 entries (load factor ~37%)
        let bits = 14u32;
        let size = 1usize << bits;
        let shift = 32 - bits;
        let mask = size - 1;

        let mut keys = vec![EMPTY; size];
        let mut values = vec![0u16; size];

        for &(key, value) in entries {
            let mut idx = ((key.wrapping_mul(GOLDEN_RATIO)) >> shift) as usize & mask;
            loop {
                if keys[idx] == EMPTY {
                    keys[idx] = key;
                    values[idx] = value;
                    break;
                }
                idx = (idx + 1) & mask;
            }
        }

        FastLookup {
            keys,
            values,
            shift,
            mask,
        }
    }

    /// Look up a prime product. Returns 7462 (worst hand) if not found.
    #[inline(always)]
    pub fn get(&self, key: u32) -> u16 {
        let mut idx = ((key.wrapping_mul(GOLDEN_RATIO)) >> self.shift) as usize & self.mask;
        loop {
            let k = self.keys[idx];
            if k == key {
                return self.values[idx];
            }
            if k == EMPTY {
                return 7462;
            }
            idx = (idx + 1) & self.mask;
        }
    }
}

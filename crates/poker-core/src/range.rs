use serde::{Deserialize, Serialize};
use std::fmt;

/// A range represented as 169 weight values (0.0 to 1.0) for each canonical hand group.
/// Index maps to the 13x13 matrix: row*13 + col.
#[derive(Clone, Serialize, Deserialize)]
pub struct Range {
    weights: Vec<f32>,
}

impl Range {
    /// Create an empty range (all weights 0).
    pub fn empty() -> Self {
        Range {
            weights: vec![0.0; 169],
        }
    }

    /// Create a full range (all weights 1.0).
    pub fn full() -> Self {
        Range {
            weights: vec![1.0; 169],
        }
    }

    /// Get weight for a canonical index.
    pub fn weight(&self, index: usize) -> f32 {
        self.weights.get(index).copied().unwrap_or(0.0)
    }

    /// Set weight for a canonical index.
    pub fn set_weight(&mut self, index: usize, weight: f32) {
        if let Some(w) = self.weights.get_mut(index) {
            *w = weight.clamp(0.0, 1.0);
        }
    }

    /// Get the raw weights slice.
    pub fn weights(&self) -> &[f32] {
        &self.weights
    }

    /// Get mutable weights slice.
    pub fn weights_mut(&mut self) -> &mut [f32] {
        &mut self.weights
    }

    /// Count the number of combos in this range (accounting for suited/offsuit/pair counts).
    pub fn combo_count(&self) -> f32 {
        let mut total = 0.0f32;
        for row in 0..13 {
            for col in 0..13 {
                let idx = row * 13 + col;
                let w = self.weights[idx];
                if w > 0.0 {
                    let count = if row == col {
                        6.0 // pair
                    } else if row < col {
                        4.0 // suited (upper triangle)
                    } else {
                        12.0 // offsuit (lower triangle)
                    };
                    total += w * count;
                }
            }
        }
        total
    }

    /// Fraction of all 1326 combos that are in this range.
    pub fn density(&self) -> f32 {
        self.combo_count() / 1326.0
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Range({:.1}% / {:.0} combos)",
            self.density() * 100.0,
            self.combo_count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_169_weights() {
        let mut range = Range::empty();
        assert_eq!(range.combo_count(), 0.0);

        range.set_weight(0, 1.0); // AA at (0,0)
        assert_eq!(range.weight(0), 1.0);
        assert_eq!(range.combo_count(), 6.0); // pair = 6 combos

        range.set_weight(1, 0.5); // AKs at (0,1)
        let expected = 6.0 + 0.5 * 4.0; // 6 + 2 = 8
        assert!((range.combo_count() - expected).abs() < 0.01);

        let full = Range::full();
        assert_eq!(full.combo_count(), 1326.0);
        assert!((full.density() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_range_clamping() {
        let mut range = Range::empty();
        range.set_weight(0, 2.0); // Should clamp to 1.0
        assert_eq!(range.weight(0), 1.0);
        range.set_weight(0, -1.0); // Should clamp to 0.0
        assert_eq!(range.weight(0), 0.0);
    }
}

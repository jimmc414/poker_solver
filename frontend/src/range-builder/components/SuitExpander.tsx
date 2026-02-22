import React from 'react';
import { MATRIX_LABELS } from '../../shared/components/HandMatrix';
import { theme } from '../../shared/styles/theme';
import type { Suit } from '../../shared/types/poker';

interface SuitExpanderProps {
  /** Selected cell index (0-168), or null if nothing selected */
  selectedIndex: number | null;
}

const SUITS: Suit[] = ['s', 'h', 'd', 'c'];

const SUIT_SYMBOLS: Record<Suit, string> = {
  s: '\u2660',
  h: '\u2665',
  d: '\u2666',
  c: '\u2663',
};

const SUIT_COLORS: Record<Suit, string> = {
  s: theme.colors.spade,
  h: theme.colors.heart,
  d: theme.colors.diamond,
  c: theme.colors.club,
};

interface ComboDisplay {
  label: string;
  suits: [Suit, Suit];
}

/**
 * Returns the hand type for a given matrix index.
 */
function getHandType(index: number): 'pair' | 'suited' | 'offsuit' {
  const row = Math.floor(index / 13);
  const col = index % 13;
  if (row === col) return 'pair';
  if (col > row) return 'suited';
  return 'offsuit';
}

/**
 * Generates the specific suit combos for a canonical hand.
 */
function getSuitCombos(index: number): ComboDisplay[] {
  const handType = getHandType(index);
  const label = MATRIX_LABELS[index];
  const rank1 = label[0];
  const rank2 = label[1];
  const combos: ComboDisplay[] = [];

  if (handType === 'pair') {
    // 6 combos: all pairs of suits
    for (let i = 0; i < SUITS.length; i++) {
      for (let j = i + 1; j < SUITS.length; j++) {
        combos.push({
          label: `${rank1}${SUIT_SYMBOLS[SUITS[i]]}${rank2}${SUIT_SYMBOLS[SUITS[j]]}`,
          suits: [SUITS[i], SUITS[j]],
        });
      }
    }
  } else if (handType === 'suited') {
    // 4 combos: same suit
    for (const suit of SUITS) {
      combos.push({
        label: `${rank1}${SUIT_SYMBOLS[suit]}${rank2}${SUIT_SYMBOLS[suit]}`,
        suits: [suit, suit],
      });
    }
  } else {
    // 12 combos: different suits
    for (const suit1 of SUITS) {
      for (const suit2 of SUITS) {
        if (suit1 !== suit2) {
          combos.push({
            label: `${rank1}${SUIT_SYMBOLS[suit1]}${rank2}${SUIT_SYMBOLS[suit2]}`,
            suits: [suit1, suit2],
          });
        }
      }
    }
  }

  return combos;
}

/**
 * Shows individual suit combinations when a cell is selected.
 * Pairs = 6 combos, Suited = 4 combos, Offsuit = 12 combos.
 */
export const SuitExpander: React.FC<SuitExpanderProps> = ({ selectedIndex }) => {
  if (selectedIndex === null || selectedIndex < 0 || selectedIndex >= 169) {
    return (
      <div data-testid="suit-expander" style={{ color: theme.colors.textMuted, fontSize: theme.fontSize.sm }}>
        Select a hand to see suit combinations
      </div>
    );
  }

  const handLabel = MATRIX_LABELS[selectedIndex];
  const handType = getHandType(selectedIndex);
  const combos = getSuitCombos(selectedIndex);

  return (
    <div data-testid="suit-expander">
      <div
        style={{
          fontSize: theme.fontSize.sm,
          color: theme.colors.textSecondary,
          marginBottom: theme.spacing.xs,
        }}
      >
        {handLabel} ({handType}) - {combos.length} combos
      </div>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: handType === 'offsuit' ? 'repeat(4, 1fr)' : 'repeat(3, 1fr)',
          gap: theme.spacing.xs,
        }}
      >
        {combos.map((combo, idx) => (
          <div
            key={idx}
            style={{
              padding: `${theme.spacing.xs}px`,
              background: theme.colors.bgTertiary,
              border: `1px solid ${theme.colors.border}`,
              borderRadius: theme.borderRadius.sm,
              fontSize: theme.fontSize.xs,
              textAlign: 'center',
              color: SUIT_COLORS[combo.suits[0]],
              cursor: 'default',
            }}
          >
            {combo.label}
          </div>
        ))}
      </div>
    </div>
  );
};

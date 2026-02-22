import React from 'react';
import { theme } from '../../shared/styles/theme';

interface RangeSummaryProps {
  weights: number[];
}

const TOTAL_COMBOS = 1326;

/**
 * Returns the number of specific combos for a canonical hand index.
 */
function getComboCount(index: number): number {
  const row = Math.floor(index / 13);
  const col = index % 13;
  if (row === col) return 6;       // pair
  if (col > row) return 4;         // suited
  return 12;                       // offsuit
}

/**
 * Displays a summary of the current range:
 * - Number of combos included (weighted)
 * - Percentage of total 1326 combos
 */
export const RangeSummary: React.FC<RangeSummaryProps> = ({ weights }) => {
  let weightedCombos = 0;
  let handsIncluded = 0;

  for (let i = 0; i < 169; i++) {
    const w = weights[i] ?? 0;
    if (w > 0) {
      handsIncluded++;
      weightedCombos += getComboCount(i) * (w / 100);
    }
  }

  const percentage = ((weightedCombos / TOTAL_COMBOS) * 100).toFixed(1);

  return (
    <div data-testid="range-summary" style={{ display: 'flex', flexDirection: 'column', gap: theme.spacing.xs }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: theme.fontSize.sm }}>
        <span style={{ color: theme.colors.textSecondary }}>Hands</span>
        <span style={{ color: theme.colors.text }}>{handsIncluded} / 169</span>
      </div>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: theme.fontSize.sm }}>
        <span style={{ color: theme.colors.textSecondary }}>Combos</span>
        <span style={{ color: theme.colors.text }}>{weightedCombos.toFixed(1)}</span>
      </div>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: theme.fontSize.sm }}>
        <span style={{ color: theme.colors.textSecondary }}>Range %</span>
        <span style={{ color: theme.colors.accent, fontWeight: 600 }}>{percentage}%</span>
      </div>
    </div>
  );
};

import React from 'react';
import { theme } from '../../shared/styles/theme';

interface WeightControlsProps {
  weight: number;
  onWeightChange: (weight: number) => void;
}

/**
 * Slider control for setting the paint weight (0-100%).
 * Displays the current value as a percentage.
 */
export const WeightControls: React.FC<WeightControlsProps> = ({ weight, onWeightChange }) => {
  return (
    <div data-testid="weight-controls" style={{ display: 'flex', flexDirection: 'column', gap: theme.spacing.xs }}>
      <label
        style={{
          fontSize: theme.fontSize.sm,
          color: theme.colors.textSecondary,
          display: 'flex',
          justifyContent: 'space-between',
        }}
      >
        <span>Weight</span>
        <span style={{ color: theme.colors.text, fontWeight: 600 }}>{weight}%</span>
      </label>
      <input
        type="range"
        min={0}
        max={100}
        value={weight}
        onChange={(e) => onWeightChange(Number(e.target.value))}
        data-testid="weight-slider"
        style={{ width: '100%', accentColor: theme.colors.accent }}
      />
    </div>
  );
};

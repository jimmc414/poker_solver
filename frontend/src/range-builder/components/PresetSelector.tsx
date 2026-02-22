import React from 'react';
import { theme } from '../../shared/styles/theme';

interface PresetSelectorProps {
  activePreset: string | null;
  onLoadPreset: (weights: number[]) => void;
}

interface PresetDefinition {
  name: string;
  label: string;
  weights: number[];
}

/**
 * Creates a 169-element weight array filled with a given value.
 */
function fillWeights(value: number): number[] {
  return new Array(169).fill(value);
}

/**
 * Preset range definitions.
 * In a production build, these would be loaded from the backend.
 * For now, hardcoded approximations of common opening ranges.
 */
const PRESETS: PresetDefinition[] = [
  { name: 'empty', label: 'Empty', weights: fillWeights(0) },
  { name: 'full', label: 'Full', weights: fillWeights(100) },
  {
    name: 'utg-open',
    label: 'UTG Open',
    weights: (() => {
      const w = fillWeights(0);
      // Pairs: AA-77
      [0, 14, 28, 42, 56, 70, 84, 98].forEach((i) => { w[i] = 100; });
      // Suited broadway: AKs, AQs, AJs, ATs, KQs, KJs, QJs
      [1, 2, 3, 4, 15, 16, 29].forEach((i) => { w[i] = 100; });
      // Offsuit broadway: AKo, AQo
      [13, 26].forEach((i) => { w[i] = 100; });
      return w;
    })(),
  },
  {
    name: 'btn-open',
    label: 'BTN Open',
    weights: (() => {
      const w = fillWeights(0);
      // Pairs: AA-22
      for (let i = 0; i < 13; i++) { w[i * 13 + i] = 100; }
      // Suited: top half
      for (let r = 0; r < 8; r++) {
        for (let c = r + 1; c < 13; c++) {
          w[r * 13 + c] = 100;
        }
      }
      // Offsuit: top-left quadrant
      for (let r = 1; r < 7; r++) {
        for (let c = 0; c < r; c++) {
          w[r * 13 + c] = 100;
        }
      }
      return w;
    })(),
  },
  {
    name: '3-bet',
    label: '3-Bet',
    weights: (() => {
      const w = fillWeights(0);
      // Pairs: AA, KK, QQ, JJ
      [0, 14, 28, 42].forEach((i) => { w[i] = 100; });
      // AKs, AQs
      [1, 2].forEach((i) => { w[i] = 100; });
      // AKo
      w[13] = 100;
      return w;
    })(),
  },
];

/**
 * Dropdown selector for preset ranges.
 * Loading a preset dispatches the weights to the range state reducer.
 */
export const PresetSelector: React.FC<PresetSelectorProps> = ({ activePreset, onLoadPreset }) => {
  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selected = PRESETS.find((p) => p.name === e.target.value);
    if (selected) {
      onLoadPreset(selected.weights);
    }
  };

  return (
    <div data-testid="preset-selector">
      <label
        style={{
          display: 'block',
          fontSize: theme.fontSize.sm,
          color: theme.colors.textSecondary,
          marginBottom: theme.spacing.xs,
        }}
      >
        Presets
      </label>
      <select
        data-testid="preset-dropdown"
        value={activePreset ?? ''}
        onChange={handleChange}
        style={{
          width: '100%',
          padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
          background: theme.colors.bgTertiary,
          color: theme.colors.text,
          border: `1px solid ${theme.colors.border}`,
          borderRadius: theme.borderRadius.sm,
          fontSize: theme.fontSize.md,
          cursor: 'pointer',
        }}
      >
        <option value="" disabled>
          Select preset...
        </option>
        {PRESETS.map((preset) => (
          <option key={preset.name} value={preset.name}>
            {preset.label}
          </option>
        ))}
      </select>
    </div>
  );
};

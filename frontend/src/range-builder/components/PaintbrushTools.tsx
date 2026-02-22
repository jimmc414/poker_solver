import React from 'react';
import { theme } from '../../shared/styles/theme';

interface PaintbrushToolsProps {
  activeMode: 'select' | 'paint' | 'erase';
  onModeChange: (mode: 'select' | 'paint' | 'erase') => void;
}

const MODES: { key: 'select' | 'paint' | 'erase'; label: string }[] = [
  { key: 'select', label: 'Select' },
  { key: 'paint', label: 'Paint' },
  { key: 'erase', label: 'Erase' },
];

/**
 * Three-button toggle for paint interaction modes: Select, Paint, and Erase.
 */
export const PaintbrushTools: React.FC<PaintbrushToolsProps> = ({ activeMode, onModeChange }) => {
  return (
    <div
      data-testid="paintbrush-tools"
      style={{ display: 'flex', gap: theme.spacing.xs }}
    >
      {MODES.map(({ key, label }) => (
        <button
          key={key}
          data-testid={`mode-${key}`}
          onClick={() => onModeChange(key)}
          style={{
            flex: 1,
            padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
            background: activeMode === key ? theme.colors.accent : theme.colors.bgTertiary,
            color: activeMode === key ? '#fff' : theme.colors.text,
            border: `1px solid ${activeMode === key ? theme.colors.accent : theme.colors.border}`,
            borderRadius: theme.borderRadius.sm,
            cursor: 'pointer',
            fontSize: theme.fontSize.sm,
            fontWeight: activeMode === key ? 600 : 400,
          }}
        >
          {label}
        </button>
      ))}
    </div>
  );
};

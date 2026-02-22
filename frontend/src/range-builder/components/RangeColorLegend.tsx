import React from 'react';
import { theme } from '../../shared/styles/theme';

/**
 * Color legend showing how cell weights map to visual colors.
 * 0% = transparent, 50% = medium green, 100% = full green.
 */
export const RangeColorLegend: React.FC = () => {
  const stops = [
    { label: '0%', alpha: 0 },
    { label: '25%', alpha: 0.25 },
    { label: '50%', alpha: 0.5 },
    { label: '75%', alpha: 0.75 },
    { label: '100%', alpha: 1 },
  ];

  return (
    <div data-testid="range-color-legend">
      <div
        style={{
          fontSize: theme.fontSize.sm,
          color: theme.colors.textSecondary,
          marginBottom: theme.spacing.xs,
        }}
      >
        Weight Legend
      </div>
      <div style={{ display: 'flex', gap: theme.spacing.xs, alignItems: 'center' }}>
        {stops.map(({ label, alpha }) => (
          <div key={label} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 2 }}>
            <div
              style={{
                width: 28,
                height: 20,
                backgroundColor: alpha === 0 ? theme.colors.bgTertiary : `rgba(76, 175, 80, ${alpha})`,
                border: `1px solid ${theme.colors.border}`,
                borderRadius: theme.borderRadius.sm,
              }}
            />
            <span style={{ fontSize: theme.fontSize.xs, color: theme.colors.textMuted }}>{label}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

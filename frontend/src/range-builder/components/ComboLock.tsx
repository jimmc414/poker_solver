import React from 'react';
import { theme } from '../../shared/styles/theme';

interface ComboLockProps {
  /** Number of currently selected cells */
  selectedCount: number;
  /** Whether any of the selected cells are locked */
  hasLockedSelected: boolean;
  /** Toggle lock on all selected cells */
  onToggleLock: () => void;
}

/**
 * Toggle button to lock or unlock the currently selected combos.
 * Locked combos are immune to paint operations.
 */
export const ComboLock: React.FC<ComboLockProps> = ({
  selectedCount,
  hasLockedSelected,
  onToggleLock,
}) => {
  const isDisabled = selectedCount === 0;

  return (
    <div data-testid="combo-lock">
      <button
        data-testid="lock-toggle"
        onClick={onToggleLock}
        disabled={isDisabled}
        style={{
          width: '100%',
          padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
          background: isDisabled ? theme.colors.bgTertiary : theme.colors.bgSecondary,
          color: isDisabled ? theme.colors.textMuted : theme.colors.text,
          border: `1px solid ${isDisabled ? theme.colors.border : theme.colors.warning}`,
          borderRadius: theme.borderRadius.sm,
          fontSize: theme.fontSize.sm,
          cursor: isDisabled ? 'not-allowed' : 'pointer',
          opacity: isDisabled ? 0.5 : 1,
        }}
      >
        {hasLockedSelected ? 'Unlock Selected' : 'Lock Selected'} ({selectedCount})
      </button>
    </div>
  );
};

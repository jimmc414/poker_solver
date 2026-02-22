import React, { useMemo } from 'react';
import type { AppMode } from '../types/common';
import type { MatrixOverlay } from '../types/matrix';
import { useKeyboardShortcuts } from '../hooks/useKeyboardShortcuts';

interface KeyboardShortcutsProps {
  onModeChange: (mode: AppMode) => void;
  onToggleSidebar?: () => void;
  onOverlayChange?: (overlay: MatrixOverlay) => void;
}

const OVERLAY_MAP: Record<string, MatrixOverlay> = {
  '1': 'strategy',
  '2': 'ev',
  '3': 'equity',
  '4': 'eqr',
};

/**
 * Global keyboard shortcut handler.
 * Renders nothing -- purely registers event listeners.
 *
 * Shortcuts:
 *   j -> Study
 *   s -> Solve
 *   p -> Practice
 *   q -> Analyze
 *   Space -> Toggle sidebar
 *   1-4 -> Switch matrix overlay
 */
export const KeyboardShortcuts: React.FC<KeyboardShortcutsProps> = ({
  onModeChange,
  onToggleSidebar,
  onOverlayChange,
}) => {
  const shortcuts = useMemo(() => {
    const map: Record<string, () => void> = {
      j: () => onModeChange('study'),
      s: () => onModeChange('solve'),
      p: () => onModeChange('practice'),
      q: () => onModeChange('analyze'),
    };

    if (onToggleSidebar) {
      map[' '] = onToggleSidebar;
    }

    if (onOverlayChange) {
      for (const [key, overlay] of Object.entries(OVERLAY_MAP)) {
        map[key] = () => onOverlayChange(overlay);
      }
    }

    return map;
  }, [onModeChange, onToggleSidebar, onOverlayChange]);

  useKeyboardShortcuts(shortcuts);

  return null;
};

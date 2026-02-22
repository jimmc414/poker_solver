import { useEffect } from 'react';

/**
 * Custom hook that registers keyboard shortcuts.
 * Takes a map of key identifiers to callback functions.
 * Ignores events when the user is typing in an input/textarea.
 *
 * @param shortcuts - Map of keyboard key (e.g., 'j', 'Escape', ' ') to handler function.
 * @param enabled - Whether shortcuts are active. Defaults to true.
 */
export function useKeyboardShortcuts(
  shortcuts: Record<string, () => void>,
  enabled: boolean = true,
): void {
  useEffect(() => {
    if (!enabled) return;

    const handler = (event: KeyboardEvent) => {
      // Ignore events from input fields
      const target = event.target as HTMLElement;
      if (
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.tagName === 'SELECT' ||
        target.isContentEditable
      ) {
        return;
      }

      const callback = shortcuts[event.key];
      if (callback) {
        event.preventDefault();
        callback();
      }
    };

    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [shortcuts, enabled]);
}

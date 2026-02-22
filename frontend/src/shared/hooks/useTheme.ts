import type { Theme } from '../styles/theme';
import { theme } from '../styles/theme';

/**
 * Returns the application theme object.
 * Currently returns the static dark theme; can be extended
 * to support dynamic theming via context if needed.
 */
export function useTheme(): Theme {
  return theme;
}

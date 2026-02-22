import type { ActionType } from '../types/poker';
import type { ActionFrequencies } from '../types/matrix';
import { theme } from '../styles/theme';

/**
 * Returns the theme color associated with a poker action.
 */
export function getActionColor(action: ActionType): string {
  switch (action) {
    case 'bet':
    case 'raise':
    case 'allin':
      return theme.colors.bet;
    case 'check':
    case 'call':
      return theme.colors.check;
    case 'fold':
      return theme.colors.fold;
  }
}

/**
 * Returns the dark variant of an action color, used for borders/accents.
 */
export function getActionColorDark(action: ActionType): string {
  switch (action) {
    case 'bet':
    case 'raise':
    case 'allin':
      return theme.colors.betDark;
    case 'check':
    case 'call':
      return theme.colors.checkDark;
    case 'fold':
      return theme.colors.foldDark;
  }
}

interface ColorStop {
  color: string;
  fraction: number;
}

/**
 * Returns an array of color stops for rendering proportional action frequency bars.
 * Each stop has a color and a fraction (0-1) representing the proportion of that action.
 * Stops are ordered: bet, raise, call, check, fold (non-zero only).
 */
export function getActionColors(frequencies: ActionFrequencies): ColorStop[] {
  const stops: ColorStop[] = [];

  if (frequencies.bet > 0) {
    stops.push({ color: theme.colors.bet, fraction: frequencies.bet });
  }
  if (frequencies.raise && frequencies.raise > 0) {
    stops.push({ color: theme.colors.warning, fraction: frequencies.raise });
  }
  if (frequencies.call && frequencies.call > 0) {
    stops.push({ color: theme.colors.checkDark, fraction: frequencies.call });
  }
  if (frequencies.check > 0) {
    stops.push({ color: theme.colors.check, fraction: frequencies.check });
  }
  if (frequencies.fold > 0) {
    stops.push({ color: theme.colors.fold, fraction: frequencies.fold });
  }

  return stops;
}

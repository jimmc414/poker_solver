/**
 * Dark theme design tokens for the GTO Poker Solver.
 * All color, spacing, and typography values are centralized here.
 */

export const theme = {
  colors: {
    bg: '#1a1a1a',
    bgSecondary: '#242424',
    bgTertiary: '#2d2d2d',
    text: '#e0e0e0',
    textSecondary: '#a0a0a0',
    textMuted: '#666666',
    border: '#404040',
    borderLight: '#555555',

    // Action colors
    bet: '#ef5350',
    betDark: '#c62828',
    check: '#66bb6a',
    checkDark: '#2e7d32',
    fold: '#42a5f5',
    foldDark: '#1565c0',

    // Suits
    spade: '#e0e0e0',
    heart: '#ef5350',
    diamond: '#42a5f5',
    club: '#66bb6a',

    // Status
    accent: '#bb86fc',
    warning: '#ffb74d',
    error: '#ef5350',
    success: '#66bb6a',
  },

  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 24,
    xl: 32,
  },

  borderRadius: {
    sm: 4,
    md: 8,
    lg: 12,
  },

  fontSize: {
    xs: 10,
    sm: 12,
    md: 14,
    lg: 16,
    xl: 20,
    xxl: 24,
  },
} as const;

export type Theme = typeof theme;

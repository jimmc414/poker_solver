import React from 'react';
import type { Card, Suit } from '../types/poker';
import { theme } from '../styles/theme';

const SUIT_SYMBOLS: Record<Suit, string> = {
  s: '\u2660', // spade
  h: '\u2665', // heart
  d: '\u2666', // diamond
  c: '\u2663', // club
};

const SUIT_COLORS: Record<Suit, string> = {
  s: theme.colors.spade,
  h: theme.colors.heart,
  d: theme.colors.diamond,
  c: theme.colors.club,
};

const SIZE_STYLES: Record<string, React.CSSProperties> = {
  sm: { fontSize: 12, padding: '2px 4px', minWidth: 28 },
  md: { fontSize: 16, padding: '4px 6px', minWidth: 36 },
  lg: { fontSize: 22, padding: '6px 10px', minWidth: 48 },
};

interface CardRendererProps {
  card: Card;
  size?: 'sm' | 'md' | 'lg';
}

/**
 * Renders a playing card with rank, suit symbol, and appropriate color.
 */
export const CardRenderer: React.FC<CardRendererProps> = ({ card, size = 'md' }) => {
  const suitColor = SUIT_COLORS[card.suit];
  const suitSymbol = SUIT_SYMBOLS[card.suit];
  const sizeStyle = SIZE_STYLES[size];

  const containerStyle: React.CSSProperties = {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 1,
    backgroundColor: theme.colors.bgTertiary,
    border: `1px solid ${theme.colors.border}`,
    borderRadius: theme.borderRadius.sm,
    fontFamily: "'Courier New', Courier, monospace",
    fontWeight: 700,
    color: suitColor,
    userSelect: 'none',
    lineHeight: 1,
    ...sizeStyle,
  };

  return (
    <span style={containerStyle} data-testid="card" data-rank={card.rank} data-suit={card.suit}>
      {card.rank}
      {suitSymbol}
    </span>
  );
};

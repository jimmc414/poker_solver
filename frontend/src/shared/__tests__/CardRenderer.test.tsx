import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { CardRenderer } from '../components/CardRenderer';
import type { Card, Suit } from '../types/poker';

describe('CardRenderer', () => {
  it('renders a card with rank and suit symbol', () => {
    const card: Card = { rank: 'A', suit: 's' };
    render(<CardRenderer card={card} />);

    const el = screen.getByTestId('card');
    expect(el).toBeInTheDocument();
    expect(el.textContent).toContain('A');
    // Spade symbol
    expect(el.textContent).toContain('\u2660');
  });

  it('renders heart suit with correct symbol', () => {
    const card: Card = { rank: 'K', suit: 'h' };
    render(<CardRenderer card={card} />);

    const el = screen.getByTestId('card');
    expect(el.textContent).toContain('K');
    expect(el.textContent).toContain('\u2665');
  });

  it('renders diamond suit with correct symbol', () => {
    const card: Card = { rank: 'Q', suit: 'd' };
    render(<CardRenderer card={card} />);

    const el = screen.getByTestId('card');
    expect(el.textContent).toContain('Q');
    expect(el.textContent).toContain('\u2666');
  });

  it('renders club suit with correct symbol', () => {
    const card: Card = { rank: 'J', suit: 'c' };
    render(<CardRenderer card={card} />);

    const el = screen.getByTestId('card');
    expect(el.textContent).toContain('J');
    expect(el.textContent).toContain('\u2663');
  });

  it('sets data-rank and data-suit attributes', () => {
    const card: Card = { rank: 'T', suit: 'h' };
    render(<CardRenderer card={card} />);

    const el = screen.getByTestId('card');
    expect(el).toHaveAttribute('data-rank', 'T');
    expect(el).toHaveAttribute('data-suit', 'h');
  });

  it('renders at different sizes', () => {
    const card: Card = { rank: '2', suit: 'c' };

    const { unmount: unmountSm } = render(<CardRenderer card={card} size="sm" />);
    expect(screen.getByTestId('card')).toBeInTheDocument();
    unmountSm();

    const { unmount: unmountMd } = render(<CardRenderer card={card} size="md" />);
    expect(screen.getByTestId('card')).toBeInTheDocument();
    unmountMd();

    render(<CardRenderer card={card} size="lg" />);
    expect(screen.getByTestId('card')).toBeInTheDocument();
  });

  it('applies correct suit color via inline style', () => {
    // JSDOM normalizes hex colors to rgb(), so we compare against rgb equivalents
    const expectedColors: Record<Suit, string> = {
      s: 'rgb(224, 224, 224)',   // #e0e0e0
      h: 'rgb(239, 83, 80)',    // #ef5350
      d: 'rgb(66, 165, 245)',   // #42a5f5
      c: 'rgb(102, 187, 106)',  // #66bb6a
    };

    const suits: Suit[] = ['s', 'h', 'd', 'c'];
    for (const suit of suits) {
      const card: Card = { rank: 'A', suit };
      const { unmount } = render(<CardRenderer card={card} />);
      const el = screen.getByTestId('card');
      expect(el.style.color).toBe(expectedColors[suit]);
      unmount();
    }
  });

  it('defaults to md size when no size prop is provided', () => {
    const card: Card = { rank: '9', suit: 's' };
    render(<CardRenderer card={card} />);
    const el = screen.getByTestId('card');
    expect(el).toBeInTheDocument();
    // md size has fontSize 16
    expect(el.style.fontSize).toBe('16px');
  });
});

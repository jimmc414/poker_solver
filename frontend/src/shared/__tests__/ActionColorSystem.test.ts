import { describe, it, expect } from 'vitest';
import { getActionColor, getActionColorDark, getActionColors } from '../components/ActionColorSystem';
import { theme } from '../styles/theme';
import type { ActionType } from '../types/poker';
import type { ActionFrequencies } from '../types/matrix';

describe('ActionColorSystem', () => {
  describe('getActionColor', () => {
    it('returns bet color for bet action', () => {
      expect(getActionColor('bet')).toBe(theme.colors.bet);
    });

    it('returns bet color for raise action', () => {
      expect(getActionColor('raise')).toBe(theme.colors.bet);
    });

    it('returns bet color for allin action', () => {
      expect(getActionColor('allin')).toBe(theme.colors.bet);
    });

    it('returns check color for check action', () => {
      expect(getActionColor('check')).toBe(theme.colors.check);
    });

    it('returns check color for call action', () => {
      expect(getActionColor('call')).toBe(theme.colors.check);
    });

    it('returns fold color for fold action', () => {
      expect(getActionColor('fold')).toBe(theme.colors.fold);
    });

    it('returns a valid color string for every action type', () => {
      const actions: ActionType[] = ['fold', 'check', 'call', 'bet', 'raise', 'allin'];
      for (const action of actions) {
        const color = getActionColor(action);
        expect(color).toBeTruthy();
        expect(typeof color).toBe('string');
      }
    });
  });

  describe('getActionColorDark', () => {
    it('returns dark bet color for bet action', () => {
      expect(getActionColorDark('bet')).toBe(theme.colors.betDark);
    });

    it('returns dark check color for check action', () => {
      expect(getActionColorDark('check')).toBe(theme.colors.checkDark);
    });

    it('returns dark fold color for fold action', () => {
      expect(getActionColorDark('fold')).toBe(theme.colors.foldDark);
    });
  });

  describe('getActionColors', () => {
    it('returns color stops for mixed action frequencies', () => {
      const frequencies: ActionFrequencies = {
        bet: 0.4,
        check: 0.3,
        fold: 0.3,
      };
      const stops = getActionColors(frequencies);
      expect(stops).toHaveLength(3);
      expect(stops[0].color).toBe(theme.colors.bet);
      expect(stops[0].fraction).toBe(0.4);
      expect(stops[1].color).toBe(theme.colors.check);
      expect(stops[1].fraction).toBe(0.3);
      expect(stops[2].color).toBe(theme.colors.fold);
      expect(stops[2].fraction).toBe(0.3);
    });

    it('returns empty array for all-zero frequencies', () => {
      const frequencies: ActionFrequencies = {
        bet: 0,
        check: 0,
        fold: 0,
      };
      const stops = getActionColors(frequencies);
      expect(stops).toHaveLength(0);
    });

    it('includes optional raise and call when non-zero', () => {
      const frequencies: ActionFrequencies = {
        bet: 0.2,
        check: 0.2,
        fold: 0.2,
        raise: 0.2,
        call: 0.2,
      };
      const stops = getActionColors(frequencies);
      expect(stops).toHaveLength(5);
    });

    it('excludes optional raise and call when zero', () => {
      const frequencies: ActionFrequencies = {
        bet: 0.5,
        check: 0.5,
        fold: 0,
        raise: 0,
        call: 0,
      };
      const stops = getActionColors(frequencies);
      expect(stops).toHaveLength(2);
    });

    it('fractions sum to approximately 1 for valid frequencies', () => {
      const frequencies: ActionFrequencies = {
        bet: 0.3,
        check: 0.4,
        fold: 0.3,
      };
      const stops = getActionColors(frequencies);
      const total = stops.reduce((sum, s) => sum + s.fraction, 0);
      expect(total).toBeCloseTo(1.0);
    });
  });
});

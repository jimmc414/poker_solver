import { describe, it, expect } from 'vitest';
import { rangeReducer } from '../hooks/useRangeState';
import type { RangeState } from '../types';

function createState(overrides?: Partial<RangeState>): RangeState {
  return {
    weights: new Array(169).fill(0),
    locked: new Array(169).fill(false),
    selectedCells: new Set<number>(),
    paintMode: 'paint',
    paintWeight: 100,
    activePreset: null,
    ...overrides,
  };
}

describe('rangeReducer', () => {
  describe('SET_WEIGHT', () => {
    it('sets weight for a cell', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'SET_WEIGHT', index: 0, weight: 75 });
      expect(next.weights[0]).toBe(75);
    });

    it('clamps weight to 0-100', () => {
      const state = createState();
      const over = rangeReducer(state, { type: 'SET_WEIGHT', index: 0, weight: 150 });
      expect(over.weights[0]).toBe(100);
      const under = rangeReducer(state, { type: 'SET_WEIGHT', index: 0, weight: -10 });
      expect(under.weights[0]).toBe(0);
    });

    it('does not modify locked cells', () => {
      const locked = new Array(169).fill(false);
      locked[0] = true;
      const state = createState({ locked });
      const next = rangeReducer(state, { type: 'SET_WEIGHT', index: 0, weight: 50 });
      expect(next.weights[0]).toBe(0);
    });

    it('clears activePreset', () => {
      const state = createState({ activePreset: 'test' });
      const next = rangeReducer(state, { type: 'SET_WEIGHT', index: 5, weight: 50 });
      expect(next.activePreset).toBeNull();
    });
  });

  describe('PAINT_CELLS', () => {
    it('paints multiple cells at once', () => {
      const state = createState();
      const next = rangeReducer(state, {
        type: 'PAINT_CELLS',
        indices: [0, 1, 2],
        weight: 80,
      });
      expect(next.weights[0]).toBe(80);
      expect(next.weights[1]).toBe(80);
      expect(next.weights[2]).toBe(80);
    });

    it('skips locked cells during paint', () => {
      const locked = new Array(169).fill(false);
      locked[1] = true;
      const state = createState({ locked });
      const next = rangeReducer(state, {
        type: 'PAINT_CELLS',
        indices: [0, 1, 2],
        weight: 100,
      });
      expect(next.weights[0]).toBe(100);
      expect(next.weights[1]).toBe(0); // locked, unchanged
      expect(next.weights[2]).toBe(100);
    });

    it('returns same state if all cells are locked', () => {
      const locked = new Array(169).fill(false);
      locked[0] = true;
      locked[1] = true;
      const state = createState({ locked });
      const next = rangeReducer(state, {
        type: 'PAINT_CELLS',
        indices: [0, 1],
        weight: 50,
      });
      expect(next).toBe(state);
    });
  });

  describe('TOGGLE_LOCK', () => {
    it('locks an unlocked cell', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'TOGGLE_LOCK', index: 5 });
      expect(next.locked[5]).toBe(true);
    });

    it('unlocks a locked cell', () => {
      const locked = new Array(169).fill(false);
      locked[5] = true;
      const state = createState({ locked });
      const next = rangeReducer(state, { type: 'TOGGLE_LOCK', index: 5 });
      expect(next.locked[5]).toBe(false);
    });
  });

  describe('SET_PAINT_MODE', () => {
    it('changes paint mode', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'SET_PAINT_MODE', mode: 'erase' });
      expect(next.paintMode).toBe('erase');
    });
  });

  describe('SET_PAINT_WEIGHT', () => {
    it('sets paint weight', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'SET_PAINT_WEIGHT', weight: 50 });
      expect(next.paintWeight).toBe(50);
    });

    it('clamps paint weight', () => {
      const state = createState();
      const over = rangeReducer(state, { type: 'SET_PAINT_WEIGHT', weight: 200 });
      expect(over.paintWeight).toBe(100);
    });
  });

  describe('LOAD_PRESET', () => {
    it('loads preset weights', () => {
      const state = createState();
      const presetWeights = new Array(169).fill(100);
      const next = rangeReducer(state, { type: 'LOAD_PRESET', weights: presetWeights });
      expect(next.weights).toEqual(presetWeights);
    });

    it('respects locked cells when loading preset', () => {
      const locked = new Array(169).fill(false);
      locked[0] = true;
      const weights = new Array(169).fill(50);
      weights[0] = 50;
      const state = createState({ locked, weights });
      const presetWeights = new Array(169).fill(100);
      const next = rangeReducer(state, { type: 'LOAD_PRESET', weights: presetWeights });
      expect(next.weights[0]).toBe(50); // locked, unchanged
      expect(next.weights[1]).toBe(100); // unlocked, changed
    });
  });

  describe('CLEAR_ALL', () => {
    it('clears all unlocked cells to 0', () => {
      const weights = new Array(169).fill(100);
      const state = createState({ weights });
      const next = rangeReducer(state, { type: 'CLEAR_ALL' });
      expect(next.weights.every((w) => w === 0)).toBe(true);
    });

    it('preserves locked cell weights', () => {
      const weights = new Array(169).fill(100);
      const locked = new Array(169).fill(false);
      locked[0] = true;
      const state = createState({ weights, locked });
      const next = rangeReducer(state, { type: 'CLEAR_ALL' });
      expect(next.weights[0]).toBe(100); // locked
      expect(next.weights[1]).toBe(0);   // cleared
    });

    it('clears selection', () => {
      const state = createState({ selectedCells: new Set([0, 1, 2]) });
      const next = rangeReducer(state, { type: 'CLEAR_ALL' });
      expect(next.selectedCells.size).toBe(0);
    });
  });

  describe('SELECT_ALL', () => {
    it('selects all 169 cells', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'SELECT_ALL' });
      expect(next.selectedCells.size).toBe(169);
    });
  });

  describe('SELECT_CELL', () => {
    it('adds cell to selection', () => {
      const state = createState();
      const next = rangeReducer(state, { type: 'SELECT_CELL', index: 5 });
      expect(next.selectedCells.has(5)).toBe(true);
    });

    it('removes cell from selection if already selected', () => {
      const state = createState({ selectedCells: new Set([5]) });
      const next = rangeReducer(state, { type: 'SELECT_CELL', index: 5 });
      expect(next.selectedCells.has(5)).toBe(false);
    });
  });

  describe('DESELECT_ALL', () => {
    it('clears all selection', () => {
      const state = createState({ selectedCells: new Set([0, 1, 2, 3]) });
      const next = rangeReducer(state, { type: 'DESELECT_ALL' });
      expect(next.selectedCells.size).toBe(0);
    });
  });
});

import { useReducer, useCallback } from 'react';
import type { RangeState, RangeAction } from '../types';

const HAND_COUNT = 169;

function createInitialState(): RangeState {
  return {
    weights: new Array(HAND_COUNT).fill(0),
    locked: new Array(HAND_COUNT).fill(false),
    selectedCells: new Set<number>(),
    paintMode: 'paint',
    paintWeight: 100,
    activePreset: null,
  };
}

export function rangeReducer(state: RangeState, action: RangeAction): RangeState {
  switch (action.type) {
    case 'SET_WEIGHT': {
      if (state.locked[action.index]) return state;
      const weights = [...state.weights];
      weights[action.index] = Math.max(0, Math.min(100, action.weight));
      return { ...state, weights, activePreset: null };
    }

    case 'PAINT_CELLS': {
      const weights = [...state.weights];
      let changed = false;
      for (const idx of action.indices) {
        if (!state.locked[idx]) {
          weights[idx] = Math.max(0, Math.min(100, action.weight));
          changed = true;
        }
      }
      if (!changed) return state;
      return { ...state, weights, activePreset: null };
    }

    case 'TOGGLE_LOCK': {
      const locked = [...state.locked];
      locked[action.index] = !locked[action.index];
      return { ...state, locked };
    }

    case 'SET_PAINT_MODE':
      return { ...state, paintMode: action.mode };

    case 'SET_PAINT_WEIGHT':
      return { ...state, paintWeight: Math.max(0, Math.min(100, action.weight)) };

    case 'LOAD_PRESET': {
      const weights = [...state.weights];
      for (let i = 0; i < HAND_COUNT; i++) {
        if (!state.locked[i]) {
          weights[i] = action.weights[i] ?? 0;
        }
      }
      return { ...state, weights, activePreset: null };
    }

    case 'CLEAR_ALL': {
      const weights = [...state.weights];
      for (let i = 0; i < HAND_COUNT; i++) {
        if (!state.locked[i]) {
          weights[i] = 0;
        }
      }
      return { ...state, weights, activePreset: null, selectedCells: new Set() };
    }

    case 'SELECT_ALL':
      return {
        ...state,
        selectedCells: new Set(Array.from({ length: HAND_COUNT }, (_, i) => i)),
      };

    case 'SELECT_CELL': {
      const selectedCells = new Set(state.selectedCells);
      if (selectedCells.has(action.index)) {
        selectedCells.delete(action.index);
      } else {
        selectedCells.add(action.index);
      }
      return { ...state, selectedCells };
    }

    case 'DESELECT_ALL':
      return { ...state, selectedCells: new Set() };

    default:
      return state;
  }
}

/**
 * Hook for managing the 169-cell range state.
 * Uses useReducer for predictable state updates.
 */
export function useRangeState() {
  const [state, dispatch] = useReducer(rangeReducer, undefined, createInitialState);

  const setWeight = useCallback(
    (index: number, weight: number) => dispatch({ type: 'SET_WEIGHT', index, weight }),
    [],
  );

  const paintCells = useCallback(
    (indices: number[], weight: number) => dispatch({ type: 'PAINT_CELLS', indices, weight }),
    [],
  );

  const toggleLock = useCallback(
    (index: number) => dispatch({ type: 'TOGGLE_LOCK', index }),
    [],
  );

  const setPaintMode = useCallback(
    (mode: 'select' | 'paint' | 'erase') => dispatch({ type: 'SET_PAINT_MODE', mode }),
    [],
  );

  const setPaintWeight = useCallback(
    (weight: number) => dispatch({ type: 'SET_PAINT_WEIGHT', weight }),
    [],
  );

  const loadPreset = useCallback(
    (weights: number[]) => dispatch({ type: 'LOAD_PRESET', weights }),
    [],
  );

  const clearAll = useCallback(() => dispatch({ type: 'CLEAR_ALL' }), []);

  const selectAll = useCallback(() => dispatch({ type: 'SELECT_ALL' }), []);

  const selectCell = useCallback(
    (index: number) => dispatch({ type: 'SELECT_CELL', index }),
    [],
  );

  const deselectAll = useCallback(() => dispatch({ type: 'DESELECT_ALL' }), []);

  return {
    state,
    dispatch,
    setWeight,
    paintCells,
    toggleLock,
    setPaintMode,
    setPaintWeight,
    loadPreset,
    clearAll,
    selectAll,
    selectCell,
    deselectAll,
  };
}

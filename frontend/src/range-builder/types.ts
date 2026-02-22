export interface RangeState {
  /** Weight for each of the 169 canonical hands (0-100) */
  weights: number[];
  /** Whether each of the 169 hands is locked from editing */
  locked: boolean[];
  /** Currently selected cell indices */
  selectedCells: Set<number>;
  /** Current paint interaction mode */
  paintMode: 'select' | 'paint' | 'erase';
  /** Current paint weight value (0-100) */
  paintWeight: number;
  /** Name of the active preset, or null if custom */
  activePreset: string | null;
}

export type RangeAction =
  | { type: 'SET_WEIGHT'; index: number; weight: number }
  | { type: 'PAINT_CELLS'; indices: number[]; weight: number }
  | { type: 'TOGGLE_LOCK'; index: number }
  | { type: 'SET_PAINT_MODE'; mode: 'select' | 'paint' | 'erase' }
  | { type: 'SET_PAINT_WEIGHT'; weight: number }
  | { type: 'LOAD_PRESET'; weights: number[] }
  | { type: 'CLEAR_ALL' }
  | { type: 'SELECT_ALL' }
  | { type: 'SELECT_CELL'; index: number }
  | { type: 'DESELECT_ALL' };

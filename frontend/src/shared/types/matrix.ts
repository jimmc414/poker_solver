/**
 * Types for the 13x13 hand matrix display.
 * Used by HandMatrix canvas component and solution browsing.
 */

export interface ActionFrequencies {
  bet: number;
  check: number;
  fold: number;
  raise?: number;
  call?: number;
}

export interface MatrixCell {
  row: number;
  col: number;
  label: string;
  combos: number;
  weight: number;
  actions?: ActionFrequencies;
  ev?: number;
  equity?: number;
  /** Equity realization ratio */
  eqr?: number;
  /** Whether this cell is locked from editing (used in range builder) */
  locked?: boolean;
}

export type MatrixOverlay = 'strategy' | 'ev' | 'equity' | 'eqr';

export type ActionFilter = 'all' | 'bet' | 'check' | 'fold' | 'raise' | 'call';

export interface MatrixData {
  cells: MatrixCell[][];
  overlay: MatrixOverlay;
  filter: ActionFilter;
}

import React, { useCallback, useRef, useMemo } from 'react';
import { HandMatrix, MATRIX_LABELS } from '../../shared/components/HandMatrix';
import type { MatrixCell, MatrixData } from '../../shared/types/matrix';
import type { RangeState, RangeAction } from '../types';

interface RangeGridProps {
  state: RangeState;
  dispatch: React.Dispatch<RangeAction>;
}

function getComboCount(row: number, col: number): number {
  if (row === col) return 6;       // pair
  if (col > row) return 4;         // suited
  return 12;                       // offsuit
}

/**
 * Converts range state into MatrixData (13x13 2D array) for the HandMatrix component.
 */
function buildMatrixData(state: RangeState): MatrixData {
  const cells: MatrixCell[][] = [];
  for (let row = 0; row < 13; row++) {
    const rowCells: MatrixCell[] = [];
    for (let col = 0; col < 13; col++) {
      const i = row * 13 + col;
      rowCells.push({
        row,
        col,
        label: MATRIX_LABELS[i],
        combos: getComboCount(row, col),
        weight: state.weights[i],
        locked: state.locked[i],
      });
    }
    cells.push(rowCells);
  }
  return {
    cells,
    overlay: 'strategy',
    filter: 'all',
  };
}

/**
 * 13x13 range grid wrapping the shared HandMatrix canvas component.
 * Connects range state to the matrix and handles paint interactions.
 */
export const RangeGrid: React.FC<RangeGridProps> = ({ state, dispatch }) => {
  const lastPaintedRef = useRef<number | null>(null);

  const matrixData = useMemo(() => buildMatrixData(state), [state]);

  const handleCellAction = useCallback(
    (cell: MatrixCell) => {
      const index = cell.row * 13 + cell.col;
      if (state.paintMode === 'select') {
        dispatch({ type: 'SELECT_CELL', index });
      } else if (state.paintMode === 'paint') {
        dispatch({ type: 'PAINT_CELLS', indices: [index], weight: state.paintWeight });
      } else if (state.paintMode === 'erase') {
        dispatch({ type: 'PAINT_CELLS', indices: [index], weight: 0 });
      }
    },
    [state.paintMode, state.paintWeight, dispatch],
  );

  const handleClick = useCallback(
    (cell: MatrixCell) => {
      const index = cell.row * 13 + cell.col;
      lastPaintedRef.current = index;
      handleCellAction(cell);
    },
    [handleCellAction],
  );

  const handleDragStart = useCallback(
    (cell: MatrixCell) => {
      const index = cell.row * 13 + cell.col;
      lastPaintedRef.current = index;
      handleCellAction(cell);
    },
    [handleCellAction],
  );

  const handleDragEnter = useCallback(
    (cell: MatrixCell) => {
      const index = cell.row * 13 + cell.col;
      if (lastPaintedRef.current !== index) {
        lastPaintedRef.current = index;
        handleCellAction(cell);
      }
    },
    [handleCellAction],
  );

  const handleDragEnd = useCallback(() => {
    lastPaintedRef.current = null;
  }, []);

  return (
    <div data-testid="range-grid">
      <HandMatrix
        data={matrixData}
        onCellClick={handleClick}
        onCellDragStart={handleDragStart}
        onCellDragEnter={handleDragEnter}
        onCellDragEnd={handleDragEnd}
        interactive={true}
        width={520}
        height={520}
      />
    </div>
  );
};

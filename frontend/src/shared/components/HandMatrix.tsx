import React, { useRef, useEffect, useCallback, useState, useMemo } from 'react';
import type { MatrixData, MatrixCell, MatrixOverlay, ActionFilter } from '../types/matrix';
import { renderMatrix, hitTest, getCellLabel } from './HandMatrixCanvas';

/** Labels for the 13x13 hand matrix grid */
const MATRIX_LABELS: string[] = Array.from({ length: 169 }, (_, i) => {
  const row = Math.floor(i / 13);
  const col = i % 13;
  return getCellLabel(row, col);
});

export { MATRIX_LABELS };

export interface HandMatrixProps {
  /** Matrix data: 13x13 cells with overlay and filter settings */
  data: MatrixData;
  /** Called when user hovers a cell (null when leaving) */
  onCellHover?: (cell: MatrixCell | null) => void;
  /** Called when user clicks a cell */
  onCellClick?: (cell: MatrixCell) => void;
  /** Called when action filter changes */
  onFilterChange?: (filter: ActionFilter) => void;
  /** Called when overlay changes */
  onOverlayChange?: (overlay: MatrixOverlay) => void;
  /** Called when drag starts on a cell */
  onCellDragStart?: (cell: MatrixCell) => void;
  /** Called when drag enters a cell */
  onCellDragEnter?: (cell: MatrixCell) => void;
  /** Called when drag ends */
  onCellDragEnd?: () => void;
  /** Whether the matrix supports drag interaction */
  interactive?: boolean;
  /** Width in CSS pixels */
  width?: number;
  /** Height in CSS pixels */
  height?: number;
}

/**
 * Canvas-based 13x13 hand matrix component.
 * Renders the matrix using requestAnimationFrame for smooth updates.
 * Handles mouse interaction for hover, selection, and drag painting.
 *
 * Performance: Uses canvas rendering (not DOM grid) for 60fps as required by PRF-003.
 * HiDPI: Accounts for devicePixelRatio for sharp rendering on Retina displays.
 */
export const HandMatrix: React.FC<HandMatrixProps> = ({
  data,
  onCellHover,
  onCellClick,
  onFilterChange: _onFilterChange,
  onOverlayChange: _onOverlayChange,
  onCellDragStart,
  onCellDragEnter,
  onCellDragEnd,
  interactive = true,
  width = 520,
  height = 520,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animFrameRef = useRef<number | null>(null);
  const [hoveredCell, setHoveredCell] = useState<{ row: number; col: number } | null>(null);
  const [selectedCell, setSelectedCell] = useState<{ row: number; col: number } | null>(null);
  const [isDragging, setIsDragging] = useState(false);

  // Memoize the DPR to avoid re-renders
  const dpr = useMemo(() => {
    if (typeof window !== 'undefined') {
      return window.devicePixelRatio || 1;
    }
    return 1;
  }, []);

  // Rendering loop
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas buffer size for HiDPI
    canvas.width = width * dpr;
    canvas.height = height * dpr;

    const render = () => {
      renderMatrix(ctx, data, width, height, hoveredCell, selectedCell, dpr);
    };

    animFrameRef.current = requestAnimationFrame(render);

    return () => {
      if (animFrameRef.current !== null) {
        cancelAnimationFrame(animFrameRef.current);
        animFrameRef.current = null;
      }
    };
  }, [data, width, height, hoveredCell, selectedCell, dpr]);

  // Get mouse position relative to canvas in CSS pixels
  const getCanvasCoords = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>): { x: number; y: number } => {
      const canvas = canvasRef.current;
      if (!canvas) return { x: 0, y: 0 };

      const rect = canvas.getBoundingClientRect();
      return {
        x: event.clientX - rect.left,
        y: event.clientY - rect.top,
      };
    },
    [],
  );

  // Get MatrixCell from data.cells given row,col
  const getCellData = useCallback(
    (row: number, col: number): MatrixCell | null => {
      return data.cells[row]?.[col] ?? null;
    },
    [data.cells],
  );

  // Mouse move handler for hover detection
  const handleMouseMove = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      const { x, y } = getCanvasCoords(event);
      const hit = hitTest(x, y, width, height);

      setHoveredCell((prev) => {
        if (prev?.row === hit?.row && prev?.col === hit?.col) {
          // Still in the same cell, but may need to fire drag enter
          return prev;
        }
        if (hit && onCellHover) {
          onCellHover(getCellData(hit.row, hit.col));
        } else if (!hit && onCellHover) {
          onCellHover(null);
        }
        // If dragging and entering a new cell, fire drag enter
        if (isDragging && hit) {
          const cellData = getCellData(hit.row, hit.col);
          if (cellData && onCellDragEnter) {
            onCellDragEnter(cellData);
          }
        }
        return hit;
      });
    },
    [getCanvasCoords, width, height, getCellData, onCellHover, isDragging, onCellDragEnter],
  );

  // Mouse down handler for drag start
  const handleMouseDown = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      if (!interactive) return;
      const { x, y } = getCanvasCoords(event);
      const hit = hitTest(x, y, width, height);

      if (hit) {
        setIsDragging(true);
        const cellData = getCellData(hit.row, hit.col);
        if (cellData && onCellDragStart) {
          onCellDragStart(cellData);
        }
        if (cellData && onCellClick) {
          onCellClick(cellData);
        }
      }
    },
    [interactive, getCanvasCoords, width, height, getCellData, onCellDragStart, onCellClick],
  );

  // Mouse up handler for drag end
  const handleMouseUp = useCallback(() => {
    if (isDragging) {
      setIsDragging(false);
      if (onCellDragEnd) {
        onCellDragEnd();
      }
    }
  }, [isDragging, onCellDragEnd]);

  // Click handler for cell selection (when no drag handlers are provided)
  const handleClick = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      // If drag handlers are active, click is handled in mouseDown
      if (onCellDragStart) return;

      const { x, y } = getCanvasCoords(event);
      const hit = hitTest(x, y, width, height);

      if (hit) {
        setSelectedCell(hit);
        const cellData = getCellData(hit.row, hit.col);
        if (cellData && onCellClick) {
          onCellClick(cellData);
        }
      }
    },
    [getCanvasCoords, width, height, getCellData, onCellClick, onCellDragStart],
  );

  // Mouse leave handler
  const handleMouseLeave = useCallback(() => {
    setHoveredCell(null);
    if (onCellHover) {
      onCellHover(null);
    }
    if (isDragging) {
      setIsDragging(false);
      if (onCellDragEnd) {
        onCellDragEnd();
      }
    }
  }, [onCellHover, isDragging, onCellDragEnd]);

  return (
    <canvas
      ref={canvasRef}
      data-testid="hand-matrix-canvas"
      style={{
        width,
        height,
        cursor: interactive ? 'pointer' : 'default',
        display: 'block',
        imageRendering: 'auto',
      }}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
    />
  );
};

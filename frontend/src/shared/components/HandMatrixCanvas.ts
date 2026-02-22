/**
 * Canvas rendering logic for the 13x13 hand matrix.
 * Separated from the React component for testability and performance.
 *
 * The matrix is laid out as:
 *   - Rows/columns indexed by rank (A, K, Q, ..., 2)
 *   - Upper-right triangle: suited combos (e.g., AKs)
 *   - Lower-left triangle: offsuit combos (e.g., AKo)
 *   - Diagonal: pairs (e.g., AA, KK)
 */

import type { MatrixData, MatrixCell, ActionFrequencies, MatrixOverlay, ActionFilter } from '../types/matrix';
import { theme } from '../styles/theme';

const RANKS = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const GRID_SIZE = 13;

/**
 * Generates the label for a matrix cell at (row, col).
 */
export function getCellLabel(row: number, col: number): string {
  if (row === col) {
    return RANKS[row] + RANKS[col];
  } else if (col > row) {
    return RANKS[row] + RANKS[col] + 's';
  } else {
    return RANKS[col] + RANKS[row] + 'o';
  }
}

/**
 * Determines which cell (row, col) is under a given pixel coordinate.
 * Returns null if outside the grid.
 */
export function hitTest(
  x: number,
  y: number,
  width: number,
  height: number,
): { row: number; col: number } | null {
  const cellW = width / GRID_SIZE;
  const cellH = height / GRID_SIZE;

  const col = Math.floor(x / cellW);
  const row = Math.floor(y / cellH);

  if (row < 0 || row >= GRID_SIZE || col < 0 || col >= GRID_SIZE) {
    return null;
  }

  return { row, col };
}

/**
 * Returns the background color for a cell based on the current overlay mode.
 */
function getCellBackground(
  cell: MatrixCell,
  overlay: MatrixOverlay,
  filter: ActionFilter,
): string {
  // If a filter is applied and the cell does not have that action, dim it
  if (filter !== 'all' && cell.actions) {
    const freq = getFilteredFrequency(cell.actions, filter);
    if (freq <= 0) {
      return theme.colors.bg;
    }
  }

  switch (overlay) {
    case 'strategy':
      return theme.colors.bgTertiary;
    case 'ev':
      return getHeatmapColor(cell.ev ?? 0, -2, 2);
    case 'equity':
      return getHeatmapColor(cell.equity ?? 0, 0, 1);
    case 'eqr':
      return getHeatmapColor(cell.eqr ?? 0, 0, 2);
  }
}

function getFilteredFrequency(actions: ActionFrequencies, filter: ActionFilter): number {
  switch (filter) {
    case 'all': return 1;
    case 'bet': return actions.bet;
    case 'check': return actions.check;
    case 'fold': return actions.fold;
    case 'raise': return actions.raise ?? 0;
    case 'call': return actions.call ?? 0;
  }
}

/**
 * Maps a value to a color on a red-blue heatmap scale.
 */
function getHeatmapColor(value: number, min: number, max: number): string {
  const range = max - min;
  if (range === 0) return theme.colors.bgTertiary;

  const normalized = Math.max(0, Math.min(1, (value - min) / range));

  // Blue (cold) -> neutral -> Red (hot)
  const r = Math.round(normalized * 200 + 30);
  const g = Math.round((1 - Math.abs(normalized - 0.5) * 2) * 80 + 30);
  const b = Math.round((1 - normalized) * 200 + 30);

  return `rgb(${r}, ${g}, ${b})`;
}

/**
 * Draws the strategy action frequency bars inside a cell.
 */
function drawStrategyBars(
  ctx: CanvasRenderingContext2D,
  cell: MatrixCell,
  x: number,
  y: number,
  w: number,
  h: number,
): void {
  if (!cell.actions) return;

  const barHeight = 4;
  const barY = y + h - barHeight - 2;
  const barWidth = w - 4;
  const barX = x + 2;

  const actions = cell.actions;
  const entries: Array<{ color: string; freq: number }> = [];

  if (actions.bet > 0) entries.push({ color: theme.colors.bet, freq: actions.bet });
  if (actions.raise && actions.raise > 0) entries.push({ color: theme.colors.warning, freq: actions.raise });
  if (actions.call && actions.call > 0) entries.push({ color: theme.colors.checkDark, freq: actions.call });
  if (actions.check > 0) entries.push({ color: theme.colors.check, freq: actions.check });
  if (actions.fold > 0) entries.push({ color: theme.colors.fold, freq: actions.fold });

  let offsetX = 0;
  for (const entry of entries) {
    const segmentWidth = entry.freq * barWidth;
    ctx.fillStyle = entry.color;
    ctx.fillRect(barX + offsetX, barY, segmentWidth, barHeight);
    offsetX += segmentWidth;
  }
}

/**
 * Draws the metric value text inside a cell for non-strategy overlays.
 */
function drawMetricText(
  ctx: CanvasRenderingContext2D,
  cell: MatrixCell,
  overlay: MatrixOverlay,
  x: number,
  y: number,
  w: number,
  h: number,
): void {
  let value: number | undefined;
  let suffix = '';

  switch (overlay) {
    case 'ev':
      value = cell.ev;
      suffix = '';
      break;
    case 'equity':
      value = cell.equity;
      suffix = '%';
      break;
    case 'eqr':
      value = cell.eqr;
      suffix = '';
      break;
    default:
      return;
  }

  if (value === undefined) return;

  const displayValue = overlay === 'equity'
    ? (value * 100).toFixed(0) + suffix
    : value.toFixed(2) + suffix;

  ctx.fillStyle = theme.colors.textSecondary;
  ctx.font = `${Math.max(8, Math.floor(h * 0.22))}px Inter, sans-serif`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'bottom';
  ctx.fillText(displayValue, x + w / 2, y + h - 3, w - 4);
}

/**
 * Main render function for the 13x13 hand matrix.
 *
 * @param ctx - Canvas 2D rendering context
 * @param data - Matrix data containing cells, overlay mode, and filter
 * @param width - Canvas CSS pixel width
 * @param height - Canvas CSS pixel height
 * @param hoveredCell - Currently hovered cell coordinates, or null
 * @param selectedCell - Currently selected cell coordinates, or null
 * @param dpr - Device pixel ratio for HiDPI rendering
 */
export function renderMatrix(
  ctx: CanvasRenderingContext2D,
  data: MatrixData,
  width: number,
  height: number,
  hoveredCell: { row: number; col: number } | null,
  selectedCell: { row: number; col: number } | null,
  dpr: number,
): void {
  // Scale context for HiDPI
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

  // Clear canvas
  ctx.fillStyle = theme.colors.bg;
  ctx.fillRect(0, 0, width, height);

  const cellW = width / GRID_SIZE;
  const cellH = height / GRID_SIZE;

  for (let row = 0; row < GRID_SIZE; row++) {
    for (let col = 0; col < GRID_SIZE; col++) {
      const x = col * cellW;
      const y = row * cellH;

      const cell = data.cells[row]?.[col];
      if (!cell) continue;

      // Cell background
      const bgColor = getCellBackground(cell, data.overlay, data.filter);
      ctx.fillStyle = bgColor;
      ctx.fillRect(x, y, cellW, cellH);

      // Strategy bars
      if (data.overlay === 'strategy') {
        drawStrategyBars(ctx, cell, x, y, cellW, cellH);
      } else {
        drawMetricText(ctx, cell, data.overlay, x, y, cellW, cellH);
      }

      // Cell label
      const label = cell.label || getCellLabel(row, col);
      ctx.fillStyle = theme.colors.text;
      ctx.font = `bold ${Math.max(9, Math.floor(cellH * 0.32))}px Inter, sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(label, x + cellW / 2, y + cellH / 2 - (data.overlay === 'strategy' ? 2 : 4), cellW - 2);

      // Grid lines
      ctx.strokeStyle = theme.colors.border;
      ctx.lineWidth = 0.5;
      ctx.strokeRect(x, y, cellW, cellH);
    }
  }

  // Hover highlight
  if (hoveredCell) {
    const hx = hoveredCell.col * cellW;
    const hy = hoveredCell.row * cellH;
    ctx.strokeStyle = theme.colors.accent;
    ctx.lineWidth = 2;
    ctx.strokeRect(hx + 1, hy + 1, cellW - 2, cellH - 2);
  }

  // Selection highlight
  if (selectedCell) {
    const sx = selectedCell.col * cellW;
    const sy = selectedCell.row * cellH;
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 2;
    ctx.strokeRect(sx + 1, sy + 1, cellW - 2, cellH - 2);
  }

  // Reset transform
  ctx.setTransform(1, 0, 0, 1, 0, 0);
}

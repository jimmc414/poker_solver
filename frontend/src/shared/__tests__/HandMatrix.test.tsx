import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { HandMatrix } from '../components/HandMatrix';
import type { MatrixData, MatrixCell } from '../types/matrix';

// Mock canvas getContext
const mockFillRect = vi.fn();
const mockStrokeRect = vi.fn();
const mockFillText = vi.fn();
const mockSetTransform = vi.fn();
const mockScale = vi.fn();

const mockContext: Partial<CanvasRenderingContext2D> = {
  fillRect: mockFillRect,
  strokeRect: mockStrokeRect,
  fillText: mockFillText,
  setTransform: mockSetTransform,
  scale: mockScale,
  fillStyle: '',
  strokeStyle: '',
  lineWidth: 1,
  font: '',
  textAlign: 'center',
  textBaseline: 'middle',
};

beforeEach(() => {
  vi.clearAllMocks();

  // Mock HTMLCanvasElement.getContext
  HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue(mockContext);

  // Mock requestAnimationFrame
  vi.spyOn(window, 'requestAnimationFrame').mockImplementation((cb) => {
    cb(0);
    return 0;
  });
  vi.spyOn(window, 'cancelAnimationFrame').mockImplementation(() => {});
});

/**
 * Creates minimal test matrix data.
 */
function createTestData(): MatrixData {
  const cells: MatrixCell[][] = [];
  for (let row = 0; row < 13; row++) {
    const rowCells: MatrixCell[] = [];
    for (let col = 0; col < 13; col++) {
      rowCells.push({
        row,
        col,
        label: `${row},${col}`,
        combos: row === col ? 6 : col > row ? 4 : 12,
        weight: 50,
        actions: { bet: 0.5, check: 0.3, fold: 0.2 },
        ev: 0.5,
        equity: 0.5,
        eqr: 1.0,
      });
    }
    cells.push(rowCells);
  }
  return { cells, overlay: 'strategy', filter: 'all' };
}

describe('HandMatrix', () => {
  it('renders a canvas element', () => {
    const data = createTestData();
    render(<HandMatrix data={data} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    expect(canvas).toBeInTheDocument();
    expect(canvas.tagName).toBe('CANVAS');
  });

  it('calls getContext on mount', () => {
    const data = createTestData();
    render(<HandMatrix data={data} />);

    expect(HTMLCanvasElement.prototype.getContext).toHaveBeenCalledWith('2d');
  });

  it('uses requestAnimationFrame for rendering', () => {
    const data = createTestData();
    render(<HandMatrix data={data} />);

    expect(window.requestAnimationFrame).toHaveBeenCalled();
  });

  it('calls onCellClick when canvas is clicked', () => {
    const data = createTestData();
    const onClick = vi.fn();
    render(<HandMatrix data={data} onCellClick={onClick} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');

    // Mock getBoundingClientRect
    vi.spyOn(canvas, 'getBoundingClientRect').mockReturnValue({
      left: 0,
      top: 0,
      width: 520,
      height: 520,
      right: 520,
      bottom: 520,
      x: 0,
      y: 0,
      toJSON: () => {},
    });

    // Click at the center of cell (0, 0) = top-left cell
    fireEvent.click(canvas, { clientX: 10, clientY: 10 });

    expect(onClick).toHaveBeenCalledWith(
      expect.objectContaining({ row: 0, col: 0 }),
    );
  });

  it('calls onCellHover when mouse moves over canvas', () => {
    const data = createTestData();
    const onHover = vi.fn();
    render(<HandMatrix data={data} onCellHover={onHover} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');

    vi.spyOn(canvas, 'getBoundingClientRect').mockReturnValue({
      left: 0,
      top: 0,
      width: 520,
      height: 520,
      right: 520,
      bottom: 520,
      x: 0,
      y: 0,
      toJSON: () => {},
    });

    // Move mouse to cell area
    fireEvent.mouseMove(canvas, { clientX: 10, clientY: 10 });

    expect(onHover).toHaveBeenCalledWith(
      expect.objectContaining({ row: 0, col: 0 }),
    );
  });

  it('calls onCellHover with null when mouse leaves canvas', () => {
    const data = createTestData();
    const onHover = vi.fn();
    render(<HandMatrix data={data} onCellHover={onHover} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    fireEvent.mouseLeave(canvas);

    expect(onHover).toHaveBeenCalledWith(null);
  });

  it('applies custom width and height', () => {
    const data = createTestData();
    render(<HandMatrix data={data} width={400} height={300} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    expect(canvas.style.width).toBe('400px');
    expect(canvas.style.height).toBe('300px');
  });

  it('defaults to 520x520 size', () => {
    const data = createTestData();
    render(<HandMatrix data={data} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    expect(canvas.style.width).toBe('520px');
    expect(canvas.style.height).toBe('520px');
  });

  it('cleans up animation frame on unmount', () => {
    const data = createTestData();
    const { unmount } = render(<HandMatrix data={data} />);

    unmount();

    expect(window.cancelAnimationFrame).toHaveBeenCalled();
  });

  it('sets cursor to pointer when interactive', () => {
    const data = createTestData();
    render(<HandMatrix data={data} interactive={true} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    expect(canvas.style.cursor).toBe('pointer');
  });

  it('sets cursor to default when not interactive', () => {
    const data = createTestData();
    render(<HandMatrix data={data} interactive={false} />);

    const canvas = screen.getByTestId('hand-matrix-canvas');
    expect(canvas.style.cursor).toBe('default');
  });
});

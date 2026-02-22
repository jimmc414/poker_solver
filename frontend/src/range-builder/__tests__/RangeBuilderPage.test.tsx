import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen, fireEvent, within } from '@testing-library/react';
import { RangeBuilderPage } from '../RangeBuilderPage';

// Mock canvas context since jsdom does not support canvas rendering
beforeEach(() => {
  HTMLCanvasElement.prototype.getContext = (() => {
    return {
      fillRect: () => undefined,
      strokeRect: () => undefined,
      fillText: () => undefined,
      setTransform: () => undefined,
      scale: () => undefined,
      clearRect: () => undefined,
      beginPath: () => undefined,
      moveTo: () => undefined,
      lineTo: () => undefined,
      stroke: () => undefined,
      fill: () => undefined,
      arc: () => undefined,
      save: () => undefined,
      restore: () => undefined,
      measureText: () => ({ width: 10 }),
      font: '',
      fillStyle: '',
      strokeStyle: '',
      lineWidth: 1,
      textAlign: '',
      textBaseline: '',
      globalAlpha: 1,
    };
  }) as unknown as typeof HTMLCanvasElement.prototype.getContext;

  // Mock requestAnimationFrame
  globalThis.requestAnimationFrame = ((cb: FrameRequestCallback) => {
    cb(0);
    return 0;
  }) as typeof globalThis.requestAnimationFrame;

  globalThis.cancelAnimationFrame = (() => undefined) as typeof globalThis.cancelAnimationFrame;
});

describe('RangeBuilderPage', () => {
  it('renders the page with all major sections', () => {
    render(<RangeBuilderPage />);

    expect(screen.getByTestId('range-builder-page')).toBeInTheDocument();
    expect(screen.getByTestId('range-grid')).toBeInTheDocument();
    expect(screen.getByTestId('paintbrush-tools')).toBeInTheDocument();
    expect(screen.getByTestId('weight-controls')).toBeInTheDocument();
    expect(screen.getByTestId('preset-selector')).toBeInTheDocument();
    expect(screen.getByTestId('suit-expander')).toBeInTheDocument();
    expect(screen.getByTestId('combo-lock')).toBeInTheDocument();
    expect(screen.getByTestId('range-summary')).toBeInTheDocument();
    expect(screen.getByTestId('range-color-legend')).toBeInTheDocument();
  });

  it('changes paint mode when mode buttons are clicked', () => {
    render(<RangeBuilderPage />);

    const selectButton = screen.getByTestId('mode-select');
    const paintButton = screen.getByTestId('mode-paint');
    const eraseButton = screen.getByTestId('mode-erase');

    // Default is paint mode
    expect(paintButton).toHaveStyle({ fontWeight: 600 });

    // Switch to select mode
    fireEvent.click(selectButton);
    expect(selectButton).toHaveStyle({ fontWeight: 600 });

    // Switch to erase mode
    fireEvent.click(eraseButton);
    expect(eraseButton).toHaveStyle({ fontWeight: 600 });
  });

  it('changes weight when slider is adjusted', () => {
    render(<RangeBuilderPage />);

    const slider = screen.getByTestId('weight-slider');
    fireEvent.change(slider, { target: { value: '50' } });

    // Weight display inside the weight-controls section should show 50%
    const weightControls = screen.getByTestId('weight-controls');
    expect(within(weightControls).getByText('50%')).toBeInTheDocument();
  });

  it('renders range summary with zero combos initially', () => {
    render(<RangeBuilderPage />);

    // All weights start at 0, so combo count should be 0
    expect(screen.getByText('0 / 169')).toBeInTheDocument();
    expect(screen.getByText('0.0')).toBeInTheDocument();
    expect(screen.getByText('0.0%')).toBeInTheDocument();
  });

  it('renders suit expander with placeholder text when no cell selected', () => {
    render(<RangeBuilderPage />);

    expect(screen.getByText('Select a hand to see suit combinations')).toBeInTheDocument();
  });

  it('renders preset selector with dropdown', () => {
    render(<RangeBuilderPage />);

    expect(screen.getByTestId('preset-dropdown')).toBeInTheDocument();
    expect(screen.getByText('Select preset...')).toBeInTheDocument();
  });

  it('lock button is disabled when no cells are selected', () => {
    render(<RangeBuilderPage />);

    const lockButton = screen.getByTestId('lock-toggle');
    expect(lockButton).toBeDisabled();
  });

  it('renders color legend with weight stops', () => {
    render(<RangeBuilderPage />);

    const legend = screen.getByTestId('range-color-legend');
    expect(within(legend).getByText('Weight Legend')).toBeInTheDocument();
    expect(within(legend).getByText('0%')).toBeInTheDocument();
    expect(within(legend).getByText('25%')).toBeInTheDocument();
    expect(within(legend).getByText('75%')).toBeInTheDocument();
    expect(within(legend).getByText('100%')).toBeInTheDocument();
  });
});

import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import App from '../App';

// Mock canvas context since jsdom doesn't support canvas rendering
beforeEach(() => {
  HTMLCanvasElement.prototype.getContext = (() => ({
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
  })) as unknown as typeof HTMLCanvasElement.prototype.getContext;

  globalThis.requestAnimationFrame = ((cb: FrameRequestCallback) => {
    cb(0);
    return 0;
  }) as typeof globalThis.requestAnimationFrame;

  globalThis.cancelAnimationFrame = (() => undefined) as typeof globalThis.cancelAnimationFrame;
});

describe('App', () => {
  it('renders the app layout with navigation and content', () => {
    render(<App />);

    expect(screen.getByTestId('app-layout')).toBeInTheDocument();
    expect(screen.getByTestId('navigation')).toBeInTheDocument();
    expect(screen.getByTestId('app-content')).toBeInTheDocument();
  });

  it('starts in study mode with coming soon placeholder', () => {
    render(<App />);

    expect(screen.getByTestId('nav-study')).toBeInTheDocument();
    expect(screen.getByTestId('coming-soon')).toBeInTheDocument();
    expect(screen.getByText('Coming Soon')).toBeInTheDocument();
  });

  it('switches to range builder when nav item is clicked', () => {
    render(<App />);

    fireEvent.click(screen.getByTestId('nav-range-builder'));

    expect(screen.getByTestId('range-builder-page')).toBeInTheDocument();
    expect(screen.queryByTestId('coming-soon')).not.toBeInTheDocument();
  });

  it('switches modes via navigation clicks', () => {
    render(<App />);

    fireEvent.click(screen.getByTestId('nav-solve'));
    expect(screen.getByTestId('coming-soon')).toBeInTheDocument();

    fireEvent.click(screen.getByTestId('nav-practice'));
    expect(screen.getByTestId('coming-soon')).toBeInTheDocument();
  });

  it('switches modes via keyboard shortcuts', () => {
    render(<App />);

    // Press 's' to switch to solve mode
    fireEvent.keyDown(window, { key: 's' });
    // Solve nav should be active
    const solveNav = screen.getByTestId('nav-solve');
    expect(solveNav.style.borderLeft).toContain('3px solid');

    // Press 'j' to switch to study mode
    fireEvent.keyDown(window, { key: 'j' });
    const studyNav = screen.getByTestId('nav-study');
    expect(studyNav.style.borderLeft).toContain('3px solid');

    // Press 'p' to switch to practice mode
    fireEvent.keyDown(window, { key: 'p' });
    const practiceNav = screen.getByTestId('nav-practice');
    expect(practiceNav.style.borderLeft).toContain('3px solid');

    // Press 'q' to switch to analyze mode
    fireEvent.keyDown(window, { key: 'q' });
    const analyzeNav = screen.getByTestId('nav-analyze');
    expect(analyzeNav.style.borderLeft).toContain('3px solid');
  });

  it('shows range builder page when navigated to via click', () => {
    render(<App />);

    fireEvent.click(screen.getByTestId('nav-range-builder'));
    expect(screen.getByTestId('range-builder-page')).toBeInTheDocument();
    expect(screen.getByTestId('range-grid')).toBeInTheDocument();
  });
});

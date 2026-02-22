import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Navigation } from '../components/Navigation';
import type { AppMode } from '../types/common';

describe('Navigation', () => {
  const defaultProps = {
    activeMode: 'study' as AppMode,
    onModeChange: vi.fn(),
  };

  it('renders all navigation entries', () => {
    render(<Navigation {...defaultProps} />);

    expect(screen.getByTestId('nav-study')).toBeInTheDocument();
    expect(screen.getByTestId('nav-solve')).toBeInTheDocument();
    expect(screen.getByTestId('nav-practice')).toBeInTheDocument();
    expect(screen.getByTestId('nav-analyze')).toBeInTheDocument();
    expect(screen.getByTestId('nav-range-builder')).toBeInTheDocument();
    expect(screen.getByTestId('nav-nodelock')).toBeInTheDocument();
    expect(screen.getByTestId('nav-reports')).toBeInTheDocument();
    expect(screen.getByTestId('nav-settings')).toBeInTheDocument();
  });

  it('displays labels for each entry', () => {
    render(<Navigation {...defaultProps} />);

    expect(screen.getByText('Study')).toBeInTheDocument();
    expect(screen.getByText('Solve')).toBeInTheDocument();
    expect(screen.getByText('Practice')).toBeInTheDocument();
    expect(screen.getByText('Analyze')).toBeInTheDocument();
    expect(screen.getByText('Range Builder')).toBeInTheDocument();
    expect(screen.getByText('Nodelock')).toBeInTheDocument();
    expect(screen.getByText('Reports')).toBeInTheDocument();
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });

  it('shows keyboard shortcuts for modes that have them', () => {
    render(<Navigation {...defaultProps} />);

    expect(screen.getByText('J')).toBeInTheDocument();
    expect(screen.getByText('S')).toBeInTheDocument();
    expect(screen.getByText('P')).toBeInTheDocument();
    expect(screen.getByText('Q')).toBeInTheDocument();
  });

  it('calls onModeChange when a nav item is clicked', () => {
    const onModeChange = vi.fn();
    render(<Navigation {...defaultProps} onModeChange={onModeChange} />);

    fireEvent.click(screen.getByTestId('nav-solve'));
    expect(onModeChange).toHaveBeenCalledWith('solve');

    fireEvent.click(screen.getByTestId('nav-practice'));
    expect(onModeChange).toHaveBeenCalledWith('practice');
  });

  it('highlights the active mode', () => {
    const { rerender } = render(<Navigation {...defaultProps} activeMode="study" />);
    const studyNav = screen.getByTestId('nav-study');
    // Active item should have a non-transparent left border (accent color)
    expect(studyNav.style.borderLeft).toContain(
      // The accent border is set via inline style
      '3px solid'
    );

    rerender(<Navigation {...defaultProps} activeMode="solve" />);
    // After rerender, solve should be active
    const solveNav = screen.getByTestId('nav-solve');
    expect(solveNav.style.borderLeft).toContain('3px solid');
  });

  it('renders navigation wrapper with data-testid', () => {
    render(<Navigation {...defaultProps} />);
    expect(screen.getByTestId('navigation')).toBeInTheDocument();
  });

  it('supports keyboard activation via Enter key', () => {
    const onModeChange = vi.fn();
    render(<Navigation {...defaultProps} onModeChange={onModeChange} />);

    const solveNav = screen.getByTestId('nav-solve');
    fireEvent.keyDown(solveNav, { key: 'Enter' });
    expect(onModeChange).toHaveBeenCalledWith('solve');
  });

  it('supports keyboard activation via Space key', () => {
    const onModeChange = vi.fn();
    render(<Navigation {...defaultProps} onModeChange={onModeChange} />);

    const analyzeNav = screen.getByTestId('nav-analyze');
    fireEvent.keyDown(analyzeNav, { key: ' ' });
    expect(onModeChange).toHaveBeenCalledWith('analyze');
  });
});

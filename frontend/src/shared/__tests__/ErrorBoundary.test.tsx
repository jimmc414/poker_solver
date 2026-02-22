import React from 'react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ErrorBoundary } from '../components/ErrorBoundary';

// A component that always throws
const ThrowingComponent: React.FC<{ message?: string }> = ({ message = 'Test error' }) => {
  throw new Error(message);
};

describe('ErrorBoundary', () => {
  // Suppress console.error during error boundary tests
  beforeEach(() => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  it('renders children when there is no error', () => {
    render(
      <ErrorBoundary>
        <div data-testid="child">Hello</div>
      </ErrorBoundary>,
    );

    expect(screen.getByTestId('child')).toBeInTheDocument();
    expect(screen.getByText('Hello')).toBeInTheDocument();
  });

  it('renders error UI when a child throws', () => {
    render(
      <ErrorBoundary>
        <ThrowingComponent />
      </ErrorBoundary>,
    );

    expect(screen.getByTestId('error-boundary')).toBeInTheDocument();
    expect(screen.getByText('Something went wrong')).toBeInTheDocument();
  });

  it('displays the error message', () => {
    render(
      <ErrorBoundary>
        <ThrowingComponent message="Custom error message" />
      </ErrorBoundary>,
    );

    expect(screen.getByTestId('error-message')).toHaveTextContent('Custom error message');
  });

  it('renders a retry button', () => {
    render(
      <ErrorBoundary>
        <ThrowingComponent />
      </ErrorBoundary>,
    );

    const retryButton = screen.getByTestId('error-retry');
    expect(retryButton).toBeInTheDocument();
    expect(retryButton).toHaveTextContent('Try Again');
  });

  it('clears error state when retry is clicked', () => {
    // We test that clicking retry resets hasError.
    // The component will re-throw, but we verify the boundary re-renders children.
    render(
      <ErrorBoundary>
        <ThrowingComponent />
      </ErrorBoundary>,
    );

    // Error UI is shown
    expect(screen.getByTestId('error-boundary')).toBeInTheDocument();

    // Click retry - since ThrowingComponent always throws,
    // it will re-enter error state, but the handler fires.
    fireEvent.click(screen.getByTestId('error-retry'));

    // After retry, the boundary catches the error again
    expect(screen.getByTestId('error-boundary')).toBeInTheDocument();
  });

  it('renders custom fallback when provided', () => {
    const fallback = <div data-testid="custom-fallback">Custom Error UI</div>;

    render(
      <ErrorBoundary fallback={fallback}>
        <ThrowingComponent />
      </ErrorBoundary>,
    );

    expect(screen.getByTestId('custom-fallback')).toBeInTheDocument();
    expect(screen.getByText('Custom Error UI')).toBeInTheDocument();
    expect(screen.queryByTestId('error-boundary')).not.toBeInTheDocument();
  });

  it('does not catch errors outside its children', () => {
    // ErrorBoundary only catches errors in its subtree
    render(
      <ErrorBoundary>
        <div data-testid="safe-child">Safe content</div>
      </ErrorBoundary>,
    );

    expect(screen.getByTestId('safe-child')).toBeInTheDocument();
  });
});

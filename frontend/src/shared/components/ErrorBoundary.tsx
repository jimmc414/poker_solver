import React from 'react';
import { theme } from '../styles/theme';

interface ErrorBoundaryProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

/**
 * React error boundary that catches rendering errors and displays
 * a fallback UI with error details and a retry button.
 */
export class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  handleRetry = (): void => {
    this.setState({ hasError: false, error: null });
  };

  render(): React.ReactNode {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      const containerStyle: React.CSSProperties = {
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        padding: theme.spacing.xl,
        gap: theme.spacing.md,
        color: theme.colors.text,
        minHeight: 200,
      };

      const titleStyle: React.CSSProperties = {
        fontSize: theme.fontSize.lg,
        fontWeight: 600,
        color: theme.colors.error,
      };

      const messageStyle: React.CSSProperties = {
        fontSize: theme.fontSize.sm,
        color: theme.colors.textSecondary,
        textAlign: 'center',
        maxWidth: 400,
        wordBreak: 'break-word',
      };

      const buttonStyle: React.CSSProperties = {
        padding: `${theme.spacing.sm}px ${theme.spacing.md}px`,
        backgroundColor: theme.colors.accent,
        color: '#ffffff',
        border: 'none',
        borderRadius: theme.borderRadius.sm,
        fontSize: theme.fontSize.md,
        cursor: 'pointer',
        fontWeight: 500,
      };

      return (
        <div style={containerStyle} data-testid="error-boundary">
          <div style={titleStyle}>Something went wrong</div>
          <div style={messageStyle} data-testid="error-message">
            {this.state.error?.message ?? 'An unexpected error occurred'}
          </div>
          <button style={buttonStyle} onClick={this.handleRetry} data-testid="error-retry">
            Try Again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

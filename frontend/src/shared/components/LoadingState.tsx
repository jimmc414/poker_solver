import React from 'react';
import type { LoadingState as LoadingStateType } from '../types/common';
import { theme } from '../styles/theme';

interface LoadingStateProps {
  state: LoadingStateType;
}

/**
 * Loading indicator with spinner and optional progress bar.
 * Only renders when state.isLoading is true.
 */
export const LoadingIndicator: React.FC<LoadingStateProps> = ({ state }) => {
  if (!state.isLoading) {
    return null;
  }

  const containerStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    gap: theme.spacing.sm,
    padding: theme.spacing.lg,
  };

  const spinnerStyle: React.CSSProperties = {
    width: 32,
    height: 32,
    border: `3px solid ${theme.colors.border}`,
    borderTopColor: theme.colors.accent,
    borderRadius: '50%',
    animation: 'spin 0.8s linear infinite',
  };

  const messageStyle: React.CSSProperties = {
    color: theme.colors.textSecondary,
    fontSize: theme.fontSize.sm,
  };

  const progressBarContainerStyle: React.CSSProperties = {
    width: '100%',
    maxWidth: 240,
    height: 4,
    backgroundColor: theme.colors.bgTertiary,
    borderRadius: theme.borderRadius.sm,
    overflow: 'hidden',
  };

  const progressBarFillStyle: React.CSSProperties = {
    height: '100%',
    backgroundColor: theme.colors.accent,
    borderRadius: theme.borderRadius.sm,
    width: `${(state.progress ?? 0) * 100}%`,
    transition: 'width 0.3s ease',
  };

  return (
    <div style={containerStyle} data-testid="loading-indicator">
      <style>{`@keyframes spin { to { transform: rotate(360deg); } }`}</style>
      <div style={spinnerStyle} data-testid="loading-spinner" />
      {state.message && <div style={messageStyle}>{state.message}</div>}
      {state.progress !== undefined && (
        <div style={progressBarContainerStyle}>
          <div style={progressBarFillStyle} data-testid="loading-progress" />
        </div>
      )}
    </div>
  );
};

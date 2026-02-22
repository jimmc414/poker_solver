import React from 'react';
import { AppProvider, useAppContext } from './shared/context/AppContext';
import { Navigation } from './shared/components/Navigation';
import { ErrorBoundary } from './shared/components/ErrorBoundary';
import { useKeyboardShortcuts } from './shared/hooks/useKeyboardShortcuts';
import { RangeBuilderPage } from './range-builder/RangeBuilderPage';
import { theme } from './shared/styles/theme';
import type { AppMode } from './shared/types/common';

const ComingSoon: React.FC<{ mode: string }> = ({ mode }) => (
  <div
    data-testid="coming-soon"
    style={{
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      height: '100%',
      color: theme.colors.textSecondary,
      gap: theme.spacing.md,
    }}
  >
    <div style={{ fontSize: theme.fontSize.xxl, fontWeight: 600, color: theme.colors.text }}>
      {mode.charAt(0).toUpperCase() + mode.slice(1).replace('-', ' ')}
    </div>
    <div>Coming Soon</div>
  </div>
);

const AppContent: React.FC = () => {
  const { state, dispatch } = useAppContext();

  const handleModeChange = (mode: AppMode) => {
    dispatch({ type: 'SET_MODE', payload: mode });
  };

  useKeyboardShortcuts({
    j: () => handleModeChange('study'),
    s: () => handleModeChange('solve'),
    p: () => handleModeChange('practice'),
    q: () => handleModeChange('analyze'),
  });

  const renderContent = () => {
    switch (state.activeMode) {
      case 'range-builder':
        return <RangeBuilderPage />;
      default:
        return <ComingSoon mode={state.activeMode} />;
    }
  };

  return (
    <div data-testid="app-layout" style={{ display: 'flex', width: '100%', height: '100%' }}>
      <Navigation activeMode={state.activeMode} onModeChange={handleModeChange} />
      <main
        data-testid="app-content"
        style={{ flex: 1, overflow: 'auto', backgroundColor: theme.colors.bg }}
      >
        <ErrorBoundary>
          {renderContent()}
        </ErrorBoundary>
      </main>
    </div>
  );
};

const App: React.FC = () => {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  );
};

export default App;

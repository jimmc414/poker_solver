import React, { createContext, useContext, useReducer } from 'react';
import type { AppMode, LoadingState } from '../types/common';
import type { Theme } from '../styles/theme';
import { theme as defaultTheme } from '../styles/theme';

// --- State ---

interface AppState {
  activeMode: AppMode;
  loading: LoadingState;
  theme: Theme;
}

const initialState: AppState = {
  activeMode: 'study',
  loading: { isLoading: false },
  theme: defaultTheme,
};

// --- Actions ---

type AppAction =
  | { type: 'SET_MODE'; payload: AppMode }
  | { type: 'SET_LOADING'; payload: { message?: string; progress?: number } }
  | { type: 'CLEAR_LOADING' };

// --- Reducer ---

function appReducer(state: AppState, action: AppAction): AppState {
  switch (action.type) {
    case 'SET_MODE':
      return { ...state, activeMode: action.payload };
    case 'SET_LOADING':
      return {
        ...state,
        loading: {
          isLoading: true,
          message: action.payload.message,
          progress: action.payload.progress,
        },
      };
    case 'CLEAR_LOADING':
      return { ...state, loading: { isLoading: false } };
    default:
      return state;
  }
}

// --- Context ---

interface AppContextValue {
  state: AppState;
  dispatch: React.Dispatch<AppAction>;
}

const AppContext = createContext<AppContextValue | null>(null);

// --- Provider ---

interface AppProviderProps {
  children: React.ReactNode;
}

export const AppProvider: React.FC<AppProviderProps> = ({ children }) => {
  const [state, dispatch] = useReducer(appReducer, initialState);

  return (
    <AppContext.Provider value={{ state, dispatch }}>
      {children}
    </AppContext.Provider>
  );
};

// --- Hook ---

export function useAppContext(): AppContextValue {
  const context = useContext(AppContext);
  if (context === null) {
    throw new Error('useAppContext must be used within an AppProvider');
  }
  return context;
}

export type { AppState, AppAction };

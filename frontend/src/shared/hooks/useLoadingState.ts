import { useCallback, useState } from 'react';
import type { LoadingState } from '../types/common';

/**
 * Custom hook for managing loading state with optional message and progress.
 *
 * @returns Tuple of [state, setLoading, clearLoading]
 */
export function useLoadingState(): [
  LoadingState,
  (message?: string, progress?: number) => void,
  () => void,
] {
  const [loading, setLoadingState] = useState<LoadingState>({ isLoading: false });

  const setLoading = useCallback((message?: string, progress?: number) => {
    setLoadingState({ isLoading: true, message, progress });
  }, []);

  const clearLoading = useCallback(() => {
    setLoadingState({ isLoading: false });
  }, []);

  return [loading, setLoading, clearLoading];
}

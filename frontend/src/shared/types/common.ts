/**
 * Common application-level types used across all frontend modules.
 */

export type AppMode =
  | 'study'
  | 'practice'
  | 'analyze'
  | 'solve'
  | 'range-builder'
  | 'nodelock'
  | 'reports'
  | 'settings';

export interface LoadingState {
  isLoading: boolean;
  message?: string;
  progress?: number;
}

export interface AppError {
  code: string;
  message: string;
}

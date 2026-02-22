import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * Typed wrapper around Tauri's invoke function.
 * All backend commands flow through this single entry point.
 */
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return tauriInvoke<T>(command, args);
}

import { describe, it, expect, vi } from 'vitest';
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { invoke } from '../invoke';

// The mock is set up in test-setup.ts

describe('invoke wrapper', () => {
  it('calls @tauri-apps/api invoke with command and args', async () => {
    const mockResult = { rank: { value: 1000, category: 'Pair' } };
    vi.mocked(tauriInvoke).mockResolvedValueOnce(mockResult);

    const result = await invoke('evaluate_hand', { cards: [] });

    expect(tauriInvoke).toHaveBeenCalledWith('evaluate_hand', { cards: [] });
    expect(result).toEqual(mockResult);
  });

  it('calls invoke without args when none provided', async () => {
    vi.mocked(tauriInvoke).mockResolvedValueOnce('test-dir');

    const result = await invoke<string>('get_data_dir');

    expect(tauriInvoke).toHaveBeenCalledWith('get_data_dir', undefined);
    expect(result).toBe('test-dir');
  });

  it('propagates errors from Tauri invoke', async () => {
    vi.mocked(tauriInvoke).mockRejectedValueOnce(new Error('Backend error'));

    await expect(invoke('failing_command')).rejects.toThrow('Backend error');
  });

  it('returns typed results', async () => {
    interface TestResult {
      equity: number;
    }

    vi.mocked(tauriInvoke).mockResolvedValueOnce({ equity: 0.55 });

    const result = await invoke<TestResult>('compute_equity', {
      range1: [100],
      range2: [100],
      board: [],
    });

    expect(result.equity).toBe(0.55);
  });
});

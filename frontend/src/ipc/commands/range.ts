import { invoke } from '../invoke';
import type { RangePreset, EquityCalcResponse } from '../types/range';

export async function computeEquity(
  range1: number[],
  range2: number[],
  board: string[],
): Promise<EquityCalcResponse> {
  return invoke<EquityCalcResponse>('compute_equity', { range1, range2, board });
}

export async function loadPreset(name: string): Promise<RangePreset> {
  return invoke<RangePreset>('load_preset', { name });
}

export async function savePreset(name: string, weights: number[]): Promise<void> {
  return invoke<void>('save_preset', { name, weights });
}

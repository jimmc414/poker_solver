import { invoke } from '../invoke';
import type { EvalResponse, EquityResponse } from '../types/eval';
import type { Card } from '../../shared/types/poker';

export async function evaluateHand(cards: Card[]): Promise<EvalResponse> {
  return invoke<EvalResponse>('evaluate_hand', { cards });
}

export async function equityCalculation(
  hand1: [Card, Card],
  hand2: [Card, Card],
  board: Card[],
): Promise<EquityResponse> {
  return invoke<EquityResponse>('equity_calculation', { hand1, hand2, board });
}

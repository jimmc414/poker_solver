import type { Card, HandRank } from '../../shared/types/poker';

export interface EvalRequest {
  cards: Card[];
}

export interface EvalResponse {
  rank: HandRank;
}

export interface EquityRequest {
  hand1: [Card, Card];
  hand2: [Card, Card];
  board: Card[];
}

export interface EquityResponse {
  equity1: number;
  equity2: number;
}

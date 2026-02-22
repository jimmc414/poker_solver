/**
 * TypeScript mirrors of Rust poker-core types.
 * These must stay in sync with the Rust serde representations.
 */

export type Rank = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A';

export type Suit = 'c' | 'd' | 'h' | 's';

export interface Card {
  rank: Rank;
  suit: Suit;
}

export interface Hand {
  cards: [Card, Card];
}

export type ActionType = 'fold' | 'check' | 'call' | 'bet' | 'raise' | 'allin';

export interface PokerAction {
  type: ActionType;
  amount?: number;
}

export type Position = 'UTG' | 'UTG1' | 'UTG2' | 'LJ' | 'MP' | 'CO' | 'BTN' | 'SB' | 'BB';

export type GameType = 'CashNLH' | 'MTT' | 'SNG' | 'SpinAndGo' | 'HeadsUp';

export type BetSize =
  | { type: 'pot_fraction'; value: number }
  | { type: 'absolute'; value: number }
  | { type: 'allin' };

export type HandCategory =
  | 'StraightFlush'
  | 'FourOfAKind'
  | 'FullHouse'
  | 'Flush'
  | 'Straight'
  | 'ThreeOfAKind'
  | 'TwoPair'
  | 'OnePair'
  | 'HighCard';

export interface HandRank {
  value: number;
  category: HandCategory;
}

export const RANKS: Rank[] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

export const SUITS: Suit[] = ['s', 'h', 'd', 'c'];

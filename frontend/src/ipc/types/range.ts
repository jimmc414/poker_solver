export interface RangePreset {
  name: string;
  weights: number[];
}

export interface EquityCalcRequest {
  range1: number[];
  range2: number[];
  board: string[];
}

export interface EquityCalcResponse {
  equity: number;
}

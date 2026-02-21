# Agent Implementation Prompt

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 7 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisites:** All previous build documents (1–6)

---

## 1. Preamble

### Your Role

You are an AI implementation agent building a GTO Poker Solver desktop application. You work milestone by milestone, following the specification established in 6 build documents. You write production-quality code, run tests after each logical unit, and never skip ahead.

### Ground Rules

1. **Read before writing.** Before implementing any component, read the relevant sections of the build documents listed in the reference table below.
2. **Follow the DAG.** Never import from a crate or module that hasn't been built yet. The crate dependency order is: `poker-core` → `poker-eval` → `poker-solver` / `poker-solution` / `poker-icm` / `poker-hhparser` → `poker-analyze` → `app-tauri`.
3. **Test after each unit.** Run `cargo test` after completing each Rust module. Run `npm test` after completing each frontend component. Fix failures before moving on.
4. **Use poker-core types everywhere.** All cross-crate interfaces use `Card`, `Hand`, `Board`, `Action`, `Position`, `GameType`, `BetSize` from `poker-core`.
5. **Respect prohibited patterns.** No `unwrap()` in library crates, no blocking on Tokio, no `HashMap` in solver hot path, no cross-module frontend imports. See `technical-constraints.md` §4.
6. **Match the file structure.** Create files in the exact locations specified in `project-structure.md`. Don't create files listed under later milestones.
7. **All IPC commands return `Result<T, AppError>`.** No untyped responses, no raw strings.
8. **Commit at logical boundaries.** Each milestone completion is a commit. Mid-milestone commits at stable points (e.g., after a crate passes all tests).

### Reference Table

| Document | Use When |
|----------|----------|
| `requirements.md` | Checking exact requirement text and acceptance criteria |
| `architecture.md` | Crate/module APIs, IPC commands, data flows, storage schema |
| `milestones.md` | What to build per milestone, acceptance criteria, dependency order |
| `project-structure.md` | File paths, API boundaries, "DO NOT CREATE" lists |
| `technical-constraints.md` | Performance targets, prohibited patterns, gotchas |
| `test-strategy.md` | Test plan per crate/module, benchmark specs, fixture requirements |

---

## 2. Global Rules

### 2.1 Error Handling

```rust
// Library crates: use ? operator with typed errors
pub fn evaluate(&self, cards: &[Card; 7]) -> Result<HandRank, EvalError> {
    let index = self.lookup_index(cards)?;  // ? instead of unwrap()
    Ok(self.table[index])
}

// app-tauri commands: return Result<T, AppError>
#[tauri::command]
async fn load_solution(query: SolutionQuery, state: State<'_, AppState>) -> Result<StrategyData, AppError> {
    // ...
}
```

### 2.2 Logging

```rust
// Use tracing, not println
use tracing::{info, warn, error, debug};

info!(iterations = %count, nash_distance = %nd, "Solver progress");
warn!(file = %path.display(), "Skipping malformed hand");
error!(error = %e, "Failed to load solution");
```

### 2.3 Async/Blocking Boundary

```rust
// CPU-bound work: spawn_blocking + rayon
#[tauri::command]
async fn start_solve(config: StartSolveRequest, state: State<'_, AppState>) -> Result<SolveResult, AppError> {
    let result = tokio::task::spawn_blocking(move || {
        // Runs on rayon thread pool, not Tokio
        solve(&config, progress_tx, cancel_token)
    }).await.map_err(|e| AppError::SolverError(e.to_string()))?;
    Ok(result)
}
```

### 2.4 Frontend State Pattern

```typescript
// Per-mode state with useReducer
type StudyAction =
  | { type: 'SET_SPOT'; spot: SpotConfig }
  | { type: 'SET_BOARD'; board: Board }
  | { type: 'SET_TAB'; tab: StudyTab }
  | { type: 'SET_NODE'; node: GameNode }
  | { type: 'SET_OVERLAY'; overlay: MetricOverlay };

function studyReducer(state: StudyState, action: StudyAction): StudyState {
  // Pure function — no side effects
}

function StudyPage() {
  const [state, dispatch] = useReducer(studyReducer, initialState);
  // ...
}
```

### 2.5 IPC Pattern

```typescript
// Typed command wrapper in ipc/commands/
import { invoke } from '@tauri-apps/api/core';
import type { StartSolveRequest, SolveResult } from '../types/solver';

export async function startSolve(request: StartSolveRequest): Promise<SolveResult> {
  return invoke<SolveResult>('start_solve', { request });
}

// Event listener in ipc/hooks/
import { listen } from '@tauri-apps/api/event';
import type { SolveProgress } from '../types/solver';

export function useSolveProgress(onProgress: (p: SolveProgress) => void) {
  useEffect(() => {
    const unlisten = listen<SolveProgress>('solve_progress', (event) => {
      onProgress(event.payload);
    });
    return () => { unlisten.then(fn => fn()); };
  }, [onProgress]);
}
```

---

## 3. M1 Execution Guide: Foundation & Hand Matrix

### Overview

Build the foundation: `poker-core`, `poker-eval`, Tauri app shell, shared components, range builder, and IPC layer. At the end: a running desktop app with an interactive 13x13 hand matrix and range builder.

**Requirements covered:** 45 MUST (DAT-001–004, HEV-001–007, HEV-009, HMX-001–009, RNG-001–007, RNG-010, UIF-001–009, DSK-001–006, PRF-001)

### Step-by-Step Build Order

#### Step 0: Test Fixture Generation

Before implementing any logic, create the test fixture data:
- `tests/fixtures/eval_reference.json`: 1,000 known hands with correct evaluator ranks (generated from a reference evaluator or verified by hand for edge cases).
- `tests/fixtures/sample_hands.json`: 50 sample hole card + board combinations covering all hand categories (high card through royal flush).
- `tests/fixtures/equity_reference.json`: 100 matchups with pre-computed equity values (verified against an existing equity calculator).
- These fixtures are committed to the repo and used by all milestone test suites.

#### Step 1: Workspace Setup

1. Create the Cargo workspace root `Cargo.toml` with `crates/poker-core` and `crates/poker-eval` as members.
2. Create `frontend/package.json` with React 18, TypeScript, Vite, Vitest dependencies.
3. Create `.gitignore` covering `target/`, `node_modules/`, `dist/`, `*.sol` files.
4. Create `fixtures/` directory structure: `fixtures/eval/`, `fixtures/hands/`, `fixtures/solutions/`.

**Test checkpoint:** `cargo build` succeeds for empty workspace. `npm install` succeeds.

#### Step 2: poker-core

Build in this order:
1. `card.rs` — `Rank` (A–2), `Suit` (Spade, Heart, Diamond, Club), `Card` (packed `u8`). Implement `Display`, `From<u8>`, comparison traits.
2. `hand.rs` — `Hand` (two `Card`s), `hand_notation()` (returns "AKs", "QJo", "TT"), `is_suited()`, `canonical_hand_index()` (0..168).
3. `deck.rs` — `Deck` with Fisher-Yates shuffle, deterministic seed support.
4. `board.rs` — `Board` holding 0–5 cards (preflop through river).
5. `action.rs` — `Action` enum: Fold, Check, Call, Bet(amount), Raise(amount), AllIn.
6. `position.rs` — `Position` enum with 6-max and 9-max labels; HU aliases.
7. `game_type.rs` — `GameType` enum.
8. `bet_size.rs` — `BetSize` enum: PotFraction(f64), Absolute(f64), AllIn.
9. `range.rs` — `Range` type: 169-element weight array (f32), with indexing by canonical hand.
10. `error.rs` — `CoreError` enum.
11. `lib.rs` — Re-export everything.

**Test checkpoint:** `cargo test -p poker-core` — all tests in `test-strategy.md` §3.1.

#### Step 3: poker-eval

**Evaluator lookup table design:** Use a two-stage lookup approach:
1. **Stage 1 (5 cards):** Perfect hash or offset table mapping 5-card combinations to hand ranks.
2. **Stage 2 (+2 cards):** Extend to 7-card evaluation by iterating over 21 possible 5-card sub-hands (C(7,5) = 21) and taking the best rank. Alternatively, use a direct 7-card lookup table (~130 MB) for maximum throughput.

Target: >= 200M evaluations/sec (PRF-001) on a single core in release mode. The two-stage approach typically achieves 250-350M eval/s; the direct 7-card table can exceed 500M eval/s at the cost of memory.

Build in this order:
1. `hand_rank.rs` — `HandRank` (u16), `HandCategory` enum (HighCard through StraightFlush).
2. `table_gen.rs` — Lookup table generation algorithm. Generate at first run, save to `eval_tables.bin`.
3. `lookup_table.rs` — `LookupTableEvaluator` implementing `HandEvaluator` trait. Load tables from file or generate.
4. `lib.rs` — Define `HandEvaluator` trait: `evaluate()`, `evaluate_5()`, `categorize()`, `compare()`.
5. `equity.rs` — `equity()` and `equity_batch()` — enumerate/sample remaining cards.
6. `draws.rs` — `detect_draws()` — `DrawType` enum (FlushDraw, OESD, Gutshot, ComboDraw, BackdoorFlush, BackdoorStraight).
7. `blockers.rs` — `blockers()` — `BlockerInfo` showing which opponent combos are blocked.
8. `isomorphism.rs` — `suit_isomorphism()` mapping 22,100 flops → 1,755 canonical forms.

**Test checkpoint:** `cargo test -p poker-eval` — all tests in `test-strategy.md` §3.2.

**Benchmark checkpoint:** `cargo bench -p poker-eval` — evaluator throughput >= 200M evals/sec. If under target:
- Verify release mode (`--release`)
- Check lookup table is in L3 cache (~10MB)
- Profile with `perf` or `flamegraph`
- Ensure `#[inline]` on `evaluate()` and helper functions

#### Step 4: app-tauri Skeleton

1. Create `crates/app-tauri/` with Tauri v2 scaffold: `Cargo.toml`, `tauri.conf.json`, `build.rs`, `icons/`.
2. `main.rs` — Tauri entry point. Register `eval::` and `range::` and `settings::` commands.
3. `state.rs` — `AppState` struct with DB connection pool and config.
4. `error.rs` — `AppError` enum with `Serialize` derive. Implement `From<CoreError>`, `From<EvalError>`.
5. `setup.rs` — First-run initialization: create `~/.poker-solver/`, subdirectories, `data.db` schema, default `config.toml`, generate eval tables if missing.
6. `db.rs` — SQLite initialization with WAL mode, schema creation, connection pool.
7. `commands/mod.rs` — Command module index.
8. `commands/settings.rs` — `get_config`, `update_config`, `get_data_dir`.
9. `commands/eval.rs` — `evaluate_hand`, `equity_calculation`.
10. `commands/range.rs` — `compute_equity`, `load_preset`, `save_preset`.

> **Note:** Solver and solution IPC commands (`start_solve`, `stop_solve`, `get_solution`, `load_solution`) are implemented in M2, not M1. M1 only implements foundation commands: eval, equity, range, and settings.

**Test checkpoint:** `cargo build -p app-tauri` succeeds. `cargo tauri dev` launches the app with a blank webview.

#### Step 5: Frontend — shared/

Build in this order:
1. `styles/theme.ts` — Dark theme tokens (backgrounds, text, borders, action colors).
2. `styles/global.css` — CSS reset, dark theme application.
3. `context/AppContext.tsx` — Global state provider: active mode, theme, loading.
4. `types/poker.ts` — TypeScript mirrors of `poker-core` types: Card, Hand, Board, Action, Position, GameType, BetSize.
5. `types/matrix.ts` — HandMatrix data types: cell data, overlay types.
6. `types/common.ts` — Shared utilities.
7. `components/ActionColorSystem.ts` — Color mapping: bet→red spectrum, check→green, fold→blue.
8. `components/CardRenderer.tsx` — Card display component with rank/suit symbols.
9. `components/Tooltip.tsx` — Generic tooltip wrapper.
10. `components/LoadingState.tsx` — Loading spinner and progress bar.
11. `components/ErrorBoundary.tsx` — Error display with retry option.
12. `components/Navigation.tsx` — Sidebar with mode entries.
13. `components/KeyboardShortcuts.tsx` — Global shortcut handler.
14. `components/HandMatrix.tsx` — 13x13 Canvas-based hand matrix.
15. `components/HandMatrixCanvas.ts` — Canvas rendering: mixed-strategy coloring, hover, click, action filter, metric overlays, combo counts, frequency threshold.
16. `hooks/useKeyboardShortcuts.ts`, `hooks/useLoadingState.ts`, `hooks/useTheme.ts`.

**Key implementation detail: HandMatrix Canvas rendering**
- Create a Canvas element sized to the component container
- Account for `window.devicePixelRatio` for crisp rendering on HiDPI
- Draw 13x13 grid lines
- For each cell: draw proportional color segments (stacked rectangles) based on action frequencies
- Hover handler: convert mouse coordinates to cell; show tooltip
- Click handler: convert to cell; dispatch selection
- Use `requestAnimationFrame` for smooth updates
- Memoize canvas redraw when data hasn't changed

**Test checkpoint:** `npm test` — all shared/ component tests pass.

#### Step 6: Frontend — ipc/

1. `invoke.ts` — Generic typed invoke wrapper around `@tauri-apps/api/core/invoke`.
2. `types/eval.ts` — EvalRequest, EvalResult.
3. `types/range.ts` — EquityRequest, PresetData.
4. `types/settings.ts` — AppConfig, AppError.
5. `commands/eval.ts` — `evaluateHand()`, `equityCalc()`.
6. `commands/range.ts` — `computeEquity()`, `loadPreset()`, `savePreset()`.
7. `commands/settings.ts` — `getConfig()`, `updateConfig()`.

**Test checkpoint:** IPC round-trip tests — invoke eval command, verify typed response.

#### Step 7: Frontend — range-builder/

1. `types.ts` — Range-specific types (grid state, weight map, lock set).
2. `hooks/useRangeState.ts` — useReducer managing 169-cell state.
3. `components/RangeGrid.tsx` — 13x13 grid overlaying shared HandMatrix with selection state.
4. `components/PaintbrushTools.tsx` — Click/drag painting modes.
5. `components/WeightControls.tsx` — 0–100% slider.
6. `components/SuitExpander.tsx` — Expand cell to show 4/6/12 suit combos.
7. `components/PresetSelector.tsx` — Dropdown with preset ranges (UTG open, BTN open, etc.).
8. `components/ComboLock.tsx` — Lock/unlock toggle per combo.
9. `components/RangeSummary.tsx` — Combo count, %, equity display.
10. `components/RangeColorLegend.tsx` — Color coding legend.
11. `RangeBuilderPage.tsx` — Page root assembling all components.

**Test checkpoint:** `npm test` — all range-builder/ tests pass.

#### Step 8: App.tsx and Integration

1. Wire `App.tsx` with React Router (or tab-based navigation) for mode switching.
2. Configure Navigation component with entries for all modes (Study, Practice, Analyze, Solve, Range Builder, Settings). Non-implemented modes show "Coming Soon" placeholder.
3. Verify keyboard shortcuts work globally.
4. Test responsive layout at 1280x720 and 3840x2160.

**Test checkpoint:** Full M1 gate (23 acceptance criteria from `milestones.md` §4).

### Definition of Done: M1

All 23 acceptance criteria from `milestones.md` §4 pass. `cargo test --workspace`, `npm test`, and `cargo bench` all pass. The app launches, displays the matrix, and the range builder is functional.

---

## 4. M2 Execution Guide: Solver Engine & Solution Storage

### Overview

Build the CFR solver (`poker-solver`), solution storage (`poker-solution`), and solve UI (`solve/`). At the end: solve a HU flop spot, watch progress, save and reload the solution.

**Requirements covered:** 29 MUST (SOL-001–012, SOL-016, SOL-018, STO-001–006, STO-010, STO-011, RTS-001–005, PRF-002, PRF-005)

**Algorithm:** M2 implements **CFR+** (Tammelin 2014) as the default and only solver algorithm. CFR+ floors negative cumulative regrets to zero, accelerating convergence vs vanilla CFR. Alternative algorithms (DCFR, Linear CFR) are M7 SHOULD requirements — do NOT implement them in M2.

### Step-by-Step Build Order

#### Step 1: poker-solver — Game Tree

1. `config.rs` — `SolverConfig`: positions, stack depth, pot size, bet sizes (IP and OOP), raise sizes, raise cap, stopping conditions.
2. `game_tree.rs` — Arena-allocated `GameTree`. `GameNode` enum: Decision (player, legal actions), Chance (board cards), Terminal (payoffs). `build_game_tree(config)` constructs the tree.
   - **Critical:** Use `Vec<GameNode>` arena. Nodes reference children by index, not Box pointer.
   - Enforce raise cap per street.
   - Convert to all-in when remaining stack < threshold of pot (SOL-016).
3. `info_set.rs` — `InfoSet` — player's information state: hand + board + action history. Hash for table lookup.
4. `strategy.rs` — `Strategy` — per-info-set action probability vector (`Vec<f32>`). Normalize to sum to 1.0.

**Test checkpoint:** `test_game_tree_structure`, `test_raise_cap`, `test_all_in_threshold`.

#### Step 2: poker-solver — CFR Engine

1. `cfr_plus.rs` — `CfrPlusSolver` implementing `SolverAlgorithm` trait.
   - Regret-matching: positive regrets → action probabilities.
   - CFR+ floors negative cumulative regrets to zero.
   - Store regrets and average strategies as contiguous `Vec<f32>` indexed by info set ID.
2. `mccfr.rs` — Monte Carlo CFR with external/chance sampling.
   - Sample board cards rather than enumerating all. Reduces memory dramatically.
3. `abstraction.rs` — Equity bucketing: cluster hands by equity into K buckets per street.
   - **Algorithm:** Equity-based K-means clustering. For each betting round, compute the equity of each hand against a uniform random opponent range, then cluster hands into K buckets using K-means on equity values.
   - **Default K values:** K=169 preflop (one per canonical hand), K=100 flop, K=100 turn, K=100 river.
   - **Distance metric:** Euclidean distance on equity distribution vectors (not just point equity — use the equity vs each opponent bucket).
   - **Initialization:** K-means++ for stable initialization.
   - **Reclustering:** Buckets are computed once per game tree configuration and cached.
4. `progress.rs` — `SolveProgress` (iteration, elapsed, Nash Distance estimate). `CancellationToken` (Arc<AtomicBool>).
5. `parallel.rs` — Parallel tree traversal via `rayon::scope`. Distribute subtrees across threads. Merge local regret updates.
6. `lib.rs` — `SolverAlgorithm` trait. `solve()` function: main loop calling `iterate()`, checking stopping conditions, emitting progress, checking cancellation.

**Critical algorithms:**
- CFR+: On each iteration, traverse tree for both players. Update regrets. Compute average strategy.
- MCCFR: Sample one or more chance outcomes per iteration. Reduce variance over many iterations.
- Parallel: Use `rayon::scope()` for fork-join parallelism. Each thread traverses independent subtrees with local regret accumulators, merged after each iteration.

**Test checkpoint:** `test_cfr_convergence`, `test_mccfr_convergence`, `test_parallel_speedup`, `test_stopping_*`, `test_cancellation`.

**Benchmark checkpoint:** `bench_standard_solve` < 10 sec; `bench_memory_usage` < 4 GB.

> **Performance reference:** The < 10 second target for HU, 1 street, 3 bet sizes (PRF-002) is based on comparable open-source solvers (e.g., PokerRL, OpenSpiel poker). If this target is missed by > 2x on reference hardware, profile and optimize before proceeding. Acceptable tuning includes: reducing default iteration count, adjusting convergence threshold, or tightening abstraction.

#### Step 3: poker-solution — Storage

1. `format.rs` — Magic bytes ("GTOSOL"), version number, format constants.
2. `header.rs` — `SolutionHeader` — magic, version, flags.
3. `metadata.rs` — `SolutionMetadata` — game config, Nash Distance, solve date (JSON, uncompressed in file header).
4. `quantize.rs` — `f32` strategy → `u16` fixed-point. Quantization error < 0.1%.
5. `sparse.rs` — Sparse encoding: if single action > 95%, encode as (dominant_action, exception_list).
6. `serialize.rs` — `serialize()` — convert SolveResult into binary format.
7. `compress.rs` — zstd compression (level 3) / decompression wrappers.
8. `reader.rs` — `SolutionReader` — memory-mapped file access. `open()` creates mmap; `metadata()` reads header; `load_node(id)` decompresses specific node.
9. `index.rs` — `SolutionIndex` — SQLite-backed. `search()`, `add()`, `remove()`, `total_disk_usage()`.
10. `cache.rs` — `SolutionCache` — LRU, bounded memory.

**Test checkpoint:** All `test-strategy.md` §3.4 tests. Round-trip: solve → serialize → compress → decompress → deserialize → verify identical.

**Benchmark checkpoint:** `bench_load_500mb` < 500 ms.

#### Step 4: app-tauri — Solver & Solution Commands

1. `commands/solver.rs` — `start_solve`, `cancel_solve`, `get_solve_status`.
   - `start_solve`: validate config → spawn_blocking → rayon solve → emit progress events → store result.
   - `cancel_solve`: set CancellationToken to true.
2. `commands/solution.rs` — `load`, `load_node`, `search`, `delete`, `disk_usage`, `import`, `export`.
3. `events.rs` — `solve_progress` event emission from solver thread.
4. Update `state.rs` — Add `SolverSession` map (HashMap<SessionId, SolverSession>), `SolutionIndex`, `SolutionCache`.
5. Update `main.rs` — Register new commands.

**Test checkpoint:** IPC round-trip tests for solver and solution commands.

#### Step 5: Frontend — solve/

1. `types.ts` — Solve-specific types.
2. `hooks/useSolveSession.ts` — Manages solve lifecycle: start → track progress → receive result.
3. `SolveConfigPanel.tsx` — Position selectors, stack depth, pot size, bet size editor.
4. `BetTreeEditor.tsx` — Visual bet tree configuration.
5. `ProgressDisplay.tsx` — Live progress: iteration count, Nash Distance, elapsed time, estimated remaining.
6. `ResultsViewer.tsx` — Display solved strategy using shared HandMatrix.
7. `StackDepthSlider.tsx` — 1–200bb range slider.
8. `SolvePage.tsx` — Page root.

Wire `ipc/types/solver.ts`, `ipc/types/solution.ts`, `ipc/commands/solver.ts`, `ipc/commands/solution.ts`, `ipc/hooks/useSolveProgress.ts`.

**Test checkpoint:** Full M2 gate (23 acceptance criteria from `milestones.md` §5).

### Definition of Done: M2

All 23 acceptance criteria pass. Solve a 100bb HU flop spot → watch progress → save → reload → verify strategy identity.

---

## 5. M3 Execution Guide: Study Mode & Solution Browsing

### Overview

Build Study Mode (`study/`), ICM module (`poker-icm`), and the solution browsing experience. At the end: load a pre-solved spot, browse all 4 tabs, navigate the game tree, view ICM-adjusted strategies.

**Requirements covered:** 18 MUST (STU-001–010, STU-012, ICM-001–004, AGG-001, PRF-003, PRF-004)

### Step-by-Step Build Order

#### Step 1: poker-icm

1. `types.rs` — `PayoutStructure`, `StackDistribution`, `IcmEquity`, `BubbleFactor`.
2. `standard.rs` — `StandardIcm` implementing `IcmModel` trait. Malmuth-Harville recursive calculation with memoization.
3. `bubble.rs` — Bubble factor: ratio of ICM loss-per-chip-lost to ICM gain-per-chip-won.
4. `lib.rs` — `IcmModel` trait definition and re-exports.

**Test checkpoint:** `cargo test -p poker-icm` — reference values match.

#### Step 2: app-tauri — ICM Commands

1. `commands/icm.rs` — `calculate_equity`, `bubble_factor`, `simulate_ft`.
2. Update `main.rs` — register ICM commands.

#### Step 3: Frontend — study/

Build in this order:
1. `types.ts` — Study state: spot, board, current node, active tab, overlay, filters.
2. `hooks/useStudyState.ts` — useReducer for study state management.
3. `SpotSelector.tsx` — Game type, position pair, stack depth, preflop action. On change → IPC `solution::load`.
4. `BoardSelector.tsx` — Card picking UI. Display 52 cards, grey out dealt cards. On select → update state → IPC `solution::load_node`.
5. `ActionSequence.tsx` — Horizontal bar showing the line of play: Preflop action → Flop action → etc. Click any action to navigate back.
6. `MetricOverlay.tsx` — Toggle buttons: Strategy, EV, Equity, EQR. Changes the hand matrix display mode.
7. `EvDisplay.tsx` — Per-action EV bars + overall range EV number.
8. `tabs/StrategyTab.tsx` — Action frequencies displayed in shared HandMatrix. Color segments proportional to frequency.
9. `tabs/RangesTab.tsx` — Each player's range at current node. Uses HandMatrix with range weight overlay.
10. `tabs/BreakdownTab.tsx` — Hand category groups (top pair, overpair, draws, etc.). Uses `poker-eval` categorization via IPC.
11. `tabs/ReportsTab.tsx` — Aggregated metrics across board subsets. Basic aggregate from AGG-001.
12. `StudyPage.tsx` — Four-tab container. Wire SpotSelector, BoardSelector, ActionSequence, tabs.

**Key implementation detail: Tab switching performance**
- Each tab manages its own derived data via `useMemo`.
- Tab content is lazily mounted (not hidden) to avoid stale renders.
- HandMatrix Canvas reuses the same canvas element across tabs where possible.
- Target: tab switch < 100ms (STU-001).

Wire `ipc/types/icm.ts`, `ipc/commands/icm.ts`.

**Test checkpoint:** Full M3 gate (17 acceptance criteria from `milestones.md` §6).

### Definition of Done: M3

All 17 acceptance criteria pass. Open a solved spot → navigate all 4 tabs → switch boards → view ICM-adjusted strategies.

---

## 6. M4 Execution Guide: Practice Mode

### Overview

Build Practice Mode (`practice/`). At the end: play hands against GTO, receive per-decision feedback, track session statistics.

**Requirements covered:** 9 MUST (PRA-001–009)

### Step-by-Step Build Order

#### Step 1: Backend — Practice Session Logic

In `app-tauri/commands/practice.rs`:
1. `start` — Accept config (spot, difficulty, RNG mode) → load solution → initialize game state → deal hole cards (respecting RNG mode) → return `PracticeHand`.
2. `act` — Accept player action → compare to GTO → compute EV loss → classify decision → advance game state (opponent acts according to GTO mixed strategy) → return `FeedbackResult`.
3. `next_hand` — Deal new hand, return new `PracticeHand`.
4. `get_session_stats` — Return cumulative stats.
5. `end_session` — Persist session to `practice_sessions` table.

**GTO opponent implementation:**
- At each opponent decision node, read the solver's mixed strategy.
- Generate a random number (using session PRNG).
- Select action according to cumulative probability distribution.
- Over many decisions, frequencies converge to solver's strategy within statistical tolerance.

**RNG mode implementation:**
- **Off:** Deal from shuffled deck (uniform random).
- **High:** Weight hand selection toward top 30% of starting hands by equity.
- **Low:** Weight hand selection toward bottom 30% of starting hands by equity.

**Hand Distribution Weighting (PRA-007):**
Practice mode deals hands with weighted probability to emphasize interesting spots:
- **Weight by equity vs opponent range:** Hands where player equity is 30-70% (close decisions) are weighted 3x.
- **Weight by EV loss opportunity:** Hands where the GTO strategy has > 0.5 bb EV difference between best and worst actions are weighted 2x.
- **Extreme hands de-weighted:** Hands where equity is > 90% or < 10% (trivial decisions) are weighted 0.5x.
- **Combined weight:** Multiply all applicable weights. Normalize to a probability distribution.
- **Configurable:** User can toggle weighted dealing on/off in practice settings.

**Difficulty levels:**
- **Simple:** Collapse bet sizes to 1-2 per node (small vs big).
- **Grouped:** Collapse to 2-3 per node (small, medium, large).
- **Standard:** All solver bet sizes available.

#### Step 2: Frontend — practice/

1. `types.ts` — Practice types: PracticeHand, FeedbackResult, SessionConfig.
2. `hooks/usePracticeSession.ts` — Session lifecycle management.
3. `SpotConfig.tsx` — Spot configuration panel (reuse patterns from SpotSelector).
4. `DifficultySelector.tsx` — Simple/Grouped/Standard buttons.
5. `RngSelector.tsx` — High/Low/Off toggle.
6. `GameTable.tsx` — Poker table with community cards, pot, player cards.
7. `ActionButtons.tsx` — Available actions for current node (filtered by difficulty).
8. `FeedbackPanel.tsx` — Post-decision: GTO strategy, player's action highlighted, EV loss, classification.
9. `ScoreDisplay.tsx` — Running score and classification (Perfect/Good/Inaccurate/Wrong/Blunder).
10. `SessionStats.tsx` — Hands played, decisions, average score, classification distribution.
11. `PracticePage.tsx` — Page root.

Wire `ipc/types/practice.ts`, `ipc/commands/practice.ts`.

**Test checkpoint:** Full M4 gate (9 acceptance criteria from `milestones.md` §7).

### Definition of Done: M4

All 9 acceptance criteria pass. Play 10 hands → receive correct feedback → session stats accurate.

---

## 7. M5 Execution Guide: Hand History Analysis

### Overview

Build the parser (`poker-hhparser`), analysis engine (`poker-analyze`), and Analyze Mode UI (`analyze/`). At the end: import hand histories, see analysis dashboard with EV loss.

**Requirements covered:** 18 MUST (ANL-001–010, HHP-001–008)

### Step-by-Step Build Order

#### Step 1: poker-hhparser — Parser Framework

1. `types.rs` — `ParsedHand`, `PlayerAction`, `SiteFormat`, `BatchParseResult`.
2. `encoding.rs` — Detect UTF-8, Latin-1, Windows-1252. Convert to UTF-8.
3. `position.rs` — `assign_positions()` — map seat numbers + button position → Position labels.
4. `pot.rs` — `reconstruct_pot()` — track pot through actions: blinds → bets → calls → raises. Handle side pots for multi-way all-in.
5. `registry.rs` — `ParserRegistry` with `auto_detect()` and `register()`.
6. `lib.rs` — `SiteParser` trait: `detect()`, `parse()`, `site_name()`. `parse_file()`, `parse_directory()`.

#### Step 2: poker-hhparser — Site Parsers

For each site, create `sites/{site}.rs`:

**Parser architecture pattern:**
```rust
pub struct PokerStarsParser;

impl SiteParser for PokerStarsParser {
    fn detect(content: &str) -> bool {
        content.starts_with("PokerStars Hand #")
    }

    fn parse(&self, content: &str) -> Result<Vec<ParsedHand>> {
        // Split into individual hand blocks
        // For each block: extract header, players, actions per street
        // Call reconstruct_pot() and assign_positions()
    }

    fn site_name(&self) -> &str { "PokerStars" }
}
```

Build parsers in order: PokerStars (most documented format) → GGPoker → Winamax → 888poker → PartyPoker → iPoker.

**Test checkpoint after each parser:** Parse sample file from `fixtures/hands/{site}/` → verify all fields.

#### Step 3: poker-analyze

1. `types.rs` — `DecisionClassification` (5 levels), `AnalyzedDecision`, `AnalyzedHand`, `DashboardStats`.
2. `classify.rs` — EV loss thresholds → classification. Define threshold constants.
3. `ev_loss.rs` — `compute_ev_loss(player_action, gto_strategy)` → f64 >= 0.
4. `analyze.rs` — `analyze_hand()` — for each decision in the hand: find matching solution, compute EV loss, classify.
5. `dashboard.rs` — `aggregate_stats()`, `position_breakdown()`, `street_breakdown()`, `action_breakdown()`.

**Test checkpoint:** `cargo test -p poker-analyze` — all §3.7 tests pass.

#### Step 4: app-tauri — HH & Analysis Commands

1. `commands/handhistory.rs` — `import` (single file), `import_directory` (batch), `get_import_status`. Emit `import_progress` events.
2. `commands/analyze.rs` — `get_hands`, `get_hand_detail`, `get_dashboard`, `filter_hands`.
3. Update `main.rs` and `state.rs`.

#### Step 5: Frontend — analyze/

1. `types.ts`, `hooks/useAnalyzeState.ts`.
2. `ImportWizard.tsx` — File/directory selector → import → progress bar with per-file status.
3. `HandsTable.tsx` — Virtualized list of analyzed hands.
4. `HandReplay.tsx` — Street-by-street replay: community cards, pot, player actions. Forward/backward/jump navigation.
5. `GtoOverlay.tsx` — At each decision node during replay: show solver's strategy + player's action + EV loss + classification.
6. `Dashboard.tsx` — Aggregate statistics: total hands, avg EV loss, classification pie chart.
7. `PositionBreakdown.tsx`, `StreetBreakdown.tsx`, `ActionBreakdown.tsx` — Statistical breakdowns.
8. `AnalyzePage.tsx` — Page root with import wizard, hands table, dashboard, and replay views.

Wire `ipc/types/handhistory.ts`, `ipc/types/analyze.ts`, `ipc/commands/handhistory.ts`, `ipc/commands/analyze.ts`, `ipc/hooks/useImportProgress.ts`, `ipc/hooks/useAnalyzeProgress.ts`.

**Test checkpoint:** Full M5 gate (18 acceptance criteria from `milestones.md` §8).

### Definition of Done: M5

All 18 acceptance criteria pass. Import PokerStars file → see dashboard → replay hand → verify GTO overlay.

---

## 8. M6 Execution Guide: Nodelocking & Aggregated Reports

### Overview

Build Nodelocking (`nodelock/`) and Aggregated Reports (`reports/`). At the end: lock opponent strategy, re-solve for exploits, generate aggregate reports.

**Requirements covered:** 10 MUST + 6 SHOULD (NLK-001–009, AGG-001–008)

### Step-by-Step Build Order

#### Step 1: poker-solver — Nodelock Extension

Add `nodelock.rs` to `poker-solver`:
1. `NodeLock` struct: maps NodeId → locked Strategy.
2. `apply_locks()` — during CFR iteration, skip locked nodes (use fixed strategy).
3. `re_solve_with_locks()` — warm-start from GTO solution, iterate only unlocked nodes.
4. `cascade_ranges()` — recalculate ranges downstream of locked nodes.

**Test checkpoint:** Lock a node → re-solve → exploit EV > GTO EV against locked strategy.

#### Step 2: app-tauri — Nodelock & Reports Commands

1. `commands/nodelock.rs` — `lock_node`, `unlock_node`, `edit_frequencies`, `re_solve`, `compare`, `reset`.
2. `commands/reports.rs` — `aggregate_flops`, `aggregate_turns`, `filter_by_texture`, `export_report`.

#### Step 3: Frontend — nodelock/

1. `LockControls.tsx` — Lock/unlock/reset buttons with visual indicators.
2. `FrequencyEditor.tsx` — Sliders + numeric inputs. Auto-adjust to maintain 100%.
3. `ComparisonView.tsx` — Side-by-side GTO vs nodelock matrices.
4. `GameTreeNav.tsx` — Navigate tree to select nodes for locking.
5. `BatchLockPanel.tsx` — Select multiple nodes, apply same modification.
6. `NodelockPage.tsx` — Page root.

#### Step 4: Frontend — reports/

1. `MetricSelector.tsx` — Strategy/EV/Equity/EQR radio buttons.
2. `BoardTextureFilters.tsx` — Checkboxes: monotone, two-tone, rainbow, paired, connected, high card.
3. `FlopGrouping.tsx` — Grouping selector: high card, suit composition, pairing, connectedness.
4. `ActionGrouping.tsx` — All Sizes / Grouped / Simplified radio buttons.
5. `AggregateChart.tsx` — Bar chart using Canvas or a lightweight chart library.
6. `AggregateTable.tsx` — Sortable table component.
7. `ReportsPage.tsx` — Page root.

**Test checkpoint:** Full M6 gate (17 acceptance criteria from `milestones.md` §9).

**MUST gate check:** Verify all 129 MUST requirements across M1–M6 pass their acceptance criteria.

### Definition of Done: M6

All 17 M6 criteria pass. All MUST requirements verified. Lock opponent → re-solve → compare. Generate aggregate report → filter by texture → switch views.

---

## 9. M7–M8 Summary Guides

### M7: SHOULD Enhancements (42 requirements)

M7 adds SHOULD features to existing modules. For each enhancement:
1. Read the requirement in `requirements.md`.
2. Add the file(s) listed in `project-structure.md` §6.7.
3. Wire into the existing module's page component.
4. Write tests matching `test-strategy.md` patterns.
5. Run full regression suite before committing.

### M7 Implementation Steps

**Step 1 — DCFR and Linear CFR variants**
- Add `SolverAlgorithm` enum: `{ CfrPlus, Dcfr, LinearCfr }` to `poker-solver`
- DCFR: multiply cumulative regrets by `t^α / (t^α + 1)` and cumulative strategy by `t^β / (t^β + 1)` each iteration (α=1.5, β=0.5 defaults)
- Linear CFR: weight iteration `t` by `t` (multiply strategy contribution by iteration number)
- Expose algorithm selection in solver config IPC command
- Test: solve same spot with all 3 algorithms, verify all converge to < 0.5% Nash Distance

**Step 2 — Startup optimization (PRF-006: < 3 seconds)**
- Profile startup with `tracing` spans: measure eval table load, SQLite connection, UI render
- Lazy-load eval tables: start UI immediately, load tables in background
- Pre-compile SQLite prepared statements at build time if possible
- Test: measure cold start and warm start times on reference hardware

**Step 3 — Multi-threading optimization (PRF-007: >= 6x on 8 cores)**
- Profile solver with `rayon` on 1, 2, 4, 8 threads
- Identify contention points (regret table writes, random number generation)
- Implement per-thread RNG with deterministic seeding
- Test: verify speedup scales >= 6x on 8 cores for reference spot

**Step 4 — Remaining SHOULD requirements**
- Implement remaining SHOULD requirements from requirements.md in priority order
- Each SHOULD requirement gets its own branch and test
- See expanded priority list below for ordering

**M7 SHOULD Requirements — Complete Priority Ordering:**

**Tier 1 — High Impact (implement first):**

| # | Requirement | Description | Crate/Module | Estimated Effort |
|---|-------------|-------------|--------------|------------------|
| 1 | SOL-014 | DCFR / Linear CFR variants | `poker-solver` | Medium |
| 2 | SOL-013 | Preflop range solving | `poker-solver` | Medium |
| 3 | PRF-006 | Startup < 3 seconds | `app-tauri` | Medium |
| 4 | PRF-007 | Solver >= 6x speedup on 8 cores | `poker-solver` | Medium |
| 5 | HEV-008 | Batch evaluation (SIMD/vectorized) | `poker-eval` | Medium |
| 6 | SOL-015 | Dynamic bet size discovery | `poker-solver` | Medium |
| 7 | RTS-006 | Depth-limited solving | `poker-solver` | Medium |
| 8 | UIF-011 | Four-color deck option | `shared/` | Small |
| 9 | ANL-011 | Hand filtering (position, stakes, date) | `analyze/` | Medium |
| 10 | ANL-012 | Biggest mistakes view | `analyze/` | Medium |

**Tier 2 — Medium Impact:**

| # | Requirement | Description | Crate/Module | Estimated Effort |
|---|-------------|-------------|--------------|------------------|
| 11 | STO-007 | Incremental save / checkpoint | `poker-solution` | Medium |
| 12 | STO-008 | Solution import/export | `poker-solution`, `app-tauri` | Medium |
| 13 | STO-009 | Format versioning / migration | `poker-solution` | Medium |
| 14 | STU-011 | Hand detail view | `study/` | Medium |
| 15 | STU-013 | Compare EV overlay | `study/` | Small |
| 16 | STU-014 | Range weight display | `study/` | Small |
| 17 | STU-015 | Flop subset navigation / browser | `study/` | Medium |
| 18 | RTS-007 | Dynamic sizing mode | `poker-solver` | Medium |
| 19 | ANL-013 | Session summary | `analyze/` | Medium |
| 20 | ANL-014 | Export analysis results (CSV/JSON) | `analyze/` | Small |
| 21 | PRA-010 | Hand history review | `practice/` | Medium |
| 22 | PRA-011 | Multitabling (1-4 tables) | `practice/` | Medium |
| 23 | PRA-012 | Game speed control | `practice/` | Small |
| 24 | HHP-010 | File format auto-detection | `poker-hhparser` | Small |
| 25 | HHP-011 | Incremental import | `poker-hhparser` | Medium |

**Tier 3 — Lower Impact / Polish:**

| # | Requirement | Description | Crate/Module | Estimated Effort |
|---|-------------|-------------|--------------|------------------|
| 26 | ICM-005 | FT/SNG simulator (Monte Carlo) | `poker-icm` | Medium |
| 27 | ICM-006 | Progressive Knockout (PKO) bounty | `poker-icm` | Medium |
| 28 | ICM-007 | Satellite ICM dynamics | `poker-icm` | Medium |
| 29 | HHP-009 | Currency/chip normalization | `poker-hhparser` | Small |
| 30 | UIF-010 | WCAG AA contrast compliance | `shared/` | Small |
| 31 | UIF-012 | Animation system | `shared/` | Small |
| 32 | UIF-015 | Theme customization | `shared/` | Medium |
| 33 | HMX-010 | Suit expansion view | `shared/` | Medium |
| 34 | HMX-011 | Matrix sizing options | `shared/` | Small |
| 35 | DSK-007 | Windows primary platform optimization | `app-tauri` | Medium |
| 36 | DSK-008 | Auto-update mechanism | `app-tauri` | Large |
| 37 | DSK-009 | Performance monitoring / diagnostics | `app-tauri` | Medium |
| 38 | DAT-005 | Glossary compliance audit | All | Small |
| 39 | DAT-006 | Solution file format documentation | `poker-solution` | Small |
| 40 | RNG-008 | Grade vs GTO scoring | `range-builder/` | Medium |
| 41 | RNG-009 | Range import/export (text format) | `range-builder/` | Small |
| 42 | AGG-009 | Filtered vs overall comparison | `reports/` | Small |

**Dependencies among SHOULD requirements:**
- SOL-015 (dynamic bet sizes) should be done before RTS-007 (dynamic sizing mode), as RTS-007 depends on SOL-015
- STU-014 (range weight display) depends on STU-011 (hand detail view) being complete
- ANL-013 (session summary) should precede ANL-014 (export), as export benefits from session grouping
- ICM-006 (PKO) should precede ICM-007 (satellite), as satellite builds on the same ICM infrastructure
- DSK-008 (auto-update) is independent but should be one of the last items due to deployment complexity

**M7 workflow per requirement:**
1. Create a feature branch: `git checkout -b m7/{requirement-id}`
2. Implement the feature following the existing module's patterns
3. Write tests specific to the requirement
4. Run `cargo test --workspace && npm test` — all pass including regression
5. Run any relevant benchmarks (PRF-* requirements)
6. Merge to main, move to next requirement

### M8: MAY Polish (17 requirements)

M8 adds optional features. Implement based on available time. Each feature is independent — implement in any order.

### M8 Implementation Steps

**Step 1 — Multi-way pots (3+ players)**
- Extend game tree builder to handle 3+ player decision nodes
- Modify CFR traversal for multi-player regret computation
- Key challenge: information set explosion — use abstraction aggressively
- Test: solve 3-player preflop spot, verify strategies sum to 1.0

**Step 2 — Short deck (6+) support (if applicable)**
- Add `GameVariant::ShortDeck` with 36-card deck (remove 2-5)
- Modify hand evaluator: flushes beat full houses in short deck
- Update equity calculations for reduced deck
- Test: verify evaluator correctness with known short deck hand rankings

**Step 3 — Advanced solver features**
- Implement remaining MAY requirements from requirements.md
- Focus on features with highest user value first
- Each MAY requirement gets its own feature flag for optional inclusion

#### Quick Wins (< 1 day each)
| Requirement | Description | Implementation Notes |
|-------------|-------------|---------------------|
| DAT-007 | TOML config validation | Already partially implemented in M1; add schema validation and error messages |
| NLK-010 | Reset to GTO button | Single button in nodelock/ UI calling `unlock_all()` IPC command |
| HMX-012 | Range notation string | Generate "AA,AKs,AKo,..." text from 169-cell weight array |
| UIF-013 | Context menus | Right-click handlers on matrix cells and hand list rows |
| STO-012 | Solution checksum | Add `checksum.rs` to `poker-solution`: CRC32 on serialized data, verify on load |

#### Moderate Effort (1–3 days each)
| Requirement | Description | Implementation Notes |
|-------------|-------------|---------------------|
| PRA-013 | Timebank with configurable durations | Decision timer UI; default action on expiry |
| PRA-014 | Performance trends | SQLite table for historical sessions; trend chart in practice/ |
| STU-016 | Solution comparison | Side-by-side study views with diff overlay on matrices |
| AGG-010 | Report export (PNG/SVG/CSV/JSON) | Canvas export for charts; data serialization for tables |
| HEV-010 | Evaluation caching | LRU cache for recent board+hand evaluations |
| HHP-012 | Hand history validation | Internal consistency checks (stack/bet limits, no duplicate cards) |

#### Complex Features (3+ days each)
| Requirement | Description | Implementation Notes |
|-------------|-------------|---------------------|
| SOL-017 | 3-way solving | Extend game tree for 3 players — memory and time scale cubically |
| PRF-008 | Solutions > 4 GB | Chunked mmap, 64-bit indices, streaming decompression |
| DSK-010 | Crash recovery | WAL checkpointing, solution write journaling, startup integrity check |
| RTS-008 | Background solving | Queue multiple solve requests; notify on completion |
| ANL-015 | Comparative analysis | Two-period improvement comparison view |
| ICM-008 | Mystery bounty support | Distribution-based bounty EV calculation |
| UIF-014 | Zoom and pan | Game tree visualization with mouse wheel zoom and click-drag pan |

---

## 10. Error Recovery

### Tests Fail

1. Read the error message carefully. Identify which test and which assertion.
2. Check if the failure is in new code or existing code (regression).
3. For regressions: `git diff` to find the change that broke it. Fix or revert.
4. For new test failures: the implementation doesn't match the specification. Re-read the requirement.
5. Never skip a failing test. Fix it or update the test if the specification was unclear.

### Build Breaks

1. `cargo build` failure: check for missing imports, type mismatches, or dependency issues.
2. Verify you haven't imported from a crate that isn't built yet (DAG violation).
3. `npm run build` failure: check TypeScript type errors. Ensure IPC types match Rust serde types.
4. Clear caches: `cargo clean`, `rm -rf node_modules/.vite`.

### Performance Below Target

1. Verify you're measuring in release mode (`cargo bench` uses release by default).
2. Profile with `cargo flamegraph` to find the hot path.
3. Check `technical-constraints.md` §3 for the specific constraint you're violating.
4. Common fixes:
   - Evaluator slow → check lookup tables are loaded and `#[inline]` is present.
   - Solver slow → check arena allocation, no HashMap in hot path, rayon configured.
   - Solution loading slow → check mmap is working, not doing full-file read.
   - UI janky → check Canvas rendering, not DOM-based grid.

### Milestone Blocks

If a milestone blocks due to an issue in a previous milestone:
1. Don't work around it. Go back and fix the root cause.
2. If the fix would change an API, update both the provider crate and all consumers.
3. Re-run the previous milestone's gate tests after the fix.

### IPC Type Mismatches

Symptoms: `invoke()` returns unexpected `null`, or Tauri command fails with serialization error.

1. Compare the Rust `serde` struct field names with the TypeScript interface property names. They must match exactly (Rust `snake_case` → TypeScript `camelCase` requires `#[serde(rename_all = "camelCase")]` on the Rust struct).
2. Check that Rust `Option<T>` maps to TypeScript `T | null` (not `T | undefined`).
3. Check that Rust `Vec<T>` maps to TypeScript `T[]`.
4. Check that Rust enums with `#[serde(tag = "type")]` produce the expected JSON discriminant.
5. Add a round-trip test: serialize in Rust → send to TS → send back → deserialize → compare.

### Canvas Rendering Issues

Symptoms: blurry text, misaligned grid, flickering, or wrong colors.

1. **Blurry:** Check `devicePixelRatio` scaling — canvas internal size must be `logicalSize * dpr`.
2. **Misaligned:** Ensure coordinates are rounded to integer pixels for grid lines.
3. **Flickering:** Verify using `requestAnimationFrame`, not synchronous redraws from React render.
4. **Wrong colors:** Confirm action color mapping matches `ActionColorSystem.ts` sRGB values.
5. **Memory leak:** Check that you're reusing `CanvasRenderingContext2D`, not creating new ones per frame.

### Solver Convergence Issues

Symptoms: Nash Distance not decreasing, or oscillating after many iterations.

1. Verify regret matching is correct: negative regrets clipped to zero (CFR+), then normalize to get action probabilities.
2. Check for division by zero when all regrets are zero — should default to uniform strategy.
3. If MCCFR, verify chance sampling is unbiased — each board card has equal probability.
4. If parallel, verify regret merge is atomic or correctly synchronized — lost updates cause slow convergence.
5. Log Nash Distance every 100 iterations and plot — should decrease roughly monotonically with noise.

---

## 11. Quality Checklist

Run this checklist before declaring any milestone complete.

### Per-Milestone Sign-Off

- [ ] All milestone acceptance criteria pass (from `milestones.md`)
- [ ] All previous milestone tests still pass (regression)
- [ ] `cargo test --workspace` — all tests pass
- [ ] `cargo clippy --workspace` — no warnings
- [ ] `npm test` — all frontend tests pass
- [ ] `npm run build` — frontend builds without errors
- [ ] `eslint` — no errors
- [ ] Benchmarks meet PRF-* targets (where applicable to this milestone)
- [ ] No `unwrap()` in library crates (grep to verify)
- [ ] No cross-module frontend imports (grep to verify)
- [ ] All IPC commands return `Result<T, AppError>`
- [ ] No files created that belong to a later milestone
- [ ] Code committed with descriptive message

### Quick Verification Commands

```bash
# Rust: test + lint
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Frontend: test + lint + build
cd frontend && npm test && npm run build && npx eslint src/

# Benchmarks (release mode)
cargo bench -p poker-eval
cargo bench -p poker-solver
cargo bench -p poker-solution

# Check for prohibited patterns
grep -r "unwrap()" crates/poker-core/src/ crates/poker-eval/src/ crates/poker-solver/src/ crates/poker-solution/src/ crates/poker-icm/src/ crates/poker-hhparser/src/ crates/poker-analyze/src/
grep -r "println!" crates/*/src/
```

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial agent implementation prompt |

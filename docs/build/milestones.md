# Milestones & Build Sequence

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 3 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisites:** [Requirements Specification](requirements.md) (Build Document 1), [System Architecture](architecture.md) (Build Document 2)

---

## 1. Scope & Purpose

### What This Document Defines

This document sequences the 195 requirements from Build Document 1 into **8 milestones** — vertical slices where each milestone produces a testable artifact. Every milestone builds on the previous ones, respects the crate dependency DAG from Build Document 2, and includes pass/fail acceptance criteria.

### Relationship to Source Documents

| Source | What This Document Uses |
|--------|------------------------|
| `requirements.md` | All 195 requirement IDs with priority (129 MUST, 48 SHOULD, 17 MAY) and dependency chains |
| `architecture.md` | 8 Rust crates, 9 frontend modules, crate dependency DAG, 4 data flows |

### Milestone Principles

1. **Vertical slice** — each milestone delivers end-to-end functionality, not isolated layers
2. **Testable artifact** — a reviewer can run or inspect a concrete output to confirm completion
3. **Dependency-respecting** — no milestone requires code from a later milestone
4. **Priority-ordered** — all 129 MUST requirements complete by M6; SHOULD in M7; MAY in M8
5. **Incremental** — each milestone compiles and runs on top of the previous milestone's code

---

## 2. Milestone Summary

| Milestone | Name | MUST | SHOULD | MAY | Testable Artifact |
|-----------|------|------|--------|-----|-------------------|
| **M1** | Foundation & Hand Matrix | 45 | — | — | CLI hand evaluator + Tauri app with interactive 13x13 matrix and range builder |
| **M2** | Solver Engine & Solution Storage | 29 | — | — | Solve a heads-up flop spot, persist and reload the solution |
| **M3** | Study Mode & Solution Browsing | 18 | — | — | Browse a solved spot with all 4 Study Mode tabs functional |
| **M4** | Practice Mode | 9 | — | — | Play practice hands against GTO, get per-decision feedback |
| **M5** | Hand History Analysis | 18 | — | — | Import HH file, see analysis dashboard with EV loss |
| **M6** | Nodelocking & Aggregated Reports | 10 | 6 | — | Lock opponent strategy, re-solve, view aggregate report |
| **M7** | SHOULD Enhancements | — | 42 | — | All P1 features integrated and passing |
| **M8** | MAY Polish | — | — | 17 | Final polish features integrated |
| | **Total** | **129** | **48** | **17** | |

> **Note:** The requirements specification lists 129 MUST requirements. This milestones document assigns every requirement ID to exactly one milestone; the coverage matrix in Section 12 is the definitive mapping.

---

## 3. Dependency Graph

```
M1 ──→ M2 ──→ M3 ──┬──→ M4
                    │
                    └──→ M5 ──┐
                              ├──→ M6 ──→ M7 ──→ M8
                    M4 ───────┘
```

**M4 and M5 are parallel** — they share no mutual dependencies and can be developed concurrently after M3 completes.

### Dependency Rationale

| Edge | Why |
|------|-----|
| M1 → M2 | Solver (`poker-solver`) requires `poker-core` types and `poker-eval` hand evaluation |
| M2 → M3 | Study Mode needs persisted solutions to browse; `poker-solution` reader required |
| M3 → M4 | Practice Mode reuses Study Mode's solution loading, hand matrix rendering, and IPC commands |
| M3 → M5 | Analysis computes EV loss by comparing player actions against loaded GTO solutions |
| M4, M5 → M6 | Nodelocking extends the solver + study pipeline; aggregate reports need analysis data paths |
| M6 → M7 | SHOULD enhancements build on the complete MUST feature set |
| M7 → M8 | MAY features are polish on the fully functional application |

### Crate Build Order (from `architecture.md` DAG)

```
poker-core          ← M1 (leaf crate, no dependencies)
├── poker-eval      ← M1
├── poker-icm       ← M3
├── poker-hhparser  ← M5
poker-solver        ← M2 (depends on poker-core, poker-eval)
poker-solution      ← M2 (depends on poker-core)
poker-analyze       ← M5 (depends on all library crates)
app-tauri           ← M1 skeleton, grows through M6
```

---

## 4. M1 — Foundation & Hand Matrix

**Priority:** P0 (MUST only)
**MUST requirements:** 45

### Scope

Deliver the foundational Rust crates (`poker-core`, `poker-eval`), the Tauri application shell (`app-tauri`), and the core frontend infrastructure (`shared/`, `range-builder/`, `ipc/`). The vertical slice: a user can launch the desktop app, see a 13x13 hand matrix, build ranges interactively, and evaluate hands via CLI.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Rust | `poker-core` | Card, Deck, Hand, Board, Action, Position, GameType, BetSize types |
| Rust | `poker-eval` | Lookup-table evaluator, equity calculator, draw detection, blocker analysis, suit isomorphism |
| Rust | `app-tauri` (skeleton) | Tauri v2 shell, IPC bridge, data directory init, settings persistence, window management |
| Frontend | `shared/` | HandMatrix (13x13 Canvas), CardRenderer, ActionColorSystem, Tooltip, KeyboardShortcuts, LoadingState, ErrorBoundary, Navigation |
| Frontend | `range-builder/` | RangeGrid, PaintbrushTools, WeightControls, SuitExpander, PresetSelector |
| Frontend | `ipc/` | Typed Tauri command wrappers, event listeners, request/response types |

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Data & Formats | DAT-001, DAT-002, DAT-003, DAT-004 | 4 |
| Hand Evaluation | HEV-001, HEV-002, HEV-003, HEV-004, HEV-005, HEV-006, HEV-007, HEV-009 | 8 |
| Hand Matrix | HMX-001, HMX-002, HMX-003, HMX-004, HMX-005, HMX-006, HMX-007, HMX-008, HMX-009 | 9 |
| Range Builder | RNG-001, RNG-002, RNG-003, RNG-004, RNG-005, RNG-006, RNG-007, RNG-010 | 8 |
| UI Framework | UIF-001, UIF-002, UIF-003, UIF-004, UIF-005, UIF-006, UIF-007, UIF-008, UIF-009 | 9 |
| Desktop App | DSK-001, DSK-002, DSK-003, DSK-004, DSK-005, DSK-006 | 6 |
| Performance | PRF-001 | 1 |
| **Total** | | **45** |

### Acceptance Criteria

| # | Criterion | Requirement |
|---|-----------|-------------|
| 1 | `poker-core` compiles; Card, Hand, Deck types pass unit tests covering all ranks, suits, and hand categories | DAT-001 through DAT-004 |
| 2 | `poker-eval` evaluates all 7,462 distinct 5-card hand classes correctly | HEV-001, HEV-005 |
| 3 | Hand evaluator benchmark: ≥ 200M evaluations/sec on release build (single core) | HEV-002, PRF-001 |
| 4 | Equity calculator produces correct results for 10 sample matchups (within 0.1%) | HEV-004 |
| 5 | Draw detection identifies flush draws, OESDs, gutshots, and combo draws on test boards | HEV-006 |
| 6 | Blocker analysis correctly reduces opponent combo counts for 5 known scenarios | HEV-007 |
| 7 | Suit isomorphism maps all 22,100 flops to exactly 1,755 canonical representatives | HEV-003 |
| 8 | Hand ranking comparison is transitive and handles ties correctly | HEV-009 |
| 9 | Tauri app launches, displays React frontend in webview with dark theme | DSK-001, UIF-001 |
| 10 | 13x13 hand matrix renders with correct hand labels, mixed-strategy coloring, and combo counts | HMX-001 through HMX-007 |
| 11 | Matrix supports hover detail, click selection, action filtering, metric overlays, and frequency threshold filter | HMX-003 through HMX-009 |
| 12 | Range builder: paint, erase, weight adjustment (0-100%), suit combos, presets, locking, and color coding all functional | RNG-001 through RNG-007, RNG-010 |
| 13 | Range summary shows correct combo count, percentage, and equity vs. opponent range | RNG-004 |
| 14 | IPC bridge: frontend invokes Rust commands and receives typed responses without error | DSK-003 |
| 15 | Keyboard shortcuts (J, S, P, Q, Space, 1-4) trigger documented actions | UIF-002 |
| 16 | Application renders correctly from 1280x720 to 3840x2160; responsive layout adapts | UIF-003 |
| 17 | Action color system consistent: red=bet/raise, green=check/call, blue=fold across all views | UIF-004 |
| 18 | Navigation sidebar allows switching between modes; loading indicators appear for async operations | UIF-005, UIF-006 |
| 19 | Error messages display for invalid input; tooltips appear on hover for icons/labels | UIF-007, UIF-008 |
| 20 | Cards render with correct rank/suit symbols and conventional colors | UIF-009 |
| 21 | Native file dialogs open correctly; application settings persist across restarts | DSK-004, DSK-005 |
| 22 | Window minimize/maximize/restore/resize/close all work; data directory `~/.poker-solver/` created on first run | DSK-002, DSK-006 |
| 23 | Application operates fully offline — no network calls for core functionality | DSK-002 |

### Testable Artifact

Tauri desktop app with:
- A CLI mode (`--eval "AhKhQhJhTh"`) that evaluates and ranks poker hands
- An interactive range builder UI with paintbrush selection, weight controls, and preset ranges
- A 13x13 hand matrix with all overlays, filtering, and hover/click interactions functional

### Key Risks

| Risk | Mitigation |
|------|------------|
| Lookup table generation takes too long on first run | Pre-compute tables at build time; cache to `eval_tables.bin` (~10MB, 2-5 sec generation) |
| Canvas-based hand matrix performance on low-end GPUs | Implement fallback DOM-based renderer; test on integrated graphics |
| IPC type mismatches between Rust and TypeScript | Generate TypeScript types from Rust `serde` structs; CI validates type compatibility |

---

## 5. M2 — Solver Engine & Solution Storage

**Priority:** P0 (MUST only)
**MUST requirements:** 29
**Depends on:** M1

### Scope

Build the CFR solver engine (`poker-solver`), solution storage system (`poker-solution`), and real-time solve UI (`solve/`). The vertical slice: a user configures a postflop spot, runs the solver, watches progress, and saves/reloads the resulting solution.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Rust | `poker-solver` | CFR engine, game tree builder, MCCFR, card abstraction, suit isomorphism, parallel solving (rayon), cancellation |
| Rust | `poker-solution` | Binary format, zstd compression, mmap reader, SQLite index, strategy quantization, sparse encoding, metadata, disk usage |
| Frontend | `solve/` | SolveConfigPanel, BetTreeEditor, ProgressDisplay, ResultsViewer |

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Solver Engine | SOL-001, SOL-002, SOL-003, SOL-004, SOL-005, SOL-006, SOL-007, SOL-008, SOL-009, SOL-010, SOL-011, SOL-012, SOL-016, SOL-018 | 14 |
| Solution Storage | STO-001, STO-002, STO-003, STO-004, STO-005, STO-006, STO-010, STO-011 | 8 |
| Real-Time Solver | RTS-001, RTS-002, RTS-003, RTS-004, RTS-005 | 5 |
| Performance | PRF-002, PRF-005 | 2 |
| **Total** | | **29** |

### Acceptance Criteria

| # | Criterion | Requirement |
|---|-----------|-------------|
| 1 | Game tree builder constructs correct tree for a heads-up flop spot with configurable bet sizes and raise cap | SOL-004, SOL-005, SOL-006 |
| 2 | CFR solver converges: Nash Distance < 0.5% pot within 10,000 iterations on 8-core/32GB hardware for a standard HU flop spot | SOL-001, SOL-002 |
| 3 | MCCFR chance sampling produces equivalent strategies within tolerance; memory < 4GB for standard 100bb HU spot | SOL-007, PRF-005 |
| 4 | Multi-street solving (flop → turn → river) produces strategy output for all streets | SOL-008 |
| 5 | Suit isomorphism reduces 22,100 flops to 1,755 in game tree; suit-equivalent boards produce identical strategies | SOL-011 |
| 6 | Card abstraction/bucketing reduces information sets while keeping Nash Distance < 1% pot | SOL-012 |
| 7 | Parallel solving (`rayon`) utilizes multiple cores; measured speedup > 2x on 4 cores vs 1 core | SOL-009 |
| 8 | Solve stopping conditions work: max iterations, target Nash Distance, and time limit all halt the solver | SOL-010 |
| 9 | Solve cancellation stops within 1 iteration of cancel signal | SOL-018 (cancel), SOL-010 |
| 10 | All-in threshold converts small remaining stacks to all-in automatically | SOL-016 |
| 11 | Progress events stream to frontend during solve: iteration count, elapsed time, Nash Distance, estimated remaining | SOL-018 |
| 12 | Solution serialization: write + read round-trip preserves all strategy data identically | STO-001, STO-004, STO-011 |
| 13 | Zstd compression achieves ≥ 3:1 ratio on typical solution data | STO-002 |
| 14 | Memory-mapped loading: 500MB solution loads in < 500ms with < 500MB resident memory | STO-003 |
| 15 | Solution metadata readable without full decompression; all required fields present | STO-005 |
| 16 | Solution index (SQLite): query by game_type + position + stack_depth returns correct file within 100ms | STO-006 |
| 17 | Storage space display matches actual disk usage within 1%; individual solution deletion works | STO-010 |
| 18 | Real-time solver accepts arbitrary postflop configuration and produces solution on demand | RTS-001 |
| 19 | Standard spot (HU, single street, 3 bet sizes) solves to < 0.5% Nash Distance in < 10 seconds | RTS-002, PRF-002 |
| 20 | Fixed and Automatic sizing modes both produce valid results | RTS-003 |
| 21 | Solver handles stack depths from 1bb to 200bb | RTS-004 |
| 22 | Re-querying an identical game state returns cached result in < 100ms | RTS-005 |
| 23 | Solve config UI allows setting stack sizes, pot size, bet sizes, and starting ranges | RTS-001 (UI) |

### Testable Artifact

Solve a 100bb heads-up flop spot (e.g., BTN vs BB on Q♠T♠7♥):
1. Configure the spot in the Solve Config panel
2. Watch live progress (iteration count, Nash Distance)
3. View final results with strategy display
4. Save the solution to disk
5. Reload the solution and verify identical strategy data

### Key Risks

| Risk | Mitigation |
|------|------------|
| CFR convergence too slow for complex multi-street trees | MCCFR as primary algorithm; depth-limited solving as fallback; expose convergence target in UI |
| Memory exceeds 4GB budget for large game trees | MCCFR chance sampling bounds memory; card abstraction reduces information sets; configurable memory limit |
| Parallel solving introduces non-determinism | Lock-free atomic updates for regret tables; deterministic seed option for testing |

---

## 6. M3 — Study Mode & Solution Browsing

**Priority:** P0 (MUST only)
**MUST requirements:** 18
**Depends on:** M2

### Scope

Build Study Mode (`study/`), the ICM module (`poker-icm`), and basic aggregation (`AGG-001`). The vertical slice: a user opens a previously solved spot, navigates all 4 Study Mode tabs, switches boards, and views ICM-adjusted strategies for tournament spots.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Rust | `poker-icm` | ICM equity (Malmuth-Harville), ICM-adjusted solving, configurable payouts, bubble factor |
| Frontend | `study/` | SpotSelector, BoardSelector, StrategyTab, RangesTab, BreakdownTab, ReportsTab, ActionSequence, MetricOverlay |

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Study Mode | STU-001, STU-002, STU-003, STU-004, STU-005, STU-006, STU-007, STU-008, STU-009, STU-010, STU-012 | 11 |
| Tournament & ICM | ICM-001, ICM-002, ICM-003, ICM-004 | 4 |
| Aggregated Reports | AGG-001 | 1 |
| Performance | PRF-003, PRF-004 | 2 |
| **Total** | | **18** |

### Acceptance Criteria

| # | Criterion | Requirement |
|---|-----------|-------------|
| 1 | Spot Selector: filter solutions by game type, position, stack depth; changing any parameter loads matching solution within 500ms | STU-002, PRF-004 |
| 2 | Board Selector: select any flop/turn/river cards; display updates to show strategy for selected board | STU-003 |
| 3 | Strategy tab: action frequencies per hand in matrix with correct proportional color fills; frequencies sum to 100% | STU-004 |
| 4 | Ranges tab: shows each player's range at current node with frequency weights; hand counts match expected combos | STU-005 |
| 5 | Breakdown tab: categorizes range into hand strength groups (top pair, overpair, draws, etc.) with per-group strategy | STU-006 |
| 6 | Reports tab: aggregated strategy metrics across board subsets match individual board calculations | STU-007, AGG-001 |
| 7 | Four-tab interface: all tabs render distinct content; tab switching completes in < 100ms | STU-001 |
| 8 | Action sequence navigation: clicking any action navigates to correct game tree node; back-navigation works | STU-008 |
| 9 | Metric overlays: Strategy, EV, Equity, EQR all display correct data; switching overlays updates display | STU-009 |
| 10 | EV display: per-action EV and overall range EV shown in chips/bb; frequency-weighted action EVs sum to overall EV | STU-010 |
| 11 | Keyboard shortcuts (J, S, P, Q, Space, 1-4) work within Study Mode without conflicting with text input | STU-012 |
| 12 | ICM equity calculation matches known references for standard payout structures (up to 9 players) | ICM-001 |
| 13 | ICM-adjusted solving produces higher fold frequency (≥ 5% increase) with medium-strength hands near bubble vs chip-EV solve on same spot | ICM-002 |
| 14 | Custom payout structures accepted; changing payouts changes ICM equity values | ICM-003 |
| 15 | Bubble factor displayed and > 1.0 for non-chip-leader players near the bubble | ICM-004 |
| 16 | UI interactions render at 60fps; no frames exceed 33ms during tab switching, matrix updates, node navigation | PRF-003 |
| 17 | Solution loads from disk and renders in < 500ms for files up to 500MB compressed | PRF-004 |

### Testable Artifact

Open a previously solved spot in Study Mode:
1. Use Spot Selector to filter and load a solution
2. Navigate all 4 tabs (Strategy, Ranges, Breakdown, Reports)
3. Change boards via Board Selector; navigate the action sequence tree
4. Switch between EV, Equity, and EQR overlays
5. Load a tournament spot with ICM; verify more conservative strategies vs chip-EV

### Key Risks

| Risk | Mitigation |
|------|------------|
| Board texture navigation across 1,755 flops is sluggish | Pre-index canonical flops with texture metadata; lazy-load strategies per board |
| ICM calculation too slow for 9-player stacks | Cache ICM equity results; Malmuth-Harville is O(n! / (n-k)!) — limit to 9 players |
| Study Mode state complexity leads to UI bugs | Per-mode `useReducer` state management isolates Study state; comprehensive tab-switching tests |

---

## 7. M4 — Practice Mode

**Priority:** P0 (MUST only)
**MUST requirements:** 9
**Depends on:** M3
**Parallel with:** M5

### Scope

Build Practice Mode (`practice/`). The vertical slice: a user selects a solved spot, plays practice hands against a GTO opponent, receives per-decision feedback with EV loss and classification, and views cumulative session statistics.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Frontend | `practice/` | GameTable, ActionButtons, FeedbackPanel, SessionStats, RngDice |

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Practice Mode | PRA-001, PRA-002, PRA-003, PRA-004, PRA-005, PRA-006, PRA-007, PRA-008, PRA-009 | 9 |
| **Total** | | **9** |

### Acceptance Criteria

| # | Criterion | Requirement |
|---|-----------|-------------|
| 1 | Three game modes available: Full Hand, Street, and Spot; each initializes at the correct starting point | PRA-001 |
| 2 | GTO opponent plays according to solver strategy; over 10,000 sampled decisions, action frequencies match solver within 2% | PRA-002 |
| 3 | Three difficulty levels (Simple, Grouped, Standard) present correct number of available actions per node | PRA-003 |
| 4 | Score tracking: EV loss computed as EV(GTO optimal) − EV(player choice); session average aggregates correctly | PRA-004 |
| 5 | Decision classification into 5 categories (Perfect, Good, Inaccurate, Wrong, Blunder) matches defined EV-loss thresholds | PRA-005 |
| 6 | Post-decision feedback shows complete GTO strategy at that node; player's chosen action is highlighted | PRA-006 |
| 7 | Three RNG modes (High, Low, Off) produce statistically correct hand strength distributions over 1,000 hands | PRA-007 |
| 8 | Spot configuration (game type, positions, stack depth, optional board cards) applied correctly; changing config restarts session | PRA-008 |
| 9 | Session statistics display: hands played, decisions made, average score, classification distribution; totals are internally consistent | PRA-009 |

### Testable Artifact

Play 10 practice hands against GTO on a solved spot:
1. Configure a practice session (spot, difficulty, RNG mode)
2. Play through hands making decisions at each node
3. After each decision, view GTO strategy comparison and EV loss
4. After the session, review cumulative statistics and classification distribution

### Key Risks

| Risk | Mitigation |
|------|------------|
| Mixed-strategy opponent feels non-random (clustering) | Use cryptographically seeded PRNG; display expected vs actual frequencies in debug mode |
| Practice session state lost on navigation | Persist session state in backend `AppState`; frontend restores on re-mount |

---

## 8. M5 — Hand History Analysis

**Priority:** P0 (MUST only)
**MUST requirements:** 18
**Depends on:** M3
**Parallel with:** M4

### Scope

Build the hand history parser (`poker-hhparser`), analysis engine (`poker-analyze`), and Analyze Mode UI (`analyze/`). The vertical slice: a user imports hand history files, the system parses and analyzes each decision against GTO, and displays a dashboard with EV loss, decision classification, and hand replay.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Rust | `poker-hhparser` | Multi-site parser (6 sites), format auto-detection, pot reconstruction, position assignment, error handling, encoding |
| Rust | `poker-analyze` | EV loss calculation, decision classification, dashboard aggregation (position, street, action breakdowns) |
| Frontend | `analyze/` | ImportWizard, HandsTable, HandReplay, Dashboard, FilterBar, GtoOverlay |

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Analyze Mode | ANL-001, ANL-002, ANL-003, ANL-004, ANL-005, ANL-006, ANL-007, ANL-008, ANL-009, ANL-010 | 10 |
| Hand History Parser | HHP-001, HHP-002, HHP-003, HHP-004, HHP-005, HHP-006, HHP-007, HHP-008 | 8 |
| **Total** | | **18** |

### Acceptance Criteria

| # | Criterion | Requirement |
|---|-----------|-------------|
| 1 | Parser supports PokerStars, GGPoker, Winamax, 888poker, PartyPoker, and iPoker formats; sample files from each parse without errors | HHP-001 |
| 2 | All fields extracted: game type, stakes, table size, positions, stacks, hole cards, community cards, actions with amounts | HHP-002 |
| 3 | Cash game hands parse correctly including blinds, antes, straddles, side pots, and rake | HHP-003 |
| 4 | Tournament hands parse with correct blind levels, antes, and bounty information | HHP-004 |
| 5 | Malformed hand histories: parser logs error, skips bad hand, continues processing remaining hands | HHP-005 |
| 6 | UTF-8 and Latin-1/Windows-1252 encoded files with non-ASCII player names parse without encoding errors | HHP-006 |
| 7 | Pot size reconstruction at each decision point matches hand history totals; verified for 10 test hands | HHP-007 |
| 8 | Position assignment correct for 6-max, 9-max, and heads-up formats | HHP-008 |
| 9 | Hand history import supports single files and batch directory import; mixed formats handled with per-file status | ANL-001, ANL-002 |
| 10 | Decision classification into 5 categories matches expected values for benchmark hands with known GTO solutions | ANL-003 |
| 11 | Per-decision EV loss: EV(GTO optimal) − EV(player action) ≥ 0 for all decisions; 0 when action matches GTO pure strategy | ANL-004 |
| 12 | Hand replay: step through hand street-by-street; forward, backward, and jump-to-street navigation work | ANL-005 |
| 13 | GTO comparison overlay: solver's recommended strategy displayed alongside player's actual action at each decision node | ANL-006 |
| 14 | Dashboard: total hands, average EV loss, decision classification distribution, worst errors displayed | ANL-007 |
| 15 | Position breakdown: per-position statistics consistent with overall totals; each hand assigned to exactly one position | ANL-008 |
| 16 | Street breakdown: per-street EV loss and classification distribution; multi-street hands contribute to each street | ANL-009 |
| 17 | Action type breakdown: per-action accuracy and EV loss; no actions uncategorized | ANL-010 |
| 18 | Import progress events stream to frontend during batch import | ANL-002 (progress) |

### Testable Artifact

Import a PokerStars hand history file:
1. Use Import Wizard to select file(s)
2. See import progress bar and per-file status
3. View analysis dashboard with aggregate statistics
4. Click into a specific hand; step through the hand replay
5. See GTO overlay at each decision point with EV loss and classification

### Key Risks

| Risk | Mitigation |
|------|------------|
| Poker site format changes break parser | Trait-based `SiteParser` architecture isolates changes; version-specific parser branches |
| Missing GTO solutions for analyzed hands | On-demand solving for unmatched spots; fallback to nearest available solution with disclaimer |
| Batch import of thousands of files is slow | Parallel file parsing via `rayon`; progress events stream per-file; cancellable import |

---

## 9. M6 — Nodelocking & Aggregated Reports

**Priority:** P0 MUST + P1 SHOULD (pulled forward)
**MUST requirements:** 10
**SHOULD requirements:** 6 (pulled from M7 for feature completeness)
**Depends on:** M4, M5

### Scope

Build Nodelocking (`nodelock/`) and Aggregated Reports (`reports/`). The vertical slice: a user locks an opponent's strategy at specific nodes, re-solves for the exploitative counter-strategy, compares GTO vs nodelock, and generates aggregate reports across multiple solved spots.

Six SHOULD requirements are pulled forward into M6 because they are tightly coupled with the MUST features — the frequency editor, batch locking, and overwrite modes complete the nodelocking workflow, while flop grouping, action grouping, and turn reports complete the aggregate reports feature.

### Components Built

| Layer | Component | From `architecture.md` |
|-------|-----------|----------------------|
| Frontend | `nodelock/` | LockControls, FrequencyEditor, ComparisonView, GameTreeNav, BatchLockPanel |
| Frontend | `reports/` | AggregateChart, AggregateTable, BoardTextureFilters, ActionGrouping |

### Requirements Covered

| Category | IDs | Priority | Count |
|----------|-----|----------|-------|
| Nodelocking | NLK-001, NLK-002, NLK-003, NLK-004, NLK-005, NLK-006 | MUST | 6 |
| Nodelocking | NLK-007, NLK-008, NLK-009 | SHOULD | 3 |
| Aggregated Reports | AGG-002, AGG-003, AGG-004, AGG-005 | MUST | 4 |
| Aggregated Reports | AGG-006, AGG-007, AGG-008 | SHOULD | 3 |
| **Total** | | | **16** (10 MUST + 6 SHOULD) |

### Acceptance Criteria

| # | Criterion | Requirement | Priority |
|---|-----------|-------------|----------|
| 1 | Lock any player's strategy at any decision node; locked node's strategy unchanged after re-solve | NLK-001 | MUST |
| 2 | Frequency editor: set custom action probabilities (e.g., 70% fold, 20% call, 10% raise); frequencies preserved after lock | NLK-002 | MUST |
| 3 | Re-solve with locked nodes produces exploitative counter-strategy with higher EV against locked strategy than GTO | NLK-003 | MUST |
| 4 | Locked nodes visually distinct from unlocked; lock/unlock/reset controls functional | NLK-004 | MUST |
| 5 | Cascade lock effects: range reaching downstream nodes correctly reflects locked strategy frequencies | NLK-005 | MUST |
| 6 | Comparison view: GTO vs nodelock-adjusted strategies side by side; differences highlighted | NLK-006 | MUST |
| 7 | Visual frequency editor (sliders + numeric input) with auto-adjustment to maintain 100% total | NLK-007 | SHOULD |
| 8 | Batch lock: apply same strategy modification to multiple selected nodes in one operation | NLK-008 | SHOULD |
| 9 | Two re-solve modes: Overwrite Unlocked and Overwrite All; selectable before re-solving | NLK-009 | SHOULD |
| 10 | Aggregate reports display four metric types: Strategy, EV, Equity, EQR | AGG-002 | MUST |
| 11 | Board texture filtering: monotone, two-tone, rainbow, paired, connected, high card rank | AGG-003 | MUST |
| 12 | Chart view (bar graph) with data labeled and colored by action | AGG-004 | MUST |
| 13 | Table view with numerical values, sortable by any column; data matches chart | AGG-005 | MUST |
| 14 | Flop grouping by high card, suit composition, pairing, connectedness; all 1,755 flops assigned to one group | AGG-006 | SHOULD |
| 15 | Action grouping at three levels: All Sizes, Grouped (small/medium/large), Simplified (bet/check) | AGG-007 | SHOULD |
| 16 | Turn aggregate reports show strategy across all valid turn cards for a given flop | AGG-008 | SHOULD |
| 17 | **Gate check:** All MUST requirements across M1–M6 verified as complete | All MUST | MUST |

### Testable Artifact

1. Open a solved spot in Nodelock mode
2. Lock the opponent's c-bet frequency to 80% on a specific flop
3. Re-solve; compare GTO vs exploit strategies side-by-side
4. Navigate to Reports mode; generate aggregate report across multiple solved spots
5. Filter by board texture; switch between chart and table views

### Key Risks

| Risk | Mitigation |
|------|------------|
| Nodelocked re-solve convergence is slow | Re-solve from GTO warm start (not from scratch); reduced iteration count since only one side optimizes |
| Aggregate reports across 1,755 flops memory-intensive | Stream aggregation — process one flop at a time; don't hold all in memory simultaneously |

---

## 10. M7 — SHOULD Enhancements

**Priority:** P1 (SHOULD only)
**SHOULD requirements:** 42

### Scope

Integrate all remaining SHOULD (P1) requirements not pulled into M6. These are enhancements to features already built in M1–M6.

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Solver Engine | SOL-013, SOL-014, SOL-015 | 3 |
| Hand Evaluation | HEV-008 | 1 |
| Solution Storage | STO-007, STO-008, STO-009 | 3 |
| Study Mode | STU-011, STU-013, STU-014, STU-015 | 4 |
| Practice Mode | PRA-010, PRA-011, PRA-012 | 3 |
| Analyze Mode | ANL-011, ANL-012, ANL-013, ANL-014 | 4 |
| Real-Time Solver | RTS-006, RTS-007 | 2 |
| Tournament & ICM | ICM-005, ICM-006, ICM-007 | 3 |
| Hand History Parser | HHP-009, HHP-010, HHP-011 | 3 |
| UI Framework | UIF-010, UIF-011, UIF-012, UIF-015 | 4 |
| Hand Matrix | HMX-010, HMX-011 | 2 |
| Desktop App | DSK-007, DSK-008, DSK-009 | 3 |
| Performance | PRF-006, PRF-007 | 2 |
| Data & Formats | DAT-005, DAT-006 | 2 |
| Range Builder | RNG-008, RNG-009 | 2 |
| Aggregated Reports | AGG-009 | 1 |
| **Total** | | **42** |

### Highlights by Feature Area

| Area | Enhancement | IDs |
|------|------------|-----|
| Solver | Preflop range solving, DCFR/Linear CFR variants, dynamic bet size discovery | SOL-013, SOL-014, SOL-015 |
| Evaluation | Batch evaluation mode for SIMD throughput | HEV-008 |
| Storage | Incremental save/checkpoint, solution import/export, format versioning | STO-007, STO-008, STO-009 |
| Study | Hand detail view, compare EV overlay, range weight display, flop subset browser | STU-011, STU-013, STU-014, STU-015 |
| Practice | Hand history review, multitabling (1-4 tables), game speed control | PRA-010, PRA-011, PRA-012 |
| Analysis | Hand filtering, biggest mistakes list, session summary, export results | ANL-011, ANL-012, ANL-013, ANL-014 |
| Solver (RT) | Depth-limited solving, dynamic sizing mode | RTS-006, RTS-007 |
| ICM | FT/SNG simulator, PKO bounty, satellite ICM | ICM-005, ICM-006, ICM-007 |
| Parser | Currency normalization, format auto-detection, incremental import | HHP-009, HHP-010, HHP-011 |
| UI | WCAG AA contrast, four-color deck, animations, theme customization | UIF-010, UIF-011, UIF-012, UIF-015 |
| Matrix | Suit expansion view, matrix sizing options | HMX-010, HMX-011 |
| Desktop | Windows primary platform optimization, auto-update, performance monitoring | DSK-007, DSK-008, DSK-009 |
| Performance | Startup time < 3s, near-linear multi-core scaling | PRF-006, PRF-007 |
| Data | Glossary compliance, solution format documentation | DAT-005, DAT-006 |
| Range | Grade vs GTO scoring, range import/export | RNG-008, RNG-009 |
| Reports | Filtered vs overall comparison | AGG-009 |

### Acceptance Criteria

Each SHOULD requirement's acceptance criterion from `requirements.md` passes individually. Full regression suite confirms no regressions to MUST features from M1–M6.

### Testable Artifact

All P1 features integrated into the application. Complete regression test suite passes. Each enhancement is individually demonstrable.

### Key Risks

| Risk | Mitigation |
|------|------------|
| Scope creep — SHOULD requirements expand during implementation | Strict adherence to requirements.md acceptance criteria; no new requirements added without document update |
| Feature interactions cause regressions | Comprehensive integration test suite; test each enhancement in isolation before merge |
| Auto-update mechanism introduces deployment complexity | Use Tauri's built-in updater; test update flow in CI with mock update server |

---

## 11. M8 — MAY Polish

**Priority:** P2 (MAY only)
**MAY requirements:** 17

### Scope

Integrate all MAY (P2) requirements — optional quality-of-life features and experimental capabilities.

### Requirements Covered

| Category | IDs | Count |
|----------|-----|-------|
| Solver Engine | SOL-017 | 1 |
| Hand Evaluation | HEV-010 | 1 |
| Solution Storage | STO-012 | 1 |
| Study Mode | STU-016 | 1 |
| Practice Mode | PRA-013, PRA-014 | 2 |
| Analyze Mode | ANL-015 | 1 |
| Real-Time Solver | RTS-008 | 1 |
| Nodelocking | NLK-010 | 1 |
| Aggregated Reports | AGG-010 | 1 |
| Tournament & ICM | ICM-008 | 1 |
| Hand History Parser | HHP-012 | 1 |
| UI Framework | UIF-013, UIF-014 | 2 |
| Hand Matrix | HMX-012 | 1 |
| Desktop App | DSK-010 | 1 |
| Performance | PRF-008 | 1 |
| Data & Formats | DAT-007 | 1 |
| **Total** | | **17** |

### Feature Summary

| Feature | ID | Description |
|---------|-----|-------------|
| 3-way pot solving | SOL-017 | Experimental multi-player solver |
| Evaluation caching | HEV-010 | Cache recent board+hand evaluation results |
| Solution integrity check | STO-012 | xxHash64 checksum verification |
| Solution comparison | STU-016 | Side-by-side strategy comparison for different configurations |
| Timebank | PRA-013 | Decision timer with configurable durations |
| Performance trends | PRA-014 | Historical session score trends |
| Comparative analysis | ANL-015 | Two-period improvement comparison |
| Background solving | RTS-008 | Queue multiple solve requests |
| Reset to GTO | NLK-010 | One-click nodelock reset |
| Report export | AGG-010 | Export charts/tables to PNG/SVG/CSV/JSON |
| Mystery bounty | ICM-008 | Distribution-based bounty calculation |
| HH validation | HHP-012 | Internal consistency checks on parsed hands |
| Context menus | UIF-013 | Right-click context menus |
| Zoom and pan | UIF-014 | Game tree zoom/pan navigation |
| Range notation label | HMX-012 | Text summary of displayed range |
| Crash recovery | DSK-010 | Periodic state save + restore |
| Large solution support | PRF-008 | > 4GB solutions via lazy mmap |
| Config file format | DAT-007 | Human-readable TOML configuration |

### Acceptance Criteria

Each MAY requirement's acceptance criterion from `requirements.md` passes individually. No regressions to MUST or SHOULD features.

### Testable Artifact

Final product with all implemented MAY features passing their individual acceptance criteria.

---

## 12. Requirement Coverage Matrix

Every requirement ID is assigned to exactly one milestone. This matrix is the authoritative mapping.

### MUST Requirements (M1–M6)

| ID | Priority | Milestone | Category |
|----|----------|-----------|----------|
| DAT-001 | MUST | M1 | Data & Formats |
| DAT-002 | MUST | M1 | Data & Formats |
| DAT-003 | MUST | M1 | Data & Formats |
| DAT-004 | MUST | M1 | Data & Formats |
| HEV-001 | MUST | M1 | Hand Evaluation |
| HEV-002 | MUST | M1 | Hand Evaluation |
| HEV-003 | MUST | M1 | Hand Evaluation |
| HEV-004 | MUST | M1 | Hand Evaluation |
| HEV-005 | MUST | M1 | Hand Evaluation |
| HEV-006 | MUST | M1 | Hand Evaluation |
| HEV-007 | MUST | M1 | Hand Evaluation |
| HEV-009 | MUST | M1 | Hand Evaluation |
| HMX-001 | MUST | M1 | Hand Matrix |
| HMX-002 | MUST | M1 | Hand Matrix |
| HMX-003 | MUST | M1 | Hand Matrix |
| HMX-004 | MUST | M1 | Hand Matrix |
| HMX-005 | MUST | M1 | Hand Matrix |
| HMX-006 | MUST | M1 | Hand Matrix |
| HMX-007 | MUST | M1 | Hand Matrix |
| HMX-008 | MUST | M1 | Hand Matrix |
| HMX-009 | MUST | M1 | Hand Matrix |
| RNG-001 | MUST | M1 | Range Builder |
| RNG-002 | MUST | M1 | Range Builder |
| RNG-003 | MUST | M1 | Range Builder |
| RNG-004 | MUST | M1 | Range Builder |
| RNG-005 | MUST | M1 | Range Builder |
| RNG-006 | MUST | M1 | Range Builder |
| RNG-007 | MUST | M1 | Range Builder |
| RNG-010 | MUST | M1 | Range Builder |
| UIF-001 | MUST | M1 | UI Framework |
| UIF-002 | MUST | M1 | UI Framework |
| UIF-003 | MUST | M1 | UI Framework |
| UIF-004 | MUST | M1 | UI Framework |
| UIF-005 | MUST | M1 | UI Framework |
| UIF-006 | MUST | M1 | UI Framework |
| UIF-007 | MUST | M1 | UI Framework |
| UIF-008 | MUST | M1 | UI Framework |
| UIF-009 | MUST | M1 | UI Framework |
| DSK-001 | MUST | M1 | Desktop App |
| DSK-002 | MUST | M1 | Desktop App |
| DSK-003 | MUST | M1 | Desktop App |
| DSK-004 | MUST | M1 | Desktop App |
| DSK-005 | MUST | M1 | Desktop App |
| DSK-006 | MUST | M1 | Desktop App |
| PRF-001 | MUST | M1 | Performance |
| SOL-001 | MUST | M2 | Solver Engine |
| SOL-002 | MUST | M2 | Solver Engine |
| SOL-003 | MUST | M2 | Solver Engine |
| SOL-004 | MUST | M2 | Solver Engine |
| SOL-005 | MUST | M2 | Solver Engine |
| SOL-006 | MUST | M2 | Solver Engine |
| SOL-007 | MUST | M2 | Solver Engine |
| SOL-008 | MUST | M2 | Solver Engine |
| SOL-009 | MUST | M2 | Solver Engine |
| SOL-010 | MUST | M2 | Solver Engine |
| SOL-011 | MUST | M2 | Solver Engine |
| SOL-012 | MUST | M2 | Solver Engine |
| SOL-016 | MUST | M2 | Solver Engine |
| SOL-018 | MUST | M2 | Solver Engine |
| STO-001 | MUST | M2 | Solution Storage |
| STO-002 | MUST | M2 | Solution Storage |
| STO-003 | MUST | M2 | Solution Storage |
| STO-004 | MUST | M2 | Solution Storage |
| STO-005 | MUST | M2 | Solution Storage |
| STO-006 | MUST | M2 | Solution Storage |
| STO-010 | MUST | M2 | Solution Storage |
| STO-011 | MUST | M2 | Solution Storage |
| RTS-001 | MUST | M2 | Real-Time Solver |
| RTS-002 | MUST | M2 | Real-Time Solver |
| RTS-003 | MUST | M2 | Real-Time Solver |
| RTS-004 | MUST | M2 | Real-Time Solver |
| RTS-005 | MUST | M2 | Real-Time Solver |
| PRF-002 | MUST | M2 | Performance |
| PRF-005 | MUST | M2 | Performance |
| STU-001 | MUST | M3 | Study Mode |
| STU-002 | MUST | M3 | Study Mode |
| STU-003 | MUST | M3 | Study Mode |
| STU-004 | MUST | M3 | Study Mode |
| STU-005 | MUST | M3 | Study Mode |
| STU-006 | MUST | M3 | Study Mode |
| STU-007 | MUST | M3 | Study Mode |
| STU-008 | MUST | M3 | Study Mode |
| STU-009 | MUST | M3 | Study Mode |
| STU-010 | MUST | M3 | Study Mode |
| STU-012 | MUST | M3 | Study Mode |
| ICM-001 | MUST | M3 | Tournament & ICM |
| ICM-002 | MUST | M3 | Tournament & ICM |
| ICM-003 | MUST | M3 | Tournament & ICM |
| ICM-004 | MUST | M3 | Tournament & ICM |
| AGG-001 | MUST | M3 | Aggregated Reports |
| PRF-003 | MUST | M3 | Performance |
| PRF-004 | MUST | M3 | Performance |
| PRA-001 | MUST | M4 | Practice Mode |
| PRA-002 | MUST | M4 | Practice Mode |
| PRA-003 | MUST | M4 | Practice Mode |
| PRA-004 | MUST | M4 | Practice Mode |
| PRA-005 | MUST | M4 | Practice Mode |
| PRA-006 | MUST | M4 | Practice Mode |
| PRA-007 | MUST | M4 | Practice Mode |
| PRA-008 | MUST | M4 | Practice Mode |
| PRA-009 | MUST | M4 | Practice Mode |
| ANL-001 | MUST | M5 | Analyze Mode |
| ANL-002 | MUST | M5 | Analyze Mode |
| ANL-003 | MUST | M5 | Analyze Mode |
| ANL-004 | MUST | M5 | Analyze Mode |
| ANL-005 | MUST | M5 | Analyze Mode |
| ANL-006 | MUST | M5 | Analyze Mode |
| ANL-007 | MUST | M5 | Analyze Mode |
| ANL-008 | MUST | M5 | Analyze Mode |
| ANL-009 | MUST | M5 | Analyze Mode |
| ANL-010 | MUST | M5 | Analyze Mode |
| HHP-001 | MUST | M5 | Hand History Parser |
| HHP-002 | MUST | M5 | Hand History Parser |
| HHP-003 | MUST | M5 | Hand History Parser |
| HHP-004 | MUST | M5 | Hand History Parser |
| HHP-005 | MUST | M5 | Hand History Parser |
| HHP-006 | MUST | M5 | Hand History Parser |
| HHP-007 | MUST | M5 | Hand History Parser |
| HHP-008 | MUST | M5 | Hand History Parser |
| NLK-001 | MUST | M6 | Nodelocking |
| NLK-002 | MUST | M6 | Nodelocking |
| NLK-003 | MUST | M6 | Nodelocking |
| NLK-004 | MUST | M6 | Nodelocking |
| NLK-005 | MUST | M6 | Nodelocking |
| NLK-006 | MUST | M6 | Nodelocking |
| AGG-002 | MUST | M6 | Aggregated Reports |
| AGG-003 | MUST | M6 | Aggregated Reports |
| AGG-004 | MUST | M6 | Aggregated Reports |
| AGG-005 | MUST | M6 | Aggregated Reports |

### SHOULD Requirements (M6 + M7)

| ID | Milestone | Category |
|----|-----------|----------|
| NLK-007 | M6 | Nodelocking |
| NLK-008 | M6 | Nodelocking |
| NLK-009 | M6 | Nodelocking |
| AGG-006 | M6 | Aggregated Reports |
| AGG-007 | M6 | Aggregated Reports |
| AGG-008 | M6 | Aggregated Reports |
| SOL-013 | M7 | Solver Engine |
| SOL-014 | M7 | Solver Engine |
| SOL-015 | M7 | Solver Engine |
| HEV-008 | M7 | Hand Evaluation |
| STO-007 | M7 | Solution Storage |
| STO-008 | M7 | Solution Storage |
| STO-009 | M7 | Solution Storage |
| STU-011 | M7 | Study Mode |
| STU-013 | M7 | Study Mode |
| STU-014 | M7 | Study Mode |
| STU-015 | M7 | Study Mode |
| PRA-010 | M7 | Practice Mode |
| PRA-011 | M7 | Practice Mode |
| PRA-012 | M7 | Practice Mode |
| ANL-011 | M7 | Analyze Mode |
| ANL-012 | M7 | Analyze Mode |
| ANL-013 | M7 | Analyze Mode |
| ANL-014 | M7 | Analyze Mode |
| RTS-006 | M7 | Real-Time Solver |
| RTS-007 | M7 | Real-Time Solver |
| ICM-005 | M7 | Tournament & ICM |
| ICM-006 | M7 | Tournament & ICM |
| ICM-007 | M7 | Tournament & ICM |
| HHP-009 | M7 | Hand History Parser |
| HHP-010 | M7 | Hand History Parser |
| HHP-011 | M7 | Hand History Parser |
| UIF-010 | M7 | UI Framework |
| UIF-011 | M7 | UI Framework |
| UIF-012 | M7 | UI Framework |
| UIF-015 | M7 | UI Framework |
| HMX-010 | M7 | Hand Matrix |
| HMX-011 | M7 | Hand Matrix |
| DSK-007 | M7 | Desktop App |
| DSK-008 | M7 | Desktop App |
| DSK-009 | M7 | Desktop App |
| PRF-006 | M7 | Performance |
| PRF-007 | M7 | Performance |
| DAT-005 | M7 | Data & Formats |
| DAT-006 | M7 | Data & Formats |
| RNG-008 | M7 | Range Builder |
| RNG-009 | M7 | Range Builder |
| AGG-009 | M7 | Aggregated Reports |

### MAY Requirements (M8)

| ID | Milestone | Category |
|----|-----------|----------|
| SOL-017 | M8 | Solver Engine |
| HEV-010 | M8 | Hand Evaluation |
| STO-012 | M8 | Solution Storage |
| STU-016 | M8 | Study Mode |
| PRA-013 | M8 | Practice Mode |
| PRA-014 | M8 | Practice Mode |
| ANL-015 | M8 | Analyze Mode |
| RTS-008 | M8 | Real-Time Solver |
| NLK-010 | M8 | Nodelocking |
| AGG-010 | M8 | Aggregated Reports |
| ICM-008 | M8 | Tournament & ICM |
| HHP-012 | M8 | Hand History Parser |
| UIF-013 | M8 | UI Framework |
| UIF-014 | M8 | UI Framework |
| HMX-012 | M8 | Hand Matrix |
| DSK-010 | M8 | Desktop App |
| PRF-008 | M8 | Performance |
| DAT-007 | M8 | Data & Formats |

### Coverage Verification

| Check | Result |
|-------|--------|
| Total requirement IDs assigned | 195 |
| IDs appearing in more than one milestone | 0 |
| MUST requirements in M7 or M8 | 0 |
| SHOULD requirements in M1–M5 | 0 |
| MAY requirements in M1–M7 | 0 |

---

## 13. Data Flow Activation by Milestone

The 4 data flows defined in `architecture.md` Section 6 become functional at specific milestones:

| Data Flow | Activated At | Description | Key Milestones Required |
|-----------|-------------|-------------|------------------------|
| **Flow 1: Custom Solve** | **M2** | User configures spot → solver runs → progress streams → result saved | M1 (poker-core, poker-eval) + M2 (poker-solver, poker-solution) |
| **Flow 2: Solution Browse** | **M3** | User selects spot → solution loaded (mmap) → strategy displayed in Study Mode | M2 (poker-solution) + M3 (study/) |
| **Flow 3: Practice Session** | **M4** | User configures practice → solution loaded → hands dealt → feedback per decision | M3 (solution loading) + M4 (practice/) |
| **Flow 4: HH Analysis** | **M5** | User imports files → parser extracts hands → analyzer computes EV loss → dashboard displayed | M2 (poker-solution) + M5 (poker-hhparser, poker-analyze, analyze/) |

### Cumulative System Capability by Milestone

```
M1  ██░░░░░░  Foundation: evaluate hands, render matrix, build ranges
M2  ████░░░░  + Solve spots, persist/reload solutions
M3  █████░░░  + Browse solutions in Study Mode (all 4 tabs, ICM)
M4  ██████░░  + Practice against GTO with feedback
M5  ██████░░  + Import and analyze hand histories (parallel with M4)
M6  ███████░  + Nodelock exploits, aggregate reports; all MUST complete
M7  ████████  + All SHOULD enhancements
M8  ████████  + All MAY polish features; final product
```

---

## 14. Risk Register

### Per-Milestone Risks

| Milestone | Risk | Impact | Likelihood | Mitigation |
|-----------|------|--------|------------|------------|
| M1 | Lookup table generation takes > 10 seconds on first run | User experience | Low | Pre-compute at build time; cache to `eval_tables.bin`; show progress bar |
| M1 | Canvas-based matrix performance poor on integrated GPUs | UI unusable on low-end hardware | Medium | Benchmark on Intel UHD; fallback to DOM renderer if needed |
| M1 | Tauri v2 webview inconsistencies across platforms | UI rendering bugs | Medium | Target Windows primary (DSK-007); test on Windows WebView2 early |
| M2 | CFR convergence too slow for complex multi-street trees | Solve times exceed 10-second target | Medium | MCCFR with chance sampling; card abstraction reduces tree size; depth-limited fallback |
| M2 | Memory exceeds 4GB for standard spots | OOM crashes on 8GB machines | Medium | MCCFR bounds memory by design; configurable memory limit; abort if limit approached |
| M2 | Parallel solver non-determinism | Inconsistent solve results | Low | Lock-free atomics with deterministic seed option; non-determinism bounded by Nash Distance tolerance |
| M3 | 1,755 flop navigation latency | Board browsing feels sluggish | Low | Pre-index canonical flops with metadata; lazy-load strategies per board selection |
| M3 | ICM calculation for 9 players is slow (factorial complexity) | UI freezes during ICM computation | Low | Cache results; limit to 9 players; spawn to rayon thread |
| M4 | Practice opponent feels non-random due to mixed-strategy clustering | User perceives bias | Low | Cryptographic PRNG seeding; transparency option showing expected frequencies |
| M5 | Poker site format changes break parsers | Import failures for users | High | Trait-based `SiteParser` isolates changes; parser versioning; user-reported format samples |
| M5 | Missing GTO solution for specific hand history spot | Cannot compute EV loss | Medium | On-demand solving for unmatched spots; nearest-match fallback with disclaimer |
| M4/M5 | Parallel development integration conflicts | Merge conflicts; broken builds | Medium | Shared IPC types defined in M3; CI runs both feature branches; shared `ipc/` module stable |
| M6 | Nodelocked re-solve convergence slow | Long wait for exploit strategy | Low | Warm-start from GTO solution; only one side optimizes; reduced iteration requirement |
| M7 | Scope creep — SHOULD requirements expand | Schedule overrun | Medium | Strict adherence to requirements.md text; no new requirements without document update |
| M7 | Feature interactions cause regressions | Broken MUST features | Medium | Comprehensive regression suite run before each M7 merge; feature flags for SHOULD features |
| M8 | 3-way pot solver (SOL-017) is significantly harder | Implementation incomplete | High | Time-box effort; mark as experimental; ship without if necessary |

### Cross-Cutting Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Tauri v2 breaking changes during development | Rework IPC and shell code | Pin Tauri version; update only during planned maintenance windows |
| SQLite performance degradation with large hand databases (100K+ hands) | Slow dashboard queries | Index-first schema design; WAL mode; `EXPLAIN QUERY PLAN` audits; consider pagination |
| Solution file format changes between milestones | Incompatible old solutions | Version field in binary header (STO-009 in M7); migration code for format changes |

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial milestones and build sequence document |

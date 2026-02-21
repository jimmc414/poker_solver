# Project Structure

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 4 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisites:** [Requirements Specification](requirements.md) (Doc 1), [System Architecture](architecture.md) (Doc 2), [Milestones](milestones.md) (Doc 3)

---

## 1. Scope & Purpose

### What This Document Defines

This document specifies the **exact file and directory layout** for the GTO Poker Solver workspace, annotated by milestone. For every file: when it is created, what it exports, and what it may import. For every crate and module: its public API boundary and its internal organization.

### What This Document Does NOT Define

- Implementation details (algorithms, data structures) — see `architecture.md` and `agent-prompt.md`
- Test file contents — see `test-strategy.md`
- Build configuration details — see `CLAUDE.md`

### Relationship to Source Documents

| Source | What This Document Uses |
|--------|------------------------|
| `architecture.md` §4 | 8 Rust crates: names, responsibilities, dependencies, public API surfaces |
| `architecture.md` §5 | 9 frontend modules: directories, component lists, state management |
| `architecture.md` §7 | IPC command namespaces and payload types |
| `architecture.md` §10 | Storage directory layout (`~/.poker-solver/`) |
| `architecture.md` §15 | Crate dependency DAG and frontend module dependencies |
| `milestones.md` §4–§11 | Components built per milestone, crate build order |
| `milestones.md` §3 | Crate build order: `poker-core` ← M1, `poker-eval` ← M1, etc. |

---

## 2. Workspace Root Layout

```
poker-solver/                          # Cargo workspace root
├── Cargo.toml                         [M1] Workspace manifest
├── Cargo.lock                         [M1] Dependency lock file
├── .gitignore                         [M1]
├── CLAUDE.md                          [M1] Agent/AI configuration (Build Doc 8)
├── README.md                          [M1] Project overview
│
├── crates/                            # All Rust library + binary crates
│   ├── poker-core/                    [M1]
│   ├── poker-eval/                    [M1]
│   ├── poker-solver/                  [M2]
│   ├── poker-solution/                [M2]
│   ├── poker-icm/                     [M3]
│   ├── poker-hhparser/                [M5]
│   ├── poker-analyze/                 [M5]
│   └── app-tauri/                     [M1] skeleton, grows M2–M6
│
├── frontend/                          # TypeScript/React application
│   ├── package.json                   [M1]
│   ├── tsconfig.json                  [M1]
│   ├── vite.config.ts                 [M1]
│   ├── index.html                     [M1]
│   └── src/
│       ├── main.tsx                   [M1] React entry point
│       ├── App.tsx                    [M1] Root component with AppContext
│       ├── shared/                    [M1]
│       ├── ipc/                       [M1]
│       ├── range-builder/             [M1]
│       ├── solve/                     [M2]
│       ├── study/                     [M3]
│       ├── practice/                  [M4]
│       ├── analyze/                   [M5]
│       ├── nodelock/                  [M6]
│       └── reports/                   [M6]
│
├── docs/
│   └── build/                         # Build specification documents (this file)
│       ├── requirements.md            Doc 1
│       ├── architecture.md            Doc 2
│       ├── milestones.md              Doc 3
│       ├── project-structure.md       Doc 4 (this file)
│       ├── technical-constraints.md   Doc 5
│       ├── test-strategy.md           Doc 6
│       └── agent-prompt.md            Doc 7
│
└── fixtures/                          # Test fixtures shared across crates
    ├── hands/                         [M5] Sample hand history files per site
    ├── solutions/                     [M2] Reference solution files for testing
    └── eval/                          [M1] Hand evaluation reference data
```

---

## 3. Rust Crates — Per-Crate File Layout

### 3.1 poker-core [M1]

**Role:** Leaf crate. Foundational poker types and rules. No workspace dependencies.

```
crates/poker-core/
├── Cargo.toml                         [M1]
└── src/
    ├── lib.rs                         [M1] Re-exports all public types
    ├── card.rs                        [M1] Card, Rank, Suit — packed u8
    ├── deck.rs                        [M1] Deck with Fisher-Yates shuffle
    ├── hand.rs                        [M1] Hand (2 hole cards), canonical index
    ├── board.rs                       [M1] Board (0–5 community cards)
    ├── action.rs                      [M1] Action enum: Fold, Check, Call, Bet, Raise, AllIn
    ├── position.rs                    [M1] Position enum (UTG..BB), table size helpers
    ├── game_type.rs                   [M1] GameType enum: CashNLH, MTT, SNG, SpinAndGo, HeadsUp
    ├── bet_size.rs                    [M1] BetSize: PotFraction, Absolute, AllIn
    ├── range.rs                       [M1] Range type — 169-element weight array
    └── error.rs                       [M1] CoreError enum
```

**Public API boundary (`pub` items):**
- All types in every module are `pub` — this is the shared vocabulary crate
- Functions: `card()`, `hand_notation()`, `is_suited()`, `canonical_hand_index()`
- Traits: none (pure data types)

**`pub(crate)` items:** None expected — everything is public by design.

**Requirements:** DAT-001, DAT-002, DAT-003, DAT-004

### 3.2 poker-eval [M1]

**Role:** Hand evaluation, equity, draws, blockers, suit isomorphism.

```
crates/poker-eval/
├── Cargo.toml                         [M1]
└── src/
    ├── lib.rs                         [M1] Re-exports; HandEvaluator trait definition
    ├── lookup_table.rs                [M1] LookupTableEvaluator — precomputed tables
    ├── table_gen.rs                   [M1] Table generation (build-time or first-run)
    ├── hand_rank.rs                   [M1] HandRank type, HandCategory enum
    ├── equity.rs                      [M1] equity(), equity_batch() — range vs range
    ├── draws.rs                       [M1] detect_draws() — DrawType enum
    ├── blockers.rs                    [M1] blockers() — BlockerInfo
    ├── isomorphism.rs                 [M1] suit_isomorphism() — 22,100 → 1,755
    └── batch.rs                       [M7] Batch evaluation for SIMD (HEV-008)
```

**Public API boundary:**
- `pub trait HandEvaluator` — evaluate, evaluate_5, categorize, compare
- `pub struct LookupTableEvaluator` — the default implementation
- `pub fn equity()`, `pub fn equity_batch()`, `pub fn detect_draws()`, `pub fn blockers()`, `pub fn suit_isomorphism()`
- Types: `HandRank`, `HandCategory`, `DrawType`, `BlockerInfo`, `CanonicalBoard`

**`pub(crate)` items:** `table_gen` internals, raw lookup arrays

**Dependencies:** `poker-core`

**Requirements:** HEV-001 through HEV-010, PRF-001

### 3.3 poker-solver [M2]

**Role:** CFR engine, game tree, MCCFR, parallel solving, card abstraction.

```
crates/poker-solver/
├── Cargo.toml                         [M2]
└── src/
    ├── lib.rs                         [M2] Re-exports; SolverAlgorithm trait
    ├── game_tree.rs                   [M2] GameTree (arena), GameNode enum, tree builder
    ├── info_set.rs                    [M2] InfoSet — player information state
    ├── strategy.rs                    [M2] Strategy — per-info-set action probabilities
    ├── cfr_plus.rs                    [M2] CfrPlusSolver implementation
    ├── mccfr.rs                       [M2] Monte Carlo CFR — chance sampling
    ├── abstraction.rs                 [M2] Card abstraction / equity bucketing
    ├── config.rs                      [M2] SolverConfig, StoppingCondition
    ├── progress.rs                    [M2] SolveProgress, CancellationToken
    ├── parallel.rs                    [M2] Rayon parallel tree traversal
    ├── nodelock.rs                    [M6] Nodelocking — lock/unlock nodes, re-solve
    ├── dcfr.rs                        [M7] DiscountedCfrSolver (SOL-014)
    ├── linear_cfr.rs                  [M7] LinearCfrSolver (SOL-014)
    ├── preflop.rs                     [M7] Preflop range solving (SOL-013)
    └── dynamic_sizing.rs              [M7] Dynamic bet size discovery (SOL-015)
```

**Public API boundary:**
- `pub trait SolverAlgorithm` — iterate, get_strategy, nash_distance
- `pub fn build_game_tree()`, `pub fn solve()`
- `pub struct SolverConfig`, `pub struct SolveProgress`, `pub struct CancellationToken`
- `pub struct GameTree`, `pub enum GameNode`
- Types: `InfoSet`, `Strategy`, `SolveResult`

**`pub(crate)` items:** Internal tree traversal helpers, regret table structures, parallel work distribution

**Dependencies:** `poker-core`, `poker-eval`

**Requirements:** SOL-001 through SOL-018, PRF-002, PRF-005, PRF-007

### 3.4 poker-solution [M2]

**Role:** Solution file I/O — serialization, compression, mmap, indexing, caching.

```
crates/poker-solution/
├── Cargo.toml                         [M2]
└── src/
    ├── lib.rs                         [M2] Re-exports
    ├── format.rs                      [M2] Binary format constants, magic bytes, version
    ├── header.rs                      [M2] SolutionHeader — magic, version, flags
    ├── metadata.rs                    [M2] SolutionMetadata — solver config, Nash Distance
    ├── serialize.rs                   [M2] serialize() — SolveResult → binary
    ├── compress.rs                    [M2] compress() / decompress() — zstd wrapper
    ├── quantize.rs                    [M2] Strategy quantization: f32 → u16
    ├── sparse.rs                      [M2] Sparse encoding for skewed nodes
    ├── reader.rs                      [M2] SolutionReader — mmap, partial decompression
    ├── index.rs                       [M2] SolutionIndex — SQLite-backed index
    ├── cache.rs                       [M2] SolutionCache — LRU, bounded memory
    ├── checksum.rs                    [M8] xxHash64 integrity check (STO-012)
    └── migration.rs                   [M7] Format version migration (STO-009)
```

**Public API boundary:**
- `pub fn serialize()`, `pub fn compress()`, `pub fn decompress()`
- `pub struct SolutionReader` — open, metadata, load_node, verify_checksum
- `pub struct SolutionIndex` — search, add, remove, total_disk_usage
- `pub struct SolutionCache`
- Types: `SolutionHeader`, `SolutionMetadata`, `SolutionEntry`, `SolutionQuery`

**`pub(crate)` items:** Raw binary layout helpers, compression internals, mmap page management

**Dependencies:** `poker-core`

**Note:** `poker-solution` does NOT depend on `poker-solver`. It works with serialized data, not live solver state. The `app-tauri` crate bridges them.

**Requirements:** STO-001 through STO-012, PRF-004

### 3.5 poker-icm [M3]

**Role:** ICM equity, bubble factor, PKO, satellite dynamics.

```
crates/poker-icm/
├── Cargo.toml                         [M3]
└── src/
    ├── lib.rs                         [M3] Re-exports; IcmModel trait definition
    ├── standard.rs                    [M3] StandardIcm — Malmuth-Harville model
    ├── bubble.rs                      [M3] Bubble factor calculation
    ├── types.rs                       [M3] PayoutStructure, StackDistribution, IcmEquity
    ├── pko.rs                         [M7] PkoIcm — Progressive Knockout (ICM-006)
    ├── satellite.rs                   [M7] SatelliteIcm — equal-prize (ICM-007)
    ├── simulator.rs                   [M7] FT/SNG Monte Carlo simulator (ICM-005)
    └── mystery.rs                     [M8] Mystery Bounty support (ICM-008)
```

**Public API boundary:**
- `pub trait IcmModel` — equity, bubble_factor
- `pub struct StandardIcm`, `pub struct PkoIcm`, `pub struct SatelliteIcm`
- Types: `PayoutStructure`, `StackDistribution`, `IcmEquity`, `BubbleFactor`

**`pub(crate)` items:** Internal permutation helpers, caching structures

**Dependencies:** `poker-core`

**Requirements:** ICM-001 through ICM-008

### 3.6 poker-hhparser [M5]

**Role:** Hand history parsing, multi-site support, format detection, pot reconstruction.

```
crates/poker-hhparser/
├── Cargo.toml                         [M5]
└── src/
    ├── lib.rs                         [M5] Re-exports; SiteParser trait definition
    ├── registry.rs                    [M5] ParserRegistry — auto-detection, registration
    ├── types.rs                       [M5] ParsedHand, PlayerAction, SiteFormat
    ├── pot.rs                         [M5] reconstruct_pot() — pot tracking per action
    ├── position.rs                    [M5] assign_positions() — seat → position label
    ├── encoding.rs                    [M5] Character encoding detection/conversion
    ├── sites/
    │   ├── mod.rs                     [M5] Site parser module index
    │   ├── pokerstars.rs              [M5] PokerStars format parser
    │   ├── ggpoker.rs                 [M5] GGPoker format parser
    │   ├── winamax.rs                 [M5] Winamax format parser
    │   ├── poker888.rs                [M5] 888poker format parser
    │   ├── partypoker.rs              [M5] PartyPoker format parser
    │   └── ipoker.rs                  [M5] iPoker network parser
    ├── normalize.rs                   [M7] Currency/chip normalization (HHP-009)
    ├── auto_detect.rs                 [M7] Format auto-detection improvements (HHP-010)
    ├── incremental.rs                 [M7] Incremental import (HHP-011)
    └── validate.rs                    [M8] Hand history validation (HHP-012)
```

**Public API boundary:**
- `pub trait SiteParser` — detect, parse, site_name
- `pub struct ParserRegistry` — auto_detect, register
- `pub fn parse_file()`, `pub fn parse_directory()`
- `pub fn reconstruct_pot()`, `pub fn assign_positions()`
- Types: `ParsedHand`, `PlayerAction`, `SiteFormat`, `BatchParseResult`

**`pub(crate)` items:** Per-site parser internals, regex patterns, line-by-line state machines

**Dependencies:** `poker-core`

**Requirements:** HHP-001 through HHP-012

### 3.7 poker-analyze [M5]

**Role:** Decision classification, EV loss, dashboard aggregation. Orchestrates solver + evaluator + solution + parser.

```
crates/poker-analyze/
├── Cargo.toml                         [M5]
└── src/
    ├── lib.rs                         [M5] Re-exports
    ├── classify.rs                    [M5] classify_decision() — 5 categories
    ├── ev_loss.rs                     [M5] compute_ev_loss() — per-decision EV loss
    ├── analyze.rs                     [M5] analyze_hand() — full hand analysis
    ├── dashboard.rs                   [M5] aggregate_stats(), position/street/action breakdowns
    ├── types.rs                       [M5] DecisionClassification, AnalyzedDecision, AnalyzedHand, DashboardStats
    ├── filter.rs                      [M7] Hand filtering (ANL-011)
    └── export.rs                      [M7] Export to CSV/JSON (ANL-014)
```

**Public API boundary:**
- `pub fn analyze_hand()`, `pub fn classify_decision()`, `pub fn compute_ev_loss()`
- `pub fn aggregate_stats()`, `pub fn position_breakdown()`, `pub fn street_breakdown()`, `pub fn action_breakdown()`
- Types: `DecisionClassification`, `AnalyzedDecision`, `AnalyzedHand`, `DashboardStats`

**`pub(crate)` items:** Internal EV computation helpers, spot-matching heuristics

**Dependencies:** `poker-core`, `poker-eval`, `poker-solver`, `poker-solution`, `poker-hhparser`

**Requirements:** ANL-003, ANL-004, ANL-007 through ANL-010

### 3.8 app-tauri [M1 skeleton, grows M2–M6]

**Role:** Tauri binary crate. IPC command handlers, event streaming, application lifecycle.

```
crates/app-tauri/
├── Cargo.toml                         [M1]
├── tauri.conf.json                    [M1] Tauri configuration
├── build.rs                           [M1] Tauri build script
├── icons/                             [M1] Application icons
└── src/
    ├── main.rs                        [M1] Tauri entry point, plugin registration
    ├── state.rs                       [M1] AppState — managed state container
    ├── error.rs                       [M1] AppError enum — serializable errors
    ├── setup.rs                       [M1] First-run init: dirs, DB schema, config
    ├── commands/
    │   ├── mod.rs                     [M1] Command module index
    │   ├── settings.rs                [M1] settings:: namespace commands
    │   ├── eval.rs                    [M1] eval:: namespace — hand evaluation IPC
    │   ├── range.rs                   [M1] range:: namespace — equity, presets
    │   ├── solver.rs                  [M2] solver:: namespace — start/cancel/status
    │   ├── solution.rs                [M2] solution:: namespace — load/search/delete
    │   ├── practice.rs                [M4] practice:: namespace — start/act/stats
    │   ├── handhistory.rs             [M5] handhistory:: namespace — import
    │   ├── analyze.rs                 [M5] analyze:: namespace — dashboard/filter
    │   ├── nodelock.rs                [M6] nodelock:: namespace — lock/re-solve
    │   ├── reports.rs                 [M6] reports:: namespace — aggregation
    │   └── icm.rs                     [M3] icm:: namespace — equity/bubble
    ├── events.rs                      [M2] Event streaming: solve_progress, import_progress
    └── db.rs                          [M1] SQLite schema init, connection pool
```

**Public API boundary:** All `#[tauri::command]` functions are public to the frontend via IPC. No Rust `pub` exports — this is a binary crate.

**Dependencies:** All library crates (added incrementally per milestone)

**Requirements:** DSK-001 through DSK-010

---

## 4. Frontend — Per-Module File Layout

### 4.1 shared/ [M1]

**Role:** Components used across all feature modules. Contains HandMatrix, CardRenderer, and UI infrastructure.

```
frontend/src/shared/
├── index.ts                           [M1] Barrel export
├── components/
│   ├── HandMatrix.tsx                 [M1] 13x13 Canvas-based matrix (HMX-001–009)
│   ├── HandMatrixCanvas.ts            [M1] Canvas rendering logic (PRF-003)
│   ├── CardRenderer.tsx               [M1] Card display with rank/suit (UIF-009)
│   ├── ActionColorSystem.ts           [M1] Color mapping: bet→red, check→green, fold→blue (UIF-004)
│   ├── Tooltip.tsx                    [M1] Tooltip wrapper (UIF-008)
│   ├── LoadingState.tsx               [M1] Loading spinner/progress bar (UIF-006)
│   ├── ErrorBoundary.tsx              [M1] Error display component (UIF-007)
│   ├── Navigation.tsx                 [M1] Sidebar navigation (UIF-005)
│   └── KeyboardShortcuts.tsx          [M1] Global shortcut handler (UIF-002)
├── hooks/
│   ├── useKeyboardShortcuts.ts        [M1] Shortcut registration hook
│   ├── useLoadingState.ts             [M1] Loading state management
│   └── useTheme.ts                    [M1] Theme context hook
├── context/
│   └── AppContext.tsx                  [M1] Global state: mode, theme, loading
├── types/
│   ├── poker.ts                       [M1] Card, Hand, Board, Action TypeScript types
│   ├── matrix.ts                      [M1] HandMatrix data types
│   └── common.ts                      [M1] Shared utility types
└── styles/
    ├── theme.ts                       [M1] Dark theme tokens (UIF-001)
    └── global.css                     [M1] Global styles
```

**Import rules:** `shared/` may NOT import from any feature module. It is the dependency root.

### 4.2 ipc/ [M1]

**Role:** Typed Tauri IPC wrappers. All backend communication flows through this module.

```
frontend/src/ipc/
├── index.ts                           [M1] Barrel export
├── invoke.ts                          [M1] Typed invoke wrapper
├── events.ts                          [M2] Event listener registration
├── types/
│   ├── solver.ts                      [M2] StartSolveRequest, SolveResult, SolveProgress
│   ├── solution.ts                    [M2] SolutionQuery, SolutionEntry, StrategyData
│   ├── eval.ts                        [M1] EvalRequest, EvalResult
│   ├── range.ts                       [M1] EquityRequest, PresetData
│   ├── practice.ts                    [M4] PracticeHand, FeedbackResult, SessionStats
│   ├── handhistory.ts                 [M5] ImportRequest, ImportProgress
│   ├── analyze.ts                     [M5] DashboardStats, AnalyzedHand
│   ├── nodelock.ts                    [M6] LockRequest, ComparisonData
│   ├── reports.ts                     [M6] AggregateData, TextureFilter
│   ├── icm.ts                         [M3] IcmRequest, IcmEquity, BubbleFactor
│   └── settings.ts                    [M1] AppConfig
├── commands/
│   ├── eval.ts                        [M1] eval:: command wrappers
│   ├── range.ts                       [M1] range:: command wrappers
│   ├── settings.ts                    [M1] settings:: command wrappers
│   ├── solver.ts                      [M2] solver:: command wrappers
│   ├── solution.ts                    [M2] solution:: command wrappers
│   ├── icm.ts                         [M3] icm:: command wrappers
│   ├── practice.ts                    [M4] practice:: command wrappers
│   ├── handhistory.ts                 [M5] handhistory:: command wrappers
│   ├── analyze.ts                     [M5] analyze:: command wrappers
│   ├── nodelock.ts                    [M6] nodelock:: command wrappers
│   └── reports.ts                     [M6] reports:: command wrappers
└── hooks/
    ├── useSolveProgress.ts            [M2] Subscribe to solve_progress events
    ├── useImportProgress.ts           [M5] Subscribe to import_progress events
    └── useAnalyzeProgress.ts          [M5] Subscribe to analyze_progress events
```

**Import rules:** `ipc/` imports from `shared/types/` only. It does NOT import from feature modules.

### 4.3 range-builder/ [M1]

**Role:** Interactive range construction UI.

```
frontend/src/range-builder/
├── index.ts                           [M1] Barrel export
├── RangeBuilderPage.tsx               [M1] Page root component
├── components/
│   ├── RangeGrid.tsx                  [M1] 13x13 grid with selection state (RNG-001)
│   ├── PaintbrushTools.tsx            [M1] Click/drag painting modes (RNG-002)
│   ├── WeightControls.tsx             [M1] Weight slider 0–100% (RNG-003)
│   ├── SuitExpander.tsx               [M1] Suit-specific view (RNG-005)
│   ├── PresetSelector.tsx             [M1] Preset range dropdown (RNG-006)
│   ├── ComboLock.tsx                  [M1] Lock/unlock combos (RNG-007)
│   ├── RangeSummary.tsx               [M1] Combos, %, equity (RNG-004)
│   └── RangeColorLegend.tsx           [M1] Color coding legend (RNG-010)
├── hooks/
│   └── useRangeState.ts               [M1] useReducer for range state
├── types.ts                           [M1] Range-specific types
├── GradeDisplay.tsx                   [M7] Grade vs GTO scoring (RNG-008)
└── ImportExport.tsx                   [M7] Range text import/export (RNG-009)
```

**Imports:** `shared/`, `ipc/`

### 4.4 solve/ [M2]

**Role:** Custom solve configuration, progress display, and results viewer.

```
frontend/src/solve/
├── index.ts                           [M2] Barrel export
├── SolvePage.tsx                      [M2] Page root component
├── components/
│   ├── SolveConfigPanel.tsx           [M2] Position, stack, pot, bet sizes (RTS-001, RTS-003)
│   ├── BetTreeEditor.tsx              [M2] Visual bet tree configuration (SOL-005)
│   ├── ProgressDisplay.tsx            [M2] Live progress: iteration, ND, time (SOL-018)
│   ├── ResultsViewer.tsx              [M2] Solution display after solve
│   └── StackDepthSlider.tsx           [M2] 1bb–200bb slider (RTS-004)
├── hooks/
│   └── useSolveSession.ts            [M2] Solve lifecycle management
└── types.ts                           [M2] Solve-specific types
```

**Imports:** `shared/`, `ipc/`

### 4.5 study/ [M3]

**Role:** Solution browsing with 4-tab interface.

```
frontend/src/study/
├── index.ts                           [M3] Barrel export
├── StudyPage.tsx                      [M3] Page root with tab container (STU-001)
├── components/
│   ├── SpotSelector.tsx               [M3] Game type, position, stack, action (STU-002)
│   ├── BoardSelector.tsx              [M3] Card selection for flop/turn/river (STU-003)
│   ├── ActionSequence.tsx             [M3] Clickable action line for tree nav (STU-008)
│   ├── MetricOverlay.tsx              [M3] Strategy/EV/Equity/EQR switch (STU-009)
│   ├── EvDisplay.tsx                  [M3] Per-action and overall EV (STU-010)
│   ├── tabs/
│   │   ├── StrategyTab.tsx            [M3] Action frequencies in matrix (STU-004)
│   │   ├── RangesTab.tsx              [M3] Per-player range display (STU-005)
│   │   ├── BreakdownTab.tsx           [M3] Hand category breakdown (STU-006)
│   │   └── ReportsTab.tsx             [M3] Aggregated metrics (STU-007)
│   ├── HandDetailPanel.tsx            [M7] Detailed hand info on click (STU-011)
│   ├── CompareEvOverlay.tsx           [M7] EV delta overlay (STU-013)
│   ├── RangeWeightDisplay.tsx         [M7] Weight frequency display (STU-014)
│   └── FlopBrowser.tsx                [M7] Navigate 1,755 flops by texture (STU-015)
├── hooks/
│   └── useStudyState.ts              [M3] useReducer for study state
└── types.ts                           [M3] Study-specific types
```

**Imports:** `shared/`, `ipc/`

### 4.6 practice/ [M4]

**Role:** Practice mode — play hands against GTO, get feedback.

```
frontend/src/practice/
├── index.ts                           [M4] Barrel export
├── PracticePage.tsx                   [M4] Page root component
├── components/
│   ├── GameTable.tsx                  [M4] Poker table display (PRA-001)
│   ├── ActionButtons.tsx              [M4] Player action buttons (PRA-003)
│   ├── FeedbackPanel.tsx              [M4] Post-decision GTO comparison (PRA-006)
│   ├── SessionStats.tsx               [M4] Cumulative stats display (PRA-009)
│   ├── RngSelector.tsx                [M4] High/Low/Off RNG mode (PRA-007)
│   ├── DifficultySelector.tsx         [M4] Simple/Grouped/Standard (PRA-003)
│   ├── ScoreDisplay.tsx               [M4] Score + classification (PRA-004, PRA-005)
│   ├── SpotConfig.tsx                 [M4] Practice spot configuration (PRA-008)
│   ├── HandReview.tsx                 [M7] Review completed hands (PRA-010)
│   ├── MultiTableLayout.tsx           [M7] 1–4 table layout (PRA-011)
│   └── SpeedControl.tsx               [M7] Normal/Fast/Turbo (PRA-012)
├── hooks/
│   └── usePracticeSession.ts          [M4] Session state management
└── types.ts                           [M4] Practice-specific types
```

**Imports:** `shared/`, `ipc/`

### 4.7 analyze/ [M5]

**Role:** Hand history import, analysis dashboard, hand replay.

```
frontend/src/analyze/
├── index.ts                           [M5] Barrel export
├── AnalyzePage.tsx                    [M5] Page root component
├── components/
│   ├── ImportWizard.tsx               [M5] File/directory selection + progress (ANL-001, ANL-002)
│   ├── HandsTable.tsx                 [M5] List of analyzed hands
│   ├── HandReplay.tsx                 [M5] Street-by-street replay (ANL-005)
│   ├── GtoOverlay.tsx                 [M5] GTO strategy at decision nodes (ANL-006)
│   ├── Dashboard.tsx                  [M5] Aggregate statistics (ANL-007)
│   ├── PositionBreakdown.tsx          [M5] Per-position stats (ANL-008)
│   ├── StreetBreakdown.tsx            [M5] Per-street stats (ANL-009)
│   ├── ActionBreakdown.tsx            [M5] Per-action stats (ANL-010)
│   ├── FilterBar.tsx                  [M7] Date, position, stakes filters (ANL-011)
│   ├── BiggestMistakes.tsx            [M7] Ranked EV loss list (ANL-012)
│   ├── SessionSummary.tsx             [M7] Per-session grouping (ANL-013)
│   └── ExportResults.tsx              [M7] CSV/JSON export (ANL-014)
├── hooks/
│   └── useAnalyzeState.ts            [M5] useReducer for analysis state
└── types.ts                           [M5] Analysis-specific types
```

**Imports:** `shared/`, `ipc/`

### 4.8 nodelock/ [M6]

**Role:** Strategy locking, frequency editing, exploitative re-solve.

```
frontend/src/nodelock/
├── index.ts                           [M6] Barrel export
├── NodelockPage.tsx                   [M6] Page root component
├── components/
│   ├── LockControls.tsx               [M6] Lock/unlock/reset buttons (NLK-004)
│   ├── FrequencyEditor.tsx            [M6] Slider + numeric input editor (NLK-002, NLK-007)
│   ├── ComparisonView.tsx             [M6] GTO vs nodelock side-by-side (NLK-006)
│   ├── GameTreeNav.tsx                [M6] Tree navigation for node selection
│   └── BatchLockPanel.tsx             [M6] Multi-node batch lock (NLK-008)
├── hooks/
│   └── useNodelockState.ts           [M6] useReducer for nodelock state
└── types.ts                           [M6] Nodelock-specific types
```

**Imports:** `shared/`, `ipc/`

### 4.9 reports/ [M6]

**Role:** Aggregated reports across solved spots.

```
frontend/src/reports/
├── index.ts                           [M6] Barrel export
├── ReportsPage.tsx                    [M6] Page root component
├── components/
│   ├── AggregateChart.tsx             [M6] Bar graph visualization (AGG-004)
│   ├── AggregateTable.tsx             [M6] Sortable numeric table (AGG-005)
│   ├── BoardTextureFilters.tsx        [M6] Monotone/two-tone/rainbow/etc (AGG-003)
│   ├── MetricSelector.tsx             [M6] Strategy/EV/Equity/EQR (AGG-002)
│   ├── ActionGrouping.tsx             [M6] All Sizes/Grouped/Simplified (AGG-007)
│   └── FlopGrouping.tsx               [M6] High card/suit/pair/connect (AGG-006)
├── hooks/
│   └── useReportsState.ts            [M6] useReducer for report state
└── types.ts                           [M6] Report-specific types
```

**Imports:** `shared/`, `ipc/`

---

## 5. Shared Types & IPC Contracts

### 5.1 TypeScript Types from Rust

The following TypeScript types in `ipc/types/` are derived from Rust `serde` structs. They MUST stay synchronized.

| Rust Crate | Rust Type | TypeScript File | TypeScript Type |
|------------|-----------|-----------------|-----------------|
| `poker-core` | `Card` | `shared/types/poker.ts` | `Card` |
| `poker-core` | `Hand` | `shared/types/poker.ts` | `Hand` |
| `poker-core` | `Board` | `shared/types/poker.ts` | `Board` |
| `poker-core` | `Action` | `shared/types/poker.ts` | `Action` |
| `poker-core` | `Position` | `shared/types/poker.ts` | `Position` |
| `poker-core` | `GameType` | `shared/types/poker.ts` | `GameType` |
| `poker-core` | `BetSize` | `shared/types/poker.ts` | `BetSize` |
| `poker-eval` | `HandRank` | `shared/types/poker.ts` | `HandRank` |
| `poker-eval` | `HandCategory` | `shared/types/poker.ts` | `HandCategory` |
| `app-tauri` | `AppError` | `ipc/types/settings.ts` | `AppError` |
| `app-tauri` | `StartSolveRequest` | `ipc/types/solver.ts` | `StartSolveRequest` |
| `app-tauri` | `SolveResult` | `ipc/types/solver.ts` | `SolveResult` |
| `app-tauri` | `SolveProgress` | `ipc/types/solver.ts` | `SolveProgress` |
| `poker-solution` | `SolutionMetadata` | `ipc/types/solution.ts` | `SolutionMetadata` |
| `poker-solution` | `SolutionEntry` | `ipc/types/solution.ts` | `SolutionEntry` |
| `poker-analyze` | `DashboardStats` | `ipc/types/analyze.ts` | `DashboardStats` |
| `poker-analyze` | `AnalyzedHand` | `ipc/types/analyze.ts` | `AnalyzedHand` |
| `poker-analyze` | `DecisionClassification` | `ipc/types/analyze.ts` | `DecisionClassification` |

### 5.2 IPC Type Synchronization Enforcement

Rust↔TypeScript type definitions MUST remain in sync. Enforcement mechanism:

1. **Single source of truth:** Rust `#[derive(Serialize, Deserialize)]` structs in `poker-core` and IPC command return types are the canonical definitions.
2. **TypeScript mirror:** Each Rust IPC struct has a corresponding TypeScript interface in `src/ipc/types.ts`. Field names, types, and optionality must match exactly.
3. **Round-trip integration test:** A Tauri integration test serializes each IPC response type from Rust, deserializes in TypeScript, re-serializes, and compares. This test runs in CI and fails on any mismatch.
4. **PR checklist item:** Any change to an IPC struct requires updating both Rust and TypeScript definitions in the same commit.

### 5.3 IPC Command Namespace Mapping

| Namespace | Tauri Command File | TS Wrapper File | Milestone |
|-----------|--------------------|-----------------|-----------|
| `eval::` | `commands/eval.rs` | `commands/eval.ts` | M1 |
| `range::` | `commands/range.rs` | `commands/range.ts` | M1 |
| `settings::` | `commands/settings.rs` | `commands/settings.ts` | M1 |
| `solver::` | `commands/solver.rs` | `commands/solver.ts` | M2 |
| `solution::` | `commands/solution.rs` | `commands/solution.ts` | M2 |
| `icm::` | `commands/icm.rs` | `commands/icm.ts` | M3 |
| `practice::` | `commands/practice.rs` | `commands/practice.ts` | M4 |
| `handhistory::` | `commands/handhistory.rs` | `commands/handhistory.ts` | M5 |
| `analyze::` | `commands/analyze.rs` | `commands/analyze.ts` | M5 |
| `nodelock::` | `commands/nodelock.rs` | `commands/nodelock.ts` | M6 |
| `reports::` | `commands/reports.rs` | `commands/reports.ts` | M6 |

---

## 6. Per-Milestone Creation Matrix

### 6.1 M1 — Foundation & Hand Matrix

**New files created:**

| Layer | Files |
|-------|-------|
| Workspace | `Cargo.toml`, `.gitignore`, `CLAUDE.md`, `README.md` |
| `poker-core` | All files in §3.1 (11 files) |
| `poker-eval` | All files except `batch.rs` (8 files) |
| `app-tauri` | `Cargo.toml`, `tauri.conf.json`, `build.rs`, `icons/`, `main.rs`, `state.rs`, `error.rs`, `setup.rs`, `db.rs`, `commands/mod.rs`, `commands/settings.rs`, `commands/eval.rs`, `commands/range.rs` |
| `frontend` | `package.json`, `tsconfig.json`, `vite.config.ts`, `index.html`, `main.tsx`, `App.tsx` |
| `shared/` | All files in §4.1 (18 files) |
| `ipc/` | `index.ts`, `invoke.ts`, `types/eval.ts`, `types/range.ts`, `types/settings.ts`, `commands/eval.ts`, `commands/range.ts`, `commands/settings.ts` |
| `range-builder/` | All [M1] files in §4.3 (11 files) |
| `fixtures/eval/` | Hand evaluation reference data |

**DO NOT CREATE in M1:**
- `crates/poker-solver/` — M2
- `crates/poker-solution/` — M2
- `crates/poker-icm/` — M3
- `crates/poker-hhparser/` — M5
- `crates/poker-analyze/` — M5
- `frontend/src/solve/` — M2
- `frontend/src/study/` — M3
- `frontend/src/practice/` — M4
- `frontend/src/analyze/` — M5
- `frontend/src/nodelock/` — M6
- `frontend/src/reports/` — M6
- `ipc/events.ts` — M2
- `ipc/hooks/` — M2
- Any `[M2]`–`[M8]` tagged files in existing crates

### 6.2 M2 — Solver Engine & Solution Storage

**New files created:**

| Layer | Files |
|-------|-------|
| `poker-solver` | All [M2] files in §3.3 (11 files) |
| `poker-solution` | All [M2] files in §3.4 (11 files) |
| `app-tauri` | `commands/solver.rs`, `commands/solution.rs`, `events.rs` |
| `solve/` | All [M2] files in §4.4 (8 files) |
| `ipc/` | `events.ts`, `types/solver.ts`, `types/solution.ts`, `commands/solver.ts`, `commands/solution.ts`, `hooks/useSolveProgress.ts` |
| `fixtures/solutions/` | Reference solution files |

**Files modified (extended):**
- `Cargo.toml` — add `poker-solver`, `poker-solution` workspace members
- `app-tauri/Cargo.toml` — add dependencies on `poker-solver`, `poker-solution`
- `app-tauri/src/main.rs` — register solver and solution commands
- `app-tauri/src/state.rs` — add SolverSession map, SolutionIndex, SolutionCache
- `app-tauri/src/commands/mod.rs` — add solver and solution modules
- `frontend/src/App.tsx` — add Solve route
- `shared/components/Navigation.tsx` — add Solve nav entry

**DO NOT CREATE in M2:**
- `poker-solver/src/nodelock.rs` — M6
- `poker-solver/src/dcfr.rs`, `linear_cfr.rs`, `preflop.rs`, `dynamic_sizing.rs` — M7
- `poker-solution/src/checksum.rs` — M8
- `poker-solution/src/migration.rs` — M7
- `frontend/src/study/` — M3
- `frontend/src/practice/` — M4
- `frontend/src/analyze/` — M5

### 6.3 M3 — Study Mode & Solution Browsing

**New files created:**

| Layer | Files |
|-------|-------|
| `poker-icm` | All [M3] files in §3.5 (5 files) |
| `app-tauri` | `commands/icm.rs` |
| `study/` | All [M3] files in §4.5 (13 files) |
| `ipc/` | `types/icm.ts`, `commands/icm.ts` |

**Files modified:**
- `Cargo.toml` — add `poker-icm` workspace member
- `app-tauri/Cargo.toml` — add `poker-icm` dependency
- `app-tauri/src/main.rs` — register ICM commands
- `app-tauri/src/commands/mod.rs` — add icm module
- `frontend/src/App.tsx` — add Study route
- `shared/components/Navigation.tsx` — add Study nav entry

**DO NOT CREATE in M3:**
- `poker-icm/src/pko.rs`, `satellite.rs`, `simulator.rs` — M7
- `poker-icm/src/mystery.rs` — M8
- `study/` M7 files: `HandDetailPanel.tsx`, `CompareEvOverlay.tsx`, `RangeWeightDisplay.tsx`, `FlopBrowser.tsx`
- `frontend/src/practice/` — M4
- `frontend/src/analyze/` — M5

### 6.4 M4 — Practice Mode

**New files created:**

| Layer | Files |
|-------|-------|
| `app-tauri` | `commands/practice.rs` |
| `practice/` | All [M4] files in §4.6 (11 files) |
| `ipc/` | `types/practice.ts`, `commands/practice.ts` |

**Files modified:**
- `app-tauri/src/main.rs` — register practice commands
- `app-tauri/src/commands/mod.rs` — add practice module
- `app-tauri/src/state.rs` — add practice session state
- `frontend/src/App.tsx` — add Practice route
- `shared/components/Navigation.tsx` — add Practice nav entry

**DO NOT CREATE in M4:**
- `practice/` M7 files: `HandReview.tsx`, `MultiTableLayout.tsx`, `SpeedControl.tsx`

### 6.5 M5 — Hand History Analysis

**New files created:**

| Layer | Files |
|-------|-------|
| `poker-hhparser` | All [M5] files in §3.6 (12 files) |
| `poker-analyze` | All [M5] files in §3.7 (6 files) |
| `app-tauri` | `commands/handhistory.rs`, `commands/analyze.rs` |
| `analyze/` | All [M5] files in §4.7 (11 files) |
| `ipc/` | `types/handhistory.ts`, `types/analyze.ts`, `commands/handhistory.ts`, `commands/analyze.ts`, `hooks/useImportProgress.ts`, `hooks/useAnalyzeProgress.ts` |
| `fixtures/hands/` | Sample HH files per site |

**Files modified:**
- `Cargo.toml` — add `poker-hhparser`, `poker-analyze` workspace members
- `app-tauri/Cargo.toml` — add dependencies
- `app-tauri/src/main.rs` — register HH and analyze commands
- `app-tauri/src/commands/mod.rs` — add handhistory and analyze modules
- `frontend/src/App.tsx` — add Analyze route
- `shared/components/Navigation.tsx` — add Analyze nav entry

**DO NOT CREATE in M5:**
- `poker-hhparser/src/normalize.rs`, `auto_detect.rs`, `incremental.rs` — M7
- `poker-hhparser/src/validate.rs` — M8
- `poker-analyze/src/filter.rs`, `export.rs` — M7
- `analyze/` M7 files: `FilterBar.tsx`, `BiggestMistakes.tsx`, `SessionSummary.tsx`, `ExportResults.tsx`

### 6.6 M6 — Nodelocking & Aggregated Reports

**New files created:**

| Layer | Files |
|-------|-------|
| `poker-solver` | `nodelock.rs` |
| `app-tauri` | `commands/nodelock.rs`, `commands/reports.rs` |
| `nodelock/` | All files in §4.8 (7 files) |
| `reports/` | All files in §4.9 (9 files) |
| `ipc/` | `types/nodelock.ts`, `types/reports.ts`, `commands/nodelock.ts`, `commands/reports.ts` |

**Files modified:**
- `app-tauri/src/main.rs` — register nodelock and reports commands
- `app-tauri/src/commands/mod.rs` — add nodelock and reports modules
- `frontend/src/App.tsx` — add Nodelock and Reports routes
- `shared/components/Navigation.tsx` — add Nodelock and Reports nav entries

### 6.7 M7 — SHOULD Enhancements

**New files created (additions to existing modules):**

| Layer | Files |
|-------|-------|
| `poker-eval` | `batch.rs` |
| `poker-solver` | `dcfr.rs`, `linear_cfr.rs`, `preflop.rs`, `dynamic_sizing.rs` |
| `poker-solution` | `migration.rs` |
| `poker-icm` | `pko.rs`, `satellite.rs`, `simulator.rs` |
| `poker-hhparser` | `normalize.rs`, `auto_detect.rs`, `incremental.rs` |
| `poker-analyze` | `filter.rs`, `export.rs` |
| `study/` | `HandDetailPanel.tsx`, `CompareEvOverlay.tsx`, `RangeWeightDisplay.tsx`, `FlopBrowser.tsx` |
| `practice/` | `HandReview.tsx`, `MultiTableLayout.tsx`, `SpeedControl.tsx` |
| `analyze/` | `FilterBar.tsx`, `BiggestMistakes.tsx`, `SessionSummary.tsx`, `ExportResults.tsx` |
| `range-builder/` | `GradeDisplay.tsx`, `ImportExport.tsx` |

**Files modified:** Various `lib.rs`, `mod.rs`, and page components to integrate new features.

### 6.8 M8 — MAY Polish

**New files created:**

| Layer | Files |
|-------|-------|
| `poker-solution` | `checksum.rs` |
| `poker-icm` | `mystery.rs` |
| `poker-hhparser` | `validate.rs` |

**Files modified:** Various existing modules gain optional feature integrations.

---

## 7. API Boundary Rules

### 7.1 Rust Crate Dependency Rules

1. **No cycles.** The crate DAG from `architecture.md` §15.1 is strictly enforced:
   ```
   poker-core → (nothing)
   poker-eval → poker-core
   poker-icm → poker-core
   poker-hhparser → poker-core
   poker-solver → poker-core, poker-eval
   poker-solution → poker-core
   poker-analyze → poker-core, poker-eval, poker-solver, poker-solution, poker-hhparser
   app-tauri → all library crates
   ```

2. **`poker-core` is the shared vocabulary.** All cross-crate type passing uses `poker-core` types (`Card`, `Hand`, `Board`, `Action`, `Position`, `GameType`, `BetSize`).

3. **`poker-solution` does NOT depend on `poker-solver`.** Solution reading/writing works with serialized binary data, not live solver state. The `app-tauri` crate bridges them.

4. **`poker-analyze` is the most connected library crate.** It orchestrates data from solver, evaluator, solution store, and parser. This is intentional — it is the analysis pipeline.

### 7.2 Rust Visibility Rules

| Visibility | Use For |
|-----------|---------|
| `pub` | Types and functions consumed by other crates. Defined in the crate's public API section above. |
| `pub(crate)` | Internal helpers, implementation details, performance-critical internals not meant for external consumption. |
| `pub(super)` | Module-internal helpers visible to parent module only. |
| private (default) | Everything else. |

**Rule:** Only items listed in the "Public API boundary" section of each crate (§3.1–§3.8) should be `pub`. Everything else defaults to `pub(crate)` or private.

### 7.3 Frontend Module Isolation Rules

1. **Feature modules import ONLY from `shared/` and `ipc/`.** No cross-module imports.
   - `study/` may import from `shared/`, `ipc/`
   - `study/` may NOT import from `practice/`, `analyze/`, `solve/`, etc.

2. **Cross-mode navigation uses routes, not imports.** If Study Mode has an "Open in Analyze" button, it navigates via the router (`navigate('/analyze/hand/123')`), never by importing Analyze components.

3. **`shared/` is the dependency root.** It may not import from any feature module or from `ipc/` (except `shared/types/` which `ipc/` also imports).

4. **`ipc/` imports only from `shared/types/`.** It does not import components or hooks from any module.

### 7.4 Automated Import Enforcement

The cross-module import ban is enforced via ESLint:

**ESLint rule (`eslint-plugin-import`):**
```json
{
  "rules": {
    "import/no-restricted-paths": [{
      "zones": [
        { "target": "./src/range-builder/**", "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/solve/**",         "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/study/**",         "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/practice/**",      "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/analyze/**",       "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/nodelock/**",      "from": "./src/!(shared|ipc)/**" },
        { "target": "./src/reports/**",       "from": "./src/!(shared|ipc)/**" }
      ]
    }]
  }
}
```

This ESLint rule runs in CI and blocks PRs that introduce cross-module imports. Feature modules may only import from `shared/` and `ipc/`.

### 7.5 IPC Contract Rules

1. **Coarse-grained commands.** One IPC call per user action. A "start solve" button sends one `solver::start_solve` command, not a sequence of `create_tree`, `configure_solver`, `start_iterations`.

2. **All commands return `Result<T, AppError>`.** No untyped payloads. No `any` types in TypeScript.

3. **Long-running operations use event streams.** Solving, importing, and analyzing emit progress events. The frontend subscribes, never polls.

4. **No shared mutable state across IPC.** Each command receives immutable references to `AppState` (via Tauri's managed state). Mutable access to solver sessions is guarded by `Mutex`.

---

## 8. Data Directory Layout

The application stores all user data in `~/.poker-solver/` (configurable via `config.toml`):

```
~/.poker-solver/                       [M1] Created on first launch
├── solutions/                         [M2] Binary solution files
│   ├── cash/                          [M2]
│   │   └── {game}_{stack}_{pos}_{board}.sol
│   ├── mtt/                           [M3]
│   └── custom/                        [M2]
├── hands/                             [M5] Imported HH source files
│   ├── pokerstars/                    [M5]
│   ├── ggpoker/                       [M5]
│   ├── winamax/                       [M5]
│   └── .../
├── data.db                            [M1] SQLite (WAL mode)
├── config.toml                        [M1] User configuration
└── eval_tables.bin                    [M1] Hand evaluator lookup tables (~10MB)
```

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial project structure document |

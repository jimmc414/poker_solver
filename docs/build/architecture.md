# System Architecture

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 2 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisite:** [Requirements Specification](requirements.md) (Build Document 1)

---

## 1. Scope & Conventions

### Purpose

This document defines the logical and physical architecture of the GTO Poker Solver desktop application. It translates the 195 requirements from Build Document 1 into a system decomposition: crate boundaries, module boundaries, data flows, IPC contracts, concurrency model, and storage layout.

### Relationship to Requirements

Every requirement ID from Build Document 1 (SOL-*, HEV-*, STO-*, STU-*, PRA-*, ANL-*, RTS-*, NLK-*, AGG-*, RNG-*, ICM-*, HHP-*, UIF-*, HMX-*, DSK-*, PRF-*, DAT-*) maps to at least one Rust crate and/or frontend module. Section 14 provides the full traceability matrix.

### Notation

- **ASCII diagrams** use box-drawing characters for system structure
- **Component names** use PascalCase (e.g., `SolverEngine`, `SpotSelector`)
- **Crate names** use kebab-case (e.g., `poker-solver`, `poker-eval`)
- **IPC commands** use `namespace::command_name` notation
- **Arrows** indicate dependency or data flow direction; `A → B` means A depends on B or data flows from A to B

### Cloud-to-Local Adaptation

The reference platform (GTO Wizard) uses a cloud architecture: React/Next.js frontend, Python/Django backend, C++ solver, CDN delivery, Kubernetes orchestration, PostgreSQL + Redis data stores. This architecture adapts those patterns to a **single-process local-first desktop application**:

| Cloud Pattern | Local Adaptation |
|---------------|-----------------|
| REST API + WebSocket | Tauri IPC commands + event streaming |
| CDN-cached solution delivery | Memory-mapped local files |
| PostgreSQL + Redis | SQLite (single file) |
| Kubernetes worker pool | Rayon thread pool (in-process) |
| C++ solver + Python orchestration | Pure Rust (solver + orchestration) |
| Browser-based SPA | Tauri webview (same React, native shell) |
| User account + subscription system | Local config file (no auth needed) |

---

## 2. Architectural Principles

### P1: Local-First (DSK-002)

All computation and data reside on the user's machine. The application requires zero network connectivity for core functionality. No cloud APIs, no remote databases, no CDN. The solver runs locally, solutions are stored locally, and analysis happens locally.

### P2: Separation of Concerns

Rust owns all computation: solving, evaluation, parsing, storage, and analysis. TypeScript/React owns all presentation: rendering, interaction, state management, and layout. The boundary between them is the Tauri IPC layer — a thin, well-typed interface.

### P3: Performance by Design (PRF-*)

Performance-critical paths are implemented in Rust with zero-copy semantics where possible:
- Hand evaluation uses precomputed lookup tables for 200M+ evals/sec (PRF-001)
- Solver uses MCCFR with parallel tree traversal via `rayon` (PRF-002, PRF-007)
- Solution loading uses memory-mapped I/O with partial decompression (PRF-004)
- UI renders hand matrices on Canvas for 60fps interaction (PRF-003)

### P4: Extensibility via Traits

Key subsystems define Rust traits as extension points:
- `SolverAlgorithm` — swap CFR+, DCFR, Linear CFR (SOL-014)
- `HandEvaluator` — swap lookup-table implementations (HEV-001)
- `SiteParser` — one implementation per poker site (HHP-001)
- `IcmModel` — standard ICM, PKO, satellite variants (ICM-001)

New implementations plug in without modifying existing code.

### P5: Minimal IPC Surface

The Tauri IPC layer uses coarse-grained commands, not fine-grained RPC. A single `solver::start_solve` command replaces what would be dozens of individual calls in a chatty protocol. Long-running operations stream progress via Tauri events rather than polling.

### P6: Data Integrity

Solution files include checksums (STO-012). SQLite uses WAL mode for crash resilience. Hand history files are validated before parsing (HHP-005). Configuration files use human-readable TOML format (DAT-007).

---

## 3. High-Level System Diagram

```
┌─────────────────────────────────────────────────────┐
│  Frontend  (TypeScript / React)                      │
│  Rendered in Tauri webview                           │
│                                                      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ │
│  │  Study   │ │ Practice │ │ Analyze  │ │  Solve │ │
│  │  Mode    │ │  Mode    │ │  Mode    │ │  Mode  │ │
│  └──────────┘ └──────────┘ └──────────┘ └────────┘ │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ │
│  │ Nodelock │ │  Range   │ │ Reports  │ │Settings│ │
│  │  Mode    │ │ Builder  │ │  Mode    │ │        │ │
│  └──────────┘ └──────────┘ └──────────┘ └────────┘ │
│                                                      │
│  Shared: HandMatrix, CardRenderer, ActionColors,     │
│          IPC wrappers, loading states, tooltips       │
└──────────────────┬──────────────────────────────────┘
                   │  Tauri IPC
                   │  (invoke commands / event stream)
┌──────────────────▼──────────────────────────────────┐
│  Backend  (Rust / Tauri Core)                        │
│                                                      │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│  │ poker-solver │ │  poker-eval  │ │poker-solution│ │
│  │ CFR engine,  │ │ Lookup-table │ │ Binary fmt,  │ │
│  │ game tree,   │ │ evaluator,   │ │ zstd, mmap,  │ │
│  │ MCCFR,       │ │ equity calc, │ │ indexing,    │ │
│  │ parallel     │ │ draws, block │ │ caching      │ │
│  └──────────────┘ └──────────────┘ └──────────────┘ │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│  │poker-hhparser│ │  poker-icm   │ │poker-analyze │ │
│  │ Multi-site   │ │ ICM equity,  │ │ EV loss,     │ │
│  │ parser,      │ │ bubble, PKO, │ │ decision     │ │
│  │ pot recon    │ │ satellite    │ │ classify,    │ │
│  │              │ │              │ │ dashboards   │ │
│  └──────────────┘ └──────────────┘ └──────────────┘ │
│  ┌──────────────┐ ┌──────────────────────────────┐   │
│  │  poker-core  │ │         app-tauri             │   │
│  │ Card, deck,  │ │  IPC bridge, commands,        │   │
│  │ hand types,  │ │  event streaming, lifecycle   │   │
│  │ poker rules  │ │                               │   │
│  └──────────────┘ └──────────────────────────────┘   │
└──────────────────┬──────────────────────────────────┘
                   │  File I/O + SQLite
┌──────────────────▼──────────────────────────────────┐
│  Local Data  (~/.poker-solver/)                      │
│                                                      │
│  solutions/       Binary solution files (zstd)       │
│  hands/           Imported hand history files         │
│  data.db          SQLite: user data, analysis, index │
│  config.toml      Application configuration          │
└─────────────────────────────────────────────────────┘
```

---

## 4. Rust Backend — Crate Decomposition

The Rust workspace is organized as a set of library crates with a single binary crate (`app-tauri`). Dependencies form a directed acyclic graph — no cycles permitted.

### 4.1 poker-core

**Responsibility:** Foundational poker types and rules shared by all other crates.

**Key Types:**
- `Card` — rank + suit, packed into a single `u8`
- `Deck` — 52-card deck with Fisher-Yates shuffle
- `Hand` — ordered set of 2 hole cards
- `Board` — 0–5 community cards (flop/turn/river)
- `Action` — enum: Fold, Check, Call, Bet(amount), Raise(amount), AllIn
- `Position` — enum: UTG, UTG1, MP, CO, BTN, SB, BB (DAT-002)
- `GameType` — enum: CashNLH, MTT, SNG, SpinAndGo, HeadsUp (DAT-001)
- `BetSize` — enum: PotFraction(f64), Absolute(f64), AllIn (DAT-004)

**Public API:**
```rust
pub fn card(rank: Rank, suit: Suit) -> Card;
pub fn hand_notation(hand: &Hand) -> String;       // "AKs", "QJo", "TT"
pub fn is_suited(hand: &Hand) -> bool;
pub fn canonical_hand_index(hand: &Hand) -> u8;     // 0..168
```

**Dependencies:** None (leaf crate).

**Requirement Coverage:** DAT-001, DAT-002, DAT-003, DAT-004.

### 4.2 poker-eval

**Responsibility:** Hand evaluation, equity calculation, draw detection, and blocker analysis.

**Key Types:**
- `HandRank` — u16 or u32 encoding of hand strength (7,462 distinct classes)
- `HandCategory` — enum: HighCard, Pair, TwoPair, Trips, Straight, Flush, FullHouse, Quads, StraightFlush
- `DrawType` — enum: FlushDraw, OESD, Gutshot, ComboDraw, BackdoorFlush, BackdoorStraight
- `LookupTable` — precomputed evaluation tables (~10MB, loaded at startup)

**Public API:**
```rust
pub trait HandEvaluator: Send + Sync {
    fn evaluate(&self, cards: &[Card; 7]) -> HandRank;
    fn evaluate_5(&self, cards: &[Card; 5]) -> HandRank;
    fn categorize(&self, rank: HandRank) -> HandCategory;
    fn compare(&self, a: HandRank, b: HandRank) -> Ordering;
}

pub fn equity(range_a: &Range, range_b: &Range, board: &Board) -> f64;
pub fn equity_batch(ranges: &[Range], board: &Board) -> Vec<f64>;
pub fn detect_draws(hand: &Hand, board: &Board) -> Vec<DrawType>;
pub fn blockers(hand: &Hand, board: &Board) -> BlockerInfo;
pub fn suit_isomorphism(board: &Board) -> CanonicalBoard;  // 22,100 → 1,755
```

**Dependencies:** `poker-core`

**Requirement Coverage:** HEV-001 through HEV-010, PRF-001.

**Performance Notes:**
- Lookup-table evaluator targets 200M+ evals/sec on a single core (HEV-002, PRF-001)
- Tables generated at first run, cached to `~/.poker-solver/eval_tables.bin`
- Batch evaluation exploits memory locality for 1.5x+ throughput (HEV-008)

### 4.3 poker-solver

**Responsibility:** CFR engine, game tree construction, MCCFR variants, card abstraction, and parallel solving.

**Key Types:**
- `GameTree` — arena-allocated tree of `GameNode` variants
- `GameNode` — enum: Decision { player, actions }, Chance { outcomes }, Terminal { payoffs }
- `InfoSet` — player's information state (hand + board + action history)
- `Strategy` — per-information-set action probabilities (`Vec<f32>`)
- `SolverConfig` — bet sizes, raise cap, stack depth, positions, stopping conditions
- `SolveProgress` — current iteration, elapsed time, Nash Distance estimate
- `CancellationToken` — atomic flag checked between iterations

**Public API:**
```rust
pub trait SolverAlgorithm: Send + Sync {
    fn iterate(&mut self, tree: &GameTree, iteration: u64);
    fn get_strategy(&self, info_set: &InfoSet) -> &Strategy;
    fn nash_distance(&self) -> f64;
}

pub fn build_game_tree(config: &SolverConfig) -> GameTree;
pub fn solve(
    config: &SolverConfig,
    progress_tx: Sender<SolveProgress>,
    cancel: CancellationToken,
) -> SolveResult;
```

**Trait Implementations:**
- `CfrPlusSolver` — CFR+ with regret floor at zero
- `DiscountedCfrSolver` — DCFR with time-based discounting (SOL-014)
- `LinearCfrSolver` — iteration-weighted strategy averaging

**Dependencies:** `poker-core`, `poker-eval`

**Requirement Coverage:** SOL-001 through SOL-018, PRF-002, PRF-005, PRF-007.

**Concurrency Model:**
- `rayon` thread pool for parallel tree traversal (SOL-009)
- Each thread processes independent subtrees
- Shared regret/strategy tables use atomic operations or lock-free structures
- Cancellation token checked between iterations for responsive stop (SOL-010)

### 4.4 poker-solution

**Responsibility:** Solution serialization, compression, memory-mapped access, indexing, and caching.

**Key Types:**
- `SolutionFile` — binary format: header + metadata + compressed tree data
- `SolutionHeader` — magic bytes, format version, checksum
- `SolutionMetadata` — solver version, game config, solve date, Nash Distance (STO-005)
- `SolutionReader` — memory-mapped reader with partial decompression
- `SolutionIndex` — SQLite-backed index of all stored solutions (STO-006)
- `SolutionCache` — LRU cache of recently loaded solution nodes

**Public API:**
```rust
pub fn serialize(result: &SolveResult, config: &SolverConfig) -> Vec<u8>;
pub fn compress(data: &[u8]) -> Vec<u8>;               // zstd compression
pub fn decompress(data: &[u8]) -> Vec<u8>;

pub struct SolutionReader {
    pub fn open(path: &Path) -> Result<Self>;           // mmap
    pub fn metadata(&self) -> &SolutionMetadata;
    pub fn load_node(&self, node_id: NodeId) -> Strategy;
    pub fn verify_checksum(&self) -> bool;              // STO-012
}

pub struct SolutionIndex {
    pub fn search(&self, query: &SolutionQuery) -> Vec<SolutionEntry>;
    pub fn add(&mut self, path: &Path, meta: &SolutionMetadata);
    pub fn remove(&mut self, path: &Path);
    pub fn total_disk_usage(&self) -> u64;              // STO-010
}
```

**Binary Format:**
```
┌──────────────────────────────────────┐
│ Magic: "GTOSOL" (6 bytes)            │
│ Version: u16                          │
│ Flags: u16                           │
├──────────────────────────────────────┤
│ Metadata (JSON, uncompressed):       │
│   solver_version, game_config,       │
│   solve_date, nash_distance,         │
│   positions, stack_depth, board      │
│ Metadata length: u32                 │
├──────────────────────────────────────┤
│ Tree Data (zstd-compressed):         │
│   Strategies per information set     │
│   Quantized to u16 (STO-004)        │
│   Sparse encoding for skewed nodes   │
│   (STO-011)                          │
├──────────────────────────────────────┤
│ Checksum: xxHash64 (8 bytes)         │
└──────────────────────────────────────┘
```

**Dependencies:** `poker-core`

**Requirement Coverage:** STO-001 through STO-012, PRF-004.

### 4.5 poker-hhparser

**Responsibility:** Hand history parsing for multiple poker sites, format auto-detection, pot reconstruction, and position assignment.

**Key Types:**
- `ParsedHand` — fully extracted hand data: players, stacks, actions, cards, pot sizes
- `PlayerAction` — timestamp + player + action + amount + street
- `SiteFormat` — enum: PokerStars, GGPoker, Winamax, Poker888, PartyPoker, IPoker, ...

**Public API:**
```rust
pub trait SiteParser: Send + Sync {
    fn detect(content: &str) -> bool;
    fn parse(&self, content: &str) -> Result<Vec<ParsedHand>>;
    fn site_name(&self) -> &str;
}

pub struct ParserRegistry {
    pub fn auto_detect(content: &str) -> Option<Box<dyn SiteParser>>;
    pub fn register(parser: Box<dyn SiteParser>);
}

pub fn parse_file(path: &Path) -> Result<Vec<ParsedHand>>;
pub fn parse_directory(dir: &Path) -> BatchParseResult;
pub fn reconstruct_pot(actions: &[PlayerAction]) -> Vec<PotSnapshot>;
pub fn assign_positions(seats: &[Seat], button: u8, table_size: u8) -> Vec<Position>;
```

**Dependencies:** `poker-core`

**Requirement Coverage:** HHP-001 through HHP-012.

### 4.6 poker-icm

**Responsibility:** ICM equity calculation, bubble factor, PKO bounty, and satellite dynamics.

**Key Types:**
- `PayoutStructure` — vector of prize percentages per finishing position
- `StackDistribution` — chip counts for each remaining player
- `IcmEquity` — tournament-dollar equity per player
- `BubbleFactor` — risk premium per player (> 1.0 near bubble)

**Public API:**
```rust
pub trait IcmModel: Send + Sync {
    fn equity(&self, stacks: &StackDistribution, payouts: &PayoutStructure) -> Vec<f64>;
    fn bubble_factor(&self, player: usize, stacks: &StackDistribution, payouts: &PayoutStructure) -> f64;
}

pub struct StandardIcm;          // Malmuth-Harville model
pub struct PkoIcm;               // Progressive Knockout (ICM-006)
pub struct SatelliteIcm;         // Equal-prize satellite (ICM-007)
```

**Dependencies:** `poker-core`

**Requirement Coverage:** ICM-001 through ICM-008.

### 4.7 poker-analyze

**Responsibility:** Decision classification, EV loss calculation, and dashboard aggregation. Bridges the solver and hand history parser.

**Key Types:**
- `DecisionClassification` — enum: Perfect, Good, Inaccurate, Wrong, Blunder (ANL-003)
- `AnalyzedDecision` — player action, GTO action, EV loss, classification
- `AnalyzedHand` — parsed hand + vector of analyzed decisions
- `DashboardStats` — aggregate statistics by position, street, action type

**Public API:**
```rust
pub fn analyze_hand(
    hand: &ParsedHand,
    solution_index: &SolutionIndex,
    solver_config: &SolverConfig,
) -> AnalyzedHand;

pub fn classify_decision(ev_loss: f64, gto_frequency: f64) -> DecisionClassification;
pub fn compute_ev_loss(player_action: &Action, gto_strategy: &Strategy) -> f64;
pub fn aggregate_stats(hands: &[AnalyzedHand]) -> DashboardStats;
pub fn position_breakdown(hands: &[AnalyzedHand]) -> HashMap<Position, DashboardStats>;
pub fn street_breakdown(hands: &[AnalyzedHand]) -> HashMap<Street, DashboardStats>;
pub fn action_breakdown(hands: &[AnalyzedHand]) -> HashMap<ActionType, DashboardStats>;
```

**Dependencies:** `poker-core`, `poker-eval`, `poker-solver`, `poker-solution`, `poker-hhparser`

**Requirement Coverage:** ANL-003, ANL-004, ANL-007 through ANL-010.

### 4.8 app-tauri

**Responsibility:** Tauri application shell, IPC command handlers, event streaming, and application lifecycle.

**Key Types:**
- `AppState` — managed Tauri state: solver sessions, solution index, database connection, config
- `SolverSession` — handle to a running solve (progress channel + cancel token)
- `AppError` — unified error type serialized to frontend

**Public API:** All Tauri `#[command]` functions (see Section 7).

**Dependencies:** All library crates (`poker-core`, `poker-eval`, `poker-solver`, `poker-solution`, `poker-hhparser`, `poker-icm`, `poker-analyze`)

**Requirement Coverage:** DSK-001 through DSK-010.

---

## 5. Frontend — Module Decomposition

The frontend is a React application rendered inside Tauri's webview. It follows a feature-module structure where each major mode gets its own directory.

### 5.1 Module Map

| Module | Directory | Screens/Components | Key Requirements |
|--------|-----------|-------------------|-----------------|
| **Study** | `src/study/` | Strategy tab, Ranges tab, Breakdown tab, Reports tab, SpotSelector, BoardSelector, ActionSequence, MetricOverlay | STU-001 through STU-016 |
| **Practice** | `src/practice/` | GameTable, ActionButtons, FeedbackPanel, SessionStats, RngDice, TimerBar, MultiTableLayout | PRA-001 through PRA-014 |
| **Analyze** | `src/analyze/` | ImportWizard, HandsTable, HandReplay, Dashboard, FilterBar, GtoOverlay | ANL-001 through ANL-015 |
| **Solve** | `src/solve/` | SolveConfigPanel, BetTreeEditor, ProgressDisplay, ResultsViewer | RTS-001 through RTS-008 |
| **Nodelock** | `src/nodelock/` | LockControls, FrequencyEditor, ComparisonView, GameTreeNav, BatchLockPanel | NLK-001 through NLK-010 |
| **RangeBuilder** | `src/range-builder/` | RangeGrid, PaintbrushTools, WeightControls, SuitExpander, GradeDisplay, PresetSelector | RNG-001 through RNG-010 |
| **Reports** | `src/reports/` | AggregateChart, AggregateTable, BoardTextureFilters, ActionGrouping, ExportControls | AGG-001 through AGG-010 |
| **Shared** | `src/shared/` | HandMatrix (13x13), CardRenderer, ActionColorSystem, Tooltip, KeyboardShortcuts, LoadingState, ErrorBoundary, Navigation | HMX-001 through HMX-012, UIF-001 through UIF-015 |
| **IPC** | `src/ipc/` | Typed Tauri command wrappers, event listeners, request/response types, loading state hooks | DSK-003 |

### 5.2 Shared Components Detail

**HandMatrix** — The core visualization component used across Study, Practice, Analyze, Nodelock, and RangeBuilder. Renders the 13x13 hand grid with:
- Mixed-strategy coloring (proportional color fills per cell) (HMX-002)
- Hover detail panel with suit-specific breakdown (HMX-003)
- Action filtering (click to isolate hands by action) (HMX-004)
- Click selection (updates linked panels) (HMX-005)
- Metric overlays: Strategy, EV, Equity, EQR heat maps (HMX-006)
- Combo count display, adjusted for board card removal (HMX-007)
- Frequency threshold filtering (HMX-009)
- Canvas-based rendering for 60fps updates (PRF-003)

**CardRenderer** — Renders playing cards with rank/suit symbols, supporting standard and four-color deck modes (UIF-009, UIF-011).

**ActionColorSystem** — Consistent color mapping: red spectrum for bet/raise, green spectrum for check/call, blue spectrum for fold. Configurable via settings (UIF-004, UIF-015).

### 5.3 State Management

```
┌──────────────────────────────────────────────────┐
│  React Context: AppContext                        │
│                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌───────────┐ │
│  │ Navigation  │  │  Settings   │  │  Loading  │ │
│  │ activeMode, │  │ theme,      │  │ global    │ │
│  │ route       │  │ shortcuts   │  │ states    │ │
│  └─────────────┘  └─────────────┘  └───────────┘ │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  Per-Mode State (useReducer per feature)          │
│                                                   │
│  Study:    spot, board, node, tab, overlay, filter│
│  Practice: session, hand, score, rng, timer       │
│  Analyze:  hands, filters, sort, dashboard        │
│  Solve:    config, progress, result               │
│  Nodelock: locks, comparison, editState           │
│  Range:    grid, weights, presets, grade           │
│  Reports:  filters, grouping, view                │
└──────────────────────────────────────────────────┘
```

- **Global state** (React Context): active mode, settings/theme, global loading indicators
- **Feature state** (useReducer per module): each mode manages its own state locally, reducing coupling
- **No external state library** — React Context + useReducer is sufficient for a desktop app with no server synchronization

---

## 6. Data Flow Diagrams

### Flow 1: Custom Solve

```
User                Frontend                      Tauri IPC              Rust Backend
 │                    │                              │                      │
 │ Configure spot     │                              │                      │
 │ (positions, stacks,│                              │                      │
 │  bet sizes, board) │                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke solver::start_solve   │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ build_game_tree()    │
 │                    │                              │─────────────────────>│
 │                    │                              │                      │
 │                    │                              │ spawn rayon threads  │
 │                    │                              │ CFR iterate loop     │
 │                    │                              │                      │
 │                    │  event: solve_progress       │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
 │                    │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│  {iteration, time,  │
 │ Update progress UI │                              │   nash_distance}     │
 │<─ ─ ─ ─ ─ ─ ─ ─ ─│                              │                      │
 │                    │                              │ ...iterations...     │
 │                    │                              │                      │
 │                    │                              │ stopping condition   │
 │                    │                              │ met → serialize      │
 │                    │  return SolveResult           │                      │
 │                    │<─────────────────────────────│                      │
 │                    │                              │                      │
 │ Render in Study    │                              │                      │
 │ Mode              │                              │                      │
 │<───────────────────│                              │                      │
```

**Requirements addressed:** SOL-001 through SOL-018, RTS-001 through RTS-008, DSK-003, PRF-002.

### Flow 2: Solution Browse (Study Mode)

```
User                Frontend                      Tauri IPC              Rust Backend
 │                    │                              │                      │
 │ Select spot in     │                              │                      │
 │ SpotSelector       │                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke solution::load         │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ SolutionIndex.search │
 │                    │                              │─────────────────────>│
 │                    │                              │ SolutionReader.open  │
 │                    │                              │ (mmap, no full load) │
 │                    │                              │                      │
 │                    │                              │ decompress root node │
 │                    │  return StrategyData          │                      │
 │                    │<─────────────────────────────│                      │
 │                    │                              │                      │
 │ Render HandMatrix  │                              │                      │
 │ + tabs             │                              │                      │
 │<───────────────────│                              │                      │
 │                    │                              │                      │
 │ Navigate to child  │                              │                      │
 │ node (click action)│                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke solution::load_node    │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ decompress child node│
 │                    │  return NodeStrategy          │                      │
 │                    │<─────────────────────────────│                      │
 │ Update display     │                              │                      │
 │<───────────────────│                              │                      │
```

**Requirements addressed:** STU-001 through STU-016, STO-003, STO-006, PRF-004.

**Performance budget:** Spot load < 500ms for 500MB compressed file. Achieved via mmap (no full read) + partial decompression (only requested nodes).

### Flow 3: Hand History Analysis

```
User                Frontend                      Tauri IPC              Rust Backend
 │                    │                              │                      │
 │ Select files via   │                              │                      │
 │ native dialog      │                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke handhistory::import    │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ ParserRegistry       │
 │                    │                              │ .auto_detect()       │
 │                    │                              │                      │
 │                    │                              │ For each file:       │
 │                    │                              │   parse → ParsedHand │
 │                    │                              │                      │
 │                    │  event: import_progress       │                      │
 │ Update progress    │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│                      │
 │<─ ─ ─ ─ ─ ─ ─ ─ ─│                              │                      │
 │                    │                              │ For each decision:   │
 │                    │                              │   load matching sol  │
 │                    │                              │   (or solve on the   │
 │                    │                              │    fly if missing)   │
 │                    │                              │   compute EV loss    │
 │                    │                              │   classify decision  │
 │                    │                              │                      │
 │                    │                              │ Store results:       │
 │                    │                              │   SQLite INSERT      │
 │                    │                              │                      │
 │                    │  return DashboardSummary      │                      │
 │                    │<─────────────────────────────│                      │
 │                    │                              │                      │
 │ Render dashboard   │                              │                      │
 │<───────────────────│                              │                      │
```

**Requirements addressed:** ANL-001 through ANL-010, HHP-001 through HHP-012, DSK-005.

### Flow 4: Practice Session

```
User                Frontend                      Tauri IPC              Rust Backend
 │                    │                              │                      │
 │ Configure practice │                              │                      │
 │ (spot, difficulty, │                              │                      │
 │  rng mode)         │                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke practice::start        │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ Load solution        │
 │                    │                              │ Init game state      │
 │                    │                              │ Deal cards (rng mode)│
 │                    │  return PracticeHand          │                      │
 │                    │<─────────────────────────────│                      │
 │                    │                              │                      │
 │ Display table,     │                              │                      │
 │ cards, actions     │                              │                      │
 │<───────────────────│                              │                      │
 │                    │                              │                      │
 │ Choose action      │                              │                      │
 │───────────────────>│                              │                      │
 │                    │ invoke practice::act          │                      │
 │                    │─────────────────────────────>│                      │
 │                    │                              │ Compare to GTO       │
 │                    │                              │ Compute EV loss      │
 │                    │                              │ Classify decision    │
 │                    │                              │ Advance game state   │
 │                    │  return FeedbackResult        │                      │
 │                    │<─────────────────────────────│                      │
 │                    │                              │                      │
 │ Show feedback      │                              │                      │
 │ (GTO strategy,     │                              │                      │
 │  score, class)     │                              │                      │
 │<───────────────────│                              │                      │
 │                    │                              │                      │
 │       ... loop until hand complete ...             │                      │
 │                    │                              │                      │
 │                    │ invoke practice::next_hand    │                      │
 │                    │─────────────────────────────>│                      │
 │                    │  return new PracticeHand      │                      │
 │                    │<─────────────────────────────│                      │
```

**Requirements addressed:** PRA-001 through PRA-014.

---

## 7. Tauri IPC Design

### 7.1 Command Groups

All commands are async Rust functions annotated with `#[tauri::command]`. They return `Result<T, AppError>` serialized as JSON.

| Namespace | Commands | Description |
|-----------|----------|-------------|
| `solver::` | `start_solve`, `cancel_solve`, `get_solve_status` | Custom solve lifecycle (RTS-001 through RTS-008) |
| `solution::` | `load`, `load_node`, `search`, `delete`, `disk_usage`, `import`, `export` | Solution library management (STO-001 through STO-012) |
| `handhistory::` | `import`, `import_directory`, `get_import_status` | Hand history import pipeline (HHP-001 through HHP-012) |
| `analyze::` | `get_hands`, `get_hand_detail`, `get_dashboard`, `filter_hands`, `export_results` | Analysis results retrieval (ANL-001 through ANL-015) |
| `practice::` | `start`, `act`, `next_hand`, `get_session_stats`, `end_session` | Practice session management (PRA-001 through PRA-014) |
| `range::` | `compute_equity`, `grade_vs_gto`, `load_preset`, `save_preset` | Range builder operations (RNG-001 through RNG-010) |
| `nodelock::` | `lock_node`, `unlock_node`, `edit_frequencies`, `re_solve`, `compare`, `reset` | Nodelocking workflow (NLK-001 through NLK-010) |
| `reports::` | `aggregate_flops`, `aggregate_turns`, `filter_by_texture`, `export_report` | Aggregated report generation (AGG-001 through AGG-010) |
| `icm::` | `calculate_equity`, `bubble_factor`, `simulate_ft` | ICM calculations (ICM-001 through ICM-008) |
| `settings::` | `get_config`, `update_config`, `get_data_dir` | Application settings (DSK-004) |

### 7.2 Event Stream

Long-running operations use Tauri's event system for progress reporting. The frontend subscribes to named events:

| Event | Payload | Emitted by |
|-------|---------|------------|
| `solve_progress` | `{ iteration, elapsed_ms, nash_distance, estimated_remaining_ms }` | `solver::start_solve` |
| `import_progress` | `{ files_total, files_done, hands_parsed, errors }` | `handhistory::import_directory` |
| `analyze_progress` | `{ hands_total, hands_done, current_file }` | Post-import analysis pipeline |

### 7.3 Payload Convention

Request and response types are defined in a shared `types` module within `app-tauri`, serialized with `serde`. The frontend mirrors these types in TypeScript via generated type definitions.

```rust
// Example: solver command types
#[derive(Serialize, Deserialize)]
pub struct StartSolveRequest {
    pub positions: (Position, Position),
    pub stack_depth: f64,
    pub pot_size: f64,
    pub board: Option<Vec<Card>>,
    pub bet_sizes_ip: Vec<BetSize>,
    pub bet_sizes_oop: Vec<BetSize>,
    pub raise_sizes: Vec<BetSize>,
    pub raise_cap: u8,
    pub stopping: StoppingCondition,
}

#[derive(Serialize, Deserialize)]
pub struct SolveResult {
    pub session_id: String,
    pub nash_distance: f64,
    pub iterations: u64,
    pub elapsed_ms: u64,
    pub strategy_data: StrategyData,
}
```

### 7.4 Error Handling

All IPC commands return `Result<T, AppError>`. `AppError` is a Rust enum that maps to user-facing error messages:

```rust
#[derive(Debug, Serialize)]
pub enum AppError {
    SolverError(String),
    SolutionNotFound { query: String },
    ParseError { file: String, line: usize, detail: String },
    IoError(String),
    DatabaseError(String),
    InvalidConfig(String),
    Cancelled,
}
```

Frontend displays these via the `ErrorBoundary` component (UIF-007).

---

## 8. State Management

### Frontend State

| Scope | Mechanism | Contents |
|-------|-----------|----------|
| Global | React Context (`AppContext`) | Active mode, theme, keyboard shortcut registry, global loading states |
| Per-mode | `useReducer` within mode root | All mode-specific state (current spot, hand list, solve config, etc.) |
| Component | `useState` / `useRef` | Transient UI state (hover targets, animation frames, scroll positions) |
| Derived | `useMemo` | Computed values (filtered hand lists, aggregated stats, combo counts) |

### Backend State

| State | Scope | Lifetime | Storage |
|-------|-------|----------|---------|
| `SolverSession` map | `AppState` (Tauri managed) | Per-solve; removed on cancel/complete | In-memory `HashMap<SessionId, SolverSession>` |
| `SolutionIndex` | `AppState` | Application lifetime | SQLite (persisted) |
| `SolutionCache` | `AppState` | Application lifetime, LRU-evicted | In-memory, bounded by size |
| Database connection | `AppState` | Application lifetime | SQLite connection pool |
| Config | `AppState` | Application lifetime, reload on change | TOML file |

All command handlers are **stateless functions** that read from `AppState`. The only mutable shared state is the solver session map (guarded by `Mutex`).

---

## 9. Concurrency Model

```
┌──────────────────────────────────────────────────────┐
│  Tauri Runtime (Tokio async)                          │
│                                                       │
│  ┌─────────────────────────────────────────────────┐  │
│  │  IPC Command Handlers (async tasks)              │  │
│  │  - Dispatched on Tokio thread pool               │  │
│  │  - Non-blocking: await file I/O, DB queries      │  │
│  │  - Short-lived per command                        │  │
│  └──────────────────────┬──────────────────────────┘  │
│                         │ spawn_blocking              │
│  ┌──────────────────────▼──────────────────────────┐  │
│  │  Rayon Thread Pool (CPU-bound work)              │  │
│  │  - CFR tree traversal (SOL-009)                  │  │
│  │  - Parallel hand evaluation                      │  │
│  │  - Batch hand history parsing                    │  │
│  │  - Threads = num_cpus (configurable)             │  │
│  └─────────────────────────────────────────────────┘  │
│                                                       │
│  ┌─────────────────────────────────────────────────┐  │
│  │  UI Thread (Webview main thread)                 │  │
│  │  - NEVER blocked by computation                  │  │
│  │  - Receives progress via event stream            │  │
│  │  - Renders at 60fps independent of solver        │  │
│  └─────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

### Cancellation

Long-running operations (solving, batch import) accept a `CancellationToken` — an `Arc<AtomicBool>` that the solver checks between iterations. When the user clicks "Cancel" in the UI, the frontend invokes `solver::cancel_solve`, which sets the token to `true`. The solver thread exits its loop on the next iteration boundary.

### Thread Safety

- Solver regret tables: per-thread local copies merged at iteration boundaries, or lock-free atomics for shared updates
- Solution cache: `RwLock` — concurrent reads, exclusive writes
- SQLite: connection pool with `r2d2` or `deadpool-sqlite`; WAL mode allows concurrent reads
- Config: read once at startup, reloaded on explicit user action

---

## 10. Storage Architecture

### 10.1 Directory Layout

```
~/.poker-solver/
├── solutions/              # Binary solution files (zstd compressed)
│   ├── cash/
│   │   ├── 6max_100bb_btnvsbb_srp_Ks7h2d.sol
│   │   └── ...
│   ├── mtt/
│   │   └── ...
│   └── custom/
│       └── ...
├── hands/                  # Imported hand history source files
│   ├── pokerstars/
│   │   └── HH20240115_NL200.txt
│   └── ggpoker/
│       └── ...
├── data.db                 # SQLite database (WAL mode)
├── config.toml             # User configuration
└── eval_tables.bin         # Precomputed hand evaluator lookup tables
```

### 10.2 SQLite Schema (data.db)

```sql
-- Solution index (STO-006)
CREATE TABLE solution_index (
    id          INTEGER PRIMARY KEY,
    file_path   TEXT NOT NULL UNIQUE,
    game_type   TEXT NOT NULL,
    position_ip TEXT NOT NULL,
    position_oop TEXT NOT NULL,
    stack_depth REAL NOT NULL,
    pot_size    REAL NOT NULL,
    board_hash  INTEGER,           -- xxHash of canonical board
    bet_sizes   TEXT,              -- JSON array
    nash_distance REAL,
    solve_date  TEXT NOT NULL,
    file_size   INTEGER NOT NULL,
    solver_version TEXT
);
CREATE INDEX idx_solution_spot ON solution_index(game_type, position_ip, position_oop, stack_depth);
CREATE INDEX idx_solution_board ON solution_index(board_hash);

-- Analysis results (ANL-007)
CREATE TABLE analyzed_hands (
    id          INTEGER PRIMARY KEY,
    hand_hash   TEXT NOT NULL UNIQUE,  -- dedup key
    site        TEXT NOT NULL,
    game_type   TEXT NOT NULL,
    stakes      TEXT,
    played_at   TEXT,
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    position    TEXT NOT NULL,
    hole_cards  TEXT NOT NULL,
    board       TEXT,
    pot_size    REAL,
    result_bb   REAL,
    hero_decisions TEXT NOT NULL       -- JSON array of AnalyzedDecision
);
CREATE INDEX idx_hands_position ON analyzed_hands(position);
CREATE INDEX idx_hands_date ON analyzed_hands(played_at);

-- Practice history (PRA-009)
CREATE TABLE practice_sessions (
    id          INTEGER PRIMARY KEY,
    started_at  TEXT NOT NULL,
    ended_at    TEXT,
    spot_config TEXT NOT NULL,         -- JSON
    hands_played INTEGER DEFAULT 0,
    decisions    INTEGER DEFAULT 0,
    avg_score    REAL DEFAULT 0,
    classification_dist TEXT           -- JSON {perfect, good, inaccurate, wrong, blunder}
);

-- Settings (DSK-004)
CREATE TABLE settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### 10.3 Solution Files

| Property | Specification |
|----------|--------------|
| Format | Custom binary (see Section 4.4) |
| Compression | zstd, level 3 (balanced speed/ratio) |
| Target ratio | >= 3:1 compression (STO-002) |
| Strategy encoding | u16 fixed-point per action (STO-004) |
| Sparse encoding | Threshold: if single action > 95% frequency, use sparse (STO-011) |
| Access method | Memory-mapped I/O via `memmap2` crate (STO-003) |
| Checksum | xxHash64 in file footer (STO-012) |
| Metadata | JSON header, readable without full decompression (STO-005) |

### 10.4 Configuration File (config.toml)

```toml
[general]
data_dir = "~/.poker-solver"      # overridable for portable mode
theme = "dark"
language = "en"

[solver]
default_algorithm = "dcfr"         # cfr_plus | dcfr | linear_cfr
default_raise_cap = 4
thread_count = 0                   # 0 = auto-detect (num_cpus)
memory_limit_gb = 4

[display]
ev_unit = "bb"                     # bb | pot_pct
bet_unit = "pot_pct"               # pot_pct | bb | chips
four_color_deck = false
card_theme = "classic"
layout = "horizontal"              # horizontal | horizontal_rev | split | split_rev
matrix_size = "medium"             # large | medium | compact

[practice]
default_difficulty = "standard"    # simple | grouped | standard
default_speed = "normal"           # normal | fast | turbo
auto_new_hand = true

[shortcuts]
spot_selector = "j"
cycle_grouping = "s"
clear_filters = "p"
fullscreen = "q"
toggle_detail = "space"
tab_1 = "1"
tab_2 = "2"
tab_3 = "3"
tab_4 = "4"
```

---

## 11. Performance Architecture

### 11.1 Hand Evaluator (PRF-001)

- Precomputed lookup tables: ~10MB, loaded once at startup
- Two-stage lookup: 5-card table + 2-card offset for 7-card hands
- Target: 200M+ evaluations/sec on single modern x86-64 core
- Generated at first run, cached to disk as `eval_tables.bin`
- Batch evaluation mode for SIMD-friendly memory access patterns (HEV-008)

### 11.2 Solver Engine (PRF-002, PRF-005, PRF-007)

- MCCFR chance sampling keeps memory < 4GB for standard spots (PRF-005)
- Parallel tree traversal via `rayon` work-stealing thread pool (SOL-009)
- Target scaling: >= 6x speedup on 8 cores vs single-core (PRF-007)
- Standard spot (HU, single street, 3 bet sizes): < 10 seconds to < 0.5% Nash Distance (PRF-002)
- Memory layout: strategies stored as contiguous `Vec<f32>` for cache locality

### 11.3 Solution Loading (PRF-004)

- Memory-mapped I/O via `memmap2` — no full file read into RAM
- Partial decompression: only requested nodes are decompressed on demand
- Solution cache: LRU, bounded at 512MB resident memory
- Target: < 500ms from disk to rendered display for 500MB compressed file

### 11.4 UI Rendering (PRF-003)

- Hand matrix (13x13 grid) rendered on HTML5 Canvas for immediate-mode updates
- 60fps target: < 16.7ms per frame during interaction (hover, filter, navigation)
- Large lists (analyzed hands) use React virtualization (`react-window`)
- No full re-render on partial state changes — React memoization throughout

### 11.5 Startup Time (PRF-006)

- Target: interactive UI within 3 seconds on SSD
- Startup sequence: init Tauri shell → load config → render skeleton UI → load eval tables (background) → mark ready
- Eval table loading is deferred until first use if not cached

---

## 12. Security & Data Integrity

| Concern | Mitigation | Requirement |
|---------|-----------|-------------|
| Solution file corruption | xxHash64 checksum verified on load | STO-012 |
| Database crash resilience | SQLite WAL mode; journal survives unexpected shutdown | — |
| No network dependency | Core features work fully offline; no external API calls | DSK-002 |
| Hand history injection | Parser validates input structure before processing; malformed hands logged and skipped | HHP-005 |
| Config tampering | TOML parser with strict schema validation; invalid keys ignored with defaults | DAT-007 |
| Large file handling | Memory-mapped I/O prevents OOM on multi-GB solution files | STO-003, PRF-008 |

---

## 13. Extensibility Points

### 13.1 Solver Variants (`SolverAlgorithm` trait)

```rust
pub trait SolverAlgorithm: Send + Sync {
    fn name(&self) -> &str;
    fn iterate(&mut self, tree: &GameTree, iteration: u64);
    fn get_strategy(&self, info_set: &InfoSet) -> &Strategy;
    fn nash_distance(&self) -> f64;
    fn supports_parallel(&self) -> bool;
}
```

Built-in: `CfrPlusSolver`, `DiscountedCfrSolver`, `LinearCfrSolver` (SOL-001, SOL-014).
Extension: implement the trait for new variants (e.g., Pure CFR, Deep CFR).

### 13.2 Hand Evaluator (`HandEvaluator` trait)

```rust
pub trait HandEvaluator: Send + Sync {
    fn evaluate(&self, cards: &[Card; 7]) -> HandRank;
    fn evaluate_5(&self, cards: &[Card; 5]) -> HandRank;
    fn categorize(&self, rank: HandRank) -> HandCategory;
    fn compare(&self, a: HandRank, b: HandRank) -> Ordering;
}
```

Built-in: `LookupTableEvaluator` (HEV-001, HEV-002).
Extension: alternative implementations (e.g., perfect hash evaluator, Cactus Kev style).

### 13.3 Hand History Parsers (`SiteParser` trait)

```rust
pub trait SiteParser: Send + Sync {
    fn detect(content: &str) -> bool;
    fn parse(&self, content: &str) -> Result<Vec<ParsedHand>>;
    fn site_name(&self) -> &str;
}
```

Built-in: PokerStars, GGPoker, Winamax, 888poker, PartyPoker, iPoker (HHP-001).
Extension: implement the trait for additional sites, register via `ParserRegistry`.

### 13.4 ICM Models (`IcmModel` trait)

```rust
pub trait IcmModel: Send + Sync {
    fn equity(&self, stacks: &StackDistribution, payouts: &PayoutStructure) -> Vec<f64>;
    fn bubble_factor(&self, player: usize, stacks: &StackDistribution, payouts: &PayoutStructure) -> f64;
}
```

Built-in: `StandardIcm` (Malmuth-Harville), `PkoIcm`, `SatelliteIcm` (ICM-001, ICM-006, ICM-007).
Extension: custom tournament structures or bounty models.

---

## 14. Feature-to-Component Traceability Matrix

This matrix maps every requirement category to the Rust crates and frontend modules responsible for satisfying them.

| Category | ID Prefix | Count | Rust Crate(s) | Frontend Module(s) |
|----------|-----------|-------|---------------|-------------------|
| Solver Engine | SOL | 18 | `poker-solver`, `poker-core`, `poker-eval` | `solve/`, `ipc/` |
| Hand Evaluation | HEV | 10 | `poker-eval`, `poker-core` | — (backend-only) |
| Solution Storage | STO | 12 | `poker-solution` | `ipc/` (load/save commands) |
| Study Mode | STU | 16 | `poker-solution` (data source) | `study/`, `shared/HandMatrix` |
| Practice Mode | PRA | 14 | `poker-solver`, `poker-solution` | `practice/` |
| Analyze Mode | ANL | 15 | `poker-analyze`, `poker-hhparser`, `poker-solution` | `analyze/` |
| Real-Time Solver | RTS | 8 | `poker-solver`, `poker-solution` | `solve/` |
| Nodelocking | NLK | 10 | `poker-solver` | `nodelock/` |
| Aggregated Reports | AGG | 10 | `poker-solution`, `poker-eval` | `reports/` |
| Range Builder | RNG | 10 | `poker-eval` (equity calc) | `range-builder/`, `shared/HandMatrix` |
| Tournament & ICM | ICM | 8 | `poker-icm` | `solve/` (ICM config panel) |
| Hand History Parser | HHP | 12 | `poker-hhparser` | `analyze/ImportWizard` |
| UI Framework | UIF | 15 | — | `shared/` (theme, colors, layout, keyboard) |
| Hand Matrix | HMX | 12 | — | `shared/HandMatrix` |
| Desktop App (Tauri) | DSK | 10 | `app-tauri` | All modules (webview host) |
| Performance | PRF | 8 | `poker-eval`, `poker-solver`, `poker-solution` | `shared/HandMatrix` (Canvas) |
| Data & Formats | DAT | 7 | `poker-core` | `shared/` (notation, labels) |
| **TOTAL** | | **195** | | |

Every category maps to at least one Rust crate and at least one frontend module (or is backend-only with IPC exposure).

---

## 15. Dependency Graph

### 15.1 Rust Crate DAG

```
                    ┌────────────┐
                    │ poker-core │
                    └──────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐ ┌───▼──────┐ ┌──▼──────────┐
        │poker-eval │ │poker-icm │ │poker-hhparser│
        └─────┬─────┘ └──────────┘ └─────────────┘
              │
        ┌─────▼──────┐
        │poker-solver│
        └─────┬──────┘
              │
     ┌────────┼─────────────┐
     │        │             │
┌────▼─────┐  │    ┌────────▼──────┐
│poker-    │  │    │poker-analyze  │
│solution  │  │    │(depends on    │
│          │  │    │ solver,eval,  │
└──────────┘  │    │ solution,     │
              │    │ hhparser)     │
              │    └───────────────┘
              │
     ┌────────▼────────────────┐
     │       app-tauri          │
     │  (depends on all above)  │
     └──────────────────────────┘
```

**Dependency rules:**
- `poker-core` is the leaf crate — no dependencies on other workspace crates
- `poker-eval`, `poker-icm`, and `poker-hhparser` depend only on `poker-core`
- `poker-solver` depends on `poker-core` and `poker-eval`
- `poker-solution` depends on `poker-core` (not on `poker-solver` — it handles serialized data, not live solver state)
- `poker-analyze` is the most connected library crate — it orchestrates solver, evaluator, solution store, and parser
- `app-tauri` is the sole binary crate and depends on all library crates
- **No cycles exist** in this DAG

### Hard Constraint: No Dependency Cycles

`poker-solution` MUST NEVER depend on `poker-solver`. The dependency is strictly one-way:

    poker-solver → poker-solution  (solver writes solution format)
    poker-solution → poker-core     (solution uses core types)

For M6 nodelock warm-start, solution data flows through `app-tauri` as the mediator:
    app-tauri reads solution via poker-solution → passes warm-start data → poker-solver re-solves

This prevents circular crate dependencies and keeps the build DAG acyclic.

### 15.2 Frontend Module Dependencies

```
shared/          ← (depended on by all feature modules)
  HandMatrix
  CardRenderer
  ActionColorSystem
  Tooltip
  KeyboardShortcuts

ipc/             ← (depended on by all feature modules)
  Typed wrappers
  Event listeners
  Loading hooks

study/           → shared/, ipc/
practice/        → shared/, ipc/
analyze/         → shared/, ipc/
solve/           → shared/, ipc/
nodelock/        → shared/, ipc/
range-builder/   → shared/, ipc/
reports/         → shared/, ipc/
```

Feature modules depend on `shared/` and `ipc/` but never on each other. Cross-mode navigation (e.g., "Open in Study Mode" from Analyze) uses route-based navigation, not direct component imports.

---

## 16. Deployment Model

### 16.1 Build Output

- Single Tauri executable bundle per platform (DSK-001)
- Windows: `.msi` installer or `.exe` portable
- macOS: `.dmg` with signed `.app` bundle
- Linux: `.AppImage` or `.deb` package
- Primary platform: Windows 10+ (DSK-007)

### 16.2 First-Run Initialization

On first launch, the application:
1. Creates the data directory at `~/.poker-solver/`
2. Creates subdirectories: `solutions/`, `hands/`
3. Initializes the SQLite database (`data.db`) with schema
4. Generates hand evaluator lookup tables (`eval_tables.bin`, ~10MB, ~2-5 seconds)
5. Writes default `config.toml`
6. Displays a first-run welcome/setup screen

### 16.3 Auto-Update

Tauri's built-in updater checks for new versions on startup (DSK-008):
- Update manifest hosted at a configurable URL
- User prompted before download
- Update applied on next restart
- Rollback available if update fails

### 16.4 Data Portability

For portable operation (e.g., USB drive), set `data_dir` in `config.toml` to a relative path (`./data/`). All data directories and the SQLite database relocate accordingly.

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial system architecture document |

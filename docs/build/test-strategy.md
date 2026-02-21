# Test Strategy

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 6 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisites:** [Requirements](requirements.md) (Doc 1), [Architecture](architecture.md) (Doc 2), [Milestones](milestones.md) (Doc 3), [Project Structure](project-structure.md) (Doc 4), [Technical Constraints](technical-constraints.md) (Doc 5)

---

## 1. Scope & Purpose

### What This Document Defines

This document defines the **test strategy** for the GTO Poker Solver: test types per subsystem, per-milestone test plans, benchmark specifications, test data requirements, and pass/fail criteria. This is a skeleton — actual test code is written during implementation.

### Test Philosophy

1. **Every MUST requirement has a testable acceptance criterion** (from `requirements.md`). Tests verify these criteria.
2. **Tests run at every milestone gate.** A milestone is not "done" until all its tests pass AND all previous milestone tests still pass (regression).
3. **Performance tests are first-class.** Benchmarks for PRF-* requirements are not optional — they are gate checks.
4. **Test data is checked in.** Sample hand histories, reference solutions, and evaluation fixtures live in `fixtures/`.

### Test Tools

| Layer | Tool | Purpose |
|-------|------|---------|
| Rust unit/integration | `cargo test` | Per-crate and cross-crate tests |
| Rust benchmarks | `criterion` | PRF-* performance measurements |
| Frontend unit | Vitest | Component and hook tests |
| Frontend component | React Testing Library | UI interaction tests |
| IPC round-trip | Tauri test harness | Command invoke → response verification |
| E2E | Tauri WebDriver (or Playwright) | Full user workflow tests |
| Linting | `cargo clippy`, `eslint` | Code quality gates |

---

## 2. Test Types & Their Targets

### 2.1 Unit Tests

**Scope:** Single function, single module. Pure logic with no I/O or IPC.

| Target | Examples |
|--------|----------|
| `poker-core` | Card construction, hand notation, canonical index, deck shuffle |
| `poker-eval` | Hand evaluation for all 7,462 classes, equity for known matchups, draw detection |
| `poker-solver` | Game tree construction, info set hashing, strategy normalization |
| `poker-solution` | Serialization round-trip, compression ratio, quantization error |
| `poker-icm` | ICM equity for known stack distributions, bubble factor calculation |
| `poker-hhparser` | Per-site parsing of sample hands, pot reconstruction, position assignment |
| `poker-analyze` | Decision classification thresholds, EV loss computation, dashboard aggregation |
| Frontend | Hook behavior (useReducer state transitions), utility functions, type conversions |

**Location:** `src/` alongside the code (`#[cfg(test)] mod tests`) for Rust; `__tests__/` subdirectories for TypeScript.

### 2.2 Integration Tests

**Scope:** Cross-crate or cross-module interactions. May involve I/O.

| Target | Examples |
|--------|----------|
| Solver + Evaluator | Solver uses evaluator for showdown resolution; strategy makes poker sense |
| Solver + Solution | Solve → serialize → deserialize → load → verify identical strategy |
| Parser + Analyzer | Parse hand history → analyze decisions → verify EV loss is non-negative |
| Solution Index + Reader | Index query returns correct file → reader loads → strategy data is valid |

**Location:** `tests/` directory in each crate (Rust convention for integration tests).

### 2.3 IPC Tests

**Scope:** Full round-trip: TypeScript invoke → Tauri IPC → Rust handler → response → TypeScript assertion.

| Target | Examples |
|--------|----------|
| `eval::evaluate` | Send 7 cards, receive correct HandRank |
| `range::compute_equity` | Send two ranges + board, receive equity within 0.1% of reference |
| `solver::start_solve` | Start solve, receive progress events, receive final result |
| `solution::load` | Load a known solution, verify strategy data matches reference |
| `handhistory::import` | Import a test file, verify parsed hand count and fields |

**Location:** `crates/app-tauri/tests/` for Rust-side; `frontend/src/ipc/__tests__/` for TypeScript-side.

### 2.4 UI Component Tests

**Scope:** React component rendering and interaction. No backend dependency (mocked IPC).

| Target | Examples |
|--------|----------|
| HandMatrix | Renders 169 cells; hover shows tooltip; click selects hand; action filter works |
| CardRenderer | All 52 cards render; four-color deck mode works |
| RangeGrid | Paintbrush selection; weight slider; suit expansion; preset loading |
| SpotSelector | Dropdown selections; loads matching solution |
| FeedbackPanel | Shows GTO strategy; highlights player action; displays score |
| Dashboard | Renders aggregate statistics; breakdowns sum to totals |

**Location:** `frontend/src/{module}/components/__tests__/`

### 2.5 Benchmark Tests

**Scope:** Performance measurement against PRF-* targets. Run in release mode only.

| Target | Benchmark | PRF Requirement |
|--------|-----------|-----------------|
| Hand evaluator | Random 7-card evaluation throughput | PRF-001: >= 200M evals/sec |
| Solver | Standard HU flop spot solve time | PRF-002: < 10 seconds |
| UI (manual) | Frame timing during matrix interaction | PRF-003: 60fps (< 16.7ms/frame) |
| Solution reader | Load 500MB compressed solution | PRF-004: < 500ms |
| Memory profiler | Resident memory during standard use | PRF-005: < 4GB |
| Startup (manual) | Time to interactive UI | PRF-006: < 3 seconds |
| Multi-core | Solve speedup on 8 cores vs 1 | PRF-007: >= 6x |

**Location:** `crates/{crate}/benches/` using `criterion`.

### 2.6 End-to-End Tests

**Scope:** Full user workflows through the Tauri application.

| Workflow | Steps |
|----------|-------|
| Solve and browse | Configure spot → solve → view progress → save → open in Study Mode → navigate tabs |
| Practice session | Configure → play 5 hands → verify feedback → check session stats |
| HH import and analysis | Import test file → verify dashboard → replay specific hand → verify GTO overlay |
| Nodelocking | Load solution → lock node → edit frequency → re-solve → compare views |
| Range builder | Paint range → set weights → check equity → load preset → verify grade |

**Location:** `tests/e2e/` at workspace root.

---

## 3. Per-Crate Test Plans

### 3.1 poker-core

**Test file:** `crates/poker-core/src/lib.rs` (`#[cfg(test)]`)

| Test | Description | Verifies |
|------|-------------|----------|
| `test_card_construction` | All 52 cards construct correctly from rank + suit | DAT-003 |
| `test_card_packed_repr` | Card packs into u8; round-trip through pack/unpack | DAT-003 |
| `test_hand_notation` | `hand_notation()` produces "AKs", "QJo", "TT" for all 169 groups | DAT-003 |
| `test_canonical_hand_index` | All 169 groups map to unique indices 0..168 | DAT-003 |
| `test_deck_shuffle` | Shuffled deck contains all 52 unique cards | — |
| `test_deck_deterministic_seed` | Same seed produces same shuffle | — |
| `test_position_labels` | Position enum covers UTG through BB for 6-max, 9-max, HU | DAT-002 |
| `test_game_type_enum` | All 5 game types: CashNLH, MTT, SNG, SpinAndGo, HeadsUp | DAT-001 |
| `test_bet_size_variants` | PotFraction, Absolute, AllIn all construct and compare | DAT-004 |
| `test_action_enum` | Fold, Check, Call, Bet, Raise, AllIn variants | DAT-004 |
| `test_board_progression` | Board handles 0 (preflop), 3 (flop), 4 (turn), 5 (river) cards | — |
| `test_range_169_weights` | Range type holds 169 float weights 0.0–1.0 | — |

### 3.2 poker-eval

**Test file:** `crates/poker-eval/src/lib.rs` + per-module tests

| Test | Description | Verifies |
|------|-------------|----------|
| `test_all_7462_hand_classes` | Evaluate representative hands for all 7,462 classes; verify ranking order | HEV-001, HEV-005 |
| `test_hand_categories` | Categorize each class: HighCard through StraightFlush | HEV-005 |
| `test_wheel_straight` | A-2-3-4-5 ranked below 2-3-4-5-6 | HEV-001 |
| `test_split_pot` | Two identical-strength hands produce `Ordering::Equal` | HEV-009 |
| `test_kicker_comparison` | QQ-77-A beats QQ-77-K | HEV-009 |
| `test_transitivity` | If A > B and B > C, then A > C (100 random triples) | HEV-009 |
| `test_equity_known_matchups` | 10 reference matchups (AA vs KK, AKs vs QQ, etc.) within 0.1% | HEV-004 |
| `test_flush_draw_detection` | 4-to-flush on flop detected as FlushDraw | HEV-006 |
| `test_oesd_detection` | Open-ended straight draw detected | HEV-006 |
| `test_gutshot_detection` | Inside straight draw detected | HEV-006 |
| `test_combo_draw` | Flush draw + straight draw detected as ComboDraw | HEV-006 |
| `test_blocker_nut_flush` | Holding A♠ blocks opponent's nut flush when board has 2 spades | HEV-007 |
| `test_blocker_combo_reduction` | Blocker correctly reduces opponent combo count for 5 scenarios | HEV-007 |
| `test_suit_isomorphism_flop_count` | All 22,100 flops map to exactly 1,755 canonical representatives | HEV-003 |
| `test_suit_isomorphism_idempotent` | `canonicalize(canonicalize(board)) == canonicalize(board)` | HEV-003 |
| `test_suit_isomorphism_equivalent` | Suit-swapped boards produce same canonical form | HEV-003 |

**Benchmark:** `crates/poker-eval/benches/evaluator_bench.rs`
- `bench_eval_7card`: Random 7-card evaluation, target >= 200M/sec (PRF-001)
- `bench_equity_range_vs_range`: Full range equity calculation

### 3.3 poker-solver

**Test file:** `crates/poker-solver/src/lib.rs` + per-module tests + `tests/integration.rs`

| Test | Description | Verifies |
|------|-------------|----------|
| `test_game_tree_structure` | Tree for HU flop with 2 bet sizes: correct node count, legal actions at each node | SOL-004 |
| `test_raise_cap` | Game tree respects raise cap (4-5 bets/street) | SOL-006 |
| `test_all_in_threshold` | Small remaining stacks convert to all-in | SOL-016 |
| `test_cfr_convergence` | 10,000 iterations on a simple spot → Nash Distance < 0.5% pot | SOL-001, SOL-002 |
| `test_mccfr_convergence` | MCCFR converges with bounded memory | SOL-007 |
| `test_multi_street` | Flop-through-river tree produces strategy for all streets | SOL-008 |
| `test_suit_isomorphism_tree` | Suit-equivalent boards produce identical strategies | SOL-011 |
| `test_card_abstraction` | Abstracted tree has fewer info sets; Nash Distance < 1% pot | SOL-012 |
| `test_parallel_speedup` | 4-core solve at least 2x faster than 1-core | SOL-009 |
| `test_stopping_max_iterations` | Solver halts at max iteration count | SOL-010 |
| `test_stopping_nash_distance` | Solver halts when target Nash Distance reached | SOL-010 |
| `test_stopping_time_limit` | Solver halts when time limit exceeded | SOL-010 |
| `test_cancellation` | Setting cancellation token stops solver within 1 iteration | SOL-018 |
| `test_progress_reporting` | Progress channel receives iteration/time/ND updates | SOL-018 |
| `test_configurable_bet_sizes` | Tree uses exact user-specified bet sizes | SOL-005 |
| `test_stack_depth_range` | Solver handles 1bb, 20bb, 100bb, 200bb | RTS-004 |

**Benchmark:** `crates/poker-solver/benches/solver_bench.rs`
- `bench_standard_solve`: HU single-street 3-bet-size spot, target < 10 sec (PRF-002)
- `bench_multicore_scaling`: 1-core vs 8-core solve time ratio (PRF-007)
- `bench_memory_usage`: Peak resident memory during solve (PRF-005)

### 3.4 poker-solution

**Test file:** `crates/poker-solution/src/lib.rs` + per-module tests

| Test | Description | Verifies |
|------|-------------|----------|
| `test_serialize_round_trip` | serialize → compress → decompress → deserialize produces identical data | STO-001 |
| `test_compression_ratio` | zstd compression achieves >= 3:1 on typical solution | STO-002 |
| `test_mmap_loading` | 500MB file loads via mmap with < 500MB resident memory | STO-003 |
| `test_quantization_error` | u16 quantization error < 0.1% per action frequency | STO-004 |
| `test_quantization_round_trip` | Quantize f32 strategy vector to u16, dequantize, verify max absolute error <= 1/(2^16 - 1) ~= 0.0000153 | STO-004 |
| `test_quantization_nash_delta` | Solve reference spot, store with quantization, reload, measure Nash Distance delta (quantized minus unquantized) < 0.1 mbb/h | STO-004 |
| `test_quantization_distribution` | Sum of dequantized probabilities for each information set equals 1.0 +/- 0.001 | STO-004 |
| `test_metadata_without_decompress` | Metadata readable from file header without decompressing tree data | STO-005 |
| `test_index_search` | Index query by game_type + position + stack_depth returns correct entries within 100ms | STO-006 |
| `test_index_add_remove` | Adding and removing solution entries updates the index correctly | STO-006 |
| `test_disk_usage_accuracy` | Reported disk usage within 1% of actual | STO-010 |
| `test_sparse_encoding` | Sparse-encoded node is smaller than dense; decoding is identical | STO-011 |
| `test_sparse_threshold` | Nodes with single action > 95% use sparse encoding automatically | STO-011 |

**Benchmark:** `crates/poker-solution/benches/solution_bench.rs`
- `bench_load_500mb`: Load and access root node of 500MB compressed file (PRF-004)
- `bench_node_access`: Random node access latency (target < 1ms)

### 3.5 poker-icm

**Test file:** `crates/poker-icm/src/lib.rs` + per-module tests

| Test | Description | Verifies |
|------|-------------|----------|
| `test_icm_2_player` | 2-player ICM equity matches chip EV (heads-up, no ICM effect) | ICM-001 |
| `test_icm_3_player_known` | 3-player ICM with known stacks/payouts matches reference values | ICM-001 |
| `test_icm_9_player` | 9-player final table ICM completes without timeout | ICM-001 |
| `test_icm_equal_stacks` | Equal-stack players have equal ICM equity | ICM-001 |
| `test_bubble_factor_positive` | Bubble factor > 1.0 for non-chip-leader near bubble | ICM-004 |
| `test_custom_payouts` | Changing payout structure changes ICM equity values | ICM-003 |
| `test_icm_solve_conservative` | ICM-adjusted solve produces higher fold frequency vs chip-EV solve | ICM-002 |

### 3.6 poker-hhparser

**Test file:** `crates/poker-hhparser/src/lib.rs` + per-site tests + `tests/integration.rs`

**Fixture files:** `fixtures/hands/{site}/` — sample hand histories per site

| Test | Description | Verifies |
|------|-------------|----------|
| `test_pokerstars_parse` | Parse PokerStars sample → correct fields | HHP-001 |
| `test_ggpoker_parse` | Parse GGPoker sample → correct fields | HHP-001 |
| `test_winamax_parse` | Parse Winamax sample → correct fields | HHP-001 |
| `test_888poker_parse` | Parse 888poker sample → correct fields | HHP-001 |
| `test_partypoker_parse` | Parse PartyPoker sample → correct fields | HHP-001 |
| `test_ipoker_parse` | Parse iPoker sample → correct fields | HHP-001 |
| `test_field_extraction` | All required fields present: game type, stakes, stacks, cards, actions | HHP-002 |
| `test_cash_game_straddle` | Cash game with straddle parses correctly | HHP-003 |
| `test_cash_game_side_pot` | Multi-way all-in with side pots reconstructed correctly | HHP-003 |
| `test_tournament_blinds` | Tournament hand with antes and blind levels | HHP-004 |
| `test_malformed_hands` | File with 10 valid + 2 malformed: 10 parsed, 2 errors logged | HHP-005 |
| `test_encoding_utf8` | UTF-8 file with non-ASCII player names | HHP-006 |
| `test_encoding_latin1` | Latin-1 encoded file | HHP-006 |
| `test_pot_reconstruction` | Pot sizes at each decision point for 10 test hands match expected | HHP-007 |
| `test_position_6max` | Position labels correct for 6-max table | HHP-008 |
| `test_position_9max` | Position labels correct for 9-max table | HHP-008 |
| `test_position_heads_up` | Position labels correct for heads-up | HHP-008 |
| `test_auto_detect` | ParserRegistry.auto_detect() identifies correct site for each sample | HHP-010 |

### 3.7 poker-analyze

**Test file:** `crates/poker-analyze/src/lib.rs` + per-module tests

| Test | Description | Verifies |
|------|-------------|----------|
| `test_classification_thresholds` | Each EV-loss threshold maps to correct category (Perfect through Blunder) | ANL-003 |
| `test_ev_loss_nonnegative` | EV loss is >= 0 for all decisions | ANL-004 |
| `test_ev_loss_zero_for_gto` | EV loss is 0 when player action matches GTO pure strategy | ANL-004 |
| `test_dashboard_aggregation` | Dashboard totals equal sum of individual hand analyses | ANL-007 |
| `test_position_breakdown_sums` | Position breakdowns sum to overall totals | ANL-008 |
| `test_street_breakdown_sums` | Street breakdowns sum to overall totals | ANL-009 |
| `test_action_breakdown_complete` | Every action categorized; no uncategorized actions | ANL-010 |

### 3.8 app-tauri (IPC)

**Test file:** `crates/app-tauri/tests/commands.rs`

| Test | Description | Verifies |
|------|-------------|----------|
| `test_eval_command` | Invoke `eval::evaluate` → correct HandRank | DSK-003 |
| `test_range_equity_command` | Invoke `range::compute_equity` → correct equity | DSK-003 |
| `test_settings_round_trip` | `settings::update_config` → `settings::get_config` → same values | DSK-004 |
| `test_solver_lifecycle` | start_solve → progress events → cancel_solve → status is cancelled | DSK-003, SOL-018 |
| `test_solution_load_command` | Load known solution → strategy data matches reference | DSK-003 |
| `test_error_serialization` | AppError variants serialize to frontend-parseable JSON | DSK-003 |

---

## 4. Per-Frontend-Module Test Plans

### 4.1 shared/ Tests

**Location:** `frontend/src/shared/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `HandMatrix.render` | 169 cells with correct hand labels; diagonal = pairs | HMX-001 |
| `HandMatrix.hover` | Hover shows tooltip with hand name, frequencies, EV | HMX-003 |
| `HandMatrix.click` | Click selects hand; linked displays update | HMX-005 |
| `HandMatrix.filter` | Action filter highlights correct subset of hands | HMX-004 |
| `HandMatrix.overlay` | Switch between Strategy, EV, Equity, EQR overlays | HMX-006 |
| `HandMatrix.combos` | Combo counts correct; reduce when board cards dealt | HMX-007 |
| `HandMatrix.threshold` | Frequency threshold filter includes/excludes correct hands | HMX-009 |
| `CardRenderer.render` | All 52 cards render with correct rank/suit | UIF-009 |
| `ActionColorSystem.consistent` | Bet=red, check=green, fold=blue across all uses | UIF-004 |
| `Navigation.modes` | All mode entries render; active mode highlighted | UIF-005 |
| `ErrorBoundary.display` | Error state shows user-friendly message | UIF-007 |
| `Tooltip.show` | Tooltip appears within 500ms of hover | UIF-008 |

### 4.2 range-builder/ Tests

**Location:** `frontend/src/range-builder/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `RangeGrid.paint` | Click + drag selects multiple cells | RNG-002 |
| `WeightControls.slider` | Slider sets weight 0–100%; cell opacity updates | RNG-003 |
| `SuitExpander.expand` | Cell expansion shows correct suit combos (4/6/12) | RNG-005 |
| `PresetSelector.load` | Loading UTG preset fills correct cells | RNG-006 |
| `ComboLock.lock` | Locked combos survive bulk clear | RNG-007 |
| `RangeSummary.stats` | Combo count, %, equity display correct values | RNG-004 |
| `RangeColorLegend.coding` | Unselected=dark, full=color, partial=gradient | RNG-010 |

### 4.3 study/ Tests

**Location:** `frontend/src/study/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `StudyPage.tabs` | All 4 tabs render; switching is instant | STU-001 |
| `SpotSelector.load` | Changing parameters loads matching solution | STU-002 |
| `BoardSelector.cards` | 52 cards selectable; dealt cards excluded | STU-003 |
| `StrategyTab.frequencies` | Frequencies sum to 100% per hand | STU-004 |
| `RangesTab.display` | Range shows only hands reaching current node | STU-005 |
| `BreakdownTab.categories` | All hands assigned to exactly one category | STU-006 |
| `ActionSequence.navigate` | Click action navigates to correct tree node | STU-008 |
| `MetricOverlay.switch` | Each overlay shows correct data type | STU-009 |
| `EvDisplay.values` | Frequency-weighted action EVs sum to overall EV | STU-010 |

### 4.4 practice/ Tests

**Location:** `frontend/src/practice/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `GameTable.modes` | Three game modes initialize at correct starting point | PRA-001 |
| `ActionButtons.difficulty` | Each difficulty level shows correct action count | PRA-003 |
| `FeedbackPanel.gto` | Shows complete GTO strategy; player action highlighted | PRA-006 |
| `ScoreDisplay.classification` | 5 classification categories render with correct thresholds | PRA-005 |
| `SessionStats.totals` | Stats internally consistent: sum of classifications = total decisions | PRA-009 |
| `RngSelector.modes` | Three RNG modes selectable | PRA-007 |

### 4.5 analyze/ Tests

**Location:** `frontend/src/analyze/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `ImportWizard.select` | File dialog triggers; progress bar shows | ANL-001, ANL-002 |
| `HandReplay.navigate` | Forward/backward/jump-to-street navigation | ANL-005 |
| `GtoOverlay.display` | GTO strategy shown at decision nodes | ANL-006 |
| `Dashboard.metrics` | Total hands, avg EV loss, classification distribution | ANL-007 |
| `PositionBreakdown.sums` | Per-position stats sum to overall | ANL-008 |
| `StreetBreakdown.sums` | Per-street stats sum to overall | ANL-009 |
| `ActionBreakdown.complete` | All actions categorized | ANL-010 |

### 4.6 nodelock/ Tests

**Location:** `frontend/src/nodelock/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `LockControls.toggle` | Lock/unlock/reset buttons toggle state | NLK-004 |
| `FrequencyEditor.auto_adjust` | Changing one frequency auto-adjusts others to 100% | NLK-002, NLK-007 |
| `ComparisonView.diff` | GTO vs nodelock differences highlighted | NLK-006 |

### 4.7 reports/ Tests

**Location:** `frontend/src/reports/components/__tests__/`

| Test | Description | Verifies |
|------|-------------|----------|
| `AggregateChart.render` | Bar graph with data labeled by action | AGG-004 |
| `AggregateTable.sort` | Table sortable by any column | AGG-005 |
| `BoardTextureFilters.apply` | Monotone filter selects only 3-suited flops | AGG-003 |
| `MetricSelector.types` | Strategy, EV, Equity, EQR all selectable | AGG-002 |

---

## 5. Per-Milestone Test Gates

### 5.1 M1 Gate: Foundation & Hand Matrix

**All of the following must pass:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | poker-core unit tests | All tests in §3.1 | DAT-001–004 |
| 2 | poker-eval unit tests | All tests in §3.2 | HEV-001–007, HEV-009 |
| 3 | poker-eval benchmarks | Evaluator >= 200M evals/sec (release build) | PRF-001 |
| 4 | HandMatrix component tests | Render, hover, click, filter, overlay, combos, threshold | HMX-001–009 |
| 5 | range-builder/ component tests | Paint, weight, suit, preset, lock, summary, color | RNG-001–007, RNG-010 |
| 6 | shared/ component tests | Card renderer, action colors, navigation, errors, tooltips | UIF-001–009 |
| 7 | IPC tests (eval, range, settings) | Typed round-trip commands work | DSK-003 |
| 8 | App launches | Tauri app starts, renders dark theme UI | DSK-001, UIF-001 |
| 9 | Keyboard shortcuts | J, S, P, Q, Space, 1-4 trigger actions | UIF-002 |
| 10 | Responsive layout | Renders at 1280x720 and 3840x2160 | UIF-003 |
| 11 | File dialogs | Native open/save dialogs work | DSK-005 |
| 12 | Settings persistence | Settings survive restart | DSK-004 |
| 13 | Data directory | `~/.poker-solver/` created with expected structure | DSK-002 |
| 14 | `cargo clippy` | No warnings | — |
| 15 | `eslint` | No errors | — |
| 16 | Startup baseline | Measure application startup time from launch to interactive. Record as baseline for PRF-006 (target: < 3s in M7). No pass/fail threshold in M1 — measurement only. | — |

### 5.2 M2 Gate: Solver Engine & Solution Storage

**All M1 gate tests still pass (regression), plus:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | poker-solver unit tests | All tests in §3.3 | SOL-001–012, SOL-016, SOL-018 |
| 2 | poker-solution unit tests | All tests in §3.4 | STO-001–006, STO-010, STO-011 |
| 3 | Solver benchmark | Standard spot < 10 sec (release build) | PRF-002 |
| 4 | Memory benchmark | Standard solve < 4 GB resident | PRF-005 |
| 5 | Solver integration test | Solve → serialize → load → verify strategy identity | SOL-001, STO-001 |
| 6 | solve/ component tests | Config panel, progress display, results viewer | RTS-001–005 |
| 7 | IPC tests (solver, solution) | start_solve, cancel_solve, load, search commands | DSK-003 |
| 8 | Event streaming | solve_progress events received with correct fields | SOL-018 |
| 9 | Solution round-trip | Write → read → verify bit-identical strategies | STO-001, STO-004 |
| 10 | Solution loading benchmark | 500 MB file loads in < 500ms | PRF-004 |

### 5.3 M3 Gate: Study Mode & Solution Browsing

**All M1+M2 gate tests still pass, plus:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | poker-icm unit tests | All tests in §3.5 | ICM-001–004 |
| 2 | study/ component tests | All tests in §4.3 | STU-001–010, STU-012 |
| 3 | Study Mode integration | Load solution → navigate 4 tabs → all data correct | STU-001 |
| 4 | Frame timing | UI interactions at 60fps (< 16.7ms/frame) | PRF-003 |
| 5 | Solution load time | < 500ms from disk to displayed Study Mode for 500MB file | PRF-004 |
| 6 | ICM integration | ICM-adjusted solve produces more conservative strategy | ICM-002 |
| 7 | IPC tests (icm) | ICM equity and bubble factor commands | DSK-003 |

### 5.4 M4 Gate: Practice Mode

**All M1+M2+M3 gate tests still pass, plus:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | practice/ component tests | All tests in §4.4 | PRA-001–009 |
| 2 | GTO opponent test | 10,000 sampled decisions: frequencies match solver within 2% | PRA-002 |
| 3 | RNG mode test | High/Low/Off distributions statistically correct over 1,000 hands | PRA-007 |
| 4 | Practice IPC tests | start, act, next_hand, get_session_stats commands | DSK-003 |
| 5 | Session stats consistency | Sum of classifications = total decisions | PRA-009 |

### 5.5 M5 Gate: Hand History Analysis

**All M1+M2+M3 gate tests still pass (M4 tests too if M4 completed), plus:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | poker-hhparser unit tests | All tests in §3.6 | HHP-001–008 |
| 2 | poker-analyze unit tests | All tests in §3.7 | ANL-003, ANL-004, ANL-007–010 |
| 3 | analyze/ component tests | All tests in §4.5 | ANL-001–002, ANL-005–010 |
| 4 | Parser integration | Each of 6 site sample files parses without error | HHP-001 |
| 5 | Analysis pipeline | Parse → analyze → dashboard → verify totals match | ANL-007 |
| 6 | Import progress | Progress events stream correctly during batch import | ANL-002 |
| 7 | IPC tests (handhistory, analyze) | import, get_dashboard, filter_hands commands | DSK-003 |

### 5.6 M6 Gate: Nodelocking & Aggregated Reports

**All M1–M5 gate tests still pass, plus:**

| # | Test Category | What Passes | Req IDs |
|---|--------------|-------------|---------|
| 1 | nodelock/ component tests | All tests in §4.6 | NLK-001–006 |
| 2 | reports/ component tests | All tests in §4.7 | AGG-002–005 |
| 3 | Nodelock integration | Lock → re-solve → exploit strategy has higher EV against locked strategy | NLK-003 |
| 4 | Frequency editor | Auto-adjustment maintains 100% total | NLK-007 |
| 5 | Aggregate reports | 1,755 flops covered; texture filters reduce set correctly | AGG-001, AGG-003 |
| 6 | IPC tests (nodelock, reports) | All command round-trips | DSK-003 |
| 7 | **MUST gate check** | All 129 MUST requirements verified by tests | All MUST |

### 5.7 M7 Gate: SHOULD Enhancements

**All M1–M6 gate tests still pass, plus:**

Each of the 42 SHOULD requirements passes its acceptance criterion from `requirements.md`. No regressions to any MUST feature.

### 5.8 M8 Gate: MAY Polish

**All M1–M7 gate tests still pass, plus:**

Each implemented MAY requirement passes its acceptance criterion. No regressions.

---

## 6. Benchmark Specifications

### Reference Hardware Baseline

All performance benchmarks (PRF-001 through PRF-007) are measured on:

| Component | Specification |
|-----------|--------------|
| CPU | Intel Core i7-10700K (8 cores / 16 threads, 3.8 GHz base) |
| RAM | 32 GB DDR4-3200 |
| Storage | NVMe SSD (>= 2 GB/s sequential read) |
| GPU | Not required (CPU-only computation) |
| OS | Windows 10/11 or Linux (Ubuntu 22.04+) |

Results on slower hardware may differ. Performance targets are minimum thresholds on reference hardware — actual user hardware may be faster or slower.

### 6.1 PRF-001: Hand Evaluator Throughput

| Specification | Value |
|---------------|-------|
| **Target** | >= 200M 7-card evaluations per second, single core |
| **Method** | `criterion` benchmark; evaluate 10M random 7-card hands in tight loop; report throughput |
| **Hardware baseline** | Modern x86-64 (Intel i7/AMD Ryzen 7, 2020+); results logged with CPU model |
| **Build mode** | `--release` only; debug builds are not measured |
| **Variance** | < 5% across 10 runs |
| **Active from** | M1 |

### 6.2 PRF-002: Solver Speed

| Specification | Value |
|---------------|-------|
| **Target** | < 10 seconds for standard HU single-street 3-bet-size spot to < 0.5% Nash Distance |
| **Method** | `criterion` benchmark; 20 representative spots; report mean and p95 |
| **Hardware baseline** | 8-core CPU, 32GB RAM |
| **Build mode** | `--release` |
| **Spots** | BTN vs BB, 100bb, Q♠T♠7♥ flop, bet sizes: 33%, 75%, 150% |
| **Active from** | M2 |

### 6.3 PRF-003: UI Frame Timing

| Specification | Value |
|---------------|-------|
| **Target** | 60fps during matrix interactions (< 16.7ms per frame, no frame > 33ms) |
| **Method** | Chrome DevTools Performance timeline in Tauri dev mode; measure during hover, filter, tab switch |
| **Measurement** | Manual observation + `performance.now()` instrumentation in Canvas render loop |
| **Active from** | M3 |

### 6.4 PRF-004: Solution Loading Speed

| Specification | Value |
|---------------|-------|
| **Target** | < 500ms from disk to rendered display for 500MB compressed solution |
| **Method** | `criterion` benchmark; timer from `SolutionReader::open()` to root node Strategy available |
| **Hardware baseline** | SSD storage (NVMe or SATA SSD) |
| **Active from** | M2 (file load), M3 (full display pipeline) |

### 6.5 PRF-005: Memory Budget

| Specification | Value |
|---------------|-------|
| **Target** | < 4GB resident memory for standard use; < 16GB during solve |
| **Method** | `memory_profiler` or OS-level monitoring during standard workflow (load solution, browse, practice) |
| **Active from** | M2 |

### 6.6 PRF-006: Startup Time

| Specification | Value |
|---------------|-------|
| **Target** | < 3 seconds from launch to interactive UI |
| **Method** | Timer from process start to first user-interactive frame (navigation clickable) |
| **Hardware baseline** | SSD storage |
| **Active from** | M7 (SHOULD) |

### 6.7 PRF-007: Multi-Core Scaling

| Specification | Value |
|---------------|-------|
| **Target** | >= 6x speedup on 8 cores vs single-core solve |
| **Method** | Run same benchmark spot with `RAYON_NUM_THREADS=1` and `RAYON_NUM_THREADS=8`; compute ratio |
| **Active from** | M7 (SHOULD) |

---

## 7. Test Data & Fixtures

### 7.1 Hand Evaluation Reference Data

**Location:** `fixtures/eval/`

| File | Contents | Used By |
|------|----------|---------|
| `hand_classes_7462.json` | All 7,462 distinct hand classes with rank ordering | poker-eval unit tests |
| `equity_reference.json` | 10 known equity matchups (e.g., AA vs KK = 81.95%) | poker-eval equity tests |
| `draw_detection.json` | Board + hand → expected draw types | poker-eval draw tests |
| `blocker_scenarios.json` | 5 blocker reduction test cases | poker-eval blocker tests |
| `isomorphism_mapping.json` | Sample flop → canonical form mappings | poker-eval isomorphism tests |

### 7.2 Hand History Samples

**Location:** `fixtures/hands/`

| Directory | Contents | Used By |
|-----------|----------|---------|
| `pokerstars/` | 5 cash game hands, 5 tournament hands, 1 malformed hand | poker-hhparser tests |
| `ggpoker/` | 5 cash game hands, 3 tournament hands | poker-hhparser tests |
| `winamax/` | 3 cash game hands, 2 tournament hands | poker-hhparser tests |
| `888poker/` | 3 cash game hands | poker-hhparser tests |
| `partypoker/` | 3 cash game hands | poker-hhparser tests |
| `ipoker/` | 3 cash game hands | poker-hhparser tests |
| `edge_cases/` | Straddle hand, multi-way side pot, UTF-8 names, Latin-1 encoding | poker-hhparser edge case tests |

### 7.3 Reference Solutions

**Location:** `fixtures/solutions/`

| File | Contents | Used By |
|------|----------|---------|
| `reference_hu_flop.sol` | Pre-solved HU flop spot with known strategies | poker-solution round-trip tests |
| `reference_strategies.json` | Expected strategy values for specific nodes in reference solution | Verification in IPC and UI tests |

### 7.4 ICM Reference Calculations

**Location:** `fixtures/eval/icm_reference.json`

| Scenario | Contents | Used By |
|----------|----------|---------|
| 3-player equal stacks | Known ICM equity values | poker-icm tests |
| 9-player final table | Known ICM + bubble factor values | poker-icm tests |
| Satellite (3 tickets, 5 players) | Known satellite ICM values | poker-icm tests (M7) |

---

## 8. Pass/Fail Criteria Summary Table

Every milestone acceptance criterion mapped to its test type and pass condition.

### M1 Criteria (23 items)

| # | Criterion | Test Type | Pass Condition |
|---|-----------|-----------|----------------|
| 1 | poker-core types pass unit tests | Unit | All tests green |
| 2 | 7,462 hand classes correct | Unit | All classes ranked correctly |
| 3 | Evaluator >= 200M evals/sec | Benchmark | Throughput above threshold |
| 4 | Equity within 0.1% for 10 matchups | Unit | All within tolerance |
| 5 | Draw detection correct | Unit | All draw types identified |
| 6 | Blocker reduction for 5 scenarios | Unit | Combo counts match expected |
| 7 | 22,100 → 1,755 canonical flops | Unit | Exactly 1,755 unique outputs |
| 8 | Hand ranking transitive | Unit | No transitivity violations |
| 9 | Tauri app launches with dark theme | E2E/Manual | App window opens, dark background |
| 10 | 13x13 matrix renders correctly | Component | 169 cells, correct labels |
| 11 | Matrix interactions work | Component | Hover, click, filter, overlay pass |
| 12 | Range builder functional | Component | Paint, weight, suit, preset pass |
| 13 | Range summary correct | Component | Combo count, %, equity correct |
| 14 | IPC round-trip works | IPC | All command wrappers return typed data |
| 15 | Keyboard shortcuts work | Component | 8 shortcuts trigger actions |
| 16 | Responsive layout | Manual | 1280x720 and 3840x2160 |
| 17 | Action colors consistent | Component | Red/green/blue verified |
| 18 | Navigation works | Component | All modes accessible |
| 19 | Error display and tooltips | Component | Messages and tooltips appear |
| 20 | Cards render correctly | Component | 52 cards rendered |
| 21 | File dialogs and settings persist | IPC/Manual | Settings survive restart |
| 22 | Window operations work | Manual | Window opens, renders dark theme UI, closes cleanly; min/max/restore functional |
| 23 | Offline operation | Manual | No network calls |

> **Alignment note (item 22):** This test validates M1 AC only (window opens, renders, closes cleanly). Full window management (resize persistence, minimize/restore state, multi-monitor) is validated in later milestones.

### M2 Criteria (23 items)

| # | Criterion | Test Type | Pass Condition |
|---|-----------|-----------|----------------|
| 1 | Game tree correct structure | Unit | Node count, legal actions verified |
| 2 | CFR converges < 0.5% ND | Unit | Nash Distance below threshold |
| 3 | MCCFR bounded memory | Unit/Bench | < 4 GB resident |
| 4 | Multi-street strategies present | Unit | All streets have output |
| 5 | Suit isomorphism in tree | Unit | Equivalent boards → same strategy |
| 6 | Card abstraction reduces info sets | Unit | Fewer info sets; ND < 1% |
| 7 | Parallel speedup | Benchmark | > 2x on 4 cores |
| 8 | Stopping conditions work | Unit | Halts at each condition |
| 9 | Cancellation works | Unit | Stops within 1 iteration |
| 10 | All-in threshold | Unit | Auto-converts to all-in |
| 11 | Progress events stream | IPC | Frontend receives updates |
| 12 | Serialization round-trip | Unit | Bit-identical strategies |
| 13 | Compression >= 3:1 | Unit | Ratio above threshold |
| 14 | Mmap loading < 500ms | Benchmark | Within time budget |
| 15 | Metadata without decompress | Unit | Header readable standalone |
| 16 | Index query < 100ms | Unit | Within time budget |
| 17 | Disk usage accurate | Unit | Within 1% |
| 18 | RT solver accepts any config | Integration | Arbitrary configs produce results |
| 19 | Standard spot < 10 sec | Benchmark | Within time budget |
| 20 | Fixed + Auto sizing modes | Unit | Both produce valid results |
| 21 | 1bb to 200bb stack depths | Unit | All produce results |
| 22 | Cached re-query < 100ms | Unit | Within time budget |
| 23 | Solve config UI functional | Component | All inputs work |

### M3 — Study Mode Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | Solution browser loads strategy for any solved node | Strategy displayed within 500ms | Timeout, blank, or error |
| 2 | Hand matrix renders at 60fps during interaction | Average frame time < 16.7ms | Dropped frames or jank |
| 3 | Range display shows correct frequencies per hand | Frequencies match solver output +/- 0.1% | Mismatch or missing hands |
| 4 | Breakdown tab shows EV for each action | EV values match solution data | Incorrect values or crash |
| 5 | Navigation between nodes updates all tabs | All 4 tabs refresh correctly | Stale data in any tab |

### M4 — Practice Mode Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | Practice hand dealt from valid spot | Hand matches configured game parameters | Invalid hand or crash |
| 2 | GTO feedback shown after each decision | Correct EV loss displayed | Wrong feedback or no feedback |
| 3 | Session statistics track accuracy | Stats update after each hand | Stats missing or incorrect |
| 4 | Practice uses weighted hand distribution | Interesting spots appear more frequently when RNG mode is High/Low | Uniform random only when High/Low selected |

### M5 — Analysis Mode Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | Hand history import parses at least 3 site formats | PokerStars, GGPoker, and 1 other parse correctly | Parse failure on supported format |
| 2 | EV loss calculated for each decision | EV loss matches manual calculation +/- 0.01 bb | Incorrect EV loss |
| 3 | Dashboard shows aggregate statistics | Correct aggregation across imported hands | Wrong totals or missing data |
| 4 | Hand replay shows action-by-action with GTO overlay | GTO line shown alongside actual play | Missing overlay or wrong GTO line |

### M6 — Nodelock & Reports Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | Strategy lock applies and holds during re-solve | Locked node strategy unchanged after re-solve | Lock ignored or modified |
| 2 | Downstream nodes re-solved with correct reaching probabilities | Downstream strategies change appropriately | Unchanged or incorrect downstream |
| 3 | Cascade propagation follows defined semantics | Multi-level locks produce valid strategies | Invalid strategies or errors |
| 4 | Aggregate reports generated across solved spots | Reports show correct cross-spot statistics | Incorrect aggregation or crash |

### M7 — Performance & SHOULD Requirements Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | DCFR and Linear CFR converge to GTO | Nash Distance < 0.5% for reference spot | Divergence or incorrect strategies |
| 2 | Startup time < 3 seconds (PRF-006) | Cold start measured < 3s on reference hardware | Exceeds 3 seconds |
| 3 | Solver scales >= 6x on 8 cores (PRF-007) | Measured speedup >= 6.0x | Speedup < 6.0x |
| 4 | All SHOULD requirements implemented pass their individual tests | Per-requirement test passes | Any SHOULD test fails |

### M8 — Advanced Features Pass/Fail Criteria

| # | Criterion | Pass | Fail |
|---|-----------|------|------|
| 1 | Multi-way pot solver produces valid strategies | 3-player strategies sum to 1.0, converge | Invalid strategies or non-convergence |
| 2 | Each implemented MAY feature individually toggleable | Feature flags enable/disable cleanly | Hard dependency or crash |
| 3 | All implemented MAY features pass their individual acceptance criteria | Per-requirement test passes | Any MAY test fails |
| 4 | No regressions to MUST or SHOULD features | Full regression suite passes | Any regression detected |

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial test strategy document |

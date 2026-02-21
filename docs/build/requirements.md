# Requirements Specification

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 1 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell

---

## Conventions

### RFC 2119 Keywords

The key words "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY" in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt):

| Keyword | Meaning |
|---------|---------|
| **MUST** | Absolute requirement. The system is non-functional or non-conforming without it. |
| **MUST NOT** | Absolute prohibition. |
| **SHOULD** | Recommended. May be omitted only with documented justification. |
| **SHOULD NOT** | Discouraged. May be included only with documented justification. |
| **MAY** | Optional. Included at implementer discretion. |

### Requirement ID Format

Each requirement has a stable identifier: `PREFIX-NNN`

- `PREFIX` — 3-letter category code (e.g., `SOL` for Solver Engine)
- `NNN` — zero-padded sequential number within the category

### Priority Mapping

| Priority | RFC 2119 | Implementation Phase |
|----------|----------|---------------------|
| P0 — Core | MUST | Milestone 1-3 |
| P1 — Important | SHOULD | Milestone 4-6 |
| P2 — Enhancement | MAY | Post-MVP or backlog |

### Requirement Entry Format

Each requirement follows this structure:

```
### PREFIX-NNN: Title [MUST|SHOULD|MAY]
Description using RFC 2119 language.

**Source:** docs/NN_filename.md
**Depends on:** PREFIX-NNN (or "—" if none)
**Verified by:** Testable acceptance criterion
```

---

## 1. Solver Engine (SOL)

### SOL-001: CFR Algorithm Implementation [MUST]
The solver engine MUST implement Counterfactual Regret Minimization that converges to a Nash Equilibrium. The implementation MUST support at least one modern CFR variant (CFR+, Discounted CFR, or Linear CFR).

**Source:** docs/02_solver_engine.md
**Depends on:** —
**Verified by:** Solver produces a strategy profile; exploitability decreases monotonically over iterations.

### SOL-002: Nash Distance Convergence [MUST]
The solver MUST converge to a strategy with exploitability (Nash Distance) below 0.5% of the pot for heads-up postflop scenarios within a reasonable number of iterations on consumer hardware (8-core CPU, 32GB RAM).

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-001
**Verified by:** Nash Distance measurement < 0.5% pot on benchmark spots (e.g., BTN vs BB single-raised pot, 100bb deep).

### SOL-003: Heads-Up Postflop Solving [MUST]
The solver MUST support heads-up (2-player) postflop game trees from any flop, turn, or river starting point with configurable bet sizes, stack depths, and pot sizes.

**Source:** docs/02_solver_engine.md, docs/06_gto_wizard_ai.md
**Depends on:** SOL-001, HEV-001
**Verified by:** Solver accepts arbitrary postflop configurations and produces strategy output for both players.

### SOL-004: Game Tree Construction [MUST]
The solver MUST construct valid game trees with decision nodes (player actions), chance nodes (card deals), and terminal nodes (showdown/fold). The tree MUST enforce poker rules: legal actions, pot management, and hand progression through streets.

**Source:** docs/02_solver_engine.md
**Depends on:** —
**Verified by:** Generated trees pass structural validation — no illegal actions, correct pot arithmetic, proper street transitions.

### SOL-005: Configurable Bet Sizing [MUST]
The solver MUST accept user-defined bet sizes as percentages of the pot (e.g., 33%, 50%, 75%, 100%, 150%) and as absolute chip amounts. Multiple bet sizes per decision node MUST be supported.

**Source:** docs/02_solver_engine.md, docs/06_gto_wizard_ai.md
**Depends on:** SOL-004
**Verified by:** Solver generates strategies with the exact bet sizes provided in configuration.

### SOL-006: Raise Cap [MUST]
The solver MUST enforce a configurable raise cap per street. The default MUST be a maximum of 4-5 bets/raises per street to bound tree size.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-004
**Verified by:** Game tree does not contain more raises than the configured cap on any street.

### SOL-007: Chance Sampling (MCCFR) [MUST]
The solver MUST implement Monte Carlo CFR (external sampling or chance sampling) to reduce memory consumption by sampling chance nodes rather than enumerating all possible outcomes.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-001
**Verified by:** Standard usage (browsing solutions, practice mode, analysis): < 4 GB resident memory. During active solve: < 4 GB for standard spots (HU, single street, ≤3 bet sizes); solver still converges to < 0.5% Nash Distance.

### SOL-008: Multi-Street Solving [MUST]
The solver MUST solve complete postflop game trees spanning flop through river (flop→turn→river) in a single solve session.

**Source:** docs/02_solver_engine.md, docs/06_gto_wizard_ai.md
**Depends on:** SOL-003, SOL-004
**Verified by:** Strategy output exists for all streets in a multi-street solve; strategy at river terminal nodes produces valid showdown comparisons.

### SOL-009: Parallel Solving [MUST]
The solver MUST utilize multiple CPU cores for parallel tree traversal and regret updates. Work MUST be distributed across available threads.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-001
**Verified by:** Solver utilization scales across cores; solving time decreases with additional threads (measured speedup > 2x on 4 cores vs 1 core).

### SOL-010: Iteration Control [MUST]
The solver MUST support stopping conditions including: maximum iteration count, target Nash Distance threshold, and elapsed time limit. The user MUST be able to configure at least one of these.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-001, SOL-002
**Verified by:** Solver halts when any configured stopping condition is met.

### SOL-011: Suit Isomorphism [MUST]
The solver MUST exploit suit isomorphisms to reduce the number of strategically distinct flops from 22,100 to 1,755, reducing memory and computation by approximately 12x.

**Mathematical basis:** 52-choose-2 = 1,326 hole card combos. Under suit isomorphism, there are 169 strategically distinct hands (13 pairs + 78 suited + 78 offsuit). For flop textures, the 22,100 raw flop combinations (50-choose-3) reduce to ~1,755 suit-isomorphic equivalence classes. This reduction is well-established in the poker AI literature (Johanson 2007; Waugh et al. 2009) and follows from the 4! = 24 suit permutations minus symmetry-breaking constraints imposed by the hole cards.

**Source:** docs/02_solver_engine.md, docs/08_aggregated_reports.md
**Depends on:** HEV-003
**Verified by:** Solver treats suit-equivalent boards identically; solution for A♠K♠7♦ matches A♥K♥7♣.

### SOL-012: Card Abstraction/Bucketing [MUST]
The solver MUST support card abstraction through equity bucketing or similar hand clustering to reduce the number of information sets in large game trees.

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-001
**Verified by:** Abstracted solutions have fewer information sets than unabstracted; Nash Distance remains acceptable (< 1% pot).

### SOL-013: Preflop Range Solving [SHOULD]
The solver SHOULD support preflop range computation for heads-up and 6-max formats, producing open, call, 3-bet, and 4-bet ranges for each position.

**Source:** docs/02_solver_engine.md, docs/03_study_mode.md
**Depends on:** SOL-001
**Verified by:** Solver produces preflop range charts for all 6-max positions; ranges sum to valid frequencies (0-100% for each hand combo).

### SOL-014: Discounted CFR Variants [SHOULD]
The solver SHOULD implement Discounted CFR (DCFR) or Linear CFR for faster convergence compared to vanilla CFR, applying time-based discounting to earlier iteration regrets.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-001
**Verified by:** DCFR converges to target Nash Distance in fewer iterations than vanilla CFR on the same benchmark spot.

### SOL-015: Dynamic Bet Size Discovery [SHOULD]
The solver SHOULD support a mode that automatically identifies optimal bet sizes from a continuous range, rather than requiring user-specified fixed sizes.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-005
**Verified by:** Solver outputs an optimal sizing that was not in the predefined set; the discovered size produces lower exploitability than the nearest fixed size.

### SOL-016: All-In Threshold [MUST]
The solver MUST automatically convert bets to all-in when the remaining stack is small relative to the bet (e.g., when a bet would leave less than a configurable threshold of the pot behind).

**Source:** docs/02_solver_engine.md, docs/06_gto_wizard_ai.md
**Depends on:** SOL-005
**Verified by:** Bets that would leave less than the threshold automatically become all-in in the game tree.

### SOL-017: 3-Way Pot Solving [MAY]
The solver MAY support 3-player postflop solving for multiway pots.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-003
**Verified by:** Solver accepts a 3-player configuration and produces strategy for all three players.

### SOL-018: Solve Progress Reporting [MUST]
The solver MUST report progress during a solve: current iteration count, elapsed time, and current Nash Distance estimate. Progress MUST be observable from the frontend.

**Source:** docs/06_gto_wizard_ai.md, docs/11_ui_ux_screens.md
**Depends on:** SOL-001, DSK-001
**Verified by:** Frontend displays updating progress metrics during a solve; values refresh at least once per second.

---

## 2. Hand Evaluation (HEV)

### HEV-001: 5-to-7 Card Hand Evaluation [MUST]
The hand evaluator MUST correctly rank any poker hand of 5, 6, or 7 cards according to standard Texas Hold'em hand rankings (Royal Flush through High Card).

**Source:** docs/02_solver_engine.md
**Depends on:** —
**Verified by:** Evaluator returns correct ranking for all 7,462 distinct hand classes; comprehensive unit tests covering edge cases (e.g., wheel straight, split pots).

### HEV-002: Lookup-Table Performance [MUST]
The hand evaluator MUST achieve at least 200 million evaluations per second on a single core using precomputed lookup tables.

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-001
**Verified by:** Benchmark harness evaluates random 7-card hands; throughput >= 200M evals/sec on reference hardware (modern x86-64 CPU).

### HEV-003: Suit Isomorphism Mapping [MUST]
The hand evaluator MUST provide a function to canonicalize board cards by suit permutation, mapping any flop to its canonical representative among the 1,755 distinct flop classes.

**Source:** docs/02_solver_engine.md, docs/08_aggregated_reports.md
**Depends on:** —
**Verified by:** All 22,100 possible flops map to exactly 1,755 canonical representatives; mapping is deterministic and consistent.

### HEV-004: Equity Calculation [MUST]
The system MUST calculate all-in equity between two ranges on any board (flop, turn, or river) by enumerating or sampling remaining cards.

**Source:** docs/02_solver_engine.md, docs/10_range_builder.md
**Depends on:** HEV-001
**Verified by:** Equity calculations match reference values within 0.1% for known test scenarios.

### HEV-005: Hand Strength Categorization [MUST]
The evaluator MUST classify evaluated hands into standard categories: high card, pair, two pair, three of a kind, straight, flush, full house, four of a kind, straight flush.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HEV-001
**Verified by:** Category labels are correct for all 7,462 hand classes.

### HEV-006: Draw Detection [MUST]
The evaluator MUST detect common draw types on flop and turn boards: flush draws (4-to-flush), open-ended straight draws (OESD), gutshot straight draws, and combo draws.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HEV-001
**Verified by:** Known draw hands are correctly identified on test boards.

### HEV-007: Blocker Analysis [MUST]
The evaluator MUST determine which hole cards block specific opponent holdings (e.g., holding A♠ blocks opponent's nut flush draw when board has two spades).

**Source:** docs/03_study_mode.md, docs/07_nodelocking.md
**Depends on:** HEV-001
**Verified by:** Blocker computation correctly reduces opponent combo counts for known scenarios.

### HEV-008: Batch Evaluation [SHOULD]
The evaluator SHOULD support batch evaluation of multiple hands against a board in a single call to maximize throughput through memory locality and SIMD opportunities.

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-001
**Verified by:** Batch mode achieves >= 1.5x throughput compared to sequential single-hand evaluation.

### HEV-009: Hand Ranking Comparison [MUST]
The evaluator MUST provide a comparison function that determines the winner between two evaluated hands, supporting outcomes: player 1 wins, player 2 wins, or tie (split pot).

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-001
**Verified by:** All pairwise comparisons are transitive and consistent; tie detection works for identical hand strengths with different suits.

### HEV-010: Evaluation Caching [MAY]
The evaluator MAY cache evaluation results for recently queried board+hand combinations to avoid redundant lookups during tree traversal.

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-001
**Verified by:** Cache hit rate > 50% during a typical solve; no correctness regression.

---

## 3. Solution Storage (STO)

### STO-001: Binary Solution Format [MUST]
Solutions MUST be stored in a compact binary format. The format MUST include: game tree structure, strategy frequencies for each information set, and metadata (bet sizes, stack depth, positions, board).

**Source:** docs/02_solver_engine.md, docs/14_architecture_blueprint.md
**Depends on:** SOL-001
**Verified by:** A solved game tree can be serialized to binary and deserialized to produce identical strategy output.

### STO-002: Compression [MUST]
Solution files MUST be compressed using zstd or equivalent algorithm, achieving at least 3:1 compression ratio on typical solution data.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** STO-001
**Verified by:** Compressed file size is <= 33% of uncompressed size; decompression produces bit-identical data.

### STO-003: Memory-Mapped I/O [MUST]
The solution reader MUST support memory-mapped file access for large solution trees, enabling partial loading without reading the entire file into RAM.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** STO-001
**Verified by:** Loading a 2GB solution file uses < 500MB resident memory; random node access latency is < 1ms.

### STO-004: Strategy Quantization [MUST]
Strategy frequencies MUST be stored using quantized representation (e.g., 16-bit or 8-bit fixed-point) to reduce storage size, with negligible impact on strategic accuracy.

**Source:** docs/02_solver_engine.md
**Depends on:** STO-001
**Verified by:** Quantization error < 0.1% per action frequency; overall Nash Distance increase from quantization < 0.05% pot.

### STO-005: Solution Metadata [MUST]
Each solution file MUST contain embedded metadata: solver version, game configuration (positions, stack depth, pot size, bet sizes, board cards), solve date, and Nash Distance achieved.

**Source:** docs/03_study_mode.md
**Depends on:** STO-001
**Verified by:** Metadata is readable without decompressing the full solution; all required fields are present.

### STO-006: Solution Library Index [MUST]
The application MUST maintain an index of all stored solutions, searchable by game type, positions, stack depth, and board texture. The index MUST update when solutions are added or removed.

**Source:** docs/03_study_mode.md, docs/08_aggregated_reports.md
**Depends on:** STO-001, STO-005
**Verified by:** Index query returns matching solutions within 100ms; index is consistent with on-disk solutions.

### STO-007: Incremental Save [SHOULD]
The solver SHOULD support saving partial solutions (intermediate checkpoints) during long-running solves, enabling resume-from-checkpoint.

**Source:** docs/02_solver_engine.md
**Depends on:** STO-001, SOL-001
**Verified by:** A solve interrupted at iteration N can resume from checkpoint and reach the same final Nash Distance as an uninterrupted solve.

### STO-008: Solution Import/Export [SHOULD]
The application SHOULD support exporting solutions to a portable format and importing solutions from the same format, enabling sharing between users.

**Source:** docs/01_product_overview.md
**Depends on:** STO-001, STO-002
**Verified by:** Exported solution re-imported on a different machine produces identical strategy display.

### STO-009: Solution Versioning [SHOULD]
The storage format SHOULD include a version number, and the reader MUST reject or migrate solutions saved with incompatible format versions.

**Compatibility rules:**
- The solution file header includes a format version number (u16).
- The reader MUST reject files with a major version higher than supported, returning a clear error: "Solution format v{X} not supported; please update the application."
- Minor version increments are forward-compatible: the reader silently ignores unknown fields.
- No automatic migration: if a file is incompatible, the user re-solves the spot.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** STO-001
**Verified by:** Attempting to load a solution with an unknown version produces a clear error; forward-compatible reader handles older versions.

### STO-010: Storage Space Management [MUST]
The application MUST display total disk space used by stored solutions and allow the user to delete individual solutions or batch-delete by filter criteria.

**Source:** docs/01_product_overview.md
**Depends on:** STO-006
**Verified by:** Displayed space matches actual disk usage within 1%; deletion frees the reported space.

### STO-011: Sparse Strategy Encoding [MUST]
For nodes where most hands take a single action (e.g., 95% fold), the format MUST use sparse encoding to avoid storing near-zero frequencies for every hand combo.

**Source:** docs/02_solver_engine.md
**Depends on:** STO-001
**Verified by:** Sparse-encoded solutions are smaller than dense encoding; decoding produces identical strategies.

### STO-012: Solution Integrity Check [MAY]
Solution files MAY include a checksum (e.g., CRC32 or xxHash) to detect corruption.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** STO-001
**Verified by:** Corrupted file is detected on load; intact file passes the check.

---

## 4. Study Mode (STU)

### STU-001: Four-Tab Interface [MUST]
Study Mode MUST present a four-tab interface: Strategy, Ranges, Breakdown, and Reports. Each tab MUST display distinct analytical data for the selected game state.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** UIF-001, STO-001
**Verified by:** All four tabs render with appropriate content; tab switching is instant (< 100ms).

### STU-002: Spot Selector [MUST]
Study Mode MUST provide a spot selector allowing the user to configure: game type, positions (all 6-max pairs), stack depth, and preflop action sequence. The selected spot MUST load the corresponding solution.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STO-006
**Verified by:** Changing any spot selector parameter loads the matching solution within 500ms.

### STU-003: Board Selector [MUST]
Study Mode MUST allow the user to input specific board cards (flop, turn, river) using card selection controls. The display MUST update to show the strategy for the selected board.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-002
**Verified by:** Selecting specific board cards displays the correct strategy; all 52 cards are selectable with dealt cards excluded.

### STU-004: Strategy Tab — Action Frequencies [MUST]
The Strategy tab MUST display the GTO strategy as action frequencies for the active player at the current decision node. Frequencies MUST be shown in the 13x13 hand matrix and as an overall frequency bar.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-001, HMX-001
**Verified by:** Displayed frequencies sum to 100% for each hand; colors match the assigned action colors.

### STU-005: Ranges Tab — Range Display [MUST]
The Ranges tab MUST display the range of hands each player holds at the current node, with hands colored by strength category or action taken.

**Source:** docs/03_study_mode.md
**Depends on:** STU-001, HMX-001
**Verified by:** Range display shows only hands that reach the current node; hand counts match expected combo counts.

### STU-006: Breakdown Tab — Hand Categories [MUST]
The Breakdown tab MUST categorize the active player's range into hand strength groups (e.g., top pair, overpair, flush draw, gutshot) and display the strategy for each group.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-001, HEV-005, HEV-006
**Verified by:** All hands in the range are assigned to exactly one category; category strategies sum to overall strategy frequencies.

### STU-007: Reports Tab — Aggregated Metrics [MUST]
The Reports tab MUST display aggregated strategy metrics across board subsets, including overall action frequencies, EV, and equity for the current spot.

**Source:** docs/03_study_mode.md, docs/08_aggregated_reports.md
**Depends on:** STU-001, AGG-001
**Verified by:** Metrics match individual board calculations when aggregated manually.

### STU-008: Action Sequence Navigation [MUST]
Study Mode MUST allow the user to navigate the game tree by clicking actions in the action sequence display (the line of play). Clicking any previous action MUST navigate back to that node.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-001
**Verified by:** Clicking an action in the sequence navigates to the correct game tree node; back-navigation works from any depth.

### STU-009: Metric Overlay Selection [MUST]
Study Mode MUST support multiple metric overlays on the hand matrix: Strategy, EV, Equity, and EQR. The user MUST be able to switch between overlays.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-001, HMX-001
**Verified by:** Each overlay displays the correct data type; values update when the game state changes.

### STU-010: EV Display [MUST]
Study Mode MUST display the Expected Value (EV) of each action at the current node, and the overall EV of the active player's range. EV MUST be shown in chips/bb.

**Source:** docs/03_study_mode.md
**Depends on:** SOL-001
**Verified by:** EV values are consistent with solver output; sum of frequency-weighted action EVs equals overall EV.

### STU-011: Hand Detail View [SHOULD]
Study Mode SHOULD allow clicking on a specific hand in the matrix to display detailed information: action frequencies, EV of each action, equity vs. opponent range, and best/worst actions.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** STU-004, HMX-005
**Verified by:** Clicking a hand shows correct frequencies and EVs for that specific combo.

### STU-012: Keyboard Shortcuts [MUST]
Study Mode MUST support keyboard shortcuts: J (open spot selector), S (cycle grouping), P (clear filters), Q (toggle fullscreen), Spacebar (toggle hand detail), 1-4 (switch tabs).

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** UIF-002
**Verified by:** Each shortcut triggers the documented action; shortcuts do not conflict with text input fields.

### STU-013: Compare EV Overlay [SHOULD]
Study Mode SHOULD provide a "Compare EV" overlay showing the EV difference between each action and the highest-EV action for each hand, enabling quick identification of close decisions.

**Source:** docs/03_study_mode.md
**Depends on:** STU-009, STU-010
**Verified by:** Compare EV values equal max(EV) - EV(action) for each hand/action pair.

### STU-014: Range Weight Display [SHOULD]
Study Mode SHOULD display how frequently each hand combo reaches the current node (range weight), indicating which hands are present at full frequency vs. partially.

**Source:** docs/03_study_mode.md
**Depends on:** STU-005
**Verified by:** Range weights for hands that are always present show 100%; partially mixed hands show correct partial frequency.

### STU-015: Flop Subset Navigation [SHOULD]
Study Mode SHOULD provide a flop browser or category view allowing the user to quickly navigate between flop subsets by board texture characteristics.

**Source:** docs/03_study_mode.md, docs/08_aggregated_reports.md
**Depends on:** STU-003, HEV-003
**Verified by:** Flop browser categorizes all 1,755 distinct flops and allows selection by texture.

### STU-016: Solution Comparison [MAY]
Study Mode MAY allow comparing two solutions for the same spot side-by-side (e.g., comparing strategies at different stack depths or with different bet sizes).

**Source:** docs/03_study_mode.md
**Depends on:** STU-001
**Verified by:** Two solutions load simultaneously with visible differences highlighted.

---

## 5. Practice Mode (PRA)

### PRA-001: Three Game Modes [MUST]
Practice Mode MUST support three game modes: Full Hand (play from preflop through river), Street (play a single street), and Spot (play from a specific decision point).

**Source:** docs/04_practice_mode.md
**Depends on:** SOL-003, UIF-001
**Verified by:** Each mode initializes the practice session at the correct starting point; all three modes are selectable.

### PRA-002: GTO Opponent [MUST]
Practice Mode MUST provide a GTO-optimal opponent that plays according to the solver's strategy at each decision point, including mixed strategies executed with correct frequencies.

**Source:** docs/04_practice_mode.md
**Depends on:** STO-001, SOL-001
**Verified by:** Over 10,000 sampled decisions, opponent action frequencies match solver frequencies within 2% for each action.

### PRA-003: Three Difficulty Levels [MUST]
Practice Mode MUST support three difficulty levels that control bet size granularity: Simple (1-2 bet sizes per node), Grouped (2-3 sizes), and Standard (full solver sizes).

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-001
**Verified by:** Each difficulty level presents the correct number of available actions.

### PRA-004: Score Tracking [MUST]
Practice Mode MUST track the player's performance using a scoring system that measures EV loss relative to the GTO strategy. Scores MUST be displayed after each hand and as a session average.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-002, STU-010
**Verified by:** Score is calculated as the EV difference between the player's action and the GTO-optimal action; session average aggregates correctly.

### PRA-005: Decision Classification [MUST]
Each practice decision MUST be classified into one of five categories based on EV loss: Perfect (0 EV loss), Good (minimal EV loss), Inaccurate (small EV loss), Wrong (significant EV loss), Blunder (large EV loss).

**Source:** docs/04_practice_mode.md, docs/16_screenshot_analysis.md
**Depends on:** PRA-004
**Verified by:** Classifications match the defined EV-loss thresholds; boundary values are correctly categorized.

### PRA-006: Post-Decision Feedback [MUST]
After each decision in Practice Mode, the system MUST display the GTO strategy for that node, showing the correct action frequencies and the EV of each action, alongside the player's choice.

**Source:** docs/04_practice_mode.md, docs/11_ui_ux_screens.md
**Depends on:** PRA-002
**Verified by:** Feedback panel shows the complete strategy; player's chosen action is highlighted.

### PRA-007: RNG Mode [MUST]
Practice Mode MUST support three RNG modes: High (player consistently receives favorable cards), Low (player receives unfavorable cards), and Off (true random). RNG mode MUST be selectable before the session starts.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-001
**Verified by:** High mode produces above-average hand strength distribution over 1,000 hands; Low mode produces below-average; Off mode matches expected distribution.

### PRA-008: Spot Configuration [MUST]
Practice Mode MUST allow configuring: game type, positions, stack depth, and optional board cards for Street/Spot modes.

**Source:** docs/04_practice_mode.md
**Depends on:** STU-002
**Verified by:** All configuration parameters are applied to the practice session; changing configuration restarts the session.

### PRA-009: Session Statistics [MUST]
Practice Mode MUST display cumulative session statistics: number of hands played, decisions made, average score, and distribution of decision classifications (Perfect/Good/Inaccurate/Wrong/Blunder).

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-004, PRA-005
**Verified by:** Statistics update in real-time; totals are consistent (e.g., sum of classifications equals total decisions).

### PRA-010: Hand History Review [SHOULD]
Practice Mode SHOULD allow reviewing completed practice hands, showing each decision point with the player's choice vs. the GTO strategy.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-006
**Verified by:** Review displays all decision points from the completed hand with correct GTO data.

### PRA-011: Multitabling [SHOULD]
Practice Mode SHOULD support playing 1-4 tables simultaneously, with independent game states on each table.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-001, UIF-003
**Verified by:** 4 simultaneous tables operate independently; actions on one table do not affect others.

### PRA-012: Game Speed Control [SHOULD]
Practice Mode SHOULD support speed settings (Normal, Fast, Turbo) that control animation timing and automatic progression delays.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-001
**Verified by:** Each speed setting produces visibly different pacing; Turbo mode completes hands significantly faster than Normal.

### PRA-013: Timebank [MAY]
Practice Mode MAY enforce a decision timer (timebank) with configurable durations (e.g., 7, 15, or 25 seconds) that forces a default action when time expires.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-001
**Verified by:** Timer counts down visually; default action is applied on expiry.

### PRA-014: Performance Trends [MAY]
Practice Mode MAY display performance trends over time, showing improvement in accuracy across sessions.

**Source:** docs/04_practice_mode.md
**Depends on:** PRA-009
**Verified by:** Trend chart shows historical session scores; data persists between application restarts.

---

## 6. Analyze Mode (ANL)

### ANL-001: Hand History Import [MUST]
Analyze Mode MUST import hand histories from at least the following formats: PokerStars, GGPoker, Winamax, 888poker, PartyPoker, and iPoker networks. Additional site support (17+ total) is a stretch goal.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-001
**Verified by:** Sample hand histories from each supported site import successfully; hand data is correctly parsed.

### ANL-002: Batch Upload [MUST]
Analyze Mode MUST support batch import of multiple hand history files or directories. The import process MUST handle mixed file formats and report per-file import success/failure.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-001
**Verified by:** Importing a directory of 100+ mixed-format files completes with individual file status reports.

### ANL-003: Decision Classification [MUST]
Analyze Mode MUST classify each player decision into five categories based on EV loss against the GTO strategy: Perfect (0 EV loss), Good (< threshold 1), Inaccurate (< threshold 2), Wrong (< threshold 3), Blunder (>= threshold 3).

**Source:** docs/05_analyze_mode.md, docs/16_screenshot_analysis.md
**Depends on:** SOL-001, HEV-001
**Verified by:** Classifications match expected values for benchmark hands with known GTO solutions.

### ANL-004: EV Loss Calculation [MUST]
Analyze Mode MUST calculate per-decision EV loss as: EV(GTO optimal action) - EV(player's chosen action). The calculation MUST use the solver's strategy for the specific game state.

**Source:** docs/05_analyze_mode.md
**Depends on:** SOL-001
**Verified by:** EV loss is >= 0 for all decisions; EV loss is 0 when player's action matches the GTO pure strategy.

### ANL-005: Hand Replay [MUST]
Analyze Mode MUST provide a hand replay interface showing the imported hand street-by-street with player actions, pot size, and community cards. Navigation MUST support forward, backward, and jump-to-street.

**Source:** docs/05_analyze_mode.md, docs/11_ui_ux_screens.md
**Depends on:** ANL-001, UIF-001
**Verified by:** Replay accurately recreates the hand; all streets and actions are present; navigation works in both directions.

### ANL-006: GTO Comparison Overlay [MUST]
During hand replay, Analyze Mode MUST overlay the GTO strategy at each decision point, showing what the solver recommends alongside the player's actual action.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-005, SOL-001
**Verified by:** GTO overlay appears at decision nodes; strategy matches the solver's output for that game state.

### ANL-007: Dashboard View [MUST]
Analyze Mode MUST provide a dashboard displaying aggregate statistics across all imported hands: total hands, average EV loss, decision classification distribution, and worst errors.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-003, ANL-004
**Verified by:** Dashboard metrics are consistent with individual hand analyses; totals match hand-by-hand sums.

### ANL-008: Position Breakdown [MUST]
The dashboard MUST break down performance by position (UTG, MP, CO, BTN, SB, BB), showing per-position statistics including average EV loss and classification distribution.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Position breakdowns sum to overall totals; each hand is assigned to exactly one position.

### ANL-009: Street Breakdown [MUST]
The dashboard MUST break down performance by street (Preflop, Flop, Turn, River), showing per-street EV loss and classification distribution.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Street breakdowns sum to overall totals; multi-street hands contribute to each street separately.

### ANL-010: Action Type Breakdown [MUST]
The dashboard MUST break down performance by action type (Fold, Check, Call, Bet, Raise), showing per-action accuracy and EV loss metrics.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Action type breakdowns cover all player actions; no actions are uncategorized.

### ANL-011: Hand Filtering [SHOULD]
Analyze Mode SHOULD support filtering analyzed hands by: date range, position, stake level, hand result (won/lost), and decision classification.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Applying filters correctly reduces the displayed hand set; combining filters intersects correctly.

### ANL-012: Biggest Mistakes List [SHOULD]
Analyze Mode SHOULD display a ranked list of the player's largest EV-loss decisions, allowing quick navigation to the most impactful errors.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-004
**Verified by:** List is sorted by EV loss descending; clicking an entry navigates to the hand replay at the relevant decision.

### ANL-013: Session Summary [SHOULD]
Analyze Mode SHOULD provide per-session summaries grouping hands played in the same time window, with session-level aggregate statistics.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Hands are correctly grouped by session; session boundaries are detected from hand timestamps.

### ANL-014: Export Analysis Results [SHOULD]
Analyze Mode SHOULD support exporting analysis results (decision classifications, EV loss data) to CSV or JSON format for external analysis.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-007
**Verified by:** Exported file contains all analyzed hands with correct metrics; file is valid CSV/JSON.

### ANL-015: Comparative Analysis [MAY]
Analyze Mode MAY support comparing two time periods or session groups to show improvement trends.

**Source:** docs/05_analyze_mode.md
**Depends on:** ANL-013
**Verified by:** Comparison view shows delta metrics between two selected time periods.

---

## 7. Real-Time Solver (RTS)

### RTS-001: On-Demand Solving [MUST]
The real-time solver MUST accept an arbitrary postflop game state (board, positions, stack depth, pot, bet sizes) and solve it on demand without requiring a pre-solved solution in the library.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-003, SOL-005
**Verified by:** Any valid postflop configuration produces a solution; configurations not in the pre-solved library still produce results.

### RTS-002: Solve Time Target [MUST]
The real-time solver MUST complete a standard solve (heads-up, single street, 3-4 bet sizes) in under 10 seconds on consumer hardware (8-core CPU, 32GB RAM).

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** RTS-001, SOL-009
**Verified by:** Benchmark suite of 20 representative spots averages < 10 sec solve time.

### RTS-003: Configurable Sizing Modes [MUST]
The real-time solver MUST support at least two sizing modes: Fixed (user-specified bet sizes per street) and Automatic (solver selects from a reasonable default set).

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-005
**Verified by:** Fixed mode uses exact user-specified sizes; Automatic mode produces valid sizes without user input.

### RTS-004: Stack Depth Range [MUST]
The real-time solver MUST support stack depths from 1bb (short-stack push/fold) to at least 200bb.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-003
**Verified by:** Solver produces valid results at 1bb, 20bb, 100bb, and 200bb stack depths.

### RTS-005: Result Persistence [MUST]
Real-time solver results MUST be cached or stored so that re-querying the same game state returns results instantly (< 100ms) without re-solving.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** STO-001
**Verified by:** Second query for an identical game state returns in < 100ms.

### RTS-006: Depth-Limited Solving [SHOULD]
The real-time solver SHOULD implement depth-limited solving that estimates values at future street boundaries (e.g., using blueprint strategies or neural network value predictions) to reduce solve time.

**Source:** docs/02_solver_engine.md, docs/06_gto_wizard_ai.md
**Depends on:** SOL-001, SOL-012
**Verified by:** Depth-limited solve is at least 3x faster than full-depth solve; Nash Distance remains < 1% pot.

### RTS-007: Dynamic Sizing Mode [SHOULD]
The real-time solver SHOULD support a Dynamic sizing mode that uses an algorithm to discover optimal bet sizes from a continuous range.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** SOL-015
**Verified by:** Dynamic mode produces sizing recommendations that differ from the default fixed set.

### RTS-008: Background Solving [MAY]
The real-time solver MAY support queuing multiple solve requests and processing them in the background, notifying the user when each completes.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** RTS-001
**Verified by:** Multiple solve requests are queued; user is notified on completion; results are accessible.

---

## 8. Nodelocking (NLK)

### NLK-001: Strategy Locking [MUST]
The system MUST allow the user to lock (fix) the strategy at any decision node for either player. A locked node's strategy is not modified during re-solving.

**Source:** docs/07_nodelocking.md
**Depends on:** SOL-001, STU-001
**Verified by:** Locking a node and re-solving produces a different strategy for the opponent while the locked node's strategy remains unchanged.

### NLK-002: Strategy Editing [MUST]
The system MUST allow the user to manually edit action frequencies at any node before locking, setting custom probabilities for each action (e.g., 70% fold, 20% call, 10% raise).

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-001
**Verified by:** User-set frequencies are preserved after locking; frequencies sum to 100%.

### NLK-003: Exploitative Re-Solve [MUST]
After locking one or more nodes, the system MUST re-solve the remaining unlocked nodes to find the maximally exploitative counter-strategy against the locked strategy.

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-001, SOL-001
**Verified by:** Re-solved strategy achieves higher EV against the locked strategy than the original GTO strategy does.

### NLK-004: Lock/Unlock Controls [MUST]
The interface MUST clearly indicate which nodes are locked (visually distinct from unlocked nodes) and provide controls to lock, unlock, or reset individual nodes.

**Source:** docs/07_nodelocking.md, docs/11_ui_ux_screens.md
**Depends on:** NLK-001, UIF-001
**Verified by:** Locked nodes have a visible indicator; unlock removes the indicator and frees the node for re-solving.

### NLK-005: Cascade Lock Effects [MUST]
When a node is locked, the system MUST propagate the locked strategy's effects through all downstream nodes (cascade), ensuring the game tree remains consistent.

**Cascade semantics:** When a node's strategy is locked, downstream nodes must be re-solved. The cascade procedure is:
1. Lock the target node's strategy (action probabilities become fixed).
2. Recompute the reaching probabilities for all downstream nodes using the locked strategy probabilities.
3. Re-solve only the downstream sub-tree with updated reaching probabilities.
4. Upstream nodes remain unchanged (the lock is a one-way constraint).

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-001
**Verified by:** Range reaching downstream nodes correctly reflects the locked strategy's action frequencies.

### NLK-006: Compare Nodes [MUST]
The system MUST provide a comparison view showing the difference between the GTO solution and the nodelock-adjusted solution side by side, highlighting which hands change strategy.

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-003
**Verified by:** Comparison shows both strategies with differences highlighted; unchanged hands are shown as neutral.

### NLK-007: Frequency Editing Interface [SHOULD]
The nodelocking interface SHOULD provide a visual frequency editor (e.g., sliders or direct numeric input) for adjusting action probabilities at each node.

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-002
**Verified by:** Sliders and numeric inputs are synchronized; adjusting one action's frequency automatically adjusts others to maintain 100% total.

### NLK-008: Batch Nodelocking [SHOULD]
The system SHOULD support locking multiple nodes simultaneously (e.g., lock all of an opponent's flop c-bet nodes to a specific frequency).

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-001
**Verified by:** Batch lock applies the same strategy modification to all selected nodes in one operation.

### NLK-009: Overwrite Mode Selection [SHOULD]
The system SHOULD offer two re-solve modes: Overwrite Unlocked (only update unlocked nodes) and Overwrite All (re-solve everything including currently locked opponent nodes).

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-003
**Verified by:** Each mode produces the documented behavior; mode is selectable before re-solving.

### NLK-010: Reset to GTO [MAY]
The system MAY provide a one-click reset that unlocks all nodes and restores the original GTO solution.

**Source:** docs/07_nodelocking.md
**Depends on:** NLK-001
**Verified by:** Reset restores exact GTO strategy at all nodes; all lock indicators are removed.

---

## 9. Aggregated Reports (AGG)

### AGG-001: Flop Aggregate Analysis [MUST]
The system MUST generate aggregate strategy reports across all 1,755 strategically distinct flops for a given preflop spot, displaying overall action frequencies weighted by flop probability.

**Source:** docs/08_aggregated_reports.md
**Depends on:** SOL-011, STO-006
**Verified by:** Report covers all 1,755 flops; overall frequencies match hand-calculated weighted averages for a subset of 10 spot-checked flops.

### AGG-002: Four Metric Types [MUST]
Aggregated reports MUST display four metric types: Strategy (action frequencies), Expected Value (EV), Equity (EQ), and Equity Realization (EQR).

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-001
**Verified by:** All four metric types are selectable and display correct data.

### AGG-003: Board Texture Filtering [MUST]
Aggregated reports MUST support filtering by board texture: suit composition (monotone, two-tone, rainbow), pairing status, connectedness, and high card rank.

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-001
**Verified by:** Applying each filter reduces the flop set correctly; monotone filter selects only 3-suited flops.

### AGG-004: Chart View [MUST]
Aggregated reports MUST provide a chart (bar graph) view displaying metrics across flop categories.

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-001, UIF-001
**Verified by:** Chart renders with correct data; bars are labeled and colored by action.

### AGG-005: Table View [MUST]
Aggregated reports MUST provide a table view with numerical values, sortable by any column.

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-001, UIF-001
**Verified by:** Table displays numerical values matching chart data; sorting reorders rows correctly.

### AGG-006: Flop Grouping Options [SHOULD]
Reports SHOULD support grouping flops by: high card, suit composition, pairing status, and connectedness.

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-003
**Verified by:** Each grouping option produces the correct category breakdown; all 1,755 flops are assigned to exactly one group.

### AGG-007: Action Grouping [SHOULD]
Reports SHOULD support displaying actions at three granularity levels: All Sizes (each bet size separately), Grouped (small/medium/large), and Simplified (bet/check binary).

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-002
**Verified by:** Switching between action groupings correctly aggregates frequencies.

### AGG-008: Turn Aggregate Reports [SHOULD]
The system SHOULD generate aggregate reports for turn cards on a given flop, showing strategy across all possible turn runout cards.

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-001
**Verified by:** Turn report shows data for all valid turn cards (48 remaining cards after flop); data matches individual turn solutions.

### AGG-009: Filtered vs Overall Comparison [MAY]
Reports MAY display a comparison between filtered subset metrics and overall averages, with color coding (green = above average, red = below average).

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-003
**Verified by:** Delta values are correctly computed; color coding matches the direction of deviation.

### AGG-010: Report Export [MAY]
Aggregated reports MAY support export to image (PNG/SVG) or data format (CSV/JSON).

**Source:** docs/08_aggregated_reports.md
**Depends on:** AGG-004, AGG-005
**Verified by:** Exported image matches on-screen chart; exported data matches table values.

---

## 10. Range Builder (RNG)

### RNG-001: 13x13 Range Grid [MUST]
The Range Builder MUST display a 13x13 grid representing all 169 starting hand combinations (13 pocket pairs on diagonal, suited combos above, offsuit combos below).

**Source:** docs/10_range_builder.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001
**Verified by:** Grid displays all 169 cells with correct hand labels; pairs on diagonal, suited above, offsuit below.

### RNG-002: Paintbrush Selection [MUST]
The Range Builder MUST allow selecting/deselecting hands by clicking individual cells or click-dragging across multiple cells (paintbrush mode).

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001
**Verified by:** Clicking toggles individual cells; click-drag selects all cells traversed; visual feedback is immediate.

### RNG-003: Weight Assignment [MUST]
The Range Builder MUST support assigning weights (0-100%) to each hand or group of hands, representing the frequency at which that hand is included in the range.

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001
**Verified by:** Weights are visually displayed (e.g., opacity or numeric label); weights correctly affect equity calculations.

### RNG-004: Range Summary Statistics [MUST]
The Range Builder MUST display summary statistics: total combos selected, percentage of all hands, and range equity vs. a specified opponent range.

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001, HEV-004
**Verified by:** Combo count matches hand-counted selections; equity matches standalone equity calculator.

### RNG-005: Suit-Specific View [MUST]
The Range Builder MUST support a detailed view showing all 16 suit combinations for non-pair hands and all 6 combinations for pairs, allowing suit-specific weight adjustments.

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001
**Verified by:** Expanding any cell shows the correct number of suit combos; suit-level weights are independent.

### RNG-006: Range Presets [MUST]
The Range Builder MUST provide preset ranges (e.g., UTG open, BTN open, BB defend) that can be loaded and modified.

**Source:** docs/10_range_builder.md, docs/03_study_mode.md
**Depends on:** RNG-001
**Verified by:** Presets load correct default ranges; presets are available for all standard 6-max opening and defending positions.

### RNG-007: Combo Locking [MUST]
The Range Builder MUST allow locking specific hand combos so they are not modified by bulk operations (e.g., clearing or randomizing the range).

**Source:** docs/10_range_builder.md
**Depends on:** RNG-002
**Verified by:** Locked combos remain unchanged after bulk clear/fill operations; lock indicator is visible.

### RNG-008: Grade vs GTO [SHOULD]
The Range Builder SHOULD provide a grading feature that scores a user-constructed range against the GTO-optimal range for the selected spot, producing a 0-100% similarity score.

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001, STO-001
**Verified by:** GTO range grades at 100%; completely opposite range grades near 0%; intermediate ranges produce proportional scores.

### RNG-009: Range Import/Export [SHOULD]
The Range Builder SHOULD support importing and exporting ranges in standard text formats (e.g., "AKs, AQs, KQs, JJ+") and as weight arrays.

**Source:** docs/10_range_builder.md
**Depends on:** RNG-001
**Verified by:** Exported range text re-imports to produce the identical grid state.

### RNG-010: Range Color Coding [MUST]
The Range Builder MUST color-code cells based on their selection/weight state: unselected (dark/empty), fully selected (full color), and partially weighted (proportional opacity or gradient).

**Source:** docs/10_range_builder.md, docs/11_ui_ux_screens.md
**Depends on:** RNG-001
**Verified by:** Visual distinction between 0%, 50%, and 100% weight is clear; colors match the configured action palette.

---

## 11. Tournament & ICM (ICM)

### ICM-001: ICM Equity Calculation [MUST]
The system MUST calculate ICM equity (chip-to-prize-pool conversion) using the Malmuth-Harville model or equivalent, for tournament structures with up to 9 players at the final table.

**Source:** docs/09_tournament_icm.md
**Depends on:** —
**Verified by:** ICM equity values match known reference calculations for standard payout structures.

### ICM-002: ICM-Adjusted Solving [MUST]
The solver MUST support ICM-adjusted solving where decision values use ICM equity (tournament dollars) instead of chip EV, producing risk-averse strategies near pay jumps.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001, SOL-001
**Verified by:** ICM solution is more conservative than chip-EV solution in bubble scenarios (measurably higher fold frequency with medium hands).

### ICM-003: Configurable Payout Structures [MUST]
The ICM module MUST accept configurable payout structures: number of paid positions, payout percentages, and starting chip stacks for each player.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001
**Verified by:** ICM calculations correctly use custom payout structures; changing payouts changes the ICM equity values.

### ICM-004: Bubble Factor Display [MUST]
The system MUST calculate and display bubble factor (risk premium) for each player in a tournament scenario, indicating how much more costly it is to lose chips vs. how valuable it is to gain them.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001
**Verified by:** Bubble factor > 1.0 for all non-chip-leader players near the bubble; values match reference calculations.

### ICM-005: FT/SNG Simulator [SHOULD]
The system SHOULD simulate final table or SNG outcomes (ICM equity distribution) for given stack distributions using Monte Carlo simulation with at least 10,000 iterations.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001
**Verified by:** Simulation results converge within 1% of analytical ICM values for standard scenarios.

### ICM-006: Progressive Knockout (PKO) [SHOULD]
The system SHOULD support Progressive Knockout (PKO) bounty calculations, where eliminating a player awards a portion of their accumulated bounty.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001
**Verified by:** PKO bounty values are correctly tracked; eliminator receives the correct bounty fraction.

### ICM-007: Satellite ICM Dynamics [SHOULD]
The system SHOULD support satellite tournament ICM where all finishing positions above the cutoff receive equal prizes (e.g., tournament tickets), producing extreme risk aversion near the qualifying threshold.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-001
**Verified by:** Satellite ICM produces more extreme fold frequencies than standard ICM near the qualifying bubble.

### ICM-008: Mystery Bounty Support [MAY]
The system MAY support Mystery Bounty tournaments where bounty values are drawn from a distribution rather than fixed.

**Source:** docs/09_tournament_icm.md
**Depends on:** ICM-006
**Verified by:** Bounty EV calculation uses the expected value of the bounty distribution.

---

## 12. Hand History Parser (HHP)

### HHP-001: Multi-Site Support [MUST]
The hand history parser MUST support parsing hand histories from at least 6 major poker sites: PokerStars, GGPoker, Winamax, 888poker, PartyPoker, and iPoker network.

**Source:** docs/05_analyze_mode.md
**Depends on:** —
**Verified by:** Sample hand histories from each supported site parse without errors; all fields are extracted.

### HHP-002: Hand Data Extraction [MUST]
The parser MUST extract from each hand: game type, stakes/blinds, table size, positions, player stack sizes, hole cards (when shown), community cards, and all actions (fold, check, call, bet, raise amounts) with timestamps.

**Source:** docs/05_analyze_mode.md
**Depends on:** —
**Verified by:** Every extracted field matches the source hand history data; no fields are missing or incorrect.

### HHP-003: Cash Game Support [MUST]
The parser MUST correctly handle cash game hand histories including: blinds, antes, straddles, multi-way pots, side pots, and rake.

**Source:** docs/05_analyze_mode.md, docs/12_game_formats_and_pricing.md
**Depends on:** HHP-001
**Verified by:** Cash game hands parse correctly including side pots and straddle scenarios.

### HHP-004: Tournament Support [MUST]
The parser MUST correctly handle tournament hand histories including: blind levels, antes, bounties, and table changes.

**Source:** docs/05_analyze_mode.md, docs/09_tournament_icm.md
**Depends on:** HHP-001
**Verified by:** Tournament hands parse with correct blind levels and bounty information.

### HHP-005: Error Handling [MUST]
The parser MUST handle malformed or incomplete hand histories gracefully: logging the error, skipping the problematic hand, and continuing to process remaining hands without crashing.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-001
**Verified by:** Parser processes a file containing 10 valid and 2 malformed hands, extracting all 10 valid hands with error reports for the 2 failures.

### HHP-006: Character Encoding [MUST]
The parser MUST handle UTF-8 and common alternative encodings (Latin-1, Windows-1252) for player names and file content.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-001
**Verified by:** Hand histories with non-ASCII player names (accented characters, CJK characters) parse without encoding errors.

### HHP-007: Pot Size Reconstruction [MUST]
The parser MUST reconstruct accurate pot sizes at each decision point from the sequence of actions, correctly handling blinds, antes, bets, calls, and raises.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-002
**Verified by:** Reconstructed pot size at showdown matches the hand history's total pot; intermediate pot sizes are verified for 10 test hands.

### HHP-008: Position Assignment [MUST]
The parser MUST correctly assign standard position labels (UTG, UTG+1, MP, CO, BTN, SB, BB) based on seat positions, button location, and table size.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-002
**Verified by:** Position labels are correct for 6-max, 9-max, and heads-up formats.

### HHP-009: Currency/Chip Normalization [SHOULD]
The parser SHOULD normalize monetary values across different currencies and tournament chip denominations to a common unit (e.g., big blinds) for consistent analysis.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-002
**Verified by:** Hands from USD, EUR, and GBP sites are all expressed in BB; tournament chips are normalized to BB.

### HHP-010: File Format Auto-Detection [SHOULD]
The parser SHOULD automatically detect the poker site format from file content without requiring user specification.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-001
**Verified by:** Parser correctly identifies the source site for sample files from each supported site without user input.

### HHP-011: Incremental Import [SHOULD]
The parser SHOULD support incremental import, detecting and importing only new hands from a directory that has been previously imported.

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-001
**Verified by:** Re-importing a directory with 5 new files out of 100 only processes the 5 new files.

### HHP-012: Hand History Validation [MAY]
The parser MAY validate parsed hands for internal consistency (e.g., action amounts don't exceed stacks, community cards don't duplicate hole cards).

**Source:** docs/05_analyze_mode.md
**Depends on:** HHP-002
**Verified by:** Validation catches intentionally invalid test hands (overbet beyond stack, duplicate cards).

---

## 13. UI Framework (UIF)

### UIF-001: Dark Theme [MUST]
The application MUST use a dark color theme as the default and primary interface style, consistent with poker software conventions.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** DSK-001
**Verified by:** Background colors are dark (< 50 luminance); text is light; contrast ratios meet WCAG AA.

### UIF-002: Keyboard Shortcuts [MUST]
The application MUST support keyboard shortcuts for common actions. At minimum: J (spot selector), S (cycle grouping), P (clear filters), Q (fullscreen), Spacebar (toggle detail), 1-4 (tab switching).

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Each shortcut triggers the documented action; shortcuts work from any view where they are applicable.

### UIF-003: Responsive Layout [MUST]
The application MUST support window sizes from 1280x720 to 4K (3840x2160) with appropriate layout adjustments. Minimum supported width is 1280px.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** DSK-001
**Verified by:** Application renders correctly at 1280x720 and 3840x2160; no content overflow or truncation at minimum size.

### UIF-004: Action Color System [MUST]
The application MUST use a consistent color system for poker actions: red spectrum for bet/raise, green spectrum for check/call, blue spectrum for fold. Colors MUST be visually distinct and consistent across all views.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Action colors are consistent across Study, Practice, and Analyze modes; colors are distinguishable for common color vision deficiencies.

### UIF-005: Navigation Sidebar [MUST]
The application MUST provide a sidebar or top-level navigation allowing switching between major modes: Study, Practice, Analyze, Solve, Range Builder, and Settings.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** All modes are accessible from navigation; active mode is visually highlighted.

### UIF-006: Loading States [MUST]
The application MUST display loading indicators during asynchronous operations (solving, file import, solution loading). The UI MUST remain responsive during loading.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Loading spinner/progress bar appears during operations > 500ms; UI does not freeze during loading.

### UIF-007: Error Display [MUST]
The application MUST display user-facing error messages for recoverable errors (invalid input, file not found, solve failure) with actionable guidance. Errors MUST NOT crash the application.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Known error conditions produce user-friendly messages; application continues to function after dismissing the error.

### UIF-008: Tooltip System [MUST]
The application MUST provide tooltips on hover for icons, abbreviated labels, and data values that need additional context.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Hovering over icons/labels shows descriptive tooltips within 500ms; tooltips are accurate.

### UIF-009: Card Rendering [MUST]
The application MUST render playing cards with standard rank and suit symbols, using conventional suit colors (red for hearts/diamonds, black for spades/clubs — or the four-color deck variant).

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** All 52 cards render correctly; suits are visually distinguishable.

### UIF-010: WCAG AA Contrast [SHOULD]
Text and interactive elements SHOULD meet WCAG AA contrast ratios (4.5:1 for normal text, 3:1 for large text) against their backgrounds.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** UIF-001
**Verified by:** Automated contrast checker verifies AA compliance for primary text and background combinations.

### UIF-011: Four-Color Deck Option [SHOULD]
The application SHOULD offer a four-color deck option (spades=black, hearts=red, diamonds=blue, clubs=green) for improved suit readability.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** UIF-009
**Verified by:** Four-color mode renders each suit in a distinct color; setting persists across sessions.

### UIF-012: Animation System [SHOULD]
The application SHOULD use subtle animations for state transitions (tab switching, node navigation, card dealing) to provide visual continuity.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Animations complete in < 300ms; animations can be disabled in settings.

### UIF-013: Context Menus [MAY]
The application MAY provide right-click context menus for common actions on hands, nodes, and ranges.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Right-click produces a menu with relevant actions; menu items function correctly.

### UIF-014: Zoom and Pan [MAY]
Game tree visualizations MAY support zoom and pan interactions for navigating large trees.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Mouse wheel zooms; click-drag pans; zoom range is bounded to prevent unusable scales.

### UIF-015: Theme Customization [SHOULD]
The application SHOULD allow users to customize key UI colors (action colors, background shade) and save preferences.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** UIF-001
**Verified by:** Changed colors persist after restart; custom colors apply across all modes.

---

## 14. Hand Matrix (HMX)

### HMX-001: 13x13 Grid Display [MUST]
The hand matrix MUST display a 13x13 grid representing all 169 canonical Hold'em starting hand groups: 13 pocket pairs (diagonal), 78 suited combos (upper triangle), and 78 offsuit combos (lower triangle).

**Source:** docs/03_study_mode.md, docs/10_range_builder.md, docs/11_ui_ux_screens.md
**Depends on:** —
**Verified by:** Grid has exactly 169 cells; labels match standard notation (AA, AKs, AKo, etc.); diagonal, upper, and lower triangles are correct.

### HMX-002: Mixed-Strategy Coloring [MUST]
Each cell MUST display mixed-strategy information using proportional color fills (pie-chart or stacked-bar style), where each action's color segment is proportional to its frequency.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001, UIF-004
**Verified by:** A cell with 50% bet / 50% check shows equal-area red and green segments; pure-strategy cells show a single color.

### HMX-003: Cell Hover Detail [MUST]
Hovering over a matrix cell MUST display a tooltip or detail panel showing: hand name, action frequencies (%), EV of each action, and equity.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001, UIF-008
**Verified by:** Hover shows correct data for the hovered hand; data matches the Strategy and EV tabs.

### HMX-004: Action Filtering [MUST]
The user MUST be able to filter the matrix by action (e.g., show only hands that bet, or only hands that check), highlighting or isolating the matching hands.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001
**Verified by:** Filtering by "bet" highlights only cells with bet frequency > 0; filter is reversible.

### HMX-005: Cell Click Interaction [MUST]
Clicking a matrix cell MUST select that hand and update associated displays (detail panel, equity graph, EV breakdown) for the selected hand.

**Source:** docs/03_study_mode.md
**Depends on:** HMX-001
**Verified by:** Click selection updates all linked displays; clicking a different cell changes the selection.

### HMX-006: Metric Overlay Support [MUST]
The matrix MUST support displaying different metric overlays: Strategy (action colors), EV (heat map), Equity (heat map), and EQR (heat map). Only one overlay is active at a time.

**Source:** docs/03_study_mode.md
**Depends on:** HMX-001
**Verified by:** Each overlay shows the correct metric; switching overlays changes the visual representation.

### HMX-007: Combo Count Display [MUST]
The matrix MUST display combo counts: the number of specific card combinations each cell represents, accounting for removed (board or blocked) cards.

**Source:** docs/03_study_mode.md, docs/10_range_builder.md
**Depends on:** HMX-001
**Verified by:** Pairs show 6 combos (max), suited show 4, offsuit show 12; counts reduce correctly when board cards are dealt.

### HMX-008: Hand Category Highlighting [MUST]
The matrix MUST support highlighting hands by category (e.g., all pairs, all suited connectors, all broadways) for quick visual identification.

**Source:** docs/03_study_mode.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001
**Verified by:** Category filter highlights the correct set of hands; categories are mutually exclusive and exhaustive.

### HMX-009: Frequency Threshold Filter [MUST]
The matrix MUST allow filtering by frequency threshold (e.g., show hands where bet frequency > 50%), isolating hands that predominantly take a specific action.

**Source:** docs/03_study_mode.md
**Depends on:** HMX-001
**Verified by:** Threshold filter correctly includes/excludes hands based on the set percentage; slider or numeric input controls the threshold.

### HMX-010: Suit Expansion [SHOULD]
The matrix SHOULD support expanding a cell to show all suit combinations (4 for suited, 6 for pairs, 12 for offsuit) with individual strategy data for each combo.

**Source:** docs/10_range_builder.md, docs/11_ui_ux_screens.md
**Depends on:** HMX-001
**Verified by:** Expanded view shows the correct number of suit combos with individual data.

### HMX-011: Matrix Sizing [SHOULD]
The matrix SHOULD support at least two size options (compact and expanded) to balance information density with readability.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** HMX-001
**Verified by:** Both sizes render correctly; text is readable in compact mode; expanded mode shows more detail.

### HMX-012: Range Notation Label [MAY]
The matrix MAY display a text summary of the currently displayed range in standard notation (e.g., "AA-TT, AKs-ATs, KQs, AKo").

**Source:** docs/10_range_builder.md
**Depends on:** HMX-001
**Verified by:** Notation accurately describes the hands shown in the matrix; standard shorthand conventions are used.

---

## 15. Desktop App — Tauri (DSK)

### DSK-001: Tauri v2 Shell [MUST]
The application MUST be built using Tauri v2 with a Rust backend and a webview-based frontend. The Rust backend handles computation (solver, evaluator, parser); the frontend handles UI rendering.

**Source:** docs/14_architecture_blueprint.md (adapted for local-first architecture)
**Depends on:** —
**Verified by:** Application builds and launches as a native desktop executable using Tauri v2; Rust backend and webview frontend communicate via Tauri's IPC.

### DSK-002: Local-First Data [MUST]
All user data (solutions, hand histories, preferences, analysis results) MUST be stored locally on the user's machine. The application MUST NOT require a network connection for core functionality.

**Source:** docs/01_product_overview.md (adapted for local-first)
**Depends on:** DSK-001
**Verified by:** Application launches and operates fully with network disabled; no external API calls for core features.

### DSK-003: IPC Bridge [MUST]
The Tauri IPC bridge MUST expose backend functions (solve, evaluate, parse, load solution) to the frontend as typed async commands with structured request/response payloads.

**Source:** docs/14_architecture_blueprint.md (adapted)
**Depends on:** DSK-001
**Verified by:** Frontend can invoke all backend functions via IPC; responses contain correctly typed data.

### DSK-004: Application Settings [MUST]
The application MUST persist user settings (preferences, theme, default configurations) locally in a configuration file that survives application updates.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** DSK-001
**Verified by:** Changed settings persist after application restart; settings file location follows OS conventions.

### DSK-005: File Dialogs [MUST]
The application MUST use native file dialogs for opening hand history files, solution files, and selecting export locations.

**Source:** docs/05_analyze_mode.md
**Depends on:** DSK-001
**Verified by:** File dialog opens with OS-native appearance; selected files are accessible to the application.

### DSK-006: Window Management [MUST]
The application MUST support standard window operations: minimize, maximize, restore, resize, and close. Window position and size SHOULD be persisted between sessions.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** DSK-001
**Verified by:** All window operations work correctly; window state persists across restarts.

### DSK-007: Windows Primary Platform [SHOULD]
The application SHOULD prioritize Windows (10+) as the primary platform, with macOS and Linux as secondary targets.

**Source:** docs/01_product_overview.md
**Depends on:** DSK-001
**Verified by:** Application runs on Windows 10+ without issues; CI pipeline includes Windows build and test.

### DSK-008: Auto-Update [SHOULD]
The application SHOULD support checking for and applying updates through Tauri's built-in updater mechanism.

**Source:** docs/14_architecture_blueprint.md (adapted)
**Depends on:** DSK-001
**Verified by:** Update check detects available updates; update applies and application restarts with new version.

### DSK-009: Performance Monitoring [SHOULD]
The application SHOULD log performance metrics (memory usage, CPU usage, solve times) for diagnostic purposes, accessible from a developer/debug menu.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** DSK-001
**Verified by:** Performance log file is written with timestamp, metric name, and value; memory usage matches OS-reported values within 10%.

### DSK-010: Crash Recovery [MAY]
The application MAY implement crash recovery that saves application state periodically, enabling restore-to-last-state after an unexpected shutdown.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** DSK-004
**Verified by:** After simulated crash, application offers to restore previous state; restored state matches the last checkpoint.

---

## 16. Performance (PRF)

### PRF-001: Hand Evaluation Throughput [MUST]
The hand evaluator MUST achieve at least 200 million 7-card evaluations per second on a single core of a modern x86-64 CPU (e.g., Intel i7/AMD Ryzen 7, 2020+).

**Source:** docs/02_solver_engine.md
**Depends on:** HEV-002
**Verified by:** Benchmark harness measures throughput >= 200M evals/sec; variance < 5% across runs.

### PRF-002: Real-Time Solve Speed [MUST]
A standard postflop spot (heads-up, single street, 3 bet sizes per decision) MUST solve to < 0.5% Nash Distance in under 10 seconds on reference hardware.

**Source:** docs/06_gto_wizard_ai.md
**Depends on:** RTS-002
**Verified by:** Benchmark suite average < 10 sec; 95th percentile < 15 sec.

### PRF-003: UI Render Performance [MUST]
UI interactions (tab switching, matrix updates, node navigation) MUST render at 60fps without visible jank or dropped frames.

**Source:** docs/11_ui_ux_screens.md
**Depends on:** UIF-001
**Verified by:** Frame timing measurements show < 16.7ms per frame during interactions; no frames exceed 33ms.

### PRF-004: Solution Loading Speed [MUST]
Loading a pre-solved solution from disk and displaying it in Study Mode MUST complete in under 500ms for solutions up to 500MB compressed.

**Source:** docs/03_study_mode.md
**Depends on:** STO-003
**Verified by:** Benchmark: 500MB compressed solution loads and renders in < 500ms.

### PRF-005: Memory Budget [MUST]
Standard usage (browsing solutions, practice mode, analysis): < 4 GB resident memory. During active solve: < 4 GB for standard spots (HU, single street, ≤3 bet sizes); < 16 GB for complex multi-street solves.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-007
**Verified by:** Memory profiling shows < 4 GB resident memory during standard operations; solver stays under 4 GB for standard spots and under 16 GB for complex multi-street benchmark solves.

### PRF-006: Startup Time [SHOULD]
The application SHOULD start and be ready for user interaction within 3 seconds from launch on reference hardware (SSD storage).

**Source:** docs/11_ui_ux_screens.md
**Depends on:** DSK-001
**Verified by:** Measured time from process start to interactive UI < 3 sec.

### PRF-007: Multi-Core Scaling [SHOULD]
Solver performance SHOULD scale near-linearly with available CPU cores, achieving at least 6x speedup on 8 cores compared to single-core.

**Source:** docs/02_solver_engine.md
**Depends on:** SOL-009
**Verified by:** Benchmark: 8-core solve time is <= 1.5x of the theoretical 8x speedup (i.e., <= 1.33x single-core time / 8).

### PRF-008: Large Solution Support [MAY]
The application MAY support solution trees exceeding 4GB by using memory-mapped I/O and lazy loading, without requiring the full tree to fit in RAM.

**Source:** docs/02_solver_engine.md
**Depends on:** STO-003
**Verified by:** An 8GB solution file loads and displays correctly with < 2GB resident memory.

---

## 17. Data & Formats (DAT)

### DAT-001: Game Type Definitions [MUST]
The system MUST support the following game types: No-Limit Hold'em cash games, No-Limit Hold'em tournaments (MTT), Sit & Go (including Spin & Go / hyper-turbo format), and Heads-Up.

**Source:** docs/12_game_formats_and_pricing.md
**Depends on:** —
**Verified by:** Each game type can be selected in spot configuration; solver and parser handle each type correctly.

### DAT-002: Position Definitions [MUST]
The system MUST support standard positional labels: UTG, UTG+1 (HJ), MP (LJ), CO, BTN, SB, BB for 6-max; extended positions for 9-max; and heads-up (BTN/SB, BB).

**Source:** docs/03_study_mode.md, docs/15_glossary.md
**Depends on:** —
**Verified by:** All position labels are recognized; 6-max uses 6 positions; 9-max uses 9; HU uses 2.

### DAT-003: Poker Hand Notation [MUST]
The system MUST use standard poker hand notation: ranks A, K, Q, J, T, 9-2; suits s (spade), h (heart), d (diamond), c (club); "s" suffix for suited, "o" suffix for offsuit (e.g., AKs, AKo, TT).

**Source:** docs/15_glossary.md
**Depends on:** —
**Verified by:** All notation is correctly parsed and displayed throughout the application.

### DAT-004: Bet Size Notation [MUST]
The system MUST support bet sizes expressed as: percentage of pot (e.g., 33%, 75%, 150%), absolute chip amount, and "all-in." Sizes MUST be displayed consistently across all views.

**Source:** docs/06_gto_wizard_ai.md, docs/15_glossary.md
**Depends on:** —
**Verified by:** Bet sizes render correctly in all formats; conversion between formats is accurate.

### DAT-005: Glossary Compliance [SHOULD]
All user-facing terminology SHOULD be consistent with the definitions in the project glossary, including: EV, EQR, Nash Distance, exploitability, range advantage, nut advantage, polarized, merged, GTO.

**Source:** docs/15_glossary.md
**Depends on:** —
**Verified by:** Terminology audit finds no inconsistencies between UI labels and glossary definitions.

### DAT-006: Solution File Format Documentation [SHOULD]
The binary solution file format SHOULD be documented with a specification file, enabling third-party tools to read solution files.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** STO-001
**Verified by:** A third-party parser (written from the specification alone) can read and display solution data correctly.

### DAT-007: Configuration File Format [MAY]
Application configuration and presets MAY use a human-readable format (TOML or JSON) for user-editable settings.

**Source:** docs/14_architecture_blueprint.md
**Depends on:** DSK-004
**Verified by:** Configuration file opens in a text editor and is syntactically valid; manual edits are respected by the application.

---

## Requirement Summary

| # | Category | ID Prefix | Total | MUST | SHOULD | MAY |
|---|----------|-----------|-------|------|--------|-----|
| 1 | Solver Engine | SOL | 18 | 14 | 3 | 1 |
| 2 | Hand Evaluation | HEV | 10 | 8 | 1 | 1 |
| 3 | Solution Storage | STO | 12 | 8 | 3 | 1 |
| 4 | Study Mode | STU | 16 | 10 | 4 | 2 |
| 5 | Practice Mode | PRA | 14 | 9 | 3 | 2 |
| 6 | Analyze Mode | ANL | 15 | 10 | 4 | 1 |
| 7 | Real-Time Solver | RTS | 8 | 5 | 2 | 1 |
| 8 | Nodelocking | NLK | 10 | 6 | 3 | 1 |
| 9 | Aggregated Reports | AGG | 10 | 5 | 3 | 2 |
| 10 | Range Builder | RNG | 10 | 7 | 2 | 1 |
| 11 | Tournament & ICM | ICM | 8 | 4 | 3 | 1 |
| 12 | Hand History Parser | HHP | 12 | 8 | 3 | 1 |
| 13 | UI Framework | UIF | 15 | 8 | 5 | 2 |
| 14 | Hand Matrix | HMX | 12 | 9 | 2 | 1 |
| 15 | Desktop App (Tauri) | DSK | 10 | 6 | 3 | 1 |
| 16 | Performance | PRF | 8 | 5 | 2 | 1 |
| 17 | Data & Formats | DAT | 7 | 4 | 2 | 1 |
| | **TOTAL** | | **195** | **129** | **48** | **17** |

---

## Cross-Reference Index

| Source Document | Requirement IDs |
|----------------|----------------|
| `docs/01_product_overview.md` | STO-008, STO-010, DSK-002, DSK-007 |
| `docs/02_solver_engine.md` | SOL-001 through SOL-018, HEV-001 through HEV-010, STO-004, STO-007, STO-011, PRF-001, PRF-005, PRF-007, PRF-008 |
| `docs/03_study_mode.md` | STU-001 through STU-016, HMX-001 through HMX-012, UIF-002, DAT-002, PRF-004 |
| `docs/04_practice_mode.md` | PRA-001 through PRA-014 |
| `docs/05_analyze_mode.md` | ANL-001 through ANL-015, HHP-001 through HHP-012, DSK-005 |
| `docs/06_gto_wizard_ai.md` | SOL-003, SOL-005, SOL-015, SOL-016, SOL-017, RTS-001 through RTS-008, DAT-004, PRF-002 |
| `docs/07_nodelocking.md` | NLK-001 through NLK-010, HEV-007 |
| `docs/08_aggregated_reports.md` | AGG-001 through AGG-010, SOL-011, HEV-003, STU-007, STU-015 |
| `docs/09_tournament_icm.md` | ICM-001 through ICM-008, HHP-004 |
| `docs/10_range_builder.md` | RNG-001 through RNG-010, HMX-007, HMX-010, HMX-012 |
| `docs/11_ui_ux_screens.md` | UIF-001 through UIF-015, HMX-002, HMX-003, HMX-008, HMX-011, STU-012, PRA-006, DSK-006, PRF-003, PRF-006 |
| `docs/12_game_formats_and_pricing.md` | DAT-001, HHP-003 |
| `docs/13_competitive_landscape.md` | *(informs priority decisions; no direct requirements)* |
| `docs/14_architecture_blueprint.md` | STO-001, STO-002, STO-003, STO-009, STO-012, DSK-001, DSK-008, DSK-009, DSK-010, DAT-006, DAT-007 |
| `docs/15_glossary.md` | DAT-002, DAT-003, DAT-004, DAT-005 |
| `docs/16_screenshot_analysis.md` | PRA-005, ANL-003 |

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-19 | 1.0 | Initial requirements specification |

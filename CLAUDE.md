# GTO Poker Solver

Local-first desktop poker solver. Rust backend (CFR engine, hand evaluation, storage), TypeScript/React frontend, Tauri v2 shell. No cloud dependencies — all computation and data on the user's machine.

---

## Build Commands

### Rust
```bash
cargo build --workspace              # Debug build
cargo build --workspace --release    # Release build
cargo test --workspace               # Run all tests
cargo bench -p poker-eval            # Evaluator throughput benchmark
cargo bench -p poker-solver          # Solver speed benchmark
cargo bench -p poker-solution        # Solution loading benchmark
cargo clippy --workspace -- -D warnings  # Lint (zero warnings policy)
```

### Frontend
```bash
cd frontend
npm install                          # Install dependencies
npm run dev                          # Dev server
npm test                             # Run Vitest tests
npm run build                        # Production build
npx eslint src/                      # Lint
```

### Tauri
```bash
cargo tauri dev                      # Dev mode (Rust + frontend hot-reload)
cargo tauri build                    # Production build (installer)
```

---

## Architecture

### 8 Rust Crates (dependency order)

```
poker-core          Foundational types: Card, Hand, Board, Action, Position, GameType, BetSize
├── poker-eval      Hand evaluator (lookup tables), equity, draws, blockers, suit isomorphism
├── poker-icm       ICM equity, bubble factor, PKO, satellite
├── poker-hhparser  Hand history parser (6 sites), pot reconstruction, position assignment
poker-solver        CFR engine, game tree, MCCFR, parallel solving (rayon)
poker-solution      Binary format, zstd compression, mmap reader, SQLite index
poker-analyze       EV loss, decision classification, dashboard aggregation
app-tauri           Tauri binary: IPC commands, event streaming, app lifecycle
```

`poker-core` is the leaf (no deps). `app-tauri` depends on all library crates.

### 9 Frontend Modules

| Module | Path | Purpose |
|--------|------|---------|
| shared | `src/shared/` | HandMatrix (Canvas), CardRenderer, theme, navigation, shortcuts |
| ipc | `src/ipc/` | Typed Tauri command wrappers and event hooks |
| range-builder | `src/range-builder/` | Interactive range construction |
| solve | `src/solve/` | Custom solve config, progress, results |
| study | `src/study/` | 4-tab solution browsing (Strategy, Ranges, Breakdown, Reports) |
| practice | `src/practice/` | Play hands vs GTO, feedback, session stats |
| analyze | `src/analyze/` | HH import, dashboard, hand replay, GTO overlay |
| nodelock | `src/nodelock/` | Lock strategy, re-solve for exploits |
| reports | `src/reports/` | Aggregate reports across solved spots |

### IPC: Tauri Commands (not REST)

All commands async. All return `Result<T, AppError>`. Long ops use event streams for progress.

---

## Current Milestone

**Active:** Pre-implementation (build docs complete)
**Next:** M1 — Foundation & Hand Matrix

Milestones: M1 (Foundation) → M2 (Solver) → M3 (Study) → M4 (Practice) → M5 (Analysis) → M6 (Nodelock+Reports) → M7 (SHOULD) → M8 (MAY)

---

## Critical Rules

### Rust
- **No `unwrap()` in library crates.** Use `?` operator with typed errors. `expect()` only with justification comment.
- **No `unsafe` without `// SAFETY:` comment.**
- **No `println!()`.** Use `tracing::{info, warn, error, debug}`.
- **No blocking on Tokio runtime.** CPU work → `tokio::task::spawn_blocking()` → rayon.
- **No `HashMap` in solver hot path.** Use flat `Vec<T>` indexed by info set ID for O(1) access.
- **No `Box<Node>` for game tree.** Arena allocation: `Vec<GameNode>` with index references (eliminates pointer-chasing).
- **No individual `.clone()` on large structures.** Use `Arc`, `Cow`, or references.
- **No fine-grained IPC.** One Tauri command per user action; batch data in single payloads.
- **All IPC commands return `Result<T, AppError>`.** No raw strings or untyped errors.
- **Run `cargo test` before committing.**

### Frontend
- **No cross-module imports.** Feature modules import only from `shared/` and `ipc/`.
- **No external state library.** React Context (global) + useReducer (per-mode).
- **No `any` types.** Typed interfaces matching Rust serde structs.
- **No DOM-based hand matrix.** Canvas rendering for 60fps (PRF-003).
- **No `window.fetch()`.** Use `@tauri-apps/api` invoke via `ipc/` wrappers.
- **No polling for progress.** Use Tauri event listeners (push-based).

### Storage
- **No full-file reads for solutions.** Memory-mapped I/O only (PRF-004).
- **No uncompressed solution writes.** zstd compression always (STO-002).
- **SQLite WAL mode required.** Connection pooling for concurrent reads.

---

## Conventions

### Naming
- **Rust:** snake_case functions/variables, PascalCase types, kebab-case crate names
- **TypeScript:** PascalCase components, camelCase functions/variables, kebab-case file names
- **Commits:** imperative mood, concise ("Add CFR+ solver implementation")

### Visibility
- `pub` only for cross-crate API (types and functions other crates consume)
- `pub(crate)` for internal helpers
- Default private for everything else

### Cross-Crate Types
All cross-crate interfaces use `poker-core` types: `Card`, `Hand`, `Board`, `Action`, `Position`, `GameType`, `BetSize`.

---

## Performance Targets

| ID | Target | Active From |
|----|--------|-------------|
| PRF-001 | Hand evaluator >= 200M evals/sec (single core, release) | M1 |
| PRF-002 | Standard solve < 10 sec (HU, 1 street, 3 bet sizes) | M2 |
| PRF-003 | UI interactions at 60fps (< 16.7ms/frame) | M3 |
| PRF-004 | Solution load < 500ms for 500MB compressed | M2 |
| PRF-005 | Memory < 4GB standard use, < 16GB solving | M2 |
| PRF-006 | Startup < 3 seconds to interactive | M7 |
| PRF-007 | Solver >= 6x speedup on 8 cores | M7 |

---

## Data Directory

```
~/.poker-solver/
├── solutions/         Binary solution files (zstd compressed)
├── hands/             Imported hand history files
├── data.db            SQLite database (WAL mode)
├── config.toml        User configuration
└── eval_tables.bin    Hand evaluator lookup tables (~10MB)
```

---

## Reference Docs

All build specification documents:

| # | File | Description |
|---|------|-------------|
| 1 | `docs/build/requirements.md` | 195 requirements (129 MUST, 48 SHOULD, 17 MAY) |
| 2 | `docs/build/architecture.md` | Crate decomposition, frontend modules, IPC, storage |
| 3 | `docs/build/milestones.md` | 8 milestones (M1–M8), acceptance criteria |
| 4 | `docs/build/project-structure.md` | File layout by milestone, API boundaries |
| 5 | `docs/build/technical-constraints.md` | Platform limits, prohibited patterns, gotchas |
| 6 | `docs/build/test-strategy.md` | Test types, per-milestone test gates, benchmarks |
| 7 | `docs/build/agent-prompt.md` | Per-milestone implementation guides (M1–M6+) |

### Subsystem → Document Mapping

| Subsystem | Primary Doc | Supporting Docs |
|-----------|------------|-----------------|
| Hand evaluation | `requirements.md` (HEV-*) | `agent-prompt.md` §3, `test-strategy.md` §5 |
| Solver / CFR | `requirements.md` (SOL-*) | `architecture.md` §3, `agent-prompt.md` §4, `technical-constraints.md` §3.2 |
| Solution storage | `requirements.md` (STO-*) | `architecture.md` §4.4, `technical-constraints.md` §3.3 |
| Game tree | `architecture.md` §3 | `technical-constraints.md` §3.1, `project-structure.md` §3 |
| IPC layer | `architecture.md` §5 | `project-structure.md` §5, `technical-constraints.md` §4.4 |
| Frontend / UI | `requirements.md` (UI-*) | `project-structure.md` §6, `technical-constraints.md` §5 |
| Practice mode | `requirements.md` (PRA-*) | `agent-prompt.md` §6 |
| Analysis / HH | `requirements.md` (ANA-*) | `agent-prompt.md` §7, `architecture.md` §6 |
| Nodelock | `requirements.md` (NLK-*) | `agent-prompt.md` §8 |
| ICM | `requirements.md` (ICM-*) | `architecture.md` §7 |
| Testing | `test-strategy.md` | `agent-prompt.md` (per-milestone test notes) |
| Performance | `requirements.md` (PRF-*) | `test-strategy.md` §6, `technical-constraints.md` §3 |

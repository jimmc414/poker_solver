# Technical Constraints

## GTO Poker Solver — Local-First Desktop Application

**Document:** Build Document 5 of 8
**Version:** 1.0
**Technology Stack:** Rust backend, TypeScript/React frontend, Tauri v2 desktop shell
**Prerequisites:** [Requirements](requirements.md) (Doc 1), [Architecture](architecture.md) (Doc 2), [Milestones](milestones.md) (Doc 3), [Project Structure](project-structure.md) (Doc 4)

---

## 1. Scope & Purpose

### What This Document Defines

This document catalogs **platform limitations, prohibited patterns, and gotchas** that the implementation must respect. Violations of these constraints produce bugs, performance failures, or architectural drift that compound across milestones.

### Organization

- **Sections 2–4:** Constraints by category (platform, performance, prohibited patterns)
- **Section 5:** Gotchas and edge cases by subsystem
- **Section 6:** Per-milestone constraint activation index

---

## 2. Platform Constraints

### 2.1 Tauri v2 / WebView2 (DSK-001, DSK-007)

| Constraint | Detail | Impact |
|-----------|--------|--------|
| **WebView2 on Windows** | Tauri v2 uses the system WebView2 runtime. Windows 10 1803+ includes it; older builds may need the Evergreen installer. | M1: Installer must bundle or check for WebView2. |
| **No Node.js runtime** | The frontend runs in a webview, not Node.js. No `fs`, `path`, `child_process`, or any Node API. | All file access goes through Tauri IPC commands. |
| **Single webview** | Tauri v2 provides one webview per window (no iframes to sandboxed origins). | Multi-window features (M7: PRA-011 multitabling) must use Tauri's multi-window API or render within the single webview. |
| **CSP restrictions** | Tauri applies a Content Security Policy. No inline scripts, no `eval()`, no external CDN resources. | All assets must be bundled. No runtime code generation. |
| **IPC serialization** | All data crossing the IPC boundary is JSON-serialized via `serde`. No binary transfer, no shared memory. | Large payloads (e.g., full strategy arrays) must be chunked or referenced by ID. Strategy data transferred as quantized arrays. |
| **macOS notarization** | macOS builds require code signing and notarization for distribution. | CI pipeline must include Apple signing step for macOS builds. |
| **Linux WebKitGTK** | Linux uses WebKitGTK, which may lag behind WebView2/Safari in CSS/JS feature support. | Test CSS Grid, Canvas performance, and Web Workers on Linux WebKitGTK. |

### 2.2 File System (DSK-002)

| Constraint | Detail | Impact |
|-----------|--------|--------|
| **Data directory** | `~/.poker-solver/` on all platforms. On Windows, `~` resolves to `C:\Users\{user}\`. | Use Tauri's `app_data_dir()` API, not hardcoded paths. |
| **Path separators** | Windows uses `\`, Unix uses `/`. | Use `std::path::Path` and `PathBuf` everywhere. Never construct paths with string concatenation. |
| **File name length** | Windows: 255 chars. Some solution filenames may be long (board + positions + stack + bet sizes). | Truncate or hash long filenames. Solution files use a hash-based naming scheme. |
| **File locking** | Windows locks open files more aggressively than Unix. | Close mmap handles before attempting file deletion. Use `SolutionReader` RAII pattern. |
| **SQLite on Windows** | WAL mode works on Windows but requires delete journal fallback on network drives. | Enforce local storage only — `~/.poker-solver/` must be on a local filesystem. |

### 2.3 Memory Budget (PRF-005)

| Scenario | Budget | Enforcement |
|----------|--------|-------------|
| **Standard use** (Study, Practice, Analyze) | 4 GB resident | Memory-mapped solutions (no full loads); LRU cache bounded at 512 MB; React virtualization for long lists. |
| **Active solving** | 16 GB peak | MCCFR chance sampling bounds tree memory; configurable `memory_limit_gb` in `config.toml`; solver aborts if approaching limit. |
| **Hand evaluator tables** | ~10 MB | Loaded once at startup, never duplicated. |
| **Solution cache** | 512 MB (configurable) | LRU eviction; configurable via settings. |
| **Frontend heap** | < 500 MB | Canvas-based matrix rendering avoids DOM node explosion; virtualized lists for analyzed hands. |

### 2.4 Cross-Platform Differences

| Area | Windows | macOS | Linux |
|------|---------|-------|-------|
| WebView | WebView2 (Chromium-based) | WKWebView (Safari-based) | WebKitGTK |
| File paths | `C:\Users\{user}\.poker-solver\` | `/Users/{user}/.poker-solver/` | `/home/{user}/.poker-solver/` |
| Thread pool | `rayon` uses Windows thread pool | `rayon` uses pthreads | `rayon` uses pthreads |
| SIMD | x86-64 SSE4.2/AVX2 | x86-64 or Apple Silicon NEON | x86-64 SSE4.2/AVX2 |
| mmap | `CreateFileMapping` | `mmap(2)` | `mmap(2)` |
| SQLite | dll bundled | dylib bundled or system | shared library |

**Primary platform:** Windows 10+ (DSK-007). macOS and Linux are secondary targets — test on CI but prioritize Windows for edge cases.

---

## 3. Performance Constraints

### 3.1 Hand Evaluator: >= 200M evals/sec (PRF-001, HEV-002)

| Constraint | Rationale |
|-----------|-----------|
| **Lookup tables mandatory** | 200M evals/sec is only achievable with precomputed tables. Runtime hand ranking computation is ~100x slower. |
| **No allocation in hot path** | `evaluate()` must not allocate heap memory. All lookups use stack-local indices into pre-loaded arrays. |
| **Table size ~10 MB** | Tables must fit in L3 cache for sustained throughput. Larger tables cause cache misses. |
| **`#[inline]` on critical functions** | The evaluate function and its helpers must be inlined for the compiler to optimize the hot loop. |
| **Release mode only** | Debug builds are ~10-20x slower due to bounds checking and lack of optimization. Benchmarks MUST use `--release`. |

**Active from:** M1

### 3.2 Solver: < 10s for Standard Spot (PRF-002, RTS-002)

| Constraint | Rationale |
|-----------|-----------|
| **MCCFR required** | Full CFR enumerates all chance outcomes — too slow for real-time. MCCFR samples chance nodes. |
| **rayon parallelism required** | Single-threaded CFR is ~8x too slow on 8-core hardware. Must distribute tree traversal across cores. |
| **Arena-allocated game tree** | Game tree nodes must be arena-allocated (single `Vec<GameNode>`) for cache locality. No individual `Box<Node>` allocations. |
| **Regret tables: contiguous memory** | Strategy/regret vectors stored as flat `Vec<f32>` indexed by info set ID. No `HashMap` lookups in the hot loop. |
| **Atomic operations for shared state** | Parallel threads updating shared regret tables must use `AtomicF32` or per-thread local copies merged at iteration boundaries. |

**Active from:** M2

#### Parallel Regret Accumulation

During parallel MCCFR iterations, regret and strategy values are updated concurrently. The required approach:

- **Per-thread local copies:** Each worker thread maintains a local regret accumulation buffer (not shared).
- **Merge after iteration:** After each iteration batch completes, local buffers are merged into the global regret table using a single-threaded reduction step.
- **Rationale:** Atomic operations on regret values cause excessive cache-line contention on multi-core systems. Per-thread copies with periodic merging achieve near-linear speedup.
- **Merge frequency:** Merge every N iterations where N = batch_size (typically 100-1000). Configurable via solver settings.

### 3.3 Solution Loading: < 500ms for 500 MB (PRF-004)

| Constraint | Rationale |
|-----------|-----------|
| **mmap mandatory** | Reading 500 MB from disk into RAM takes ~1-2 seconds on SSD. Memory mapping avoids the full read. |
| **No full-file decompression** | The file is zstd-compressed in blocks. Only the requested node's block is decompressed on demand. |
| **Partial decompression** | Individual node access decompresses only the relevant segment. Average access latency: < 1 ms. |
| **Solution cache** | LRU cache of recently loaded nodes avoids redundant decompression. |

**Active from:** M2 (serialization), M3 (full loading pipeline)

#### Solution File Block Structure

Solutions use a blocked compression format for random-access mmap reads:

- **Block size:** 64 KB uncompressed (last block may be smaller).
- **Compression:** Each block is independently zstd-compressed.
- **Block offset index:** A footer at the end of the file contains an array of `(uncompressed_offset: u64, compressed_offset: u64)` pairs, one per block.
- **Footer structure:** `[block_index_array][block_count: u32][magic: u32]`
- **Random access:** To read byte range `[a, b)`, compute which blocks overlap, decompress only those blocks, and extract the requested bytes.

This design allows O(1) seeking into any part of the solution without decompressing the entire file.

### 3.4 UI Rendering: 60fps (PRF-003)

| Constraint | Rationale |
|-----------|-----------|
| **Canvas for hand matrix** | The 13x13 matrix with mixed-strategy coloring requires up to 169 * N color segments per frame. DOM-based rendering causes layout thrashing. Canvas is immediate-mode. |
| **No DOM-based 13x13 grid** | A 169-cell grid with per-cell color gradients, hover handlers, and animation causes > 16.7 ms layout when using DOM elements. |
| **React.memo on stable subtrees** | Components that don't change between frames (navigation, static labels) must be memoized. |
| **Virtualized long lists** | Analyzed hands list (potentially 100K+ rows) must use `react-window` or equivalent. No rendering of off-screen rows. |
| **requestAnimationFrame for Canvas** | Canvas redraws scheduled via rAF, not synchronous React render cycles. |
| **Debounced IPC calls** | Mouse hover on matrix cells must not fire an IPC call per pixel. Debounce to 50-100 ms. |

**Active from:** M1 (matrix), M3 (study mode interactions)

### 3.5 Startup Time: < 3s (PRF-006)

| Constraint | Rationale |
|-----------|-----------|
| **Lazy eval table loading** | Eval tables loaded in background after UI is interactive. Not blocking the main thread. |
| **No synchronous DB queries on startup** | SQLite schema check and config load are fast, but large index queries must be deferred. |
| **Skeleton UI first** | Render navigation and empty content area immediately, then load data. |

**Active from:** M1 (SHOULD, enforced in M7)

---

## 4. Prohibited Patterns

### 4.1 Rust — Library Crates (`poker-core`, `poker-eval`, `poker-solver`, `poker-solution`, `poker-hhparser`, `poker-icm`, `poker-analyze`)

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| `unwrap()` | Panics crash the entire Tauri process | `?` operator with proper `Error` types; `expect()` only in truly unreachable paths with justification comment |
| `unsafe` without justification | Memory safety is the primary Rust benefit | Document the safety invariant in a `// SAFETY:` comment if truly needed |
| `println!()` / `eprintln!()` | Output goes nowhere in a GUI app | `tracing::info!()`, `tracing::error!()` via the `tracing` crate |
| `std::thread::spawn` for compute | Unmanaged threads bypass the rayon pool | `rayon::spawn()` or `rayon::scope()` for CPU-bound work |
| Blocking on Tokio async runtime | Deadlocks the Tauri event loop | `tokio::task::spawn_blocking()` for CPU-bound work; never call `.blocking_lock()` from async context |
| `HashMap` in solver hot path | Hash computation + random memory access kills cache performance | Use flat `Vec<T>` indexed by info set ID |
| Individual `Box<Node>` allocations | Pointer chasing destroys cache locality in tree traversal | Arena allocation: single `Vec<GameNode>` with index-based references |
| `.clone()` on large data structures | Hidden O(n) copies cause memory spikes | Pass references; use `Arc` for shared ownership; use `Cow` for conditional ownership |
| `String` for card representation | Allocation + UTF-8 overhead for a 1-byte value | Use `Card` (packed `u8`) from `poker-core` |

### 4.2 Rust — Binary Crate (`app-tauri`)

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| Long-running computation in `#[tauri::command]` | Blocks the Tokio runtime; UI freezes | Spawn to rayon thread pool via `spawn_blocking`; return progress via events |
| Accessing `AppState` without proper guards | Data races on mutable state | `Mutex` for mutable state (solver sessions); `RwLock` for read-heavy state (solution cache) |
| Returning raw `String` errors from commands | Frontend cannot parse or display meaningfully | Return `AppError` enum with structured fields |
| Panicking in command handlers | Crashes the application | Catch all panics at the command boundary; return as `AppError` |

### 4.3 Frontend — TypeScript/React

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| Direct DOM manipulation | Breaks React's virtual DOM reconciliation | Use refs + Canvas API for hand matrix; React state for everything else |
| Cross-module imports | Couples feature modules; blocks independent development | Import only from `shared/` and `ipc/`; use router for cross-mode navigation |
| External state library (Redux, Zustand, MobX) | Unnecessary for a desktop app with no server sync | React Context (global) + useReducer (per-mode) |
| `any` type | Disables TypeScript's type checking benefit | Proper typed interfaces matching Rust `serde` types |
| Inline styles for theming | Inconsistent, hard to maintain | Theme tokens from `shared/styles/theme.ts` |
| `setInterval` for progress polling | Wastes CPU; misses updates; stale state | Tauri event listeners (push-based) |
| `eval()` or dynamic code execution | Blocked by CSP; security vulnerability | Static imports only |
| Uncontrolled form inputs | State drift between React state and DOM | Controlled components with React state |
| `window.fetch()` for backend calls | No REST server; Tauri IPC is the only backend interface | `@tauri-apps/api` invoke wrappers in `ipc/` |

### 4.4 IPC

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| Fine-grained RPC (many small calls) | IPC overhead per call (~0.5ms JSON serialize/deserialize) | Coarse-grained commands: one call per user action |
| Polling for progress | Wastes CPU; UI updates lag behind reality | Event stream: `emit()` from Rust, `listen()` in TS |
| Untyped payloads | Runtime type errors cross the boundary silently | Typed request/response structs on both sides |
| Passing large binary data as JSON | JSON encoding of binary data is 33% larger (base64) + slow to parse | Reference by ID; frontend requests specific nodes via ID-based commands |
| Synchronous IPC calls | Blocks the UI thread | All IPC commands are async |

**Coarse-grained IPC examples (correct):**
- `solve_spot(config)` — one command per user click on "Solve"
- `get_strategy(node_id)` — one command per node selection
- `import_hand_history(file_path)` — one command per file import

**Fine-grained IPC examples (prohibited):**
- Sending individual card evaluations over IPC (batch them)
- Polling solver progress every 10ms (use Tauri event push instead)
- Fetching individual cell values from the hand matrix (send full matrix data in one call)

**Debounced hover/interaction:** Hover-triggered IPC (e.g., tooltip data for a matrix cell) is acceptable IF debounced to >=100ms intervals and the payload is a single request/response.

### 4.5 Storage

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| Full-file read for solutions | 500 MB read takes 1-2 seconds; exceeds PRF-004 budget | Memory-mapped I/O via `memmap2` with partial decompression |
| Uncompressed solution writes | Solutions are 3-10x larger uncompressed; wastes disk space | zstd compression (level 3) for all solution files (STO-002) |
| Raw SQL strings in Rust | SQL injection risk; hard to maintain | `rusqlite` with parameterized queries; consider `sqlx` for compile-time checked queries |
| Multiple SQLite databases | Complicates backup, migration, and integrity | Single `data.db` with multiple tables |
| Network file paths for data.db | SQLite WAL mode unreliable on network drives | Enforce local filesystem paths only |
| Auto-vacuum on every operation | Causes unpredictable pauses | Manual vacuum on user request or periodic schedule |

### 4.6 Solver-Specific

| Prohibited | Why | Use Instead |
|-----------|-----|-------------|
| Shared mutable state without synchronization | Data races in parallel CFR tree traversal | Atomic operations (`AtomicF32`), or per-thread local copies merged after iteration |
| Unbounded memory allocation | Solver can OOM on complex trees | Check `memory_limit_gb` before allocating; abort with error if limit approached |
| Ignoring cancellation token | Solver runs forever if user cancels and token is not checked | Check `CancellationToken` between every iteration |
| Floating-point comparisons with `==` | Strategy frequencies have rounding errors | Use epsilon-based comparison (tolerance: 1e-6) or quantized integer comparisons |
| Non-deterministic behavior without opt-in | Makes debugging and testing unreliable | Default to deterministic seed; optional random seed for production diversity |

---

## 5. Gotchas & Edge Cases

### 5.1 Suit Isomorphism (SOL-011, HEV-003)

**The mapping of 22,100 flops → 1,755 canonical forms must be consistent everywhere.**

| Gotcha | Detail |
|--------|--------|
| **Canonical form must be deterministic** | The same suit permutation algorithm must be used in `poker-eval::isomorphism`, `poker-solver::game_tree`, and `poker-solution::serialize`. If they disagree, solutions are indexed under wrong canonical forms. |
| **Hole card remapping** | When a board is canonicalized (e.g., A♠K♠7♦ → A♣K♣7♥), the players' hole cards must be remapped with the same suit permutation. Forgetting this produces wrong equity calculations. |
| **Turn/river extension** | Adding a turn card to a canonical flop must re-canonicalize the 4-card board. The canonical form of a 4-card board may differ from the canonical flop + arbitrary turn. |
| **Consistency check** | A test must verify: for every flop, `canonicalize(flop)` produces one of exactly 1,755 outputs, and `canonicalize(canonicalize(flop)) == canonicalize(flop)` (idempotent). |

### 5.2 Hand Evaluation Edge Cases (HEV-001)

| Edge Case | Detail |
|-----------|--------|
| **Wheel straight** | A-2-3-4-5 is the lowest straight. The ace plays low. The evaluator must rank it below 2-3-4-5-6. |
| **Split pots** | Two hands with identical rank (e.g., both have K-high flush with same kickers from the board) must produce `Ordering::Equal`. |
| **Kicker comparison** | Two pair: Q-Q-7-7-A beats Q-Q-7-7-K. The 5th card (kicker) matters. Full evaluation must compare all relevant cards. |
| **Board plays** | If the 5 community cards form the best hand for both players, it's a split pot. The evaluator must handle this without special-casing. |
| **Counterfeit** | A player with 7-7 on a board of A-A-K-K-Q has their pair counterfeited — their best hand is A-A-K-K-Q (no sevens used). The evaluator must select the best 5 from 7 cards. |

### 5.3 Hand History Parser Edge Cases (HHP-001 through HHP-008)

| Edge Case | Detail |
|-----------|--------|
| **Straddles** | Some cash game hands have a straddle (third blind). The parser must handle straddle as a forced bet and adjust position labels accordingly. |
| **Side pots** | Multi-way pots where a player is all-in create side pots. `reconstruct_pot()` must track main pot and side pots separately. |
| **Missing showdown** | Many hands end without showdown (all fold). Hole cards may not be shown. The parser must handle absent hole card data gracefully. |
| **Site format changes** | Poker sites periodically change their hand history format (new fields, changed delimiters, different encodings). The trait-based `SiteParser` architecture isolates these changes. |
| **Currency symbols** | Different sites use `$`, `€`, `£`, or chip values. The parser must extract the numeric value regardless of currency prefix/suffix. |
| **Table size detection** | A "6-max" table with only 3 players still uses 6-max position labels. Table size comes from table metadata, not player count. |
| **Antes in tournaments** | Tournament antes may be posted by all players, by the big blind only (big blind ante), or by a specific position. Each format variation needs handling. |
| **Hand IDs** | Each poker site uses different hand ID formats. PokerStars uses numeric IDs; GGPoker uses alphanumeric. IDs must be stored as strings. |

### 5.4 ICM Complexity (ICM-001)

| Gotcha | Detail |
|--------|--------|
| **Factorial complexity** | Malmuth-Harville ICM computes probability of each finishing order. For N players, this involves N! permutations. At 9 players: 362,880 permutations. |
| **Caching required** | ICM equity for a given stack distribution + payout structure should be cached. Re-computing during solver iterations is prohibitive. |
| **Numerical precision** | ICM probabilities involve products of fractions that can underflow to zero. Use log-space computation for intermediate values. |
| **Equal stacks** | When two players have identical chip stacks, their ICM equities must be identical. Floating-point arithmetic may produce tiny differences — round to eliminate. |

### 5.5 SQLite (STO-006)

| Gotcha | Detail |
|--------|--------|
| **WAL mode required** | Without WAL (Write-Ahead Logging), concurrent reads block during writes. WAL mode allows concurrent readers + one writer. |
| **Connection pooling** | Multiple Tauri command handlers may query the DB concurrently. Use `r2d2` or `deadpool` connection pool, not a single connection. |
| **Migration safety** | Schema changes between milestones must use versioned migrations. Check schema version on startup; apply pending migrations. |
| **VACUUM after deletions** | Deleting solution entries doesn't shrink the DB file. Periodic `VACUUM` reclaims space. |
| **32-bit integer limits** | SQLite `INTEGER` is 64-bit, but some ORMs/wrappers default to 32-bit. Use `i64` in Rust for all SQLite integer columns. |

### 5.6 Solver Convergence (SOL-002)

| Gotcha | Detail |
|--------|--------|
| **Nash Distance is an estimate** | MCCFR Nash Distance fluctuates between iterations due to sampling variance. Convergence should be assessed over a moving average, not a single iteration. |
| **CFR+ regret floor** | CFR+ clips negative cumulative regrets to zero. This accelerates convergence but can cause oscillation in some game trees. Monitor Nash Distance for non-monotonic behavior. |
| **Action pruning** | Actions with very negative regret can be pruned (not explored). Aggressive pruning speeds convergence but can miss optimal strategies in rare board runouts. |
| **Warm-start for re-solve** | When nodelocking and re-solving (M6), initialize from the GTO solution rather than uniform random. Warm-starting converges in ~10% of the iterations. |

### 5.7 Frontend Canvas Rendering (PRF-003)

| Gotcha | Detail |
|--------|--------|
| **DPI scaling** | On high-DPI displays, Canvas must account for `window.devicePixelRatio`. Set canvas width/height to logical pixels * DPR, then scale with CSS. |
| **Font rendering on Canvas** | `fillText()` on Canvas produces blurry text at fractional positions. Round text positions to integer pixel coordinates. |
| **Color space** | Canvas uses sRGB. Ensure action colors from `ActionColorSystem.ts` are defined in sRGB. |
| **Touch events** | If running on touch-screen devices, Canvas must handle `touchstart`/`touchmove` in addition to `mousedown`/`mousemove`. |
| **Memory leaks** | Creating new `CanvasRenderingContext2D` objects or large `ImageData` buffers per frame leaks memory. Reuse context and buffers. |

#### Canvas Rendering Degradation Strategy

The hand matrix Canvas renderer (PRF-003) must maintain 60fps (< 16.7ms per frame):

1. **Benchmark requirement:** Measure frame time on reference hardware (Intel i7-10700K, 32 GB DDR4-3200, integrated GPU). If average frame time exceeds 12ms, optimize before shipping.
2. **Degradation tiers:**
   - **Tier 1 (< 8ms):** Full rendering — gradients, anti-aliased text, hover highlights, animations.
   - **Tier 2 (8-14ms):** Reduce quality — disable gradients, use solid fills, simplify hover effects.
   - **Tier 3 (> 14ms):** Minimal rendering — no animations, static cell colors, batch text rendering.
3. **Auto-detection:** Measure rolling average frame time over 60 frames. Automatically step down/up tiers if threshold is exceeded for 3 consecutive seconds.
4. **User override:** Settings option to force a specific rendering tier.

---

## 6. Per-Milestone Constraint Activation Index

This table maps each constraint to the milestone(s) where it becomes active.

### 6.1 Active from M1

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| No `unwrap()` in library crates | §4.1 | `poker-core` and `poker-eval` are library crates from M1 |
| No `unsafe` without justification | §4.1 | All crates from M1 |
| No `println!()` — use `tracing` | §4.1 | All crates from M1 |
| Lookup-table evaluator (no runtime computation) | §3.1 | PRF-001 target from M1 |
| Canvas-based hand matrix (no DOM grid) | §3.4, §4.3 | PRF-003 from M1 |
| No cross-module frontend imports | §4.3 | Module isolation from M1 |
| No external state library | §4.3 | Architecture decision from M1 |
| No `any` types in TypeScript | §4.3 | Type safety from M1 |
| Use `std::path::Path` (no string paths) | §2.2 | Cross-platform from M1 |
| `~/.poker-solver/` data directory | §2.2 | DSK-002 from M1 |
| SQLite WAL mode | §5.5 | Database initialized in M1 |
| Dark theme default | — | UIF-001 from M1 |
| Keyboard shortcuts must not conflict with text inputs | — | UIF-002 from M1 |

### 6.2 Active from M2

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| MCCFR for real-time solving | §3.2 | PRF-002 target from M2 |
| rayon for parallel tree traversal | §3.2 | SOL-009 from M2 |
| Arena-allocated game tree | §3.2, §4.1 | Performance from M2 |
| No `HashMap` in solver hot path | §4.1 | Solver performance from M2 |
| No blocking on Tokio runtime | §4.1 | Solver spawn_blocking from M2 |
| CancellationToken checked between iterations | §4.6 | SOL-010, SOL-018 from M2 |
| Memory-mapped solution loading | §3.3 | STO-003 from M2 |
| zstd compression for solutions | §4.5 | STO-002 from M2 |
| No full-file reads for solutions | §4.5 | PRF-004 from M2 |
| Coarse-grained IPC (not fine-grained RPC) | §4.4 | Architecture from M2 |
| Event streams for progress (not polling) | §4.4 | SOL-018 from M2 |
| Quantized strategy storage (u16) | — | STO-004 from M2 |
| Sparse encoding for skewed nodes | — | STO-011 from M2 |
| Memory budget: 4 GB standard, 16 GB solving | §2.3 | PRF-005 from M2 |

### 6.3 Active from M3

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| 60fps UI rendering | §3.4 | PRF-003 enforced from M3 study mode |
| Solution loading < 500ms | §3.3 | PRF-004 from M3 |
| ICM caching for factorial complexity | §5.4 | ICM-001 from M3 |
| Suit isomorphism consistency (evaluator = solver = solution) | §5.1 | Cross-crate consistency from M3 |
| Canvas DPI scaling | §5.7 | Matrix display in study mode from M3 |
| Debounced IPC on hover | §3.4 | Study mode matrix interaction from M3 |

### 6.4 Active from M4

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| Cryptographic PRNG for mixed strategies | §5.6 | PRA-002, PRA-007 from M4 |
| Practice session state persistence in AppState | — | Session survives navigation from M4 |

### 6.5 Active from M5

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| Parser error handling (skip bad hands, continue) | §5.3, §4.1 | HHP-005 from M5 |
| Character encoding detection | §5.3 | HHP-006 from M5 |
| Pot reconstruction accuracy | §5.3 | HHP-007 from M5 |
| Batch import parallelism via rayon | — | ANL-002 from M5 |
| Virtualized hand lists | §3.4 | Large hand databases from M5 |

### 6.6 Active from M6

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| Warm-start for nodelocked re-solve | §5.6 | NLK-003 from M6 |
| Streaming aggregation (not all-in-memory) | — | 1,755 flops aggregate from M6 |

### 6.7 Active from M7

| Constraint | Section | Rationale |
|-----------|---------|-----------|
| Startup time < 3s | §3.5 | PRF-006 enforced in M7 |
| Multi-core scaling >= 6x on 8 cores | §3.2 | PRF-007 enforced in M7 |
| Format version migration | §5.5 | STO-009 from M7 |

---

## 7. Dependency & Tooling Constraints

### 7.1 Rust Toolchain

| Constraint | Detail |
|-----------|--------|
| **Edition 2021** | All crates use `edition = "2021"` in Cargo.toml. |
| **Stable toolchain only** | No nightly-only features. All code must compile on stable Rust. |
| **Workspace dependencies** | Shared dependencies (`serde`, `tracing`, `thiserror`, `tokio`) declared in workspace `Cargo.toml` `[workspace.dependencies]` and inherited by crates via `{ workspace = true }`. Prevents version drift. |
| **`cargo clippy` clean** | All library and binary crates must pass `cargo clippy -- -D warnings` with zero warnings. Clippy lint suppressions (`#[allow(...)]`) require a justification comment. |
| **`cargo fmt` enforced** | All Rust code formatted with `cargo fmt`. No manual formatting overrides. |
| **Feature flags** | No Cargo feature flags unless gating optional heavyweight dependencies (e.g., a future GPU compute path). Compile-time configuration uses `config.toml` or build-time constants. |

### 7.2 Frontend Toolchain

| Constraint | Detail |
|-----------|--------|
| **React 18** | Use concurrent features (Suspense, transitions) for deferred rendering. No legacy class components. |
| **TypeScript strict mode** | `tsconfig.json` must have `"strict": true`. No `// @ts-ignore` without justification. |
| **Vite bundler** | Dev server and production build via Vite. No webpack, Parcel, or other bundlers. |
| **Vitest for tests** | Frontend unit tests use Vitest (compatible with Vite). No Jest or Mocha. |
| **ESLint + Prettier** | Linting with ESLint (recommended React/TypeScript rules). Formatting with Prettier. Both must pass in CI. |
| **No CDN dependencies** | All npm packages bundled locally. No `<script>` tags to external CDNs (CSP violation, offline requirement). |

### 7.3 Tauri CLI

| Constraint | Detail |
|-----------|--------|
| **Tauri v2** | Use `@tauri-apps/cli` v2.x. Tauri v1 APIs are incompatible. |
| **Plugin versions** | Tauri plugins must match the v2 release channel. Check compatibility before adding plugins. |
| **`tauri.conf.json` validation** | Tauri CLI validates `tauri.conf.json` on build. Invalid fields cause silent failures — always test with `cargo tauri dev` after config changes. |
| **Bundle identifiers** | macOS and Windows installers require unique bundle identifiers (e.g., `com.poker-solver.app`). Set in `tauri.conf.json` before first build. |

### 7.4 Crate Dependencies — Allowed List

Only these external crates are pre-approved. Adding unlisted crates requires justification.

| Category | Crate | Purpose | Used By |
|----------|-------|---------|---------|
| Serialization | `serde`, `serde_json`, `bincode` | JSON + binary serialization | All crates |
| Error handling | `thiserror`, `anyhow` | `thiserror` in libraries; `anyhow` only in `app-tauri` | All crates |
| Async runtime | `tokio` | Tauri async runtime (not used in library crates) | `app-tauri` |
| Parallelism | `rayon` | CPU-parallel work stealing | `poker-solver`, `poker-eval`, `poker-analyze` |
| Logging | `tracing`, `tracing-subscriber` | Structured logging | All crates |
| Database | `rusqlite`, `r2d2` | SQLite access + connection pool | `app-tauri` |
| Memory mapping | `memmap2` | Memory-mapped solution files | `poker-solution` |
| Compression | `zstd` | Solution file compression | `poker-solution` |
| Random | `rand`, `rand_chacha` | Deterministic + crypto PRNG | `poker-core`, `poker-solver` |
| Benchmarking | `criterion` | Benchmark framework | `poker-eval`, `poker-solver`, `poker-solution` |
| Desktop framework | `tauri` | Desktop shell | `app-tauri` |
| TOML config | `toml` | Config file parsing | `app-tauri` |

---

## 8. Security Constraints

### 8.1 Local-First Security Model

This is a **local-first application** — no user accounts, no cloud sync, no telnet. Security focuses on data integrity and safe defaults.

| Constraint | Detail | Requirement |
|-----------|--------|-------------|
| **No network calls** | The application must not make any outgoing network requests during normal operation. Auto-update (M7, DSK-008) is the only exception and requires explicit user consent. | DSK-005 |
| **No telemetry** | No analytics, crash reporting, or usage tracking without explicit opt-in. | Privacy |
| **CSP enforced** | Tauri's Content Security Policy prevents XSS. Do not weaken CSP directives. | DSK-001 |
| **Path traversal prevention** | All file operations must validate paths stay within `~/.poker-solver/`. Reject paths containing `..` that escape the data directory. | STO-006 |
| **SQLite parameterized queries** | All database queries use parameterized statements. No string interpolation into SQL. | STO-006, §4.5 |
| **User data stays local** | Solution files, hand histories, and analysis databases never leave the local filesystem. Export features write to user-chosen paths only. | DSK-005 |

---

## 9. Glossary

| Term | Definition |
|------|------------|
| **Nash Equilibrium** | A strategy profile where no player can improve their expected value by unilaterally changing strategy. In poker, approximated as a GTO (Game Theory Optimal) solution. |
| **Nash Distance** | A measure of how far a strategy profile is from Nash Equilibrium, defined as the sum of each player's exploitability. Measured in milli-big-blinds per hand (mbb/h). |
| **Exploitability** | The maximum additional EV (in mbb/h) an opponent could gain by playing a best-response strategy against the given strategy. Lower = closer to GTO. |
| **Information Set** | The set of all game states indistinguishable to a player given their private information (hole cards) and the public history (actions + board). |
| **CFR (Counterfactual Regret Minimization)** | An iterative algorithm that converges to Nash Equilibrium by accumulating regret values for each action at each information set. |
| **CFR+** | A CFR variant that floors negative cumulative regrets to zero, accelerating convergence. Default algorithm for M2. |
| **DCFR (Discounted CFR)** | A CFR variant that applies time-based discounting to regrets and strategies, often converging faster than vanilla CFR. |
| **Linear CFR** | A CFR variant that weights iterations linearly (later iterations count more), typically faster convergence for large games. |
| **Regret** | The difference between the EV of taking a specific action and the EV of the current strategy at an information set. Positive regret means the action was under-used. |
| **Suit Isomorphism** | The observation that suits in poker are interchangeable (only relative suit relationships matter), allowing equivalent game states to be merged, reducing the game tree. |
| **Nodelock** | Fixing a node's strategy to specific action probabilities and re-solving the rest of the tree. Used to model opponent deviations from GTO. |
| **ICM (Independent Chip Model)** | A model that converts tournament chip stacks to prize equity, accounting for the non-linear value of chips in tournaments. |
| **Bubble Factor** | The ratio of tournament equity lost by busting to equity gained by doubling up, derived from ICM. Always >= 1 on the bubble. |
| **EV (Expected Value)** | The average outcome over all possible scenarios, weighted by probability. Measured in big blinds (bb) or chips. |
| **mbb/h** | Milli-big-blinds per hand. Standard unit for exploitability and Nash Distance. 1 mbb/h = 0.001 bb per hand. |
| **Game Tree** | The full tree of all possible action sequences in a poker hand. Nodes represent decision points; edges represent actions. |
| **Arena Allocation** | Memory management pattern where all nodes are stored in a contiguous `Vec<GameNode>` and referenced by index rather than pointers. Eliminates pointer-chasing overhead. |
| **mmap (Memory-Mapped I/O)** | OS mechanism that maps file contents directly into virtual address space, allowing random access without explicit read() calls. Used for solution file access. |
| **zstd** | Zstandard compression algorithm by Facebook/Meta. Used for solution file compression at level 3 (fast decompression). |
| **Quantization** | Converting f32 strategy/regret values to u16 for storage. Reduces solution file size by ~50% with bounded precision loss. |

---

## Change Log

| Date | Version | Description |
|------|---------|-------------|
| 2026-02-20 | 1.0 | Initial technical constraints document |

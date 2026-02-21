# GTO Wizard Architecture Blueprint

## Overview

GTO Wizard is a cloud-based poker strategy platform serving precomputed GTO solutions and real-time AI-powered custom solving to a global user base. This document presents an inferred architecture based on publicly available information, including blog posts, help documentation, job listings, technology references, and performance benchmarks. Where specific implementation details are not disclosed, reasonable inferences are drawn from the platform's observable behavior, stated performance characteristics, and industry norms.

---

## 1. High-Level System Architecture

```
+-----------------------------------------------------------------------+
|                          CLIENT LAYER                                  |
|                                                                        |
|   +------------------+    +------------------+    +------------------+ |
|   | Web Application  |    | Mobile (PWA/App) |    | API Clients      | |
|   | (React/Next.js)  |    |                  |    | (Python SDK)     | |
|   +--------+---------+    +--------+---------+    +--------+---------+ |
|            |                       |                       |           |
+-----------------------------------------------------------------------+
             |                       |                       |
             v                       v                       v
+-----------------------------------------------------------------------+
|                       CDN / EDGE LAYER                                 |
|                                                                        |
|   +------------------+    +------------------+    +------------------+ |
|   | Static Assets    |    | API Gateway /    |    | WebSocket        | |
|   | (JS, CSS, media) |    | Load Balancer    |    | Proxy            | |
|   +------------------+    +--------+---------+    +--------+---------+ |
|                                    |                       |           |
+-----------------------------------------------------------------------+
             |                       |                       |
             v                       v                       v
+-----------------------------------------------------------------------+
|                      APPLICATION LAYER                                 |
|                                                                        |
|   +------------------+    +------------------+    +------------------+ |
|   | Auth / Account   |    | Solution Lookup  |    | Custom Solve     | |
|   | Service          |    | Service          |    | Orchestrator     | |
|   +------------------+    +--------+---------+    +--------+---------+ |
|                                    |                       |           |
|   +------------------+    +------------------+    +------------------+ |
|   | HH Processing    |    | Report Engine    |    | Subscription /   | |
|   | Pipeline         |    | (Power Credits)  |    | Billing Service  | |
|   +------------------+    +------------------+    +------------------+ |
|                                                                        |
+-----------------------------------------------------------------------+
             |                       |                       |
             v                       v                       v
+-----------------------------------------------------------------------+
|                       COMPUTE LAYER                                    |
|                                                                        |
|   +---------------------------+    +-------------------------------+   |
|   | Precomputed Solution      |    | GTO Wizard AI Solver          |   |
|   | Database                  |    | (Ruse AI Engine)              |   |
|   |                           |    |                               |   |
|   | - CFR-solved strategies   |    | - Neural network inference   |   |
|   | - Indexed by format,      |    | - Depth-limited CFR          |   |
|   |   stack, action, board    |    | - Dynamic sizing             |   |
|   | - Compressed storage      |    | - Multiway support           |   |
|   +---------------------------+    +-------------------------------+   |
|                                                                        |
|   +---------------------------+    +-------------------------------+   |
|   | Hand Evaluator            |    | Training / Model Pipeline     |   |
|   | (C++, 400M hands/sec)     |    | (Self-play, CFR training)     |   |
|   +---------------------------+    +-------------------------------+   |
|                                                                        |
+-----------------------------------------------------------------------+
             |                       |                       |
             v                       v                       v
+-----------------------------------------------------------------------+
|                        DATA LAYER                                      |
|                                                                        |
|   +------------------+    +------------------+    +------------------+ |
|   | Solution Store   |    | User Database    |    | Analytics /      | |
|   | (Strategies,     |    | (Accounts, HH,   |    | Telemetry        | |
|   |  EV tables)      |    |  progress, subs) |    |                  | |
|   +------------------+    +------------------+    +------------------+ |
|                                                                        |
+-----------------------------------------------------------------------+
```

---

## 2. Frontend Architecture

### 2.1 Web Application

Based on the platform's interactive UI, real-time range visualization, and single-page-application behavior, the frontend is likely built with a modern JavaScript framework. Evidence points toward **React** or **Next.js**:

- Rich interactive components (range grids, strategy trees, EV charts)
- Client-side state management for complex solver interfaces
- Server-side rendering for SEO on public pages (blog, glossary, landing pages)
- Progressive enhancement and responsive design for mobile web

### 2.2 Key Frontend Components

| Component | Function | Technical Notes |
|-----------|----------|----------------|
| **Range Grid Viewer** | Displays 13x13 hand matrix with color-coded action frequencies | Canvas or SVG rendering; requires efficient updates |
| **Game Tree Navigator** | Visualizes decision trees with expandable nodes | Tree data structure with lazy loading |
| **EV Comparison Tool** | Side-by-side strategy analysis with bar charts | Charting library (e.g., D3.js, Recharts) |
| **Practice Mode Interface** | Real-time poker table simulation | WebSocket or polling for AI opponent responses |
| **Hand History Viewer** | Street-by-street replay with annotations | Custom player component |
| **Board Selector** | Card picker with suit/rank grid | Custom component with drag-and-drop |

### 2.3 State Management

The application manages complex state across multiple dimensions:

- Current game configuration (format, stacks, positions, bet sizes)
- Solution data (strategy arrays, EV tables, frequency distributions)
- Navigation state (which street, which node in the tree)
- User session (subscription tier, power credits balance, study progress)

Given the volume of strategy data transferred per page view (a single tree can contain millions of data points), the frontend likely uses:

- Aggressive data pagination (loading one street at a time)
- Web Workers for off-thread computation (equity calculations, range filtering)
- IndexedDB or in-memory caching for recently viewed solutions

---

## 3. Backend API Layer

### 3.1 Technology Stack

The CTO (Viktor Stiskala) has public skills in **Python**, **Django**, and **PHP**, suggesting a Python-based backend. The architecture likely uses:

- **Django** or **FastAPI** for the primary API server
- RESTful endpoints for solution queries, user management, and billing
- WebSocket connections for real-time features (Practice Mode AI, custom solve progress)
- Background task queues for asynchronous operations (HH processing, report generation, custom solves)

### 3.2 API Endpoints (Inferred)

```
GET  /api/solutions/{format}/{stacks}/{action}/{board}/{street}
     -> Returns precomputed strategy data

POST /api/solve/custom
     -> Initiates GTO Wizard AI custom solve
     -> Body: ranges, bet sizes, stacks, board, format
     -> Returns: job_id for polling or WebSocket subscription

GET  /api/solve/status/{job_id}
     -> Returns solve progress and results

POST /api/analyze/hand-history
     -> Upload hand history for analysis
     -> Consumes power credits

GET  /api/user/profile
     -> Account info, subscription, credits balance

GET  /api/reports/{report_id}
     -> Aggregated report data
```

### 3.3 Authentication and Authorization

- OAuth 2.0 or session-based authentication
- Subscription tier enforcement at the API layer
- Power credit deduction for compute-intensive operations
- Rate limiting to prevent abuse of custom solving resources

---

## 4. Solver Compute Infrastructure

### 4.1 Precomputed Solution Generation

The offline pipeline for generating precomputed solutions runs on dedicated compute infrastructure:

```
+-------------------+      +-------------------+      +-------------------+
|  Configuration    | ---> |  CFR Solver       | ---> |  Solution Store   |
|  Generator        |      |  Cluster          |      |  (Compressed)     |
|                   |      |                   |      |                   |
|  - Format defs    |      |  - CPU-optimized  |      |  - Indexed by     |
|  - Stack depths   |      |  - Multi-threaded |      |    config hash    |
|  - Action trees   |      |  - ~0.1-0.3% dEV  |      |  - Versioned      |
|  - Bet size grids |      |  - Hours per spot |      |  - Cacheable      |
+-------------------+      +-------------------+      +-------------------+
```

Traditional CFR solving is CPU-bound and memory-intensive:
- **16+ CPU cores at high clock speeds** (5+ GHz) per solve instance
- **128+ GB RAM** for large game trees (a single 100bb HU tree can be 95 GB)
- Solutions are batched and run continuously as new formats/configurations are added

### 4.2 GTO Wizard AI (Real-Time Solving)

The AI solving infrastructure is fundamentally different:

```
+-------------------+      +-------------------+      +-------------------+
|  User Request     | ---> |  AI Solve Worker  | ---> |  Response Cache   |
|  (Custom params)  |      |                   |      |                   |
|                   |      |  - NN inference   |      |  - TTL-based      |
|  - Ranges         |      |    (GPU/CPU)      |      |  - Keyed by       |
|  - Bet sizes      |      |  - Single-street  |      |    param hash     |
|  - Stack depth    |      |    CFR solve      |      |                   |
|  - Board          |      |  - 2 cores, 8GB   |      |                   |
|  - Format         |      |  - ~3 sec/street  |      |                   |
+-------------------+      +-------------------+      +-------------------+
```

- **Neural network inference** may run on GPU (for batch throughput) or optimized CPU inference (for latency)
- **CFR solving** for each street runs on CPU with the neural network providing leaf values
- Worker pool scales horizontally to handle concurrent custom solve requests
- Power credit system meters access to this compute-intensive resource

### 4.3 Hand Evaluator

GTO Wizard's hand evaluator, referenced in their MIT poker competition entry, is written in **C++** and achieves:

- **400 million random hand evaluations per second** on a single thread
- Support for 5 to 8 card hands
- Lookup-table-based evaluation for speed
- Used throughout the solver pipeline and in real-time gameplay simulation

---

## 5. Solution Database and Caching

### 5.1 Solution Storage Architecture

```
+-----------------------------------------------------------------------+
|                    SOLUTION DATA PIPELINE                              |
|                                                                        |
|  +----------------+     +----------------+     +------------------+   |
|  | Raw Solutions  | --> | Compression &  | --> | Solution Store   |   |
|  | (CFR output)   |     | Quantization   |     | (Object/Blob)    |   |
|  |                |     |                |     |                  |   |
|  | Per-hand       |     | - Float16/Int8 |     | - Indexed by     |   |
|  | action probs   |     | - Sparse repr  |     |   format+config  |   |
|  | at every node  |     | - Tree pruning |     | - CDN-cached     |   |
|  +----------------+     +----------------+     +------------------+   |
|                                                                        |
+-----------------------------------------------------------------------+
```

### 5.2 Storage Optimization

Each solved tree stores action probabilities for ~1,000+ hand combos at every decision node. Optimization strategies include:

| Technique | Reduction | Notes |
|-----------|-----------|-------|
| **Quantization** | 2--4x | Reduce float64 to float16 or int8 |
| **Sparse encoding** | Variable | Omit near-zero frequency actions |
| **Suit isomorphism** | ~4x preflop | Store canonical suits only |
| **Delta encoding** | Variable | Store differences from heuristic baseline |
| **Tree trimming** | Variable | Remove unreachable nodes |

### 5.3 Caching Layers

The platform employs multiple caching layers:

1. **CDN edge cache**: Static solution data for popular spots (likely Cloudflare or AWS CloudFront)
2. **Application cache**: In-memory (Redis/Memcached) for recently accessed solutions
3. **Database query cache**: For user-specific queries (hand history lookups, saved analyses)
4. **Client-side cache**: Browser IndexedDB or memory for the current session

---

## 6. Real-Time Solving Pipeline for GTO Wizard AI

### 6.1 End-to-End Flow

```
User clicks           API Gateway          Solve                Neural Net         CFR
"Custom Solve"        routes request       Orchestrator         Inference          Engine
     |                     |                    |                    |                |
     | POST /solve/custom  |                    |                    |                |
     |------------------->|                    |                    |                |
     |                     | validate + debit  |                    |                |
     |                     | power credits     |                    |                |
     |                     |------------------->|                    |                |
     |                     |                    | build game tree    |                |
     |                     |                    | for street 1       |                |
     |                     |                    |------------------->|                |
     |                     |                    |  leaf values       |                |
     |                     |                    |<-------------------|                |
     |                     |                    |                    |                |
     |                     |                    |----------------------------------->|
     |                     |                    |   run CFR on street 1              |
     |                     |                    |<-----------------------------------|
     |                     |                    |                    |                |
     |                     |                    | (repeat for each street)           |
     |                     |                    |                    |                |
     |                     | solution data      |                    |                |
     |<-----------------------------------------|                    |                |
     |                     |                    |                    |                |
     | Render strategy     |                    |                    |                |
```

### 6.2 Latency Budget

For a typical custom solve:

| Phase | Estimated Time |
|-------|---------------|
| API validation + credit debit | ~50 ms |
| Tree construction | ~200 ms |
| Neural network inference (per street) | ~500 ms |
| CFR convergence (per street) | ~2,000 ms |
| Response serialization + transfer | ~250 ms |
| **Total per street** | **~3,000 ms** |

For a full flop-through-river solve: approximately 9--12 seconds total.

### 6.3 Fast Mode Pipeline

Fast Mode uses dynamic-depth-limited solving:

```
+-------------------+      +-------------------+      +-------------------+
|  Current Action   | ---> |  Small Lookahead  | ---> |  Resolve          |
|  in game tree     |      |  Construction     |      |  Recursively      |
|                   |      |                   |      |                   |
|  Player acts      |      |  Build limited    |      |  As new actions   |
|  at a node        |      |  subtree from     |      |  occur, build     |
|                   |      |  current state    |      |  new lookaheads   |
+-------------------+      +-------------------+      +-------------------+
```

This avoids solving complete streets upfront, enabling faster response for interactive use cases like Practice Mode.

---

## 7. User Account and Subscription Management

### 7.1 Subscription Tiers

| Tier | Price (Monthly) | Key Features |
|------|-----------------|--------------|
| **Free** | $0 | Limited precomputed solutions |
| **Premium** | ~$35 | Full precomputed solution access |
| **Elite** | ~$129--149 | GTO Wizard AI custom solving, advanced reports |

### 7.2 Power Credits System

Power Credits are a metered-compute currency:

- Purchased in bundles (separate from subscription)
- Consumed by compute-intensive operations: custom solves, aggregated reports, advanced analysis
- Credit cost scales with computational complexity (tree size, number of streets, multiway)
- Credits expire one year after Elite subscription ends

### 7.3 Account Data Model

```
User
  |-- Subscription (tier, billing cycle, status)
  |-- Power Credit Balance (current, history)
  |-- Hand History Library (uploaded hands, analysis results)
  |-- Study Progress (completed drills, accuracy stats)
  |-- Saved Analyses (bookmarked positions, custom solves)
  |-- Training Stats (practice mode results, EV tracking)
```

---

## 8. Hand History Processing Pipeline

### 8.1 Pipeline Architecture

```
+-------------------+      +-------------------+      +-------------------+
|  Upload / Import  | ---> |  Parser           | ---> |  Analysis Engine  |
|                   |      |                   |      |                   |
|  - File upload    |      |  - Multi-format   |      |  - Match to pre-  |
|  - Direct import  |      |    (PokerStars,   |      |    computed sims  |
|  - Poker site     |      |    GGPoker, etc.) |      |  - Compute EV     |
|    integration    |      |  - Normalize to   |      |    loss per action |
|                   |      |    internal format |      |  - Generate       |
+-------------------+      +-------------------+      |    reports        |
                                                       +-------------------+
                                                               |
                                                               v
                                                       +-------------------+
                                                       |  Results Store    |
                                                       |                   |
                                                       |  - Per-hand EV    |
                                                       |  - Aggregate      |
                                                       |    statistics     |
                                                       |  - Mistake        |
                                                       |    classification |
                                                       +-------------------+
```

### 8.2 Analysis Process

1. **Parse** hand histories from various poker site formats into a normalized internal representation.
2. **Match** each decision point against the closest precomputed solution or invoke GTO Wizard AI for custom analysis.
3. **Compute** EV loss: the difference between the player's action's EV and the GTO-optimal action's EV.
4. **Classify** mistakes by type (pure mistakes, mixing errors, sizing errors) and severity.
5. **Aggregate** results into reports: overall bb/100 EV loss, worst leaks by position/street/action, and trend analysis.

### 8.3 HH Analyzer 2.0

The latest version of the Hand History Analyzer provides:
- Per-hand EV loss measurements
- Aggregate reports by position, street, and action type
- Custom filtering and grouping
- Integration with Power Credits for advanced analysis

---

## 9. CDN and Delivery Architecture

### 9.1 Content Distribution

```
+-------------------+      +-------------------+      +-------------------+
|  Origin Servers   | ---> |  CDN Edge Nodes   | ---> |  End Users        |
|                   |      |  (Global PoPs)    |      |  (Worldwide)      |
|  - API servers    |      |                   |      |                   |
|  - Static assets  |      |  - JS/CSS/images  |      |  - Low latency    |
|  - Solution data  |      |  - Cached API     |      |  - Fast loads     |
|                   |      |    responses       |      |                   |
+-------------------+      +-------------------+      +-------------------+
```

### 9.2 Optimization Strategies

| Layer | Technique | Impact |
|-------|-----------|--------|
| **Network** | CDN with global PoPs | Sub-100ms static asset delivery |
| **Application** | Gzip/Brotli compression | 60--80% transfer size reduction |
| **Data** | Solution data compression | Minimize bandwidth for large trees |
| **Frontend** | Code splitting, lazy loading | Fast initial page load |
| **API** | Response caching (ETags, Cache-Control) | Reduce origin load |
| **Protocol** | HTTP/2 or HTTP/3 | Multiplexed connections |

---

## 10. Infrastructure and Deployment

### 10.1 Inferred Cloud Architecture

Based on the platform's global reach, real-time solving requirements, and scaling needs:

```
+-----------------------------------------------------------------------+
|                    CLOUD INFRASTRUCTURE                                 |
|                                                                        |
|  +------------------+    +------------------+    +------------------+  |
|  | Web / API        |    | Solver Compute   |    | Data Stores      |  |
|  | Tier             |    | Tier             |    |                  |  |
|  |                  |    |                  |    | - Solution DB    |  |
|  | - Auto-scaling   |    | - GPU instances  |    | - User DB        |  |
|  |   web servers    |    |   (NN inference) |    | - Cache (Redis)  |  |
|  | - Load balanced  |    | - CPU instances  |    | - Object Storage |  |
|  | - Container-     |    |   (CFR solving)  |    | - Analytics DB   |  |
|  |   ized (K8s)     |    | - Job queue      |    |                  |  |
|  |                  |    |   (Celery/RQ)    |    |                  |  |
|  +------------------+    +------------------+    +------------------+  |
|                                                                        |
|  +------------------+    +------------------+    +------------------+  |
|  | Background       |    | Monitoring       |    | CI/CD Pipeline   |  |
|  | Workers          |    |                  |    |                  |  |
|  |                  |    | - APM            |    | - GitHub Actions  |  |
|  | - HH processing  |    | - Logging        |    | - Automated      |  |
|  | - Report gen     |    | - Alerting       |    |   testing        |  |
|  | - Batch solves   |    | - Usage metrics  |    | - Staged deploy  |  |
|  +------------------+    +------------------+    +------------------+  |
|                                                                        |
+-----------------------------------------------------------------------+
```

### 10.2 Scaling Considerations

| Dimension | Approach |
|-----------|----------|
| **Web traffic** | Horizontal auto-scaling behind load balancer |
| **Custom solves** | Worker pool with job queue; scale by demand |
| **Solution storage** | Append-only; grow with new formats/configs |
| **Global latency** | CDN edge caching; regional API replicas if needed |
| **Peak load** | Power credit system naturally throttles demand |

### 10.3 Reliability

- Solution data is immutable after computation, simplifying backup and replication
- Custom solve workers are stateless; failed jobs can be retried transparently
- User data (accounts, hand histories, progress) backed by transactional database with standard HA patterns

---

## 11. Security Considerations

| Concern | Mitigation |
|---------|------------|
| **Solution piracy** | Solutions served via authenticated API, not downloadable bulk files |
| **Account security** | Standard OAuth/session auth; subscription enforcement server-side |
| **Solve abuse** | Power credit metering prevents unlimited compute consumption |
| **Data privacy** | Hand histories contain sensitive gameplay data; encrypted at rest |
| **Bot prevention** | Rate limiting and behavioral analysis on API endpoints |

---

## 12. Technology Summary

| Component | Inferred Technology |
|-----------|-------------------|
| **Frontend** | React or Next.js, TypeScript |
| **Backend API** | Python (Django/FastAPI) |
| **Solver Engine** | C++ (hand evaluator, CFR core), Python (orchestration) |
| **Neural Networks** | PyTorch or TensorFlow (training), ONNX or TensorRT (inference) |
| **Task Queue** | Celery, RQ, or cloud-native equivalent |
| **Databases** | PostgreSQL (users), Redis (cache), Object Store (solutions) |
| **CDN** | Cloudflare, AWS CloudFront, or equivalent |
| **Container Orchestration** | Kubernetes or cloud-native container service |
| **CI/CD** | GitHub Actions (public GitHub org: gtowizard-ai) |
| **Monitoring** | Standard APM + custom solver performance metrics |

---

## 13. Feature-to-Architecture Component Mapping

This section maps each major user-facing feature to the architectural components that support it, providing a cross-reference between the feature documentation and the technical infrastructure described above.

### 13.1 Study Mode ([Doc 03](03_study_mode.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Strategy Matrix rendering (13x13 grid, color-coded actions) | Frontend: Range Grid Viewer (Canvas/SVG), Client-side state management |
| Instant solution loading | Solution Store (Object/Blob) -> CDN Edge Cache -> Client-side cache |
| 1,755 flop aggregate data (Reports tab) | Precomputed Solution Database, indexed by format + config hash |
| Filtering (hand class, suit, equity buckets) | Client-side computation via Web Workers (filtering pre-loaded strategy arrays) |
| Navigation across game tree nodes | Backend: Solution Lookup Service (REST API), Frontend: Game Tree Navigator |
| Metric overlays (EV, EQ, EQR, Compare EV) | Pre-stored per-node metric data in compressed solution files |

### 13.2 Practice Mode ([Doc 04](04_practice_mode.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| GTO opponent play (solver-accurate responses) | Solution Lookup Service for precomputed spots; GTO Wizard AI for custom drills |
| Real-time scoring (GTOW Score, EV loss) | Client-side computation comparing player action to solution data |
| Multi-table support (1-4 simultaneous tables) | Frontend: Multi-instance table rendering, independent state per table |
| RNG system for mixed strategies | Client-side random number generation, mapped to solution frequency distributions |
| Session statistics and history | User Database (PostgreSQL): Study Progress, Training Stats |
| Drill configuration and saving | Backend: User account data model (saved analyses, study progress) |

### 13.3 Analyze Mode ([Doc 05](05_analyze_mode.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Hand history upload (17+ sites) | HH Processing Pipeline: Upload/Import -> Multi-format Parser |
| Multi-format parsing (PokerStars, GGPoker, etc.) | Parser service: normalizes site-specific formats to internal representation |
| Decision comparison vs GTO | Analysis Engine: matches each action to closest precomputed solution |
| EV loss calculation | Analysis Engine: computes EV(GTO) - EV(Player's action) per decision |
| Blunder/Mistake/Inaccuracy classification | Analysis Engine: frequency threshold (3.5%) and EV loss severity |
| Analyzer 2.0 filtering and reports | User Database (Results Store): per-hand EV, aggregate statistics, custom saved reports |
| Table Wizard auto-upload | Table Wizard (Windows native) -> HH Processing Pipeline (direct import) |

### 13.4 GTO Wizard AI Custom Solving ([Doc 06](06_gto_wizard_ai.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Custom solve request | API Gateway -> Custom Solve Orchestrator -> Power Credit debit |
| Neural network value prediction | GTO Wizard AI Solver: NN inference (GPU/CPU), trained models |
| Single-street CFR solving | GTO Wizard AI Solver: CPU-based CFR with NN leaf values |
| Dynamic sizing algorithm | Solve Orchestrator: iterative size optimization at each node |
| Classic Mode vs Fast Mode | Classic: full street expansion + CFR; Fast: Dynamic-Depth-Limited Solving (DDLS) |
| 3-way solving | Solver: player reduction (max 3 to flop), expanded game trees |
| Range editor (paint, import, export) | Frontend: Range Grid Viewer with editing capabilities, Backend: range serialization |
| Solution saving and tagging | User Database: Saved Analyses with tag metadata |

### 13.5 Nodelocking ([Doc 07](07_nodelocking.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Strategy locking at specific nodes | Custom Solve Orchestrator: constrained CFR (fixed strategy nodes) |
| Re-equilibration (exploitative response) | GTO Wizard AI Solver: re-runs CFR with locked constraints |
| Compare Nodes (before/after) | Frontend: dual-panel rendering of two solution states |
| Frequency adjustment with EV-based hand selection | Solver: sorts hands by EV loss, reassigns actions optimally |

### 13.6 Aggregated Reports ([Doc 08](08_aggregated_reports.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Standard reports (precomputed library) | Solution Store: pre-aggregated data across 1,755 flops, CDN-cached |
| Custom reports (GTO Wizard AI) | Solve Orchestrator: batch solves across all 1,755 flops, Report Engine |
| Power Credits consumption | Subscription/Billing Service: credit formula Max(175, 10 x sqrt(Nodes)) |
| Board texture filtering and grouping | Frontend: client-side filtering of aggregate data arrays |
| Chart and Table display formats | Frontend: charting library (D3.js/Recharts) for visual reports |

### 13.7 Tournament/ICM ([Doc 09](09_tournament_icm.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| ICM equity calculation | Solver Engine: combinatorial probability model for chip-to-equity conversion |
| Bubble factor computation | Derived from ICM model: ratio of equity lost (bust) vs equity gained (double) |
| 50,000+ final table simulations | Precomputed Solution Database: indexed by player count, stack distribution, payout structure |
| PKO bounty modeling | Solver: three-component EV model (immediate KO, future bounty, ICM prize) |
| Custom ICM solving | GTO Wizard AI Solver: ICM-aware value function in neural network |
| Payout structure input (up to 4,096 players) | Backend: payout structure parser, tournament configuration service |

### 13.8 Range Builder ([Doc 10](10_range_builder.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| 13x13 hand grid with paintbrush | Frontend: Range Grid Viewer with edit mode (Canvas interaction) |
| Suit-specific strategy assignment | Frontend: Hand Matrix panel with per-combo painting |
| Grading vs GTO (0-100% score) | Client-side: comparison of user-assigned frequencies vs solution frequencies |
| Integration with Study Mode and Practice Mode | Frontend: cross-navigation via shared spot identifiers |

### 13.9 PokerArena ([Doc 11](11_ui_ux_screens.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Real-time heads-up poker games | WebSocket Proxy: low-latency bidirectional communication |
| Matchmaking and rating system | Backend: matchmaking service with Elo-style rating |
| Seasonal leaderboards | User Database: aggregated match results, periodic resets |
| Cross-platform play (web, iOS) | API Gateway: shared game state across clients |
| Post-game GTO analysis | Integration: PokerArena hand data -> Study Mode / Analyze Mode pipeline |

### 13.10 Table Wizard ([Doc 11](11_ui_ux_screens.md))

| Feature Aspect | Architectural Component |
|---------------|------------------------|
| Multi-table management | Windows native application: screen capture + overlay rendering |
| Real-time bet/raise/call overlays | Client-side: game state detection + UI overlay engine |
| Automatic hand history upload | Integration: file watcher -> HH Processing Pipeline (auto-upload) |
| Custom hotkeys and bet sliders | Windows native: keyboard hook + configurable action mapping |

---

## 14. Cross-Reference: Technical Details from Feature Documentation

This section consolidates specific technical facts mentioned across the feature documentation that inform the architectural understanding.

### Solving Performance

- Precomputed solutions target **0.1--0.3% Nash Distance** ([Doc 02](02_solver_engine.md), Section 7.2)
- GTO Wizard AI averages **0.12% Nash Distance** on flop solutions after the QRE upgrade ([Doc 02](02_solver_engine.md), Section 7.6)
- A benchmark position (A-7-3 rainbow, 100bb HU SRP, 95 GB tree) solves in **6 seconds** vs PioSolver's **4,862 seconds** ([Doc 02](02_solver_engine.md), Section 7.4)
- 3-way river solutions achieve **0.1% Nash Distance** in seconds ([Doc 02](02_solver_engine.md), Section 10.3)
- Dynamic sizing captures **99.95% of available river EV** ([Doc 02](02_solver_engine.md), Section 4.3)

### Data Scale

- **10M+ presolved spots** in the solution library ([Doc 12](12_game_formats_and_pricing.md), Section 2)
- **1,755 strategically distinct flops** after suit isomorphism reduction ([Doc 02](02_solver_engine.md), Section 5)
- **~622,000 preflop nodes** for a 6-max 100bb tree with 3 bet sizes ([Doc 02](02_solver_engine.md), Section 4.2)
- A single complex postflop tree can contain **87+ million nodes** ([Doc 02](02_solver_engine.md), Section 4.2)
- **50,000+ ICM final table simulations** for 200-player tournaments ([Doc 09](09_tournament_icm.md))
- **17+ poker site formats** supported for hand history import ([Doc 05](05_analyze_mode.md))

### Resource Requirements

- Traditional CFR: **16+ cores at 5+ GHz, 128+ GB RAM** for comparable accuracy ([Doc 02](02_solver_engine.md), Section 3.3)
- GTO Wizard AI: **2 CPU cores, 8 GB RAM** per solve ([Doc 02](02_solver_engine.md), Section 3.3)
- Hand evaluator: **400M evaluations/sec** on single thread, C++ implementation ([Doc 14](14_architecture_blueprint.md), Section 4.3)
- Power Credits formula: **Max(175, 10 x sqrt(Nodes))** per custom aggregated report ([Doc 08](08_aggregated_reports.md))

### Neural Network Training

- Trained through **self-play** over **hundreds of millions of hands** ([Doc 02](02_solver_engine.md), Section 3.2)
- Uses **Deep CFR** paradigm with counterfactual reasoning ([Doc 02](02_solver_engine.md), Section 3.2)
- Ruse AI engine developed at **Mila** (Quebec AI research laboratory) ([Doc 06](06_gto_wizard_ai.md))
- Academic lineage: DeepStack (2017) -> Libratus (2017) -> Pluribus (2019) -> Ruse AI (2022) -> GTO Wizard AI (2023+) ([Doc 02](02_solver_engine.md), Section 9.3)

---

## Sources

- https://blog.gtowizard.com/gto-wizard-ai-explained/
- https://blog.gtowizard.com/gto-wizard-ai-benchmarks/
- https://blog.gtowizard.com/gto-wizard-ai/
- https://blog.gtowizard.com/gto-wizard-ai-custom-multiway-solving/
- https://blog.gtowizard.com/pioneering-poker-ai-research/
- https://blog.gtowizard.com/how-solvers-work/
- https://blog.gtowizard.com/crushing-a-top-hunl-poker-bot/
- https://blog.gtowizard.com/ai-and-the-future-of-poker/
- https://github.com/gtowizard-ai (GTO Wizard AI GitHub organization)
- https://github.com/gtowizard-ai/mitpoker-2024 (MIT poker competition entry)
- https://rocketreach.co/viktor-stiskala-email_4714035 (CTO technology background)
- https://test.gtowizard.com/accuracy-and-benchmarks/
- https://help.gtowizard.com/subscription/
- https://blog.gtowizard.com/introducing_custom_aggregated_reports/

**Note**: This architecture document is based on publicly available information and reasonable inference. GTO Wizard has not published its full architecture. Specific technology choices (framework versions, cloud providers, database engines) are inferred from observable behavior, team backgrounds, and industry standards, and may differ from the actual implementation.

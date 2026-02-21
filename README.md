# GTO Wizard Research Package

A comprehensive reverse-engineering and research study of **GTO Wizard**, the world's leading poker training and strategy platform. This package documents the product's features, technology, architecture, competitive position, and user experience based on publicly available information including blog posts, help documentation, external reviews, and performance benchmarks.

**Research date**: February 2026

**Caveats**: This research is based entirely on publicly available information. Architecture details are inferred from observable behavior, stated performance characteristics, team backgrounds, and industry norms. No proprietary code, private APIs, or internal documents were accessed.

---

## Table of Contents

| # | Document | Description |
|---|----------|-------------|
| 01 | [Product Overview](docs/01_product_overview.md) | Executive summary of GTO Wizard: company background, product evolution timeline, market position, core value proposition, key differentiators, technology foundation, feature suite overview, and target user segments. The starting point for understanding the entire research package. |
| 02 | [Solver Engine](docs/02_solver_engine.md) | Technical deep dive into the solving algorithms: Counterfactual Regret Minimization (CFR) and its variants (MCCFR, Linear CFR, DCFR, CFR+), neural network architecture for strategy approximation, game tree construction and sizes, the 1,755 strategically distinct flop subsets, card abstraction and bucketing techniques, Nash Distance metrics and accuracy benchmarks, and the April 2025 Quantal Response Equilibrium (QRE) engine upgrade. |
| 03 | [Study Mode](docs/03_study_mode.md) | The primary analytical workspace: Strategy tab (13x13 hand grid, color coding, metric overlays), Ranges tab (side-by-side equity distributions), Breakdown tab (hand class composition of each action), Reports tab (aggregate data across 1,755 flops), filtering system, keyboard shortcuts, and component specifications. |
| 04 | [Practice Mode](docs/04_practice_mode.md) | Interactive training environment: three game modes (Full Hand, Street, Spot), three difficulty tiers, multitabling (1-4 tables), GTOW scoring system (-100% to +100%), RNG for mixed strategies, drill configuration, session statistics, the 5 Levels of Trainer Mastery framework, and recommended drill configurations. |
| 05 | [Analyze Mode](docs/05_analyze_mode.md) | Hand history analysis tool: upload process (17+ supported poker sites), processing pipeline, blunder/mistake/inaccuracy detection, EV loss calculation methodology, session review filtering (Analyzer 2.0), hand replay interface, performance analytics dashboard, and integration with Study Mode. |
| 06 | [GTO Wizard AI](docs/06_gto_wizard_ai.md) | Real-time custom solving engine: neural network approach vs. traditional CFR, speed/accuracy benchmarks, configurable parameters (stacks, ranges, bet sizing modes), dynamic sizing algorithm, multiway solving (3-way), credits and cost system, and saving/organization features. |
| 07 | [Nodelocking](docs/07_nodelocking.md) | Exploitative strategy adjustment: 3-tab interface (Set Strategy, Set Frequency, Lock/Unlock), solver re-equilibration process, cascade effects, Compare Nodes feature, practical applications (population tendencies, opponent modeling), Nodelocking 2.0 improvements, and limitations. |
| 08 | [Aggregated Reports](docs/08_aggregated_reports.md) | Macro-level strategy analysis across all 1,755 flop subsets: standard vs. custom reports, Power Credits system and cost formula, report types (Strategy, EV, EQ, EQR), filtering and segmentation options, board texture analysis, and practical workflow examples. |
| 09 | [Tournament / ICM](docs/09_tournament_icm.md) | Tournament-specific strategy: ICM model and calculations, bubble factor analysis, PKO progressive knockout mechanics (bounty power, adjusted pot odds, ICM Dial), standard KO, mystery bounty, satellite strategy, 50,000+ final table simulations, payout structure configurations, and differences from cash game GTO. |
| 10 | [Range Builder](docs/10_range_builder.md) | Active learning range construction tool: 13x13 hand grid interface, paintbrush and selection methods, suit-specific strategies (hand pinning), weight assignment controls, combo locking, filters (suit, hand category, equity), grading system (0-100%), reference features (lightbulb, opponent range viewer), and integration with Study/Practice modes. |
| 11 | [UI/UX Screens](docs/11_ui_ux_screens.md) | Comprehensive visual reference: design system overview (color coding, themes, layouts), ASCII layout diagrams for every major screen (Dashboard, Study Mode tabs, Practice Mode, Analyze Mode, Range Builder, GTO Wizard AI, Nodelocking, PokerArena, Table Wizard, Settings), component specifications, state change documentation, and responsive design notes. |
| 12 | [Game Formats & Pricing](docs/12_game_formats_and_pricing.md) | Supported game formats (Cash, Straddle+Ante, MTT, Spin & Go, HU SNG, 3-Way), solution library statistics, pricing tiers (Free/Starter/Premium/Elite), format-specific pricing, Power Credits system, platform availability (Web, Mobile, Desktop), supported poker sites for hand analysis, content resources, and feature timeline. |
| 13 | [Competitive Landscape](docs/13_competitive_landscape.md) | Market analysis: competitor profiles (PioSolver, GTO+, MonkerSolver, TexasSolver, SimplePostflop, Jesolver, PokerSnowie, Deepsolver, GTOBase), comprehensive feature matrices, pricing comparison over time, market position analysis, SWOT analysis, competitive threat assessment, and market trends (cloud vs. desktop, AI integration, training over solving). |
| 14 | [Architecture Blueprint](docs/14_architecture_blueprint.md) | Inferred system architecture: high-level diagram, frontend architecture (React/Next.js), backend API layer (Python/Django), solver compute infrastructure (precomputed CFR pipeline, GTO Wizard AI pipeline), solution database and caching, real-time solving pipeline, user/subscription management, HH processing pipeline, CDN delivery, infrastructure/deployment, security considerations, feature-to-architecture component mapping. |
| 15 | [Glossary](docs/15_glossary.md) | Comprehensive reference of 100+ terms: poker fundamentals (positions, actions, hand types, board textures, game formats, metrics), game theory and solver terminology (Nash Equilibrium, GTO, exploitative strategy, QRE, convergence, Nash Distance), technical/algorithmic terms (CFR variants, abstraction methods, neural network concepts, poker AI systems), and GTO Wizard-specific terms (features, subscription tiers, analysis concepts, mathematical notation). |

**Source Index**: [sources/source_index.md](sources/source_index.md) -- Compiled index of all URLs referenced across all documents, organized by document and by source type.

---

## Cross-Reference Map

The documents are interconnected. Here is how they relate:

```
                    +-------------------+
                    | 01 Product        |
                    |    Overview       |
                    | (Executive        |
                    |  Summary)         |
                    +--------+----------+
                             |
              +--------------+--------------+
              |              |              |
     +--------v----+  +-----v------+  +----v--------+
     | TECHNOLOGY  |  | FEATURES   |  | BUSINESS    |
     |             |  |            |  |             |
     | 02 Solver   |  | 03 Study   |  | 12 Formats  |
     |    Engine   |  | 04 Practice|  |    & Pricing|
     | 14 Arch.    |  | 05 Analyze |  | 13 Compet.  |
     |    Blueprint|  | 06 AI      |  |    Landscape|
     | 15 Glossary |  | 07 Nodelock|  |             |
     |             |  | 08 Reports |  |             |
     |             |  | 09 Tourney |  |             |
     |             |  | 10 Range   |  |             |
     +-------------+  | 11 UI/UX   |  +-------------+
                       +------------+
```

Key cross-document relationships:
- **Doc 02 (Solver Engine)** underpins Docs 06 (AI), 08 (Reports), 09 (ICM), and 14 (Architecture)
- **Doc 06 (GTO Wizard AI)** is the engine behind Docs 07 (Nodelocking), 08 (Custom Reports), and 09 (Custom ICM)
- **Doc 11 (UI/UX)** provides visual references for Docs 03-10 (all features)
- **Doc 12 (Pricing)** cross-references Docs 13 (Competition) and 11 (Platforms)
- **Doc 13 (Competition)** compares features documented in Docs 03-10 against competitors
- **Doc 14 (Architecture)** maps components to features in Docs 03-10
- **Doc 15 (Glossary)** defines terminology used throughout all documents

---

## Suggested Reading Order

### For Developers / Engineers
1. **01 Product Overview** -- Understand the product scope
2. **02 Solver Engine** -- Core algorithms and technical foundations
3. **14 Architecture Blueprint** -- System design and infrastructure
4. **06 GTO Wizard AI** -- Neural network engine details
5. **15 Glossary** -- Reference for unfamiliar poker/solver terminology
6. Remaining docs as needed for specific feature understanding

### For Poker Players / Students
1. **01 Product Overview** -- What the product does and why it matters
2. **03 Study Mode** -- Primary study interface
3. **04 Practice Mode** -- Training system
4. **05 Analyze Mode** -- Hand review workflow
5. **09 Tournament / ICM** -- Tournament-specific strategy (if applicable)
6. **07 Nodelocking** -- Exploitative adjustments
7. **12 Game Formats & Pricing** -- What you get at each tier

### For Product Managers / Business Analysts
1. **01 Product Overview** -- Executive summary with market context
2. **13 Competitive Landscape** -- Market position, SWOT, competitor analysis
3. **12 Game Formats & Pricing** -- Business model and pricing strategy
4. **11 UI/UX Screens** -- User experience and design system
5. **04 Practice Mode** + **05 Analyze Mode** -- Core engagement loops
6. **02 Solver Engine** -- Technology differentiators at a high level

---

## Research Methodology

This research was conducted through systematic analysis of:
- **GTO Wizard blog posts** (blog.gtowizard.com) -- Feature announcements, technical explanations, strategy articles
- **GTO Wizard help center** (help.gtowizard.com) -- Feature documentation, guides, FAQs
- **GTO Wizard official site** (gtowizard.com) -- Product pages, pricing, landing pages
- **Third-party reviews** (solvers.poker, hand2noteguide.com, hudstore.poker, gipsyteam.com) -- Independent evaluations
- **Academic papers** -- Original CFR paper (Zinkevich 2007), MCCFR (Lanctot 2009), DCFR (Brown & Sandholm 2019), Pluribus (Science 2019)
- **GitHub** (github.com/gtowizard-ai) -- Public repositories and MIT poker competition entry
- **App Store listings** -- iOS applications (PokerArena, GTO Preflop Wizard)

All sources are documented in the individual document Sources sections and compiled in the [Source Index](sources/source_index.md).

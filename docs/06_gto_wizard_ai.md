# GTO Wizard AI: Custom Solving and Neural Network Engine

## Overview

GTO Wizard AI is a real-time poker solver powered by neural networks, developed in partnership with Ruse AI (a team from Mila, Quebec's premier AI research laboratory specializing in multi-agent reinforcement learning). Unlike GTO Wizard's library of precomputed solutions -- which are static, pre-solved strategy databases that cannot be modified -- GTO Wizard AI generates fresh game-theory optimal strategies on demand for any configuration of stack depths, bet sizes, ranges, and pot sizes. This distinction is fundamental: precomputed solutions cover common spots with fixed parameters, while GTO Wizard AI can solve novel, unusual, or highly specific scenarios in seconds.

The feature launched as an Elite Tier exclusive at $149/month ($129/month annual), with an early-bird discount of $109/month. All users can freely solve one demonstration flop -- Q-spade T-spade 7-heart -- to evaluate the technology.

---

## How GTO Wizard AI Differs from Traditional Solvers

### Traditional CFR-Based Solvers

Conventional solvers such as PioSolver use Counterfactual Regret Minimization (CFR). This algorithm explores every possible decision point in the game, for every hand combination, to complete a single iteration. The solver must compute through all future streets simultaneously, creating computational bottlenecks. A single solve can take minutes to hours depending on tree complexity, and the user must manually configure every bet size and parameter.

### GTO Wizard AI's Neural Network Approach

GTO Wizard AI fundamentally differs by analyzing **one street at a time**. The system uses expected values learned from its neural networks to eliminate the need for computing future streets. These learned values serve as strategic guidance for immediate decisions, enabling dramatically faster solving.

The AI was trained through **self-play**: hundreds of millions of hands played against progressively stronger versions of itself. This methodology develops generalized "intuition" for hand values across diverse scenarios. The training uses **counterfactual reasoning** -- after each hand, the network reviews previous decisions and identifies higher-value actions.

```
+-------------------------------------------------------------------+
|              TRADITIONAL SOLVER (CFR)                              |
|                                                                   |
|   Flop ──> Turn ──> River ──> Terminal                            |
|     |         |        |        |                                 |
|     v         v        v        v                                 |
|   [Solve all streets simultaneously]                              |
|   [Iterate until convergence]                                     |
|   Time: Minutes to Hours                                          |
+-------------------------------------------------------------------+

+-------------------------------------------------------------------+
|              GTO WIZARD AI (Neural Network)                        |
|                                                                   |
|   Flop ──> Neural Net EV ──> Strategy                             |
|   Turn ──> Neural Net EV ──> Strategy                             |
|   River ──> Neural Net EV ──> Strategy                            |
|     |                                                             |
|     v                                                             |
|   [Solve one street at a time]                                    |
|   [Use learned values for future streets]                         |
|   Time: ~3 seconds per street                                     |
+-------------------------------------------------------------------+
```

Key architectural advantages:
- **No abstractions**: The solver computes each custom spot precisely rather than mapping it to pre-computed simplified versions, preventing the anomalies that abstraction-based solvers produce.
- **No blueprint strategies**: Even for unusual custom spots, the results are computed from scratch.
- **Generalization across stack depths**: The neural network generalizes without re-solving, handling any stack depth up to 200 big blinds.

---

## Speed and Accuracy Benchmarks

### Speed

GTO Wizard AI generates optimal strategies for games of up to 200 big blinds with any bet size variation in an **average of 3 seconds per street**. This is orders of magnitude faster than traditional CFR solvers.

### Competitive Validation

The AI defeated Slumbot, an Annual Computer Poker Competition (ACPC) champion bot, at a rate of **19.4 bb/100 over 150,000 hands** of heads-up no-limit hold'em. This winrate demonstrates strong strategic play against a well-established benchmark.

### Nash Distance

When benchmarked against PioSolver through nodelocking, GTO Wizard AI solutions averaged a Nash Distance of only **0.12% pot** on flop solutions (some reports cite 0.21% pot when nodelocking flop solutions against PioSolver). Nash Distance measures how far a strategy deviates from perfect equilibrium -- these figures indicate high convergence to game-theoretic optimality.

For 3-way solving, turn solutions average **0.24% +/- 0.08%** Nash Distance, and river solutions consistently stay below **0.10%**.

| Metric                    | Value                           |
|---------------------------|---------------------------------|
| Average solve time        | ~3 seconds per street           |
| vs. Slumbot winrate       | 19.4 bb/100 over 150k hands    |
| Nash Distance (flop)      | ~0.12-0.21% pot vs. PioSolver  |
| Nash Distance (3-way turn)| 0.24% +/- 0.08%                |
| Nash Distance (river)     | < 0.10%                         |
| Max stack depth           | 200 big blinds                  |
| Max stack-to-pot ratio    | 100:1                           |

---

## Configurable Parameters

### Stack and Pot Configuration

- **Stack sizes**: Adjustable in 0.1bb increments, maximum 999bb
- **Pot size**: Adjustable in 0.1bb increments, maximum 999bb
- **Stack-to-pot ratio**: Limited to a maximum of 100 to ensure solution accuracy
- **Starting street**: Currently postflop only (preflop solving on roadmap)

### Range Configuration

Players can modify starting hand ranges through:
- Manual painting with mouse tools on the hand matrix
- Linear scaling via "From/To" input boxes
- Weight adjustment for individual hand frequency (0-100%)
- Import/export functionality for sharing ranges across tools
- Saving custom ranges with tags for future use
- PREFILL button to import ranges from pre-solved solutions

### Betting Tree Configuration

Three sizing modes are available:

```
+---------------------------------------------------------------+
|                   BET SIZING MODES                             |
+---------------------------------------------------------------+
|                                                                |
|  AUTOMATIC                                                     |
|  +----------------------------------------------------------+ |
|  | GTO Wizard selects highest-EV size(s) at each node       | |
|  | Best for: Beginners, quick analysis                       | |
|  +----------------------------------------------------------+ |
|                                                                |
|  DYNAMIC                                                       |
|  +----------------------------------------------------------+ |
|  | User chooses HOW MANY sizes; AI optimizes which ones      | |
|  | Option: "Specify Sizes to Compare" for bounded selection  | |
|  | Best for: Balanced simplification                         | |
|  +----------------------------------------------------------+ |
|                                                                |
|  FIXED                                                         |
|  +----------------------------------------------------------+ |
|  | User specifies exact bet/raise sizes manually             | |
|  | Similar to traditional solver inputs                       | |
|  | Best for: Precise tree control                            | |
|  +----------------------------------------------------------+ |
+---------------------------------------------------------------+
```

**Bet sizing input formats:**
- Percentage of pot (e.g., 33%, 67%, 150%)
- Geometric sizing (`e`) -- automatically adjusts to stack depth
- Multiple of previous bet (`x`)
- Big blinds (`bb`)

### Advanced Tree Options

- **Always add all-in**: Forces shove availability regardless of other sizing parameters
- **Force all-in threshold**: Converts oversized bets to all-ins (e.g., setting 80% converts any bet larger than 40bb into a shove when the pot warrants it)
- **Add all-in threshold**: Minimum pot percentage before considering shove options
- **Bet size merging**: Eliminates redundant sizings using the formula: `X% > (1 + Higher) / (1 + Lower) - 1`. Recommended threshold: 5-20% (12% balances simplification without over-reducing diversity)
- **Per-street/per-player customization**: Toggle switches to create separate betting trees for IP vs. OOP players, and for flop, turn, and river independently

### Node/Tree Editing Post-Solve

After a solution is generated, users can access node editing via the "Edit Node" pencil icon to:
- Add or remove specific bet sizes at any decision point
- Delete check or fold options
- Change input types for individual actions
- Then re-solve to see the impact of changes

---

## Dynamic Sizing

Dynamic sizing is one of GTO Wizard AI's most distinctive features. When selected, you choose how many bet or raise sizes you want (e.g., 1, 2, or 3 sizes), and the AI automatically simplifies to the optimal, highest-EV sizings at each decision point. This means the solver might choose a 33% pot bet on one board texture and a 75% pot bet on another, based on which sizing maximizes expected value.

With the "Specify Sizes to Compare" option enabled in Dynamic mode, users can constrain which sizes the algorithm considers before simplifying. For example, you might tell it to consider 33%, 50%, 75%, and 150% of pot, and the dynamic algorithm selects the best 2 from that set at each node.

This approach mirrors how strong players actually play: they do not use every possible bet size at every decision point, but rather simplify to a few well-chosen sizes that capture most of the EV.

---

## Multiway Solving (3+ Players)

### Technical Achievement

GTO Wizard AI can instantly solve custom 3-way postflop spots with full control over stack sizes, bet sizes, ranges, rake, and opponent tendencies (via nodelocking). The system supports up to 9 players for preflop actions, though only 3 players can reach the flop.

### Computational Challenges

Multiway solving faces exponential complexity growth. A 6-max 100bb game tree with full preflop actions would contain over 622,000 nodes. The platform manages this through:

- **Player reduction**: Priority rules determine which 3 players proceed to postflop, based on action position and pot investment
- **Two solving modes**: "Classic Mode" uses full street expansion for maximum accuracy; "Fast Mode" uses Dynamic-Depth-Limited Solving (DDLS) with shorter lookaheads that resolve multiple times within a street, enabling very large trees in seconds while maintaining high accuracy
- **No abstractions**: Even in multiway spots, the solver works without mapping to simplified pre-computed models

### Capabilities and Limitations

| Feature               | Supported | Notes                                    |
|-----------------------|-----------|------------------------------------------|
| Custom ranges         | Yes       | Full range editor for each player        |
| Arbitrary bet sizing  | Yes       | All three sizing modes available         |
| Mixed stack depths    | Yes       | Side pot calculations included           |
| Rake structures       | Yes       | Customizable rake and cap               |
| Nodelocking           | Yes       | Lock any player's strategy              |
| Bunching effects      | Yes       | Accounts for folded players' cards      |
| Practice mode         | Yes       | Train against 3-way solutions           |
| ICM integration       | Planned   | Coming soon for multiway                |
| Dynamic bet sizing    | Planned   | Coming soon for 3-way                   |
| Aggregate reports     | Planned   | Coming soon for 3-way                   |
| 4+ player postflop    | Planned   | Longer-term roadmap                     |

---

## Use Cases

### Custom and Unusual Spots

The primary use case is solving spots that precomputed solution libraries do not cover:
- Non-standard stack depths (e.g., 47bb, 183bb)
- Unusual bet sizes from opponents (e.g., 2.7x raises, 83% pot bets)
- Specific range vs. range matchups after unusual preflop action
- Mixed game structures (BB ante, straddle, Mississippi straddle)

### Strategy Simplification

A simplified strategy implemented well outperforms a complicated strategy implemented poorly. GTO Wizard AI's dynamic sizing helps players find the minimal set of bet sizes that captures most of the EV, allowing them to practice with realistic simplified strategies.

### Exploitative Study via Nodelocking

Combined with nodelocking (see [Nodelocking](07_nodelocking.md)), custom solving allows players to:
1. Define how an opponent plays specific hands
2. Solve for the optimal counter-strategy
3. Practice implementing that exploit

### Multiway Pot Analysis

Study spots involving 3 players -- a common occurrence in live poker and tournament play -- where precomputed 2-way solutions do not apply.

---

## Credits and Cost System

### Custom Solves

Elite Tier membership ($149/month or $129/month annual) provides **unlimited custom solves**. There is no per-solve cost for standard custom solutions.

### Power Credits (for Aggregated Reports)

Power Credits are a separate currency used for custom aggregated reports (see [Aggregated Reports](08_aggregated_reports.md)). They are purchased in bundles with volume discounts, and the cost per report scales with complexity:

| Spot Type                 | Approximate Credit Cost |
|---------------------------|------------------------|
| 3-bet+ pots               | 175 credits (minimum)  |
| 100bb single-raised pots  | 175-300 credits        |
| Deep-stacked limped pots  | 500+ credits           |

The formula is: **Required Credits = Max(175, 10 x sqrt(Nodes))**

Credits expire one year after the Elite subscription ends. Ultra tier subscribers receive 1,500 Power Credits per month included.

---

## Saving and Organization

### Tagging System

Users can organize ranges, parameters, and solutions using tags with custom colors. Filtering supports both "match any tag" and "match all tags" conditions.

### Saving Options

- **Ranges**: Save customized hand distributions with tags
- **Parameters**: Store betting tree configurations for reuse across spots
- **Solutions**: Archive solved spots in "Saved solutions" or "Recently solved" sections

A recommended organizational approach is to tag by position and preflop action (e.g., tag "BB" for all big blind spots, "3bet" for three-bet pots).

---

## Screenshot References

> **Screenshot references (AI solver):** See `screenshots/website/ai-solver-default.avif`, `screenshots/website/ai-solver-hover.avif`

> **Screenshot references (3-way solving):** See `screenshots/blog/3way-solving-interface.png`, `screenshots/blog/3way-sb-decision-tree.png`

---

## Sources

- https://blog.gtowizard.com/introducing-gto-wizard-ai/
- https://blog.gtowizard.com/gto-wizard-ai-explained/
- https://blog.gtowizard.com/gto-wizard-ai-custom-multiway-solving/
- https://help.gtowizard.com/how-to-build-custom-solutions/
- https://help.gtowizard.com/custom-solving-faq/
- https://help.gtowizard.com/custom-solving-overview/
- https://blog.gtowizard.com/now_live_3_way_solving_nodelocking_2_0_and_50k_icm_ft_sims/

# GTO Wizard Solver Engine: Technical Deep Dive

## Overview

GTO Wizard's solver engine represents the state of the art in computational poker strategy. At its core, the platform combines two complementary solving paradigms: a traditional **Counterfactual Regret Minimization (CFR)** solver for precomputed solutions and a hybrid **neural network + CFR** engine (GTO Wizard AI, powered by Ruse AI technology) for real-time custom solving. This document examines the algorithms, data structures, and engineering decisions that make large-scale poker solving tractable.

---

## 1. Counterfactual Regret Minimization (CFR)

### 1.1 The Core Algorithm

CFR, published in 2007 by Martin Zinkevich at the University of Alberta, is the foundational algorithm behind virtually every modern poker solver. It operates on extensive-form game trees---directed graphs where each node represents a decision point for a player or chance event.

The algorithm proceeds iteratively:

1. **Initialize** all strategies to uniform random (equal probability for every legal action at every decision point).
2. **Traverse** the game tree, computing the *counterfactual value* of each action for each hand at each information set.
3. **Compute regret** for each action: the difference between the action's counterfactual value and the weighted average value of the current strategy.
4. **Update** the strategy using *regret matching*: actions with positive cumulative regret receive probability proportional to their regret; actions with zero or negative regret receive zero probability.
5. **Alternate** between players and repeat.

The key formula for regret matching at each information set is:

```
New_Strategy(action) = max(0, Cumulative_Regret(action)) / Sum(max(0, Cumulative_Regret(all actions)))
```

Over many iterations, the *average strategy profile* across all iterations provably converges to a Nash Equilibrium in two-player zero-sum games.

### 1.2 Why CFR Works

CFR's theoretical guarantee rests on the decomposition of overall regret into per-information-set counterfactual regrets. Minimizing counterfactual regret at every information set independently is sufficient to minimize total regret for the game. This decomposition enables parallelization and makes the algorithm practical for games with millions of information sets.

### 1.3 Convergence Properties

Convergence follows a characteristic pattern:

- Moving from an unsolved state to 0.5% Nash Distance (dEV) takes roughly as much compute as moving from 0.5% to 0.25%.
- Each halving of Nash Distance approximately doubles computation time.
- In practice, solvers target a pragmatic accuracy threshold rather than pursuing diminishing returns.

---

## 2. Monte Carlo CFR (MCCFR) Variants

### 2.1 The Scalability Problem

Vanilla CFR requires a full traversal of the entire game tree per iteration. For No-Limit Hold'em, where even a simplified tree can contain tens of millions of nodes, this becomes prohibitively expensive. A truly complete representation of NLHE has more nodes than atoms in the observable universe.

### 2.2 MCCFR Solution

Monte Carlo CFR (introduced by Lanctot et al., 2009) partitions the set of terminal histories into blocks and samples one block per iteration. Instead of traversing every path, MCCFR samples a subset of chance outcomes (card deals) and player actions. The key property is that the *expected* counterfactual regret update equals the exact regret update, so convergence guarantees are preserved in expectation.

Common MCCFR variants include:

| Variant | Sampling Strategy | Use Case |
|---------|-------------------|----------|
| **External Sampling** | Samples opponent actions and chance; traverses all of the traversing player's actions | Good balance of speed and variance |
| **Outcome Sampling** | Samples a single terminal history per iteration | Fastest per-iteration but highest variance |
| **Chance Sampling** | Samples only chance events (board cards); traverses all player actions | Lower variance, used in subgame solving |
| **Pure CFR** | Samples by choosing the highest-regret action deterministically | Used by some ACPC competitors |

### 2.3 Linear CFR and Discounted CFR

Modern variants weight recent iterations more heavily:

- **Linear CFR**: Weights iteration *t* by *t* when averaging strategies, giving more influence to later (more refined) iterations.
- **Discounted CFR (DCFR)**: Applies discount factors to positive regret, negative regret, and strategy contributions separately. Published by Brown & Sandholm (2019), DCFR converges significantly faster in practice.
- **CFR+**: Introduced by Tammelin (2014), floors cumulative regret at zero rather than allowing negative accumulation. Used by Libratus for its blueprint strategy and known for faster convergence in small- to medium-sized games.

GTO Wizard's precomputed solutions use iterative CFR to solve individual postflop subtrees to high accuracy. GTO Wizard AI's engine, by contrast, employs Monte Carlo Linear CFR for larger subgames and optimized vector-based Linear CFR for smaller ones, mirroring the approach used by Pluribus.

---

## 3. Neural Network Architecture for Strategy Approximation

### 3.1 Depth-Limited Solving

GTO Wizard AI, powered by the Ruse AI engine acquired in 2023, fundamentally differs from traditional solvers. Rather than constructing and solving the entire game tree from a given state to all terminal nodes, it imposes a **depth limit** at the end of each street (flop, turn, or river).

When the depth limit is reached, the value of the remainder of the game is **approximated by a neural network** rather than computed exactly. This transforms an intractable multi-street problem into a series of single-street solves.

```
Traditional Solver:     Flop --> Turn --> River --> Terminal (full tree)
GTO Wizard AI:          Flop --> NN_value | Turn --> NN_value | River --> Terminal
                        (solve one street at a time)
```

### 3.2 Neural Network Value Prediction

The neural networks are trained through **self-play** over hundreds of millions of hands. The training loop follows the Deep CFR paradigm:

1. Play hands using the current strategy.
2. After each hand, perform **counterfactual reasoning**: review each decision and compute what would have happened under alternative actions.
3. Use the resulting counterfactual values as training targets for the value network.
4. Update the neural network weights to better predict expected values for arbitrary game states.

The trained network learns to estimate the expected value of any hand in any game state, effectively encoding "poker intuition" into a differentiable function. This learned value function replaces the expensive recursive tree traversal that traditional solvers require.

### 3.3 Performance Characteristics

The depth-limited approach with neural value prediction enables:

- **Speed**: Optimal strategies generated for games up to 200bb deep with arbitrary bet sizes in an average of **3 seconds per street**.
- **Flexibility**: No predetermined bet size abstractions required---the solver considers the full continuous space of bet sizes.
- **Resource Efficiency**: Requires only 2 CPU cores and 8 GB RAM, compared to traditional solvers demanding 16+ cores at 5 GHz with 128 GB RAM for comparable accuracy.

### 3.4 Classic Mode vs. Fast Mode

GTO Wizard AI offers two solving modes:

- **Classic Mode**: Street-by-street solving. The solver constructs the full single-street game tree, uses the neural network for leaf evaluation at the street boundary, and runs CFR to convergence.
- **Fast Mode**: Dynamic-depth-limited solving. Constructs smaller lookahead windows and resolves recursively as actions occur, rather than waiting for a street boundary. This enables faster turnaround with a novel safe subgame-solving method that preserves strategic soundness.

---

## 4. Game Tree Construction and Sizes

### 4.1 Tree Structure

A poker game tree is a directed acyclic graph where:

- **Decision nodes** represent points where a player chooses an action (bet, check, call, raise, fold).
- **Chance nodes** represent card deals (flop, turn, river cards).
- **Terminal nodes** represent hand conclusions (showdown or fold).

Each decision node fans out into branches for every legal action. The tree's size is determined by the number of allowed bet sizes, raise caps, and streets remaining.

### 4.2 Representative Tree Sizes

| Configuration | Approximate Nodes |
|--------------|-------------------|
| Simple postflop tree (limited bet sizes) | ~696,613 |
| Complex postflop tree (multiple bet sizes) | ~87,364,678 |
| 6-max 100bb preflop tree (3 bet sizes) | ~622,000 preflop nodes |
| Full NLHE game tree (theoretical) | > 10^165 |

Tree size grows exponentially with:
- Number of allowed bet sizes per street
- Number of streets remaining
- Stack depth (deeper stacks allow more raise sequences)
- Number of players

### 4.3 Betting Tree Abstractions

To manage tree size, solvers restrict the set of legal actions:

- **Bet sizing restrictions**: Instead of a continuous range, solvers allow a discrete set (e.g., 33%, 50%, 67%, 75%, 100%, 125%, 150% of pot, and all-in).
- **Raise caps**: GTO Wizard enforces a maximum of **5 bets/raises per street**, with the final action converted to all-in.
- **Tree pruning**: Dominated or redundant bet sizes are removed.

GTO Wizard AI's **Dynamic Sizing** algorithm avoids fixed bet-size grids entirely, instead identifying the highest-EV bet sizes automatically and capturing **99.95% of available river EV** on average.

---

## 5. The 1,755 Strategically Distinct Flop Subsets

### 5.1 Flop Isomorphism

Of the 22,100 possible three-card flops in Hold'em, many are strategically equivalent due to suit symmetry. After accounting for suit isomorphisms (e.g., A-K-3 with two hearts plays identically to A-K-3 with two spades, all else equal), the number of strategically distinct flops reduces to **1,755**.

### 5.2 Flop Subsets for Preflop Solving

Since preflop and postflop play are inseparable---preflop decisions depend on how hands play across all possible boards---solving preflop requires considering all 1,755 flops. This is computationally extreme, so solvers use **weighted subsets** of representative flops.

Each flop in the subset carries a weight reflecting how many of the full 1,755 flops it represents. The weighted average EV across the subset approximates the true preflop EVs.

GTO Wizard offers four subset sizes:

| Subset Size | Flops | Accuracy | Use Case |
|-------------|-------|----------|----------|
| Small | 25 | Good approximation | Quick preflop estimation |
| Medium | 49 | Better accuracy | Standard analysis |
| Large | 85 | High accuracy | Detailed study |
| Full | 184 | Near-exact | Research-grade analysis |

### 5.3 Historical Development

- **Will Tipton** produced the first systematic flop subsets using mathematical frequency constraints.
- **PioSolver** validated subset accuracy by solving all 1,755 distinct flops and comparing aggregate preflop EVs.
- GTO Wizard builds on this work, using validated subsets to ensure preflop solutions closely match full-flop results.

---

## 6. Card Abstraction and Bucketing Techniques

### 6.1 The Abstraction Problem

Even after bet-size and raise-cap restrictions, the state space remains enormous because each player can hold any of ~1,326 preflop hand combinations, and boards further differentiate their strategic relevance. **Card abstraction** (bucketing) groups hands with similar strategic properties into equivalence classes, reducing the number of distinct "hands" the solver must track.

### 6.2 Bucketing Methods

| Method | Description | Typical Application |
|--------|-------------|---------------------|
| **Suit isomorphism** | Groups hands differing only in suit labels (e.g., A-K suited in hearts = A-K suited in spades) | Universal; lossless |
| **Equity bucketing** | Groups hands by their equity against a reference range into percentage buckets | Preflop and multiway |
| **K-means clustering** | Clusters hands by multi-dimensional feature vectors (equity distributions, draw potential, blockers) | Advanced postflop |
| **Expected Hand Strength (EHS)** | Buckets by expected equity considering future cards | Turn and river |
| **Potential-aware abstraction** | Considers equity *transitions* across future streets, not just current equity | State of the art |

### 6.3 Bucketing in Practice

The bucketing process typically works bottom-up:

1. On the **river** (final street), cluster hands into *k* buckets using equity or hand-strength features.
2. On the **turn**, derive buckets from the distribution of river buckets each hand transitions into.
3. On the **flop**, derive buckets from turn-bucket transition probabilities.
4. **Preflop** buckets can be derived similarly or defined by conventional hand categories.

Example: On a board of A-3-9 with a club draw, a hand like A-K with no backdoor flush draw might be bucketed as "top pair, good kicker, no draw." All hands in this bucket are played identically by the solver.

### 6.4 Abstraction-Free Solving

GTO Wizard AI claims to be **abstraction and blueprint-free** for its custom solutions. Rather than pre-bucketing hands, the neural network evaluates each hand individually in the context of the current game state. This eliminates the information loss inherent in bucketing, a key advantage over traditional abstraction-based solvers like Slumbot (which requires 250,000 core-hours and 2 TB RAM to generate its blueprint strategy).

---

## 7. Nash Distance Metrics and Accuracy Benchmarks

### 7.1 Defining Nash Distance

**Nash Distance** (also called dEV or exploitability) measures the maximum expected value an optimal adversary could gain by exploiting a given strategy, expressed as a percentage of the pot. A strategy with 0% Nash Distance is a perfect Nash Equilibrium; any positive value indicates room for exploitation.

Formally, Nash Distance for a two-player zero-sum game is:

```
Nash Distance = max over all counter-strategies of (EV_counter - EV_equilibrium) / Pot
```

### 7.2 GTO Wizard Accuracy Targets

| Solution Type | Nash Distance Target | Typical Range |
|--------------|---------------------|---------------|
| Precomputed (river) | 0.1% pot | Exact to 0.1% |
| Precomputed (flop/turn) | 0.1%--0.3% pot | Varies by spot complexity |
| GTO Wizard AI (custom) | 0.12% pot average | 0.06%--0.18% typical |
| 3-way river solutions | 0.1% pot | Solved in seconds |

### 7.3 Benchmark Methodology

GTO Wizard validates accuracy through multiple methods:

1. **Nodelock evaluation**: Lock the AI's flop strategy into a traditional solver (e.g., PioSolver) and measure the exploitability by computing the best response.
2. **EV comparison**: Compare the AI's strategy EV against PioSolver's EV for the same position. GTO Wizard AI captures **99.8% of available flop EV**.
3. **Head-to-head competition**: Test against established bots. GTO Wizard AI defeated Slumbot at **19.4 bb/100** over 150,000 hands of 200bb HUNL---the highest win rate ever recorded against Slumbot in its format.

### 7.4 PioSolver Speed Comparison

For a benchmark position (A-7-3 rainbow, 100bb heads-up single-raised pot, 95 GB tree):

| Solver | Time | Nash Distance |
|--------|------|---------------|
| PioSolver | 4,862 seconds | 0.23% |
| GTO Wizard AI | 6 seconds | 0.22% |

### 7.5 The Convergence--Accuracy Tradeoff

The **Law of Mixed Actions** states that in a perfect Nash Equilibrium, when a hand mixes between two or more actions, those actions must have identical EV. Solver noise---small EV differences between mixed actions---is a direct consequence of imperfect convergence. Players studying solver output should focus on action frequencies rather than the apparent EV ranking of mixed actions, as these rankings are artifacts of finite convergence.

### 7.6 April 2025 Engine Upgrade: Quantal Response Equilibrium

GTO Wizard's most recent engine upgrade replaced Nash Equilibrium with **Quantal Response Equilibrium (QRE)** for custom solutions:

- **QRE** assumes players make mistakes probabilistically, with the likelihood of a mistake inversely related to its EV cost. This contrasts with Nash Equilibrium's assumption of perfect play.
- The practical benefit is solving the **ghostline problem**: in traditional Nash solvers, zero-frequency decision nodes (positions that "never" arise in equilibrium play) receive minimal optimization. QRE naturally provides well-converged strategies for all nodes, including rarely visited ones.
- The upgrade produced a **25% reduction** in average flop exploitability, from 0.165% to 0.12% of pot.
- QRE applies only to GTO Wizard AI custom solutions; precomputed solutions continue to use traditional Nash algorithms.

---

## 8. Precomputed Solutions: Storage and Serving

### 8.1 Precomputation Pipeline

GTO Wizard maintains a massive library of precomputed postflop solutions covering:

- Multiple game formats (cash, MTT, SNG, PKO, ICM)
- Multiple stack depths
- Common preflop action sequences
- Standard board textures

Each solution is generated by running CFR to convergence (targeting 0.1%--0.3% Nash Distance) and storing the resulting strategy---the probability distribution over actions for each hand at each decision point.

### 8.2 Solution Storage

Strategies are stored as compressed lookup tables indexed by:

- Game format and configuration
- Preflop action sequence
- Board texture
- Street (flop, turn, river)
- Player position
- Hand

The storage requirements are substantial. For reference, a single complex postflop tree can contain 87+ million nodes, each storing action probabilities for ~1,000 possible hands. Compression techniques (quantization, sparse representation of near-zero frequencies) reduce storage by orders of magnitude.

### 8.3 Solution Serving

When a user queries a precomputed solution:

1. The frontend sends the position parameters (format, stacks, preflop action, board, street).
2. The backend identifies the matching precomputed solution.
3. Strategy data is retrieved and served, typically with aggressive caching at the CDN layer.
4. The frontend renders the strategy as range grids, EV comparisons, and frequency charts.

Precomputed solutions offer instant response times since no computation occurs at query time.

---

## 9. Exact Solving vs. Approximation Approaches

### 9.1 Comparison Matrix

| Property | Traditional CFR (Exact) | GTO Wizard AI (Hybrid) |
|----------|------------------------|----------------------|
| **Algorithm** | Full-tree CFR/MCCFR | Depth-limited CFR + neural network |
| **Speed** | Hours to days per spot | 3 seconds per street average |
| **Hardware** | 16+ cores, 128+ GB RAM | 2 cores, 8 GB RAM |
| **Accuracy** | 0.1--0.3% Nash Distance | 0.12% Nash Distance average |
| **Bet sizes** | Fixed discrete grid | Dynamic, continuous |
| **Abstraction** | Required for large trees | Abstraction-free |
| **Multiway** | Intractable beyond 2 players | Supports 3-way solving |
| **Preflop** | Requires flop subsets | Full preflop-through-river |
| **Customization** | Full control over tree | Full control over parameters |

### 9.2 When to Use Each Approach

- **Precomputed (exact CFR)** solutions are ideal for standard spots where speed of lookup matters and the position matches a precomputed configuration exactly.
- **GTO Wizard AI (hybrid)** is essential for custom spots---non-standard bet sizes, unusual stack depths, specific opponent tendencies (via nodelocking), multiway pots, and any position not covered by the precomputed library.

### 9.3 Academic Lineage

GTO Wizard AI builds on a lineage of poker AI breakthroughs:

| System | Year | Key Innovation |
|--------|------|---------------|
| **Cepheus** | 2015 | "Solved" Limit Hold'em using CFR+ |
| **DeepStack** | 2017 | First to use neural networks for depth-limited solving |
| **Libratus** | 2017 | Blueprint + real-time subgame solving; defeated top pros at HUNL |
| **Pluribus** | 2019 | Extended subgame solving to 6-player poker |
| **Ruse AI** | 2022 | Combined neural nets with CFR for real-time NLHE solving |
| **GTO Wizard AI** | 2023+ | Productionized Ruse AI; added QRE, multiway, dynamic sizing |

---

## 10. Multiway Solving

### 10.1 The Multiway Challenge

Nash Equilibrium for games with three or more players is not unique, and computing exact Nash Distance becomes intractable. The preflop game tree for a 6-max game with 3 bet sizes contains approximately 622,000 nodes, and this grows rapidly as players enter postflop.

### 10.2 GTO Wizard's Approach

GTO Wizard AI restricts multiway solutions to a maximum of **3 players reaching the flop**, which reduces the game tree by approximately 20x in typical spots. This makes the problem tractable while covering the vast majority of real-world multiway scenarios.

### 10.3 3-Way Benchmarks

- 3-way river positions are solved to 0.1% Nash Distance or lower in a few seconds.
- Turn benchmarks show an average Nash Distance of 0.24% +/- 0.08% pot against maximally exploitative opponents.
- Accuracy verification uses internal benchmarks against third-party solvers and extended-runtime comparison with deeper search depths.

---

## Sources

- https://blog.gtowizard.com/how-solvers-work/
- https://blog.gtowizard.com/gto-wizard-ai/
- https://blog.gtowizard.com/gto-wizard-ai-explained/
- https://blog.gtowizard.com/gto-wizard-ai-benchmarks/
- https://blog.gtowizard.com/poker-subsets-and-abstractions/
- https://blog.gtowizard.com/understanding-nash-distance/
- https://blog.gtowizard.com/introducing-quantal-response-equilibrium-the-next-evolution-of-gto/
- https://blog.gtowizard.com/pioneering-poker-ai-research/
- https://blog.gtowizard.com/crushing-a-top-hunl-poker-bot/
- https://blog.gtowizard.com/ai-and-the-future-of-poker/
- https://blog.gtowizard.com/gto-wizard-ai-custom-multiway-solving/
- https://blog.gtowizard.com/gto_wizard_ai_3_way_benchmarks/
- https://test.gtowizard.com/accuracy-and-benchmarks/
- https://poker.cs.ualberta.ca/publications/NIPS07-cfr.pdf (Zinkevich et al., 2007 - original CFR paper)
- https://mlanctot.info/files/papers/nips09mccfr.pdf (Lanctot et al., 2009 - MCCFR)
- https://arxiv.org/pdf/1809.04040 (Brown & Sandholm, 2019 - Discounted CFR)
- https://en.wikipedia.org/wiki/Libratus
- https://www.science.org/doi/10.1126/science.aay2400 (Pluribus)
- https://aipokertutorial.com/game-abstractions/

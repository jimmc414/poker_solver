# GTO Wizard Glossary

A comprehensive reference covering poker fundamentals, game-theory and solver terminology, technical concepts, and GTO Wizard-specific terms. Organized alphabetically within each category.

---

## 1. Poker Fundamentals

### Positions

| Term | Definition |
|------|-----------|
| **Big Blind (BB)** | The larger of two mandatory preflop bets, posted by the player two seats left of the dealer button. Also used as a unit of measurement for stack sizes and win rates. |
| **Button (BTN)** | The dealer position; last to act on all postflop streets, conferring maximum positional advantage. |
| **Cutoff (CO)** | The seat immediately right of the button; second-best position postflop. |
| **Early Position (EP)** | The first seats to act after the blinds; typically UTG and UTG+1 in a full ring game. |
| **Hijack (HJ)** | The seat immediately right of the cutoff; a middle position at a 6-max table. |
| **In Position (IP)** | The player who acts last on postflop streets, providing an informational advantage. |
| **Late Position (LP)** | Button and cutoff; positions that act last or near-last postflop. |
| **Middle Position (MP)** | Seats between early and late position. |
| **Out of Position (OOP)** | The player who acts first on postflop streets, at an informational disadvantage. |
| **Small Blind (SB)** | The smaller mandatory preflop bet, posted by the player immediately left of the button. |
| **Under the Gun (UTG)** | The first player to act preflop, immediately left of the big blind. |

### Actions

| Term | Definition |
|------|-----------|
| **All-in** | Wagering all remaining chips on a single action. |
| **Bet** | The first voluntary chips placed into the pot on a given street. |
| **Call** | Matching the current bet amount to stay in the hand. |
| **Check** | Declining to bet when no bet has been made on the current street; passes action to the next player. |
| **Check-raise (XR)** | Checking when first to act, then raising after an opponent bets on the same street. |
| **Continuation bet (C-bet)** | A bet made by the previous street's aggressor on the current street. |
| **Donk bet** | A bet by the out-of-position player into the previous street's aggressor before that player has acted. |
| **Fold** | Discarding hole cards and forfeiting any claim to the pot. |
| **Limp** | Calling the big blind preflop without raising. |
| **Overbet** | A bet larger than the current pot size. |
| **Probe bet** | An out-of-position bet on the turn or river after the in-position player checked back the previous street. |
| **Raise** | Increasing the size of an existing bet. |
| **Squeeze** | A preflop raise after an open-raise and one or more callers. |
| **Three-bet (3-bet)** | The third bet on a street; in preflop context, a raise over an open-raise. |
| **Four-bet (4-bet)** | The fourth bet on a street; a re-raise of a three-bet. |

### Hand Types and Rankings

| Term | Definition |
|------|-----------|
| **Air** | A hand with no made hand and no meaningful draw; typically pure bluff candidates. |
| **Bluff catcher** | A hand strong enough to beat an opponent's bluffs but not their value range. |
| **Broadway** | A straight from Ten through Ace (T-J-Q-K-A). |
| **Combo draw** | A hand with multiple simultaneous draws (e.g., flush draw + straight draw). |
| **Full house (Boat)** | Three of a kind plus a pair. |
| **Gutshot (Gutter)** | An inside straight draw requiring one specific card rank to complete; 4 outs. |
| **Nuts** | The strongest possible hand given the current board. |
| **Open-ended straight draw (OESD)** | Four consecutive cards that can be completed on either end; 8 outs. |
| **Overpair** | A pocket pair higher than all community cards. |
| **Pocket pair** | Two hole cards of the same rank. |
| **Set** | Three of a kind made with a pocket pair plus one matching board card. |
| **Suited connectors** | Hole cards of the same suit and consecutive rank (e.g., 7h-8h). |
| **Top pair** | A pair made with one hole card matching the highest community card. |
| **Trips** | Three of a kind made with one hole card and two matching board cards. |
| **Wheel** | The lowest possible straight: A-2-3-4-5. |

### Board Textures

| Term | Definition |
|------|-----------|
| **Dry** | A board with few draws and little connectivity between cards (e.g., K-7-2 rainbow). |
| **Monotone** | A board where all community cards share the same suit. |
| **Paired** | A board containing two cards of the same rank. |
| **Rainbow** | A flop with all three cards in different suits, eliminating flush draws. |
| **Two-tone** | A board with two cards of one suit, creating flush draw possibilities. |
| **Wet** | A board with many draws and connected cards (e.g., 8-9-T with two hearts). |

### Game Formats

| Term | Definition |
|------|-----------|
| **Cash game (Ring game)** | A format where chips have direct monetary value; players can join and leave freely. |
| **Heads-up (HU)** | A game or pot involving exactly two players. |
| **Multi-Table Tournament (MTT)** | A tournament spread across multiple tables, with players eliminated until one remains. |
| **Progressive Knockout (PKO)** | A tournament where part of each player's buy-in becomes a bounty; half of each bounty collected is added to the winner's own bounty. |
| **Sit and Go (SNG)** | A tournament that begins when a predetermined number of players register. |
| **Single-raised pot (SRP)** | A pot that was raised once preflop (open-raise plus call). |
| **Three-bet pot (3BP)** | A pot that was three-bet preflop. |
| **Four-bet pot (4BP)** | A pot that was four-bet preflop. |

### Key Metrics

| Term | Definition |
|------|-----------|
| **bb/100** | Win rate measured in big blinds won or lost per 100 hands played. |
| **Equity** | The percentage probability of winning the pot at showdown, given current cards and assuming no further betting. |
| **Expected Value (EV)** | The average outcome of an action calculated over all possible future card runouts and opponent responses, weighted by probability. |
| **Pot odds** | The ratio of the current pot size to the cost of a call, used to determine whether calling is mathematically justified. |
| **Stack-to-Pot Ratio (SPR)** | The effective stack size divided by the pot size; a measure of how "deep" the remaining play is relative to the pot. |
| **VPIP** | "Voluntarily Put In Pot"---the percentage of hands a player voluntarily enters preflop. |

---

## 2. Game Theory and Solver Terminology

### Core Concepts

| Term | Definition |
|------|-----------|
| **Nash Equilibrium** | A strategy profile where no player can improve their expected value by unilaterally changing their own strategy. In two-player zero-sum games like heads-up poker, a Nash Equilibrium guarantees at least a break-even result regardless of the opponent's strategy. |
| **Game Theory Optimal (GTO)** | A strategy that constitutes a Nash Equilibrium---the least exploitable fixed strategy, yielding the highest guaranteed expected value. In practice, GTO strategies are approximated by solvers to within a small exploitability margin. |
| **Exploitative strategy** | A strategy that deviates from GTO to take advantage of specific opponent mistakes. Increases EV against the target opponent but may become exploitable by a sufficiently adaptive adversary. |
| **Maximally Exploitative Strategy (MaxES / Nemesis)** | The strategy that extracts the maximum possible EV against a fixed opponent strategy. Also called the "best response" or nemesis strategy. See [Solver Engine](02_solver_engine.md) for how this is computed. |
| **Minimally Exploitative Strategy (MinES)** | A strategy that exploits a single specific opponent mistake at one node while remaining GTO everywhere else. |
| **Quantal Response Equilibrium (QRE)** | An equilibrium concept where players make probabilistic mistakes inversely proportional to their cost. Used by GTO Wizard AI since April 2025 for custom solutions. See [Solver Engine](02_solver_engine.md), Section 7.6. |

### Strategy Concepts

| Term | Definition |
|------|-----------|
| **Balance** | Constructing ranges that include an appropriate mix of value hands and bluffs, making it difficult for opponents to determine hand strength from action alone. |
| **Blueprint strategy** | A precomputed strategy covering the full game, typically generated by running CFR on an abstracted game tree. Used as a starting point; refined through subgame solving in real-time play. |
| **Indifference** | A state where two or more actions yield the same expected value for a given hand. At Nash Equilibrium, hands that mix between actions are indifferent between them. |
| **Mixed strategy** | A strategy that randomizes between multiple actions with specified probabilities for a given hand at a given decision point. |
| **Mixing mistake** | An error in the execution of a mixed strategy---taking one action too frequently or too infrequently compared to the equilibrium frequencies. |
| **Passive exploitation** | Gaining EV from opponent mistakes without actively deviating from GTO. A GTO strategy passively exploits any opponent who deviates from equilibrium. |
| **Polarized range** | A range consisting of very strong hands (value) and very weak hands (bluffs), with few medium-strength hands. Typically used when overbetting or on later streets. |
| **Pure strategy** | A strategy where the same action is always taken with a given hand at a given decision point (no randomization). |
| **Range advantage** | When one player's range of possible hands is collectively stronger than the other's at a given point in the hand. |
| **Range morphology** | The shape and distribution of a player's range---linear, polarized, condensed, merged, or capped. |

### Exploitability and Accuracy

| Term | Definition |
|------|-----------|
| **Convergence** | The process by which iterative algorithms (CFR) approach a Nash Equilibrium. Measured by the decreasing Nash Distance over iterations. |
| **EV loss** | The expected value sacrificed by taking a suboptimal action compared to the GTO-optimal action. Measured in big blinds. |
| **Exploitability** | The maximum EV an optimal adversary could gain against a given strategy, usually expressed as a percentage of the pot. Synonymous with Nash Distance. |
| **Ghostline** | A zero-frequency decision node in a solver's game tree---a line of play that the equilibrium strategy never takes. Traditional Nash solvers often produce poorly converged strategies at ghostlines because there is no "pressure" to optimize them. QRE addresses this. |
| **Nash Distance (dEV)** | The maximum aggregate EV a strategy can be exploited for, expressed as a percentage of the starting pot. GTO Wizard targets 0.1--0.3% for precomputed solutions and 0.12% average for AI solutions. See [Solver Engine](02_solver_engine.md), Section 7. |
| **Nodelock** | Fixing one player's strategy at specific decision nodes and re-solving the rest of the tree. Used to model opponent tendencies and compute exploitative adjustments. See [AI Features](07_ai_features.md) for GTO Wizard's Nodelocking 2.0. |
| **Solver noise** | Small EV differences between mixed-strategy actions that arise from imperfect convergence. Not strategically meaningful; diminishes as Nash Distance approaches zero. |

### Structural Concepts

| Term | Definition |
|------|-----------|
| **Game tree** | The complete directed graph of all possible action sequences in a game, from root (initial state) to terminal nodes (hand conclusion). |
| **Information set** | The set of game states that are indistinguishable to a player. In poker, a player cannot see opponent hole cards, so multiple game states map to the same information set. |
| **Node** | A single decision point in the game tree where a player (or chance) takes an action. |
| **Subgame** | A portion of the game tree starting from a particular decision point. Subgame solving resolves this portion independently, using boundary values from the larger game. |
| **Terminal node** | A game tree node where the hand ends (showdown or fold), with a defined payoff for each player. |

---

## 3. Technical and Algorithmic Terms

### Algorithms

| Term | Definition |
|------|-----------|
| **CFR (Counterfactual Regret Minimization)** | The foundational iterative algorithm for solving imperfect-information games. Decomposes overall regret into per-information-set counterfactual regrets and minimizes them independently. Published by Zinkevich et al. (2007). See [Solver Engine](02_solver_engine.md), Section 1. |
| **CFR+** | A CFR variant by Tammelin (2014) that floors cumulative regret at zero. Converges faster in small to medium games. Used by Libratus for blueprint computation. |
| **Deep CFR** | An approach that uses neural networks to approximate the regret tables in CFR, enabling the algorithm to scale to very large games without storing regrets for every information set. |
| **Depth-limited solving** | Solving the game tree only to a fixed depth (e.g., one street), with a neural network estimating the value of game states beyond that depth. The core approach of GTO Wizard AI. See [Solver Engine](02_solver_engine.md), Section 3. |
| **Discounted CFR (DCFR)** | A CFR variant (Brown & Sandholm, 2019) that applies separate discount factors to positive regret, negative regret, and strategy contributions. Converges significantly faster in practice. |
| **Linear CFR** | A CFR variant that weights iteration *t* by *t* when computing the average strategy, giving more influence to recent (more refined) iterations. |
| **MCCFR (Monte Carlo CFR)** | A family of CFR variants that sample portions of the game tree rather than traversing it fully, dramatically reducing per-iteration cost while preserving convergence guarantees in expectation. See [Solver Engine](02_solver_engine.md), Section 2. |
| **Regret matching** | The mechanism used in CFR to convert cumulative regrets into a strategy: each action's probability is proportional to its positive cumulative regret. |
| **Safe subgame solving** | Techniques that solve a subgame while guaranteeing the resulting strategy is no worse than the original blueprint, even if opponents deviate. Used by Libratus, Pluribus, and GTO Wizard AI's Fast Mode. |
| **Self-play** | A training paradigm where an AI agent plays against itself, generating training data from both sides. Used to train GTO Wizard AI's neural networks across hundreds of millions of hands. |

### Abstraction and Compression

| Term | Definition |
|------|-----------|
| **Abstraction** | Any method that reduces the size of the game's state space to make solving tractable. Includes card abstraction, action abstraction, and information abstraction. See [Solver Engine](02_solver_engine.md), Sections 5--6. |
| **Bet sizing abstraction** | Restricting the set of allowed bet sizes to a finite grid (e.g., 33%, 67%, 100%, 150% pot) rather than allowing continuous values. |
| **Bucketing (Binning)** | Grouping strategically similar hands into equivalence classes. Hands within the same bucket are played identically by the solver, reducing the number of distinct "hands" to track. |
| **Card isomorphism** | The principle that hands differing only in suit labels are strategically equivalent (e.g., A-K suited in hearts = A-K suited in diamonds). Exploited universally as a lossless abstraction. |
| **Dynamic sizing** | GTO Wizard AI's algorithm that automatically identifies optimal bet/raise sizes from the continuous space, avoiding fixed bet-size grids. Captures 99.95% of available river EV. |
| **Flop subset** | A weighted set of representative flops (from the full 1,755 strategically distinct flops) used for preflop solving. GTO Wizard offers subsets of 25, 49, 85, and 184 flops. See [Solver Engine](02_solver_engine.md), Section 5. |
| **Imperfect recall abstraction** | An abstraction where a player "forgets" some information available to them on earlier streets, reducing the game tree at the cost of solution quality. |
| **K-means clustering** | A machine learning algorithm used to group hands into buckets based on multi-dimensional feature vectors (equity, draw potential, blockers, etc.). |
| **Raise cap** | The maximum number of bets or raises allowed per street. GTO Wizard uses a cap of 5, with the final action converted to all-in. |

### Neural Networks and Machine Learning

| Term | Definition |
|------|-----------|
| **Counterfactual reasoning** | After playing a hand, reviewing each decision and computing what would have happened under alternative actions. Used to generate training data for the neural network. |
| **Inference** | Running a trained neural network forward to produce predictions (EV estimates) for a given input (game state). Distinguished from training, which updates the network's weights. |
| **Value network** | A neural network trained to predict the expected value of a hand in a given game state, used as the leaf evaluator in depth-limited solving. |

### Poker AI Systems

| Term | Definition |
|------|-----------|
| **Cepheus** | A poker AI (2015, University of Alberta) that essentially "solved" Limit Hold'em using CFR+. |
| **DeepStack** | A poker AI (2017) that pioneered the use of neural networks for depth-limited solving in NLHE. |
| **Libratus** | A poker AI (2017, CMU) that defeated top human professionals at HUNL using blueprint computation (CFR+) combined with real-time subgame solving. |
| **Pluribus** | A poker AI (2019, CMU/Meta) that extended subgame solving to 6-player NLHE, using Monte Carlo Linear CFR and a novel search algorithm. |
| **Ruse AI** | A poker solver developed in 2022 by Philippe Beardsell and Marc-Antoine Provost, combining neural networks with traditional CFR for real-time solving. Acquired by GTO Wizard in 2023; now powers GTO Wizard AI. |
| **Slumbot** | An open-source HUNL poker bot using traditional CFR with hand abstractions. GTO Wizard AI defeated it at 19.4 bb/100 over 150,000 hands. |

---

## 4. GTO Wizard-Specific Terms

### Platform Features

| Term | Definition |
|------|-----------|
| **Custom Solve** | An on-demand solve using GTO Wizard AI where users specify all parameters: ranges, bet sizes, stack depth, board, and format. Consumes Power Credits. |
| **Dynamic Sizing** | A GTO Wizard AI feature that automatically determines optimal bet/raise sizes rather than requiring users to specify a fixed grid. |
| **EV Comparison Tool** | A feature that displays the expected value of each possible action for every hand, enabling users to compare the EV cost of different strategic choices. |
| **GTO Wizard AI** | The platform's real-time AI solver, powered by Ruse AI technology. Uses depth-limited solving with neural network value prediction to generate strategies in seconds. See [Solver Engine](02_solver_engine.md), Section 3. |
| **Hand History Analyzer (HH Analyzer)** | A tool that imports hand histories from poker sites, compares each decision against GTO, and measures EV loss. Version 2.0 includes aggregated statistics and custom filtering. |
| **Nodelocking** | Fixing one player's strategy at specific tree nodes to model non-GTO opponent behavior. The solver then computes the optimal exploitative response. Nodelocking 2.0 added enhanced multi-node locking and improved convergence. |
| **Power Credits** | A metered-compute currency that users purchase and spend on compute-intensive operations (custom solves, aggregated reports, advanced analysis). Reflects the processing power required. Credits expire one year after Elite subscription ends. |
| **Practice Mode** | An interactive training mode where users play hands against a GTO opponent, receiving feedback on each decision including EV loss and correct action frequencies. |
| **Range Builder** | A tool for constructing, editing, and saving hand ranges as 13x13 grid matrices with per-combo action frequencies. |
| **Study Mode** | The core interface for browsing precomputed GTO solutions. Users navigate the game tree, view strategy frequencies, and compare EV across actions. See [Study Mode](03_study_mode.md). |

### Subscription Tiers

| Term | Definition |
|------|-----------|
| **Free tier** | Basic access with limited precomputed solutions and restricted features. |
| **Premium** | Paid subscription (~$35/month) providing full access to precomputed solutions across formats. |
| **Elite** | Top-tier subscription (~$129--149/month) including GTO Wizard AI custom solving, advanced reports, and Power Credit allocation. |

### Analysis Concepts

| Term | Definition |
|------|-----------|
| **Aggregated Report** | A statistical summary computed across many hands or positions, showing patterns in strategy, mistakes, and frequencies. Requires Power Credits. |
| **Board coverage** | The ability of a range to form strong hands across diverse board textures. A range with good board coverage has natural nutted hands on most flop types. |
| **Bubble Factor (BF)** | An ICM metric measuring how much more losing hurts relative to how much winning helps at a given tournament stage. BF > 1 means losing a pot is disproportionately costly. |
| **Card removal (Blockers)** | The effect of holding specific cards on the composition of opponents' possible ranges. For example, holding the A of spades blocks opponents from having the nut flush draw on a spade board. |
| **Chip EV** | Expected value measured in raw chips, ignoring ICM considerations. Used in cash games and as a simplification in tournament analysis. |
| **Dollar EV ($EV)** | Expected value measured in real money, accounting for ICM and payout structures. The true economic value of a decision in tournament play. |
| **Equity Realization (EQR)** | The ratio of a hand's actual expected value to its raw equity. Accounts for positional advantage, playability, and ability to navigate postflop action. EQR > 1 means the hand performs better than raw equity suggests. |
| **ICM (Independent Chip Model)** | A mathematical model that converts tournament chip stacks into prize equity, accounting for the nonlinear relationship between chips and money in tournaments. |
| **Isomorphic** | Strategically equivalent; typically referring to hands or board states that differ only in suit labels and can be treated identically. |
| **Pure mistake** | An action that is strictly dominated---it loses EV against the current GTO strategy regardless of future play. Distinguished from mixing mistakes, which involve incorrect frequencies of individually viable actions. |
| **Risk Premium** | The extra EV cost of all-in confrontations in tournaments due to ICM; going all-in and losing is disproportionately costly compared to the upside of winning. |
| **Simulations (Sims)** | The iterative computational process by which solvers derive GTO strategies. Commonly used colloquially to refer to solver outputs ("running sims"). |
| **Table Equity (TEQ)** | A player's ICM equity expressed relative to the total chips at their table, providing a normalized measure of tournament standing. |

---

## 5. Mathematical Notation

| Symbol | Meaning |
|--------|---------|
| **EV** | Expected Value; the probability-weighted average of all possible outcomes. |
| **dEV** | Delta EV; the difference in expected value, often used synonymously with Nash Distance. |
| **Alpha** | The minimum fold frequency an opponent needs for a pure bluff to break even: alpha = bet / (bet + pot). |
| **MDF** | Minimum Defense Frequency: the minimum percentage of a range a player must continue with to prevent the opponent's bluffs from being automatically profitable. MDF = 1 - alpha = pot / (pot + bet). |
| **SPR** | Stack-to-Pot Ratio: effective_stack / pot_size. |
| **EQR** | Equity Realization: actual_EV / (equity * pot). |
| **BF** | Bubble Factor: measures ICM pressure; BF = (cost of losing) / (benefit of winning). |

---

## Sources

- https://pages.gtowizard.com/glossary/ (GTO Wizard official poker glossary - primary source for definitions)
- https://blog.gtowizard.com/how-solvers-work/
- https://blog.gtowizard.com/what-is-gto-in-poker/
- https://blog.gtowizard.com/understanding-nash-distance/
- https://blog.gtowizard.com/introducing-quantal-response-equilibrium-the-next-evolution-of-gto/
- https://blog.gtowizard.com/gto-wizard-ai-explained/
- https://blog.gtowizard.com/poker-subsets-and-abstractions/
- https://blog.gtowizard.com/gto-wizard-ai-benchmarks/
- https://blog.gtowizard.com/pioneering-poker-ai-research/
- https://aipokertutorial.com/game-abstractions/
- https://poker.cs.ualberta.ca/publications/NIPS07-cfr.pdf
- https://en.wikipedia.org/wiki/Libratus

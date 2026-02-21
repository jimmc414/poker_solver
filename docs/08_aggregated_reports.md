# Aggregated Reports: Macro-Level Strategy Analysis

## Overview

Aggregated reports are one of GTO Wizard's most powerful analytical tools. Rather than studying individual flops in isolation, aggregated reports analyze strategy across **all 1,755 strategically distinct flop subsets** simultaneously. This allows players to identify broad strategic patterns, frequency trends, and macro-level heuristics that would be invisible when studying one board at a time.

As the GTO Wizard team describes it, aggregated reports help you "see the forest through the trees."

---

## Why 1,755 Flops?

In Texas Hold'em, there are 22,100 possible three-card flop combinations. However, many flops are strategically identical when suits are interchangeable. For example, A-heart K-heart 7-spade plays identically to A-spade K-spade 7-heart in a game with no suit-dependent preflop ranges. After accounting for suit isomorphisms, exactly **1,755 strategically distinct flops** remain. Aggregated reports solve and display data for all of them.

---

## Standard vs. Custom Aggregated Reports

### Standard Aggregated Reports

Standard aggregate reports use GTO Wizard's pre-solved solution library. They are available to all subscribers (not just Elite Tier) and cover the spots already in the solution database. These reports use fixed parameters -- the same stack depths, bet sizes, and ranges as the precomputed solutions.

Standard reports can be accessed from the Reports tab in the solution browser at any flop or turn node.

### Custom Aggregated Reports

Custom aggregated reports are powered by GTO Wizard AI and allow full parameter customization. Every detail is configurable:

- Starting hand ranges for both players
- Bet sizing (fixed or dynamic)
- Stack depth
- Rake structure
- ICM dynamics
- Prize structures
- Bounties

With one click, GTO Wizard AI generates the optimal strategy across all 1,755 flops, solved in minutes, and displayed in a clean interface.

Custom aggregated reports are **exclusively available to Elite subscribers** and require Power Credits (see below).

---

## Power Credits System

### What Are Power Credits?

Power Credits are GTO Wizard's currency for generating custom aggregated reports. They were introduced to sustainably fund the computational infrastructure required -- solving all 1,755 flops with custom parameters demands significant server resources.

### Cost Formula

The credit cost per report scales with computational complexity:

```
Required Credits = Max(175, 10 x sqrt(Nodes))
```

Where `Nodes` refers to the total number of decision points in the game tree across all flops.

### Typical Costs

| Spot Type                      | Approximate Credit Cost |
|-------------------------------|------------------------|
| 3-bet+ pots (small trees)     | 175 credits (minimum)  |
| 100bb single-raised pots      | 175-300 credits        |
| Deep-stacked limped pots      | 500+ credits           |

The minimum cost is 175 credits (approximately one cent per flop), which covers computational overhead. Complexity increases with stack depth and the number of available bet sizes.

### Credit Bundles

Credits are purchased in bundles, with larger bundles offering better per-credit rates. Three bundle tiers are available, accommodating both casual users (occasional reports) and serious grinders (frequent analysis).

### Credit Policies

- Credits expire one year after your Elite subscription ends
- If you maintain your subscription, credits carry over indefinitely
- Ultra Tier subscribers receive **1,500 Power Credits per month** included (approximately $140 value)
- Reports remain accessible as long as your subscription is active; access suspends if it expires but restores upon resubscription

---

## Report Types and Metrics

Aggregated reports measure four primary data types:

### 1. Strategy (Action Frequencies)

Shows how often each action (check, bet small, bet medium, bet large, overbet, raise, fold) is taken across all flops. This reveals patterns such as:
- What percentage of flops the solver c-bets
- How often large vs. small sizes are preferred
- Which board textures trigger checking vs. betting

```
+--------------------------------------------------------------+
|            STRATEGY REPORT: SB vs BB, 3-Bet Pot              |
+--------------------------------------------------------------+
|                                                                |
|  Overall Flop C-Bet Frequency:                                 |
|                                                                |
|  Check:     28% |========                                      |
|  Bet 33%:   52% |================                              |
|  Bet 75%:   14% |====                                          |
|  Bet 150%:   6% |==                                            |
|                                                                |
|  High Card Flops:                                              |
|  Check:     18% |=====                                         |
|  Bet 33%:   62% |===================                           |
|  Bet 75%:   15% |=====                                         |
|  Bet 150%:   5% |=                                             |
|                                                                |
|  Low Card Flops:                                               |
|  Check:     42% |=============                                 |
|  Bet 33%:   38% |============                                  |
|  Bet 75%:   12% |====                                          |
|  Bet 150%:   8% |==                                            |
+--------------------------------------------------------------+
```

### 2. Expected Value (EV)

Displays the EV of each action across flop subsets. This helps identify:
- Which board textures are most profitable for each player
- Where betting vs. checking generates the most EV
- Relative profitability of different sizing options

### 3. Equity (EQ)

Shows raw hand strength (equity percentage) distributions across boards. Useful for understanding:
- Which player has the range advantage on different board textures
- How equity distributions shift between positions
- Where nut advantages and range advantages align or diverge

### 4. Equity Realization (EQR)

Measures how effectively a player converts their raw equity into actual value. EQR above 100% means the player captures more than their fair share of the pot; below 100% means they realize less.

This metric is crucial for understanding positional advantage, as the in-position player typically realizes more equity than the out-of-position player.

---

## Filtering and Segmentation

### Board Texture Filters

Reports can be segmented by board characteristics:

| Filter Category    | Options                                           |
|-------------------|---------------------------------------------------|
| Suit composition   | Monotone, two-tone, rainbow                        |
| Pairing           | Paired boards, unpaired boards                     |
| Connectedness     | Connected, gapped, disconnected                    |
| High card         | By highest card rank (Ace-high, King-high, etc.)   |
| Board type        | Broadway-heavy, low boards, mixed                  |

### Flop Grouping Options

Data can be organized by:
- **High card**: Group flops by the highest card
- **Suits**: Group by suit composition (monotone, two-tone, rainbow)
- **Pairing**: Group by whether the board is paired
- **Connectedness**: Group by how connected the board cards are

### Action Grouping

Actions can be displayed as:
- **All sizes**: Show each individual bet size separately
- **Grouped**: Combine into small/medium/large/overbet categories
- **Simplified**: Binary bet/check view

### Hole Card Filtering (Turn Reports)

Turn reports allow filtering by specific hole cards, enabling analysis of how particular hands interact with different runouts.

---

## Display Formats

### Chart View

Visual bar graphs showing data trends across flop categories. Best for:
- Identifying visual patterns
- Quick comparison between board textures
- Presentation and study sessions

### Table View

Numerical format with precise values and sorting capabilities. Best for:
- Exact frequency/EV numbers
- Sorting by specific metrics
- Detailed quantitative analysis

---

## How to Interpret Aggregated Data

### Overall Statistics

The "Overall" row displays weighted averages across all applicable flops. This gives the global average -- for example, the overall c-bet frequency across all 1,755 flops.

### Filtered Statistics

When filters are applied, a "Filtered" row appears showing metrics for the selected subset. The interface color-codes values:
- **Green**: Above the overall average
- **Red**: Below the overall average

This makes it easy to spot which board textures deviate from the norm.

### Comparison Analysis

The most powerful analytical technique is comparing filtered subsets against the full dataset:

```
+--------------------------------------------------------------+
|            INTERPRETING FILTERED DATA                          |
+--------------------------------------------------------------+
|                                                                |
|  Example: Monotone flops vs. All flops                         |
|                                                                |
|  Metric          All Flops    Monotone    Delta                |
|  -----------     ---------    --------    -----                |
|  C-Bet Freq       65%          48%        -17%                 |
|  Avg Bet Size     45% pot      33% pot    -12%                 |
|  IP Equity        52%          49%        -3%                  |
|  IP EQR           108%         103%       -5%                  |
|                                                                |
|  Interpretation: On monotone boards, the c-bettor bets less    |
|  frequently and smaller. Positional advantage decreases.       |
+--------------------------------------------------------------+
```

---

## Turn Aggregate Reports

Turn reports are accessed from the Reports tab on any turn node. Rather than aggregating across flops, turn reports show data for **all possible turn cards on a given flop**. This reveals:

- How strategy changes across different runout cards
- Which turn cards favor which player
- Sizing preferences across turn textures
- How equity distributions shift from flop to turn

Turn reports are particularly useful for studying how board development affects continuation strategies.

---

## Use Cases

### Developing Heuristics

Aggregated reports allow players to identify rules of thumb backed by data:
- "In 3-bet pots as the preflop raiser, c-bet 65% of flops at 33% pot"
- "On monotone boards, reduce c-bet frequency by 15-20%"
- "Use larger sizes on dry boards (Kxx rainbow) vs. wet boards"

These heuristics are impossible to derive from studying individual flops.

### Comparing Positions

Run reports for the same spot from different positions to understand how positional advantage manifests across all board textures.

### Studying Format Differences

With custom reports, compare how strategy changes between:
- Cash game (Chip EV) vs. tournament (ICM)
- Different stack depths (20bb vs. 100bb)
- Different rake structures
- Different preflop ranges

### Identifying Leaks

Compare your actual play frequencies (from hand history analysis -- see [Hand History Analyzer](04_hand_history_analyzer.md)) against aggregated report frequencies to identify systematic strategic errors.

### Bet Size Optimization

Dynamic sizing in custom reports reveals the optimal bet sizes across all flop textures, helping you choose the right sizes for your simplified strategy.

---

## Board Texture Analysis with Aggregated Reports

One of the most valuable applications of aggregated reports is understanding how board texture systematically influences strategy. Below are the major texture categories and the kinds of patterns the reports reveal.

### High Card Boards

Flops with an ace or king as the high card tend to favor the preflop raiser, who holds more strong top-pair combinations. Reports typically show higher c-bet frequencies and smaller sizing preferences on these boards, since the range advantage allows frequent, cheap pressure.

### Low and Middling Boards

Boards like 7-5-3 or 8-6-4 tend to favor the caller (particularly the Big Blind), who retains more two-pair and set combinations. C-bet frequencies drop, and when betting does occur, the solver often prefers larger sizes to compensate for the reduced range advantage.

### Monotone Boards

Three cards of the same suit create flush draw possibilities for both players. Reports show reduced c-bet frequency (often 15-20% lower than average) and smaller sizing, as both players have more drawing equity and the range advantage is compressed.

### Paired Boards

Boards with a pair (e.g., K-7-7) create unique dynamics where trips are rare but powerful. Aggregate reports reveal high c-bet frequencies on these boards because the caller is unlikely to hold a seven, giving the raiser an unchallenged range advantage on most paired textures.

### Connected Boards

Highly connected boards (e.g., J-T-9, 8-7-6) distribute equity more evenly between players. Reports show moderate c-bet frequencies with more checking and larger sizes when betting, reflecting the need for stronger hands to justify building the pot on volatile textures.

---

## Practical Example Workflow

1. **Define the spot**: BTN opens, BB calls, 100bb deep, standard rake
2. **Set ranges**: Import BTN open range and BB call range from precomputed solutions
3. **Configure sizing**: Dynamic, 2 bet sizes
4. **Generate report**: Click solve, wait 2-5 minutes for all 1,755 flops
5. **Analyze overall**: What is the global c-bet frequency and preferred sizing?
6. **Filter by texture**: How does strategy change on paired vs. unpaired boards?
7. **Examine turn runouts**: On your most common flop textures, which turn cards change strategy?
8. **Extract heuristics**: Write down 3-5 actionable rules for this spot

---

## Comparing Standard and Custom Reports: Decision Guide

When deciding between standard and custom aggregated reports, consider the following:

```
+--------------------------------------------------------------+
|      STANDARD vs. CUSTOM AGGREGATED REPORTS                   |
+--------------------------------------------------------------+
|                                                                |
|  Feature           Standard           Custom                   |
|  -------           --------           ------                   |
|  Cost              Free with sub      Power Credits required   |
|  Parameters        Fixed (preset)     Fully customizable       |
|  Ranges            Precomputed        Any custom range         |
|  Stack depths      Fixed options      Any depth (0.1bb incr.)  |
|  Bet sizing        Preset sizes       Dynamic/Fixed/Auto       |
|  ICM support       No                 Yes                      |
|  Rake              Fixed              Customizable             |
|  Solve time        Instant (cached)   2-5 minutes              |
|  Access tier       All subscribers    Elite only               |
|  Persistence       Always available   While subscribed         |
|                                                                |
|  Use Standard when: studying common spots, general study       |
|  Use Custom when: specific ranges, unusual depths, ICM,        |
|                    non-standard bet sizes, format comparison    |
+--------------------------------------------------------------+
```

For most players, standard reports cover the majority of study needs. Custom reports become essential when studying non-standard spots, comparing format differences (cash vs. tournament), or when your actual game conditions differ significantly from the precomputed library parameters.

---

## Screenshot References

> **Screenshot references (Reports interface):** See `screenshots/website/aggregated-reports-default.avif`

> **Screenshot references (Custom reports):** See `screenshots/blog/aggregated-reports-custom-tab.png`, `screenshots/blog/aggregated-reports-flop-cbet.png`

> **Screenshot references (Pricing/credits):** See `screenshots/blog/aggregated-reports-pricing.png`

---

## Sources

- https://blog.gtowizard.com/introducing_custom_aggregated_reports/
- https://help.gtowizard.com/aggregate-reports-guide/
- https://blog.gtowizard.com/new-solutions-aggregation-reports-and-other-improvements/
- https://blog.gtowizard.com/major-upgrade-gto-reports-have-arrived-plus-tons-of-new-features/
- https://blog.gtowizard.com/new-cash-game-mtt-solutions-aggregated-reports-and-new-solutions-for-hh-analyzer/

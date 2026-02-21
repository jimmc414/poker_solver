# Nodelocking: Exploitative Strategy Adjustment

## Overview

Nodelocking is the act of fixing ("locking") a player's strategy at a specific decision point ("node") in order to compute the optimal counter-strategy. While GTO (Game Theory Optimal) strategies assume both players play perfectly, real opponents deviate from equilibrium. Nodelocking bridges this gap: you define how an opponent actually plays, and the solver calculates the most profitable response.

This feature is exclusively available to Elite Tier users, though all users can experiment with nodelocking on the free demonstration flop (Q-spade T-spade 7-heart).

---

## Why Nodelocking Matters

In practice, no human plays perfect GTO. Opponents have tendencies: some fold too much to c-bets, others never fold the river, some overbluff, and many have sizing tells. Nodelocking allows you to:

1. **Model specific opponent behaviors** -- Define exactly how an opponent plays certain hands
2. **Compute exploitative counters** -- The solver finds the mathematically optimal response in seconds
3. **Study population tendencies** -- Lock strategies to reflect how the average player pool deviates from GTO
4. **Quantify exploitability** -- Measure EV differences between GTO and locked strategies to gauge how much an exploit is worth

The result is a bridge between game theory and practical application: you study how to play optimally in real-life games where players deviate significantly from equilibrium.

---

## The 3-Tab Nodelocking Interface

To access nodelocking, solve a custom solution via GTO Wizard AI, then hover over any decision point and select the lock icon. The interface presents three primary tabs.

### Tab 1: Set Strategy

```
+----------------------------------------------------------------+
|                    SET STRATEGY TAB                              |
+----------------------------------------------------------------+
|                                                                  |
|  Action Selector:  [Fold] [Check] [Bet 33%] [Bet 75%] [All-in] |
|                                                                  |
|  +------------------------------------------------------------+ |
|  |   A   K   Q   J   T   9   8   7   6   5   4   3   2       | |
|  | A [x] [x] [x] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | K [x] [x] [x] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | Q [ ] [ ] [x] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | J [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | ...                                                         | |
|  |   (Paint hands to apply selected action)                    | |
|  +------------------------------------------------------------+ |
|                                                                  |
|  [Filters]  -- Apply by hand category (pairs, suited, etc.)     |
|  [Nodelock] -- Submit and re-solve                               |
+----------------------------------------------------------------+
```

This tab resembles the Range Builder tool. The workflow is:

1. **Choose an action** from the top-right dropdown (fold, check, specific bet size, etc.)
2. **Paint over hands** on the matrix to assign that action to specific combos
3. **Use Filters** to apply strategies to hand categories (all pairs, all broadway, etc.) rather than individual combos
4. The system intelligently applies strategies to all **strategically identical combinations** automatically (e.g., locking A-heart K-heart to bet also locks A-spade K-spade if they are strategically equivalent on the given board)

### Tab 2: Set Frequency

```
+----------------------------------------------------------------+
|                   SET FREQUENCY TAB                              |
+----------------------------------------------------------------+
|                                                                  |
|  Mode: [Overwrite Unlocked] / [Overwrite All]                   |
|                                                                  |
|  Action          Current     Target    Slider                    |
|  ---------       -------     ------    ------                    |
|  Fold              15%   -->  [40%]    |====---------|            |
|  Check             30%   -->  [30%]    |======-------|            |
|  Bet 33%           35%   -->  [20%]    |====---------|            |
|  Bet 75%           20%   -->  [10%]    |==-----------|            |
|                                                                  |
|  (Drag sliders or type exact percentages)                        |
|                                                                  |
|  Note: Does NOT auto-lock. Lock manually before submitting.      |
+----------------------------------------------------------------+
```

This tab enables sweeping adjustments to overall action frequencies. Two modes are available:

- **Overwrite Unlocked**: Adjusts only hands that are not locked -- useful when you want to preserve specific locked strategies while modifying the rest
- **Overwrite All**: Adjusts all hands regardless of lock status -- useful for broad population-level adjustments

When you change frequencies, the solver decides *which* hands to reassign based on **EV loss calculations**. For example, if you increase fold frequency from 15% to 40%, the solver prioritizes folding the weakest holdings first -- those with the least EV loss from folding.

**Important**: Set Frequency does not automatically lock the adjusted hands. You must manually lock them in the Lock/Unlock tab before submitting the nodelock, or the solver will simply re-optimize those hands back toward GTO.

### Tab 3: Lock/Unlock

```
+----------------------------------------------------------------+
|                  LOCK / UNLOCK TAB                               |
+----------------------------------------------------------------+
|                                                                  |
|  [Lock All]  [Unlock All]                                        |
|                                                                  |
|  +------------------------------------------------------------+ |
|  |   A   K   Q   J   T   9   8   7   6   5   4   3   2       | |
|  | A [L] [L] [L] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | K [L] [L] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | Q [L] [ ] [L] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | J [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ] [ ]     | |
|  | ...                                                         | |
|  |   L = Locked (strategy fixed)                               | |
|  |   [ ] = Unlocked (solver can optimize)                      | |
|  +------------------------------------------------------------+ |
|                                                                  |
|  [Nodelock] -- Submit locked strategy for re-solving             |
+----------------------------------------------------------------+
```

This tab manages which hands retain their assigned strategy versus which hands the solver can freely optimize. Options include:

- **Lock All**: Freeze the entire range so the solver cannot change any hand's strategy
- **Unlock All**: Allow the solver full freedom to optimize all hands
- **Manual toggling**: Click individual hands or hand categories to toggle lock/unlock status

The partial-locking approach is powerful: lock premium hands to specific actions while allowing marginal hands to adapt, balancing exploitation with safety.

---

## How the Solver Re-Equilibrates

When you submit a nodelocked strategy, the following process occurs:

1. **Locked hands play exactly as specified** -- their strategy is fixed and cannot change
2. **The opposing player's strategy re-optimizes** -- the solver computes the best response to the locked strategy
3. **Unlocked hands of the locked player also re-optimize** -- to minimize EV losses from the locked positions

This creates a new equilibrium that accounts for the deviation. The solver finds the best way to play the rest of the game tree given the constraint you imposed.

### Cascade Effects

A critical insight: **adjustments often occur elsewhere on the game tree**. When you lock one player to check more frequently, the counter-adjustment might not appear where you expect. For example:

```
EXAMPLE: Locking UTG to check flop more often (74% bet --> 31% bet)

Expected Response:           Actual Response:
BB adjusts to UTG checks     BB starts LEADING (betting first)
                              BB narrows calling ranges
                              BB increases raising frequencies
                              BB initiates action preemptively
```

The solver's response to a locked strategy may manifest as changes in:
- **Previous streets** -- adjusting opening ranges to prevent exploitation
- **Different branches** -- initiating action rather than responding
- **Sizing changes** -- shifting from large bets to small bets or vice versa
- **Range composition** -- fundamentally changing which hands take which lines

---

## Compare Nodes Feature

After nodelocking, the Compare Nodes tool provides side-by-side analysis of strategies before and after the lock. Access it by selecting "Compare Nodes" at any decision point.

```
+----------------------------------------------------------------+
|                    COMPARE NODES                                 |
+----------------------------------------------------------------+
|                                                                  |
|   Left Panel (Current)     |    Right Panel (Previous)           |
|   -----------------------  |    --------------------------       |
|   Post-Nodelock Strategy   |    Original GTO Strategy            |
|                            |                                     |
|   Bet 33%:  45%            |    Bet 33%:  35%                    |
|   Bet 75%:  15%            |    Bet 75%:  20%                    |
|   Check:    25%            |    Check:    30%                    |
|   Fold:     15%            |    Fold:     15%                    |
|                            |                                     |
|   EV: 12.3bb               |    EV: 11.8bb                       |
|                            |                                     |
+----------------------------------------------------------------+
|                                                                  |
|  View By: [Hand Category] [Equity Buckets] [Draws]               |
|  Metric:  [Strategy] [Expected Value] [Equity]                   |
+----------------------------------------------------------------+
```

The tool displays:
- **Strategy comparison**: Current locked solution vs. original GTO solution
- **Expected Value shifts**: Quantify how much EV the exploit generates
- **Equity changes**: How equity distributions shift
- **Equity Realization**: How effectively equity converts to actual value
- **Breakdown options**: View by equity buckets, hand category, or draw status

This is particularly useful for gauging exploitability -- comparing the EV before and after a nodelock reveals exactly how profitable the adjustment is.

---

## Practical Applications

### Exploiting Population Tendencies

Lock the opponent to common population tendencies and solve for the counter:

| Population Tendency            | How to Lock                          | Expected Counter                    |
|-------------------------------|--------------------------------------|-------------------------------------|
| Folds too much to c-bets      | Increase opponent fold frequency     | Bluff more, value bet thinner       |
| Never folds the river          | Set fold frequency to 0% on river    | Never bluff river, value bet wide   |
| Overbets too frequently        | Lock opponent to overbet lines       | Call wider with medium-strength     |
| Checks back too often IP       | Increase check frequency for IP      | Lead out more from OOP             |
| 3-bets too tight               | Narrow opponent's 3-bet range        | Open wider, fold less to 3-bets    |

### Hand Category Locking

Rather than locking individual combos, lock entire categories for faster, broader adjustments. For example, lock all pairs to a specific action, or lock all suited connectors to call.

### Studying Specific Opponents

If you have hand history data on a specific opponent:
1. Identify their key deviations from GTO
2. Lock those deviations into the solver
3. Generate the optimal counter-strategy
4. Practice implementing it in Practice Mode

### Verifying "Rules of Thumb"

Test whether common strategic maxims hold. For example, does "never fold to a river bet with top pair" actually cost EV? Lock that strategy and measure the difference.

### Worked Example: Opponent Never Folds River

A common population tendency is calling too much on the river. Here is how to study the exploit:

```
+--------------------------------------------------------------+
|        NODELOCKING WORKED EXAMPLE                             |
+--------------------------------------------------------------+
|                                                                |
|  Spot: BTN vs BB, single-raised pot, river decision            |
|  Board: K-heart 9-spade 4-club 2-diamond 7-heart              |
|  BB faces a 75% pot bet from BTN                               |
|                                                                |
|  Step 1: Open the nodelocking interface at BB's river node     |
|                                                                |
|  Step 2: Set Frequency tab                                     |
|          - Change BB fold frequency from 40% --> 10%           |
|          - Mode: Overwrite All                                 |
|                                                                |
|  Step 3: Lock/Unlock tab                                       |
|          - Lock All (freeze BB's adjusted strategy)            |
|                                                                |
|  Step 4: Click "Nodelock" to re-solve                          |
|                                                                |
|  Step 5: Examine BTN's adjusted strategy:                      |
|          - Bluffing frequency drops significantly               |
|          - Value betting range widens (thin value)              |
|          - Overall EV increases vs. GTO baseline               |
|                                                                |
|  Step 6: Compare Nodes to quantify the EV gain                 |
|          - BTN EV before lock: 8.2bb                           |
|          - BTN EV after lock:  9.7bb                           |
|          - Exploit value: +1.5bb per hand in this spot         |
+--------------------------------------------------------------+
```

This workflow demonstrates how a single population tendency (calling too much) creates measurable exploitative opportunity. The Compare Nodes feature quantifies exactly how much the exploit is worth, helping players prioritize which adjustments to implement at the table.

---

## Nodelocking 2.0

GTO Wizard released an upgraded version of nodelocking ("Nodelocking 2.0") featuring:
- Smarter, more balanced deviation algorithms
- Improved frequency-locking calculations
- Enhanced interface for exploring exploitative strategies against specific opponent tendencies
- Integration with 3-way solving for multiway nodelocking

---

## Limitations and Cautions

### One-Street Horizon

GTO Wizard AI solves one street at a time. When you lock a flop strategy, the solver assumes perfect play on turn and river. This means it can exploit current-street mistakes but cannot anticipate later-street errors.

### Interconnected Adjustments

Locking one node creates cascading effects throughout the game tree. Always examine multiple branches after locking -- the most impactful adjustment might be somewhere unexpected.

### Over-Exploitation Risk

A maximally exploitative strategy against one opponent is itself exploitable if that opponent adjusts. Use nodelocking as a study tool to understand directional adjustments rather than memorizing exact frequencies.

### Solver Salvaging

Before and after a lock, the solver tries to "salvage the ship" by playing as close to GTO as possible. The adjustment to a leak may happen on a different part of the game tree than expected. This is not a bug -- it reflects how interconnected poker strategy truly is.

---

## Screenshot References

> **Screenshot references (Set Strategy):** See `screenshots/blog/nodelocking-interface.png`, `screenshots/blog/nodelocking-set-strategy.gif`

> **Screenshot references (Set Frequency):** See `screenshots/blog/nodelocking-set-frequency.png`, `screenshots/blog/nodelocking-frequency-slider.gif`

> **Screenshot references (Strategy visualization):** See `screenshots/blog/nodelocking-strategy-viz.png`

> **Screenshot references (Nodelocking 2.0):** See `screenshots/blog/redesigned-nodelocking-handclass.png`

---

## Sources

- https://blog.gtowizard.com/introducing-nodelocking/
- https://blog.gtowizard.com/the-limits-of-nodelocking/
- https://help.gtowizard.com/how-to-use-nodelocking/
- https://blog.gtowizard.com/now_live_3_way_solving_nodelocking_2_0_and_50k_icm_ft_sims/
- https://blog.gtowizard.com/how-to-adjust-when-villain-has-no-bluffs/
- https://pages.gtowizard.com/glossary/nodelock/

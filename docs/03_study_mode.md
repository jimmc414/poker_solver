# GTO Wizard Study Mode

## Overview

Study Mode is the primary analytical workspace in GTO Wizard, providing deep access to pre-computed Game Theory Optimal (GTO) solutions. It allows players to explore solver outputs across four interconnected tabs: **Strategy**, **Ranges**, **Breakdown**, and **Reports**. Each tab offers a different lens for examining the same underlying equilibrium data, enabling players to understand not just what the GTO strategy is, but why it takes the shape it does.

Study Mode is accessed via the main navigation bar and presents solutions for any supported game format (Cash, MTT, Spin & Go, Heads-Up SNG). The interface is organized around a central strategy matrix with surrounding contextual panels.

---

## Navigation and the Infobox

### Spot Selector

At the top of the Study Mode interface, a series of dropdown menus allow sequential selection of positions and preflop actions. Players build a game tree path by choosing:

1. Game format and configuration (e.g., Cash 6max 100bb)
2. Preflop positions and actions (e.g., UTG opens, BTN 3-bets, UTG calls)
3. Postflop streets and actions (flop, turn, river nodes)

A shortcut popup (hotkey **J**) opens a table enabling direct selection of any position and action sequence without navigating dropdown by dropdown.

### Infobox Panel

The infobox sits beside the strategy matrix and displays critical context for the current node:

| Field            | Description                                              |
|------------------|----------------------------------------------------------|
| Positions        | Both players shown, active player highlighted            |
| Pot Size         | Current pot in big blinds                                |
| Board Cards      | Community cards dealt so far                             |
| Stack Depth      | Effective stacks remaining                               |
| Bet Amounts      | Current bet/raise sizes on the table                     |
| Pot Before/After | Shows pot before and after the most recent bet           |
| Pot Odds         | Required calling frequency based on bet-to-pot ratio     |

```
+-------------------------------------------------------+
|  INFOBOX                                              |
|                                                       |
|  Positions:  [BTN] vs BB     (BTN highlighted)        |
|  Pot:        6.5 BB                                   |
|  Board:      Ks 9h 4d                                 |
|  Stacks:     96.75 BB effective                       |
|  Last Bet:   BTN bets 4.3 BB (66% pot)               |
|  Pot Odds:   BB needs 30% equity to call              |
+-------------------------------------------------------+
```

### Actions Table

Located below the infobox, the Actions Table displays the range-wide action distribution for the active player at the current node:

```
+-------------------------------------------+
|  ACTIONS TABLE                            |
|                                           |
|  Check     [========     ]  42.3%         |
|  Bet 33%   [=====        ]  21.8%         |
|  Bet 75%   [======       ]  26.1%         |
|  Bet 150%  [==           ]   9.8%         |
|                                           |
|  [Toggle: % / BB]  (Spacebar)             |
+-------------------------------------------+
```

- Clicking an action in this table filters the strategy matrix to show only hands taking that action
- Pressing **S** groups similar bet sizes together (e.g., "small bet" vs "large bet")
- **Spacebar** toggles between percentage display and BB amounts

---

## Tab 1: Strategy Tab

The Strategy Tab is the default view and the most frequently used interface. It consists of four interconnected areas: the **Strategy Matrix**, the **Hand Matrix**, the **Actions Table**, and the **Infobox**.

### Strategy Matrix (13x13 Hand Grid)

The strategy matrix displays a standard 13x13 poker hand grid where each cell represents a hand combination (e.g., AKs, QJo, 77). Suited hands appear above the diagonal, offsuit hands below, and pocket pairs along the diagonal.

```
+----+----+----+----+----+----+----+----+----+----+----+----+----+
| AA | AKs| AQs| AJs| ATs| A9s| A8s| A7s| A6s| A5s| A4s| A3s| A2s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|AKo | KK | KQs| KJs| KTs| K9s| K8s| K7s| K6s| K5s| K4s| K3s| K2s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|AQo |KQo | QQ | QJs| QTs| Q9s| Q8s| Q7s| Q6s| Q5s| Q4s| Q3s| Q2s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|AJo |KJo |QJo | JJ | JTs| J9s| J8s| J7s| J6s| J5s| J4s| J3s| J2s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|ATo |KTo |QTo |JTo | TT | T9s| T8s| T7s| T6s| T5s| T4s| T3s| T2s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
| ...                    (continues to 22)                       |
+----+----+----+----+----+----+----+----+----+----+----+----+----+
```

### Color Coding System

Each cell in the strategy matrix is color-coded based on the GTO action for that hand. When a hand has a mixed strategy (multiple actions at different frequencies), the cell displays proportional color segments.

| Color          | Action           | Context                            |
|----------------|------------------|------------------------------------|
| Red / Orange   | Bet / Raise      | Aggressive actions                 |
| Green          | Call / Check     | Passive actions                    |
| Blue           | Fold             | Surrendering the hand              |
| Black          | Not in range     | Hand was folded earlier            |
| Mixed colors   | Mixed strategy   | Multiple actions at GTO frequencies|

For **mixed strategies**, a cell might show 60% red and 40% green, indicating the solver bets this hand 60% of the time and checks 40% of the time. The proportional color segments within each cell visually communicate the frequency split.

When multiple bet sizes are available, different shades or distinct colors differentiate between sizes (e.g., lighter red for a small bet, darker red for an overbet).

### Hand Hovering and the Hand Matrix

Hovering over any cell in the strategy matrix reveals detailed information in the **Hand Matrix** panel (bottom-right corner). This panel shows:

- The specific hand combination (e.g., AKs)
- All 4 suit combinations for suited hands (or 12 for offsuit hands)
- The GTO action breakdown for each individual suit combination
- Frequency weights for each action

```
+----------------------------------------------+
|  HAND MATRIX: AKs                            |
|                                              |
|  A♠K♠:  Bet 75% (58%)  |  Check (42%)       |
|  A♥K♥:  Bet 75% (62%)  |  Check (38%)       |
|  A♦K♦:  Bet 75% (55%)  |  Check (45%)       |
|  A♣K♣:  Bet 75% (61%)  |  Check (39%)       |
|                                              |
|  Average: Bet 75% (59%) | Check (41%)        |
+----------------------------------------------+
```

This granularity reveals suit-specific strategies, where blockers to flush draws or backdoor draws can shift optimal frequencies.

### Metric Selection Dropdown

A dropdown at the top-right of the strategy matrix enables viewing different metrics overlaid on the hand grid:

| Metric         | Description                                                      |
|----------------|------------------------------------------------------------------|
| Strategy       | Default GTO action color coding                                  |
| Strategy+EV    | Action colors with EV values overlaid                            |
| Strategy+EQ    | Action colors with equity percentages                            |
| EV             | Expected value heat map for each hand                            |
| Compare EV     | Color-coded EV difference between two actions                    |
| Equity         | Raw equity if all hands were checked down                        |
| EQR            | Equity Realization ratio                                         |
| Range weights  | Frequency of each hand within the player's range                 |
| All            | Displays all metrics simultaneously                              |

### Compare EV Feature

When comparing two actions, the grid uses a green-white-red gradient:
- **Green**: Higher EV for the selected action
- **White**: Marginal difference (close to indifferent)
- **Red**: Lower EV for the selected action

This is particularly useful for understanding which hands are clear bets vs. clear checks vs. true mixed strategies.

### View Options

A dropdown at the strategy matrix top-right provides visualization modes:

| View Mode    | Description                                                 |
|--------------|-------------------------------------------------------------|
| Range Height | Cell height proportional to hand's frequency in range       |
| Full Height  | All cells same height regardless of range frequency         |
| Normalized   | Adjusted proportional display                               |
| Horizontal   | Shows individual suit combinations within each cell         |

The **Horizontal** mode is especially useful for studying suit-specific strategies, as it expands each cell to show all combinations side by side.

### Filtering System

Study Mode provides multiple layers of filtering to isolate specific subsets of the range:

**Hand Category Filters**: Toggle hand classes (top pair, flush draw, gutshot, etc.) to filter the matrix. Action frequencies update dynamically to reflect only the filtered subset.

**Suit Filters**: A dropdown above the matrix enables inclusion/exclusion of specific suits. The top row filters the highest card's suit, and the bottom row filters the lowest card's suit. This is useful for isolating flush draw combos or suited connectors.

**Bet Filters**: Clicking actions in the Actions Table filters for hands taking those specific actions. Multiple filters can be combined.

**Clear Filters**: The **CLEAR** button (hotkey **P**) removes all active filters. Individual filters can also be removed via their "x" buttons.

```
+----------------------------------------------------+
|  FILTER BAR                                        |
|                                                    |
|  Hand Classes: [Top Pair] [Flush Draw] [Gutshot]   |
|  Suits: [♠] [♥] [♦] [♣]  x  [♠] [♥] [♦] [♣]     |
|  Actions: [Bet 75% selected]                       |
|                                                    |
|  [CLEAR ALL] (P)                                   |
+----------------------------------------------------+
```

---

## Tab 2: Ranges Tab

The Ranges Tab (hotkey **2**) provides a side-by-side comparison of both players' ranges at the current decision node. This enables direct analysis of range asymmetries and equity distributions.

### Layout

```
+---------------------------+---+---------------------------+
|       OOP RANGE           |   |       IP RANGE            |
|  (Out of Position)        |   |  (In Position)            |
|                           |   |                           |
|  +---+---+---+---+       |   |  +---+---+---+---+        |
|  |AA |AKs|AQs|...|       |   |  |AA |AKs|AQs|...|        |
|  +---+---+---+---+       |   |  +---+---+---+---+        |
|  |AKo|KK |KQs|...|       |   |  |AKo|KK |KQs|...|        |
|  +---+---+---+---+       |   |  +---+---+---+---+        |
|  |...|...|...|...|       |   |  |...|...|...|...|        |
|  +---+---+---+---+       |   |  +---+---+---+---+        |
|                           |   |                           |
|  Combos: 186              |   |  Combos: 234              |
|  Equity: 45.2%            |   |  Equity: 54.8%            |
|  EV: 3.24 BB              |   |  EV: 4.12 BB              |
|  EQR: 112%                |   |  EQR: 98%                 |
+---------------------------+---+---------------------------+
```

### Key Metrics

| Metric  | Description                                           | Dynamic? |
|---------|-------------------------------------------------------|----------|
| Combos  | Number of hand combinations in range (updates w/filter)| Yes     |
| Equity  | Equity vs. opposing range (static)                    | No       |
| EV      | Expected value at this node (static)                  | No       |
| EQR     | Equity Realization ratio (dynamic)                    | Yes      |

### Equity Distribution Graph

Below the range grids, an equity distribution graph plots every combo's equity:

- **Horizontal axis**: Percentile rank within the player's range (0-100%)
- **Vertical axis**: Equity percentage against the opposing range
- Each dot represents one hand combination, sorted by equity strength
- Click-and-drag zooming allows focusing on specific equity regions

```
  100% |           *  **
       |        * ** ****
  Eq%  |     *** *********
       |  ****  ***********
       | *** ****************
    0% +------------------------
       0%    25%   50%   75%  100%
             Range Percentile
```

### Range Copy Feature

Players can copy ranges in standard PioSolver/GTO+ text format:
- **Full range**: Click the copy button above either player's range
- **Partial range**: Use dropdown to copy a specific action's range (e.g., "betting range only")

### Filtering Options

**Equity Buckets**: Two modes:
- **Simple**: Fewer groupings (e.g., 0-33%, 33-66%, 66-100%)
- **Advanced**: Granular groupings (e.g., 0-10%, 10-25%, 25-50%, etc.)

**Hand & Draw Classifications**: Filter by hand class (top pair, two pair, etc.) or draw type (flush draw, straight draw). Multiple filters can be combined.

### Full-Screen Mode

Press **Q** or click the expand icon to enter full-screen range view for detailed comparison.

---

## Tab 3: Breakdown Tab

The Breakdown Tab (hotkey **3**) enables deep analysis of how GTO constructs its betting and checking ranges. It reveals the composition of each action, showing which hand categories make up value bets, bluffs, and checks.

### Core Purpose

The Breakdown Tab answers questions like:
- What hands comprise the overbetting range?
- What ratio of value to bluffs does each bet size use?
- How does the solver distribute different hand classes across actions?

### Key Metrics

| Metric        | Description                                                     |
|---------------|-----------------------------------------------------------------|
| Range%        | Proportion of combos in each hand category relative to total range |
| Performed%    | How often a hand category takes the selected action              |
| Represented%  | Share of each hand category within the filtered action selection  |

### Summary Graphs (Manhattan Plots)

The Breakdown Tab features Manhattan-style plots that visualize every hand's strategy sorted by equity:

```
  Bet 150% |     ####
  Bet 75%  |   ########  ##
  Check    | ##############  ####
  Fold     |                   ######
           +----------------------------
           Low Equity  -->  High Equity
```

These plots show:
- How actions distribute across the equity spectrum
- Polarity (value-heavy vs. balanced betting ranges)
- Threshold equity points where the strategy shifts
- How implied odds affect decisions (e.g., drawing hands calling with low equity)

### Hand Class Strategy Analysis

Hovering over the **Performed%** column reveals how different hand categories prefer specific actions. For example, in a spot where the solver uses two bet sizes:

```
+--------------------------------------------------+
|  BREAKDOWN: BTN c-bet on Ks 9h 4d               |
|                                                  |
|  Action     | Range% | Performed% | Represented%|
|  -----------|--------|------------|-------------|
|  Bet 175%   |        |            |             |
|    Top pair+ |  18%  |    72%     |    41%      |
|    Bluffs    |  32%  |    44%     |    59%      |
|  Bet 33%    |        |            |             |
|    Mid pair  |  14%  |    68%     |    38%      |
|    Draws     |  22%  |    51%     |    44%      |
|  Check      |        |            |             |
|    Air       |  28%  |    85%     |    62%      |
|    Showdown  |  16%  |    54%     |    38%      |
+--------------------------------------------------+
```

### Interactive Controls

- Click column headers to sort by any metric
- Press **S** to group similar bet sizes
- Toggle between "Strategy" view (action frequencies) and "All" view (equity/EV data)
- Expand category graphs by hovering over rows

---

## Tab 4: Reports Tab

The Reports Tab (hotkey **4**) provides aggregate data across all board textures for a given preflop scenario. This is the most powerful tool for identifying broad GTO heuristics and patterns.

### Aggregate Flop Reports

Flop reports analyze all **1,755 strategically distinct flops** simultaneously. The data is weighted by probability to reflect real-game distributions.

### Access Methods

1. Navigate to a specific flop via the solution browser, then click the Reports tab
2. Access directly from the main menu by selecting positions and preflop actions (SRP/3bet/4bet)

### Display Formats

| Format | Description                                              |
|--------|----------------------------------------------------------|
| Chart  | Bar graphs showing all flops or categories (default)     |
| Table  | Numerical precision with sortable columns               |

The Table view includes a "filtered" row comparing filtered data against the complete 1,755-flop dataset.

### Data Metrics

Reports display four primary measurements for every flop:
- **Strategy**: Action frequencies (bet/check/raise percentages)
- **EV**: Expected value for the active player
- **EQ**: Raw equity
- **EQR**: Equity realization ratio

### Filtering and Grouping

**Board Texture Filters**: Isolate specific flop characteristics:

| Filter Category | Examples                                    |
|-----------------|---------------------------------------------|
| Suits           | Monotone, two-tone, rainbow                 |
| Pairing         | Paired, unpaired, trips                     |
| Connectedness   | Disconnected, OESD-possible, connected      |
| High card       | Ace-high, King-high, low boards             |

**Flop Grouping Options**: Group flops by high card, suits, pairing status, or connectedness to reveal patterns.

**Action Grouping**: Organize bet sizes into:
- **All**: Every individual bet size shown
- **Grouped**: Small / Medium / Large / Overbet
- **Simple**: Bet or Check

### Aggregate Turn Reports

Turn reports display strategy data across all 52 possible turn cards for a given flop:

```
+---------------------------------------------------+
|  TURN REPORT: Ks 9h 4d (BTN vs BB SRP)           |
|                                                   |
|  Turn Card | Bet%  | Check% | Avg EV | Equity    |
|  ----------|-------|--------|--------|-----------|
|  A♠        | 72%   |  28%   | 4.21   |  56.3%    |
|  K♠        | 45%   |  55%   | 3.89   |  58.1%    |
|  Q♠        | 68%   |  32%   | 4.05   |  55.7%    |
|  ...       | ...   |  ...   | ...    |  ...      |
+---------------------------------------------------+
```

- Default view color-codes all 52 turn cards according to selected metrics
- Turn cards can be sorted by suits, specific cards, or actions
- Hole card filtering targets specific hand combinations rather than turn cards

---

## Summary Tab (Within Strategy Tab)

The Summary Tab provides a sortable table of every hand combination with comprehensive metrics:

| Column          | Description                                                    |
|-----------------|----------------------------------------------------------------|
| Hand            | The specific combination                                       |
| Strategy        | GTO action breakdown (color-coded bar)                         |
| Range Weight    | How frequent this hand is in the player's range                |
| EV              | Expected value in big blinds                                   |
| Equity          | Raw equity vs. opponent's range                                |
| EQR             | Equity Realization (actual EV / equity-implied EV)             |
| Value Removal   | 0-10 score: how much EV removed from top of opponent's range   |
| Trash Removal   | 0-10 score: how much the hand blocks opponent's weak hands     |

### Blocker Scores Explained

**Value Removal (0-10)**: Measures card removal effect on opponent's strong hands.
- Low score = Good for value betting (opponent still has strong hands to pay you off)
- High score = Good for bluffing (opponent's value range is diminished)

**Trash Removal (0-10)**: Measures card removal effect on opponent's weak hands.
- Low score = Good for bluffing (opponent still has many weak hands that fold)
- High score = Good for value betting (opponent has fewer trash hands to fold)

---

## Blockers Tab

The Blockers Tab shows how holding specific cards affects the opponent's action frequencies. Each card is displayed with its impact on the opponent's checking, betting, folding, and calling frequencies.

```
+------------------------------------------+
|  BLOCKERS: Impact on Villain's Actions   |
|                                          |
|  Card | Bet%  | Check% | Fold% | Call%  |
|  -----|-------|--------|-------|--------|
|  A♠   | -3.2% | +1.8%  | +1.4% | -0.1% |
|  K♠   | +2.1% | -1.3%  | -0.8% | +0.0% |
|  Q♠   | +0.5% | -0.2%  | -0.3% | +0.1% |
|  ...  |  ...  |  ...   |  ...  |  ...   |
+------------------------------------------+
```

Sortable by any header to identify the most impactful blockers for value betting, bluffing, or slow-playing decisions.

---

## Keyboard Shortcuts

| Key        | Action                                            |
|------------|---------------------------------------------------|
| 1          | Switch to Strategy Tab                            |
| 2          | Switch to Ranges Tab                              |
| 3          | Switch to Breakdown Tab                           |
| 4          | Switch to Reports Tab                             |
| J          | Open spot selector popup                          |
| P          | Clear all filters                                 |
| S          | Toggle action grouping / graphing                 |
| Q          | Full-screen mode (Ranges Tab)                     |
| Spacebar   | Toggle between % and BB display                   |

---

## Color Customization

The settings icon (top-right corner) enables theme selection and color customization. Players can choose from preset themes or create custom color schemes for the strategy matrix, allowing personalization of the action-to-color mapping.

---

## Study Mode Component Specifications

| Component          | Position      | Data Displayed                          | Interaction Model                |
|--------------------|---------------|-----------------------------------------|----------------------------------|
| Strategy Matrix    | Center        | 13x13 hand grid with action colors      | Hover, click to filter, dropdown |
| Hand Matrix        | Bottom-right  | Suit-specific action breakdowns          | Updates on hover                 |
| Infobox            | Top-right     | Pot, positions, board, stacks           | Read-only context                |
| Actions Table      | Below infobox | Range-wide action frequencies           | Click to filter, toggle display  |
| Filter Bar         | Above matrix  | Hand class, suit, action filters         | Toggle on/off, clear all         |
| Spot Selector      | Top           | Game tree path navigation               | Dropdowns + popup (J)            |
| Metric Dropdown    | Matrix corner | Strategy/EV/EQ/EQR/Compare EV          | Select metric to display         |
| View Dropdown      | Matrix corner | Range Height/Full Height/Normalized/Horiz| Select visualization mode       |
| Tab Bar            | Below nav     | Strategy/Ranges/Breakdown/Reports       | Click or hotkey 1-4              |

---

## Screenshot References

> **Screenshot references (Strategy matrix):** See `screenshots/website/ai-solver-default.avif`, `screenshots/website/library-of-solutions-default.avif`

> **Screenshot references (Aggregated reports tab):** See `screenshots/website/aggregated-reports-default.avif`, `screenshots/blog/aggregated-reports-flop-cbet.png`

> **Screenshot references (Third-party views):** See `screenshots/reviews/solvers-poker-main.png`, `screenshots/reviews/h2n-library.webp`

---

## Sources

- [Study Mode - GTO Wizard Help](https://help.gtowizard.com/study-mode/)
- [Ranges Tab - GTO Wizard Help](https://help.gtowizard.com/ranges-tab/)
- [Breakdown Tab - GTO Wizard Help](https://help.gtowizard.com/breakdown-tab/)
- [Aggregate Reports Guide - GTO Wizard Help](https://help.gtowizard.com/aggregate-reports-guide/)
- [How To Study GTO Solutions - GTO Wizard Blog](https://blog.gtowizard.com/how-to-study-gto-solutions/)
- [GTO Wizard - Official Site](https://gtowizard.com/)

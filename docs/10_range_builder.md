# GTO Wizard Range Builder

## Overview

The Range Builder is GTO Wizard's active learning tool for practicing range construction. Unlike the Trainer (which tests individual hand decisions), the Range Builder requires players to construct their **entire strategy for any spot** -- assigning actions and frequencies to every hand in their range -- then grades the result against the GTO solution.

This approach leverages retrieval practice principles: rather than passively studying solver output, players actively reconstruct strategies from memory, which strengthens recall and deepens strategic understanding. The Range Builder reveals blind spots and systemic weaknesses that are difficult to notice when making decisions one hand at a time.

The Range Builder is accessed via the **Practice** tab in the main navigation and is available for both pre-solved library spots and custom AI-generated solutions.

---

## Accessing the Range Builder

### From Practice Tab

1. Navigate to the **Practice** tab
2. Select **Range Builder** from the training options
3. Choose a solution set (e.g., Cash 6max 100bb)
4. Select a specific preflop action sequence
5. Configure difficulty and board settings
6. Click **START BUILDING**

### From Other Interfaces

- **Solution Browser**: Click the Range Builder icon to practice the current spot
- **Custom AI Solutions**: Generate a solution, save the spot, then access it through Range Builder's saved spots selector

---

## Configuration

### Pre-Build Settings

Before building a range, players configure:

| Setting          | Description                                                |
|------------------|------------------------------------------------------------|
| **Solution**     | Which pre-solved or custom solution set to use             |
| **Line**         | The preflop action sequence (e.g., UTG opens, BB calls)    |
| **Board**        | Specific board, random board, or board filter              |
| **Difficulty**   | Easy, Medium, or Hard (controls number of bet sizes)       |

### Difficulty Levels

| Level     | Bet Size Options                                          |
|-----------|-----------------------------------------------------------|
| **Easy**  | Fewer action choices (broad categories)                   |
| **Medium**| Moderate number of bet sizes                              |
| **Hard**  | All individual bet sizes available                        |

Players can switch difficulty levels at any time by pressing **S** or using the dropdown menu. The evaluation updates to reflect whichever grouping is currently selected.

### Board Selection

- **Specific board**: Choose exact flop/turn/river cards
- **Random board**: Let the system deal a random board
- **Board filters**: Apply texture filters (paired, monotone, connected, etc.)

```
+---------------------------------------------------+
|  RANGE BUILDER CONFIGURATION                      |
|                                                   |
|  Solution:    Cash 6max 100bb                     |
|  Line:        BTN opens, BB calls                 |
|  Board:       [Random] / [Specific] / [Filter]    |
|  Difficulty:  [Easy] [Medium] [Hard]              |
|                                                   |
|  [START BUILDING]                                 |
+---------------------------------------------------+
```

---

## The Range Editor Grid Interface

### 13x13 Hand Matrix

The Range Builder presents the standard 13x13 poker hand grid. Each cell represents a hand combination that may be in the player's range at this decision point.

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
|A9o |K9o |Q9o |J9o |T9o | 99 | 98s| 97s| 96s| 95s| 94s| 93s| 92s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A8o |K8o |Q8o |J8o |T8o |98o | 88 | 87s| 86s| 85s| 84s| 83s| 82s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A7o |K7o |Q7o |J7o |T7o |97o |87o | 77 | 76s| 75s| 74s| 73s| 72s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A6o |K6o |Q6o |J6o |T6o |96o |86o |76o | 66 | 65s| 64s| 63s| 62s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A5o |K5o |Q5o |J5o |T5o |95o |85o |75o |65o | 55 | 54s| 53s| 52s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A4o |K4o |Q4o |J4o |T4o |94o |84o |74o |64o |54o | 44 | 43s| 42s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A3o |K3o |Q3o |J3o |T3o |93o |83o |73o |63o |53o |43o | 33 | 32s|
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|A2o |K2o |Q2o |J2o |T2o |92o |82o |72o |62o |52o |42o |32o | 22 |
+----+----+----+----+----+----+----+----+----+----+----+----+----+
```

### View Options

A dropdown at the top-right of the grid controls visualization:

| View Mode      | Description                                                |
|----------------|------------------------------------------------------------|
| Range Height   | Cell height proportional to hand's weight in range         |
| Full Height    | All cells displayed at equal height                        |
| Normalized     | Adjusted proportional display                              |
| Horizontal     | Expands each cell to show individual suit combinations     |

**Horizontal mode** is especially recommended for range construction, as it allows seeing and painting individual suit combinations within each hand.

---

## Painting and Selecting Hands

### The Paintbrush Tool

Activating the paintbrush icon enables strategy assignment. Once active, clicking and dragging across cells assigns the currently selected action at the specified frequency.

### Selection Methods

| Method                | Description                                              |
|-----------------------|----------------------------------------------------------|
| **Click**             | Select/deselect a single hand                            |
| **Click and drag**    | Paint strategy across multiple hands simultaneously      |
| **Mouse wheel**       | Scroll over a hand to adjust its weight incrementally    |
| **Pin icon**          | Mouse over a combo, click pin to select it, then paint specific suits in the hand matrix (bottom-right) |
| **Category painting** | Paint entire hand categories (e.g., all flush draws) at once |

### Hand Pinning for Suit-Specific Strategies

When you mouse over an individual combo, a **pin icon** appears. Clicking it selects that combo and enables painting specific suit combinations in the hand matrix panel (bottom-right corner). This is essential for suit-specific strategies where, for example, the solver bets flush draw combos but checks non-flush-draw combos of the same hand.

```
+----------------------------------------------+
|  HAND MATRIX: AKs (pinned)                  |
|                                              |
|  A♠K♠:  [Bet 75%] painted                   |
|  A♥K♥:  [Check]   painted                   |
|  A♦K♦:  [Bet 75%] painted                   |
|  A♣K♣:  [Check]   painted                   |
+----------------------------------------------+
```

---

## Weight Assignment

### Setting Frequencies

Each hand can be assigned a weight from **0% to 100%** for each available action. The weight determines how often the hand takes that action.

### Weight Controls

| Control              | Description                                            |
|----------------------|--------------------------------------------------------|
| **Weight box**       | Direct numeric input (type a percentage)               |
| **Slider**           | Drag to adjust frequency                               |
| **Arrow buttons**    | Increment/decrement by preset steps                    |
| **Presets**          | Quick-set buttons on each action                       |
| **Mouse wheel**      | Scroll over a hand to adjust weight incrementally      |
| **Manual input**     | Type exact frequencies for precise control             |

### The From/To Slider

The **From** and **To** boxes (or the range slider) control how wide the range is on a linear scale. This enables quickly setting a range boundary (e.g., "top 60% of hands bet, bottom 40% check").

### Action Assignment Panel

```
+---------------------------------------------------+
|  ACTION ASSIGNMENT                                |
|                                                   |
|  Selected: Top Pair+ (18 combos)                  |
|                                                   |
|  Action        | Freq   | Slider                  |
|  --------------|--------|-------------------------|
|  Bet 150%      | [60%]  | [===========     ]      |
|  Bet 75%       | [25%]  | [=====           ]      |
|  Bet 33%       | [ 5%]  | [=               ]      |
|  Check         | [10%]  | [==              ]      |
|                                                   |
|  Weight: [100%]  From: [0]  To: [100]             |
+---------------------------------------------------+
```

---

## Locking and Protection

### Combo Locking

Clicking the **lock icon** on any hand freezes its assigned strategy. Locked hands will not be overwritten when painting entire categories or using broad selection tools.

This is crucial for workflow efficiency:
1. Carefully assign strategies to specific hands
2. Lock those hands
3. Paint broad categories over the rest without losing your precise work

```
+----+----+----+----+
| AA | AKs| AQs|... |   AA = locked (lock icon shown)
| 🔒 |    |    |    |   Painting "top pair" category won't affect AA
+----+----+----+----+
```

---

## Filters

### Suit Filters

A suit selector above the range grid allows including or excluding specific suits from view. This provides an alternative to manual locking and painting when constructing suit-dependent strategies.

The suit filter has two rows:
- **Top row**: Filters the highest card's suit
- **Bottom row**: Filters the lowest card's suit

### Hand Category Filters

Players can filter the display by hand categories (top pair, flush draw, gutshot, etc.) to focus construction on specific hand types.

### Equity Filters

Hands can be filtered by equity bucket, showing only hands within a specified equity range against the opponent's range.

```
+---------------------------------------------------+
|  FILTERS                                          |
|                                                   |
|  Suits: [♠] [♥] [♦] [♣]  High Card               |
|         [♠] [♥] [♦] [♣]  Low Card                 |
|                                                   |
|  Categories: [Top Pair] [Flush Draw] [OESD]       |
|              [Two Pair] [Set] [Air]               |
|                                                   |
|  Equity: [0%] ----[========]---- [100%]           |
+---------------------------------------------------+
```

---

## Reference Features During Building

### Opponent Range Viewer

An icon enables **side-by-side comparison of both players' ranges** for strategic context. This shows what hands the opponent can have without revealing the GTO frequencies for the hero's actions.

### Cheat Function (Lightbulb Icon)

The **lightbulb icon** displays overall range frequencies compared to correct GTO frequencies at the bottom of the Range Builder. This is useful for checking whether your aggregate frequencies are in the right ballpark before submitting.

```
+---------------------------------------------------+
|  FREQUENCY COMPARISON (Lightbulb)                 |
|                                                   |
|  Action    | Your Freq | GTO Freq | Difference    |
|  ----------|-----------|----------|---------------|
|  Bet 150%  |   12%     |   10%    |   +2%         |
|  Bet 75%   |   28%     |   32%    |   -4%         |
|  Bet 33%   |   18%     |   20%    |   -2%         |
|  Check     |   42%     |   38%    |   +4%         |
+---------------------------------------------------+
```

### Solution Browser Integration

A **hat icon** allows jumping directly to the Solution Browser for the current spot. This provides full access to the GTO solution in Study Mode for reference (see [Study Mode](03_study_mode.md)).

---

## Display and Focus Options

### Focus Mode

An icon (or hotkey **F**) expands the display and removes surrounding UI clutter, giving maximum space to the range grid for detailed work.

### Undo/Redo

| Control      | Shortcut   | Description                      |
|--------------|------------|----------------------------------|
| Undo         | Ctrl+X     | Reverse last action              |
| Redo         | Ctrl+Y     | Reapply undone action            |
| Clear All    | --         | Reset all assignments to blank   |

Top-left arrows also provide undo/redo navigation.

---

## Submission and Evaluation

### Grading System

After submitting a constructed range, GTO Wizard grades each hand based on the frequencies of the player's chosen actions compared to the GTO frequencies:

```
Score = f(player_frequencies, gto_frequencies)
```

Scores range from **0% to 100%**, though achieving a perfect score is described as "nearly impossible for a human" due to the precision required for mixed strategies across hundreds of combinations.

### Results Display

The evaluation screen presents both the player's constructed range and the GTO solution with a comparison view between them:

```
+---------------------------+---+---------------------------+
|    YOUR RANGE             |   |    GTO RANGE              |
|                           |   |                           |
|  +---+---+---+---+       | C |  +---+---+---+---+        |
|  |AA |AKs|AQs|...|       | O |  |AA |AKs|AQs|...|        |
|  +---+---+---+---+       | M |  +---+---+---+---+        |
|  |AKo|KK |KQs|...|       | P |  |AKo|KK |KQs|...|        |
|  +---+---+---+---+       | A |  +---+---+---+---+        |
|  |...|...|...|...|       | R |  |...|...|...|...|        |
|  +---+---+---+---+       | E |  +---+---+---+---+        |
|                           |   |                           |
|  Score: 78%               |   |  (Perfect GTO)            |
+---------------------------+---+---------------------------+
```

### Category-Based Breakdown

Results can be compared by three different groupings, each graded separately:

| Grouping        | Description                                              |
|-----------------|----------------------------------------------------------|
| **Hands**       | Individual hand combinations                             |
| **Draws**       | Hand categories by draw type (flush draw, OESD, etc.)   |
| **Equity Buckets** | Hands grouped by equity range against opponent       |

### Individual Hand Review

Players can click on any individual hand to see a detailed comparison of their assigned frequencies vs. GTO frequencies for that specific combo.

---

## Saving and Loading Ranges

### Range Export

The Range Builder provides the option to **copy constructed strategies as standard PioSolver/GTO+ text**. This enables players to import their constructed range into external solvers for further testing and analysis.

### Saving Custom Spots

For custom AI-generated solutions:
1. Generate the AI solution for a specific spot
2. Click the "Saved spots" icon
3. Enter a description and save the spot
4. Access the saved spot through Range Builder's saved spots selector for future practice

---

## Integration with Other Modes

### Practice Mode Connection

Drills can be created from Range Builder spots. When studying a specific spot in Range Builder, clicking the drill icon creates a Trainer drill for that same spot, enabling both range-level and hand-level practice of the same scenario.

### Study Mode Connection

The hat icon in Range Builder opens the corresponding spot in Study Mode's Solution Browser, providing full access to:
- Strategy Matrix with color-coded actions
- Ranges Tab for equity distribution analysis
- Breakdown Tab for hand class composition
- Reports Tab for aggregate board texture data

See [Study Mode](03_study_mode.md) and [Practice Mode](04_practice_mode.md) for details on these connected features.

---

## Keyboard Shortcuts

| Key        | Action                                       |
|------------|----------------------------------------------|
| F          | Toggle Focus Mode (expanded display)         |
| S          | Change difficulty grouping                   |
| Ctrl+X     | Undo last action                             |
| Ctrl+Y     | Redo last action                             |

Additional hotkeys are displayed within the interface for rapid workflow.

---

## Practical Tips

### Recommended Workflow

1. **Start with broad categories**: Assign actions to hand classes (top pair, draws, air) before refining individual combos
2. **Lock completed work**: After assigning precise strategies, lock those hands before painting broader categories
3. **Use the lightbulb**: Check aggregate frequencies against GTO before submitting to catch obvious imbalances
4. **Practice in progression**: Start at Easy difficulty and advance to Hard as accuracy improves
5. **Focus on conceptual accuracy**: Understanding which hand types take which actions matters more than exact percentage precision

### Best Use Cases

| Use Case                    | Description                                         |
|-----------------------------|-----------------------------------------------------|
| Learning preflop ranges     | Especially multi-action spots (open, 3-bet, call)   |
| Postflop frequency tuning   | C-bet ranges on different board textures             |
| Identifying blind spots     | Reveals categories where your intuition diverges from GTO |
| Pattern recognition         | Forces global thinking about range vs. hand-by-hand |

### Common Mistakes

1. **Ignoring suit-specific strategies**: The solver often has different actions for different suit combos of the same hand (due to blockers, flush draws)
2. **Over-betting or under-bluffing**: Players typically include too many value hands and too few bluffs in aggressive actions
3. **Forgetting to lock**: Painting broad categories over carefully assigned hands
4. **Skipping the lightbulb check**: Submitting without comparing aggregate frequencies

---

## Range Builder Component Specifications

| Component             | Position       | Data Displayed                             | Interaction Model              |
|-----------------------|----------------|--------------------------------------------|--------------------------------|
| Hand Grid             | Center         | 13x13 matrix with assigned action colors   | Click, drag, scroll to paint   |
| Hand Matrix           | Bottom-right   | Suit-specific combo breakdown              | Pin, paint individual suits    |
| Action Panel          | Right sidebar  | Action buttons, frequency sliders/inputs   | Click action, adjust frequency |
| Weight Box            | Action panel   | Current frequency percentage               | Type, scroll, arrows           |
| Suit Filter           | Above grid     | Suit inclusion/exclusion toggles           | Click to toggle                |
| Category Filter       | Above grid     | Hand class toggles                         | Click to toggle                |
| Lock Button           | Per-combo      | Lock/unlock individual combos              | Click to toggle                |
| Paintbrush Button     | Toolbar        | Enable/disable painting mode               | Click to toggle                |
| Lightbulb Button      | Toolbar        | Show aggregate frequency comparison        | Click to show/hide             |
| Opponent Range Button | Toolbar        | Side-by-side range comparison              | Click to show/hide             |
| Solution Browser Link | Toolbar        | Jump to Study Mode for current spot        | Click to navigate              |
| Focus Mode Button     | Toolbar        | Expand display, remove clutter             | Click or press F               |
| Undo/Redo Arrows      | Top-left       | Navigate edit history                      | Click or Ctrl+X/Y             |
| Difficulty Selector   | Top            | Easy/Medium/Hard toggle                    | Dropdown or press S            |
| Spot Selector         | Top-left       | Solution and line selection                | Dropdown menus                 |
| Score Display         | Post-submit    | Overall score 0-100%                       | Read-only                      |
| Comparison View       | Post-submit    | Side-by-side Your Range vs GTO            | Click hands for detail         |

---

## Screenshot References

> **Screenshot references (Range builder grid):** See `screenshots/reviews/h2n-practice.webp`

> **Screenshot references (Nodelocking set strategy, similar grid):** See `screenshots/blog/nodelocking-interface.png`

---

## Sources

- [How To Use The Range Builder - GTO Wizard Help](https://help.gtowizard.com/how-to-use-the-range-builder/)
- [How To Use Range Builder in GTO Wizard - Blog](https://blog.gtowizard.com/how-to-use-range-builder-in-gto-wizard-to-improve-your-game/)
- [Study Mode - GTO Wizard Help](https://help.gtowizard.com/study-mode/)
- [How To Build Custom Solutions - GTO Wizard Help](https://help.gtowizard.com/how-to-build-custom-solutions/)
- [Practice Mode Overview - GTO Wizard Help](https://help.gtowizard.com/practice-mode-overview/)

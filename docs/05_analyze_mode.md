# GTO Wizard Analyze Mode

## Overview

Analyze Mode (also called the Hand History Analyzer) is GTO Wizard's tool for reviewing real poker sessions against GTO standards. Players upload hand histories from their online poker sessions, and the system automatically compares every decision against pre-computed solver solutions, identifying blunders, mistakes, inaccuracies, and correct plays with precise EV loss calculations.

The analyzer takes the grunt work out of hand review by automatically analyzing and sorting hand histories. Players can submit a single hand, full sessions, or an entire database for analysis. Once analyzed, results are presented in a sortable table that enables rapid identification of the biggest leaks.

---

## Hand History Upload Process

### Accessing the Upload Interface

The upload function is accessed via the **Upload** button located in the top-right corner of the Analyze Mode interface. Clicking it opens the upload dialog.

### Upload Methods

**Method 1 - Single Hand Upload**:
1. Select the "Single Hand" tab in the upload dialog
2. Paste the hand history text directly (right-click paste)
3. Click "Analyze" to process

**Method 2 - Batch Upload (Files/Folders)**:
1. Select the "Files" or "Folder" button at the bottom of the dialog
2. Drag and drop files into the analyzer, or use the file browser
3. Supports entire sessions or complete hand history databases

```
+---------------------------------------------------+
|  UPLOAD DIALOG                                    |
|                                                   |
|  [Single Hand]  [Files]  [Folder]                 |
|                                                   |
|  +---------------------------------------------+ |
|  |                                             | |
|  |     Drag and drop files here                | |
|  |                                             | |
|  |     or click [Browse] to select             | |
|  |                                             | |
|  +---------------------------------------------+ |
|                                                   |
|  Solution Format: [Cash 6max 100bb    v]          |
|                                                   |
|  [UPLOAD & ANALYZE]                               |
+---------------------------------------------------+
```

### Pre-Upload Configuration

Before uploading, players must select a **solution format** in the Analyzer settings tab. This tells the system which solver configuration to compare against (e.g., Cash 6max 100bb, MTT 8max, Spin & Go).

### Supported Poker Sites

GTO Wizard accepts hand histories from **17+ online poker platforms**:

| Site           | Site           | Site           |
|----------------|----------------|----------------|
| 888            | Chico          | Coin Poker     |
| GG Poker       | Ignition       | iPoker         |
| PartyPoker     | PokerBros      | PokerMaster    |
| PokerStars     | PokerTime      | PPPoker        |
| Unibet         | UPoker         | WePoker        |
| Winamax        | WPN (ACR)      |                |

If a poker site is not on the supported list, hand histories exported from poker trackers/HUDs (such as PokerTracker or Hold'em Manager) may still be compatible.

### Supported Game Formats

| Format                | Description                           |
|-----------------------|---------------------------------------|
| Cash 6max             | Standard 6-handed cash game           |
| Cash 6max straddle+ante | 6-handed with straddle and ante    |
| Cash 8max straddle+ante | 8-handed with straddle and ante    |
| Cash Heads-Up         | Heads-up cash game                    |
| MTT 8max              | Multi-table tournament (8-handed)     |
| Spin & Go             | 3-handed hyper-turbo SNG              |
| HU SNG                | Heads-up sit and go                   |

### Upload Requirements

- Table must have **3 to 6 players** (for supported formats)
- Hero cards must be fully visible
- Hand history must not exceed character limits
- File must be from a supported site or exported in compatible format

---

## Processing Pipeline

### Status Indicators

After upload, the system tracks each batch through five processing stages:

| Status         | Description                                            |
|----------------|--------------------------------------------------------|
| **Uploading**  | Files being transferred to GTO Wizard servers          |
| **In Queue**   | Waiting for processing (during traffic spikes)         |
| **Processing** | Actively analyzing hands against solver solutions      |
| **Analyzed**   | Complete and ready for review                          |
| **Errors**     | Unreadable files or technical failures                 |

Players do not need to wait for processing to complete. They can navigate away and return later without affecting processing speed.

### Hand Classification on Upload

The system categorizes uploaded hands:

| Category           | Description                                          |
|--------------------|------------------------------------------------------|
| **Total hands**    | Complete count of hands in the uploaded file(s)      |
| **Duplicate hands**| Identical hands appearing more than once             |
| **Unsupported**    | Multiway postflop spots or incompatible formats      |
| **Errors**         | Unreadable or corrupted hand entries                 |

```
+---------------------------------------------------+
|  UPLOAD STATUS                                    |
|                                                   |
|  Session: 2024-01-15_PokerStars.txt               |
|  Status:  [====== Analyzed ======]                |
|                                                   |
|  Total Hands:     842                             |
|  Analyzed:        687                             |
|  Duplicates:       23                             |
|  Unsupported:     118  (multiway postflop)        |
|  Errors:           14                             |
|                                                   |
|  [VIEW ANALYZED HANDS]                            |
+---------------------------------------------------+
```

---

## Analysis Methodology

### How GTO Wizard Analyzes Hands

The analysis workflow operates differently across streets:

| Street        | Analysis Method                                                |
|---------------|----------------------------------------------------------------|
| **Preflop**   | Compared against pre-solved solutions                          |
| **Flop**      | Compared against pre-solved solutions                          |
| **Turn**      | Compared against pre-solved solutions                          |
| **River**     | Calculated using actual stack depth and bet sizes from the hand, with ranges from presolved solutions |

The system **automatically selects the closest rake structure and stack size** based on the hand history information. This matching process ensures the comparison is as relevant as possible to the actual game conditions.

### Limitations

- **Multiway postflop spots** are ignored (no solver solutions available)
- **Partial solutions**: When a player takes an action that is never part of GTO strategy, the hand may have a "partial solution" flag, as the solver did not compute responses to that action
- **Stack depth approximation**: The closest available pre-solved stack depth is used, which may not exactly match the hand

---

## Blunder, Mistake, and Inaccuracy Detection

### Classification System

Every decision in an analyzed hand is classified using a color-coded system:

| Classification  | Color  | Symbol | Criteria                                                |
|-----------------|--------|--------|---------------------------------------------------------|
| **Correct**     | Green  | Check  | Action matches GTO strategy at meaningful frequency     |
| **Inaccuracy**  | Yellow | Dot    | Action taken less than 3.5% of the time in GTO solution |
| **Mistake**     | Red    | Dot    | Action not present in GTO strategy                      |
| **Blunder**     | Red    | X      | Action never taken in GTO AND causes significant EV loss|

### Visual Indicators in the Hand Table

```
+------------------------------------------------------------------+
|  ANALYZED HANDS TABLE                                            |
|                                                                  |
|  # | Hand   | Position | EV Loss | Status  | Street | Action    |
|  --|--------|----------|---------|---------|--------|-----------|
|  1 | As Kh  | BTN      | -1.24BB | Blunder | Flop   | Check     |
|  2 | Jd Tc  | CO       | -0.45BB | Mistake | Turn   | Bet 33%   |
|  3 | 9s 8s  | BB       | -0.12BB | Inacc.  | River  | Call      |
|  4 | Ah Qd  | BTN      |  0.00BB | Correct | Preflop| 3-Bet     |
|  5 | 7c 6c  | SB       | -0.87BB | Blunder | Flop   | Fold      |
|  ...                                                             |
+------------------------------------------------------------------+
```

---

## EV Loss Calculation

### Per-Decision EV Loss

For every decision point in a hand, GTO Wizard calculates the **EV loss** as the difference between the EV of the player's chosen action and the EV of the GTO-optimal action:

```
EV Loss = EV(GTO Action) - EV(Player's Action)
```

This is measured in **big blinds (BB)** and represents how much expected value the player sacrificed at that specific decision point.

### Aggregate Metrics

| Metric                     | Description                                             |
|----------------------------|---------------------------------------------------------|
| **Total EV Loss**          | Sum of all EV losses across all analyzed hands          |
| **Avg EV Loss per Hand**   | Total EV loss / number of hands analyzed                |
| **Avg EV Loss per Mistake**| Total EV loss / number of non-correct decisions         |
| **EV Loss as % of Pot**    | Loss relative to pot size at the decision point         |

The **EV loss as percentage of pot** metric was added in Analyzer 2.0 to enable better comparison of accuracy across different pot sizes. A 0.5 BB mistake in a 2 BB pot is far more significant than a 0.5 BB mistake in a 50 BB pot.

---

## Comparison: Player Action vs. GTO Action

### Strategy Table View

When reviewing an individual hand, the strategy table displays:

```
+---------------------------------------------------+
|  HAND REVIEW: As Kh  (BTN vs BB, Flop)           |
|                                                   |
|  Board: Qs 9h 4d        Pot: 6.5 BB              |
|                                                   |
|  GTO Strategy:                                    |
|    Bet 75%:   52% frequency   EV: 4.21 BB        |
|    Bet 33%:   31% frequency   EV: 4.18 BB        |
|    Check:     17% frequency   EV: 3.95 BB        |
|                                                   |
|  Your Action: Check  (EV: 3.95 BB)               |
|  Best Action: Bet 75% (EV: 4.21 BB)              |
|                                                   |
|  EV Loss: -0.26 BB                               |
|  Classification: Inaccuracy (yellow)              |
|                                                   |
|  Note: Check is taken 17% in GTO, not a blunder  |
|  but not the highest-EV action.                   |
+---------------------------------------------------+
```

The strategy table shows EV loss in the action breakdown, with the player's actual action highlighted. Mistakes and blunders are colored **red**, inaccuracies are colored **yellow**, and correct moves are **green**.

---

## Session Review and Filtering

### Analyzer 2.0 Interface

The updated analyzer (HH Analyzer 2.0) features a comprehensive filtering system for reviewing analyzed hands:

### Available Filters

| Filter Category       | Options                                               |
|-----------------------|-------------------------------------------------------|
| **Date Range**        | Custom start and end dates                            |
| **Preflop Action**    | RFI, 3-bet, call, fold, etc.                         |
| **Game Type**         | Cash, MTT, Spin, HU SNG                              |
| **Position**          | UTG, HJ, CO, BTN, SB, BB                            |
| **Preflop Aggression**| Passive, aggressive, very aggressive                  |
| **EV Loss Threshold** | Minimum EV loss to display                            |
| **Session**           | Filter by specific upload session                     |
| **Hand Status**       | OK (has GTO solution), partial, unsupported           |
| **Postflop Criteria** | Board texture, hand strength, draw type               |

### Sorting Options

The hands table supports multiple sort orders:

| Sort By             | Use Case                                               |
|---------------------|--------------------------------------------------------|
| **EV Loss (desc)**  | Find biggest blunders first (most common workflow)     |
| **Avg EV Loss**     | Identify patterns of consistent mistakes               |
| **BB Won/Lost**     | Review biggest winning and losing pots                 |
| **Date**            | Chronological review                                   |
| **Position**        | Analyze positional weaknesses                          |

### Saved Reports

Players can save custom filter configurations as named reports. When new hands are uploaded, saved reports automatically update with the new data, enabling consistent performance tracking without reapplying filters.

```
+---------------------------------------------------+
|  FILTER BAR                                       |
|                                                   |
|  Date: [Jan 1] to [Jan 31]                        |
|  Position: [BTN] [BB]                             |
|  Action: [3-Bet Pots]                             |
|  EV Loss: [> 0.5 BB]                              |
|  Status: [OK only]                                |
|                                                   |
|  Saved Reports: [BTN 3-Bet Leaks v] [Save New]   |
+---------------------------------------------------+
```

---

## Hand Replay Interface

### Replay Controls

Clicking any analyzed hand opens the replayer, which provides a street-by-street walkthrough of the hand with GTO annotations:

```
+---------------------------------------------------+
|  HAND REPLAYER                                    |
|                                                   |
|  Hand: As Kh   Position: BTN vs BB               |
|  Format: Cash 6max 100bb                          |
|                                                   |
|  [<< Prev Street]  Flop: Qs 9h 4d  [Next Street >>]|
|                                                   |
|  +---------------------------------------------+ |
|  |                                             | |
|  |  BB checks                                  | |
|  |  BTN bets 4.3 BB (66% pot)                  | |
|  |  BB calls                                   | |
|  |                                             | |
|  |  GTO for BTN here:                          | |
|  |    Bet 75%:  52%  (EV: 4.21)               | |
|  |    Bet 33%:  31%  (EV: 4.18)  <-- Yours    | |
|  |    Check:    17%  (EV: 3.95)               | |
|  |                                             | |
|  |  EV Loss: -0.03 BB (Correct move)          | |
|  +---------------------------------------------+ |
|                                                   |
|  [Solution Browser]  [Replay]  [Share]            |
+---------------------------------------------------+
```

### Navigation Features

| Control              | Function                                            |
|----------------------|-----------------------------------------------------|
| Left/Right arrows    | Navigate between streets                            |
| Solution Browser     | Jump to the full Study Mode solution for this spot  |
| Replay button        | Re-watch the hand action animation                  |
| Share button         | Generate snapshot for sharing with others            |
| Sidebar expand (L)   | Expand to see all streets and action details         |

### Integration with Study Mode

From the hand replayer, players can click the Solution Browser icon to jump directly into Study Mode with the board and action sequence pre-populated. This enables deep analysis including:

- Range accuracy at each decision point
- Who holds the range advantage and nut advantage
- Continuation strategies on later streets
- Alternative lines and bet sizing logic
- What hands each bet size targets
- Stack-to-pot ratio considerations

See [Study Mode](03_study_mode.md) for full details on the Study Mode interface.

---

## Performance Analytics (Analyzer 2.0)

### Dashboard Metrics

The Analyzer 2.0 dashboard provides aggregate performance analytics:

```
+---------------------------------------------------+
|  PERFORMANCE DASHBOARD                            |
|                                                   |
|  Overall GTOW Score:    +82%                      |
|  Total Hands Analyzed:  4,521                     |
|  Total EV Loss:         -87.3 BB                  |
|  Avg EV Loss/Hand:      -0.019 BB                 |
|                                                   |
|  By Street:                                       |
|    Preflop:  +91%   (-12.1 BB total)              |
|    Flop:     +84%   (-28.7 BB total)              |
|    Turn:     +79%   (-25.9 BB total)              |
|    River:    +76%   (-20.6 BB total)              |
|                                                   |
|  By Position:                                     |
|    BTN:  +88%    CO:  +85%    HJ:  +81%          |
|    UTG:  +83%    SB:  +78%    BB:  +76%          |
|                                                   |
|  By Action Type:                                  |
|    RFI:        +89%                               |
|    vs 3-Bet:   +76%                               |
|    3-Bet:      +82%                               |
|    Defense:    +74%                                |
+---------------------------------------------------+
```

### Breakdown Dimensions

Performance can be analyzed across:
- **Preflop action type**: Calling, raising, folding impact
- **Position**: Per-seat performance tracking
- **Street**: Flop, turn, river decision patterns
- **Preflop aggression level**: Passive vs. aggressive lines
- **Date range**: Tracking improvement over time

---

## Tips for Studying Analyzed Hands

### Workflow Recommendations

1. **Sort by EV Loss** first to find the biggest blunders
2. **Sort by Avg EV Loss** to find patterns of consistent mistakes
3. **Use the Solution Browser** to understand why GTO recommends a different action
4. **Filter by hand class** to identify action preferences for specific hand types
5. **Modify board cards** in Study Mode to see how strategy changes across textures
6. **Compare positional ranges** to understand how position affects optimal play
7. **Use flop/turn reports** to identify underlying strategic trends

### Separating Process from Outcome

GTO Wizard emphasizes that biggest losses are not necessarily biggest mistakes, and biggest wins are not necessarily best plays. The analyzer measures decision quality independent of results.

### Interpreting Scores in Context

- Scores above 85% indicate solid play
- 6-max players below 80% should focus on fundamentals
- Solid regulars achieve 90-95%
- 95%+ represents GTO excellence
- Wide-range formats (HU, Spins) naturally produce lower scores
- Achieving 100% means never exploiting opponent mistakes, which is suboptimal in practice

---

## Analyze Mode Component Specifications

| Component            | Position       | Data Displayed                           | Interaction Model              |
|----------------------|----------------|------------------------------------------|--------------------------------|
| Upload Dialog        | Modal overlay  | File drop zone, format selector          | Drag-drop, click browse        |
| Uploads Page         | Main area      | Upload batches with status indicators    | Click to view, sort            |
| Hands Table          | Main area      | Analyzed hands with EV loss, status      | Sort, filter, click to review  |
| Filter Bar           | Top of table   | Date, position, action, EV loss filters  | Dropdown selectors, toggles    |
| Hand Sidebar         | Right panel    | Individual hand overview and streets     | Click to expand, replay        |
| Replayer             | Center modal   | Street-by-street hand replay with GTO    | Arrow navigation, buttons      |
| Solution Browser Link| Replayer       | Direct link to Study Mode for this spot  | Click to open Study Mode       |
| Performance Dashboard| Top section    | Aggregate metrics and breakdowns         | Filter by date, format         |
| Saved Reports        | Filter bar     | Named filter configurations              | Save, load, auto-update        |

---

## Screenshot References

> **Screenshot references (Analyzer interface):** See `screenshots/blog/analyzer2-hands-table.png`, `screenshots/blog/analyzer2-hand-analysis.png`

> **Screenshot references (Analyzer features):** See `screenshots/blog/analyzer2-position-breakdown.png`, `screenshots/blog/analyzer2-preflop-action.png`

> **Screenshot references (Redesigned analyzer):** See `screenshots/blog/redesigned-preflop-mistakes.png`, `screenshots/blog/redesigned-hand-filtering.png`

> **Screenshot references (Third-party views):** See `screenshots/reviews/h2n-analyzed-hands.webp`

---

## Sources

- [Analyze Mode Guide - GTO Wizard Help](https://help.gtowizard.com/analyze-mode-guide/)
- [How to Upload Your Hand Histories - GTO Wizard Help](https://help.gtowizard.com/how-to-upload-your-hand-histories/)
- [Tips & Tricks to Study Analyzed Hands - GTO Wizard Help](https://help.gtowizard.com/tips-and-tricks-to-study-analyzed-hands/)
- [HH Analyzer 2.0 - GTO Wizard Blog](https://blog.gtowizard.com/redefining-precision-introducing-hh-analyzer-2-0-and-much-more/)
- [Hand History Analyzer Launch - GTO Wizard Blog](https://blog.gtowizard.com/hand-history-analyzer-is-here-patch-notes/)
- [Supported Sites and Formats - GTO Wizard Blog](https://blog.gtowizard.com/suported-sites-and-formats-for-hh-analyzer/)
- [Fixing a Poker Leak - GTO Wizard Blog](https://blog.gtowizard.com/fixing-a-poker-leak-part-1-spotting-and-correcting-errors/)

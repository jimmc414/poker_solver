# GTO Wizard UI/UX & Screen Documentation

This document provides a comprehensive visual and functional reference for every major screen in GTO Wizard, including ASCII layout diagrams, component specifications, color systems, and interaction patterns.

---

## 1. Design System Overview

### 1.1 Color Coding Systems

GTO Wizard uses a consistent color language across all screens to communicate poker actions, EV values, and strategic information.

#### Action Colors

| Action        | Color          | Hex Range       | Usage Context                        |
|---------------|----------------|-----------------|--------------------------------------|
| Bet / Raise   | Red shades     | #E04040-#FF6666 | All bet and raise actions            |
| Check / Call  | Green shades   | #40B040-#66CC66 | Passive actions                      |
| Fold          | Blue shades    | #4060D0-#6688EE | Surrender actions                    |
| All-in        | Deep red/gold  | #CC2020-#FFD700 | Maximum commitment actions           |
| Mixed strategy| Multi-color    | Proportional    | Cells showing blended frequencies    |

#### EV Heatmap Gradient

| Value Range     | Color         | Meaning                              |
|-----------------|---------------|--------------------------------------|
| High positive   | Bright green  | Strong profitable spot               |
| Moderate positive| Light green  | Slightly profitable                  |
| Near zero       | White/neutral | Breakeven                            |
| Moderate negative| Light red    | Slightly losing                      |
| High negative   | Bright red    | Significant EV loss                  |

#### Deviation Indicators (Analyzer / Reports)

| Indicator       | Color         | Meaning                              |
|-----------------|---------------|--------------------------------------|
| Best action     | Double checkmark (green) | Optimal GTO play              |
| Correct         | Single checkmark (green) | Acceptable alternative        |
| Inaccuracy      | Yellow/amber  | Minor deviation from GTO             |
| Wrong move      | Orange        | Significant deviation                |
| Blunder         | Red           | Critical mistake with large EV loss  |

#### Strategy Matrix Cell Colors

- **Solid color**: Pure strategy (100% one action)
- **Multi-color split**: Mixed strategy showing proportional frequencies
- **Black/dark gray**: Hand not in the range (blocked by board cards)
- **Dimmed/faded**: Hand filtered out by current filter selection

### 1.2 Theme System

GTO Wizard supports extensive visual customization:

- **Built-in themes**: Multiple preloaded dark and light themes
- **Custom themes**: Users can create and import themes via hex color code strings
- **Community themes**: Gallery of user-created themes (e.g., "Ice Cold", "Wisdom", "Colorblinds Arise")
- **Customizable elements**: Background colors, card themes, card backs, text colors, shadows, action colors, bet size colors
- **Color-blind options**: Community-created accessibility themes available
- **Resolution support**: Optimized for 2K and 4K monitors

### 1.3 Layout Options (Study Mode)

Four layout configurations are available:

| Layout              | Description                                    |
|---------------------|------------------------------------------------|
| Horizontal          | Side-by-side panels (default)                  |
| Horizontal Reversed | Side-by-side, components swapped               |
| Split               | Stacked vertical arrangement                   |
| Split Reversed      | Stacked vertical, components swapped           |

Three sizing options: **Large**, **Medium**, **Compact**. Press **Q** to maximize the solution browser.

---

## 2. Main Dashboard / Home Screen

```
+-----------------------------------------------------------------------+
|  [Logo] GTO Wizard          [Study] [Practice] [Analyze] [Content]    |
|                              [PokerArena] [Table Wizard] [Settings]   |
+-----------------------------------------------------------------------+
|                                                                       |
|  +-------------------------+  +-------------------------+             |
|  |     STUDY MODE          |  |     PRACTICE MODE       |             |
|  |  [Strategy icon]        |  |  [Training icon]        |             |
|  |  10M+ presolved spots   |  |  GTO Trainer            |             |
|  |  Preflop to River       |  |  Customizable drills    |             |
|  |  [Enter Study >>]       |  |  [Start Training >>]    |             |
|  +-------------------------+  +-------------------------+             |
|                                                                       |
|  +-------------------------+  +-------------------------+             |
|  |     ANALYZE MODE        |  |     CONTENT             |             |
|  |  [Upload icon]          |  |  [Articles icon]        |             |
|  |  Hand History Analysis  |  |  Strategy articles      |             |
|  |  GTO Reports            |  |  Study plans, quizzes   |             |
|  |  [Upload Hands >>]      |  |  [Browse Content >>]    |             |
|  +-------------------------+  +-------------------------+             |
|                                                                       |
|  +-------------------------+  +-------------------------+             |
|  |     POKER ARENA         |  |     TABLE WIZARD        |             |
|  |  [Arena icon]           |  |  [Tables icon]          |             |
|  |  Competitive HU poker   |  |  Multi-table manager    |             |
|  |  Ranked & Casual        |  |  Real-time overlays     |             |
|  |  [Play Now >>]          |  |  [Launch >>]            |             |
|  +-------------------------+  +-------------------------+             |
|                                                                       |
|  +--- Session Stats ------------------------------------------------+ |
|  | Trainer Score: 82.4  | Hands Practiced: 1,247 | Blunders: 14     | |
|  | Analyzed Hands: 5,600 | Avg EV Loss: 2.1bb   | Sessions: 43     | |
|  +------------------------------------------------------------------+ |
+-----------------------------------------------------------------------+
```

**Component Specification**

| Component        | Position      | Data Displayed                    | Interaction               |
|------------------|---------------|-----------------------------------|---------------------------|
| Navigation Bar   | Top           | Mode links, settings, profile     | Click to navigate         |
| Mode Cards       | Center grid   | Feature name, description, icon   | Click to enter mode       |
| Session Stats    | Bottom        | Aggregate training/analysis stats | Hover for details         |
| Language Selector| Settings      | 17 languages available            | Dropdown selection        |

---

## 3. Study Mode

### 3.1 Spot Selector (Top Navigation)

```
+-----------------------------------------------------------------------+
| Format: [Cash 6max v] | Stack: [100bb v] | Rake: [NL500 v]           |
+-----------------------------------------------------------------------+
| Position Flow:                                                        |
| [UTG] -> [HJ] -> [CO] -> [BTN] -> [SB] -> [BB]                     |
|                                                                       |
| Action Sequence:                                                      |
| Preflop: [Open 2.5x] -> [3-Bet 8.5x] -> [Call]                     |
| Flop:    [Ks 7h 2d]  -> [Check] -> [Bet 33%]                       |
|                                                                       |
| [J] Jump to spot popup    [< Back]  [Forward >]                     |
+-----------------------------------------------------------------------+
```

The spot selector lets users navigate any poker situation by selecting positions and actions sequentially. The **Jump (J)** shortcut opens a popup table for fast navigation to specific matchups.

### 3.2 Strategy Matrix View (Tab 1)

```
+-----------------------------------------------------------------------+
| [1:Strategy] [2:Ranges] [3:Breakdown] [4:Reports]      [Settings]    |
+-----------------------------------------------------------------------+
| INFOBOX                          | STRATEGY MATRIX (13x13 grid)      |
| +-----------------------------+  | +-------------------------------+ |
| | Positions:  CO  vs  BB      |  | |  AA  AKs AQs AJs ATs ...     | |
| | Pot: 6.5bb -> 13bb          |  | |  AKo KK  KQs KJs KTs ...     | |
| | Board: Ks 7h 2d             |  | |  AQo KQo QQ  QJs QTs ...     | |
| | Pot Odds: 33% (2:1)         |  | |  AJo KJo QJo JJ  JTs ...     | |
| | SPR: 7.2                    |  | |  ATo KTo QTo JTo TT  ...     | |
| |                             |  | |  ...                          | |
| | [Overview | Table View]     |  | |                               | |
| +-----------------------------+  | | Color = action (red/green/blue| |
| |                             |  | | Split cells = mixed strategy  | |
| | ACTIONS TABLE               |  | +-------------------------------+ |
| | +-------------------------+ |  |                                   |
| | | Action   | Freq  | Size| |  | HAND DETAIL (hover popup)         |
| | |----------|-------|-----| |  | +-------------------------------+ |
| | | Bet 33%  | 42.1% | 4bb | |  | | AKs: Bet 33% (65%) Call (35%)| |
| | | Bet 75%  | 15.3% | 10bb| |  | | EV: +2.3bb  EQ: 62.1%       | |
| | | Check    | 42.6% |  -  | |  | | Suit breakdown:              | |
| | +-------------------------+ |  | | AhKh: Bet 33% (80%)         | |
| |                             |  | | AsKs: Check (90%)            | |
| | [S] Toggle chart/table      |  | +-------------------------------+ |
| +-----------------------------+  |                                   |
+-----------------------------------------------------------------------+
| HAND CATEGORY FILTERS:                                                |
| [Pairs] [Suited] [Offsuit] [Broadways] [Connectors] [Suited Gappers] |
| SUIT FILTERS: [s][h][d][c]   BET FILTERS: [Bet33%] [Bet75%] [Check] |
| [CLEAR (P)]                                                          |
+-----------------------------------------------------------------------+
```

**Component Specification**

| Component          | Position       | Data Displayed                          | Interaction                    |
|--------------------|----------------|-----------------------------------------|--------------------------------|
| Infobox            | Top-left       | Positions, pot, board, pot odds, SPR    | Toggle Overview/Table view     |
| Strategy Matrix    | Top-right      | 13x13 hand grid, color-coded actions    | Hover for detail, click filter |
| Actions Table      | Bottom-left    | Action frequencies, bet sizes           | Click to filter matrix, S key  |
| Hand Detail Popup  | Bottom-right   | Per-hand/suit breakdown on hover        | Appears on matrix hover        |
| Hand Category Filters | Bottom bar  | Category proportions in range           | Toggle filters on/off          |
| Suit Filters       | Bottom bar     | Card suit inclusion/exclusion           | Dropdown in matrix corner      |

**Metrics Available (Dropdown)**

| Metric     | Key         | Description                                    |
|------------|-------------|------------------------------------------------|
| EV         | Default     | Expected value in big blinds                   |
| EQ         | -           | Equity percentage at showdown                  |
| EQR        | -           | Equity realization (EV / EQ ratio)             |
| Range Weight| -          | Frequency of hand combos in range              |
| Compare EV | -           | Side-by-side EV for different actions          |

**Keyboard Shortcuts**

| Key       | Action                                  |
|-----------|-----------------------------------------|
| 1/2/3/4   | Switch between Strategy/Ranges/Breakdown/Reports tabs |
| J         | Open jump-to-spot popup                 |
| S         | Toggle chart vs table in Actions Table  |
| P         | Clear all filters                       |
| Spacebar  | Toggle BB vs pot % display              |
| Shift+Space | Toggle EV vs EV/Pot display          |
| Q         | Maximize solution browser               |

### 3.3 Ranges View (Tab 2)

```
+-----------------------------------------------------------------------+
| [1:Strategy] [2:Ranges] [3:Breakdown] [4:Reports]                     |
+-----------------------------------------------------------------------+
|                                                                       |
|  PLAYER 1 RANGE (CO)          |  PLAYER 2 RANGE (BB)                 |
|  +-------------------------+  |  +-------------------------+          |
|  | 13x13 hand matrix       |  |  | 13x13 hand matrix       |         |
|  | Color = weight in range  |  |  | Color = weight in range  |        |
|  | Bright = high frequency  |  |  | Bright = high frequency  |        |
|  | Dim = low frequency      |  |  | Dim = low frequency      |        |
|  +-------------------------+  |  +-------------------------+          |
|                                                                       |
|  EQUITY DISTRIBUTION          |  HAND CLASS BREAKDOWN                 |
|  +-------------------------+  |  +-------------------------+          |
|  | Histogram showing EQ    |  |  | Top pair: 22%           |          |
|  | distribution for both   |  |  | Overpair: 8%            |          |
|  | players across all      |  |  | Two pair: 5%            |          |
|  | hand combos             |  |  | Draws: 15%              |          |
|  +-------------------------+  |  | Air: 50%                |          |
|                               |  +-------------------------+          |
+-----------------------------------------------------------------------+
```

The Ranges view provides a comparative analysis of both players' ranges, showing equity distributions and hand class proportions. Players can compare how different hand categories contribute to each range.

### 3.4 Breakdown View (Tab 3)

```
+-----------------------------------------------------------------------+
| [1:Strategy] [2:Ranges] [3:Breakdown] [4:Reports]                     |
+-----------------------------------------------------------------------+
|                                                                       |
|  ACTION COMPOSITION BREAKDOWN                                         |
|  +----------------------------------------------------------------+  |
|  | Bet 33% Range Construction:                                    |  |
|  |                                                                |  |
|  | [=======] Nutted hands (top pair+)        35%                 |  |
|  | [=====]   Strong draws (OESD, FD)         25%                 |  |
|  | [====]    Medium strength (2nd pair)       20%                 |  |
|  | [===]     Bluffs (gutshots, air)           15%                 |  |
|  | [=]       Blockers                          5%                 |  |
|  +----------------------------------------------------------------+  |
|                                                                       |
|  | Bet 75% Range Construction:                                    |  |
|  | [========] Nutted hands                    45%                 |  |
|  | [======]   Strong draws                    30%                 |  |
|  | [===]      Bluffs                          20%                 |  |
|  | [=]        Other                            5%                 |  |
|  +----------------------------------------------------------------+  |
|                                                                       |
|  | Check Range Construction:                                      |  |
|  | [=======] Medium strength                  40%                 |  |
|  | [=====]   Traps (slow-plays)               15%                 |  |
|  | [====]    Weak draws                       20%                 |  |
|  | [====]    Weak made hands                  25%                 |  |
|  +----------------------------------------------------------------+  |
+-----------------------------------------------------------------------+
```

The Breakdown view reveals the internal composition of each betting action, showing what categories of hands make up each bet size. This helps players understand *why* specific hands take certain actions in GTO strategy.

### 3.5 Reports View (Tab 4)

```
+-----------------------------------------------------------------------+
| [1:Strategy] [2:Ranges] [3:Breakdown] [4:Reports]                     |
+-----------------------------------------------------------------------+
|                                                                       |
|  AGGREGATED FLOP REPORTS                                              |
|  Analyzing all 1,755 possible flop textures                          |
|                                                                       |
|  +----------------------------------------------------------------+  |
|  | Board Texture    | C-Bet% | Check% | Avg Size | OOP EQ        |  |
|  |------------------|--------|--------|----------|---------------|  |
|  | Monotone high    | 72%    | 28%    | 33% pot  | 42%           |  |
|  | Rainbow low      | 55%    | 45%    | 50% pot  | 45%           |  |
|  | Paired boards    | 48%    | 52%    | 40% pot  | 44%           |  |
|  | Connected mid    | 63%    | 37%    | 45% pot  | 43%           |  |
|  | Dry A-high       | 80%    | 20%    | 33% pot  | 38%           |  |
|  | ...              | ...    | ...    | ...      | ...           |  |
|  +----------------------------------------------------------------+  |
|                                                                       |
|  FILTERS:                                                            |
|  Board texture: [High cards v] [Suits v] [Connectedness v]          |
|  Tagging: [Color-coded tags for saved filter combinations]           |
|                                                                       |
+-----------------------------------------------------------------------+
```

Reports provide aggregate data across all possible board textures for the selected spot, enabling pattern recognition across thousands of flop/turn scenarios.

### 3.6 Blockers Tab

```
+-----------------------------------------------------------------------+
| BLOCKERS ANALYSIS                                                     |
+-----------------------------------------------------------------------+
| Card   | Bet Freq Impact | Check Freq Impact | Fold Freq Impact      |
|--------|-----------------|-------------------|-----------------------|
| As     | +8.2%           | -5.1%             | -3.1%                |
| Kh     | +3.4%           | -1.2%             | -2.2%                |
| Qd     | -1.5%           | +2.3%             | -0.8%                |
| ...    | ...             | ...               | ...                  |
+-----------------------------------------------------------------------+
| Sorted by: [Bet Freq Impact v]   Columns sortable by click           |
+-----------------------------------------------------------------------+
```

The Blockers tab shows how holding specific cards affects action frequencies due to card removal effects, helping players understand blocker-based bluffing and value betting decisions.

---

## 4. Practice Mode (GTO Trainer)

### 4.1 Main Trainer Table View

```
+-----------------------------------------------------------------------+
|  [3-dot Menu]  Table 1 of 4                    [Lightbulb: Info Panel]|
+-----------------------------------------------------------------------+
|                                                                       |
|         Opponent (BB)                                                 |
|         [??] [??]  (card backs / hidden)                             |
|         Stack: 97.5bb                                                |
|                                                                       |
|     +---------------------------------------+                         |
|     |           COMMUNITY CARDS             |                         |
|     |     [Ks]  [7h]  [2d]  [ ]  [ ]       |                         |
|     |           Pot: 13bb                   |                         |
|     +---------------------------------------+                         |
|                                                                       |
|         Hero (CO)                                                     |
|         [Ah] [Kd]                                                    |
|         Stack: 93.5bb                                                |
|                                                                       |
+-----------------------------------------------------------------------+
| ACTION BUTTONS:                                                       |
| [Fold]  [Check/Call]  [Bet 33% (4.3bb)]  [Bet 75% (9.75bb)]        |
|                       [Bet 125% (16.25bb)]  [All-In (93.5bb)]       |
+-----------------------------------------------------------------------+
| RNG Dice: [47]  |  Timer: [12s remaining]  |  Score: 85.2           |
+-----------------------------------------------------------------------+
```

### 4.2 Trainer Results / Scoring Display

```
+-----------------------------------------------------------------------+
| HAND RESULT                                                           |
+-----------------------------------------------------------------------+
|                                                                       |
|  Your Action: Bet 33%        GTO Action: Bet 33% (65%)              |
|  Rating: [**] BEST ACTION                                           |
|  Points: +10                                                         |
|                                                                       |
|  Alternative Actions:                                                |
|  +------------------------------------------------------------+     |
|  | Check      | 35% freq | EV: +1.8bb  | EV Loss: -0.5bb     |     |
|  | Bet 75%    |  0% freq | EV: +0.9bb  | EV Loss: -1.4bb     |     |
|  +------------------------------------------------------------+     |
|                                                                       |
|  [View in Study Mode]  [Next Hand >>]  [Replay]                     |
+-----------------------------------------------------------------------+
```

### 4.3 Session Summary

```
+-----------------------------------------------------------------------+
| SESSION COMPLETE - 50 Hands                                           |
+-----------------------------------------------------------------------+
|                                                                       |
|  Overall Score: 82.4 / 100                                           |
|                                                                       |
|  +------------------------------------------------------------+     |
|  | Rating       | Count | Percentage                          |     |
|  |--------------|-------|-------------------------------------|     |
|  | Best         |  22   | 44%  [================]             |     |
|  | Correct      |  15   | 30%  [============]                 |     |
|  | Inaccuracy   |   8   | 16%  [======]                      |     |
|  | Wrong        |   3   |  6%  [==]                           |     |
|  | Blunder      |   2   |  4%  [=]                            |     |
|  +------------------------------------------------------------+     |
|                                                                       |
|  Total EV Lost: 14.2bb                                               |
|  Biggest Blunder: Hand #34 (ATo on K72r, -3.8bb)                    |
|                                                                       |
|  [Review Blunders]  [New Session]  [Export to Analyze]               |
+-----------------------------------------------------------------------+
```

**Trainer Component Specification**

| Component       | Position        | Data Displayed                      | Interaction                  |
|-----------------|-----------------|-------------------------------------|------------------------------|
| Card Display    | Center          | Hole cards, community cards         | Visual only                  |
| Stack Displays  | Near players    | Current stack sizes                 | Visual only                  |
| Pot Display     | Center table    | Current pot size                    | Visual only                  |
| Action Buttons  | Bottom          | Available actions with sizes        | Click to act                 |
| RNG Dice        | Bottom-left     | Random number 1-100                 | Yellow = active, white = off |
| Timer           | Bottom-center   | Countdown for timed mode            | Auto-counting                |
| Score           | Bottom-right    | Running session score               | Updates per hand             |
| Info Panel      | Side (toggleable)| Range tab + Strategy tab           | Eye icon toggles, pop-out    |

**Trainer Configuration Options**

| Setting           | Options                                          |
|-------------------|--------------------------------------------------|
| Game Mode         | Full Hand, Spot (single decision), Street        |
| Difficulty        | Simple (action only), Grouped (size category), Standard (exact size) |
| Input Mode        | Actions (choose action), Frequency (specify %)   |
| Speed             | Normal, Fast, Turbo                              |
| Timebank          | 7s, 15s, 25s                                     |
| Tables            | 1, 2, 3, or 4 simultaneous                      |
| Auto New Hand     | Instant, 3-second delay                          |
| Pause After       | Every action, Mistakes only, Never               |
| Learning Mode     | Auto-show info panel on pause                    |
| RNG               | Off, High (aggressive at high numbers), Low      |
| Hand Selection    | All, Close decisions, Shortstack push/fold, Premium only |

---

## 5. Analyze Mode (Hand History Analyzer)

### 5.1 Upload Screen

```
+-----------------------------------------------------------------------+
| ANALYZE - Hand History Upload                                         |
+-----------------------------------------------------------------------+
|                                                                       |
|  +----------------------------------------------+                    |
|  |                                              |                    |
|  |    [Drag & Drop Zone]                        |                    |
|  |                                              |                    |
|  |    Drop hand history files here              |                    |
|  |    or click to browse                        |                    |
|  |                                              |                    |
|  |    Supported: .txt, .xml, folder upload      |                    |
|  |                                              |                    |
|  +----------------------------------------------+                    |
|                                                                       |
|  Supported Sites:                                                    |
|  [PokerStars] [888Poker] [GGPoker] [WPN] [Winamax]                 |
|  [PartyPoker] [iPoker] [Coinpoker] [Chico] [Bovada]                |
|                                                                       |
|  Upload Limit: [XX / month] (varies by tier)                        |
|                                                                       |
|  -- OR --                                                            |
|  [Table Wizard Auto-Upload: Connected]                               |
|                                                                       |
+-----------------------------------------------------------------------+
```

### 5.2 Analysis Results / Hands Table

```
+-----------------------------------------------------------------------+
| ANALYZED HANDS                                            [Filters v] |
+-----------------------------------------------------------------------+
| Active Filters: [SRP] [CO vs BB] [Flop C-Bet]    [x remove] [Clear] |
+-----------------------------------------------------------------------+
| #   | Hand    | Position | Action    | GTO     | EV Loss | Rating    |
|-----|---------|----------|-----------|---------|---------|-----------|
| 1   | AhKd    | CO       | Bet 75%   | Bet 33% | -2.1bb  | Wrong     |
| 2   | QsJs    | BTN      | Check     | Check   |  0.0bb  | Best      |
| 3   | 9c8c    | SB       | Fold      | Call    | -1.8bb  | Blunder   |
| 4   | TdTc    | HJ       | Bet 33%   | Bet 33% | -0.1bb  | Correct   |
| ...                                                                   |
+-----------------------------------------------------------------------+
| Page [1] of 12    Sort by: [EV Loss v]    [Bulk Tag] [Export]        |
+-----------------------------------------------------------------------+
|                                                                       |
| [Open in Study Mode]  [Open in Practice Mode]  [Replay Hand]        |
+-----------------------------------------------------------------------+
```

### 5.3 Hand Replay View

```
+-----------------------------------------------------------------------+
| HAND REPLAY - Hand #3: 9c8c (SB vs BB)                              |
+-----------------------------------------------------------------------+
|                                                                       |
|         Opponent (BB)                                                 |
|         [Revealed at showdown]                                       |
|                                                                       |
|     +---------------------------------------+                         |
|     |     [Js]  [Th]  [3d]  [7c]  [2s]     |                         |
|     |           Pot: 24bb                   |                         |
|     +---------------------------------------+                         |
|                                                                       |
|         Hero (SB)                                                     |
|         [9c] [8c]                                                    |
|                                                                       |
+-----------------------------------------------------------------------+
| STREET NAVIGATION: [Preflop] [Flop >>] [Turn >>] [River >>]         |
+-----------------------------------------------------------------------+
| YOUR ACTION: Fold                    GTO: Call (72% freq)            |
| EV of your action: 0.0bb            EV of GTO action: +1.8bb        |
| EV LOSS: -1.8bb                     Rating: BLUNDER                  |
+-----------------------------------------------------------------------+
| FULL STRATEGY:                                                        |
| Call: 72%  |  Raise: 18%  |  Fold: 10%                              |
+-----------------------------------------------------------------------+
```

### 5.4 GTO Reports View

```
+-----------------------------------------------------------------------+
| GTO REPORTS                                                           |
+-----------------------------------------------------------------------+
| Report: [Default Analysis v]  [+ New Report]  [Edit]  [Save]        |
+-----------------------------------------------------------------------+
|                                                                       |
| PREFLOP MISTAKES (Top 10 by EV Loss)                                 |
| +----------------------------------------------------------------+   |
| | Hand    | Spot          | Your Action | GTO Action | EV Loss   |   |
| |---------|---------------|-------------|------------|-----------|   |
| | Q7o     | UTG Open      | Raise       | Fold       | -0.8bb    |   |
| | K3s     | SB vs BTN 3B  | Call        | Fold       | -0.6bb    |   |
| | A2o     | CO vs 3-Bet   | Call        | Fold       | -0.5bb    |   |
| | ...     | ...           | ...         | ...        | ...       |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| GENERAL FLOP VIEW - Frequency Comparison                             |
| +----------------------------------------------------------------+   |
| | Spot           | Your Freq | GTO Freq | Deviation | Indicator |   |
| |----------------|-----------|----------|-----------|-----------|   |
| | C-Bet SRP IP   | 78%       | 62%      | +16%      | Over-bet  |   |
| | Fold to C-Bet  | 42%       | 35%      | +7%       | Over-fold |   |
| | Check-Raise    |  5%       | 12%      | -7%       | Under-agg |   |
| | ...            | ...       | ...      | ...       | ...       |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| Color coding: Green = within range | Yellow = slight deviation       |
|               Red = significant deviation | Gray = insufficient data |
| Sample size tooltips show statistical reliability                     |
+-----------------------------------------------------------------------+
```

---

## 6. Range Builder Interface

```
+-----------------------------------------------------------------------+
| RANGE BUILDER                                              [GTO Score]|
+-----------------------------------------------------------------------+
|                                                                       |
| CONFIGURATION:                                                        |
| Format: [Cash 6max v]  Stack: [100bb v]  Position: [BTN v]          |
| Facing: [3-Bet from BB v]  Rake: [NL500 v]                          |
|                                                                       |
| +---------------------------+  +----------------------------------+  |
| | HAND SELECTION MATRIX     |  | STRATEGY OUTPUT                  |  |
| | 13x13 grid                |  |                                  |  |
| |                           |  | Selected Range:                  |  |
| | Click cells to toggle     |  | 152 combos (11.4%)              |  |
| | Drag to select regions    |  |                                  |  |
| | Right-click for frequency |  | Actions Assigned:                |  |
| |                           |  | Raise: 45% of range             |  |
| | [Select All] [Clear]      |  | Call: 35% of range              |  |
| | [Load Range] [Save Range] |  | Fold: 20% of range              |  |
| |                           |  |                                  |  |
| | Color indicates:          |  | GTO Score: 87.3 / 100           |  |
| | Selected weight (bright   |  | vs GTO deviation: -1.2bb/100    |  |
| | = high, dim = low)        |  |                                  |  |
| +---------------------------+  +----------------------------------+  |
|                                                                       |
| PARAMETERS:                                                          |
| Every parameter is customizable. Strategy output receives GTO scoring.|
+-----------------------------------------------------------------------+
```

---

## 7. GTO Wizard AI (Custom Solve Configuration)

```
+-----------------------------------------------------------------------+
| GTO WIZARD AI - Custom Solver                        [Elite Feature]  |
+-----------------------------------------------------------------------+
|                                                                       |
| SOLVE CONFIGURATION:                                                  |
| +----------------------------------------------------------------+   |
| | Players: [2 v]  (up to 9 for preflop multiway)                |   |
| |                                                                |   |
| | Player 1 (IP):                                                 |   |
| | Position: [CO v]  Range: [Edit Range]  Stack: [100bb]         |   |
| |                                                                |   |
| | Player 2 (OOP):                                                |   |
| | Position: [BB v]  Range: [Edit Range]  Stack: [100bb]         |   |
| |                                                                |   |
| | Pot Size: [6.5bb]  Board: [Ks 7h 2d] or [Random]            |   |
| |                                                                |   |
| | BETTING TREE:                                                  |   |
| | IP Bet Sizes:  [33%] [67%] [125%] [+ Add Size]               |   |
| | OOP Bet Sizes: [33%] [50%] [75%]  [+ Add Size]               |   |
| | Raise Sizes:   [2.5x] [3x] [All-in] [+ Add Size]            |   |
| |                                                                |   |
| | Accuracy Target: [0.3% pot v]                                  |   |
| | Nash Distance: displayed after solve                           |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| [Solve] [Load Preset] [Save Configuration]                          |
|                                                                       |
| POWER CREDITS: 47 / 100 remaining this month                        |
+-----------------------------------------------------------------------+
```

**GTO Wizard AI achieves an average Nash Distance of 0.21% pot (dEV), described as the most accurate of any cloud solver.**

---

## 8. Nodelocking Interface

```
+-----------------------------------------------------------------------+
| NODELOCKING 2.0                                      [Elite Feature]  |
+-----------------------------------------------------------------------+
|                                                                       |
| GAME TREE NAVIGATION:                                                 |
| +----------------------------------------------------------------+   |
| |  Root                                                          |   |
| |  +-- CO Opens 2.5x                                            |   |
| |      +-- BB 3-Bets 8.5x                                       |   |
| |          +-- CO Calls                                          |   |
| |              +-- Flop: Ks 7h 2d                                |   |
| |                  +-- BB Checks  <-- [LOCK THIS NODE]           |   |
| |                  +-- BB Bets 33%                                |   |
| |                  +-- BB Bets 75%                                |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| NODE LOCK SETTINGS:                                                   |
| +----------------------------------------------------------------+   |
| | Lock Type: [Frequency Lock v]                                  |   |
| |                                                                |   |
| | Opponent Check Frequency:  [70%] (GTO: 55%)                   |   |
| | Opponent Bet 33% Frequency: [20%] (GTO: 30%)                  |   |
| | Opponent Bet 75% Frequency: [10%] (GTO: 15%)                  |   |
| |                                                                |   |
| | [Apply Lock]  [Reset to GTO]  [Lock All Similar Nodes]        |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| EXPLOITATIVE RESPONSE:                                                |
| After re-solving with locked nodes, the strategy updates to show     |
| the maximally exploitative counter-strategy.                          |
|                                                                       |
| Custom Profiles: [Tight Passive v] [Loose Aggressive] [+ New]       |
+-----------------------------------------------------------------------+
```

---

## 9. PokerArena Interface

```
+-----------------------------------------------------------------------+
| POKER ARENA                                                           |
+-----------------------------------------------------------------------+
| MODE: [Competitive] [Casual] [Play With Friends]     Season 5        |
+-----------------------------------------------------------------------+
|                                                                       |
|         Opponent: "PokerPro99" (Rating: 1,847)                       |
|         [??] [??]                                                    |
|                                                                       |
|     +---------------------------------------+                         |
|     |     [Ts]  [8h]  [3c]  [ ]  [ ]       |                         |
|     |           Pot: 6bb                    |                         |
|     +---------------------------------------+                         |
|                                                                       |
|         You (Rating: 1,623)                                          |
|         [Ah] [Js]                                                    |
|                                                                       |
+-----------------------------------------------------------------------+
| [Fold]  [Check]  [Bet 50%]  [Bet 75%]  [All-In]                    |
+-----------------------------------------------------------------------+
|                                                                       |
| LEADERBOARD (Season 5):                                              |
| +--------------------------------------------+                       |
| | Rank | Player        | Rating | W/L        |                       |
| |------|---------------|--------|------------|                       |
| | 1    | GTOMaster     | 2,145  | 342-156    |                       |
| | 2    | SolverKing    | 2,089  | 298-142    |                       |
| | 3    | RangeWarrior  | 2,034  | 267-133    |                       |
| +--------------------------------------------+                       |
|                                                                       |
| [Post-Game: Review in GTO Wizard]                                    |
+-----------------------------------------------------------------------+
```

**Platform**: Desktop, mobile browser, iOS app. Full crossplay. Free to play; deep analysis requires GTO Wizard subscription.

---

## 10. Table Wizard Interface

```
+-----------------------------------------------------------------------+
| TABLE WIZARD - Multi-Table Manager                                    |
+-----------------------------------------------------------------------+
|                                                                       |
| +-----------------------------+  +-----------------------------+      |
| | TABLE 1 (PokerStars NL200) |  | TABLE 2 (GGPoker NL100)    |      |
| | +-------------------------+|  | +-------------------------+ |      |
| | |  [Overlay: Bet/Raise/   ||  | |  [Overlay: Bet/Raise/   | |      |
| | |   Call indicators]      ||  | |   Call indicators]      | |      |
| | |                         ||  | |                         | |      |
| | |  Auto-tracking active   ||  | |  Auto-tracking active   | |      |
| | +-------------------------+|  | +-------------------------+ |      |
| +-----------------------------+  +-----------------------------+      |
|                                                                       |
| +-----------------------------+  +-----------------------------+      |
| | TABLE 3                     |  | TABLE 4                     |      |
| | ...                         |  | ...                         |      |
| +-----------------------------+  +-----------------------------+      |
|                                                                       |
| STATUS BAR:                                                          |
| Tables: 4 active | Hands tracked: 247 | Auto-upload: ON             |
| Hotkeys: [Active] | Bet slider: [Custom sizes enabled]              |
+-----------------------------------------------------------------------+
```

**Table Wizard Features:**
- Real-time overlays showing bets, raises, and calls
- Hotkey support for fast action across tables
- Custom bet slider with preset sizes
- Automatic hand history upload to Analyzer 2.0 after each hand
- Seamless session-to-analysis workflow
- Platform: Windows desktop application

---

## 11. Settings / Preferences Screen

```
+-----------------------------------------------------------------------+
| SETTINGS                                                              |
+-----------------------------------------------------------------------+
|                                                                       |
| APPEARANCE                                                           |
| +----------------------------------------------------------------+   |
| | Theme: [Dark Mode v] [Import Theme] [Theme Gallery]            |   |
| | Card Theme: [Classic v] | Card Backs: [Blue v]                |   |
| | Background: [#1a1a2e]  | Text Color: [#ffffff]                |   |
| | Matrix Text: [Custom shadow and color options]                 |   |
| | Folded Hand BG: [Dimmed v]  | In-Range BG: [Bright v]        |   |
| | Action Colors:                                                 |   |
| |   Bet/Raise: [#E04040]  Check/Call: [#40B040]  Fold: [#4060D0]|   |
| |   Custom bet size colors: [Per size configurable]              |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| DISPLAY                                                              |
| +----------------------------------------------------------------+   |
| | Layout: [Horizontal v] [Horizontal Rev] [Split] [Split Rev]   |   |
| | Size: [Large] [Medium] [Compact]                               |   |
| | EV Display: [Big Blinds v] or [% of Pot]                      |   |
| | Bet Display: [BB v] or [% Pot]                                 |   |
| | Information Density: [Standard v]                               |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| HOTKEYS                                                              |
| +----------------------------------------------------------------+   |
| | Fully customizable keyboard shortcuts                          |   |
| | Default set included, user can remap all keys                  |   |
| +----------------------------------------------------------------+   |
|                                                                       |
| LANGUAGE: [English v] (17 languages available)                       |
|                                                                       |
| ACCOUNT                                                              |
| +----------------------------------------------------------------+   |
| | Subscription: [Premium - Cash 6max]                            |   |
| | Power Credits: 47/100 remaining                                |   |
| | Upload Limit: 234/500 hands remaining                          |   |
| | [Manage Subscription] [Billing History]                        |   |
| +----------------------------------------------------------------+   |
+-----------------------------------------------------------------------+
```

---

## 12. State Changes & Interactions Summary

### Global Interactions

| Trigger             | State Change                                          |
|---------------------|-------------------------------------------------------|
| Hover on matrix cell| Hand detail popup with suit breakdown appears         |
| Click action in table| Strategy matrix filters to show only that action     |
| Click filter tag    | Matrix dims non-matching hands                       |
| Press Spacebar      | Toggle between BB and % pot display                  |
| Press Q             | Maximize/minimize solution browser                    |
| Select metric dropdown | Matrix recolors based on EV/EQ/EQR/Weight         |
| Right-click matrix  | Context menu with range weight adjustment            |

### Trainer Interactions

| Trigger                  | State Change                                   |
|--------------------------|------------------------------------------------|
| Click action button      | Hand advances, scoring popup appears           |
| Hover action button      | EV preview shown                               |
| Best action selected     | Green checkmark, full points awarded           |
| Blunder selected         | Red indicator, points deducted, pause optional |
| Session complete (50 hands) | Summary screen with breakdown shown         |
| Click "View in Study"    | Opens same spot in Study Mode                  |

### Analyzer Interactions

| Trigger                  | State Change                                   |
|--------------------------|------------------------------------------------|
| Drop files in upload zone| Processing begins, progress indicator          |
| Click hand in table      | Replay view opens with GTO analysis            |
| Click column header      | Sort by that column                            |
| Apply filter             | Table filters, active filter badges appear     |
| Click "Open in Study"    | Navigates to exact spot in Study Mode          |
| Click "Open in Practice" | Opens spot in Trainer for drilling              |

---

## 13. Responsive Design & Platform Notes

| Platform      | Availability | Notes                                          |
|---------------|-------------|------------------------------------------------|
| Web (Desktop) | Full access | Recommended experience, all features           |
| Web (Mobile)  | PWA         | Progressive web app, installable via browser   |
| iOS           | App Store   | PokerArena dedicated app + GTO Preflop Wizard  |
| Android       | Browser PWA | Mobile browser-based, installable              |
| Windows       | Table Wizard| Native desktop app for table management        |

GTO Wizard is optimized for modern browsers and supports 2K and 4K resolution displays with adjustable information density settings.

---

## 14. Screenshot References

All collected screenshots are organized in the `screenshots/` directory:

- `screenshots/website/` -- 60 professional CDN screenshots (AVIF format, default + hover states)
- `screenshots/blog/` -- 43 blog post feature screenshots (PNG + GIF)
- `screenshots/reviews/` -- 15 third-party review screenshots (PNG + WebP)

See `screenshots/MANIFEST.md` for the complete index mapping every image to its feature, source, and related documentation.

### Key Screenshot Mappings by Section

| Doc Section | Feature | Key Screenshots |
|-------------|---------|----------------|
| Section 2 (Dashboard) | Home screen | `website/study-anchor-default.avif`, `reviews/h2n-dashboard.webp` |
| Section 3 (Study) | Strategy matrix | `website/ai-solver-default.avif`, `reviews/solvers-poker-main.png`, `reviews/h2n-library.webp` |
| Section 4 (Practice) | GTO Trainer | `website/practice-*-default.avif`, `reviews/solvers-poker-trainer.png`, `reviews/h2n-trainer.webp` |
| Section 5 (Analyze) | HH Analyzer | `blog/analyzer2-*.png`, `blog/redesigned-*.png`, `reviews/h2n-analyzed-hands.webp` |
| Section 6 (Range Builder) | Range grid | `blog/nodelocking-interface.png` |
| Section 7 (AI Solver) | Custom solving | `website/ai-solver-default.avif`, `blog/3way-*.png` |
| Section 8 (Nodelocking) | Lock interface | `blog/nodelocking-*.png`, `blog/nodelocking-*.gif` |
| Section 9 (PokerArena) | Arena game | `website/battle-heads-up-poker-default.avif`, `website/master-your-game-default.avif`, `website/prove-you-are-the-best-default.avif` |
| Section 10 (Table Wizard) | Multi-table | `website/streamlined-table-layouts.avif`, `blog/table-wizard-hero.png`, `blog/table-wizard-overlay.png` |
| Section 11 (Settings) | Preferences | `reviews/h2n-settings.webp` |

---

## Sources

- https://help.gtowizard.com/study-mode/
- https://help.gtowizard.com/how-to-use-the-trainer/
- https://help.gtowizard.com/analyze-mode-guide/
- https://help.gtowizard.com/subscription/
- https://blog.gtowizard.com/simplified-solutions-and-a-new-interface/
- https://blog.gtowizard.com/gto-wizard-themes-winners/
- https://blog.gtowizard.com/redesigned_analyzer_and_upgraded_gto_reports/
- https://blog.gtowizard.com/introducing_table_wizard_the_ultimate_table_management_software/
- https://blog.gtowizard.com/enter_the_poker_arena_a_new_era_of_competitive_poker_has_begun/
- https://blog.gtowizard.com/whats-new-in-gto-wizard/
- https://solvers.poker/review/gtowizard/
- https://hand2noteguide.com/gto-wizard-review/
- https://gtowizard.com

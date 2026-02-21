# Screenshot Gap Analysis: Visual Reference vs. Documentation

## Methodology

This analysis compares 21+ screenshots from three sources (website CDN, blog posts, third-party reviews) against nine existing documentation files (docs 03 through 11). The goal is to identify discrepancies between the real GTO Wizard UI and our current ASCII mockups and component descriptions, and to catalog missing elements, incorrect layouts, and undocumented features.

**Screenshots reviewed**: 21 images across `/mnt/c/python/poker_solver/screenshots/blog/`, `/mnt/c/python/poker_solver/screenshots/reviews/`, and `/mnt/c/python/poker_solver/screenshots/website/`. Note: AVIF-format files from the website directory could not be rendered and are excluded.

**Documentation reviewed**: `03_study_mode.md`, `04_practice_mode.md`, `05_analyze_mode.md`, `06_gto_wizard_ai.md`, `07_nodelocking.md`, `08_aggregated_reports.md`, `09_tournament_icm.md`, `10_range_builder.md`, `11_ui_ux_screens.md`.

---

## Summary of Findings

| Feature Area | Doc Accuracy | Key Gaps | Priority |
|---|---|---|---|
| Dashboard | Moderate | Real dashboard has 9-tile grid with icons, Trainer stats and Analyzer stats panels at bottom; docs show simpler layout | Medium |
| Study Mode | Good | Tabs confirmed (Strategy+EV, Ranges, Breakdown, Reports: Flops); "Reports: Flops" label not "Reports" in docs; dual-panel view with two independent tab bars visible in aggregated reports screenshot | Low |
| Practice Mode | Good | Action buttons use color-coded sizing labels (CHECK green, BET amounts in graduated red shades, ALLIN darkest red); right sidebar has Strategy/Range tabs with 13x13 matrix; scoring icons differ from docs (checkmark=Best, checkmark=Correct, circle-i=Inaccuracy, circle-i=Wrong, triangle=Blunder) | Medium |
| Analyze Mode (Analyzer 2.0) | Moderate | 6-category scoring system confirmed but labels differ from docs; "Pot Type" column (SRP/3Bet/ISO/LIMP/SQUEEZE) not fully documented; filter system is far more complex than described; natural language filter search is undocumented | High |
| GTO Reports | Low | Entire feature area is underdocumented; Game Type selector (Cash/MTT/Spins/HuSng), position-vs-position matrix, VALUE vs GTO comparison bars, deviation indicators, Fold/Call/Raise sub-tabs, defense analysis views are all missing from docs | Critical |
| Nodelocking | Good | Game tree is horizontal card-based (not vertical tree); lock/pencil/compare icons inline on game tree; hand class filtering in Set Strategy tab visible | Low |
| Aggregated Reports | Good | Chart view uses stacked red/green bar chart; table view has Flops/Strategy/Check/Bet sizes/OOP/EV/IP/EQ/EQR columns; "Custom reports" tab in study mode selector confirmed; 1755 flops confirmed in screenshot | Low |
| Range Builder | No screenshots | Could not verify -- no renderable screenshots available | Unknown |
| Table Wizard | Moderate | Overlay screen with toggle categories (Table Borders, RNG, Betting Buttons, Actions, Table Info) not documented; left sidebar nav (Dashboard, Table Layout, Overlays, Hot Keys, Automated Controls) not documented; poker table preview with position-specific overlays visible | High |
| PokerArena | No screenshots | Could not verify -- no screenshots available | Unknown |
| Navigation / Top Bar | Moderate | Real nav shows: W logo, "GTO Wizard", Play, Study, Practice, Analyze tabs; right side has Upload button, gear/settings icons, user avatar; "Go Elite" upsell badge shown for non-Elite users | Medium |
| Color System | Good | Confirmed: green=check/call, red=bet/raise (graduated by size), blue=fold; dark theme (#1a1a2e-like background) throughout; teal/cyan accent for primary actions and highlights | Low |
| Scoring Categories | Moderate | Real UI uses PERFECT/GOOD/INNACURATE/WRONG/BLUNDER (note: "INNACURATE" typo in actual UI); docs use Best/Correct/Inaccuracy/Wrong/Blunder -- terminology mismatch | High |

---

## Detailed Analysis by Feature

### 1. Dashboard

**Source screenshots**: `h2n-dashboard.webp` (reviews)

**What the real UI shows**:
- Top navigation bar: W logo + "GTO Wizard" | "Go Elite" badge | Study | Practice | Analyze | Upload button | settings icons | user avatar
- Main content: "Dashboard" header with edit (pencil) icon
- 9-tile grid of feature shortcuts arranged in 3x3:
  - Row 1: Study ("Study any spot you want"), Trainer ("Play vs. GTO opponent"), Uploads ("Analyze your game")
  - Row 2: Custom solutions ("Use AI to solve any spot"), Range builder ("Practice range construction"), Hands ("Study analyzed hands")
  - Row 3: Coaching ("Live coaching with pros"), Drills ("Manage training drills"), Help center ("Tips & Tricks for GTO Wizard")
- Each tile has a colored icon (teal/green/orange themed)
- Bottom section: two side-by-side stat panels:
  - "Trainer stats" panel: HANDS, MOVES, MISTAKES, GTOW SCORE with colored horizontal bars for Best (teal), Correct (green), Inaccuracy (amber/yellow), Wrong (amber), Blunder (red)
  - "Analyzer stats" panel: TOTAL, CORRECT, WRONG, AVG. EV LOSS

**Documentation gaps**:
- `11_ui_ux_screens.md` describes a dashboard with "Quick Navigation Tiles" and "Recent Activity" / "Stats Summary" but does not list exact tile names, icons, or the Coaching/Drills/Help center tiles
- Trainer stats panel scoring labels use "Best/Correct/Inaccuracy/Wrong/Blunder" in the dashboard (matching the older docs) but Analyzer 2.0 tables use "PERFECT/GOOD/INNACURATE/WRONG/BLUNDER" -- this inconsistency exists in the real product
- "Go Elite" upsell badge position and behavior not documented
- Edit (pencil) icon on Dashboard header suggesting customizable layout is not mentioned

### 2. Study Mode

**Source screenshots**: `aggregated-reports.png` (website), `redesigned-flop-general-view.png` (blog), `aggregated-reports-custom-tab.png` (blog)

**What the real UI shows**:
- Tab bar within Study Mode: "Strategy + EV" (dropdown), "Ranges", "Breakdown", "Reports: Flops" (dropdown)
- The tab label is "Reports: Flops" with a dropdown arrow, not simply "Reports" as in docs
- Left sidebar shows spot configuration: position matchup (e.g., "BB vs. BTN"), Stack depth, Pot size, "Change" button
- Study mode selector has three sub-tabs: "Solutions library", "Custom solutions", "Custom reports"
- Custom reports tab shows a table with columns: Name, Date, Positions, Model (ChipEV/ICM)
- Flop selector shows three card placeholders with "W" watermarks when unselected

**Documentation gaps**:
- The "Reports: Flops" label and its dropdown nature should be specified in `03_study_mode.md`
- The "Solutions library / Custom solutions / Custom reports" sub-selector within Study Mode is not fully described
- Custom reports table structure (Name, Date, Positions, Model) not documented
- The "Strategy + EV" combined dropdown (rather than separate Strategy and EV tabs) differs from docs

### 3. Practice Mode

**Source screenshots**: `solvers-poker-trainer.png` (reviews), `h2n-trainer.webp` (reviews)

**What the real UI shows**:
- Central poker table with oval shape, position labels around the rim (UTG, HJ, CO, BTN, SB, BB)
- Player stacks shown as numbers next to position labels (e.g., "BB 120", "CO 120")
- Board cards displayed center-table as large colored card images (red for hearts/diamonds, blue for clubs, green for spades)
- Pot size displayed above board cards (e.g., "15.1 BB") with pot icon
- Spot description below table top: "CO vs. BB, 4-bet, 150bb"
- Action buttons in colored blocks below the table:
  - CHECK (green, full width or partial)
  - BET amounts in red gradient (BET 6.05 lighter red, BET 15.1 darker red)
  - BET 30.25 (medium red), BET 45.4 (darker), BET 60.5 (darkest red)
  - ALLIN (darkest red, full width)
- Top action bar: scrollable list of previous actions in the hand (e.g., "UJ 150 Fold | CO 150 Raise 2.5 | BTN 150 Fold | ...")
- Left sidebar: Controls dropdown, Stats panel showing Hands/Moves/GTOW Score counts, and scoring category legend with colored icons
- Right sidebar: "Info" panel with Strategy/Range tabs, 13x13 hand matrix, Actions list with percentages, Hand info section (Strength, Equity bars), Hands distribution, History section
- "CHANGE MOVE" and "CONTINUE" buttons below action area
- EV shown for each action option (e.g., "85.1% / 17.21 EV")

**Documentation gaps**:
- `04_practice_mode.md` does not describe the action button color gradient system (graduated reds by bet size)
- The left sidebar "Controls" and "Stats" panels are not documented in detail
- The right sidebar Info panel with Strategy/Range tabs is described in `11_ui_ux_screens.md` but lacks specifics about the Hand info sub-sections (Strength bar, Equity bar, hand classification)
- "CHANGE MOVE" / "CONTINUE" button flow not documented
- The scrollable action history bar at top is not described
- EV display next to each action option during decision-making is not documented

### 4. Analyze Mode (Analyzer 2.0)

**Source screenshots**: `analyzer2-hands-table.png`, `analyzer2-hand-analysis.png`, `analyzer2-position-breakdown.png`, `analyzer2-preflop-action.png`, `analyzer2-srp-report.png`, `analyzer2-street-decisions.png`, `redesigned-hand-filtering.png`, `redesigned-natural-language-filter.png`, `redesigned-preflop-mistakes.png` (all blog)

**What the real UI shows**:

**Hands Table View**:
- Header: "Hands" in large bold text | "All Reports" button | "New Report" | "Save Report"
- Filter tabs: Filters | Streets Actions | Hands Details | Statistics and Results | Hole Cards | Game Type | Other | Search icon
- Table columns: checkbox, Pot (bb), Tag, Status, Format, Site, Position, Hand (card images), Board (card images), Pot Type, Preflop (action codes like "RRC", "FFRFF"), Flop actions, and more
- Active/Source/File Upload sub-tabs
- Pot Type categories visible: SRP, 3Bet, ISO, LIMP, SQUEEZE, Preflop
- Status icons: green checkmark (analyzed), red icon (errors)

**Stats View** (from analyzer2-srp-report.png):
- Top navigation: "GTO Wizard" | Play | Study | Practice | Analyze (highlighted)
- Report tabs: "+ New Report" | "SRP - Went Postflop" (active report name)
- Sub-tabs: Table | Stats | Site | Blinds | Pot type: SRP | Hero went postflop: Yes | Filters
- Five stat cards in a row:
  - GTO Score: 70.6% (with sparkline graph showing trend over time, scale 60-80)
  - Hands: 2254 (6116 moves)
  - EV Loss: 5.081 bb (per 100 hands)
  - Avg EV Loss: 0.74% (per 100 mistakes -- note: label says "per 100 mistakes" not "per hand")
  - Frequency Diff: 55.58%
- Breakdown table below stats: dropdown selector (Street/Position/Pot Type), columns: TOTAL, PERFECT, GOOD, INNACURATE, WRONG, BLUNDER
- Score Breakdown panel: horizontal colored bars (Perfect=green, Good=teal, Innacurate=yellow/amber, Mistake=red, Blunder=red dot)
- Simple/Complex toggle at bottom left
- "Add Table" button at bottom right

**Advanced Filtering**:
- Filter bar with categories: All, Date, Game, Hero (with count badges), Statistics, Other
- IP/OOP toggle
- "Flop Line - Actions Sequence" builder with rows of toggleable buttons: Check | Bet (with S/M/L/O sub-sizes)
- "Turn Line - Actions Sequence" with same pattern plus "OB" (overbet) indicator
- Response rows: Fold | Call | Raise (with M/L/O sub-sizes)
- "Remove filter" option per filter
- Natural language filter search: typing "did" shows autocomplete: "Hero Preflop Action (Did RFI)", "Hero Preflop Action (Did Limp)", "Hero Preflop Action (Did ISO)", "Hero Preflop Action (Did Squeeze)", "Hero Preflop Action (Did 3Bet)", "Hero Preflop Action (Did 4Bet)", "Hero Preflop Action (Did 5Bet)"

**Documentation gaps**:
- `05_analyze_mode.md` scoring categories use "Best/Correct/Inaccuracy/Wrong/Blunder" but the real UI consistently shows "PERFECT/GOOD/INNACURATE/WRONG/BLUNDER" in table headers -- this is a critical terminology mismatch
- The Score Breakdown panel on the right side uses "Mistake" instead of "Wrong" in some views, creating a third variant
- The "Frequency Diff" stat card is not mentioned in docs
- "GTO Score" sparkline trend graph is not described
- The "Avg EV Loss per 100 mistakes" metric framing is not documented (docs say "per hand")
- Filter system is vastly more complex than documented:
  - Streets Actions tab with action sequence builder (Check/Bet S/M/L/O patterns) is not described
  - Hands Details, Statistics and Results, Hole Cards, Game Type, Other filter tabs are listed but content not detailed
  - Natural language filter search with autocomplete is completely undocumented
  - "Custom Flop Line" with multi-row action sequence patterns is undocumented
- Pot Type categories (SRP, 3Bet, ISO, LIMP, SQUEEZE) as filter/grouping options need more detail
- The "Hero Saw Flop" toggle in filtering is not mentioned
- Simple/Complex toggle for breakdown tables is not documented
- "Add Table" button for adding additional breakdown tables is not documented
- Report naming and saving ("New Report", "Save Report", "All Reports") workflow not documented

### 5. GTO Reports

**Source screenshots**: `gto-reports-preflop-stats.png`, `gto-reports-filtering.png`, `gto-reports-position-matrix.png`, `gto-reports-stat-comparisons.png`, `gto-reports-defense-analysis.png`, `gto-reports-postflop-actions.png`, `redesigned-preflop-mistakes.png` (all blog)

**What the real UI shows**:

**Game Type Selector**:
- Header: "GTO Reports" in large bold text with "How to use reports" link
- "Game Type" dropdown showing: "Cash, 6max, 100bb, NL500, GTO"
- Expandable configuration panel:
  - Game Format: Cash | MTT | Spins | HuSng (toggle buttons)
  - Players: 6max (pill button)
  - Stacks: 100bb | 200bb (toggle buttons)
  - Rake: NL50 | NL500 (toggle buttons)
  - Opening Size: GTO (pill button)

**Preflop Stats Comparison**:
- Left panel showing preflop statistics with VALUE vs GTO columns:
  - Stats listed: VPIP, PFR, RFI, LIMP, SQUEEZE, 3BET, 4BET, 5BET
  - Each stat has a horizontal deviation bar: green checkmark when matching GTO, red indicator when over GTO, blue indicator when under GTO
  - Numerical values shown for both actual VALUE and GTO reference
- Deviation indicator legend: colored bar with center line representing GTO, deviation shown as colored extension left (under) or right (over)

**Position Matrix**:
- "3BET" header with "vs 3BET" / "GTO Values" toggle
- "by Position" breakdown: UTG through BB with VALUE and GTO columns
- Horizontal bars showing deviation from GTO per position
- "3BET vs RFI" specific matrix: 6x6 grid (UTG/HJ/CO/BTN/SB/BB vs UTG/HJ/CO/BTN/SB/BB)
- Each cell shows two numbers (VALUE above, GTO below) with color coding (red when significantly deviating)

**Defense Analysis**:
- "3BET" header with sub-tabs: Total | Raise | Call | Fold
- "Fold vs 3BET by Position" view with same VALUE/GTO bar format
- "Fold vs 3BET Specific" matrix: position-vs-position grid
- Warning icons (amber circle with !) next to positions with significant deviations
- Color coding: green when matching, red when over-folding, blue when under-folding

**Postflop Actions View**:
- "Set Your Filter Bar" modal/panel
- Left sidebar categories: All, Date, Game, Hero (with count), Statistics, Other
- IP/OOP toggle
- Flop Line and Turn Line action sequence builders

**Overall GTO Reports Layout** (from redesigned-preflop-mistakes.png):
- Three-column layout:
  - Left: Preflop stats panel with bars and tabs
  - Center: Position breakdown table + Mistakes list (with EV Loss per mistake, GTO action, Hero action)
  - Right: Definition/articles panel with contextual help

**Documentation gaps**:
- GTO Reports is barely mentioned in existing docs. It needs its own dedicated documentation file
- The entire Game Type selector with Cash/MTT/Spins/HuSng, Players, Stacks, Rake, Opening Size configuration is undocumented
- Preflop stat categories (VPIP, PFR, RFI, LIMP, SQUEEZE, 3BET, 4BET, 5BET) are not documented
- VALUE vs GTO comparison format with deviation bars is not documented
- Position-vs-position matrix views are not documented
- Defense analysis views (Fold/Call/Raise sub-tabs by position) are not documented
- Warning icons for significant deviations are not documented
- The three-column layout with contextual help panel is not documented
- Filtering by date range and "Deviation from GTO" (All/Higher/Lower) with Absolute/relative values is not documented
- This represents the single largest documentation gap in the project

### 6. Nodelocking

**Source screenshots**: `nodelocking-interface.png`, `nodelocking-set-frequency.png`, `redesigned-nodelocking-handclass.png` (all blog)

**What the real UI shows**:

**Game Tree Navigation**:
- Horizontal card-based game tree (not vertical as ASCII mockups suggest)
- Shows: FLOP card images (e.g., A98) --> OOP action nodes --> IP action nodes --> OOP response nodes
- Each node shows action taken (Check, Bet size, etc.)
- Inline icons on hover: pencil (edit strategy), lock (nodelock), compare (compare nodes)
- "Nodelock" button prominently displayed

**Set Frequency Tab**:
- Three tabs clearly visible: "Set strategy" | "Set frequency" | "Lock / Unlock"
- "Overwrite unlocked" / "Overwrite all" toggle (matches docs)
- Colored horizontal frequency bars:
  - Raise: red bar, 14% (with editable percentage input)
  - Call: green bar, 52%
  - Fold: blue bar, 34%
- Lock icon per action line (to lock individual actions)
- Percentage input boxes for exact values

**Hand Class View** (from nodelocking-handclass.png):
- Complex interface combining:
  - Left panel: 13x13 hand matrix with colored cells
  - Center: "Current Strategy" and other strategy panels with detailed hand class breakdowns
  - Horizontal colored bars per hand category showing action distribution
  - Multiple data columns visible

**Documentation gaps**:
- `07_nodelocking.md` ASCII mockups show a vertical tree layout; the real UI uses horizontal card-based navigation -- this should be corrected
- The inline hover icons (pencil, lock, compare) on game tree nodes are not described
- Per-action lock icons in Set Frequency tab are not mentioned (docs only describe the Lock/Unlock tab)
- The hand class filtering view shown in the redesigned screenshot is far more detailed than documented
- The editable percentage input boxes alongside sliders in Set Frequency are not mentioned

### 7. Aggregated Reports

**Source screenshots**: `aggregated-reports-flop-cbet.png` (blog), `aggregated-reports.png` (website)

**What the real UI shows**:

**Chart View**:
- Stacked bar chart with red and green segments (red = betting actions, green = checking)
- Bars organized along x-axis by flop category
- Right panel showing summary: action buttons (Bet 5.5, Bet 1.7, Check) with percentages (17.5%, 36.9%, 45.7%)

**Table View** (from aggregated-reports.png):
- Tab bar: "Strategy + EV" | "Ranges" | "Breakdown" | "Reports: Flops"
- Two independent panels visible side by side, each with their own tab bars
- Table columns: Flops | Strategy (colored bars) | Check | Bet 66% | Allin 333% | OOP | EV | IP | OOP | EQ | IP
- Row examples: "Used by" row showing 1755 (100%) and 1752 (99.8%) and 3 (0.2%)
- Individual flop rows: A A A, A A K, etc.
- Strategy column shows red/green stacked mini-bars per flop

**Documentation gaps**:
- `08_aggregated_reports.md` describes the concepts well but lacks visual specifics
- The stacked bar chart format (red=bet, green=check) for the chart view is not described
- Specific column headers in the table view (Flops, Strategy bars, individual bet sizes, OOP/IP split for EV and EQ) are not listed
- The "Used by" summary row showing how many of 1755 flops use each action is not described
- The dual-panel side-by-side layout with independent tab bars is not documented
- The action summary buttons with percentages in the chart view right panel are not documented

### 8. Table Wizard

**Source screenshots**: `table-wizard-overlay.png`, `table-wizard-hero.png` (blog)

**What the real UI shows**:

**Overlay Configuration Screen**:
- Header: "W Table Wizard BETA" with "Active" toggle (green)
- Page title: "Overlays" with subtitle "Visual aids to enhance your gameplay experience."
- Left sidebar navigation:
  - Dashboard
  - Table Layout
  - Overlays (currently active, highlighted)
  - Hot Keys
  - Automated Controls
- Five toggle categories with expand arrows:
  - Table Borders (green toggle on)
  - RNG (green toggle on)
  - Betting Buttons (grey toggle off)
  - Actions (green toggle on)
  - Table Info (green toggle on)
- Preview area: poker table visualization showing:
  - Position labels around oval table (UTG, UTG1, LJ, HJ, CO, BU, SB, BB)
  - Stack/SPR/PO/EFF overlay stats (e.g., stack 96, SPR 0.45, PO 31.1%, EFF 35.4)
  - Per-position colored action indicators (red for raises with percentages, green for checks)
  - Preview toolbar: zoom, edit, settings icons

**Hero Page** (marketing):
- "Table Wizard is here!" announcement
- "The Ultimate Table Management Software for Online Poker"
- "Download for Free Now" button (green)
- Windows icon (Available Now) and Apple icon (Coming Soon)
- Shows a laptop mockup with the overlay interface

**3-Way Solving Compatibility Table** (from 3way-solving-interface.png):
- Shows solving capabilities matrix:
  - Heads Up: Preflop=Customizable, Postflop=Customizable
  - 3-way: Preflop=Static, Postflop=Customizable
  - 4-way+: Preflop=Static, Postflop=Not Available

**Documentation gaps**:
- Table Wizard is mentioned in `11_ui_ux_screens.md` with a basic ASCII mockup but lacks:
  - The left sidebar navigation structure (Dashboard, Table Layout, Overlays, Hot Keys, Automated Controls)
  - The five toggle overlay categories and their specific options
  - The "Active" toggle for enabling/disabling the tool
  - The "BETA" badge
  - Preview area with per-position overlay indicators
  - Stack/SPR/PO/EFF stat overlay format
  - Platform availability (Windows only, Mac "Coming Soon")
- 3-way solving compatibility matrix should be documented somewhere (either in `06_gto_wizard_ai.md` or a separate multiway section)

### 9. Range Builder

**Source screenshots**: None renderable (AVIF format files could not be viewed)

**Status**: Cannot verify. Documentation in `10_range_builder.md` could not be compared against screenshots.

### 10. PokerArena

**Source screenshots**: None available

**Status**: Cannot verify. No screenshots of PokerArena were found in any of the three screenshot directories.

---

## Component Inventory Updates

### Components Confirmed by Screenshots

| Component | Documented | Screenshot Verified | Notes |
|---|---|---|---|
| 13x13 Hand Matrix | Yes | Yes | Visible in Practice Mode right sidebar, nodelocking |
| Action Color Coding (red/green/blue) | Yes | Yes | Red=bet/raise, Green=check/call, Blue=fold confirmed |
| Dark Theme | Yes | Yes | Consistent dark background (#1a1a2e range) throughout all screenshots |
| Poker Table (oval) | Yes | Yes | Positions around rim, cards center, pot above |
| Top Navigation Bar | Partially | Yes | Play/Study/Practice/Analyze confirmed; icons differ from docs |
| Scoring Category Table | Partially | Yes | Column structure confirmed but labels differ |
| Stacked Bar Charts | Partially | Yes | Aggregated reports chart view confirmed |
| Game Tree | Partially | Yes | Horizontal card-based, not vertical tree |

### Components Found in Screenshots but Not Documented

| Component | Where Found | Description |
|---|---|---|
| GTO Score Sparkline | Analyzer Stats view | Mini trend graph showing score over time |
| Natural Language Filter Search | Analyzer Hands table | Autocomplete search with "Hero Preflop Action (Did X)" suggestions |
| Action Sequence Builder | Analyzer Filter Bar | Multi-row Check/Bet(S/M/L/O)/Fold/Call/Raise pattern builder |
| VALUE vs GTO Deviation Bars | GTO Reports | Horizontal bars with center-line GTO reference and colored deviation |
| Position-vs-Position Matrix | GTO Reports | 6x6 grid for stat comparison between position pairs |
| Score Breakdown Horizontal Bars | Analyzer Stats | Colored bars by category with count labels |
| Per-Action Lock Icons | Nodelocking Set Frequency | Lock icon next to each action's frequency bar |
| Overlay Toggle Categories | Table Wizard | Table Borders/RNG/Betting Buttons/Actions/Table Info toggles |
| Report Naming System | Analyzer | "New Report" / "Save Report" / named report tabs |
| "Go Elite" Upsell Badge | Dashboard / Navigation | Green badge next to logo for non-Elite users |
| Custom Flop Line Builder | Analyzer Filtering | Multi-row action sequence pattern definition |
| Frequency Diff Metric | Analyzer Stats | Percentage showing overall frequency deviation from GTO |
| Simple/Complex Toggle | Analyzer Breakdown | Switches between simplified and detailed breakdown views |
| Add Table Button | Analyzer Breakdown | Allows adding multiple breakdown tables to the same view |

### Components Documented but Not Verified

| Component | Document | Reason |
|---|---|---|
| Range Builder paintbrush tool | `10_range_builder.md` | No renderable screenshots (AVIF only) |
| Range Builder weight slider | `10_range_builder.md` | No renderable screenshots |
| PokerArena matchmaking | `11_ui_ux_screens.md` | No screenshots available |
| PokerArena leaderboard | `11_ui_ux_screens.md` | No screenshots available |
| Settings/Themes page | `11_ui_ux_screens.md` | No screenshots available |
| ICM calculator interface | `09_tournament_icm.md` | No renderable screenshots |
| Keyboard shortcuts overlay | `03_study_mode.md` | No screenshots available |

---

## Color System Verification

### Confirmed Color Mappings

| Element | Documented Color | Screenshot Color | Match? |
|---|---|---|---|
| Fold action | Blue | Blue | Yes |
| Check/Call action | Green | Green | Yes |
| Bet/Raise action | Red | Red (graduated by size) | Partial -- gradient not documented |
| Background (dark theme) | Dark (#1a1a2e range) | Dark charcoal/navy | Yes |
| Primary accent | Teal/Cyan | Teal/Green (#4CAF50 range) | Yes |
| "Best" score | Green | Teal/Green (double checkmark) | Yes |
| "Correct"/"Good" score | Green | Green (single checkmark) | Yes |
| "Inaccuracy" score | Amber/Yellow | Amber/Yellow (circle-i icon) | Yes |
| "Wrong"/"Mistake" score | Orange/Red | Red (circle-i icon) | Partial -- icon differs |
| "Blunder" score | Red | Red (triangle-! icon) | Yes |

### Newly Observed Color Patterns

| Pattern | Description | Where Observed |
|---|---|---|
| Graduated red for bet sizes | Lighter red for small bets, progressively darker for larger bets, darkest for all-in | Practice Mode action buttons |
| Green with checkmark for GTO match | Green circle with checkmark when VALUE matches GTO exactly | GTO Reports deviation bars |
| Red deviation indicator | Red colored bar extension when VALUE exceeds GTO | GTO Reports stat comparisons |
| Blue deviation indicator | Blue colored bar extension when VALUE is below GTO | GTO Reports stat comparisons |
| Amber warning icon | Circle with exclamation mark next to positions with significant deviation | GTO Reports defense analysis |
| Teal highlight for active tabs | Active nav items and filter tabs highlighted in teal/cyan | Throughout all views |

---

## Typography and Spacing Observations

### Font Usage (observed from screenshots)

| Element | Observed Style | Notes |
|---|---|---|
| Page titles | Large bold sans-serif, white | "Hands", "GTO Reports", "Dashboard" -- prominent, ~24-32px equivalent |
| Section headers | Medium bold, white | "Trainer stats", "Analyzer stats" -- ~18-20px equivalent |
| Table headers | Uppercase, semi-bold, muted grey | "TOTAL", "PERFECT", "GOOD", etc. -- tracking/letter-spacing visible |
| Table data | Regular weight, white or colored | Numbers right-aligned, text left-aligned |
| Navigation items | Regular weight, muted white | Active item highlighted with color |
| Stat card values | Extra-large bold | "70.6%", "2254" -- hero numbers ~36-48px equivalent |
| Stat card labels | Small, muted grey | "GTO Score", "Hands", "EV Loss" -- ~12-14px equivalent |
| Button text | Medium weight, uppercase or title case | "CHECK", "BET 30.25", "ALLIN 120" |

### Spacing Patterns

| Pattern | Description |
|---|---|
| Card spacing | Stat cards in horizontal row with equal gaps, rounded corners, subtle border |
| Table row height | Generous row heights (~48-60px) for readability on dark background |
| Panel padding | Consistent internal padding (~16-24px) in all panels |
| Grid layout | Dashboard uses 3-column grid; Stats view uses 5-column card row; Analysis uses 2-3 column layouts |
| Sidebar width | Left sidebars ~200-250px; Right info panels ~300-350px |

### Key Typography Differences from Documentation

- Docs do not specify the uppercase treatment for table column headers (TOTAL, PERFECT, etc.)
- Docs do not describe the stat card "hero number" pattern (large value + small label below)
- The muted grey color for secondary text and labels is not specified in docs

---

## Recommendations

### Critical Priority (terminology/accuracy issues)

1. **Create dedicated GTO Reports documentation**: This is the largest gap. GTO Reports needs its own comprehensive doc file covering the Game Type selector, preflop stat categories (VPIP/PFR/RFI/etc.), VALUE vs GTO deviation bars, position matrices, defense analysis views, and filtering.

2. **Standardize scoring terminology**: The documentation uses "Best/Correct/Inaccuracy/Wrong/Blunder" but the actual UI uses "PERFECT/GOOD/INNACURATE/WRONG/BLUNDER" in Analyzer 2.0. The Dashboard trainer stats still use the old terminology. Both variants should be documented, with the Analyzer 2.0 version treated as the primary modern terminology.

3. **Document the natural language filter system**: The Analyzer 2.0 has an advanced autocomplete search that allows filtering by "Hero Preflop Action (Did RFI/Limp/ISO/Squeeze/3Bet/4Bet/5Bet)" and likely many more patterns. This is completely absent from docs.

### High Priority (missing features)

4. **Document the action sequence builder**: The Analyzer filter system includes a visual "Flop Line" and "Turn Line" action sequence builder with Check/Bet(S/M/L/O)/Fold/Call/Raise toggleable rows. This powerful filtering mechanism is not documented.

5. **Update Table Wizard documentation**: Add the left sidebar navigation structure, five overlay toggle categories, preview area, BETA status, and platform availability.

6. **Document the Analyzer Stats view**: The five stat cards (GTO Score with sparkline, Hands, EV Loss, Avg EV Loss, Frequency Diff) and the Score Breakdown panel need full documentation.

7. **Document the Simple/Complex toggle**: This toggle in the Analyzer breakdown tables switches between simplified and detailed views and affects data presentation.

### Medium Priority (layout corrections)

8. **Correct nodelocking game tree representation**: Update ASCII mockups from vertical tree to horizontal card-based layout to match the real UI.

9. **Update Dashboard documentation**: Add the 9-tile grid with specific tile names and icons, the Trainer stats and Analyzer stats bottom panels, and the edit/customization capability.

10. **Document bet size color gradient**: The graduated red system for bet sizes (lighter for smaller bets, darker for larger, darkest for all-in) should be added to the color system documentation.

11. **Add report management documentation**: The "New Report" / "Save Report" / "All Reports" system and named report tabs in Analyzer 2.0 need documentation.

### Low Priority (refinements)

12. **Update aggregated reports visuals**: Add specifics about the stacked bar chart format, table column headers, "Used by" summary row, and dual-panel layout.

13. **Document the "Go Elite" upsell badge**: Its positioning, appearance, and behavior for non-Elite users.

14. **Add typography specification**: Document the uppercase table headers, stat card hero-number pattern, and muted grey secondary text color.

15. **Obtain PNG screenshots for Range Builder and PokerArena**: The AVIF format files could not be analyzed. PNG or WEBP alternatives should be sourced to complete the visual gap analysis for these features.

---

## Appendix: Screenshots Reviewed

### Blog Screenshots (blog/)
1. `gto-reports-preflop-stats.png` -- GTO Reports preflop stat comparison
2. `gto-reports-filtering.png` -- GTO Reports date/deviation filtering
3. `nodelocking-interface.png` -- Nodelocking game tree navigation
4. `nodelocking-set-frequency.png` -- Nodelocking Set Frequency tab
5. `analyzer2-hands-table.png` -- Analyzer 2.0 hands table view
6. `analyzer2-hand-analysis.png` -- Analyzer 2.0 hand replay
7. `analyzer2-position-breakdown.png` -- Analyzer 2.0 position breakdown table
8. `redesigned-preflop-mistakes.png` -- GTO Reports full interface
9. `redesigned-flop-general-view.png` -- GTO Reports flop CBET view
10. `aggregated-reports-flop-cbet.png` -- Aggregated reports chart view
11. `3way-solving-interface.png` -- 3-way solving compatibility matrix
12. `table-wizard-overlay.png` -- Table Wizard overlay configuration
13. `table-wizard-hero.png` -- Table Wizard marketing/hero page
14. `redesigned-hand-filtering.png` -- Analyzer advanced hand filtering
15. `redesigned-natural-language-filter.png` -- Analyzer natural language filter search
16. `redesigned-nodelocking-handclass.png` -- Nodelocking hand class view
17. `analyzer2-preflop-action.png` -- Analyzer preflop action breakdown by pot type
18. `analyzer2-srp-report.png` -- Analyzer SRP stats report view
19. `analyzer2-street-decisions.png` -- Analyzer street-by-street breakdown
20. `gto-reports-stat-comparisons.png` -- GTO Reports game type selector
21. `gto-reports-position-matrix.png` -- GTO Reports 3BET position matrix
22. `gto-reports-defense-analysis.png` -- GTO Reports fold vs 3BET defense
23. `gto-reports-postflop-actions.png` -- GTO Reports postflop filter bar
24. `aggregated-reports-custom-tab.png` -- Custom reports tab in Study Mode

### Review Screenshots (reviews/)
25. `solvers-poker-main.png` -- GTO Wizard homepage/marketing
26. `solvers-poker-trainer.png` -- Practice mode full view
27. `h2n-dashboard.webp` -- Dashboard with all tiles and stats
28. `h2n-trainer.webp` -- Practice mode trainer detailed view

### Website Screenshots (website/)
29. `aggregated-reports.png` -- Aggregated reports dual-panel view

### Not Renderable (AVIF format)
- All `.avif` files from the website/ directory (approximately 60 files) could not be rendered by the image viewing tool and are excluded from this analysis.

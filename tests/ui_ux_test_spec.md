# UI/UX Acceptance Test Specification

Comprehensive acceptance test suite for the GTO Wizard poker solver application rebuild. Each test has a unique ID, description, expected behavior, and priority level.

**Priority Levels:**
- **P1 (Critical):** Core functionality that must pass for the app to be usable
- **P2 (High):** Important features that affect primary workflows
- **P3 (Medium):** Nice-to-have features, visual polish, secondary workflows
- **P4 (Low):** Edge cases, minor cosmetics, tertiary features

**Total Tests:** 160

---

## 1. Layout & Structure Tests (LS-001 through LS-030)

Tests for correct component hierarchy, navigation structure, and responsive behavior.

---

#### LS-001: Main Navigation Bar Renders Correctly
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: App loaded, user authenticated
- **Steps**:
  1. Load the application
  2. Observe the top navigation bar
- **Expected Result**: Navigation bar contains logo ("W" + "GTO Wizard" text), and links/tabs for Play, Study, Practice, Analyze. Right side contains Upload button, settings/gear icon, and user avatar.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### LS-002: Navigation Bar Is Sticky on Scroll
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: App loaded on a page with scrollable content
- **Steps**:
  1. Load any page with content taller than the viewport
  2. Scroll down past two viewport heights
- **Expected Result**: Navigation bar remains fixed at the top of the viewport, always visible regardless of scroll position. Content scrolls underneath the nav bar.
- **Screenshot Reference**: None

---

#### LS-003: Dashboard Shows Mode Cards in Grid Layout
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: App loaded, user on Dashboard
- **Steps**:
  1. Navigate to the Dashboard / Home screen
  2. Observe the mode selection cards
- **Expected Result**: Dashboard displays 9 feature tiles arranged in a 3x3 grid: Study, Trainer, Uploads (row 1); Custom solutions, Range builder, Hands (row 2); Coaching, Drills, Help center (row 3). Each tile has a colored icon, title, and short description text.
- **Screenshot Reference**: `screenshots/reviews/h2n-dashboard.webp`

---

#### LS-004: Dashboard Stats Panels Render Below Mode Cards
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: App loaded, user on Dashboard
- **Steps**:
  1. Navigate to the Dashboard
  2. Scroll to the bottom section
- **Expected Result**: Two side-by-side stats panels appear: "Trainer stats" (showing HANDS, MOVES, MISTAKES, GTOW SCORE with colored horizontal bars for Best/Correct/Inaccuracy/Wrong/Blunder) and "Analyzer stats" (showing TOTAL, CORRECT, WRONG, AVG. EV LOSS).
- **Screenshot Reference**: `screenshots/reviews/h2n-dashboard.webp`

---

#### LS-005: Study Mode Loads with Strategy Tab Active by Default
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User navigates to Study Mode
- **Steps**:
  1. Click "Study" in the navigation bar
  2. Select a game format and spot
  3. Observe the tab bar
- **Expected Result**: The tab bar shows "Strategy + EV", "Ranges", "Breakdown", "Reports: Flops" tabs. The "Strategy + EV" tab is active/highlighted by default.
- **Screenshot Reference**: `screenshots/website/ai-solver-default.avif`

---

#### LS-006: Tab Switching Between Strategy, Ranges, Breakdown, Reports
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode with a spot loaded
- **Steps**:
  1. Click each tab in order: Strategy + EV, Ranges, Breakdown, Reports: Flops
  2. Observe that the view changes accordingly
- **Expected Result**: Each tab click loads its corresponding view. Active tab is visually highlighted (teal/cyan accent). Only one tab is active at a time. Tab switching is instantaneous with no page reload.
- **Screenshot Reference**: None

---

#### LS-007: Blockers Tab Renders When Available
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode on a postflop spot
- **Steps**:
  1. Navigate to a postflop decision node
  2. Look for a Blockers tab or panel
- **Expected Result**: A Blockers analysis section is accessible, showing a sortable table of cards with their impact on opponent Bet%, Check%, Fold%, and Call% frequencies.
- **Screenshot Reference**: None

---

#### LS-008: Infobox Panel Renders on Left Side
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode Strategy tab
- **Steps**:
  1. Load a spot in Study Mode
  2. Observe the left panel
- **Expected Result**: Infobox panel appears on the left side containing: Positions (both players, active player highlighted), Pot size in BB, Board cards, Stack depth, Pot odds. Below the infobox is the Actions Table.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### LS-009: Strategy Matrix Renders as 13x13 Grid on Right Side
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode Strategy tab
- **Steps**:
  1. Load a spot in Study Mode
  2. Observe the right panel
- **Expected Result**: A 13x13 hand grid renders on the right side of the layout. All 169 hand combinations are displayed as labeled, color-coded cells. The grid occupies the primary content area alongside the infobox.
- **Screenshot Reference**: `screenshots/website/ai-solver-default.avif`, `screenshots/reviews/h2n-main.webp`

---

#### LS-010: Actions Table Renders Below Infobox
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode Strategy tab
- **Steps**:
  1. Load a spot in Study Mode
  2. Observe the area below the infobox
- **Expected Result**: Actions Table displays showing each available action (e.g., Check, Bet 33%, Bet 75%) with horizontal frequency bars and percentage labels. Pressing S toggles between chart and table view.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### LS-011: Hand Detail Popup Appears on Matrix Cell Hover
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode Strategy tab with a spot loaded
- **Steps**:
  1. Hover the mouse over any cell in the 13x13 strategy matrix (e.g., AKs)
  2. Observe popup behavior
- **Expected Result**: A popup appears within 200ms showing the hand label (e.g., "AKs"), suit-specific breakdown for each combination (e.g., AhKh, AsKs, AdKd, AcKc), GTO action percentages per suit combo, and EV/Equity values.
- **Screenshot Reference**: None

---

#### LS-012: Practice Mode Trainer Table View Layout
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User starts a Practice Mode session
- **Steps**:
  1. Navigate to Practice Mode
  2. Configure and start a drill
  3. Observe the table layout
- **Expected Result**: Center area shows an oval poker table with position labels around the rim (UTG, HJ, CO, BTN, SB, BB). Hero's cards are face-up at the bottom. Opponent cards are face-down at the top. Community cards display in the center with pot size above. Action buttons appear below the table. A scrollable action history bar appears at the top.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`, `screenshots/reviews/h2n-trainer.webp`

---

#### LS-013: Practice Mode Right Sidebar Info Panel
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User in Practice Mode with info panel visible
- **Steps**:
  1. Start a Practice Mode session
  2. Toggle the Info panel open (lightbulb icon)
- **Expected Result**: Right sidebar appears containing Strategy and Range tabs, a 13x13 hand matrix, action list with percentages, Hand info section with Strength and Equity bars, Hands distribution, and History section.
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### LS-014: Analyze Mode Results Table Layout
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User has uploaded and analyzed hand histories
- **Steps**:
  1. Navigate to Analyze Mode
  2. View the analyzed hands table
- **Expected Result**: Table displays with columns for: checkbox, Pot (bb), Tag, Status, Format, Site, Position, Hand (card images), Board (card images), Pot Type, Preflop actions, Flop actions, and more. Header shows "Hands" title with "All Reports", "New Report", "Save Report" buttons. Filter tabs appear above the table.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hands-table.png`

---

#### LS-015: Analyze Mode Stats Cards Row
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Analyze Mode with analyzed hands and a report selected
- **Steps**:
  1. Navigate to the Stats view in Analyze Mode
  2. Observe the top row of stat cards
- **Expected Result**: Five stat cards display in a horizontal row: GTO Score (with sparkline trend graph), Hands (with move count), EV Loss (bb per 100 hands), Avg EV Loss (per 100 mistakes), and Frequency Diff (percentage). Each card shows a large "hero number" value with a small label below.
- **Screenshot Reference**: `screenshots/blog/analyzer2-srp-report.png`

---

#### LS-016: Range Builder Layout
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User navigates to Range Builder
- **Steps**:
  1. Open Range Builder
  2. Observe the two-panel layout
- **Expected Result**: Left panel contains a 13x13 hand selection matrix with Select All, Clear, Load Range, and Save Range buttons. Right panel shows Strategy Output with selected range combo count, action assignments (Raise/Call/Fold percentages), GTO Score, and deviation from GTO.
- **Screenshot Reference**: `screenshots/blog/nodelocking-interface.png` (shared matrix component)

---

#### LS-017: GTO Wizard AI Configuration Screen Layout
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User has Elite subscription, navigates to GTO Wizard AI
- **Steps**:
  1. Open GTO Wizard AI custom solver
  2. Observe the configuration form
- **Expected Result**: Form contains: Players dropdown (up to 9), Player 1 and Player 2 configuration sections (Position, Range editor, Stack input), Pot Size input, Board card selector (specific or Random), Betting Tree section with IP/OOP bet sizes and raise sizes with "Add Size" buttons, Accuracy Target dropdown. Bottom shows Solve/Load Preset/Save Configuration buttons and Power Credits remaining counter.
- **Screenshot Reference**: `screenshots/website/ai-solver-default.avif`

---

#### LS-018: Nodelocking 3-Tab Interface
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User has Elite subscription, navigates to Nodelocking
- **Steps**:
  1. Open Nodelocking interface
  2. Observe the tab structure
- **Expected Result**: Three tabs are clearly visible: "Set strategy", "Set frequency", and "Lock / Unlock". A horizontal card-based game tree shows the action sequence with card images at flop nodes. Inline hover icons (pencil, lock, compare) appear on game tree nodes.
- **Screenshot Reference**: `screenshots/blog/nodelocking-interface.png`, `screenshots/blog/nodelocking-set-frequency.png`

---

#### LS-019: PokerArena Game View Layout
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User enters PokerArena mode
- **Steps**:
  1. Navigate to PokerArena
  2. Start or join a match
- **Expected Result**: Layout shows opponent name and rating at top, their face-down cards, community cards in center with pot size, hero's name, rating, and face-up cards at bottom, action buttons below. Mode selector shows Competitive/Casual/Play With Friends options. Leaderboard section shows Season rank table with Rank, Player, Rating, and W/L columns.
- **Screenshot Reference**: `screenshots/website/battle-heads-up-poker-default.avif`

---

#### LS-020: Table Wizard Multi-Table Grid Layout
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: Table Wizard installed and active
- **Steps**:
  1. Launch Table Wizard
  2. Open multiple poker tables
- **Expected Result**: Tables arranged in a grid layout (up to 2x2 for 4 tables). Each table shows its own overlay with position labels, stack/SPR/PO/EFF stats, and per-position colored action indicators. Status bar at bottom shows active table count, hands tracked, and auto-upload toggle.
- **Screenshot Reference**: `screenshots/blog/table-wizard-overlay.png`

---

#### LS-021: Table Wizard Left Sidebar Navigation
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: Table Wizard application open
- **Steps**:
  1. Open Table Wizard
  2. Observe the left sidebar
- **Expected Result**: Left sidebar contains five navigation items: Dashboard, Table Layout, Overlays, Hot Keys, Automated Controls. The currently active section is highlighted. An "Active" toggle (green when enabled) appears in the header alongside the "W Table Wizard BETA" branding.
- **Screenshot Reference**: `screenshots/blog/table-wizard-overlay.png`

---

#### LS-022: Settings Screen Sections
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: User navigates to Settings
- **Steps**:
  1. Click the settings/gear icon in the navigation bar
  2. Observe the settings page structure
- **Expected Result**: Settings page contains clearly separated sections: Appearance (Theme, Card Theme, Card Backs, Background, Text Color, Action Colors), Display (Layout options, Size, EV/Bet display toggles), Hotkeys (customizable keyboard shortcuts), Language selector (17 languages), and Account (Subscription tier, Power Credits, Upload Limit, Manage Subscription button).
- **Screenshot Reference**: `screenshots/reviews/h2n-settings.webp`

---

#### LS-023: Modal Dialogs Overlay Correctly
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: Any modal-triggering action available
- **Steps**:
  1. Trigger a modal dialog (e.g., Upload dialog, Jump-to-spot popup via J key)
  2. Observe modal rendering
- **Expected Result**: Modal appears centered on screen with a semi-transparent dark overlay behind it. Background content is not interactive while modal is open. Modal has clear close/dismiss controls (X button or Escape key). Modal content is fully visible without scrolling on 1920x1080 viewport.
- **Screenshot Reference**: None

---

#### LS-024: Sidebar Panels Collapse and Expand
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User in a mode with sidebar panels (Study Mode or Practice Mode)
- **Steps**:
  1. Open Study Mode or Practice Mode
  2. Click the collapse/expand control on the info sidebar panel
  3. Toggle between collapsed and expanded states
- **Expected Result**: Sidebar collapses smoothly (animation < 300ms). When collapsed, the main content area expands to fill the available space. When expanded, the sidebar re-appears at its original width (approximately 300-350px). The Q key maximizes the solution browser, hiding sidebars.
- **Screenshot Reference**: None

---

#### LS-025: Responsive Layout at Desktop Breakpoint (1920x1080)
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: App loaded in browser at 1920x1080 viewport
- **Steps**:
  1. Set browser viewport to 1920x1080
  2. Navigate through Dashboard, Study, Practice, Analyze modes
- **Expected Result**: All layouts render correctly with no horizontal scrolling, no content overflow, no overlapping elements. Two-column layouts (infobox + matrix) display side by side. Stats card rows are fully visible without wrapping.
- **Screenshot Reference**: All review screenshots at approximately this resolution

---

#### LS-026: Responsive Layout at Tablet Breakpoint (1024x768)
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: App loaded in browser at 1024x768 viewport
- **Steps**:
  1. Set browser viewport to 1024x768
  2. Navigate through all major modes
- **Expected Result**: Layout adapts gracefully. Two-column layouts may stack vertically. Navigation remains accessible. 13x13 matrix cells may be smaller but remain legible and clickable. No horizontal scrolling on primary content.
- **Screenshot Reference**: None

---

#### LS-027: Responsive Layout at Mobile Breakpoint (375x812)
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: App loaded on mobile viewport (375x812)
- **Steps**:
  1. Set browser viewport to 375x812
  2. Navigate through all major modes
- **Expected Result**: Navigation collapses to a hamburger menu or bottom navigation. Content stacks in single-column layout. 13x13 matrix is scrollable or zoomable. Action buttons are touch-friendly (minimum 44px tap target). All text remains readable without zooming.
- **Screenshot Reference**: None

---

#### LS-028: Spot Selector Renders Game Tree Path
- **Priority**: P1 (Critical)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode
- **Steps**:
  1. Open Study Mode
  2. Select a format (e.g., Cash 6max 100bb)
  3. Choose positions and actions through the spot selector
- **Expected Result**: Spot selector at the top displays the full game tree path: Format dropdown, Stack dropdown, Rake dropdown, Position flow (UTG -> HJ -> CO -> BTN -> SB -> BB), and Action Sequence showing each action node (e.g., "Open 2.5x -> 3-Bet 8.5x -> Call"). Back/Forward navigation arrows and Jump (J) button are visible.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### LS-029: GTO Reports Three-Column Layout
- **Priority**: P2 (High)
- **Category**: Layout & Structure
- **Precondition**: User navigates to GTO Reports
- **Steps**:
  1. Open GTO Reports from Analyze Mode
  2. Select a report configuration
- **Expected Result**: Layout displays in three columns: Left panel shows preflop stats with VALUE vs GTO deviation bars; Center panel shows position breakdown table and Mistakes list; Right panel shows contextual help/definitions. Game Type selector at top shows Cash/MTT/Spins/HuSng toggles with Players, Stacks, Rake, and Opening Size configuration.
- **Screenshot Reference**: `screenshots/blog/redesigned-preflop-mistakes.png`, `screenshots/blog/gto-reports-preflop-stats.png`

---

#### LS-030: Study Mode Layout Options (Horizontal, Split, Reversed)
- **Priority**: P3 (Medium)
- **Category**: Layout & Structure
- **Precondition**: User in Study Mode
- **Steps**:
  1. Open Study Mode
  2. Navigate to Settings or layout selector
  3. Switch between Horizontal, Horizontal Reversed, Split, and Split Reversed layouts
  4. Also test Large, Medium, and Compact sizing
- **Expected Result**: Each layout option rearranges the infobox and strategy matrix correctly. Horizontal places panels side-by-side. Split stacks them vertically. "Reversed" variants swap the panel positions. Size options adjust component dimensions proportionally. All content remains fully visible in each configuration.
- **Screenshot Reference**: None

---

## 2. Color System Tests (CS-001 through CS-020)

Tests for correct color application across the action system, heatmaps, and deviation indicators.

---

#### CS-001: Bet/Raise Actions Render in Red
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Study Mode with a spot containing bet/raise actions
- **Steps**:
  1. Load a spot where the GTO strategy includes betting or raising
  2. Observe the strategy matrix cell colors for betting hands
- **Expected Result**: All cells where the primary action is Bet or Raise display in red shades within the #E04040 to #FF6666 hex range. The red is clearly distinguishable from green (check/call) and blue (fold).
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### CS-002: Check/Call Actions Render in Green
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Study Mode with a spot containing check/call actions
- **Steps**:
  1. Load a spot where the GTO strategy includes checking or calling
  2. Observe the strategy matrix cell colors for checking/calling hands
- **Expected Result**: All cells where the primary action is Check or Call display in green shades within the #40B040 to #66CC66 hex range.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### CS-003: Fold Actions Render in Blue
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Study Mode with a spot containing fold actions
- **Steps**:
  1. Load a spot where the GTO strategy includes folding
  2. Observe the strategy matrix cell colors for folding hands
- **Expected Result**: All cells where the primary action is Fold display in blue shades within the #4060D0 to #6688EE hex range.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### CS-004: All-In Actions Render in Deep Red/Gold
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in Study Mode with a spot containing all-in actions
- **Steps**:
  1. Load a spot where the GTO strategy includes going all-in (e.g., short stack preflop)
  2. Observe the cell colors for all-in hands
- **Expected Result**: All-in actions display in deep red to gold shades within the #CC2020 to #FFD700 hex range, visually distinct from standard bet/raise red.
- **Screenshot Reference**: None

---

#### CS-005: Mixed Strategy Cells Show Proportional Color Splits
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Study Mode with a spot containing mixed strategies
- **Steps**:
  1. Load a spot where hands have mixed strategies (e.g., 60% bet, 40% check)
  2. Observe cells with multiple actions
- **Expected Result**: Cells display proportional color segments. A hand with 60% bet / 40% check shows approximately 60% red and 40% green within the cell. The proportions visually match the action frequencies within a tolerance of +/- 5%.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### CS-006: EV Heatmap Uses Green-White-Red Gradient
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in Study Mode, metric dropdown available
- **Steps**:
  1. Load a spot in Study Mode
  2. Change the metric dropdown to "EV"
  3. Observe the strategy matrix color changes
- **Expected Result**: Matrix recolors using a continuous gradient: bright green for high positive EV, light green for moderate positive, white/neutral for near-zero EV, light red for moderate negative, bright red for high negative EV. The gradient center (white) corresponds to approximately 0 EV.
- **Screenshot Reference**: None

---

#### CS-007: Best Action Deviation Indicator (Double Green Checkmark)
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Practice Mode or Analyze Mode after making a decision
- **Steps**:
  1. Complete a hand in Practice Mode selecting the highest-frequency GTO action
  2. Observe the scoring indicator
- **Expected Result**: The "Best" action displays a double green checkmark symbol. In Analyzer 2.0, this corresponds to the "PERFECT" classification. The green color is consistent with the check/call green palette.
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### CS-008: Correct Action Shows Single Green Checkmark
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Practice Mode or Analyze Mode
- **Steps**:
  1. Complete a hand selecting an action present in GTO strategy but not the highest-frequency one
  2. Observe the scoring indicator
- **Expected Result**: The "Correct" action displays a single green checkmark. In Analyzer 2.0, this corresponds to the "GOOD" classification. Color is green, distinguishable from the double checkmark of "Best/PERFECT".
- **Screenshot Reference**: None

---

#### CS-009: Deviation Indicators for Inaccuracy, Wrong, and Blunder
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Practice Mode or Analyze Mode
- **Steps**:
  1. Review results showing each deviation level
- **Expected Result**: Inaccuracy shows a yellow/amber indicator (circle-i icon). Wrong shows an orange/red indicator (circle-i icon). Blunder shows a red indicator (triangle-! icon). Colors are clearly graduated: yellow (minor) -> orange (significant) -> red (critical).
- **Screenshot Reference**: `screenshots/blog/analyzer2-srp-report.png`

---

#### CS-010: Analyzer 2.0 Uses PERFECT/GOOD/INACCURATE/WRONG/BLUNDER Labels
- **Priority**: P1 (Critical)
- **Category**: Color System
- **Precondition**: User in Analyzer 2.0 Stats view
- **Steps**:
  1. Navigate to Analyzer 2.0 stats or breakdown table
  2. Observe column headers and score breakdown labels
- **Expected Result**: Table headers use uppercase labels: PERFECT, GOOD, INACCURATE (note: real UI shows "INNACURATE" as a known typo), WRONG, BLUNDER. Score Breakdown panel uses colored bars: Perfect=green, Good=teal, Inaccurate=yellow/amber, Wrong/Mistake=red, Blunder=red with distinct icon.
- **Screenshot Reference**: `screenshots/blog/analyzer2-srp-report.png`

---

#### CS-011: Bet Size Colors Use Graduated Red Intensity
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in Practice Mode with multiple bet size options
- **Steps**:
  1. Start a Practice Mode hand with multiple bet sizes available
  2. Observe the action button colors
- **Expected Result**: Bet/raise buttons use graduated red intensity: small bets (e.g., 33%) are lighter red, medium bets (e.g., 75%) are medium red, large bets (e.g., 125%) are darker red, and ALL-IN is the darkest red. CHECK remains green. FOLD remains blue.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`, `screenshots/reviews/h2n-trainer.webp`

---

#### CS-012: Theme Switching Maintains Color Consistency
- **Priority**: P3 (Medium)
- **Category**: Color System
- **Precondition**: User in Settings with theme options
- **Steps**:
  1. Open Settings > Appearance > Theme
  2. Switch between at least two different themes
  3. Navigate to Study Mode and observe the strategy matrix
- **Expected Result**: Action color semantics are preserved across themes (red=bet, green=check, blue=fold). Background, text, and accent colors change according to the selected theme. No action colors become invisible or indistinguishable from the background.
- **Screenshot Reference**: None

---

#### CS-013: Dark Theme Background Color
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: App loaded in dark theme (default)
- **Steps**:
  1. Load the application
  2. Observe the background color
- **Expected Result**: Background color is a dark charcoal/navy in the #1a1a2e range. Text appears in white or light gray for readability. The overall aesthetic matches the dark theme observed across all reference screenshots.
- **Screenshot Reference**: All screenshots

---

#### CS-014: WCAG AA Contrast Compliance in Dark Mode
- **Priority**: P3 (Medium)
- **Category**: Color System
- **Precondition**: App in dark mode
- **Steps**:
  1. Load the application in dark mode
  2. Test text-to-background contrast ratios for body text, labels, and headings
- **Expected Result**: All text elements meet WCAG AA contrast ratio requirements: 4.5:1 for normal text, 3:1 for large text (18px+ or 14px+ bold). Interactive elements have at least 3:1 contrast against their backgrounds.
- **Screenshot Reference**: None

---

#### CS-015: GTO Reports VALUE vs GTO Deviation Bars
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in GTO Reports with analyzed data
- **Steps**:
  1. Open GTO Reports
  2. View preflop statistics (VPIP, PFR, RFI, etc.)
  3. Observe the deviation bars next to each stat
- **Expected Result**: Each stat displays a horizontal deviation bar with a center line representing the GTO reference value. When VALUE exceeds GTO, the deviation bar extends to the right in red. When VALUE is below GTO, the bar extends to the left in blue. When VALUE matches GTO, a green checkmark indicator appears. The numerical VALUE and GTO reference are both displayed.
- **Screenshot Reference**: `screenshots/blog/gto-reports-preflop-stats.png`

---

#### CS-016: Position Matrix Color Coding in GTO Reports
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in GTO Reports position matrix view
- **Steps**:
  1. Open GTO Reports
  2. Navigate to a position-vs-position matrix (e.g., 3BET vs RFI)
- **Expected Result**: 6x6 grid (UTG/HJ/CO/BTN/SB/BB vs UTG/HJ/CO/BTN/SB/BB) displays two numbers per cell (VALUE above, GTO below). Cells are color-coded: green when VALUE is close to GTO, red when significantly deviating. Warning icons (amber circle with !) appear next to positions with significant deviations.
- **Screenshot Reference**: `screenshots/blog/gto-reports-position-matrix.png`

---

#### CS-017: Blocked/Dead Hands Render as Dark/Disabled
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in Study Mode on a postflop spot where board cards block some hands
- **Steps**:
  1. Load a postflop spot (e.g., board Ks 7h 2d)
  2. Observe hands containing board cards in the matrix
- **Expected Result**: Hands that are blocked by board cards (e.g., hands containing Ks, 7h, or 2d) render in black or dark gray, visually indicating they are impossible/not in range. These cells are clearly distinguished from active range cells.
- **Screenshot Reference**: None

---

#### CS-018: Filtered Hands Render as Dimmed/Faded
- **Priority**: P2 (High)
- **Category**: Color System
- **Precondition**: User in Study Mode with filters applied
- **Steps**:
  1. Load a spot in Study Mode
  2. Apply a hand category filter (e.g., "Pairs only")
  3. Observe non-matching hands
- **Expected Result**: Hands not matching the active filter appear dimmed/faded compared to matching hands. Matching hands retain full color intensity. The contrast between filtered and non-filtered hands is clearly visible.
- **Screenshot Reference**: None

---

#### CS-019: Teal/Cyan Accent for Primary Actions and Active States
- **Priority**: P3 (Medium)
- **Category**: Color System
- **Precondition**: App loaded in dark theme
- **Steps**:
  1. Navigate through the app observing active nav items, active tabs, primary buttons
- **Expected Result**: Active/selected navigation items, active tab indicators, and primary interactive elements use a teal/cyan accent color (approximately #4CAF50 range). This accent is consistent across all screens and modes.
- **Screenshot Reference**: All screenshots

---

#### CS-020: Aggregated Reports Stacked Bar Chart Colors
- **Priority**: P3 (Medium)
- **Category**: Color System
- **Precondition**: User in Study Mode Reports tab (Aggregated Reports)
- **Steps**:
  1. Navigate to Reports: Flops in Study Mode
  2. View the chart mode
- **Expected Result**: Stacked bar chart uses red segments for betting actions and green segments for checking. Each bar represents a flop category. The right panel shows action summary buttons with percentages using matching colors.
- **Screenshot Reference**: `screenshots/blog/aggregated-reports-flop-cbet.png`

---

## 3. Strategy Matrix Tests (SM-001 through SM-025)

Tests for the 13x13 hand grid behavior, metrics, filters, and interactions.

---

#### SM-001: 13x13 Grid Renders All 169 Hand Combinations
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with a spot loaded
- **Steps**:
  1. Load any spot in Study Mode
  2. Count the cells in the strategy matrix
- **Expected Result**: Grid displays exactly 13 rows and 13 columns for a total of 169 cells. Every cell is labeled with its hand combination (AA, AKs, AKo, KK, etc.). No cells are missing or duplicated.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### SM-002: Suited Hands Above Diagonal
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Examine cells above the main diagonal (top-right triangle)
  2. Verify hand labels
- **Expected Result**: All cells above the diagonal display suited hands (denoted by "s" suffix): AKs, AQs, AJs, ... KQs, KJs, ... down to 32s. Each suited cell represents 4 suit combinations.
- **Screenshot Reference**: None

---

#### SM-003: Offsuit Hands Below Diagonal
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Examine cells below the main diagonal (bottom-left triangle)
  2. Verify hand labels
- **Expected Result**: All cells below the diagonal display offsuit hands (denoted by "o" suffix): AKo, AQo, AJo, ... KQo, KJo, ... down to 32o. Each offsuit cell represents 12 suit combinations.
- **Screenshot Reference**: None

---

#### SM-004: Pocket Pairs on Diagonal
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Examine cells on the main diagonal (top-left to bottom-right)
  2. Verify hand labels
- **Expected Result**: All 13 diagonal cells display pocket pairs: AA, KK, QQ, JJ, TT, 99, 88, 77, 66, 55, 44, 33, 22 from top-left to bottom-right. Each pair cell represents 6 suit combinations.
- **Screenshot Reference**: None

---

#### SM-005: Hover on Cell Reveals Suit-Specific Breakdown Popup
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with a spot loaded
- **Steps**:
  1. Hover over a suited hand cell (e.g., AKs)
  2. Observe the popup that appears
- **Expected Result**: Popup displays the hand name and all 4 suit combinations (e.g., AhKh, AsKs, AdKd, AcKc for AKs). Each combination shows its individual GTO action breakdown with frequency percentages and EV values. Popup appears within 200ms of hover.
- **Screenshot Reference**: None

---

#### SM-006: Click on Cell Filters to That Hand's Action
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with a spot loaded
- **Steps**:
  1. Click on a cell in the strategy matrix
  2. Observe filtering behavior
- **Expected Result**: Clicking a cell filters the view to show details for only that hand combination. The Actions Table updates to reflect the frequencies for the selected hand only. Other cells may dim to indicate the active filter. Clicking again or pressing P clears the filter.
- **Screenshot Reference**: None

---

#### SM-007: Metric Dropdown Changes Cell Coloring (EV Mode)
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with metric dropdown visible
- **Steps**:
  1. Open the metric dropdown at the top of the strategy matrix
  2. Select "EV"
- **Expected Result**: Matrix recolors from action-based colors (red/green/blue) to an EV heatmap using the green-white-red gradient. Hands with high EV appear green, near-zero EV appear white, and negative EV appear red. Cell labels may display EV values.
- **Screenshot Reference**: None

---

#### SM-008: Metric Dropdown Changes Cell Coloring (EQ Mode)
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with metric dropdown visible
- **Steps**:
  1. Open the metric dropdown
  2. Select "Equity" or "EQ"
- **Expected Result**: Matrix recolors to show raw equity percentages against the opponent's range. Cells display equity values and use a heatmap gradient where higher equity hands are more intensely colored.
- **Screenshot Reference**: None

---

#### SM-009: Metric Dropdown Changes Cell Coloring (EQR Mode)
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with metric dropdown visible
- **Steps**:
  1. Open the metric dropdown
  2. Select "EQR" (Equity Realization)
- **Expected Result**: Matrix recolors based on equity realization ratio. Hands that realize more equity than expected (EQR > 100%) are highlighted differently from hands that underperform their equity (EQR < 100%).
- **Screenshot Reference**: None

---

#### SM-010: Metric Dropdown Changes Cell Coloring (Range Weight Mode)
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with metric dropdown visible
- **Steps**:
  1. Open the metric dropdown
  2. Select "Range weights"
- **Expected Result**: Matrix recolors based on the frequency of each hand in the player's range. Bright/intense colors indicate hands that are frequent in the range. Dim/faded colors indicate hands that are rare or absent from the range.
- **Screenshot Reference**: None

---

#### SM-011: Blocked Hands Rendering on Postflop Boards
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode on a postflop spot
- **Steps**:
  1. Load a flop spot (e.g., board Ks 7h 2d)
  2. Observe hands containing board cards
- **Expected Result**: Cells for hands that are impossible given the board (e.g., any hand containing Ks on a Ks 7h 2d board) render as black/dark gray. These blocked cells are non-interactive for filtering purposes.
- **Screenshot Reference**: None

---

#### SM-012: Pairs Filter Toggle
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Pairs" filter toggle in the hand category filter bar
- **Expected Result**: Matrix highlights only pocket pair cells (diagonal). All non-pair cells are dimmed/faded. Actions Table frequencies update to reflect only pairs. Clicking "Pairs" again or pressing P clears the filter.
- **Screenshot Reference**: None

---

#### SM-013: Suited Hands Filter Toggle
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Suited" filter toggle
- **Expected Result**: Matrix highlights only suited hand cells (above diagonal). All offsuit and pair cells are dimmed. Actions Table updates to reflect suited hands only.
- **Screenshot Reference**: None

---

#### SM-014: Offsuit Hands Filter Toggle
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Offsuit" filter toggle
- **Expected Result**: Matrix highlights only offsuit hand cells (below diagonal). All suited and pair cells are dimmed. Actions Table updates to reflect offsuit hands only.
- **Screenshot Reference**: None

---

#### SM-015: Broadway Filter
- **Priority**: P3 (Medium)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Broadways" filter toggle
- **Expected Result**: Matrix highlights only broadway hand cells (hands containing T, J, Q, K, A). Non-broadway cells are dimmed. Actions Table updates accordingly.
- **Screenshot Reference**: None

---

#### SM-016: Connectors Filter
- **Priority**: P3 (Medium)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Connectors" filter toggle
- **Expected Result**: Matrix highlights connected hand cells (adjacent rank hands like JTs, T9s, 98o, etc.). Non-connector cells are dimmed.
- **Screenshot Reference**: None

---

#### SM-017: Suited Gapper Filter
- **Priority**: P3 (Medium)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with filter bar visible
- **Steps**:
  1. Click the "Suited Gappers" filter toggle
- **Expected Result**: Matrix highlights suited gapper hands (one-gap suited like J9s, T8s, etc.). Non-gapper cells are dimmed.
- **Screenshot Reference**: None

---

#### SM-018: Suit Filter Dropdown
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Locate the suit filter control (typically above or in the corner of the matrix)
  2. Select or deselect specific suits
- **Expected Result**: A dropdown or grid allows toggling individual suits (spades, hearts, diamonds, clubs). Top row filters the highest card's suit, bottom row filters the lowest card's suit. Matrix updates to show only combos matching the selected suit filter.
- **Screenshot Reference**: None

---

#### SM-019: Clear Filters (P Key)
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with one or more filters active
- **Steps**:
  1. Apply any filter (hand category, suit, or action)
  2. Press the P key
- **Expected Result**: All active filters are cleared immediately. The matrix returns to its unfiltered state showing all 169 hands at full intensity. The Actions Table resets to show range-wide frequencies. Individual filter "x" buttons also work to remove individual filters.
- **Screenshot Reference**: None

---

#### SM-020: Compare EV Mode Shows Side-by-Side Action Comparison
- **Priority**: P2 (High)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with metric dropdown visible
- **Steps**:
  1. Open the metric dropdown
  2. Select "Compare EV"
  3. Choose two actions to compare
- **Expected Result**: Matrix recolors using a green-white-red gradient showing EV difference between the two selected actions. Green cells indicate the first action has higher EV, red cells indicate the second action has higher EV, and white cells indicate marginal/indifferent differences.
- **Screenshot Reference**: None

---

#### SM-021: Mixed Strategy Cell Proportional Rendering Accuracy
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with mixed strategies present
- **Steps**:
  1. Load a spot with mixed strategies
  2. Identify a cell with a known mixed strategy (e.g., 70% bet, 30% check)
  3. Visually assess the color split proportions
- **Expected Result**: The color segments within each cell are proportional to the action frequencies. A 70%/30% split shows approximately 70% of the cell in one color and 30% in another. Multi-way splits (3+ actions) display all colors proportionally.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### SM-022: Cell Text Shows Correct Hand Label
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Examine the text in several cells throughout the grid
- **Expected Result**: Every cell displays its correct hand label text (e.g., "AA", "AKs", "AKo", "KK", "72o"). Text is readable against the cell's background color. Labels use a consistent font size and weight. No labels are truncated or missing.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`

---

#### SM-023: Click Action in Actions Table Filters Matrix
- **Priority**: P1 (Critical)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode Strategy tab
- **Steps**:
  1. Load a spot with multiple actions (e.g., Check, Bet 33%, Bet 75%)
  2. Click "Bet 33%" in the Actions Table
- **Expected Result**: Strategy matrix updates to highlight only hands that take the "Bet 33%" action. Hands that never bet 33% are dimmed. The selected action is visually indicated in the Actions Table (e.g., highlighted border or background). Multiple actions can be selected simultaneously.
- **Screenshot Reference**: None

---

#### SM-024: Right-Click Context Menu on Matrix Cell
- **Priority**: P3 (Medium)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with strategy matrix visible
- **Steps**:
  1. Right-click on any cell in the strategy matrix
- **Expected Result**: A context menu appears with options for range weight adjustment and potentially other actions. The menu positions correctly relative to the clicked cell without going off-screen.
- **Screenshot Reference**: None

---

#### SM-025: View Mode Options (Range Height, Full Height, Normalized, Horizontal)
- **Priority**: P3 (Medium)
- **Category**: Strategy Matrix
- **Precondition**: User in Study Mode with view dropdown visible
- **Steps**:
  1. Open the view mode dropdown at the matrix corner
  2. Switch between Range Height, Full Height, Normalized, and Horizontal modes
- **Expected Result**: Range Height: cell height proportional to hand's range frequency. Full Height: all cells same height. Normalized: adjusted proportional display. Horizontal: each cell expands to show individual suit combinations side by side. Each mode renders correctly without overlapping or missing elements.
- **Screenshot Reference**: None

---

## 4. Practice Mode Tests (PM-001 through PM-020)

Tests for the GTO Trainer gameplay, scoring, and configuration.

---

#### PM-001: Hole Cards Render Correctly (Face Up for Hero)
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User starts a Practice Mode hand
- **Steps**:
  1. Start a new hand in Practice Mode
  2. Observe the hero's hole cards at the bottom of the table
- **Expected Result**: Hero's two hole cards display face-up with correct rank and suit symbols. Card images show colored suit icons (red for hearts/diamonds, blue/black for clubs/spades). Cards are clearly legible at normal zoom level.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`

---

#### PM-002: Opponent Cards Render Face Down
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User in a Practice Mode hand
- **Steps**:
  1. Observe the opponent's card display at the top of the table
- **Expected Result**: Opponent's cards display as card backs (face down). The card back design matches the selected card back theme from Settings. Cards are the same physical size as hero's cards.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`

---

#### PM-003: Community Cards Display (Flop, Turn, River)
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User in a Practice Mode hand at a postflop street
- **Steps**:
  1. Progress to the flop
  2. Observe 3 community cards
  3. Progress to the turn and observe the 4th card
  4. Progress to the river and observe the 5th card
- **Expected Result**: Flop displays 3 cards face-up in the center of the table. Turn adds a 4th card to the right. River adds a 5th card. Empty card positions show placeholder slots. Pot size displays above the community cards with a pot icon.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`

---

#### PM-004: Action Buttons Show Correct Sizing Options
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User facing a decision in Practice Mode
- **Steps**:
  1. Face a betting decision in Practice Mode
  2. Observe the action buttons below the table
- **Expected Result**: Action buttons display all available actions with their exact sizes in BB (e.g., "CHECK", "BET 4.3", "BET 9.75", "BET 16.25", "ALLIN 93.5"). Each button shows the action name and the amount. Buttons are color-coded (green for check, graduated red for bets, darkest red for all-in).
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### PM-005: Simple Difficulty Shows Only Action Types
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: Practice Mode configured with Simple difficulty
- **Steps**:
  1. Set difficulty to "Simple" in drill configuration
  2. Face a betting decision
- **Expected Result**: Only broad action categories are shown: Bet/Raise, Check/Call, Fold. No specific bet sizes appear. The trainer auto-selects the highest-frequency size within the chosen action category.
- **Screenshot Reference**: None

---

#### PM-006: Grouped Difficulty Shows Size Categories
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: Practice Mode configured with Grouped difficulty
- **Steps**:
  1. Set difficulty to "Grouped"
  2. Face a betting decision with multiple sizes
- **Expected Result**: Action buttons show grouped categories: Small bet, Medium bet, Large bet, Overbet (when applicable), plus Check/Call and Fold. Individual exact percentages are not shown.
- **Screenshot Reference**: None

---

#### PM-007: Standard Difficulty Shows Exact Percentages
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: Practice Mode configured with Standard difficulty
- **Steps**:
  1. Set difficulty to "Standard"
  2. Face a betting decision
- **Expected Result**: All individual bet sizes from the solver are shown with exact percentages (e.g., "BET 33%", "BET 50%", "BET 75%", "BET 125%", "BET 175%"). Each size is a separate selectable button.
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### PM-008: RNG Dice Appears and Rolls 1-100 When Enabled
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: RNG enabled in Practice Mode settings (High or Low mode)
- **Steps**:
  1. Enable RNG (High or Low mode)
  2. Start a new hand
  3. Observe the RNG display
- **Expected Result**: A dice/number display appears showing a randomly generated number between 1 and 100. The die icon is yellow when RNG is active, white when off. The rolled number determines which action is considered "best" based on GTO frequency distribution.
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### PM-009: Score Updates After Each Decision
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User in an active Practice Mode session
- **Steps**:
  1. Make a decision (click an action button)
  2. Observe the score display
- **Expected Result**: The running GTOW Score updates immediately after each decision. The score reflects cumulative performance. A scoring popup briefly shows the classification (Best/Correct/Inaccuracy/Wrong/Blunder) and points earned/lost for the decision.
- **Screenshot Reference**: None

---

#### PM-010: Best Action Shows Green Double Checkmark
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User makes the optimal action in Practice Mode
- **Steps**:
  1. Select the highest-frequency GTO action (or correct RNG-rolled action)
  2. Observe the result feedback
- **Expected Result**: Green double checkmark icon appears with "BEST ACTION" or "PERFECT" label. Full points are awarded. EV values display: hero's action EV, alternative actions with their EVs, and EV loss of 0.0bb.
- **Screenshot Reference**: None

---

#### PM-011: Blunder Shows Red Indicator
- **Priority**: P1 (Critical)
- **Category**: Practice Mode
- **Precondition**: User makes a significant mistake in Practice Mode
- **Steps**:
  1. Select an action that is never taken in GTO and causes significant EV loss
  2. Observe the result feedback
- **Expected Result**: Red triangle/X indicator appears with "BLUNDER" label. Points are deducted. The EV loss amount is displayed prominently. If "Pause after mistakes" is enabled, the hand pauses for review.
- **Screenshot Reference**: None

---

#### PM-012: Session Summary Chart Displays After Completion
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: User completes a Practice Mode session (e.g., 50 hands)
- **Steps**:
  1. Complete all hands in a session
  2. Observe the summary screen
- **Expected Result**: Summary screen shows: Overall GTOW Score, Hands Played, Total EV Lost. A breakdown table shows counts and percentages for Best, Correct, Inaccuracy, Wrong, and Blunder with colored horizontal bars. Biggest blunder hand is identified. Action buttons for Review Blunders, New Session, and Export to Analyze appear.
- **Screenshot Reference**: None

---

#### PM-013: Multitabling 1-4 Tables Grid
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: Practice Mode configured for multiple tables
- **Steps**:
  1. Set tables to 2, 3, or 4 in Practice Mode configuration
  2. Start the session
- **Expected Result**: For 2 tables: side-by-side layout. For 3 tables: 2+1 or 1+2 arrangement. For 4 tables: 2x2 grid. Each table has its own card display, pot, action buttons, and three-dot menu. Each table operates independently with its own hand and decision.
- **Screenshot Reference**: None

---

#### PM-014: Timer Countdown
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: Practice Mode with timebank enabled (7s, 15s, or 25s)
- **Steps**:
  1. Enable timebank in drill configuration
  2. Face a decision
  3. Wait without acting
- **Expected Result**: A countdown timer displays showing seconds remaining. Timer starts at the configured value (7, 15, or 25 seconds) and counts down. When time expires, the decision is auto-resolved. Timer resets for each new decision point.
- **Screenshot Reference**: None

---

#### PM-015: Auto New Hand (Instant and 3-Second Delay)
- **Priority**: P3 (Medium)
- **Category**: Practice Mode
- **Precondition**: Practice Mode with Auto New Hand configured
- **Steps**:
  1. Test with "Instant" auto new hand setting: complete a hand
  2. Test with "3-second delay" setting: complete a hand
- **Expected Result**: With Instant: next hand deals immediately after the current hand result is shown. With 3-second delay: a 3-second pause occurs showing the result before the next hand deals automatically.
- **Screenshot Reference**: None

---

#### PM-016: Pause After Mistakes Setting
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: "Pause after mistakes" enabled in Practice Mode settings
- **Steps**:
  1. Enable "Pause after mistakes only" in settings
  2. Make a correct decision (should not pause)
  3. Make a mistake (should pause)
- **Expected Result**: After correct/best decisions, the hand continues automatically. After Inaccuracy/Wrong/Blunder decisions, the hand pauses to display the scoring result, EV loss, and alternative actions. Player must click "Continue" or "Next Hand" to proceed.
- **Screenshot Reference**: None

---

#### PM-017: Learning Mode Info Panel Auto-Show
- **Priority**: P3 (Medium)
- **Category**: Practice Mode
- **Precondition**: Learning Mode enabled in Practice Mode settings
- **Steps**:
  1. Enable Learning Mode
  2. Make a mistake that triggers a pause
  3. Observe the Info Panel
- **Expected Result**: The Info Panel automatically opens/expands when the hand pauses after a mistake, showing the Strategy and Range tabs with the 13x13 matrix and GTO action breakdown for the current spot.
- **Screenshot Reference**: None

---

#### PM-018: View in Study Mode Button
- **Priority**: P2 (High)
- **Category**: Practice Mode
- **Precondition**: User viewing hand result in Practice Mode
- **Steps**:
  1. Complete a hand in Practice Mode
  2. Click "View in Study Mode" button
- **Expected Result**: Application navigates to Study Mode with the exact same spot pre-loaded (same positions, stack depth, board, and action sequence). The strategy matrix shows the full solver solution for this spot.
- **Screenshot Reference**: None

---

#### PM-019: Replay Button
- **Priority**: P3 (Medium)
- **Category**: Practice Mode
- **Precondition**: User viewing hand result in Practice Mode
- **Steps**:
  1. Complete a hand
  2. Click the "Replay" button
- **Expected Result**: The hand action replays step by step, showing each decision point and its resolution. Navigation controls allow stepping forward and backward through the streets.
- **Screenshot Reference**: None

---

#### PM-020: Speed Settings (Normal/Fast/Turbo)
- **Priority**: P3 (Medium)
- **Category**: Practice Mode
- **Precondition**: Practice Mode active
- **Steps**:
  1. Switch between Normal, Fast, and Turbo speed settings
  2. Observe animation and pacing differences
- **Expected Result**: Normal: standard animation delays between cards and actions. Fast: noticeably reduced animation delays. Turbo: minimal delay, near-instant transitions between decision points. All speeds function correctly without rendering glitches.
- **Screenshot Reference**: None

---

## 5. Analyze Mode Tests (AN-001 through AN-015)

Tests for the hand history analyzer upload, results, filtering, and review functionality.

---

#### AN-001: File Upload Drag-and-Drop Zone Accepts Files
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User in Analyze Mode upload screen
- **Steps**:
  1. Open the Upload dialog
  2. Drag a hand history file over the drop zone
  3. Release the file
- **Expected Result**: Drop zone visually highlights (border color change or background change) when a file is dragged over it. Releasing the file initiates the upload. A progress indicator appears. The text "Drop hand history files here or click to browse" is displayed. Clicking the zone opens a file browser dialog.
- **Screenshot Reference**: None

---

#### AN-002: Upload Accepts Supported Formats (17+ Poker Sites)
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User has hand history files from various poker sites
- **Steps**:
  1. Upload hand histories from different supported sites (PokerStars, 888, GGPoker, WPN, Winamax, PartyPoker, iPoker, Coinpoker, Chico, Bovada/Ignition, PokerBros, PokerMaster, PokerTime, PPPoker, Unibet, UPoker, WePoker)
  2. Check that each is accepted and processed
- **Expected Result**: Files from all 17+ supported sites are accepted without error. The system detects the file format automatically. Unsupported formats show a clear error message listing supported sites. Accepts .txt and .xml formats, plus folder upload.
- **Screenshot Reference**: None

---

#### AN-003: Processing Indicator Shows During Analysis
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: Files uploaded for analysis
- **Steps**:
  1. Upload a batch of hand history files
  2. Observe the status display during processing
- **Expected Result**: Status progresses through stages: Uploading -> In Queue -> Processing -> Analyzed. Each stage is visually indicated with progress bars or spinners. Counts show: Total hands, Analyzed, Duplicates, Unsupported, Errors. Users can navigate away during processing.
- **Screenshot Reference**: None

---

#### AN-004: Results Table Renders with Sortable Columns
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User has analyzed hands in Analyze Mode
- **Steps**:
  1. Navigate to the analyzed hands table
  2. Click various column headers to sort
- **Expected Result**: Table renders with columns including: checkbox, Pot (bb), Tag, Status, Format, Site, Position, Hand (card images), Board (card images), Pot Type, and action columns. Clicking any column header sorts the table by that column (ascending/descending toggle). Sort direction indicator (arrow) appears on the active sort column.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hands-table.png`

---

#### AN-005: Classification Colors Correct (PERFECT Green, BLUNDER Red)
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User viewing analyzed hands with various classifications
- **Steps**:
  1. View the analyzed hands table or stats breakdown
  2. Observe the color coding for each classification
- **Expected Result**: PERFECT: green icon/label. GOOD: teal/green icon/label. INACCURATE: yellow/amber icon/label. WRONG: red icon/label. BLUNDER: red icon/label with distinct marker (triangle-!). Colors in the breakdown table match the legend consistently.
- **Screenshot Reference**: `screenshots/blog/analyzer2-srp-report.png`

---

#### AN-006: Hand Replay Navigates Between Streets
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User clicks on an analyzed hand to open the replay view
- **Steps**:
  1. Click on a hand in the results table
  2. Use navigation controls to move between streets (Preflop, Flop, Turn, River)
- **Expected Result**: Street navigation buttons (Preflop, Flop, Turn, River) allow stepping through the hand. Each street shows: board cards for that street, player actions taken, GTO strategy for the decision point, EV of the player's action vs. GTO action, EV loss and classification. Arrow keys or navigation buttons allow forward/backward movement.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hand-analysis.png`

---

#### AN-007: EV Loss Displayed Per Decision Point
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User viewing a hand replay
- **Steps**:
  1. Open a hand replay
  2. Navigate to a decision point with a non-zero EV loss
- **Expected Result**: EV loss displays prominently in big blinds (e.g., "-1.8bb"). The player's action EV, GTO action EV, and the difference (EV loss) are all shown. The classification (PERFECT/GOOD/INACCURATE/WRONG/BLUNDER) appears with the corresponding color indicator.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hand-analysis.png`

---

#### AN-008: Filter Bar Filters by Position
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User viewing analyzed hands table with filter bar
- **Steps**:
  1. Open the filter system
  2. Select a position filter (e.g., "BTN" or "BB")
  3. Apply the filter
- **Expected Result**: Table updates to show only hands played from the selected position. An active filter badge appears showing the position filter. The badge has an "x" button to remove the filter. Stats cards update to reflect filtered results.
- **Screenshot Reference**: `screenshots/blog/redesigned-hand-filtering.png`

---

#### AN-009: Filter Bar Filters by Street
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User viewing analyzed hands with filters available
- **Steps**:
  1. Open the filter bar
  2. Apply a street filter (e.g., "Flop" decisions only)
- **Expected Result**: Table shows only decisions made on the selected street. Hands without decisions on that street are hidden. Filter badge appears. Stats update to reflect the filtered subset.
- **Screenshot Reference**: None

---

#### AN-010: Filter Bar Filters by Action Type
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User viewing analyzed hands with filters available
- **Steps**:
  1. Open the filter bar
  2. Select a Pot Type filter (e.g., "SRP", "3Bet", "ISO", "LIMP", "SQUEEZE")
- **Expected Result**: Table filters to show only hands matching the selected pot type. Multiple pot types can be combined. Filter badges appear for each active filter.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hands-table.png`

---

#### AN-011: Filter by Deviation Severity
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User in Analyze Mode with analyzed hands
- **Steps**:
  1. Filter hands by deviation category (e.g., show only BLUNDER or WRONG decisions)
- **Expected Result**: Table filters to show only hands matching the selected deviation severity. This enables rapid identification of the worst mistakes.
- **Screenshot Reference**: None

---

#### AN-012: Sort by EV Loss (Descending)
- **Priority**: P1 (Critical)
- **Category**: Analyze Mode
- **Precondition**: User viewing analyzed hands table
- **Steps**:
  1. Click the EV Loss column header
  2. Set sort order to descending
- **Expected Result**: Hands are sorted with the largest EV losses (biggest blunders) at the top. This is the primary workflow for hand review. Sort arrow indicates descending direction.
- **Screenshot Reference**: None

---

#### AN-013: Open in Study Mode from Analyzed Hand
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User viewing an analyzed hand detail
- **Steps**:
  1. Select an analyzed hand
  2. Click "Open in Study Mode" button
- **Expected Result**: Application navigates to Study Mode with the exact board, positions, stack depth, and action sequence from the analyzed hand pre-loaded. The strategy matrix shows the complete solver solution for that spot.
- **Screenshot Reference**: None

---

#### AN-014: Natural Language Filter Autocomplete
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User in Analyze Mode filter area
- **Steps**:
  1. Click on the search/filter input
  2. Type a partial query (e.g., "did")
- **Expected Result**: An autocomplete dropdown appears showing matching filter suggestions: "Hero Preflop Action (Did RFI)", "Hero Preflop Action (Did Limp)", "Hero Preflop Action (Did ISO)", "Hero Preflop Action (Did Squeeze)", "Hero Preflop Action (Did 3Bet)", "Hero Preflop Action (Did 4Bet)", "Hero Preflop Action (Did 5Bet)". Selecting an option applies the filter.
- **Screenshot Reference**: `screenshots/blog/redesigned-natural-language-filter.png`

---

#### AN-015: Action Sequence Builder (Check/Bet S/M/L/O)
- **Priority**: P2 (High)
- **Category**: Analyze Mode
- **Precondition**: User in Analyze Mode advanced filtering
- **Steps**:
  1. Open the "Streets Actions" filter tab
  2. Observe the "Flop Line - Actions Sequence" builder
- **Expected Result**: A multi-row visual builder appears with toggleable buttons: first row shows "Check | Bet" with Bet sub-sizes (S/M/L/O for Small/Medium/Large/Overbet). Response row shows "Fold | Call | Raise" with Raise sub-sizes (M/L/O). "Turn Line" builder follows the same pattern. Each button toggles independently. "Remove filter" option appears per filter row.
- **Screenshot Reference**: `screenshots/blog/redesigned-hand-filtering.png`

---

## 6. Range Builder Tests (RB-001 through RB-015)

Tests for range construction, editing, and evaluation.

---

#### RB-001: 13x13 Grid Supports Click to Toggle
- **Priority**: P1 (Critical)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Open the Range Builder
  2. Click on any empty cell in the 13x13 grid
  3. Click the same cell again
- **Expected Result**: First click selects/toggles the hand into the range (cell becomes highlighted/colored). Second click deselects/removes the hand from the range (cell returns to unselected state). The selected range combo count updates in real-time in the Strategy Output panel.
- **Screenshot Reference**: None

---

#### RB-002: Drag to Select Multiple Cells
- **Priority**: P1 (Critical)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Click and hold on one cell
  2. Drag across multiple cells
  3. Release
- **Expected Result**: All cells touched during the drag operation are selected/toggled. Selection is visible as cells change color during the drag. The drag operation feels responsive with no perceptible lag. Combo count updates after release.
- **Screenshot Reference**: None

---

#### RB-003: Shift-Click for Painting Mode
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Hold Shift
  2. Click and drag across cells
- **Expected Result**: Painting mode activates, allowing continuous selection (or deselection) of cells as the cursor moves over them. This is more fluid than individual toggle clicking. The paint operation applies the current selection state (add or remove) consistently across all touched cells.
- **Screenshot Reference**: None

---

#### RB-004: Weight Slider Adjusts 0-100% Per Combo
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder with cells selected
- **Steps**:
  1. Select a hand in the range
  2. Adjust the weight slider for that hand
- **Expected Result**: A slider control allows setting a frequency weight between 0% and 100% for each combo. The cell color intensity changes proportionally (bright at 100%, dim at lower weights). The overall range composition updates to reflect weighted combos.
- **Screenshot Reference**: None

---

#### RB-005: Lock Icon Prevents Overwriting
- **Priority**: P3 (Medium)
- **Category**: Range Builder
- **Precondition**: User in Range Builder with cells selected
- **Steps**:
  1. Select a hand and set its weight
  2. Click the lock icon for that hand
  3. Attempt to modify the locked hand via drag or paint operations
- **Expected Result**: The lock icon toggles to a locked state (visually indicated by a closed lock). Subsequent drag or paint operations skip locked cells. Locked hands retain their set weights. Individual unlock via clicking the lock icon again.
- **Screenshot Reference**: None

---

#### RB-006: Suit Filter Grid Toggles Individual Suits
- **Priority**: P3 (Medium)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Locate the suit filter control
  2. Toggle individual suits on/off
- **Expected Result**: A 4x4 suit combination grid allows selecting specific suit combos for suited hands (e.g., only spade-heart combinations). Deselected suits are excluded from the range. The grid updates the combo count and visual display immediately.
- **Screenshot Reference**: None

---

#### RB-007: Evaluation Shows Side-by-Side (User Range vs GTO)
- **Priority**: P1 (Critical)
- **Category**: Range Builder
- **Precondition**: User has constructed a range in Range Builder
- **Steps**:
  1. Build a range by selecting hands
  2. Observe the evaluation section
- **Expected Result**: The Strategy Output panel displays the user's constructed range alongside the GTO reference range. Differences are highlighted. GTO Score shows a 0-100% accuracy rating. Deviation from GTO is displayed in bb/100. Per-action breakdowns show Raise/Call/Fold percentages for both user and GTO ranges.
- **Screenshot Reference**: None

---

#### RB-008: GTO Score Grades 0-100%
- **Priority**: P1 (Critical)
- **Category**: Range Builder
- **Precondition**: User has a range constructed in Range Builder
- **Steps**:
  1. Build a range
  2. Observe the GTO Score display
- **Expected Result**: A numerical score between 0% and 100% is displayed, representing how closely the user's range matches the GTO solution. 100% = perfect match. The score updates in real-time as the user modifies the range. Deviation from GTO (in bb/100) is also shown.
- **Screenshot Reference**: None

---

#### RB-009: Load/Save Range
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Build a custom range
  2. Click "Save Range"
  3. Name and save the range
  4. Clear the range
  5. Click "Load Range" and select the saved range
- **Expected Result**: Save dialog allows naming the range. The saved range appears in the load dialog. Loading restores the exact same selection with correct weights. Multiple ranges can be saved and loaded.
- **Screenshot Reference**: None

---

#### RB-010: Clear Selection
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder with cells selected
- **Steps**:
  1. Select multiple hands in the range
  2. Click "Clear" button
- **Expected Result**: All selected cells are deselected. The range is empty. Combo count shows 0. GTO Score resets. The grid returns to its default unselected state.
- **Screenshot Reference**: None

---

#### RB-011: Select All
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder
- **Steps**:
  1. Click "Select All" button
- **Expected Result**: All 169 cells are selected at 100% weight. Combo count shows the full range. The entire grid is highlighted/colored. GTO Score updates to reflect a 100% range.
- **Screenshot Reference**: None

---

#### RB-012: Board Selection (Specific/Random/Filtered)
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User in Range Builder configuration
- **Steps**:
  1. Test specific board selection: choose exact cards (e.g., Ks 7h 2d)
  2. Test random board: select "Random"
  3. Test filtered board: apply board texture filters
- **Expected Result**: Specific: the selected board cards appear and the range evaluation uses that exact board. Random: a random board is generated each time. Filtered: board generation respects texture filters (e.g., monotone, paired, connected).
- **Screenshot Reference**: None

---

#### RB-013: Right-Click for Frequency Adjustment
- **Priority**: P3 (Medium)
- **Category**: Range Builder
- **Precondition**: User in Range Builder with cells in the grid
- **Steps**:
  1. Right-click on a cell in the range builder grid
- **Expected Result**: A context menu or popup appears allowing direct frequency/weight input for that specific hand combo. The frequency can be set as a percentage (0-100%).
- **Screenshot Reference**: None

---

#### RB-014: Difficulty Levels (Easy/Medium/Hard)
- **Priority**: P3 (Medium)
- **Category**: Range Builder
- **Precondition**: User in Range Builder with difficulty options
- **Steps**:
  1. Select Easy difficulty
  2. Select Medium difficulty
  3. Select Hard difficulty
- **Expected Result**: Easy: fewer decision points, broader hand groupings. Medium: standard hand-by-hand decisions. Hard: includes suit-specific frequency decisions. The difficulty level affects the scoring granularity and what constitutes a passing range.
- **Screenshot Reference**: None

---

#### RB-015: GTO Score Deviation Display
- **Priority**: P2 (High)
- **Category**: Range Builder
- **Precondition**: User has built a range with deviations from GTO
- **Steps**:
  1. Build a range that intentionally deviates from GTO (e.g., missing some 3-bet hands)
  2. Observe the deviation display
- **Expected Result**: The interface shows which specific hands deviate from GTO. Hands that should be included but are not are indicated. Hands that are included but should not be are indicated. The overall deviation cost in bb/100 is displayed.
- **Screenshot Reference**: None

---

## 7. Interaction & Keyboard Tests (KB-001 through KB-015)

Tests for keyboard shortcuts, hover states, and interaction patterns.

---

#### KB-001: Keys 1/2/3/4 Switch Study Mode Tabs
- **Priority**: P1 (Critical)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode with a spot loaded
- **Steps**:
  1. Press the "1" key
  2. Press the "2" key
  3. Press the "3" key
  4. Press the "4" key
- **Expected Result**: 1: Strategy + EV tab activates. 2: Ranges tab activates. 3: Breakdown tab activates. 4: Reports: Flops tab activates. Each keypress switches the view immediately. The active tab indicator updates correctly.
- **Screenshot Reference**: None

---

#### KB-002: J Key Opens Jump-to-Spot Popup
- **Priority**: P1 (Critical)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode
- **Steps**:
  1. Press the "J" key
- **Expected Result**: A popup/modal opens showing a table for direct spot selection. The popup allows choosing positions and action sequences without navigating dropdown by dropdown. Pressing Escape or clicking outside closes the popup.
- **Screenshot Reference**: None

---

#### KB-003: S Key Toggles Chart vs Table in Actions Table
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode with the Actions Table visible
- **Steps**:
  1. Observe the Actions Table (default view: table or chart)
  2. Press the "S" key
  3. Press "S" again
- **Expected Result**: Pressing S toggles between the bar chart visualization and the numerical table view of action frequencies. The data content is the same in both views, just the visualization format changes. Each press toggles the state.
- **Screenshot Reference**: None

---

#### KB-004: P Key Clears All Filters
- **Priority**: P1 (Critical)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode with one or more filters active
- **Steps**:
  1. Apply several filters (hand category, suit, action)
  2. Press the "P" key
- **Expected Result**: All active filters are cleared instantly. The strategy matrix returns to full unfiltered display. All filter badges/toggles reset to their default (off) state. Actions Table shows full-range frequencies.
- **Screenshot Reference**: None

---

#### KB-005: Q Key Maximizes Solution Browser
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode
- **Steps**:
  1. Press the "Q" key
  2. Press "Q" again
- **Expected Result**: First press: the solution browser (strategy matrix area) maximizes to fill available screen space, hiding sidebars/infobox. Second press: restores the normal multi-panel layout. The transition is smooth (< 300ms).
- **Screenshot Reference**: None

---

#### KB-006: Spacebar Toggles BB vs Pot % Display
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode with bet/pot values displayed
- **Steps**:
  1. Observe current display format (BB or % pot)
  2. Press Spacebar
  3. Press Spacebar again
- **Expected Result**: Spacebar toggles between displaying values in big blinds (e.g., "4.3 BB") and percentage of pot (e.g., "33% pot"). All relevant displays update simultaneously (Actions Table, infobox, cell popups). Each press toggles the state.
- **Screenshot Reference**: None

---

#### KB-007: Shift+Space Toggles EV vs EV/Pot
- **Priority**: P3 (Medium)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode with EV values displayed
- **Steps**:
  1. Observe current EV display format
  2. Press Shift+Space
  3. Press Shift+Space again
- **Expected Result**: Toggles between displaying raw EV in big blinds (e.g., "+2.3bb") and EV as a percentage of pot (e.g., "+35% pot"). Both formats are displayed with consistent precision.
- **Screenshot Reference**: None

---

#### KB-008: Hover States Show Tooltips Within 200ms
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in any mode with hoverable elements
- **Steps**:
  1. Hover over a strategy matrix cell
  2. Time the tooltip/popup appearance
  3. Move to another cell
  4. Verify tooltip updates
- **Expected Result**: Tooltips/popups appear within 200ms of the cursor entering the hover target. Moving to a different target updates the tooltip content within 200ms. Moving away from all targets dismisses the tooltip within 200ms. No flickering or jittering during rapid mouse movement across cells.
- **Screenshot Reference**: None

---

#### KB-009: Click States Provide Visual Feedback
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in any mode with clickable buttons
- **Steps**:
  1. Click on any button (action buttons, nav links, filter toggles)
  2. Observe visual feedback
- **Expected Result**: Buttons show a visual pressed/depressed state on mousedown (darker shade, slight scale reduction, or border change). The active/selected state is visually distinct from hover and default states. Feedback is immediate (< 50ms response time).
- **Screenshot Reference**: None

---

#### KB-010: Drag Operations on Range Builder Responsive
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Range Builder
- **Steps**:
  1. Click and drag across multiple cells in the range builder grid
  2. Assess responsiveness
- **Expected Result**: Drag selection updates in real-time with no perceptible lag (< 100ms). Selected cells highlight immediately as the cursor passes over them. No frame drops or stuttering during drag operations across the 13x13 grid.
- **Screenshot Reference**: None

---

#### KB-011: All Dropdowns Open on Click and Close on Escape/Outside Click
- **Priority**: P2 (High)
- **Category**: Interaction & Keyboard
- **Precondition**: User at any screen with dropdown menus
- **Steps**:
  1. Click on a dropdown trigger (e.g., metric dropdown, format selector)
  2. Press Escape
  3. Re-open the dropdown
  4. Click outside the dropdown
- **Expected Result**: Click opens the dropdown with options visible. Pressing Escape closes the dropdown without selection. Clicking outside the dropdown closes it without selection. Selecting an option closes the dropdown and applies the selection. Only one dropdown is open at a time.
- **Screenshot Reference**: None

---

#### KB-012: Tab Key Navigation Between Form Fields
- **Priority**: P3 (Medium)
- **Category**: Interaction & Keyboard
- **Precondition**: User on a screen with form inputs (e.g., GTO Wizard AI config, Settings)
- **Steps**:
  1. Click on the first form field
  2. Press Tab to move to next field
  3. Continue Tab through all fields
  4. Press Shift+Tab to go back
- **Expected Result**: Tab moves focus to the next interactive element in logical order. Shift+Tab moves focus backward. Focus indicator is clearly visible on each focused element. No elements are skipped or trapped.
- **Screenshot Reference**: None

---

#### KB-013: Focus Indicators on Interactive Elements
- **Priority**: P3 (Medium)
- **Category**: Interaction & Keyboard
- **Precondition**: User navigating via keyboard
- **Steps**:
  1. Use Tab to navigate through the interface
  2. Observe focus indicators
- **Expected Result**: Every interactive element (buttons, links, inputs, toggles, dropdowns) shows a visible focus indicator when focused. The indicator is clearly visible against the dark background (e.g., a bright border, outline, or glow). Focus indicators are not hidden or suppressed.
- **Screenshot Reference**: None

---

#### KB-014: Context Menus Position Correctly Near Screen Edges
- **Priority**: P3 (Medium)
- **Category**: Interaction & Keyboard
- **Precondition**: User near the edge of the viewport
- **Steps**:
  1. Right-click on a cell near the right edge of the strategy matrix
  2. Right-click on a cell near the bottom edge
  3. Observe context menu positioning
- **Expected Result**: Context menus reposition to stay fully within the viewport. Menus that would overflow the right edge open to the left instead. Menus that would overflow the bottom open upward. No menu content is clipped or hidden.
- **Screenshot Reference**: None

---

#### KB-015: Keyboard Shortcut Help Overlay
- **Priority**: P3 (Medium)
- **Category**: Interaction & Keyboard
- **Precondition**: User in Study Mode
- **Steps**:
  1. Look for a keyboard shortcuts help trigger (e.g., "?" key or help icon)
  2. Open the shortcuts overlay
- **Expected Result**: An overlay or panel shows all available keyboard shortcuts: 1/2/3/4 (tabs), J (jump), S (toggle), P (clear), Q (maximize), Spacebar (toggle display), Shift+Space (toggle EV format). The overlay is dismissable via Escape or clicking outside.
- **Screenshot Reference**: None

---

## 8. Visual Fidelity Tests (VF-001 through VF-020)

Screenshot-comparison tests verifying the rebuilt UI matches reference screenshots within defined thresholds.

---

#### VF-001: Study Mode Strategy View Matches Reference
- **Priority**: P1 (Critical)
- **Category**: Visual Fidelity
- **Precondition**: App loaded in Study Mode with the same spot as the reference screenshot
- **Steps**:
  1. Load the same game format, positions, and board as in the reference
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Overall layout, component placement, color scheme, and proportions match the reference screenshot within a 1.0% pixel difference threshold. Infobox position, matrix position, tab bar, and filter area are all visually consistent.
- **Screenshot Reference**: `screenshots/website/ai-solver-default.avif`

---

#### VF-002: Study Mode Library View Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App loaded in Study Mode solution browser / library view
- **Steps**:
  1. Navigate to the solutions library
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Library tree structure, game category layout, spot navigation controls, and solution browser panel match the reference within 1.0% pixel difference threshold.
- **Screenshot Reference**: `screenshots/website/library-of-solutions-default.avif`

---

#### VF-003: Practice Mode Trainer View Matches Reference
- **Priority**: P1 (Critical)
- **Category**: Visual Fidelity
- **Precondition**: App loaded in Practice Mode with an active hand
- **Steps**:
  1. Start a Practice Mode session
  2. Capture a screenshot during a decision
  3. Compare against reference
- **Expected Result**: Table shape, card rendering, position labels, pot display, action buttons, and scoring display match the reference within 2.0% pixel difference threshold (higher tolerance for third-party screenshot source).
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`

---

#### VF-004: Practice Mode Advanced Configuration Matches Reference
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App loaded on Practice Mode drill configuration screen
- **Steps**:
  1. Open advanced drill configuration
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Configuration panel layout, dropdown placement, option groupings, and button styling match the reference within 1.0% pixel difference threshold.
- **Screenshot Reference**: `screenshots/website/practice-advanced-default.avif`

---

#### VF-005: Analyze Mode Results Table Matches Reference
- **Priority**: P1 (Critical)
- **Category**: Visual Fidelity
- **Precondition**: App in Analyze Mode with analyzed hands visible
- **Steps**:
  1. Navigate to the analyzed hands table
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Table column widths, header styling (uppercase, muted grey), row heights, card image rendering, status icons, and filter tab positions match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hands-table.png`

---

#### VF-006: Analyze Mode Hand Review Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App in Analyze Mode hand replay view
- **Steps**:
  1. Open a hand replay
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Hand replay layout, board card rendering, action display, EV loss presentation, and GTO strategy comparison match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/analyzer2-hand-analysis.png`

---

#### VF-007: GTO Reports Preflop Stats View Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App in GTO Reports with preflop stats visible
- **Steps**:
  1. Load GTO Reports with a configured game type
  2. View preflop statistics
  3. Capture a screenshot
  4. Compare against reference
- **Expected Result**: Stat categories (VPIP, PFR, RFI, etc.), VALUE vs GTO deviation bars, color coding, and layout match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/gto-reports-preflop-stats.png`

---

#### VF-008: GTO Reports Filtering Interface Matches Reference
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App in GTO Reports with filtering panel open
- **Steps**:
  1. Open the Game Type selector / filter configuration
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Game Type dropdown, format toggle buttons (Cash/MTT/Spins/HuSng), Players, Stacks, Rake options, and layout match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/gto-reports-filtering.png`

---

#### VF-009: Nodelocking Interface Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App in Nodelocking mode
- **Steps**:
  1. Open Nodelocking
  2. Navigate to a game tree node
  3. Capture a screenshot
  4. Compare against reference
- **Expected Result**: Horizontal card-based game tree, three-tab interface (Set strategy/Set frequency/Lock Unlock), lock/pencil/compare icons, and overall layout match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/nodelocking-interface.png`

---

#### VF-010: Nodelocking Set Frequency View Matches Reference
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App in Nodelocking, Set Frequency tab active
- **Steps**:
  1. Open the "Set frequency" tab in Nodelocking
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Frequency bars (Raise red, Call green, Fold blue), percentage inputs, per-action lock icons, and "Overwrite unlocked/Overwrite all" toggle match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/nodelocking-set-frequency.png`

---

#### VF-011: Aggregated Reports View Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App in Study Mode Reports: Flops tab
- **Steps**:
  1. Navigate to Aggregated Reports (Reports: Flops in Study Mode)
  2. View the table mode
  3. Capture a screenshot
  4. Compare against reference
- **Expected Result**: Dual-panel layout with independent tab bars, table columns (Flops, Strategy bars, Check%, Bet sizes, OOP, EV, IP, EQ, EQR), and "Used by" summary row match the reference within 1.5% pixel difference threshold.
- **Screenshot Reference**: `screenshots/website/aggregated-reports.png`

---

#### VF-012: Card Rendering (Suit Symbols, Face Design)
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: Cards visible in any mode (Practice, Analyze, Study)
- **Steps**:
  1. Navigate to a view with visible cards
  2. Capture close-up of card rendering
  3. Compare against reference
- **Expected Result**: Suit symbols (spades, hearts, diamonds, clubs) are clearly rendered with correct colors (red for hearts/diamonds, black/blue for spades/clubs). Face values are legible. Card backgrounds and borders match the selected card theme. Card back design is consistent.
- **Screenshot Reference**: `screenshots/reviews/solvers-poker-trainer.png`

---

#### VF-013: Font Sizes and Families Match Reference
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App loaded
- **Steps**:
  1. Compare rendered text at various hierarchy levels against reference screenshots
- **Expected Result**: Page titles are large bold sans-serif (~24-32px). Section headers are medium bold (~18-20px). Table headers are uppercase, semi-bold, muted grey with visible letter-spacing. Stat card values are extra-large bold (~36-48px). Stat card labels are small, muted grey (~12-14px). Font family is consistent throughout.
- **Screenshot Reference**: All screenshots

---

#### VF-014: Spacing and Padding Match Within 5px Tolerance
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App loaded
- **Steps**:
  1. Measure internal padding of panels, stat cards, and table rows
  2. Compare against reference screenshots
- **Expected Result**: Panel internal padding is approximately 16-24px. Table row heights are approximately 48-60px. Stat cards have consistent gaps and rounded corners. Sidebar widths are approximately 200-250px (left) and 300-350px (right). All measurements within 5px of reference values.
- **Screenshot Reference**: All screenshots

---

#### VF-015: Border Radius on Buttons/Cards/Panels
- **Priority**: P4 (Low)
- **Category**: Visual Fidelity
- **Precondition**: App loaded
- **Steps**:
  1. Observe border radius on buttons, stat cards, panels, and modals
  2. Compare against reference screenshots
- **Expected Result**: UI elements use consistent border radius values. Buttons, cards, and panels have rounded corners matching the design system. No elements have unexpected sharp corners or overly rounded shapes. Border radius values are consistent across similar component types.
- **Screenshot Reference**: All screenshots

---

#### VF-016: Dashboard Layout Matches Reference
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App loaded on Dashboard
- **Steps**:
  1. Navigate to Dashboard
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: 9-tile grid arrangement, tile icons, tile descriptions, bottom stats panels (Trainer stats, Analyzer stats), and overall layout match the reference within 2.0% pixel difference threshold.
- **Screenshot Reference**: `screenshots/reviews/h2n-dashboard.webp`

---

#### VF-017: GTO Wizard AI Configuration View
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: App in GTO Wizard AI custom solver screen
- **Steps**:
  1. Open GTO Wizard AI
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Player configuration sections, betting tree inputs, accuracy target dropdown, Solve/Load/Save buttons, and Power Credits display match the reference within 1.0% pixel difference threshold.
- **Screenshot Reference**: `screenshots/website/ai-solver-default.avif`

---

#### VF-018: Table Wizard Overlay Configuration Matches Reference
- **Priority**: P3 (Medium)
- **Category**: Visual Fidelity
- **Precondition**: Table Wizard application open on Overlays page
- **Steps**:
  1. Navigate to Overlays in Table Wizard
  2. Capture a screenshot
  3. Compare against reference
- **Expected Result**: Left sidebar navigation, five toggle categories (Table Borders, RNG, Betting Buttons, Actions, Table Info), poker table preview with position overlays, and "Active" toggle match the reference within 2.0% pixel difference threshold.
- **Screenshot Reference**: `screenshots/blog/table-wizard-overlay.png`

---

#### VF-019: Action Button Styling in Practice Mode
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: Practice Mode with action buttons visible
- **Steps**:
  1. Face a decision in Practice Mode
  2. Capture a screenshot of the action buttons
  3. Compare against reference
- **Expected Result**: Action buttons show: CHECK as green, BET amounts in graduated red (lighter for smaller bets, darker for larger), ALLIN as darkest red. Button sizes, text formatting (uppercase), and spacing match the reference. Each button displays the action name and BB amount.
- **Screenshot Reference**: `screenshots/reviews/h2n-trainer.webp`

---

#### VF-020: Navigation Bar Styling Across All Screens
- **Priority**: P2 (High)
- **Category**: Visual Fidelity
- **Precondition**: App loaded
- **Steps**:
  1. Navigate to multiple screens (Dashboard, Study, Practice, Analyze)
  2. Capture the navigation bar on each
  3. Compare consistency and against reference
- **Expected Result**: Navigation bar is visually identical across all screens: same height, same background color, same logo position, same link styling, same active state indicator (teal/cyan highlight). The active tab/link changes to reflect the current mode. No layout shift or visual changes to the nav bar structure between screens.
- **Screenshot Reference**: `screenshots/reviews/h2n-main.webp`, `screenshots/reviews/h2n-dashboard.webp`

---

## Appendix: Test Summary

| Category | ID Range | Count | P1 | P2 | P3 | P4 |
|----------|----------|-------|----|----|----|----|
| Layout & Structure | LS-001 to LS-030 | 30 | 11 | 12 | 6 | 1 |
| Color System | CS-001 to CS-020 | 20 | 7 | 8 | 4 | 1 |
| Strategy Matrix | SM-001 to SM-025 | 25 | 10 | 10 | 5 | 0 |
| Practice Mode | PM-001 to PM-020 | 20 | 6 | 8 | 5 | 1 |
| Analyze Mode | AN-001 to AN-015 | 15 | 6 | 7 | 2 | 0 |
| Range Builder | RB-001 to RB-015 | 15 | 4 | 5 | 4 | 2 |
| Interaction & Keyboard | KB-001 to KB-015 | 15 | 3 | 6 | 6 | 0 |
| Visual Fidelity | VF-001 to VF-020 | 20 | 3 | 9 | 7 | 1 |
| **Total** | | **160** | **50** | **65** | **39** | **6** |

---

## Appendix: Priority Distribution

- **P1 (Critical):** 50 tests (31%) -- Must pass for release
- **P2 (High):** 65 tests (41%) -- Should pass for quality release
- **P3 (Medium):** 39 tests (24%) -- Desired for polish
- **P4 (Low):** 6 tests (4%) -- Nice-to-have

---

## Appendix: Screenshot Reference Index

| Reference File | Tests Using It |
|---------------|----------------|
| `screenshots/reviews/h2n-main.webp` | LS-001, LS-008, LS-009, LS-010, LS-028, CS-001, CS-002, CS-003, SM-001, SM-005, SM-021, SM-022, VF-020 |
| `screenshots/reviews/h2n-dashboard.webp` | LS-003, LS-004, VF-016, VF-020 |
| `screenshots/reviews/h2n-trainer.webp` | LS-012, LS-013, CS-007, CS-011, PM-004, PM-007, PM-008, VF-019 |
| `screenshots/reviews/solvers-poker-trainer.png` | LS-012, CS-011, PM-001, PM-002, PM-003, VF-003, VF-012 |
| `screenshots/reviews/h2n-settings.webp` | LS-022 |
| `screenshots/website/ai-solver-default.avif` | LS-005, LS-009, LS-017, VF-001, VF-017 |
| `screenshots/website/library-of-solutions-default.avif` | VF-002 |
| `screenshots/website/practice-advanced-default.avif` | VF-004 |
| `screenshots/website/aggregated-reports.png` | VF-011 |
| `screenshots/website/battle-heads-up-poker-default.avif` | LS-019 |
| `screenshots/blog/analyzer2-hands-table.png` | LS-014, AN-004, AN-010, VF-005 |
| `screenshots/blog/analyzer2-hand-analysis.png` | AN-006, AN-007, VF-006 |
| `screenshots/blog/analyzer2-srp-report.png` | LS-015, CS-009, CS-010, AN-005 |
| `screenshots/blog/gto-reports-preflop-stats.png` | LS-029, CS-015, VF-007 |
| `screenshots/blog/gto-reports-filtering.png` | VF-008 |
| `screenshots/blog/gto-reports-position-matrix.png` | CS-016 |
| `screenshots/blog/redesigned-preflop-mistakes.png` | LS-029 |
| `screenshots/blog/redesigned-hand-filtering.png` | AN-008, AN-015 |
| `screenshots/blog/redesigned-natural-language-filter.png` | AN-014 |
| `screenshots/blog/nodelocking-interface.png` | LS-016, LS-018, VF-009 |
| `screenshots/blog/nodelocking-set-frequency.png` | LS-018, VF-010 |
| `screenshots/blog/aggregated-reports-flop-cbet.png` | CS-020 |
| `screenshots/blog/table-wizard-overlay.png` | LS-020, LS-021, VF-018 |

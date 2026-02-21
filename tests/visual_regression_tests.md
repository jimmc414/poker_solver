# Visual Regression Test Suite

Maps each visual regression test to its reference screenshot with acceptable diff thresholds. This suite complements the functional UI/UX tests in `ui_ux_test_spec.md` by verifying pixel-level visual accuracy against reference screenshots.

---

## Test Infrastructure

### Recommended Tools

| Tool | Type | Best For | Notes |
|------|------|----------|-------|
| **Playwright** | Built-in | Full-page and element screenshots | `toHaveScreenshot()` API, built-in comparison, CI-friendly. Recommended primary tool. |
| **Percy** (percy.io) | Cloud SaaS | Cross-browser visual testing | CI-integrated, automatic baselines, GitHub PR comments with diffs |
| **Chromatic** (chromatic.com) | Cloud SaaS | Storybook component libraries | Component-level testing, catches interaction state changes |
| **reg-suit** | Open-source | Self-hosted visual regression | S3/GCS backend, GitHub integration, free |
| **jest-image-snapshot** | Jest plugin | Unit-level image comparison | Pixel-level diff, customizable thresholds, local execution |
| **BackstopJS** | Open-source | Responsive visual testing | Multiple viewport configs, Docker support, visual diff reports |

### Configuration

```yaml
# Recommended default configuration
visual_regression:
  default_threshold: 0.5       # 0.5% pixel difference tolerance
  anti_aliasing_tolerance: true # Ignore anti-aliasing differences
  color_comparison: "deltaE2000" # Perceptual color difference (CIE Delta E 2000)

  viewports:
    desktop:
      width: 1920
      height: 1080
    tablet:
      width: 1024
      height: 768
    mobile:
      width: 375
      height: 812

  # Per-element thresholds (override default)
  element_thresholds:
    strategy_matrix: 0.3       # Tight threshold for precise grid rendering
    card_rendering: 0.5        # Standard threshold for card images
    navigation_bar: 0.3        # Tight threshold for consistent nav
    action_buttons: 0.5        # Standard threshold
    text_content: 0.8          # Looser threshold for text-heavy areas (font rendering varies)
    third_party_refs: 2.0      # Loose threshold for third-party reference screenshots

  # Ignore regions (dynamic content that changes between runs)
  ignore_regions:
    - selector: "[data-testid='rng-dice']"    # Random number changes each hand
    - selector: "[data-testid='timer']"        # Countdown value changes
    - selector: "[data-testid='score-value']"  # Score varies by session
    - selector: "[data-testid='hand-cards']"   # Dealt cards are random
    - selector: "[data-testid='power-credits']" # Credits remaining varies
```

### Playwright Example Setup

```typescript
import { test, expect } from '@playwright/test';

test.describe('Visual Regression Tests', () => {
  test('VR-001: Study Mode Strategy Tab', async ({ page }) => {
    await page.goto('/study');
    // Navigate to a specific spot for deterministic rendering
    await page.selectOption('[data-testid="format"]', 'cash-6max-100bb');
    await page.click('[data-testid="spot-co-vs-bb"]');

    await expect(page).toHaveScreenshot('study-mode-strategy.png', {
      maxDiffPixelRatio: 0.01,  // 1.0% threshold
      animations: 'disabled',
    });
  });
});
```

---

## Reference Screenshot Map

### Full-Page Screenshots

| Test ID | Screen/Component | Reference Screenshot | Viewport | Threshold | Notes |
|---------|-----------------|---------------------|----------|-----------|-------|
| VR-001 | Study Mode - Strategy Tab (Default) | `screenshots/website/ai-solver-default.avif` | Desktop | 1.0% | Primary marketing render; allow higher threshold for AVIF artifacts |
| VR-002 | Study Mode - Strategy Tab (Hover State) | `screenshots/website/ai-solver-hover.avif` | Desktop | 1.0% | Hover state with expanded detail |
| VR-003 | Study Mode - Library View | `screenshots/website/library-of-solutions-default.avif` | Desktop | 1.0% | CDN screenshot of solution browser |
| VR-004 | Study Mode - Library View (Hover) | `screenshots/website/library-of-solutions-hover.avif` | Desktop | 1.0% | Library hover state |
| VR-005 | Practice Mode - Trainer View | `screenshots/reviews/solvers-poker-trainer.png` | Desktop | 2.0% | Third-party screenshot, higher tolerance |
| VR-006 | Practice Mode - Trainer Detailed | `screenshots/reviews/h2n-trainer.webp` | Desktop | 2.0% | Third-party review screenshot |
| VR-007 | Practice Mode - Advanced Config | `screenshots/website/practice-advanced-default.avif` | Desktop | 1.0% | CDN marketing screenshot |
| VR-008 | Practice Mode - Custom Drills | `screenshots/website/practice-custom-drills-default.avif` | Desktop | 1.0% | CDN screenshot |
| VR-009 | Practice Mode - Instant Feedback | `screenshots/website/practice-instant-feedback-default.avif` | Desktop | 1.0% | Feedback display state |
| VR-010 | Practice Mode - Train Any Spot | `screenshots/website/practice-train-any-spot-default.avif` | Desktop | 1.0% | Spot selection interface |
| VR-011 | Analyze Mode - Hands Table | `screenshots/blog/analyzer2-hands-table.png` | Desktop | 1.5% | Blog screenshot of Analyzer 2.0 |
| VR-012 | Analyze Mode - Hand Analysis | `screenshots/blog/analyzer2-hand-analysis.png` | Desktop | 1.5% | Individual hand replay view |
| VR-013 | Analyze Mode - Position Breakdown | `screenshots/blog/analyzer2-position-breakdown.png` | Desktop | 1.5% | Position stats breakdown |
| VR-014 | Analyze Mode - Preflop Action | `screenshots/blog/analyzer2-preflop-action.png` | Desktop | 1.5% | Preflop action by pot type |
| VR-015 | Analyze Mode - SRP Report | `screenshots/blog/analyzer2-srp-report.png` | Desktop | 1.5% | SRP stats with 5 stat cards |
| VR-016 | Analyze Mode - Street Decisions | `screenshots/blog/analyzer2-street-decisions.png` | Desktop | 1.5% | Street-by-street breakdown |
| VR-017 | Analyze Mode - Hand Filtering | `screenshots/blog/redesigned-hand-filtering.png` | Desktop | 1.5% | Advanced filter interface with action sequence builder |
| VR-018 | Analyze Mode - Natural Language Filter | `screenshots/blog/redesigned-natural-language-filter.png` | Desktop | 1.5% | Autocomplete filter search |
| VR-019 | GTO Reports - Preflop Stats | `screenshots/blog/gto-reports-preflop-stats.png` | Desktop | 1.5% | VALUE vs GTO deviation bars |
| VR-020 | GTO Reports - Game Type Filtering | `screenshots/blog/gto-reports-filtering.png` | Desktop | 1.5% | Date range, deviation filters |
| VR-021 | GTO Reports - Stat Comparisons | `screenshots/blog/gto-reports-stat-comparisons.png` | Desktop | 1.5% | Game type selector (Cash/MTT/Spins/HuSng) |
| VR-022 | GTO Reports - Position Matrix | `screenshots/blog/gto-reports-position-matrix.png` | Desktop | 1.5% | 6x6 position-vs-position grid |
| VR-023 | GTO Reports - Defense Analysis | `screenshots/blog/gto-reports-defense-analysis.png` | Desktop | 1.5% | Fold vs 3BET defense view |
| VR-024 | GTO Reports - Postflop Actions | `screenshots/blog/gto-reports-postflop-actions.png` | Desktop | 1.5% | Postflop filter bar and actions |
| VR-025 | GTO Reports - Full Interface | `screenshots/blog/redesigned-preflop-mistakes.png` | Desktop | 1.5% | Three-column layout with preflop mistakes |
| VR-026 | Nodelocking - Main Interface | `screenshots/blog/nodelocking-interface.png` | Desktop | 1.5% | Horizontal game tree, 13x13 matrix |
| VR-027 | Nodelocking - Set Frequency | `screenshots/blog/nodelocking-set-frequency.png` | Desktop | 1.5% | Frequency bars with lock icons |
| VR-028 | Nodelocking - Strategy Visualization | `screenshots/blog/nodelocking-strategy-viz.png` | Desktop | 1.5% | Range matrix after nodelock applied |
| VR-029 | Nodelocking - Hand Class View | `screenshots/blog/redesigned-nodelocking-handclass.png` | Desktop | 1.5% | Detailed hand class filtering interface |
| VR-030 | Aggregated Reports - Chart View | `screenshots/blog/aggregated-reports-flop-cbet.png` | Desktop | 1.5% | Stacked bar chart (red/green) |
| VR-031 | Aggregated Reports - Table View | `screenshots/website/aggregated-reports.png` | Desktop | 1.0% | Dual-panel table with Flops/Strategy columns |
| VR-032 | Aggregated Reports - Custom Tab | `screenshots/blog/aggregated-reports-custom-tab.png` | Desktop | 1.5% | Custom reports tab with Name/Date/Positions/Model |
| VR-033 | Table Wizard - Overlay Config | `screenshots/blog/table-wizard-overlay.png` | Desktop | 1.5% | Left sidebar, five toggle categories, table preview |
| VR-034 | Table Wizard - Hero/Marketing | `screenshots/blog/table-wizard-hero.png` | Desktop | 2.0% | Marketing page, download button |
| VR-035 | Dashboard - Full View | `screenshots/reviews/h2n-dashboard.webp` | Desktop | 2.0% | 9-tile grid, trainer/analyzer stats panels |
| VR-036 | Main Study Interface (Third-Party) | `screenshots/reviews/h2n-main.webp` | Desktop | 2.0% | Nav bar, matrix, infobox, actions table |
| VR-037 | Solutions Library (Third-Party) | `screenshots/reviews/h2n-library.webp` | Desktop | 2.0% | Game tree, stack depths, spot navigation |
| VR-038 | Analyzed Hands (Third-Party) | `screenshots/reviews/h2n-analyzed-hands.webp` | Desktop | 2.0% | Hand history table, mistake icons |
| VR-039 | Settings/Preferences | `screenshots/reviews/h2n-settings.webp` | Desktop | 2.0% | Display options, theme config, hotkeys |
| VR-040 | Solver Main Page (Third-Party) | `screenshots/reviews/solvers-poker-main.png` | Desktop | 2.0% | Navigation bar, solver view, strategy matrix |
| VR-041 | 3-Way Solving Interface | `screenshots/blog/3way-solving-interface.png` | Desktop | 1.5% | Multi-player game tree, compatibility matrix |
| VR-042 | Redesigned Flop General View | `screenshots/blog/redesigned-flop-general-view.png` | Desktop | 1.5% | Updated flop analysis with "Reports: Flops" tab |
| VR-043 | Study Plans | `screenshots/website/study-plans-default.avif` | Desktop | 1.0% | Learning paths, structured content |
| VR-044 | PokerArena - Battle HU Poker | `screenshots/website/battle-heads-up-poker-default.avif` | Desktop | 1.0% | Arena game mode, HU matchmaking |
| VR-045 | PokerArena - Master Your Game | `screenshots/website/master-your-game-default.avif` | Desktop | 1.0% | Game mastery features |
| VR-046 | PokerArena - Leaderboard | `screenshots/website/prove-you-are-the-best-default.avif` | Desktop | 1.0% | Competitive rankings display |
| VR-047 | Customizable Bet Sliders | `screenshots/website/customizable-bet-sliders.avif` | Desktop | 1.0% | Bet sizing slider UI |
| VR-048 | Hotkeys Overlay | `screenshots/website/hotkeys-for-every-action-default.avif` | Desktop | 1.0% | Keyboard shortcut overlay |
| VR-049 | Analyzer 2.0 Feature Card | `screenshots/website/analyzer-2.0-default.avif` | Desktop | 1.0% | Analyzer feature marketing card |
| VR-050 | GTO Reports Feature Card | `screenshots/website/gto-reports-default.avif` | Desktop | 1.0% | Reports feature marketing card |

---

## Animated Interaction Tests

For GIF screenshots (animated demonstrations), extract key frames for comparison. These test multi-step interactions where intermediate states matter.

| Test ID | Animation | Reference GIF | Key Frames | Threshold | Description |
|---------|-----------|---------------|------------|-----------|-------------|
| VR-A01 | Nodelocking Strategy Painting | `screenshots/blog/nodelocking-set-strategy.gif` | Frame 1 (initial), Frame 10 (mid-paint), Frame 20 (complete) | 2.0% | Paint hands to assign actions on 13x13 matrix; verify color fills appear progressively |
| VR-A02 | Nodelocking Frequency Slider | `screenshots/blog/nodelocking-frequency-slider.gif` | Frame 1 (default frequencies), Frame 8 (slider mid-drag), Frame 15 (final position) | 2.0% | Adjust frequency slider; verify bars resize in real-time and percentages update |

### GIF Frame Extraction Configuration

```yaml
gif_extraction:
  tool: "ffmpeg"  # or ImageMagick's "convert"
  commands:
    # Extract specific frames
    extract_frame: "ffmpeg -i {input} -vf 'select=eq(n\\,{frame_num})' -vframes 1 {output}"
    # Extract all frames
    extract_all: "ffmpeg -i {input} {output_dir}/frame_%04d.png"

  comparison:
    # Compare extracted frames against reference frames
    threshold: 2.0  # Higher tolerance for animation artifacts
    ignore_timing: true  # Frame timing may vary
```

---

## Component-Level Visual Tests

Atomic components that appear across multiple screens. Each component test verifies consistent rendering regardless of the parent screen context.

### Hand Matrix (13x13 Grid)

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C01 | Default strategy colors | Study, Practice sidebar, Nodelocking | `screenshots/blog/nodelocking-interface.png` | Grid alignment, cell sizing (all 169 cells equal), label positions (centered), color fills (red/green/blue), border lines between cells | 0.5% |
| VR-C02 | EV heatmap mode | Study Mode | `screenshots/website/ai-solver-default.avif` | Green-white-red gradient rendering, cell value text overlay, legend display | 1.0% |
| VR-C03 | Mixed strategy cells | Study Mode | `screenshots/reviews/h2n-main.webp` | Proportional color splits within cells, no gap between color segments, segment boundaries aligned | 1.0% |
| VR-C04 | Filtered/dimmed state | Study Mode with filters | None (generate baseline) | Dimmed cells at reduced opacity, filtered cells at full intensity, visual separation between states | 1.0% |
| VR-C05 | Blocked/dead cells | Study Mode postflop | None (generate baseline) | Black/dark gray cells for impossible hands, non-interactive appearance, consistent with active cells in size | 1.0% |

### Action Buttons

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C06 | Default bet buttons | Practice, PokerArena | `screenshots/reviews/h2n-trainer.webp` | Button sizing (consistent height/width), color coding (green CHECK, graduated red BETs, darkest red ALLIN), text labels (action name + amount), spacing between buttons | 1.0% |
| VR-C07 | Hover state | Practice Mode | None (generate baseline) | Hover highlight (lighter shade or border), cursor change to pointer | 1.0% |
| VR-C08 | Pressed/active state | Practice Mode | None (generate baseline) | Darker shade or depression effect on press, immediate visual feedback | 1.0% |
| VR-C09 | Simple difficulty buttons | Practice Mode | None (generate baseline) | Only Bet/Check/Fold shown, full-width layout, simplified labels | 1.5% |
| VR-C10 | Grouped difficulty buttons | Practice Mode | None (generate baseline) | Small/Medium/Large/Overbet groups, bucket labels instead of exact sizes | 1.5% |

### Card Rendering

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C11 | Face-up cards (hero) | Practice, Analyze, PokerArena | `screenshots/reviews/solvers-poker-trainer.png` | Rank text legibility, suit symbol rendering (red hearts/diamonds, black/blue spades/clubs), card border and shadow, card dimensions | 0.5% |
| VR-C12 | Face-down cards (opponent) | Practice, PokerArena | `screenshots/reviews/solvers-poker-trainer.png` | Card back design matches selected theme, consistent size with face-up cards, no information leakage | 0.5% |
| VR-C13 | Community cards (board) | Practice, Analyze, Study | `screenshots/reviews/solvers-poker-trainer.png` | Cards displayed in a horizontal row, consistent spacing, same size as hole cards, empty slots show placeholder | 0.5% |
| VR-C14 | Small card icons (table/matrix) | Analyze Mode hands table, Study spot selector | `screenshots/blog/analyzer2-hands-table.png` | Miniature card rendering legible at small sizes, suit colors distinguishable, inline with table text | 1.0% |

### Filter Bar

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C15 | Active filter badges | Analyze, Study Mode | `screenshots/blog/redesigned-hand-filtering.png` | Badge styling (pill shape, colored background), "x" dismiss button, filter label text, spacing between badges | 1.0% |
| VR-C16 | Filter category tabs | Analyze Mode | `screenshots/blog/redesigned-hand-filtering.png` | Tab labels (Filters, Streets Actions, Hands Details, Statistics and Results, Hole Cards, Game Type, Other), active tab highlight, count badges | 1.0% |
| VR-C17 | Autocomplete dropdown | Analyze Mode natural language filter | `screenshots/blog/redesigned-natural-language-filter.png` | Dropdown positioning below input, suggestion text formatting, hover highlight on suggestions, scroll behavior | 1.5% |

### Navigation Bar

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C18 | Default state | All screens | `screenshots/reviews/h2n-main.webp` | Logo position (left), link labels (Play, Study, Practice, Analyze), right-side icons (Upload, settings, avatar), background color, height | 0.5% |
| VR-C19 | Active tab state | All screens | `screenshots/reviews/h2n-dashboard.webp` | Active tab highlighted with teal/cyan accent or underline, other tabs muted/default color | 0.5% |
| VR-C20 | "Go Elite" upsell badge | Dashboard (non-Elite user) | `screenshots/reviews/h2n-dashboard.webp` | Green badge next to logo, badge text readable, badge does not overlap other nav elements | 1.5% |

### Stat Cards

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C21 | Stats card row (5 cards) | Analyze Mode stats view | `screenshots/blog/analyzer2-srp-report.png` | Horizontal row layout, equal card widths, rounded corners, hero number (large bold text), small label below, sparkline in GTO Score card | 1.0% |
| VR-C22 | Score Breakdown panel | Analyze Mode stats view | `screenshots/blog/analyzer2-srp-report.png` | Colored horizontal bars (Perfect green, Good teal, Inaccurate yellow, Wrong/Mistake red, Blunder red), count labels, proportion accuracy | 1.0% |

### Deviation Bars (GTO Reports)

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C23 | VALUE vs GTO deviation bar | GTO Reports | `screenshots/blog/gto-reports-preflop-stats.png` | Center line position (GTO reference), red extension right (over GTO), blue extension left (under GTO), green checkmark when matching, numerical labels | 1.0% |
| VR-C24 | Position-vs-position matrix cell | GTO Reports | `screenshots/blog/gto-reports-position-matrix.png` | Two-number cell (VALUE above, GTO below), color coding (green close, red deviating), warning icons for significant deviation | 1.0% |

### Poker Table

| Test ID | Component State | Screens Used | Reference Screenshot | Properties to Verify | Threshold |
|---------|----------------|-------------|---------------------|---------------------|-----------|
| VR-C25 | Oval table with positions | Practice, PokerArena | `screenshots/reviews/solvers-poker-trainer.png` | Oval shape, position labels around rim (UTG through BB), stack numbers next to positions, pot display above center cards | 1.5% |
| VR-C26 | Table overlay (Table Wizard) | Table Wizard | `screenshots/blog/table-wizard-overlay.png` | Position labels, stack/SPR/PO/EFF stats per position, colored action indicators (red for raises, green for checks), overlay transparency | 2.0% |

---

## Responsive Visual Tests

Tests verifying visual appearance at different viewport sizes. These generate their own baselines on first run.

| Test ID | Screen | Viewport | Baseline Source | Key Verifications | Threshold |
|---------|--------|----------|----------------|-------------------|-----------|
| VR-R01 | Dashboard | 1920x1080 | Generate baseline | 3x3 grid layout, stats panels side-by-side, full nav bar | 0.5% |
| VR-R02 | Dashboard | 1024x768 | Generate baseline | Grid may reflow to 2 columns, stats may stack, nav intact | 1.0% |
| VR-R03 | Dashboard | 375x812 | Generate baseline | Single column, hamburger nav, touch-friendly tiles | 1.5% |
| VR-R04 | Study Mode | 1920x1080 | Generate baseline | Side-by-side infobox + matrix, full filter bar | 0.5% |
| VR-R05 | Study Mode | 1024x768 | Generate baseline | Panels may stack, matrix may be smaller, all visible | 1.0% |
| VR-R06 | Study Mode | 375x812 | Generate baseline | Single column, scrollable matrix, collapsed sidebar | 1.5% |
| VR-R07 | Practice Mode | 1920x1080 | Generate baseline | Full table, action buttons row, sidebars visible | 0.5% |
| VR-R08 | Practice Mode | 375x812 | Generate baseline | Table scales down, buttons stack or scroll, minimal sidebar | 1.5% |
| VR-R09 | Analyze Mode | 1920x1080 | Generate baseline | Full table with all columns, filter bar, stats cards | 0.5% |
| VR-R10 | Analyze Mode | 375x812 | Generate baseline | Horizontal scroll for table, stacked stats, collapsed filters | 1.5% |

---

## Theme Variation Tests

Tests verifying visual consistency when switching between themes. Each test captures the same screen in different themes and compares structural consistency (layout, sizing) while allowing color differences.

| Test ID | Screen | Theme | Baseline | Key Verifications | Threshold |
|---------|--------|-------|----------|-------------------|-----------|
| VR-T01 | Study Mode Strategy | Default Dark | Generate baseline | Layout structure preserved, contrast sufficient | 0.5% |
| VR-T02 | Study Mode Strategy | Light Theme | Compare layout structure to VR-T01 | Same layout, inverted colors, all text readable | 3.0% (structural comparison only) |
| VR-T03 | Study Mode Strategy | Color-blind friendly | Compare layout structure to VR-T01 | Same layout, accessible color palette, action colors distinguishable | 3.0% (structural comparison only) |
| VR-T04 | Practice Mode | Default Dark | Generate baseline | Action button colors, card rendering, table colors | 0.5% |
| VR-T05 | Practice Mode | Custom Theme | Compare layout structure to VR-T04 | Same layout, custom colors applied, all interactive elements visible | 3.0% (structural comparison only) |

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Visual Regression Tests
on:
  pull_request:
    branches: [main]

jobs:
  visual-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies
        run: npm ci

      - name: Install Playwright browsers
        run: npx playwright install --with-deps chromium

      - name: Run visual regression tests
        run: npx playwright test tests/visual-regression/
        env:
          CI: true

      - name: Upload diff artifacts
        if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: visual-diff-report
          path: test-results/
          retention-days: 30

      - name: Comment PR with diff summary
        if: failure()
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const diffReport = fs.readFileSync('test-results/diff-summary.txt', 'utf8');
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: `## Visual Regression Test Results\n\n${diffReport}`
            });
```

### Baseline Management

```yaml
baseline_management:
  storage: "git-lfs"  # Store baselines in Git LFS to avoid bloating repo
  update_command: "npx playwright test --update-snapshots"
  approval_workflow:
    - "Visual diff exceeding threshold requires manual review"
    - "PR must include before/after screenshots in description"
    - "At least one reviewer must approve visual changes"

  branching:
    main: "Production baselines"
    develop: "Development baselines (may differ from main)"
    feature/*: "Use develop baselines, generate diffs against develop"
```

---

## Test Execution Order

Recommended execution priority for visual regression tests:

1. **Component-level tests (VR-C01 through VR-C26):** Run first to catch atomic rendering issues
2. **Full-page tests (VR-001 through VR-050):** Run second to verify page compositions
3. **Responsive tests (VR-R01 through VR-R10):** Run third for viewport variations
4. **Animation tests (VR-A01, VR-A02):** Run last (slowest, least critical)
5. **Theme tests (VR-T01 through VR-T05):** Run in separate CI job (structural comparison)

---

## Test Summary

| Category | ID Range | Count |
|----------|----------|-------|
| Full-Page Screenshots | VR-001 to VR-050 | 50 |
| Animated Interaction Tests | VR-A01 to VR-A02 | 2 |
| Component-Level Tests | VR-C01 to VR-C26 | 26 |
| Responsive Visual Tests | VR-R01 to VR-R10 | 10 |
| Theme Variation Tests | VR-T01 to VR-T05 | 5 |
| **Total** | | **93** |

---

## Appendix: Screenshot Source Quality Tiers

Reference screenshots come from three sources with different quality and reliability characteristics. Thresholds are adjusted accordingly.

| Source | Directory | Format | Quality | Default Threshold | Notes |
|--------|-----------|--------|---------|-------------------|-------|
| GTO Wizard CDN | `screenshots/website/` | AVIF | Professional, consistent | 1.0% | Marketing renders, may include visual polish not in actual app |
| GTO Wizard Blog | `screenshots/blog/` | PNG, GIF | Good, authentic | 1.5% | Actual product screenshots from feature announcements |
| Third-Party Reviews | `screenshots/reviews/` | PNG, WebP | Variable, authentic | 2.0% | Independent captures, may include browser chrome or older versions |

---

## Appendix: Reference Screenshot Inventory

Complete mapping of all 118 reference screenshots to their visual regression test associations.

### Website Screenshots (60 files, AVIF format)

| Screenshot | Visual Test(s) | Component Test(s) |
|------------|---------------|-------------------|
| `ai-solver-default.avif` | VR-001 | VR-C02 |
| `ai-solver-hover.avif` | VR-002 | -- |
| `library-of-solutions-default.avif` | VR-003 | -- |
| `library-of-solutions-hover.avif` | VR-004 | -- |
| `practice-advanced-default.avif` | VR-007 | -- |
| `practice-custom-drills-default.avif` | VR-008 | -- |
| `practice-instant-feedback-default.avif` | VR-009 | -- |
| `practice-train-any-spot-default.avif` | VR-010 | -- |
| `aggregated-reports.png` | VR-031 | -- |
| `aggregated-reports-default.avif` | -- | -- |
| `analyzer-2.0-default.avif` | VR-049 | -- |
| `gto-reports-default.avif` | VR-050 | -- |
| `study-plans-default.avif` | VR-043 | -- |
| `battle-heads-up-poker-default.avif` | VR-044 | VR-C25 |
| `master-your-game-default.avif` | VR-045 | -- |
| `prove-you-are-the-best-default.avif` | VR-046 | -- |
| `customizable-bet-sliders.avif` | VR-047 | -- |
| `hotkeys-for-every-action-default.avif` | VR-048 | -- |
| `streamlined-table-layouts.avif` | -- | VR-C25 |

### Blog Screenshots (43 files, PNG + GIF)

| Screenshot | Visual Test(s) | Component Test(s) |
|------------|---------------|-------------------|
| `analyzer2-hands-table.png` | VR-011 | VR-C14 |
| `analyzer2-hand-analysis.png` | VR-012 | -- |
| `analyzer2-position-breakdown.png` | VR-013 | -- |
| `analyzer2-preflop-action.png` | VR-014 | -- |
| `analyzer2-srp-report.png` | VR-015 | VR-C21, VR-C22 |
| `analyzer2-street-decisions.png` | VR-016 | -- |
| `redesigned-hand-filtering.png` | VR-017 | VR-C15, VR-C16 |
| `redesigned-natural-language-filter.png` | VR-018 | VR-C17 |
| `gto-reports-preflop-stats.png` | VR-019 | VR-C23 |
| `gto-reports-filtering.png` | VR-020 | -- |
| `gto-reports-stat-comparisons.png` | VR-021 | -- |
| `gto-reports-position-matrix.png` | VR-022 | VR-C24 |
| `gto-reports-defense-analysis.png` | VR-023 | -- |
| `gto-reports-postflop-actions.png` | VR-024 | -- |
| `redesigned-preflop-mistakes.png` | VR-025 | -- |
| `nodelocking-interface.png` | VR-026 | VR-C01 |
| `nodelocking-set-frequency.png` | VR-027 | -- |
| `nodelocking-strategy-viz.png` | VR-028 | -- |
| `redesigned-nodelocking-handclass.png` | VR-029 | -- |
| `aggregated-reports-flop-cbet.png` | VR-030 | -- |
| `aggregated-reports-custom-tab.png` | VR-032 | -- |
| `table-wizard-overlay.png` | VR-033 | VR-C26 |
| `table-wizard-hero.png` | VR-034 | -- |
| `3way-solving-interface.png` | VR-041 | -- |
| `redesigned-flop-general-view.png` | VR-042 | -- |
| `nodelocking-set-strategy.gif` | VR-A01 | -- |
| `nodelocking-frequency-slider.gif` | VR-A02 | -- |

### Review Screenshots (15 files, PNG + WebP)

| Screenshot | Visual Test(s) | Component Test(s) |
|------------|---------------|-------------------|
| `solvers-poker-trainer.png` | VR-005 | VR-C06, VR-C11, VR-C12, VR-C13, VR-C25 |
| `h2n-trainer.webp` | VR-006 | VR-C06 |
| `h2n-dashboard.webp` | VR-035 | VR-C19, VR-C20 |
| `h2n-main.webp` | VR-036 | VR-C03, VR-C18 |
| `h2n-library.webp` | VR-037 | -- |
| `h2n-analyzed-hands.webp` | VR-038 | -- |
| `h2n-settings.webp` | VR-039 | -- |
| `solvers-poker-main.png` | VR-040 | -- |
| `h2n-practice.webp` | -- | -- |
| `h2n-game-types.webp` | -- | -- |
| `h2n-login.webp` | -- | -- |
| `solvers-poker-study-plan.png` | -- | -- |
| `solvers-poker-pricing.png` | -- | -- |
| `h2n-pricing.png` | -- | -- |
| `h2n-discount.png` | -- | -- |

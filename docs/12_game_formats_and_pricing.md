# GTO Wizard Game Formats & Pricing

This document covers all supported game formats, solution configurations, subscription tiers, pricing details, and platform availability for GTO Wizard.

---

## 1. Game Formats Overview

GTO Wizard supports five distinct game format categories, each with its own solution library and subscription options.

### 1.1 Cash Games - No-Limit Hold'em

Cash games are the primary format with the largest solution library. GTO Wizard offers solutions across multiple table sizes and configurations.

**Table Sizes:**

| Format     | Players | Description                              |
|------------|---------|------------------------------------------|
| Heads-Up   | 2       | Two-player cash game                     |
| 6-Max      | 6       | Six-handed (most popular online format)  |
| 9-Max      | 9       | Full-ring cash game                      |

**Solution Complexity Tiers:**

| Tier         | Bet Sizings | Accuracy   | Coverage          | Description                       |
|--------------|-------------|------------|-------------------|-----------------------------------|
| Simplified   | 1-2         | ~0.3% pot  | Preflop to River  | Beginner-friendly, fewer options  |
| Basic        | 4           | ~0.3% pot  | Preflop to River  | Standard study material           |
| Complex      | 8-19        | ~0.3% pot  | Preflop to River  | Advanced with more bet sizings    |
| Community    | 4           | ~0.3% pot  | Preflop to River  | Community-voted sizing selections |

**Rake Structures:**
Solutions are pre-solved for standard rake configurations corresponding to common stake levels:
- NL50 rake structure (micro/small stakes)
- NL500 rake structure (mid/high stakes)
- Additional rake structures available at higher tiers

**Stack Depths:**
Standard 100bb stacks with additional stack depth options available for different game types and scenarios.

### 1.2 Straddle + Ante Games

A specialized cash game format accounting for straddle and ante structures commonly found in live poker and some online games.

| Format     | Players | Description                              |
|------------|---------|------------------------------------------|
| 6-Max      | 6       | Six-handed with straddle and ante        |
| 8-Max      | 8       | Eight-handed with straddle and ante      |

These solutions account for the modified pot dynamics created by straddles and antes, which significantly alter preflop and postflop strategies compared to standard cash games.

### 1.3 Multi-Table Tournaments (MTT)

MTT solutions account for ICM (Independent Chip Model) pressure and varying stack depths encountered during tournament play.

**Configurations:**

| Format     | Stack Depth | Sizings | Accuracy   | Coverage             |
|------------|-------------|---------|------------|----------------------|
| MTT 6-Max  | 20bb        | 3-8     | 0.3% pot   | Preflop to River     |
| MTT 6-Max  | 25bb        | 3-8     | 0.3% pot   | Preflop to River     |
| MTT 8-Max  | 30bb        | 3-8     | 0.3% pot   | Preflop Only         |
| MTT 8-Max  | 35bb        | 3-8     | 0.3% pot   | Preflop Only         |

**ICM Features:**
- 50,000+ ICM final table simulations
- Various payout structures
- Bubble factor considerations
- Stack-size-dependent strategy adjustments
- PKO (Progressive Knockout) bounty calculations (under development)

**MTT Stages Covered:**
- Early stage (deep stacks, 40bb+)
- Middle stage (20-40bb)
- Late stage / bubble (15-25bb)
- Final table with ICM pressure
- Short-stack push/fold scenarios (sub-15bb)

### 1.4 Spin & Go (Spin Format)

Spin & Go solutions cover the popular three-handed hyper-turbo tournament format with varying prize multipliers.

| Solution Type | Sizings | Accuracy   | Coverage             |
|---------------|---------|------------|----------------------|
| General       | 4-8     | 0.25% pot  | Preflop to River     |
| Complex       | 12-19   | 0.3% pot   | Preflop to River     |

**Stack Depths:** Multiple starting stack depths reflecting common Spin & Go formats from 25bb down to 10bb and below.

**Special Considerations:**
- Winner-take-all prize structure affects ICM calculations
- Three-handed dynamics with different position strategies
- Hyper-turbo blind structures with rapid escalation

### 1.5 Heads-Up Sit & Go (HU SNG)

Dedicated solutions for heads-up sit-and-go tournaments.

| Solution Type | Sizings | Accuracy   | Coverage             |
|---------------|---------|------------|----------------------|
| General       | 4-8     | 0.25% pot  | Preflop to River     |
| Complex       | 12-19   | 0.3% pot   | Preflop to River     |

**Stack Depths:** Solutions from deep-stacked play (75bb+) down to short-stack situations (sub-15bb), covering the full progression of a HU SNG match.

### 1.6 3-Way Solving (Recent Addition)

As of August 2025, GTO Wizard introduced 3-way postflop solving, extending beyond traditional heads-up postflop analysis:

- Three-player postflop scenarios
- Multiway preflop solving supporting up to 9 players (March 2026)
- Significantly more complex game trees requiring advanced solving techniques

---

## 2. Solution Library Statistics

| Metric                   | Value                    |
|--------------------------|--------------------------|
| Total presolved spots    | 10,000,000+              |
| Board textures analyzed  | 1,755 unique flops       |
| Accuracy range           | 0.1% - 0.3% of pot      |
| Average Nash Distance    | 0.21% dEV (GTO Wizard AI)|
| Formats covered          | 5 (Cash, MTT, Spin, HU SNG, Straddle+Ante) |

---

## 3. Pricing Tiers

GTO Wizard operates on a subscription model with a free tier and three paid tiers. Subscriptions are per-format (e.g., Cash, MTT, Spin) unless otherwise bundled.

### 3.1 Free Tier

The free tier provides limited but meaningful access for evaluation purposes.

| Feature                  | Free Limit                            |
|--------------------------|---------------------------------------|
| Postflop study spots     | 1 per day                             |
| Trainer hands            | 10 per day                            |
| Hand history analysis    | 5 hands per month                     |
| Preflop solutions        | Basic access                          |
| GTO Wizard AI            | Not available                         |
| Nodelocking              | Not available                         |
| GTO Reports              | Not available                         |
| Aggregated reports       | Not available                         |
| Coaching                 | Not available                         |
| PokerArena               | Free to play (limited analysis depth) |

### 3.2 Starter Plan

| Detail                   | Specification                         |
|--------------------------|---------------------------------------|
| Monthly price            | $39/month                             |
| Annual price             | Discounted (exact % varies)           |
| HU SNG monthly price     | $26/month                             |

**Included Features:**
- Full study mode access for selected format
- Simplified and Basic solution libraries
- Preflop and postflop solutions
- Study plans and structured learning content
- Limited trainer access
- Basic hand analysis capability
- Access to community solutions

**Not Included:**
- GTO Wizard AI (custom solving)
- Complex solution libraries
- Nodelocking
- GTO Reports
- Advanced coaching sessions
- Full trainer customization

### 3.3 Premium Plan

| Detail                   | Specification                         |
|--------------------------|---------------------------------------|
| Monthly price            | $69/month                             |
| Annual price             | Discounted (approx 30% off in some promotions) |
| HU SNG monthly price     | $49/month                             |

**Included Features (everything in Starter plus):**
- Full GTO Trainer with all customization options
- Multi-table trainer (up to 4 tables)
- Puzzles and advanced drills
- Enhanced hand history analysis (higher upload limits)
- Complex solution libraries
- Aggregated reports
- Live coaching sessions
- Priority support
- Advanced filtering and tagging

**Not Included:**
- GTO Wizard AI (custom solving)
- Nodelocking
- Full GTO Reports

### 3.4 Elite Plan

| Detail                   | Specification                         |
|--------------------------|---------------------------------------|
| Monthly price            | $129/month                            |
| Annual price             | Discounted                            |

**Included Features (everything in Premium plus):**
- GTO Wizard AI (custom solving with adjustable parameters)
- Nodelocking 2.0
- Full GTO Reports
- Power credits for custom solves
- Custom player profiles for opponent modeling
- 3-Way solving access
- Multiway preflop solving (up to 9 players)
- Maximum hand history upload limits
- All solution complexity tiers
- All game formats accessible

### 3.5 Pricing Summary Table

| Feature                        | Free     | Starter ($39/mo) | Premium ($69/mo) | Elite ($129/mo) |
|--------------------------------|----------|-------------------|-------------------|-----------------|
| Preflop Solutions              | Basic    | Full              | Full              | Full            |
| Postflop Solutions             | 1/day    | Full              | Full              | Full            |
| Simplified Solutions           | Limited  | Yes               | Yes               | Yes             |
| Basic Solutions                | Limited  | Yes               | Yes               | Yes             |
| Complex Solutions              | No       | No                | Yes               | Yes             |
| GTO Trainer                    | 10/day   | Limited           | Full              | Full            |
| Multi-Table Trainer            | No       | No                | Yes (4 tables)    | Yes (4 tables)  |
| Hand Analysis                  | 5/month  | Moderate          | High              | Maximum         |
| Aggregated Reports             | No       | No                | Yes               | Yes             |
| GTO Reports                    | No       | No                | No                | Yes             |
| GTO Wizard AI                  | No       | No                | No                | Yes             |
| Nodelocking                    | No       | No                | No                | Yes             |
| Custom Profiles                | No       | No                | No                | Yes             |
| 3-Way Solving                  | No       | No                | No                | Yes             |
| Multiway Preflop               | No       | No                | No                | Yes             |
| Power Credits                  | 0        | 0                 | 0                 | Monthly allotment|
| Coaching / Live Streams        | No       | No                | Yes               | Yes             |
| Study Plans                    | Basic    | Full              | Full              | Full            |
| Themes / Customization         | Basic    | Full              | Full              | Full            |
| Languages (17 supported)       | Yes      | Yes               | Yes               | Yes             |

### 3.6 Format-Specific Pricing

Subscriptions are purchased per game format. Users who play multiple formats need separate subscriptions or a bundle.

| Format            | Starter/mo | Premium/mo | Elite/mo |
|-------------------|------------|------------|----------|
| Cash 6-Max        | $39        | $69        | $129     |
| Cash 9-Max        | $39        | $69        | $129     |
| Cash HU           | $39        | $69        | $129     |
| MTT               | $39        | $69        | $129     |
| Spin & Go         | $39        | $69        | $129     |
| HU SNG            | $26        | $49        | $129     |
| Straddle + Ante   | $39        | $69        | $129     |

### 3.7 Discounts and Promotions

| Discount Type              | Detail                                    |
|----------------------------|-------------------------------------------|
| Annual subscription        | Significant discount vs monthly           |
| Affiliate/referral links   | 10% off (available through partner sites) |
| Early bird pricing         | Occasionally offered for new features     |
| Bundle deals               | Multi-format bundles available             |

### 3.8 Power Credits (Elite Tier)

Power credits are the currency for GTO Wizard AI custom solves:

- Monthly allocation included with Elite subscription
- Each custom solve consumes credits based on complexity (tree size, number of sizings, accuracy target)
- Credits reset monthly
- Cannot be purchased separately (tied to Elite subscription)
- Usage tracked in the Settings panel

---

## 4. Platform Availability

### 4.1 Web Application (Primary Platform)

| Detail                 | Specification                           |
|------------------------|-----------------------------------------|
| URL                    | app.gtowizard.com                       |
| Type                   | Progressive Web App (PWA)               |
| Browser support        | Modern browsers (Chrome, Firefox, Safari, Edge) |
| Installation           | None required; installable as PWA       |
| Offline support        | Limited (cloud-dependent)               |
| Resolution support     | Optimized for 2K and 4K monitors        |
| Mobile browser         | Fully responsive, installable as PWA    |

The web application is the primary and most feature-complete platform. All study, practice, analyze, and AI features are accessed through the browser.

### 4.2 Mobile Access

| Platform   | Type            | App Name                        | Features                    |
|------------|-----------------|----------------------------------|-----------------------------|
| iOS        | Native App      | GTO Preflop Wizard Poker AI     | Preflop training focused    |
| iOS        | Native App      | PokerArena by GTO Wizard        | Competitive HU poker        |
| iOS        | PWA             | GTO Wizard (browser)            | Full feature access         |
| Android    | PWA             | GTO Wizard (browser)            | Full feature access         |
| Android    | Browser         | PokerArena (browser)            | Competitive HU poker        |

**Mobile Notes:**
- The main GTO Wizard application is browser-based (PWA), not a native app
- PokerArena has a dedicated iOS app with crossplay support
- GTO Preflop Wizard is a separate focused app for preflop study on iOS
- Full feature parity is available through mobile browser PWA
- Installable to home screen for app-like experience

### 4.3 Desktop Application

| Platform   | Application      | Purpose                                     |
|------------|-----------------|----------------------------------------------|
| Windows    | Table Wizard     | Multi-table management with real-time overlays|

**Table Wizard** is the only native desktop application, designed specifically for:
- Managing multiple online poker tables simultaneously
- Real-time bet/raise/call overlays during play
- Custom hotkeys and bet sliders
- Automatic hand history capture and upload to Analyzer 2.0
- Seamless integration with the web-based analysis tools

### 4.4 API Access

GTO Wizard does not offer a public API for third-party integrations. However, Table Wizard provides automated integration between live play and the analysis platform.

### 4.5 Broadcasting Integration

GTO Wizard has partnered with GGPoker for live broadcast analysis:
- Real-time on-screen GTO analysis overlays for poker streams
- Human-readable bet size labels (e.g., "Bet Large" instead of "Bet 2.7M")
- Used on GGMillion$ final table streams
- Tracks player adherence to GTO strategy in real-time

---

## 5. Supported Poker Sites (for Hand History Analysis)

The Analyzer supports hand history imports from the following online poker platforms:

| Poker Site          | Format Support | Notes                         |
|---------------------|----------------|-------------------------------|
| PokerStars          | Full           | Most common format            |
| GGPoker             | Full           | Partnership integration       |
| 888Poker            | Full           | Standard HH format            |
| PartyPoker          | Full           | Standard HH format            |
| Winamax             | Full           | European-focused site         |
| WPN (Winning Poker Network) | Full  | Americas Cardroom, etc.       |
| iPoker Network      | Full           | Multiple skins supported      |
| Coinpoker           | Full           | Crypto poker platform         |
| Chico Poker Network | Full           | BetOnline, etc.               |
| Bovada / Ignition   | Supported      | Anonymous hand histories      |

Upload methods:
- Manual file upload (single file or folder)
- Table Wizard automatic upload (real-time during play)
- Supported formats: .txt, .xml, folder batch upload

---

## 6. Content & Learning Resources

Beyond the solver and training tools, GTO Wizard provides educational content included across subscription tiers:

| Content Type        | Description                                | Access        |
|---------------------|--------------------------------------------|---------------|
| Strategy Articles   | Written guides on GTO concepts             | All tiers     |
| Study Plans         | Structured learning paths by format/level  | Starter+      |
| YouTube Videos      | Video tutorials and strategy content       | Free          |
| Live Coaching       | Streamed coaching sessions                 | Premium+      |
| Daily GTO Quizzes   | Quick daily practice problems              | Free          |
| Blog (What's New)   | Feature updates and patch notes            | Free          |
| Help Center         | Documentation for all features             | Free          |
| Discord Community   | Community support and discussion           | Free          |

---

## 7. Recent Feature Timeline

| Date          | Feature / Update                                             |
|---------------|--------------------------------------------------------------|
| March 2026    | Multiway preflop solving (up to 9 players)                  |
| November 2025 | Custom player profiles for opponent modeling                |
| September 2025| Analyzer 2.0 redesign + upgraded GTO Reports               |
| August 2025   | 3-Way postflop solving, Nodelocking 2.0, 50k+ ICM FT sims |
| July 2025     | PokerArena launch (competitive HU poker)                    |
| 2025          | Table Wizard launch (multi-table management)                |
| 2024          | Live Cash solutions, Straddle+Ante format                   |
| 2024          | GTO Wizard AI launch (custom solving)                       |
| 2023          | MTT postflop solutions, Spin & Go, HU SNG                  |
| 2023          | Hand History Analyzer launch                                |
| 2022          | Simplified solutions, new interface redesign                |

---

## 8. Cross-References

- For detailed UI/UX documentation of each screen, see [11_ui_ux_screens.md](./11_ui_ux_screens.md)
- For competitive analysis against PioSolver, GTO+, and others, see [13_competitive_landscape.md](./13_competitive_landscape.md)
- For solver engine architecture and accuracy details, see solver architecture documentation
- For detailed feature documentation (Study, Practice, Analyze, Range Builder), see feature documentation

---

## Sources

- https://gtowizard.com
- https://help.gtowizard.com/subscription/
- https://solvers.poker/review/gtowizard/
- https://hand2noteguide.com/gto-wizard-price/
- https://hand2noteguide.com/gto-wizard-review/
- https://blog.gtowizard.com/introducing-mtt-spin-go-and-hu-sng-postflop-solutions-for-free/
- https://blog.gtowizard.com/live-cash-solutions-and-4000-new-scenarios-for-cash-mtt-formats/
- https://blog.gtowizard.com/whats-new-in-gto-wizard/
- https://blog.gtowizard.com/suported-sites-and-formats-for-hh-analyzer/
- https://blog.gtowizard.com/introducing_table_wizard_the_ultimate_table_management_software/
- https://blog.gtowizard.com/enter_the_poker_arena_a_new_era_of_competitive_poker_has_begun/
- https://apps.apple.com/us/app/gto-preflop-wizard-poker-ai/id6590603158
- https://apps.apple.com/us/app/pokerarena-by-gto-wizard/id6742486598
- https://www.hudstore.poker/5-best-gto-poker-solvers
- https://www.gipsyteam.com/news/17-10-2025/most-useful-poker-software-2025

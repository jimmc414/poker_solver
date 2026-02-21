# GTO Wizard Practice Mode

## Overview

Practice Mode is GTO Wizard's interactive training environment that transforms theoretical GTO knowledge into practical decision-making skill. Players compete against the solver's equilibrium strategy in realistic poker scenarios, receiving immediate feedback and detailed performance analytics. The system supports multiple training formats, configurable difficulty levels, multitabling, and comprehensive session tracking.

Practice Mode is accessed via the **Practice** tab in the main navigation bar and is available from anywhere in the application. It serves as the bridge between passive study (reading solver outputs) and active skill development (making decisions under pressure).

---

## Training Modes

Practice Mode offers three distinct game modes, each targeting different aspects of poker skill development:

### Full Hand Mode

Full Hand mode simulates complete poker hands from preflop through river. The player makes every decision at every street, with the solver playing the opponent's strategy according to GTO frequencies.

- **Best for**: Developing complete hand-playing ability and understanding street-by-street strategy evolution
- **Decision points**: All streets (preflop, flop, turn, river)
- **Opponent behavior**: GTO-optimal play at each decision point

### Street Mode

Street mode isolates a single street (flop, turn, or river) for focused drilling. The hand starts at the chosen street with appropriate ranges and stack depths already established.

- **Best for**: Targeted improvement on a specific street where leaks exist
- **Decision points**: One specific street only
- **Skips**: All other streets are auto-played or pre-dealt

### Spot Mode

Spot mode repeatedly drills a single decision point. The player faces the exact same type of decision over and over with different hands from the range.

- **Best for**: Mastering specific scenarios (e.g., facing a 3-bet from the big blind, c-betting on paired boards)
- **Decision points**: One specific node in the game tree
- **Repetition**: High-frequency repetition of the same decision type

```
+----------------------------------------------------------+
|  PRACTICE MODE - GAME MODE SELECTOR                      |
|                                                          |
|  +----------------+  +----------------+  +-----------+   |
|  |   FULL HAND    |  |    STREET      |  |   SPOT    |   |
|  |                |  |                |  |           |   |
|  | Preflop-River  |  | Single Street  |  | One Node  |   |
|  | Complete hands |  | Focused drill  |  | Repeated  |   |
|  +----------------+  +----------------+  +-----------+   |
|                                                          |
+----------------------------------------------------------+
```

---

## Difficulty Tiers

The trainer offers three difficulty levels that control how bet sizing decisions are presented:

| Difficulty  | Description                                                  | Bet Size Options         |
|-------------|--------------------------------------------------------------|--------------------------|
| **Simple**  | Choose the correct action type only (bet/raise, check/call, fold) | Action categories only |
| **Grouped** | Choose the correct bet/raise grouping (small, medium, large, overbet) | Grouped size buckets |
| **Standard**| Select the exact correct bet or raise size from all options  | All individual sizes     |

In **Grouped** and **Simple** modes, the trainer automatically selects the highest-frequency action within the selected grouping. This means players can focus on broad strategic patterns before drilling precise sizing decisions.

Players can switch difficulty levels at any time, and the change applies to subsequent decisions within the same session.

```
+---------------------------------------------------+
|  DIFFICULTY SELECTOR                              |
|                                                   |
|  [Simple]     Bet / Check / Fold                  |
|  [Grouped]    Small Bet / Big Bet / Overbet       |
|  [Standard]   33% / 50% / 75% / 125% / 175%      |
+---------------------------------------------------+
```

---

## Multitabling

GTO Wizard supports simultaneous play across **1, 2, 3, or 4 tables**. Multitabling simulates the reality of online poker while increasing training volume.

### Interface Adaptations for Multi-Table Play

When multitabling is active, the interface adjusts:

- Each table has its own dedicated information panel button (since the system must distinguish which table's data to display)
- Hand replay and navigation controls move to individual tables
- The three-dot menu in the top-left of each table provides per-table options (replay hand, advance to next hand)

```
+---------------------------+---------------------------+
|        TABLE 1            |        TABLE 2            |
|  [...]                    |  [...]                    |
|  BTN vs BB                |  CO vs BTN                |
|  Board: Ks 9h 4d          |  Board: As Jh 7c          |
|  Pot: 6.5 BB              |  Pot: 12.0 BB             |
|                           |                           |
|  [Check] [Bet 33%] [75%] |  [Fold] [Call] [Raise]    |
+---------------------------+---------------------------+
|        TABLE 3            |        TABLE 4            |
|  [...]                    |  [...]                    |
|  UTG vs BB                |  BTN vs SB                |
|  Board: Qd Tc 5s 2h       |  Preflop                  |
|  Pot: 18.3 BB             |  Pot: 1.5 BB              |
|                           |                           |
|  [Check] [Bet 50%] [OB]  |  [Fold] [Call] [3-Bet]    |
+---------------------------+---------------------------+
```

### Game Speed Settings

| Speed   | Description                              |
|---------|------------------------------------------|
| Normal  | Standard animation and pacing            |
| Fast    | Reduced animation delays                 |
| Turbo   | Minimal delay, maximum decision volume   |

---

## Scoring System

### GTOW Score

The primary performance metric is the **GTOW Score**, a value between **-100% and +100%** assigned to each move based on accuracy relative to GTO.

The scoring system was rescaled from the original 0-100 range to -100 to +100, meaning:
- Inaccurate play is penalized more harshly (scoring below zero)
- Correct play is rewarded more intensely (scoring above zero)
- The neutral midpoint is zero (inaccuracy)

### Move Classification

Every decision is classified into one of five categories:

| Classification | Symbol       | Definition                                                          |
|----------------|-------------|---------------------------------------------------------------------|
| **Best Move**  | Double check | Highest frequency action (or correct RNG-rolled action)             |
| **Correct**    | Single check | Action taken at some GTO frequency, but not the most frequent       |
| **Inaccuracy** | Yellow dot   | Action taken less than 3.5% of the time in GTO, minimal EV loss    |
| **Wrong Move** | Red dot      | Action not present in the GTO solution                              |
| **Blunder**    | Red X        | Action never taken in GTO AND causes significant EV loss            |

Best moves earn maximum points. Correct moves earn partial points proportional to their GTO frequency. Blunders lose points proportional to their loss in percentage of the pot.

### EV Loss Calculation

EV loss measures how much expected value (in big blinds) a player sacrificed compared to perfect GTO play:

| Metric                    | Description                                                      |
|---------------------------|------------------------------------------------------------------|
| **Total EV Loss**         | Cumulative BB lost vs. GTO across all hands                     |
| **Avg EV Loss per Hand**  | Total EV loss divided by number of hands played                  |
| **Avg EV Loss per Mistake**| Total EV loss divided by number of mistakes only                |
| **EV Loss as % of Pot**   | Loss relative to pot size (enables cross-pot-size comparison)   |
| **Frequency Difference**  | Deviation from optimal frequencies: (P(best) - P(your action)) / total |

### Performance Benchmarks

Based on GTO Wizard's guidance for interpreting scores:

| Score Range   | Assessment                                                    |
|---------------|---------------------------------------------------------------|
| 95%+          | Excellence by GTO standards                                   |
| 90-95%        | Solid regular play                                            |
| 85-90%        | Above average, room for improvement                           |
| 80-85%        | Needs work (especially in 6-max formats)                      |
| Below 80%     | Significant leaks present                                     |

Context matters: ring game players fold 75-80% of hands preflop (automatically correct), so high scores are more easily achieved. Wide-range formats (HU, Spins) naturally produce lower scores.

---

## RNG (Random Number Generator) System

Mixed strategies are a core component of GTO play. The RNG system helps players practice implementing mixed strategies correctly.

When RNG is active, a number between **1-100** is generated for each decision. The player must choose the action corresponding to their rolled number based on the GTO frequency distribution.

### RNG Modes

| Mode      | Dice Color | Logic                                                    |
|-----------|------------|----------------------------------------------------------|
| **High**  | Yellow     | Most aggressive action when rolling highest (100 = overbet, 1 = check) |
| **Low**   | Yellow     | Most aggressive action when rolling lowest (1 = overbet, 100 = check)  |
| **Off**   | White      | No RNG; "best" = highest frequency action                |

When RNG is active, the "best move" designation shifts to the action that corresponds to the rolled number, not simply the highest-frequency action.

```
+--------------------------------------------+
|  RNG DISPLAY                               |
|                                            |
|  [72]  (Yellow die = RNG active)           |
|                                            |
|  GTO frequencies:                          |
|    Check: 40%  (rolls 1-40)                |
|    Bet 33%: 35% (rolls 41-75)              |
|    Bet 75%: 25% (rolls 76-100)             |
|                                            |
|  Your roll: 72 --> Bet 33% is "best"       |
+--------------------------------------------+
```

---

## Drill Configuration

### Creating Drills

Drills can be created from multiple access points:
1. **Practice tab**: Click "New" to create a fresh drill
2. **Solution Browser**: Click the controller icon to create a drill based on the current spot
3. **Range Builder**: Create a drill from the spot being studied
4. **Hand Analyzer**: Hover over analyzed hands and click the drill icon

### Configurable Parameters

| Parameter         | Options                                                    |
|-------------------|------------------------------------------------------------|
| Position          | UTG, HJ, CO, BTN, SB, BB (or any position pair)          |
| Action type       | RFI, vs 3-bet, 3-bet defense, etc.                        |
| Starting spot     | Specific node in the game tree                             |
| Stack depth       | Varies by solution set (e.g., 100bb, 60bb, 40bb, 20bb)   |
| Table size        | Heads-up, 6-max, 8-max, 9-max                             |
| Hand groups       | Limit to specific subsets of the range                     |
| Board filters     | Specific textures or card combinations                     |
| Difficulty        | Simple, Grouped, Standard                                  |
| Game speed        | Normal, Fast, Turbo                                        |
| Number of tables  | 1-4                                                        |
| Timebank          | 7, 15, or 25 seconds per decision                          |
| Auto New Hand     | Automatically deal new hand after completion               |
| Pause behavior    | Pause after every action, or pause only after mistakes     |

### Preflop Range Limiting

When configuring drills, players can limit the preflop hand combinations to specific subsets. This saves time by auto-skipping obvious fold spots and focuses practice on the most instructive hands.

```
+---------------------------------------------------+
|  DRILL CONFIGURATION                              |
|                                                   |
|  Solution:    Cash 6max 100bb                     |
|  Position:    CO vs BTN 3-bet                     |
|  Mode:        Street (Flop only)                  |
|  Difficulty:  Grouped                             |
|  Speed:       Fast                                |
|  Tables:      2                                   |
|  Timebank:    15 sec                              |
|  Hands:       Bottom 50% of range                 |
|                                                   |
|  [START DRILL]                                    |
+---------------------------------------------------+
```

---

## The Trainer Interface

### Main Playing Area

```
+----------------------------------------------------------+
|  GTO WIZARD TRAINER                                      |
|                                                          |
|  Pot: 6.5 BB          Board: Ks 9h 4d                   |
|                                                          |
|  +--------------------------------------------------+   |
|  |                                                  |   |
|  |              [Community Cards]                   |   |
|  |              Ks  9h  4d                          |   |
|  |                                                  |   |
|  |  Villain (BB)                                    |   |
|  |  [face down] [face down]                         |   |
|  |                                                  |   |
|  |  Hero (BTN)                                      |   |
|  |  [Ah] [Qs]                                       |   |
|  |                                                  |   |
|  +--------------------------------------------------+   |
|                                                          |
|  Actions: [Check] [Bet 33%] [Bet 75%] [Bet 150%]        |
|                                                          |
|  RNG: [72]   Score: +85%   Hands: 47/50                  |
+----------------------------------------------------------+
```

### Info Panel

The info panel provides real-time strategic context during play:

| Section      | Content                                                  |
|--------------|----------------------------------------------------------|
| Range Tab    | Compare hero's and villain's hand ranges                 |
| Strategy Tab | Overall GTO strategy, range frequencies, hand class data |
| Eye icon     | Hide/show sections for distraction-free play             |
| Pop-out      | Open info panel in separate window                       |

### Control Panel

| Control         | Options                                          |
|-----------------|--------------------------------------------------|
| Tables          | 1, 2, 3, or 4                                   |
| Game Mode       | Full Hand, Street, Spot                          |
| Game Speed      | Normal, Fast, Turbo                              |
| Input Mode      | Actions or Frequency-based selection             |
| Timebank        | 7, 15, or 25 seconds                            |
| Auto New Hand   | On / Off                                         |
| Pause After     | Every action / Only mistakes                     |

---

## Session Statistics and History

### Post-Session Summary

After completing a training session, a summary screen displays:

```
+---------------------------------------------------+
|  SESSION SUMMARY                                  |
|                                                   |
|  Hands Played:    50                              |
|  GTOW Score:      +87%                            |
|  Total EV Loss:   -2.34 BB                        |
|  Avg EV/Hand:     -0.047 BB                       |
|  Avg EV/Mistake:  -0.31 BB                        |
|                                                   |
|  Best Moves:      72%                             |
|  Correct Moves:   18%                             |
|  Inaccuracies:     6%                             |
|  Wrong Moves:      3%                             |
|  Blunders:         1%                             |
|                                                   |
|  Best Hand:   AhKs (+1.2 BB vs GTO)              |
|  Worst Hand:  7d6d (-0.85 BB vs GTO)             |
+---------------------------------------------------+
```

### Stats Page

The Stats page serves as the lifetime performance dashboard:
- Filter by format and date range
- View performance breakdowns across different pot types
- Track metrics over time: hands played, mistakes made, GTOW score, EV losses
- Identify leaks by position, street, or action type

### Practiced Hands Page

Individual decisions can be reviewed with sortable columns:
- EV loss per decision
- Date played
- Frequency difference
- Click any hand to open a sidebar with the hand history overview
- Replay functionality for step-by-step review

### Sessions Page

Tracks performance across training sessions:
- Aggregate GTOW scores per session
- EV losses per session
- Hand-by-hand breakdowns
- Built-in replay functionality

---

## Training Ideas and Recommended Drills

### The 5 Levels of Trainer Mastery

GTO Wizard recommends a progressive training framework:

**Level 1 - The Apprentice**: Master preflop fundamentals
- Focus: Opening ranges and defending ranges from each position
- Drills: RFI from each position, BB defense vs. each opener
- Mode: Use High RNG to practice mixed strategies from the start

**Level 2 - The Student**: Single-street postflop drills
- Focus: C-betting, facing c-bets, check-raising
- Drills: IP c-betting on various board textures, OOP defense vs. c-bets

**Level 3 - The Practitioner**: Multi-street sequences
- Focus: Barrel decisions, turn play after flop actions
- Drills: Full street mode covering flop-to-river sequences

**Level 4 - The Specialist**: Complex spots and edge cases
- Focus: 3-bet pots, 4-bet pots, multiway transitions to heads-up
- Drills: Spot mode for specific complex decision points

**Level 5 - The Master**: Full hand simulations at Standard difficulty
- Focus: Complete hand integration with precise sizing
- Drills: Full Hand mode with 2-4 tables at Turbo speed

### Recommended Drill Configurations

| Drill Name              | Position    | Mode       | Difficulty | Focus                          |
|-------------------------|-------------|------------|------------|--------------------------------|
| CO RFI Bottom Range     | CO          | Spot       | Simple     | Marginal open decisions        |
| BB Defense vs UTG       | BB vs UTG   | Spot       | Grouped    | Wide defense frequencies       |
| BTN C-bet Practice      | BTN vs BB   | Street     | Standard   | Flop c-bet sizing decisions    |
| Facing 3-Bet IP         | BTN vs SB   | Full Hand  | Grouped    | Complete 3-bet pot navigation  |
| Bubble 4-Bet Spots      | Various     | Spot       | Simple     | Tournament-specific aggression |

### Learning Principles

1. **Appropriate difficulty**: Training should challenge without overwhelming
2. **Focused but varied**: Master one spot at a time while avoiding over-generalization
3. **Consistent routine**: Dedicate regular sessions for immersive learning
4. **Goal-tracking**: Monitor progress metrics to maintain motivation
5. **Specificity**: The more specific the question your drill addresses, the more value you extract

---

## Practice Mode Component Specifications

| Component            | Position       | Data Displayed                              | Interaction Model            |
|----------------------|----------------|---------------------------------------------|------------------------------|
| Playing Area         | Center         | Board, cards, pot, actions                  | Click action buttons         |
| Action Buttons       | Bottom         | Available actions with bet sizes            | Click to make decision       |
| RNG Display          | Bottom-left    | Rolled number, active/inactive indicator    | Read-only                    |
| Score Display        | Bottom-right   | Running GTOW score                         | Read-only                    |
| Info Panel           | Right sidebar  | Range, strategy, hand class data           | Toggle, pop-out, hide        |
| Control Panel        | Top-right      | Tables, mode, speed, settings              | Dropdowns and toggles        |
| History Panel        | Accessible     | Previously played hands                    | Click to replay/review       |
| Drill Selector       | Top            | Current drill configuration                | Dropdown, new drill button   |
| Multitable Grid      | Full screen    | 1-4 concurrent tables                      | Per-table action buttons     |
| Session Summary      | Post-session   | Score, EV loss, move classifications       | Review and replay options    |

---

## Screenshot References

> **Screenshot references (Trainer interface):** See `screenshots/website/practice-advanced-default.avif`, `screenshots/website/practice-train-any-spot-default.avif`

> **Screenshot references (Custom drills):** See `screenshots/website/practice-custom-drills-default.avif`

> **Screenshot references (Instant feedback):** See `screenshots/website/practice-instant-feedback-default.avif`

> **Screenshot references (Third-party views):** See `screenshots/reviews/solvers-poker-trainer.png`, `screenshots/reviews/h2n-trainer.webp`, `screenshots/reviews/h2n-practice.webp`

---

## Sources

- [Practice Mode Overview - GTO Wizard Help](https://help.gtowizard.com/practice-mode-overview/)
- [How To Use the Trainer - GTO Wizard Help](https://help.gtowizard.com/how-to-use-the-trainer/)
- [Measure Performance - GTO Wizard Help](https://help.gtowizard.com/measure-performance/)
- [Train Like a Pro - GTO Wizard Blog](https://blog.gtowizard.com/train-like-a-pro/)
- [The 5 Levels of Trainer Mastery - GTO Wizard Blog](https://blog.gtowizard.com/the-5-levels-of-trainer-mastery/)
- [How to Use Practice Mode in GTO Wizard - Blog](https://blog.gtowizard.com/how-to-use-practice-mode-in-gto-wizard-to-improve-your-game/)
- [Multitabling & New Solutions - GTO Wizard Blog](https://blog.gtowizard.com/multitabling-new-solutions/)
- [Tips & Tricks - GTO Wizard Help](https://help.gtowizard.com/tips-and-tricks-to-study-analyzed-hands/)

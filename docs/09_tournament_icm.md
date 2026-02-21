# Tournament Strategy: ICM, PKO, Bounties, and Satellites

## Overview

Tournament poker differs fundamentally from cash games because chips have non-linear monetary value. In a cash game, every chip is worth exactly its face value. In a tournament, the last chip you lose (elimination) is catastrophically expensive, while the first chip you win from an opponent is worth less than face value due to the flattened payout structure. This asymmetry is captured by the **Independent Chip Model (ICM)**, and it pervades every decision in tournament play.

GTO Wizard provides comprehensive tournament solving through precomputed ICM solutions, Custom ICM Solving via GTO Wizard AI, and specialized support for knockout (KO), progressive knockout (PKO), mystery bounty, satellite, and freezeout formats. The platform offers over 50,000 ICM final table simulations and supports tournaments of up to 4,096 players.

---

## ICM: The Independent Chip Model

### What ICM Is

ICM is a mathematical framework that converts tournament chip stacks into monetary equity. It answers the question: "Given the current chip distribution among all remaining players and the payout structure, what is each player's expected prize money?"

### How ICM Works

The model uses a combinatorial probability approach:

1. **Probability of finishing 1st**: Proportional to chip stack (your chips / total chips)
2. **Probability of finishing 2nd**: Calculated by removing each possible 1st-place finisher and recomputing conditional probabilities
3. **Probability of finishing Nth**: Recursively computed through all possible elimination orderings
4. **Dollar equity**: Sum of (probability of each finish) x (payout for that finish)

```
+--------------------------------------------------------------+
|               ICM CALCULATION EXAMPLE                         |
+--------------------------------------------------------------+
|                                                                |
|  3 Players Remaining, $1000 Prize Pool                         |
|  Payouts: 1st=$500, 2nd=$300, 3rd=$200                        |
|                                                                |
|  Player    Chips    Chip%    ICM Equity    $ Value             |
|  ------    -----    -----    ----------    -------             |
|  Alice     5000     50%       45.8%        $458                |
|  Bob       3000     30%       30.0%        $300                |
|  Carol     2000     20%       24.2%        $242                |
|                                                                |
|  Note: Alice has 50% of chips but only 45.8% of prize pool.   |
|  Carol has 20% of chips but 24.2% of prize pool.              |
|  This is the non-linearity of ICM.                             |
+--------------------------------------------------------------+
```

### Key ICM Principles

- **Chips gained are worth less than chips lost**: Doubling your stack does not double your equity
- **Short stacks have proportionally more equity**: Their elimination risk is already priced in
- **Big stacks have proportionally less equity**: Additional chips yield diminishing returns
- **The chip leader's marginal chip is worth the least**: They already have a high probability of cashing

---

## Bubble Factor

### Definition

Bubble Factor quantifies ICM pressure at any point in a tournament. It measures the ratio of tournament equity lost when you bust versus the equity gained when you double up:

```
Bubble Factor = $EV lost when you lose / $EV gained when you win
```

To determine the required equity for calling an all-in:

```
Required Equity = Bubble Factor / (Bubble Factor + 1)
```

### Practical Example

In a 10-player $100 SNG with $500/$300/$200 payouts:
- A player risks $100 in equity to gain $84.44
- Bubble Factor = $100 / $84.44 = **1.18**
- Required equity to call = 1.18 / (1.18 + 1) = **54%**

Compare this to a cash game where required equity is always 50%. The 4% difference represents ICM pressure.

### Bubble Factor by Tournament Stage

```
+--------------------------------------------------------------+
|               BUBBLE FACTOR PROGRESSION                       |
+--------------------------------------------------------------+
|                                                                |
|  Bubble Factor                                                 |
|       ^                                                        |
|  2.0  |                                         *              |
|       |                                    *         *         |
|  1.5  |                              *                    *    |
|       |                         *                              |
|  1.0  |  * * * * * * * * *                                     |
|       |                                                        |
|  0.5  |                                                        |
|       +------------------------------------------>             |
|       Early    Mid    Bubble    FT     HU                      |
|                                                                |
|  Key inflection points:                                        |
|  - Money bubble: ~1.6 (medium stacks most affected)            |
|  - Final table: ~1.7 (pay jumps create pressure)               |
|  - Heads-up: drops back toward 1.0                             |
+--------------------------------------------------------------+
```

### Stack Size and Bubble Factor

| Stack Type     | Bubble Factor | Strategic Implication                    |
|---------------|---------------|------------------------------------------|
| Chip leader    | Low (~1.0-1.1)| Can apply pressure, wider ranges         |
| Medium stacks  | High (~1.5+)  | Tightest play, most ICM pressure         |
| Short stacks   | Moderate      | Less equity to lose, can shove wider     |

---

## How ICM Affects GTO Strategy

### Postflop Strategic Changes

GTO Wizard's ICM solutions reveal systematic differences from Chip EV (cash game) strategy:

1. **Downward Drift in Sizing**: "Big bets become small bets, small bets become calls, calls become folds." Under ICM pressure, the solver shifts toward smaller, lower-variance actions.

2. **Covering Advantage**: The player with more chips becomes more aggressive. They can afford confrontation; their opponent cannot.

3. **Polarization at Shallow Depths**: With extreme ICM pressure (20bb effective), strategies become "extremely polar" -- shove-or-fold dynamics dominate, with very few intermediate actions.

4. **Positional Value Increases**: "The higher the risk premium, the more valuable the BB's positional advantage, increasing their equity realization." Position matters even more under ICM than in cash games.

5. **Premium Hands Change Value**: Hands like AK and JJ become more valuable for denying equity (3-bet shoving), while even pocket aces can decrease in value for raising because they "gain less from denying equity" -- they already dominate most calling ranges.

```
+--------------------------------------------------------------+
|        ICM vs. CHIP EV: FLOP C-BET STRATEGY                  |
+--------------------------------------------------------------+
|                                                                |
|  Scenario: BTN (40bb) vs BB (70bb), A-8-3 flop               |
|  Final table, significant pay jumps                            |
|                                                                |
|  CHIP EV:                                                      |
|  Bet 33%:  35%  |===========                                  |
|  Bet 75%:  40%  |=============                                 |
|  Bet 150%: 25%  |========                                      |
|  (Mixed sizing, includes overbets)                             |
|                                                                |
|  ICM:                                                          |
|  Bet 33%:  85%  |============================                  |
|  Bet 75%:  10%  |===                                           |
|  Bet 150%:  5%  |=                                             |
|  (Almost entirely small sizing)                                |
|                                                                |
|  ICM shifts strategy toward lower variance                     |
+--------------------------------------------------------------+
```

### Preflop Strategic Changes

- Open-raising ranges tighten, especially from early positions
- 3-bet ranges become more polar (value-heavy or fold, fewer light 3-bets)
- Calling ranges narrow as flat-calling becomes riskier
- Short stacks adopt push/fold strategies at higher stack depths than in cash games

---

## PKO: Progressive Knockout Tournaments

### Prize Structure

In Progressive Knockout tournaments, the buy-in splits into two pools:

```
+--------------------------------------------------------------+
|            PKO PRIZE POOL STRUCTURE                           |
+--------------------------------------------------------------+
|                                                                |
|  $200 Buy-in Example (200 players):                           |
|                                                                |
|  Total Prize Pool: $40,000                                     |
|                                                                |
|  +---------------------------+                                 |
|  |  Regular Prize Pool: 50%  |  $20,000                       |
|  |  (Standard MTT payouts)   |  Distributed by finish         |
|  +---------------------------+                                 |
|                                                                |
|  +---------------------------+                                 |
|  |  Bounty Prize Pool: 50%   |  $20,000                       |
|  |  ($100 starting bounty    |  Awarded via knockouts         |
|  |   per player)             |                                 |
|  +---------------------------+                                 |
|                                                                |
|  When you eliminate a player:                                  |
|  - You collect 50% of their bounty immediately                 |
|  - The other 50% adds to YOUR bounty                           |
|                                                                |
|  Example: You knock out someone with a $200 bounty:            |
|  - You receive $100 cash immediately                           |
|  - Your bounty grows by $100 (from $100 to $200)              |
+--------------------------------------------------------------+
```

### Bounty Power

**Bounty Power** is a conversion factor that translates the dollar value of a bounty into big blind equivalents:

```
Bounty Power = Total Chips in Play / (Remaining Bounty Pool + Remaining Prize Pool)
```

This metric allows players to calculate adjusted pot odds that account for bounty value. A critical dynamic: **the value of a bounty decreases relative to the value of big blinds as the tournament progresses**, even as average bounties increase in absolute dollar terms.

### Adjusted Pot Odds in PKO

Traditional pot odds are modified to include bounty value:

```
Required Equity = Amount to Call / (Pot After Calling + Bounty Value in BB)
```

Example: With a $50 bounty worth 9.55 BB and a 27.5 BB pot, required equity drops from 43.6% (without bounty) to 32.4% (with bounty). This significantly widens profitable calling ranges.

### Three Components of PKO Value

GTO Wizard's solver accounts for three distinct value components:

1. **Immediate Knockout EV**: The direct bounty collected upon elimination. Neural networks predict knockout probabilities based on hand histories.

2. **Future Bounty EV**: The expected value from winning bounties in future hands, calculated via the Proportional Bounty Model:

   ```
   Future Bounty EV = Chip Share x Remaining Bounty Prizes
   ```

   This assumes bounty acquisition correlates directly with chip distribution.

3. **ICM Prize Equity**: The standard ICM calculation on the regular prize pool portion, reflecting improved stack position after winning a hand.

### Strategic Impact

PKO strategy diverges significantly from standard MTTs:

| Situation          | Standard MTT                    | PKO Tournament                     |
|-------------------|---------------------------------|------------------------------------|
| Early stages       | Moderate ranges                  | Significantly wider ranges          |
| Calling all-ins    | Tight (ICM pressure)            | Looser (bounty adds to pot odds)   |
| Multiway pots      | Avoid (survival priority)        | Seek (more bounty opportunities)   |
| Covering opponents | Moderate advantage                | Major advantage (can claim bounty)  |
| Bubble play        | Very tight                       | Looser than standard ICM            |

---

## The ICM Dial in PKO

GTO Wizard introduces the concept of the "ICM Dial" -- two competing forces that determine optimal PKO strategy:

```
+--------------------------------------------------------------+
|                    THE ICM DIAL                                |
+--------------------------------------------------------------+
|                                                                |
|  LOOSE <=========|=========> TIGHT                             |
|                  ^                                             |
|                  |                                             |
|         Current optimal play                                   |
|                                                                |
|  <-- Bounty Power           Bubble Factor -->                  |
|  (encourages loose play)    (encourages tight play)            |
|                                                                |
|  Early Tournament:                                             |
|  LOOSE <=====|===============> TIGHT                           |
|  (Bounty power dominates, BF can drop below 1.0)              |
|                                                                |
|  Bubble:                                                       |
|  LOOSE <=============|=======> TIGHT                           |
|  (ICM pressure increases, but bounties still offset)           |
|                                                                |
|  Standard MTT Bubble (for comparison):                         |
|  LOOSE <==================|==> TIGHT                           |
|  (No bounty offset, pure ICM pressure)                         |
+--------------------------------------------------------------+
```

Key insight: In PKOs near the bubble, the Bubble Factor might be 1.22 compared to 1.8 in a standard MTT. The bounties partially offset ICM tightening, keeping play significantly looser.

A unique PKO phenomenon is the **negative risk premium**: situations where "wins help more than losses hurt." This inverts traditional tournament survival pressure and occurs when a player covers their opponent and the bounty value exceeds the ICM cost of confrontation.

---

## KO: Standard Knockout Tournaments

Standard knockouts feature simpler bounty mechanics:

- **Fixed bounty amount**: Every player has the same bounty (e.g., $50)
- **Full bounty on elimination**: The entire bounty goes to the eliminator (no progressive element)
- **Bounty pool decreases linearly**: As players are eliminated, the total remaining bounty pool shrinks predictably

The strategic implications are similar to PKO but less extreme, as bounties do not compound. There is less incentive to target specific players, since all bounties are equal.

---

## Mystery Bounty Tournaments

Mystery bounties add randomness to the knockout format:

- Bounty values remain unknown during most of the tournament
- The "bounty phase" typically begins when approximately 15% of the field remains
- Bounty amounts are drawn randomly from a distribution when claimed
- Strategy uses the **average bounty value** (total bounty pool / entries) for calculations

Because individual bounty values are unknown, the solver treats each potential knockout as worth the expected average value, simplifying calculations while maintaining accuracy.

---

## Satellite Tournaments

### The Unique Payout Structure

Satellites have the most extreme ICM dynamics of any tournament format. All prizes are **equal value** -- whether you finish first or barely survive the bubble, you win the same ticket. There is zero incentive to accumulate chips beyond survival.

```
+--------------------------------------------------------------+
|           SATELLITE PAYOUT STRUCTURE                          |
+--------------------------------------------------------------+
|                                                                |
|  Standard MTT:          Satellite:                             |
|  1st: $5000             1st: $1000 ticket                      |
|  2nd: $3000             2nd: $1000 ticket                      |
|  3rd: $2000             3rd: $1000 ticket                      |
|  4th: $1500             4th: $1000 ticket                      |
|  5th: $1000             5th: $1000 ticket                      |
|  6th: $0                6th: $0                                |
|  7th: $0                7th: $0                                |
|                                                                |
|  In the MTT, chip leader earns 5x more than 5th place.        |
|  In the satellite, chip leader earns EXACTLY the same.         |
|  This creates extreme ICM pressure near the bubble.            |
+--------------------------------------------------------------+
```

### Strategic Implications

The flat payout creates radical strategic departures from standard tournaments:

1. **Mutually Assured Destruction**: Neither player benefits from confrontation near the bubble. Both prefer folding their way to a seat rather than risking elimination.

2. **Compressed Calling Ranges**: Near the bubble with equal stacks (e.g., 20bb each), calling ranges compress to only the most premium holdings (pocket aces, sometimes only aces).

3. **Wide Shoving Despite Tight Calling**: Shoving becomes nearly universal (93%+ of hands from some positions) because opponents can rarely call.

4. **Ace Blocker Value**: Hands containing an ace gain enormous value because they block opponents' premium calling range (AA, AK).

5. **Stack Size Sensitivity**: Even modest stack reductions (from 20bb to 5bb) dramatically shift opening ranges, demonstrating extreme sensitivity to ICM calculations.

```
+--------------------------------------------------------------+
|      SATELLITE BUBBLE: 6 Players, 5 Tickets                  |
+--------------------------------------------------------------+
|                                                                |
|  All players: 20bb                                             |
|                                                                |
|  UTG Shove Range:    ~93% of hands                             |
|  BB Call vs UTG:     Only AA (sometimes AKs)                   |
|                                                                |
|  Reason: Even if UTG shoves 93%, BB cannot profitably call     |
|  with KK because the ICM cost of busting far exceeds the      |
|  benefit of doubling up (you already have a ticket's worth     |
|  of chips).                                                    |
+--------------------------------------------------------------+
```

---

## Final Table Simulations

GTO Wizard provides over **50,000 precomputed ICM final table solutions** for 200-player tournaments. These cover diverse stack distributions and stages, enabling players to study specific scenarios they encounter.

Features include:
- **Streamlined search**: Quickly locate comparable stack distributions from your actual tournament experiences
- **Full postflop ICM solving**: Not just push/fold charts, but complete postflop strategy trees
- **Custom ICM solving**: Via GTO Wizard AI, solve any heads-up postflop tournament spot with custom parameters
- **Multi-format support**: Solutions for freezeout, KO, PKO, satellite, and mystery bounty final tables

---

## Payout Structure Configurations

GTO Wizard supports custom payout structure input:

- **Prefilled structures**: Common online poker site payout structures are available for quick selection
- **Tournament ID search**: Import exact tournament data by entering the tournament ID from popular sites
- **Manual input**: Define custom payout structures for any number of players and prize distribution
- **Up to 4,096 players**: Supports field sizes from heads-up to massive multi-table tournaments

---

## Differences from Cash Game GTO

The following table summarizes the key strategic differences between cash game (Chip EV) and tournament (ICM) GTO:

| Aspect                   | Cash Game (Chip EV)              | Tournament (ICM)                      |
|-------------------------|----------------------------------|---------------------------------------|
| Chip value              | Linear (1 chip = 1 unit)         | Non-linear (diminishing returns)      |
| Risk tolerance          | Neutral                          | Risk-averse (survival matters)        |
| Bet sizing              | Full range of sizes              | Shifted toward smaller sizes          |
| Calling ranges          | Based on pot odds                | Tighter (Bubble Factor > 1)           |
| Bluffing frequency      | Based on pot odds/MDF            | Reduced (cost of failure is higher)   |
| Stack depth relevance   | Pure strategic consideration     | Determines ICM pressure level         |
| Position value          | Important                        | Even more important (EQR amplified)   |
| Opponent stack matters  | Mainly for effective stack       | Critically affects your Bubble Factor |
| Multiway dynamics       | Pot odds driven                  | Survival driven (avoid confrontation) |
| All-in decisions        | Pot odds / equity based          | Bubble Factor adjusted equity needed  |

### The "Downward Drift" Principle

ICM creates a systematic downward shift in aggression:
- Overbets become large bets
- Large bets become medium bets
- Medium bets become small bets
- Small bets become checks
- Calls become folds

This drift is proportional to ICM pressure (higher Bubble Factor = more drift).

---

## GTO Wizard's Tournament Solving Technology

### Custom ICM Solving with GTO Wizard AI

The custom solver allows users to solve any heads-up postflop tournament situation with:
- Adjustable stack sizes and payout structures
- Tournament stage customization
- Bet size inputs and range adjustments
- Nodelocking for scenario-specific analysis (see [Nodelocking](07_nodelocking.md))
- Bounty power display translating $ values to bb equivalents

### Speed Advantage

Where traditional ICM solving required "multiple pieces of software, extensive computing power, and considerable time," GTO Wizard AI delivers results in seconds. The neural network approach (see [GTO Wizard AI](06_gto_wizard_ai.md)) applies to tournament solving just as it does to cash games.

### Accuracy

The evaluation function handles all tournament stages, from early registration through heads-up play. While GTO Wizard AI calculates one street at a time (using neural network value estimates for future streets), it performs remarkably well across all tournament stages and formats.

---

## Screenshot References

> **Screenshot references (3-way solving with ICM):** See `screenshots/blog/3way-btn-response.png`, `screenshots/blog/3way-sb-vs-btn-bet.png`

---

## Sources

- https://blog.gtowizard.com/revolutionizing-mtts-the-ultimate-icm-solver-upgrade/
- https://blog.gtowizard.com/bounty-models-explained-solving-knockout-tournaments/
- https://blog.gtowizard.com/the-theory-of-progressive-knockout-tournaments/
- https://blog.gtowizard.com/how-does-icm-impact-pko-strategy/
- https://blog.gtowizard.com/the-theory-of-bounty-tournaments-part-1-prize-pool-fundamentals/
- https://blog.gtowizard.com/the-theory-of-bounty-tournaments-part-2-icm-and-equity-drops/
- https://blog.gtowizard.com/the-theory-of-bounty-tournaments-part-3-key-calculations/
- https://blog.gtowizard.com/what-is-the-bubble-factor-in-poker-tournaments/
- https://blog.gtowizard.com/how-icm-impacts-postflop-strategy/
- https://blog.gtowizard.com/satellite-guide/
- https://blog.gtowizard.com/mystery-bounty-guide/
- https://blog.gtowizard.com/now_live_3_way_solving_nodelocking_2_0_and_50k_icm_ft_sims/
- https://blog.gtowizard.com/icm/

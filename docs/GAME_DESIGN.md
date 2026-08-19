# Idle Hands — Game Design Document

Status: Current post-launch product baseline
Date: 2026-08-15

## 1. High Concept

Idle Hands is a personal drawer of tactile tabletop and puzzle games. Opening
the app reveals a beautifully illustrated cabinet whose objects are the game
selector: a deck, booklet, tile tray, dice cup, and other small curios. Sessions
are short, rules are familiar, and the collection never pressures the player to
return.

The primary devices are iPhone and iPad in a browser. Desktop browsers and the
Windows build retain pointer and optional keyboard support, but the complete
experience is designed around taps, holds, and swipes.

The shipped cabinet currently contains 47 playable games. The eight titles in
Section 4 are the launch foundation; the post-launch additions are listed in
Section 11 and share the same persistence, tutorial, records, and responsive
touch contracts.

## 2. Design Pillars

1. **A satisfying object drawer.** Games should feel like physical things taken
   from a familiar cabinet, not unrelated entries in a software grid.
2. **One-thumb legibility.** Common actions are obvious, comfortably sized, and
   reachable; complex gestures always have a visible alternative.
3. **A complete game in minutes.** Every title offers a meaningful session
   without errands, energy systems, or meta-game maintenance.
4. **Quiet reasons to return.** Records, replayable seeds, optional daily
   puzzles, and cosmetic stamps add continuity without obligation.
5. **Rules before decoration.** Art and animation reinforce state; they never
   obscure legal moves, marks, values, or scoring.
6. **Arcade rounds are live.** Snake, Breakout, and Tiny Tower Defence advance
   from elapsed time, expose visible pause/resume controls, and never require a
   keyboard or repeated taps to keep the simulation moving.

## 3. Collection Loop

1. Open the cabinet.
2. Tap an object to open its game drawer.
3. Continue the saved session or start a new one.
4. Play through visible controls and direct board gestures.
5. Finish, abandon, or return to the cabinet; progress is saved automatically.
6. Optionally inspect records, rules, or cosmetic stamps.

The Records screen keeps the collection view legible with a compact summary of
stamps, achievements, and completed drawers out of the full 47-game cabinet.
Every drawer has its own completion achievement, in addition to first-finish
and full-cabinet collection awards. The Records screen opens a responsive
ACHIEVEMENTS shelf so earned and locked awards remain inspectable without
interrupting a game. Drawer awards use the drawer's own title so thematic
achievement names never obscure what was completed. ALL, EARNED, and LOCKED
filters let a player focus on the next collection goal.

Starting a new drawer session presents a clear CANCEL/START confirmation so an
accidental tap cannot replace an unfinished session.

The cabinet remembers the last-opened game and exposes a visible CONTINUE action
alongside Home, Help, Records, and Settings. Players can mark drawers with the
left-edge favorite markers and open optional FAVORITES and RECENT quick lists;
favorites and recent history are profile state and never hide or reorder the
full collection. A first-time player sees a short, game-specific
touch tutorial that can be replayed from Help.

## 4. Launch Games

### 4.1 Solitaire

Klondike with draw-one as the default launch ruleset. Tap a card to select it
and tap a destination to move it; dragging is a faster equivalent. Tapping the
stock deals, tapping a valid exposed card can auto-send it to a foundation, and
Undo/New Deal/Hint are visible controls. The initial release guarantees
rule-valid seeded deals, not necessarily deals proven solvable.

### 4.2 FreeCell

Standard 52-card FreeCell with four free cells and four foundations. It shares
card visuals, drag behavior, move animation, and seeded deck infrastructure
with Solitaire. Legal multi-card movement follows the available-free-cell and
empty-column capacity. Invalid moves return cleanly with a short explanation.

### 4.3 Sudoku

9×9 puzzles with selectable difficulty, pencil marks, undo, erase, and optional
mistake highlighting. A large number pad remains visible or one tap away.
Puzzles must have a unique solution and are generated or selected from seeded,
validated data. The player can disable all correctness assistance.

### 4.4 Minesweeper

Beginner, intermediate, expert, and a responsive custom board. The first reveal
is always safe. Normal tap reveals; marking uses either a long press or an
explicit Reveal/Flag mode, so precise timing is never required. Chording is
available through a visible action after selecting a revealed number.

### 4.5 2048

A standard 4×4 board controlled by horizontal and vertical swipes, with visible
direction buttons as an accessible alternative. One undo is available by
default. A session records score, best tile, and seed. Swipe thresholds scale
with screen density and do not trigger while navigating from the screen edge.

### 4.6 Nonogram

Seeded picture-logic puzzles beginning with 5×5, 10×10, and 15×15 boards. Fill
and Cross are explicit modes. Players may tap cells or drag a straight stroke;
the stroke locks to one row or column and previews before committing. Clues and
cells remain readable through zoom/pan or a focused-board layout on small
phones.

### 4.7 Yahtzee

Solo score chasing over thirteen rounds using the familiar five-dice category
structure. Tap dice to hold them, tap the large Roll control for up to three
rolls, then tap a legal score row. Potential scores are previews rather than
automatic choices. The game tracks personal best totals and category records.
The product uses the descriptive label “five-dice scorecard” where trademark
or storefront naming makes that preferable; exact public naming is a release
check.

### 4.8 Reversi

An 8×8 game against local AI, with same-device two-player as an option. Legal
moves are visibly marked and tapping one previews/flips the captured lines.
Passes happen only when no legal move exists and are explained on screen. AI
ships with at least two difficulty levels and performs within a bounded frame
budget so touch feedback remains immediate.

## 5. Cabinet and Navigation

The home screen is an illustrated cabinet rather than a conventional icon grid.
Each game has a distinct object, silhouette, label, and focus state. Objects
remain recognizable without color. On narrow portrait screens, cabinet shelves
stack vertically and scroll; tablets and landscape screens show the complete
cabinet or a wider arrangement.

Required global destinations are Home, Continue/New Game, Rules, Settings, and
Records. Destructive restarts require a confirmation sheet that names the game
and states that its current session will be replaced. Back navigation never
silently discards play.

The cabinet may filter its canonical order to ALL, OPEN, or DONE drawers for
quick collection browsing; filtering never changes a drawer's state or place.

## 6. Progression and Retention

The optional meta layer uses one plainly named collectible: stamps. Finishing
games and selected achievements awards stamps once. Stamps unlock only cosmetic
items such as card backs, board materials, sound sets, and small shelf
decorations. Cosmetics never alter rules, puzzle information, odds, hints, or
scores.

Daily seeded games are a post-launch extension. If added, missing a day has no
penalty and there is no escalating streak reward. Public shared seeds require a
stable seed/version scheme so future rule changes do not alter old puzzles.

## 7. Presentation and Audio

The cabinet uses warm wood, paper, brass, felt, ceramic, and ink. Individual
games can emphasize one material while sharing typography, spacing, navigation,
and motion language. Movement should feel tactile: cards settle, dice tumble,
tiles slide, and discs turn, with reduced-motion substitutions available.

Audio is optional feedback, never required information. Master, effects, and
music/ambience levels persist separately and each can be muted. Haptics may be
used only when browser/platform support is safe and every cue also has a visual
equivalent.

## 8. Accessibility and Responsive Requirements

- Support portrait and landscape layouts from 320×568 CSS pixels upward, plus
  common tablet and desktop sizes.
- Respect browser safe areas and keep primary actions away from gesture zones.
- Use 44×44 logical-point minimum targets; prefer 48 points for primary play.
- Never require hover, right-click, multi-touch, keyboard, or a timed long press.
- Provide a visible alternative for swipe, drag, hold, and chord gestures.
- Preserve readable text at user-selectable UI scale without clipping controls.
- Convey selection, legality, errors, and game-over state through more than hue.
- Provide reduced motion, high contrast, sound controls, and tutorial replay.
- Pause simulation when the page loses focus; autosave before/after lifecycle
  transitions where the platform permits.

## 9. Content and Data

Authored settings, themes, achievements, puzzle catalogs, and per-game rule
presets live in JSON. Each game owns typed schemas and semantic validation while
`macroquad_toolkit::data_loader` owns parsing and embedded/runtime loading.

Random deals, boards, dice, and AI tie breaks come from state-owned seeded RNG.
A replay identity includes the game ID, ruleset version, generator version, and
seed. Saves record enough information to resume exactly rather than regenerate
from incomplete context.

## 10. Historical Non-Goals for the First Collection

- Online multiplayer, accounts, leaderboards, cloud saves, or social feeds
- Purchases, ads, stamina, daily chests, streak pressure, or multiple currencies
- Spider Solitaire, Mahjong Solitaire, Checkers, arcade games, word games, and
  original mini-games before the first eight were complete and polished
- A single rigid landscape canvas scaled down into unreadable phone controls
- Perfect-solvability proofs for every generated card deal in the first release

## 11. Post-Launch Cabinet Additions

The current post-launch cabinet includes Lights Out, Tic-Tac-Toe, Memory,
Sliding Puzzle, Mastermind, Spider, Word Search, Hangman, Connect Four,
Checkers, Peg Solitaire, Mahjong Solitaire, Snake, Breakout, Higher or Lower,
Klondike Golf, Blackjack, Spider Solitaire, Dungeon Sweeper, Potion 2048, Tiny
Tower Defence, One Room Roguelike, Daily Dungeon, Dots & Boxes, Sokoban,
Mancala, Hanoi, Number Match, Flood It, Color Sort, Battleship, Word Grid,
Pipe Loop, Maze Walk, Match Three, Pyramid, TriPeaks, Nim, and Word Ladder.

The former candidate list is now represented in the shipped cabinet. Ongoing
post-launch work focuses on polish, accessibility, balance, and additional
cabinet ideas only when they improve the quiet, touch-first collection.

The cabinet's Arcade shelf is intentionally narrow: Snake, Breakout, and Tiny
Tower Defence are the continuously moving games. Dungeon Sweeper is grouped
with Logic, Battleship with Board, and One Room Roguelike with Misc. Higher or
Lower remains grouped with Cards.

### 11.1 Match Three depth pass

Match Three is a short score chase with a visible move budget. Matching four
tiles creates a row or column clear aligned to the run; matching five tiles or
crossing two runs creates a local burst. A marked tile fires when it joins a
match or is swapped, and chained falling matches earn an increasing cascade
multiplier. Stable boards with no legal swap reshuffle deterministically so a
round never ends through an unreadable deadlock. Standard, Hard, and Expert
increase field size, color count, target score, and available moves together.

### 11.2 Breakout wall run

Breakout is a three-wall run rather than a single uniform clear. The opening
wall teaches the field, while later seeded patterns introduce gaps and armored
bricks that take two or three hits. The player carries score and three lives
through the run; a miss consumes one life and presents a visible LAUNCH control
for the next serve. Paddle contact changes horizontal travel according to where
the ball lands, letting LEFT, STAY, and RIGHT shape the next angle instead of
only following it. Each later wall moves slightly faster.

### 11.3 Tiny Tower Defence roles and wave roster

Tiny Tower Defence asks the player to compose a defence rather than stack one
generic tower. BOLT deals focused damage along its lane, FROST trades damage for
range and a movement delay, and BURST reaches adjacent lanes around its target.
Each role has a visible build selector, its own cost, and three upgrade levels.
Early waves teach ordinary invaders, wave three introduces fast enemies that
move two cells, and wave five adds high-health armored enemies. Letter labels,
colors, and armor rings keep both tower and enemy roles readable without relying
on color alone, including at phone scale.

### 11.4 Snake cabinet variants

Snake offers three visible rulesets around the same touch steering. CLASSIC has
solid edges, WRAP joins opposite edges into a faster looping field, and GARDEN
uses a slower opening pace with twelve deterministic rock cells outside the
safe starting lane. Food never spawns inside the coil or a rock. Every fifth
point prepares a clearly starred gold food worth three points, while each
five-point band raises the pace. The head has directional eyes, rocks use crossed
marks, and mode names remain visible so these states are not conveyed by color
alone.

### 11.5 One Room Roguelike expedition

One Room Roguelike is a five-room expedition with a visible ending. Before a
run, the player can tap BLADE for three-damage strikes, WARDEN for fourteen
health and one point of protection from every hit, or ALCHEMIST for an extra
potion and six-point healing. That choice persists when NEW RUN reseeds the
rooms. Guards hold position, Stalkers step toward the hero before attacking,
and Brutes trade speed for heavier hits and larger health pools. Deeper seeded
rooms mix these archetypes, while the fifth guarantees a Brute. Letter labels,
separate colors, and the Brute's outer ring keep the roster readable without
depending on color. Clearing a room reveals its staircase; entering the fifth
staircase completes the run and records the score.

### 11.6 Daily Dungeon conditions and scouting

Daily Dungeon now rotates a deterministic condition with the day's seed.
WAYFINDER supplies three scouts against eight traps, FORAGER supplies two
scouts and two healing springs against seven traps, and DAREDEVIL supplies one
scout against nine traps while doubling the bravery reward. SCOUT spends one
charge and reveals every adjacent room without triggering it; highlighted
unknown neighbors show its exact reach. Entering a room before scouting earns
bravery score, so information has an explicit opportunity cost. Revealed traps
remain visible and hints route around them when another step is available.
Single-use springs restore one heart, and the final score rewards hearts and
unused scout charges after all three runes reach the exit.

### 11.7 Dungeon Sweeper relic routes

Dungeon Sweeper no longer lets the guaranteed-safe exit end a run on the first
tap. Each generated floor hides relic rooms away from the opening and exit, and
the exit unlocks only after every relic is revealed. EXPLORER places ten traps,
grants three hearts, and asks for two relics; DELVER uses twelve traps, two
hearts, and three relics; PERIL uses fifteen traps, one heart, and three relics.
A trap now consumes a heart and becomes a visible crossed room, so Explorer and
Delver can recover instead of every mistake ending immediately. Chords resolve
each room against the remaining heart supply. Revealed relics use a K plus their
small trap clue, and an early exit visibly reads LOCK. Difficulty buttons start
new seeded floors through the same restart confirmation as NEW DUNGEON, while
legacy in-progress floors retain their original open-exit objective.

### 11.8 Potion 2048 reaction chains

Potion 2048 rewards consecutive moves that actually merge ingredients. The
first reaction scores normally, the second doubles its reaction value, and the
multiplier continues until a slide without a merge breaks the chain. Completing
the visible chain brews a C catalyst in place of the normal spawned potion. A
catalyst reacts with any neighboring tier and produces twice that tier; two
catalysts produce a basic two-point potion. STANDARD brews C after three linked
reaction moves, HARD after four, and EXPERT after five, giving their larger
boards distinct planning rhythms as well as higher goals. Catalysts use a brass
diamond with a letter label rather than color alone. Combo state, best chain,
catalysts brewed, score, RNG, and the prior board all restore through UNDO, and
older saves begin with an empty chain.

### 11.9 Dots & Boxes opponent styles

Dots & Boxes difficulty now changes opponent policy in addition to board size.
STANDARD takes available boxes and otherwise keeps its seeded casual choice.
HARD takes boxes, avoids moves that create a three-sided gift whenever a safe
edge remains, and falls back to a seeded sacrifice. EXPERT evaluates every
available edge and minimizes the number of immediate gifts before applying its
deterministic tie break. The player hint follows the same tactical order: close
a box, choose a zero-gift edge, then name the forced sacrifice. Unclaimed boxes
with three sides display a circled exclamation mark to make chain danger
legible at phone scale. Horizontal and vertical edges now persist their actual
drawer, so red and blue line ownership no longer changes later when an adjacent
box is claimed; legacy boards expand the new owner arrays on their next move.

### 11.10 Sokoban warehouse route

Sokoban is a six-room authored warehouse route instead of a repeating set of
three openings. Each room publishes a par move count and awards GOLD, SILVER,
or BRONZE when cleared, while separate move and push totals make walking
efficiency and crate commitment visible. UNDO keeps the complete move history,
so the player can rewind several decisions rather than only the latest step.
Pushing an unsolved crate into a wall corner marks it with a red X and enters a
clear CORNERED state; the still-visible UNDO and RESTART controls provide both
local and full-room recovery. NEXT ROOM is separate from RESTART, preventing a
failed position from silently advancing to a different puzzle. The hint solver
continues to prove every authored room within its stated par.

### 11.11 Mancala openings and tactics

Mancala offers three board lengths without changing the familiar six-pit
shape: QUICK starts each pit with three stones, CLASSIC with four, and GRAND
with five. Switching the visible variant begins a confirmed fresh board while
preserving the chosen opponent strength. Legal player pits preview an E when
their last stone earns another turn and a C when the move captures the opposite
stones, making Kalah's two defining tactics learnable without opening a rules
screen. The header records captured stones and bonus turns for the current
board. GENTLE sows the first available pit, SHARP prioritizes immediate store
gain, captures, and extra turns, and EXPERT looks through chained bonus moves
while discounting choices that expose a strong player reply. Hints use the same
capture-aware tactical values and name the concrete reward they found.

### 11.12 Hanoi tower trials

Hanoi exposes three, five, and seven-disk towers as visible touch choices. Their
optimal targets scale from seven to thirty-one to one hundred twenty-seven
moves, and a completed tower receives PERFECT, CLOSE, or CLEAR feedback against
that target. Every disk carries its size number in addition to a distinct width
and color. Selecting a source marks each other peg with a green legal ring or a
red blocked ring before the destination tap, teaching the size constraint in
the board itself. UNDO retains the full move history rather than only the last
transfer, while RESTART preserves the current disk trial. The breadth-first
hint solver supports the full seven-disk state space and each selectable tower
is verified by following its shortest route to completion.

### 11.13 Number Match link rules

Number Match boards are built from deterministic horizontal and vertical
domino pairs inside varied two-by-two blocks, replacing the original row of
eighteen obvious pairings while retaining a known complete route. NEAR permits
only neighboring cells, LINES also connects equal or sum-ten numbers through
cleared horizontal and vertical space, and DIAGONAL adds clear diagonal paths.
After one number is selected, every currently valid partner gains a green
outline. Consecutive clears build a chain worth ten, twenty, thirty, and more
points; an invalid reselection breaks that chain. An alternate pairing can
strand the remaining values, which now produces an explicit NO LINKS state.
The complete UNDO history can rewind the decision, while two REMIX charges
rebuild the remaining count into a fresh guaranteed set of adjacent pairs and
can themselves be undone. Older saves resume with NEAR and two charges.

### 11.14 Flood It forecasts and surges

Flood It's color buttons now publish the exact number of cells that each choice
would add to the top-left territory, while the territory itself receives a
white cell outline. These redundant cues make both the current footprint and
the next growth options readable without inferring connectivity from color
alone. Every move that gains cells extends a chain; gained cells are multiplied
by the chain length for points, while a zero-growth color breaks it. A gain of
at least half the board width advances momentum, and three consecutive strong
gains earn a stored SURGE, capped at two. Tapping the visible SURGE control
applies the best current forecast without consuming a move, but still extends
the growth chain. Move limits therefore remain the primary win pressure while
efficient route planning creates a second resource arc. Forecasts, territory,
chains, momentum, surges, and full multi-step undo are deterministic and older
saves migrate to an uncharged field.

### 11.15 Color Sort sealed runs

Color Sort now treats a full uniform tube as sealed: it remains part of the
win condition but can no longer be selected as a source and accidentally
unbuilt. Selecting any other tube outlines every legal destination in green
and labels it with the number of top layers that will pour, including partial
pours limited by remaining capacity. Each colored layer also carries a stable
letter, so Expert's six-color layout does not rely on hue alone. Pouring onto
an empty tube begins a chain at one; successive matching-stack pours extend the
chain and multiply the number of moved layers, while sealing a tube adds a
ten-point reward. The header records sealed progress, points, and chain length.
Hints name the exact run size and whether the destination will seal, and the
complete move history supports repeated UNDO across pours and scoring state.
Legacy puzzles derive their sealed tubes directly from their existing contents
and begin without a chain.

### 11.16 Battleship fleet intelligence

Battleship now hides three vessels across seven segments: a three-cell flagship
and two two-cell escorts. Hits build a score chain worth ten points times the
current chain, misses reset it, and finishing any vessel awards a further
twenty-five points. The header separates segment hits from fully sunk ships,
while a sunk vessel's cells turn gold and carry `S` instead of the ordinary hit
cross. Each fleet also carries two SONAR charges. Tapping the visible SONAR
control arms it, and the next board tap sweeps the surrounding 3 × 3 area
without firing: clear scanned water receives a wave mark and hidden occupied
cells receive a `!` contact. Those contacts still require deliberate shots.
Hints prioritize known sonar contacts before hit neighbors and checkerboard
searching. Sonar sweeps and shots share the complete multi-step undo history,
and older saves receive two unused charges with an empty scan record.

### 11.17 Word Grid deduction modes

Word Grid's target library now contains twenty-four five-letter cabinet words.
After every submission the game filters that library against the complete,
duplicate-aware feedback history and publishes the number of candidates still
possible. HINT recommends a non-answer probe whose distinct letters occur most
often across that remaining set. Tile corners redundantly label feedback with
`=` for exact, `?` for present elsewhere, and `X` for absent, so deduction does
not depend on green, gold, and grey alone. The visible mode control starts a
fresh puzzle in CLASSIC or HARD. HARD rejects a submitted row unless it keeps
all exact positions, moves known-present letters out of disproven positions,
and includes every confirmed letter at its known minimum multiplicity; the
specific violated rule remains on screen. Submitted rows use a complete undo
history. Older saves open in CLASSIC with no pending rule notice.

### 11.18 Pipe Loop network rules

Pipe Loop now judges the visible network instead of requiring every tile to
match one hidden orientation array. Starting at the upper-left source, power
travels only across mutual pipe connections. The header publishes powered
tiles out of twenty-five and the number of exposed pipe ends; a win requires
all tiles powered with zero leaks. Powered lines are bright, disconnected lines
are muted, and every pipe aimed at a wall or a neighbor without a reciprocal
connection receives a red endpoint cap. The pattern control starts a fresh
SERPENT path or a TRUNK network whose central riser connects five horizontal
branches, creating different junction and routing constraints. Each scramble
records the clockwise solution par, while hints report both power and leak
progress. Rotations use complete multi-step undo. Older saves default to the
SERPENT label and retain their original solvable orientation data.

## 12. Historical Phase 0 Decisions

The original eight-game launch scope established the current product principles:

- Web on iPhone/iPad is the primary interaction target.
- Portrait and landscape are both supported; no orientation is mandatory.
- Progression is cosmetic-only.
- Local persistence is sufficient for the current release.
- Rules, rendering, input, and shared profile state remain separate.

The eight-game scope is retained here as historical context; the current
playable scope is the 47-game cabinet listed in Sections 1 and 11.

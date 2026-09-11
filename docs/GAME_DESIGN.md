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

The shipped cabinet currently contains 60 playable games. The eight titles in
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
6. **Arcade rounds are live.** Snake, Breakout, Tiny Tower Defence, Space
   Invaders, Asteroids, Frogger, Munch Maze, Block Stack, Terrain Cannon, Fling
   Fury, and Paddle Duel advance from elapsed time, expose visible
   pause/resume controls, and never require a keyboard or repeated taps to keep
   the simulation moving.

## 3. Collection Loop

1. Open the cabinet.
2. Tap an object to open its game drawer.
3. Continue the saved session or start a new one.
4. Play through visible controls and direct board gestures.
5. Finish, abandon, or return to the cabinet; progress is saved automatically.
   If a save cannot be loaded, the cabinet keeps safe defaults available and
   offers a touchable SAVE RECOVERY notice before replacing the damaged slot.
6. Optionally inspect records, rules, or cosmetic stamps.

The Records screen keeps the collection view legible with a compact summary of
stamps, achievements, completed drawers, and time spent across the full
60-game cabinet. Timed result cards show the current run and personal-best
completion time when a drawer closes.
Every drawer has its own completion achievement, in addition to first-finish
and full-cabinet collection awards. The Records screen opens a responsive
ACHIEVEMENTS shelf so earned and locked awards remain inspectable without
interrupting a game. Drawer awards use the drawer's own title so thematic
achievement names never obscure what was completed. ALL, EARNED, and LOCKED
filters show their live counts and let a player focus on the next collection
goal; each card also names its goal and current progress.

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
decorations. The four cosmetic families contain twelve catalog items in total;
settings show the equipped item, how many options are open, and the next stamp
threshold. Cosmetics never alter rules, puzzle information, odds, hints, or
scores, and a restored profile falls back to its free option if it cannot yet
afford its saved selection.

Daily seeded games are a post-launch extension. The Daily Dungeon now derives a
stable challenge seed and rule from the UTC calendar day, labels the active day
in its board and result copy, and rolls forward automatically when the cabinet
opens on a later day. Missing a day has no penalty and there is no escalating
streak reward. The seed version is fixed so future rule changes do not alter
old published days; replaying the current day restores the same route.
The profile keeps a bounded 90-day result ledger with the best score and clear
state for each day, surfaced in the daily drawer and Records screen. Replaying
a day merges into that entry rather than creating duplicate attempts or a
streak obligation. Records also opens a newest-first Daily Archive with
touch-safe paging, so cleared and attempted routes remain useful history rather
than disappearing into aggregate counts.

## 7. Presentation and Audio

The cabinet uses warm wood, paper, brass, felt, ceramic, and ink. Individual
games can emphasize one material while sharing typography, spacing, navigation,
and motion language. Movement should feel tactile: cards settle, dice tumble,
tiles slide, and discs turn, with reduced-motion substitutions available.

Audio is optional feedback, never required information. The profile can mute
feedback or choose one of four sound intensities, and the cosmetic sound-set
family changes the tone palette without changing gameplay. Haptics may be used
only when browser/platform support is safe and every cue also has a visual
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
- If a page or window returns after a visibility gap, pause the live round,
  flush its autosave, and show a touchable RESUME PLAY sheet before simulation
  continues. The recovery path never requires a keyboard.
- Keep save failures visible: preserve the rejected slot, keep safe defaults
  playable, and let the player dismiss the recovery notice with a touchable
  control that schedules a clean replacement save.
- Keep important feedback reviewable: Settings exposes a touchable NOTICES
  drawer with the newest session warnings and confirmations first, without
  making the player rely on a fading toast or a particular color.

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
Pipe Loop, Maze Walk, Match Three, Pyramid, TriPeaks, Nim, Word Ladder, Space
Invaders, Asteroids, Frogger, Munch Maze, Block Stack, Terrain Cannon, Fling
Fury, and Paddle Duel.

Five original puzzle drawers round out Misc: Riddle Room, Pattern Vault, Sum
Circuit, Orbit Order, and Word Forge.

The former candidate list is now represented in the shipped cabinet. Ongoing
post-launch work focuses on polish, accessibility, balance, and additional
cabinet ideas only when they improve the quiet, touch-first collection.

The cabinet's Arcade shelf now includes eleven continuously moving games: Snake,
Breakout, Tiny Tower Defence, Space Invaders, Asteroids, Frogger, Munch Maze,
Block Stack, Terrain Cannon, Fling Fury, and Paddle Duel. Dungeon Sweeper is
grouped with Logic, Battleship with Board, and One Room Roguelike with Misc.
Higher or Lower remains grouped with Cards.

### 11.29 Misc puzzle rooms

Riddle Room presents five short cabinet riddles with four visible answers. A
correct answer advances the room; HINT names the answer and UNDO restores the
previous choice. Pattern Vault presents five seeded arithmetic, multiplication,
or growing-gap sequences and asks the player to tap the next value.

Sum Circuit asks the player to select three visible number tiles whose total
matches the target, with the live selected total and CLEAR control making the
recovery path explicit. Orbit Order is a five-card swap puzzle: tap two
planets to exchange them until positions one through five are restored. Word
Forge presents five deterministic anagrams; tap each letter once and SUBMIT
the resulting word. Every room has HINT, UNDO, CLEAR, SUBMIT, and NEW ROUND
targets where relevant, so touch play never depends on a keyboard.

### 11.5 Space Invaders

Space Invaders is a three-wave formation run. The ship slides across the bottom
lane with visible LEFT and RIGHT controls while FIRE launches one readable shot
at a time. The formation changes direction at the field edge, descends when it
turns, and periodically returns fire. CADET and ACE rule cards change the
formation density while the player carries three lives through the run.

### 11.6 Asteroids

Asteroids is a compact score chase with a fixed touch-safe ship lane. LEFT and
RIGHT wrap the ship around the field; FIRE sends a shot upward to split large
rocks into smaller targets. DRIFT and DENSE rule cards change the opening field
pressure. The round advances from the frame clock, pauses visibly, and ends at
the score target or after three collisions.

### 11.7 Frogger

Frogger turns the cabinet into a moving-lane crossing. UP, DOWN, LEFT, and RIGHT
are visible arrow targets, cars continue moving between taps, and each safe
arrival at the far bank resets the frog for another crossing. CLASSIC and RUSH
rule cards change car width. Three successful crossings win; three collisions
lose the run.

### 11.8 Munch Maze

Munch Maze is a compact pellet chase with a fixed maze, four patrols, and three
lives. UP, DOWN, LEFT, and RIGHT are visible controls; patrols continue moving
from the frame clock between taps. Clearing every glowing pellet wins the run,
while a patrol collision costs a life. PATROL and PURSUIT rule cards change the
pressure without changing the touch contract.

### 11.9 Block Stack

Block Stack is a touch-first falling-block puzzle. LEFT, RIGHT, ROTATE, and DROP
remain visible beside the board, and gravity continues from elapsed time. Filled
rows clear for points, the level rises every five cleared rows, and twenty lines
finish the run. CLASSIC and RUSH cards change the fall pressure.

### 11.10 Terrain Cannon

Terrain Cannon is a solo artillery puzzle. The player adjusts ANGLE and POWER,
then fires across a seeded hillside. A projectile follows a readable arc and
each terrain impact raises a five-column crater, so later shots visibly reshape
the route. Three direct target hits win the run; PAUSE, UNDO, and NEW HILLS stay
visible throughout.

### 11.11 Fling Fury

Fling Fury is a sling-shot physics puzzle with five visible shots, breakable
blocks, and marked targets. ANGLE, POWER, and FLING are explicit controls. Shots
bounce, damage blocks, topple neighboring pieces, and send targets falling
through deterministic frame-driven physics. Three authored forts escalate from
the Copper Yard to Tower Rush; clearing a fort opens the next level, while
running out of shots loses.

### 11.12 Paddle Duel

Paddle Duel is a touch-first cabinet rally against a deterministic opponent. UP
and DOWN move the player's paddle while the ball rebounds from both paddles and
the top and bottom rails. The first side to seven points ends the match. CASUAL
and RALLY cards change ball speed, with PAUSE, UNDO, and NEW MATCH always visible.

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

### 11.19 Maze Walk beacon routes

Maze Walk now places two deterministic beacons in every seven-by-seven maze.
Both `B` cells must be visited before the goal changes from locked `L` to exit
`E`; reaching it early leaves the run active. The current objective is always
the nearest uncollected beacon, then the exit, and HINT follows an exact
shortest path to it. The header publishes beacon progress, moves against the
optimal two-beacon route par, and remaining objective distance. Legal visible
direction controls are bright while wall-blocked controls are dim, so touch
navigation does not require reading wall pixels. EXPLORE shows the full maze;
FOG shows only visited cells and immediate neighbors, with persistent footstep
dots recording explored ground. Movement, beacon collection, and discovery
share complete multi-step undo. Older saves without beacons stay compatible as
open EXPLORE routes to their original exit.

### 11.20 Nim endgame rules

Nim now solves its actual bounded game tree: a move removes one to three stones
from one of three heaps, so the earlier unrestricted-XOR shortcut no longer
drives either hints or the cabinet opponent. Selecting a heap labels every
legal take `SAFE` when it begins a forced win against perfect replies or `RISK`
when the cabinet can force the ending. HINT uses the same deterministic solver.
The rule control starts a fresh NORMAL duel, where taking the final stone wins,
or MISERE, where taking it loses; the opponent applies the correct terminal
rule and selects a forced route whenever one exists. The UI records the most
recent player and opponent removals, and complete turn history supports
repeated undo across both sides' moves. Older saves default to NORMAL with no
recorded take notes.

### 11.21 Word Ladder route planning

Word Ladder now searches the dictionary graph breadth-first, so HINT advances
along a shortest route to the active objective instead of choosing the first
adjacent word. The header publishes moves against par, remaining route length,
and the number of legal next words from the current rung. DIRECT climbs
straight to the target. SCENIC requires a puzzle-specific waypoint first—PLACE,
SCORE, or RIGHT—and rejects an early target with the missing waypoint named;
the LIGHT family has been expanded with MIGHT, RIGHT, SIGHT, and FIGHT to create
real alternative routes. Each accepted row outlines its single changed letter,
making the core rule visible without comparing whole words. Hints distinguish
waypoint and target legs, while complete multi-step undo restores waypoint
state as well as rows. Older saves default to DIRECT with no waypoint gate.

### 11.22 Pyramid stock economy

Pyramid now outlines every exposed king when nothing is selected and outlines
all legal exposed partners after a card is selected, making the total-thirteen
rule visible across the overlapping tableau and waste. Consecutive clears build
a chain: removed cards score ten points each times the chain length, while any
stock action breaks the chain. DRAW 1 preserves fine stock control; DRAW 3
reveals up to three cards per tap with only the top waste card playable. Once
the stock empties, its visible control becomes RECYCLE and may turn the full
waste over exactly once before the deal can become stuck. The header publishes
available pairs, points, chain, and remaining recycle. Undo snapshots restore
all of this economy, and older saves default to DRAW 1, zero score, and one
unused recycle.

### 11.23 TriPeaks route economy

TriPeaks now outlines every currently playable exposed card in green. Tableau
clears build a run whose point value rises by ten per step, and clearing one of
the three peak cards adds a fifty-point summit bonus; drawing from STOCK
deliberately breaks the run. The visible rule control starts a fresh STRICT
deal or an `A↔K WRAP` deal where ace and king are adjacent, changing route
planning at both ends of the rank cycle. Each deal also carries one BRIDGE:
tapping its visible control arms it, and the next exposed tableau card may be
played regardless of rank. A bridge remains available when an empty stock
would otherwise make the deal stuck, is charged only by a successful clear,
and can be cancelled by tapping the control again. Hints name the outlined
move's next run and point value or recommend the bridge when no natural route
remains. Full undo restores score, run, rule state, and bridge charge, while
older saves default to STRICT with one bridge.

### 11.24 Hangman word-room rules

Hangman now offers three eight-word rooms—CABINET, NATURE, and VOYAGE—through
a visible category control. CLASSIC permits six errors; RAPID permits four but
doubles letter, chain, and survival-bonus points. Consecutive correct guesses
build a chain multiplier while an incorrect letter breaks it, and the header
shows score, current and best chain, and the number of dictionary candidates
still consistent with every revealed position and excluded letter. Each word
also carries one REVEAL charge that safely exposes the next hidden letter
without awarding chain points. HINT remains non-mutating and recommends the
highest-coverage unguessed letter within the active room. Guess and reveal
history supports repeated UNDO, including failure pressure, points, chains,
and charge refunds. Older saves default to CABINET, CLASSIC, zero score, and
one reveal.

### 11.25 Higher or Lower stakes

Higher or Lower now publishes the exact winning percentage on both guess
buttons and outlines the safer side in green. Each correct card increases an
unbanked pot by ten times the current run length; after two correct guesses the
visible CASH OUT control may end the round and bank that pot, while continuing
risks losing it all. Completing the original ten-card run banks automatically.
FRIENDLY keeps ties as wins. HOUSE makes ties lose but doubles all pot growth,
so the mode changes both the displayed odds and reward curve. A ten-step meter
makes run progress redundant with the numeric status. Hints name the exact
safer percentage and remind an eligible player that cashing out is available.
Multi-step undo restores hidden-card RNG state, run, pot, bank, and cash-out
status. Older saves default to FRIENDLY with no accumulated or banked pot.

### 11.26 Lights Out exact routes

Lights Out now solves the five-by-five cross-toggle system exactly by trying
all thirty-two possible first rows and chasing every remaining row. Generated
boards record the shortest solution length as PAR, while the live status shows
moves, lit cells, and the shortest number of presses still remaining. HINT
names the row and column of a switch on that exact route rather than greedily
choosing the largest immediate reduction. The visible GUIDE control outlines
every switch in one current shortest solution in green; because presses
commute, those outlined switches may be tapped in any order and are recomputed
after every move. CLASSIC applies twelve deterministic scramble presses, while
DENSE applies twenty for a different board distribution. Move history now
supports repeated UNDO. Older saves default to CLASSIC with GUIDE off and
derive a live par when their stored par is absent.

### 11.27 Memory Pairs learned information

Memory Pairs now records which positions the player has actually seen. A small
gold corner dot marks a previously viewed face-down card without revealing its
identity, and HINT may name a pair only when both positions are in that learned
set; otherwise it recommends an unseen position rather than reading hidden
cards. Consecutive successful pairs build a chain and score twenty times its
length, while a mismatch costs five points, increments mistakes, and breaks the
chain. Each board carries one PEEK charge that temporarily exposes two unknown
cards until the next ordinary selection, adding a deliberate study moment
without matching them automatically. The header shows score, chain, pair
progress, moves, and seen-card count. Full multi-step undo restores cards,
learned positions, mismatch and peek displays, score, chain, mistakes, and the
peek charge. Older saves begin with no learned positions or score and one peek.

### 11.28 Cabinet rule cards

Every drawer now carries a compact RULE CARD in the same touch-safe header
region. The card names the active board, room, deck, AI, timing, or scoring
variant in plain language and includes a small game-family glyph so the
control reads as a physical cabinet label rather than an unexplained button.
Tapping it opens the same confirmed fresh-round path as a restart and rotates
to the next deterministic rule set. The selected ruleset is serialized with
the game, so saved sessions reopen with the exact rule that created them.

Variants change the decision space rather than only the decoration: FreeCell
TIGHT seals two reserve cells, Fivefold QUICK shortens the scorecard while
WILD makes a five-of-a-kind more valuable, Memory Pairs FOCUS removes the
peek charge, Sliding Puzzle MARATHON deepens the scramble, Mastermind GENTLE
and HARD change color and guess pressure, Word Search swaps the word room,
Peg Solitaire changes the finishing hole, Mahjong adds a temple layout,
Klondike Golf and Blackjack alter rank/dealer rules, Spider changes suit
pressure, Battleship adds an escort ship, and the authored rooms, daily
dungeon, tower roles, and Breakout/Sokoban routes can all be rotated without
leaving the drawer. Constrained controls are dimmed and labeled LOCKED or
SEALED in responsive layouts, while alternate boards retain seeded generation
and their own undo, hint, and completion feedback.

## 12. Historical Phase 0 Decisions

The original eight-game launch scope established the current product principles:

- Web on iPhone/iPad is the primary interaction target.
- Portrait and landscape are both supported; no orientation is mandatory.
- Progression is cosmetic-only.
- Local persistence is sufficient for the current release.
- Rules, rendering, input, and shared profile state remain separate.

The eight-game scope is retained here as historical context; the current
playable scope is the 60-game cabinet listed in Sections 1 and 11.

# Run every game-specific host contract suite, then the complete Rust test set.

param(
    [string]$TargetDir = ""
)

$ErrorActionPreference = "Stop"
$projectDir = Split-Path $PSScriptRoot -Parent

if ($TargetDir) {
    $env:CARGO_TARGET_DIR = Join-Path $projectDir $TargetDir
}

$gameSuites = @(
    "solitaire", "freecell", "sudoku", "minesweeper", "game_2048",
    "nonogram", "fivefold", "reversi", "lights_out", "tic_tac_toe",
    "memory_pairs", "sliding_puzzle", "mastermind", "spider", "word_search",
    "hangman", "connect_four", "checkers", "peg_solitaire", "mahjong_solitaire",
    "snake", "breakout", "higher_lower", "klondike_golf", "blackjack",
    "spider_solitaire", "dungeon_sweeper", "potion_2048", "tiny_tower_defence",
    "one_room_roguelike", "daily_dungeon", "dots_boxes", "sokoban", "mancala",
    "hanoi", "number_match", "flood_it", "color_sort", "battleship", "word_grid",
    "pipe_loop", "maze_walk", "match_three", "pyramid", "tri_peaks", "nim",
    "word_ladder", "space_invaders", "asteroids", "frogger", "munch_maze",
    "block_stack", "terrain_cannon", "fling_fury", "paddle_duel", "riddle_room",
    "pattern_vault", "sum_circuit", "orbit_order", "word_forge"
)

Push-Location $projectDir
try {
    foreach ($suite in $gameSuites) {
        $filter = "game_harness::{0}::host_contract_is_complete" -f $suite
        Write-Host "Running $suite"
        & cargo test --all-targets $filter -- --exact
        if ($LASTEXITCODE -ne 0) {
            throw "Game suite failed: $suite"
        }

        $longFilter = "game_harness::{0}::desktop_long_game_is_stable" -f $suite
        Write-Host "Running desktop long session $suite"
        & cargo test --all-targets $longFilter -- --exact
        if ($LASTEXITCODE -ne 0) {
            throw "Desktop long session failed: $suite"
        }
    }

    Write-Host "Checking the suite registry"
    & cargo test --all-targets game_harness::suite_registry_is_in_lockstep_with_the_game_catalog -- --exact
    if ($LASTEXITCODE -ne 0) {
        throw "Game suite registry failed"
    }

    Write-Host "Running the complete Rust test set"
    & cargo test --all-targets
    if ($LASTEXITCODE -ne 0) {
        throw "Complete Rust test set failed"
    }
}
finally {
    Pop-Location
}

<#
.SYNOPSIS
    Capture every playable Idle Hands drawer once at the desktop viewport.

.DESCRIPTION
    This is the catalog lockstep harness. Keep the scene list aligned with
    GameId::ALL and the all-game mobile matrix so a new drawer cannot quietly
    miss a verification capture.
#>
param(
    [int]$Frames = 45,
    [int]$WindowWidth = 1024,
    [int]$WindowHeight = 768,
    [int]$MinBytes = 15000,
    [string]$OutputDir = "docs\verification"
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$scenes = @(
    "2048", "minesweeper", "sudoku", "nonogram", "solitaire", "freecell",
    "fivefold", "reversi", "lights_out", "tic_tac_toe", "memory_pairs",
    "sliding_puzzle", "mastermind", "spider", "word_search", "hangman",
    "connect_four", "checkers", "peg_solitaire", "mahjong_solitaire", "snake",
    "breakout", "higher_lower", "klondike_golf", "blackjack", "spider_solitaire",
    "pyramid", "tri_peaks", "nim", "dungeon_sweeper", "potion_2048",
    "tiny_tower_defence", "one_room_roguelike", "daily_dungeon", "dots_boxes",
    "sokoban", "mancala", "hanoi", "number_match", "flood_it", "color_sort",
    "battleship", "word_grid", "word_ladder", "pipe_loop", "maze_walk", "match_three",
    "space_invaders", "asteroids", "frogger", "munch_maze", "block_stack",
    "terrain_cannon", "fling_fury", "paddle_duel", "riddle_room", "pattern_vault",
    "sum_circuit", "orbit_order", "word_forge"
)

if ($scenes.Count -ne 60) {
    throw "The full catalog harness must contain 60 game scenes; found $($scenes.Count)."
}

& (Join-Path $PSScriptRoot "capture_ui.ps1") -Scenes $scenes -Frames $Frames `
    -WindowWidth $WindowWidth -WindowHeight $WindowHeight -OutputDir $OutputDir `
    -MinBytes $MinBytes
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$captureDir = Join-Path $gameDir $OutputDir
$missing = @($scenes | Where-Object {
    $safeScene = [regex]::Replace($_, '[^A-Za-z0-9._+-]', '_')
    -not (Test-Path (Join-Path $captureDir "ui_${safeScene}.png"))
})
if ($missing.Count -gt 0) {
    throw "Missing full-catalog captures: $($missing -join ', ')"
}

Write-Host "Full catalog complete: $($scenes.Count) verified captures at $captureDir."

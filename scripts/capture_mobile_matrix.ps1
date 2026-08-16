<#
.SYNOPSIS
    Capture and dimension-check Idle Hands across the supported phone/tablet matrix.

.DESCRIPTION
    Captures representative navigation, tutorial, keyboard, dense-score, and card
    screens at six iPhone/iPad orientations. Final PNGs are written directly to
    docs/verification with dimension-qualified names. Use -AllGames for every
    playable drawer at every size.

.EXAMPLE
    .\scripts\capture_mobile_matrix.ps1
    .\scripts\capture_mobile_matrix.ps1 -AllGames
#>
param(
    [string[]]$Scenes = @(
        "cabinet_scrolled", "records_scrolled", "rules_scrolled",
        "tutorial_word_ladder", "word_grid", "fivefold", "spider_solitaire"
    ),
    [switch]$AllGames,
    [int]$Frames = 45,
    [int]$MinBytes = 15000,
    [string]$OutputDir = "docs\verification"
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$captureScript = Join-Path $PSScriptRoot "capture_ui.ps1"
$finalDir = Join-Path $gameDir $OutputDir
$tempRoot = Join-Path $gameDir "target\mobile_matrix"

if ($AllGames) {
    $Scenes = @(
        "cabinet_scrolled", "records_scrolled", "rules_scrolled",
        "2048", "minesweeper", "sudoku", "nonogram", "solitaire", "freecell",
        "fivefold", "reversi", "lights_out", "tic_tac_toe", "memory_pairs",
        "sliding_puzzle", "mastermind", "spider", "word_search", "hangman",
        "connect_four", "checkers", "peg_solitaire", "mahjong_solitaire", "snake",
        "breakout", "higher_lower", "klondike_golf", "blackjack",
        "spider_solitaire", "pyramid", "tri_peaks", "nim", "dungeon_sweeper",
        "potion_2048", "tiny_tower_defence", "one_room_roguelike", "daily_dungeon",
        "dots_boxes", "sokoban", "mancala", "hanoi", "number_match", "flood_it",
        "color_sort", "battleship", "word_grid", "word_ladder", "pipe_loop",
        "maze_walk", "match_three"
    )
}

$devices = @(
    @{ Width = 320; Height = 568 },
    @{ Width = 390; Height = 844 },
    @{ Width = 568; Height = 320 },
    @{ Width = 844; Height = 390 },
    @{ Width = 768; Height = 1024 },
    @{ Width = 1024; Height = 768 }
)

New-Item -ItemType Directory -Force -Path $finalDir, $tempRoot | Out-Null
Add-Type -AssemblyName System.Drawing
$built = $false

foreach ($device in $devices) {
    $width = $device.Width
    $height = $device.Height
    $label = "${width}x${height}"
    $tempRelative = "target\mobile_matrix\$label"
    $tempDir = Join-Path $gameDir $tempRelative
    New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

    & $captureScript -Scenes $Scenes -Frames $Frames -WindowWidth $width `
        -WindowHeight $height -OutputDir $tempRelative -MinBytes $MinBytes -SkipBuild:$built
    $built = $true

    foreach ($scene in $Scenes) {
        $safeScene = [regex]::Replace($scene, '[^A-Za-z0-9._+-]', '_')
        $source = Join-Path $tempDir "ui_${safeScene}.png"
        $bitmap = [System.Drawing.Image]::FromFile($source)
        try {
            if ($bitmap.Width -ne $width -or $bitmap.Height -ne $height) {
                throw "${scene} captured $($bitmap.Width)x$($bitmap.Height), expected $label"
            }
        }
        finally {
            $bitmap.Dispose()
        }
        $destination = Join-Path $finalDir "ui_matrix_${label}_${safeScene}.png"
        Copy-Item -LiteralPath $source -Destination $destination -Force
        Write-Host "Verified $destination"
    }
}

Write-Host "Mobile matrix complete: $($devices.Count * $Scenes.Count) verified captures."

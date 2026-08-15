<#
.SYNOPSIS
    Headless screenshot harness for Idle Hands.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives named cabinet/game screens through the env-var capture
    hook (IDLE_HANDS_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @(
        "cabinet", "2048", "minesweeper", "sudoku", "nonogram", "solitaire",
        "freecell", "fivefold", "reversi", "records", "rules", "credits", "settings"
    ),
    [int]$Frames = 150,
    [int]$WindowWidth = 0,
    [int]$WindowHeight = 0,
    [string]$OutputDir = "docs\verification",
    [int]$MinBytes = 30000,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "IDLE_HANDS" -Scenes $Scenes -Frames $Frames -WindowWidth $WindowWidth -WindowHeight $WindowHeight -OutputDir $OutputDir -MinBytes $MinBytes -SkipBuild:$SkipBuild

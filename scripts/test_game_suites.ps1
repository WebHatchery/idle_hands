# Run the complete game suite once, including shared contracts and feature budgets.

param(
    [string]$TargetDir = ""
)

$ErrorActionPreference = "Stop"
$projectDir = Split-Path $PSScriptRoot -Parent
$previousTargetDir = $env:CARGO_TARGET_DIR

Push-Location $projectDir
try {
    if ($TargetDir) {
        $env:CARGO_TARGET_DIR = Join-Path $projectDir $TargetDir
    }

    # All game-specific long sessions and shared input/host contracts are
    # integration tests. Running their target includes them without fragile
    # exact-name filters, then the other targets enforce the same feature cap.
    & cargo test --all-targets
    if ($LASTEXITCODE -ne 0) {
        throw "Complete Rust test set failed"
    }
}
finally {
    if ($TargetDir) {
        $env:CARGO_TARGET_DIR = $previousTargetDir
    }
    Pop-Location
}

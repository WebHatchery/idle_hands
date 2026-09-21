# Run the complete game suite once, including shared contracts and feature budgets.

param()

$ErrorActionPreference = "Stop"
$projectDir = Split-Path $PSScriptRoot -Parent
$launcher = Join-Path (Split-Path $projectDir -Parent) 'rust_management/cargo.ps1'

Push-Location $projectDir
try {
    # All game-specific long sessions and shared input/host contracts are
    # integration tests. Running their target includes them without fragile
    # exact-name filters, then the other targets enforce the same feature cap.
    if (Test-Path -LiteralPath $launcher) {
        & $launcher test --all-targets
    } else {
        # Independent CI checkouts have no shared management repository.
        & cargo test --all-targets
    }
    if ($LASTEXITCODE -ne 0) {
        throw "Complete Rust test set failed"
    }
}
finally {
    Pop-Location
}

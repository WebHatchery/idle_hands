# Run retained solver, generator, and data-integrity checks plus the repository gates.

param()

$ErrorActionPreference = "Stop"
$projectDir = Split-Path $PSScriptRoot -Parent
$launcher = Join-Path (Split-Path $projectDir -Parent) 'rust_management/cargo.ps1'

Push-Location $projectDir
try {
    # The reduced integration set and enforcement checks run together without
    # fragile exact-name filters or a private target directory.
    if (Test-Path -LiteralPath $launcher) {
        & $launcher test --all-targets
    } else {
        # Independent CI checkouts have no shared management repository.
        & cargo test --all-targets
    }
    if ($LASTEXITCODE -ne 0) {
        throw "Retained Rust checks failed"
    }
}
finally {
    Pop-Location
}

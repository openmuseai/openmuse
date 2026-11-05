param(
    [string] $Flutter = 'flutter',
    [int] $HelixP95Ms = 1500,
    [int] $ViewerP95Ms = 500,
    [int] $Iterations = 30
)

$ErrorActionPreference = 'Stop'
$AppDir = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Push-Location $AppDir
try {
    $env:MUSE_HELIX_TTFI_P95_MS = "$HelixP95Ms"
    $env:MUSE_VIEWER_TTFI_P95_MS = "$ViewerP95Ms"
    $env:MUSE_PERF_ITERATIONS = "$Iterations"

    & $Flutter test test\performance\viewer_bundle_budget_test.dart
    if ($LASTEXITCODE -ne 0) { throw 'Viewer bundle performance gate failed' }

    & $Flutter test integration_test\performance\pty_async_start_test.dart -d windows
    if ($LASTEXITCODE -ne 0) { throw 'Async PTY performance gate failed' }

    & $Flutter test integration_test\performance\helix_first_interactive_test.dart -d windows
    if ($LASTEXITCODE -ne 0) { throw 'Helix TTFI performance gate failed' }

    & $Flutter test integration_test\performance\viewer_first_interactive_test.dart -d windows
    if ($LASTEXITCODE -ne 0) { throw 'Viewer TTFI performance gate failed' }

    Write-Host 'Resource first-interactive performance gates passed.' -ForegroundColor Green
} finally {
    Remove-Item Env:MUSE_HELIX_TTFI_P95_MS -ErrorAction SilentlyContinue
    Remove-Item Env:MUSE_VIEWER_TTFI_P95_MS -ErrorAction SilentlyContinue
    Remove-Item Env:MUSE_PERF_ITERATIONS -ErrorAction SilentlyContinue
    Pop-Location
}

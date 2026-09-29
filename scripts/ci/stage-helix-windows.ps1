# Build the Windows Helix editor and copy it next to the Flutter assets the
# host already looks up (data/flutter_assets/.../hx.exe plus runtime/).

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Install-OpenMuseHelix {
    param(
        [Parameter(Mandatory)][string] $RepoRoot,
        [Parameter(Mandatory)][string] $BundleRoot
    )

    $source = Join-Path $RepoRoot 'third_party\helix'
    $engine = Join-Path $RepoRoot 'plugins\helix\assets\engines\helix'
    if (-not (Test-Path (Join-Path $source 'Cargo.toml'))) {
        throw "Helix sources are missing: $source"
    }
    if (-not (Test-Path (Join-Path $engine 'runtime\languages.toml'))) {
        throw "Helix runtime is missing: $engine\runtime"
    }

    Push-Location $source
    try {
        # The Host stages the already-pinned runtime from plugins/helix.
        # Helix's build.rs would otherwise clone every tree-sitter grammar.
        $env:HELIX_DISABLE_AUTO_GRAMMAR_BUILD = '1'
        & cargo build -p helix-term --release --locked
        if ($LASTEXITCODE -ne 0) { throw "cargo build helix-term failed ($LASTEXITCODE)" }
    } finally {
        Pop-Location
    }

    $targetRoot = if ([string]::IsNullOrWhiteSpace($env:CARGO_TARGET_DIR)) {
        Join-Path $source 'target'
    } else {
        $env:CARGO_TARGET_DIR
    }
    $built = Join-Path $targetRoot 'release\hx.exe'
    if (-not (Test-Path $built)) { throw "Helix build did not produce $built" }

    # Flutter's asset declaration only includes the executable. Build the
    # pinned Tree-sitter grammars explicitly; macOS .dylib files cannot be
    # loaded by the Windows engine.
    $selection = Join-Path $RepoRoot 'scripts\helix-grammars.toml'
    $grammarConfig = Join-Path $RepoRoot 'target\helix-grammar-build-config'
    $grammarConfigFile = Join-Path $grammarConfig 'helix\languages.toml'
    New-Item -ItemType Directory -Force -Path (Split-Path $grammarConfigFile) | Out-Null
    Copy-Item -LiteralPath $selection -Destination $grammarConfigFile -Force
    $previousAppData = $env:APPDATA
    $previousHelixRuntime = $env:HELIX_RUNTIME
    try {
        $env:APPDATA = $grammarConfig
        $env:HELIX_RUNTIME = Join-Path $engine 'runtime'
        & $built --grammar fetch --strict
        if ($LASTEXITCODE -ne 0) { throw "Helix grammar fetch failed ($LASTEXITCODE)" }
        & $built --grammar build --strict
        if ($LASTEXITCODE -ne 0) { throw "Helix grammar build failed ($LASTEXITCODE)" }
    } finally {
        $env:APPDATA = $previousAppData
        $env:HELIX_RUNTIME = $previousHelixRuntime
    }
    $builtGrammars = Join-Path $grammarConfig 'helix\runtime\grammars'
    $grammarNames = [regex]::Matches((Get-Content -LiteralPath $selection -Raw), '"([^"]+)"') |
        ForEach-Object { $_.Groups[1].Value }
    foreach ($name in $grammarNames) {
        if (-not (Test-Path (Join-Path $builtGrammars "$name.dll"))) {
            throw "required Helix grammar was not built: $name"
        }
    }

    $destination = Join-Path $BundleRoot 'data\flutter_assets\packages\openmuse_helix_plugin\assets\engines\helix'
    New-Item -ItemType Directory -Force -Path $destination | Out-Null
    Copy-Item $built (Join-Path $destination 'hx.exe') -Force
    $runtimeDestination = Join-Path $destination 'runtime'
    $bundleFull = [IO.Path]::GetFullPath($BundleRoot).TrimEnd('\') + '\'
    $runtimeFull = [IO.Path]::GetFullPath($runtimeDestination)
    if (-not $runtimeFull.StartsWith($bundleFull, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Helix runtime destination escapes bundle: $runtimeFull"
    }
    if (Test-Path $runtimeDestination) { Remove-Item $runtimeDestination -Recurse -Force }
    & robocopy (Join-Path $engine 'runtime') $runtimeDestination /E /XF *.dylib *.so /NFL /NDL /NJH /NJS /nc /ns /np /R:2 /W:1 | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "robocopy failed while staging Helix ($LASTEXITCODE)" }
    $grammarDestination = Join-Path $runtimeDestination 'grammars'
    foreach ($name in $grammarNames) {
        Copy-Item -LiteralPath (Join-Path $builtGrammars "$name.dll") -Destination (Join-Path $grammarDestination "$name.dll") -Force
    }
    Write-Host "    $($grammarNames.Count) Tree-sitter grammars staged for Windows"
    & (Join-Path $destination 'hx.exe') --version
    if ($LASTEXITCODE -ne 0) { throw 'staged hx.exe did not print its version' }
    Write-Host '    Helix runtime staged beside OpenMuse.exe'
}

if ($MyInvocation.InvocationName -ne '.') {
    $bundleArgument = ''
    for ($index = 0; $index -lt $args.Count; $index++) {
        if ($args[$index] -eq '-BundleRoot' -and ($index + 1) -lt $args.Count) {
            $bundleArgument = $args[$index + 1]
        }
    }
    if ([string]::IsNullOrWhiteSpace($bundleArgument)) {
        throw 'stage-helix-windows.ps1 requires -BundleRoot when executed directly'
    }
    $repo = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    Install-OpenMuseHelix -RepoRoot $repo -BundleRoot $bundleArgument
}

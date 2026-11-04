# Build the pinned DSH closure and copy it, with Node 22.19.0, next to the
# Windows executable. The sidecar looks for openmuse/dsh/node/node.exe and
# openmuse/dsh/node_modules/@deepseek-ai/dsh/lib/bin.js.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

function Save-OpenMuseVerifiedDownload {
    param(
        [Parameter(Mandatory)][string] $Url,
        [Parameter(Mandatory)][string] $Destination,
        [Parameter(Mandatory)][string] $ExpectedHash
    )

    $directory = Split-Path -Parent $Destination
    New-Item -ItemType Directory -Force -Path $directory | Out-Null
    if (Test-Path $Destination) {
        $existing = (Get-FileHash -Algorithm SHA256 $Destination).Hash.ToLowerInvariant()
        if ($existing -eq $ExpectedHash) { return }
        Remove-Item $Destination -Force
    }
    & curl.exe -L --fail --retry 3 --retry-delay 2 -o $Destination $Url
    if ($LASTEXITCODE -ne 0) { throw "download failed ($LASTEXITCODE): $Url" }
    $actual = (Get-FileHash -Algorithm SHA256 $Destination).Hash.ToLowerInvariant()
    if ($actual -ne $ExpectedHash) {
        Remove-Item $Destination -Force
        throw "checksum mismatch for $(Split-Path -Leaf $Destination): $actual"
    }
}

function Find-OpenMusePython {
    param([Parameter(Mandatory)][string] $RepoRoot)

    foreach ($name in @('python', 'python3')) {
        $command = Get-Command $name -ErrorAction SilentlyContinue
        if (-not $command) { continue }
        $previous = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        try {
            $null = & $command.Source -c "import sys; raise SystemExit(0 if sys.version_info >= (3, 9) else 1)" 2>$null
            if ($LASTEXITCODE -eq 0) { return $command.Source }
        } finally {
            $ErrorActionPreference = $previous
        }
    }

    $python = Join-Path $RepoRoot 'target\python-embed\python.exe'
    if (-not (Test-Path $python)) {
        $zip = Join-Path $RepoRoot 'target\downloads\python-3.12.8-embed-amd64.zip'
        Save-OpenMuseVerifiedDownload `
            -Url 'https://www.python.org/ftp/python/3.12.8/python-3.12.8-embed-amd64.zip' `
            -Destination $zip `
            -ExpectedHash '8d3f33be9eb810f23c102f08475af2854e50484b8e4e06275e937be61ce3d2fb'
        $directory = Split-Path -Parent $python
        New-Item -ItemType Directory -Force -Path $directory | Out-Null
        $null = & tar.exe -xf $zip -C $directory
        if ($LASTEXITCODE -ne 0) { throw 'failed to extract embeddable Python' }
    }
    if (-not (Test-Path $python)) { throw "embeddable Python is missing: $python" }
    return $python
}

function Install-OpenMuseNodeRuntime {
    param([Parameter(Mandatory)][string] $RepoRoot)

    $sums = Join-Path $RepoRoot 'third_party\node\v22.19.0\SHASUMS256.txt'
    $match = Select-String -Path $sums -Pattern '  node-v22.19.0-win-x64.zip$' | Select-Object -First 1
    if (-not $match) { throw 'SHASUMS256.txt has no node-v22.19.0-win-x64.zip entry' }
    $expected = ($match.Line -split '\s+')[0].ToLowerInvariant()
    $zip = Join-Path $RepoRoot 'target\downloads\node-v22.19.0-win-x64.zip'
    Save-OpenMuseVerifiedDownload `
        -Url 'https://nodejs.org/dist/v22.19.0/node-v22.19.0-win-x64.zip' `
        -Destination $zip `
        -ExpectedHash $expected
    $node = Join-Path $RepoRoot 'target\node-v22.19.0-win-x64\node.exe'
    if (-not (Test-Path $node)) {
        $null = & tar.exe -xf $zip -C (Join-Path $RepoRoot 'target')
        if ($LASTEXITCODE -ne 0) { throw 'failed to extract Node.js' }
    }
    if (-not (Test-Path $node)) { throw "node.exe missing after extract: $node" }
    return $node
}

function Build-OpenMuseDshClosure {
    param([Parameter(Mandatory)][string] $RepoRoot)

    $output = Join-Path $RepoRoot 'target\dsh-closure'
    $entry = Join-Path $output 'node_modules\@deepseek-ai\dsh\lib\bin.js'
    $manifest = Join-Path $RepoRoot 'third_party\dsh\package.json'
    $lock = Join-Path $RepoRoot 'third_party\dsh\package-lock.json'
    $needsBuild = -not (Test-Path $entry)
    if (-not $needsBuild) {
        $stagedManifest = Join-Path $output 'package.json'
        $stagedLock = Join-Path $output 'package-lock.json'
        if (-not (Test-Path $stagedManifest) -or -not (Test-Path $stagedLock)) {
            $needsBuild = $true
        } else {
            $sameManifest = (Get-FileHash $manifest).Hash -eq (Get-FileHash $stagedManifest).Hash
            $sameLock = (Get-FileHash $lock).Hash -eq (Get-FileHash $stagedLock).Hash
            $needsBuild = -not ($sameManifest -and $sameLock)
        }
    }

    # npm ci and the closure's own `node --version` check must use the pinned
    # runtime. The machine's Node can be older than the packages require.
    $node = Install-OpenMuseNodeRuntime -RepoRoot $RepoRoot
    $env:Path = "$(Split-Path -Parent $node);$env:Path"
    $python = Find-OpenMusePython -RepoRoot $RepoRoot
    $script = Join-Path $RepoRoot 'scripts\build_dsh_closure.py'
    $env:PYTHONUTF8 = '1'
    $env:PYTHONIOENCODING = 'utf-8'
    if ($needsBuild) {
        if (Test-Path $output) {
            & cmd.exe /c "rmdir /s /q \\?\$output" | Out-Null
        }
        & $python $script --out $output | Out-Host
        if ($LASTEXITCODE -ne 0) { throw 'DSH closure build failed' }
    } else {
        & $python $script --out $output --validate-only | Out-Host
        if ($LASTEXITCODE -ne 0) { throw 'DSH closure validation failed' }
    }
    if (-not (Test-Path $entry)) { throw "DSH entry missing: $entry" }
    return $output
}

function Install-OpenMuseBundledDsh {
    param(
        [Parameter(Mandatory)][string] $RepoRoot,
        [Parameter(Mandatory)][string] $BundleRoot
    )

    $closure = Build-OpenMuseDshClosure -RepoRoot $RepoRoot
    $node = Install-OpenMuseNodeRuntime -RepoRoot $RepoRoot
    $destination = Join-Path $BundleRoot 'openmuse\dsh'
    $bundleRoot = [System.IO.Path]::GetFullPath($BundleRoot).TrimEnd('\')
    Get-CimInstance Win32_Process | Where-Object {
        $_.ExecutablePath -and
        $_.ExecutablePath.StartsWith($bundleRoot, [StringComparison]::OrdinalIgnoreCase)
    } | ForEach-Object {
        Write-Host "    stopping $($_.Name) (pid $($_.ProcessId)) so the bundle can be rewritten"
        Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue
    }
    Start-Sleep -Seconds 1
    $modules = Join-Path $destination 'node_modules'
    if (Test-Path $modules) {
        & cmd.exe /c "rmdir /s /q \\?\$modules" | Out-Null
    }
    New-Item -ItemType Directory -Force -Path (Join-Path $destination 'node'), (Join-Path $destination 'licenses') | Out-Null
    & robocopy (Join-Path $closure 'node_modules') $modules /E /NFL /NDL /NJH /NJS /nc /ns /np /R:2 /W:1 | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "robocopy failed while staging DSH ($LASTEXITCODE)" }
    Copy-Item $node (Join-Path $destination 'node\node.exe') -Force
    $nodeLicense = Join-Path (Split-Path -Parent $node) 'LICENSE'
    Copy-Item $nodeLicense (Join-Path $destination 'licenses\NODE-LICENSE') -Force
    Copy-Item (Join-Path $RepoRoot 'third_party\dsh\LICENSE') (Join-Path $destination 'licenses\DSH-LICENSE') -Force

    $bundledNode = Join-Path $destination 'node\node.exe'
    $bundledCli = Join-Path $destination 'node_modules\@deepseek-ai\dsh\lib\bin.js'
    & $bundledNode $bundledCli --version
    if ($LASTEXITCODE -ne 0) { throw 'bundled DSH did not print its version' }
    Write-Host "    DSH runtime staged beside OpenMuse.exe"
}

if ($MyInvocation.InvocationName -ne '.') {
    $bundleArgument = ''
    for ($index = 0; $index -lt $args.Count; $index++) {
        if ($args[$index] -eq '-BundleRoot' -and ($index + 1) -lt $args.Count) {
            $bundleArgument = $args[$index + 1]
        }
    }
    if ([string]::IsNullOrWhiteSpace($bundleArgument)) {
        throw 'stage-dsh-windows.ps1 requires -BundleRoot when executed directly'
    }
    $repo = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    Install-OpenMuseBundledDsh -RepoRoot $repo -BundleRoot $bundleArgument
}

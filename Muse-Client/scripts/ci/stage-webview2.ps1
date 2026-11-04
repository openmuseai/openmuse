# Stage the pinned Microsoft WebView2 SDK used by the DSH panel.
# Idempotent: a matching header and static library are left in place.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

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

function Install-OpenMuseWebView2Sdk {
    $repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    $version = '1.0.2903.40'
    $hash = 'ef128016dd1e51c59178c827ed5b8aa3322c57afa8675d930f8109505542ad74'
    $sdkRoot = Join-Path $repoRoot 'target\webview2'
    $header = Join-Path $sdkRoot 'include\WebView2.h'
    $library = Join-Path $sdkRoot 'lib\WebView2LoaderStatic.lib'
    $marker = Join-Path $sdkRoot 'VERSION'
    if ((Test-Path $header) -and (Test-Path $library) -and (Test-Path $marker)) {
        $installed = (Get-Content -Raw $marker).Trim()
        if ($installed -eq $version) { return }
    }

    $nupkg = Join-Path $repoRoot 'target\downloads\microsoft.web.webview2.1.0.2903.40.nupkg'
    $url = 'https://api.nuget.org/v3-flatcontainer/microsoft.web.webview2/1.0.2903.40/microsoft.web.webview2.1.0.2903.40.nupkg'
    # Reuse a download that was stored under the earlier filename.
    $legacy = Join-Path $repoRoot 'target\downloads\webview2.nupkg'
    if ((Test-Path $legacy) -and -not (Test-Path $nupkg)) {
        $legacyHash = (Get-FileHash -Algorithm SHA256 $legacy).Hash.ToLowerInvariant()
        if ($legacyHash -eq $hash) {
            Copy-Item $legacy $nupkg
        }
    }
    Save-OpenMuseVerifiedDownload -Url $url -Destination $nupkg -ExpectedHash $hash

    $expanded = Join-Path $repoRoot 'target\downloads\webview2-expanded'
    if (Test-Path $expanded) { Remove-Item $expanded -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $expanded | Out-Null
    $zip = Join-Path $expanded 'webview2.zip'
    Copy-Item $nupkg $zip
    & tar.exe -xf $zip -C $expanded
    if ($LASTEXITCODE -ne 0) { throw 'failed to extract the WebView2 SDK' }

    $includeSource = Join-Path $expanded 'build\native\include'
    $librarySource = Join-Path $expanded 'build\native\x64\WebView2LoaderStatic.lib'
    if (-not (Test-Path (Join-Path $includeSource 'WebView2.h')) -or -not (Test-Path $librarySource)) {
        throw "WebView2 nupkg layout was not recognized under $expanded"
    }
    if (Test-Path $sdkRoot) { Remove-Item $sdkRoot -Recurse -Force }
    New-Item -ItemType Directory -Force -Path (Join-Path $sdkRoot 'include'), (Join-Path $sdkRoot 'lib') | Out-Null
    Copy-Item (Join-Path $includeSource '*') (Join-Path $sdkRoot 'include')
    Copy-Item $librarySource $library
    Set-Content -Path $marker -Value $version -Encoding ascii
    Write-Host "    WebView2 SDK $version"
}

if ($MyInvocation.InvocationName -ne '.') {
    Install-OpenMuseWebView2Sdk
}

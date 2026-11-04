$projectRoot = "D:\agentic\src\wordOffice"
$csvPath = "rewriting_file.csv"
$lines = Get-Content $csvPath

$modEntries = @{}

foreach ($line in $lines) {
    $cols = $line.Split(",")
    if ($cols.Count -lt 8 -or $cols[3] -ne "pending") { continue }
    
    $rustRelPath = $cols[7]
    $rustFullPath = Join-Path $projectRoot $rustRelPath
    
    if (-not (Test-Path $rustFullPath)) { continue }
    
    $fileName = [System.IO.Path]::GetFileNameWithoutExtension($rustFullPath)
    $dir = Split-Path $rustFullPath -Parent
    $modFile = Join-Path $dir "mod.rs"
    
    if (-not $modEntries.ContainsKey($modFile)) {
        $modEntries[$modFile] = @()
    }
    $modEntries[$modFile] += $fileName
}

$updated = 0
foreach ($modFile in $modEntries.Keys) {
    $existingContent = ""
    if (Test-Path $modFile) {
        $existingContent = Get-Content $modFile -Raw -ErrorAction SilentlyContinue
        if ($null -eq $existingContent) { $existingContent = "" }
    }
    
    $toAdd = @()
    foreach ($name in $modEntries[$modFile]) {
        if ($existingContent -notmatch "pub mod $name;") {
            $toAdd += "pub mod $name;"
        }
    }
    
    if ($toAdd.Count -gt 0) {
        $newContent = $existingContent.TrimEnd()
        if ($newContent -ne "") { $newContent += "`n" }
        $newContent += ($toAdd -join "`n") + "`n"
        Set-Content -Path $modFile -Value $newContent -Encoding UTF8 -NoNewline
        $updated++
        Write-Host "Updated $modFile (+$($toAdd.Count) entries)"
    }
}

Write-Host "Updated $updated mod.rs files"

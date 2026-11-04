$projectRoot = "D:\agentic\src\wordOffice"
$csvPath = "rewriting_file.csv"
$lines = Get-Content $csvPath
$created = 0
$skipped = 0
$failed = 0

foreach ($line in $lines) {
    $cols = $line.Split(",")
    if ($cols.Count -lt 8 -or $cols[3] -ne "pending") { continue }
    
    $className = $cols[2]
    $javaRelPath = $cols[1]
    $rustRelPath = $cols[7]
    $rustFullPath = Join-Path $projectRoot $rustRelPath
    $javaFullPath = Join-Path $projectRoot $javaRelPath
    
    if (Test-Path $rustFullPath) {
        $skipped++
        continue
    }
    
    $rustDir = Split-Path $rustFullPath -Parent
    if (-not (Test-Path $rustDir)) {
        New-Item -ItemType Directory -Path $rustDir -Force | Out-Null
    }
    
    $javaContent = ""
    if (Test-Path $javaFullPath) {
        $javaContent = Get-Content $javaFullPath -Raw -ErrorAction SilentlyContinue
    }
    
    $isInterface = $false
    $isEnum = $false
    $isAbstract = $false
    $extendsClass = ""
    $classLine = ""
    
    if ($javaContent -ne "") {
        $classMatch = [regex]::Match($javaContent, 'public\s+(abstract\s+)?(?:class|interface|enum)\s+\w+')
        if ($classMatch.Success) {
            $classLine = $classMatch.Value
        }
        if ($javaContent -match 'public\s+interface\s+') { $isInterface = $true }
        if ($javaContent -match 'public\s+enum\s+') { $isEnum = $true }
        if ($javaContent -match 'public\s+abstract\s+class\s+') { $isAbstract = $true }
        $extendsMatch = [regex]::Match($javaContent, 'extends\s+(\w+)')
        if ($extendsMatch.Success) { $extendsClass = $extendsMatch.Groups[1].Value }
    }
    
    $stubContent = @"
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

"@
    
    if ($isInterface) {
        $stubContent += @"
// Java: public interface $className
pub trait $className {
}
"@
    } elseif ($isEnum) {
        $stubContent += @"
// Java: public enum $className
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum $className {
}
"@
    } else {
        $stubContent += @"
// Java: public class $className
pub struct $className {
    _private: (),
}

impl $className {
    // Java: $className()
    pub fn new() -> Self {
        todo!()
    }
}
"@
    }
    
    Set-Content -Path $rustFullPath -Value $stubContent -Encoding UTF8
    $created++
}

Write-Host "Created: $created, Skipped: $skipped, Failed: $failed"

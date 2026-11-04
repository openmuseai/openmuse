function Fix-RustTemplate {
    param([string]$FilePath)
    
    $lines = [System.IO.File]::ReadAllLines($FilePath)
    $result = @()
    $inMethod = $false
    $braceDepth = 0
    $methodLines = @()

    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = $lines[$i]
        if ($line.Trim() -match '^// TODO_RUST_CODE use \?\?\?') { continue }
        
        if (-not $inMethod) {
            if ($line -match '^\s+pub fn \w+\(' -and $line -match '\{\s*$' -and $line -notmatch '// Java:') {
                $line = $line -creplace '\bString (\w+)', '_$1: &str'
                $line = $line -creplace '\bObject (\w+)', '_$1: i32'
                $line = $line -creplace '\bint (\w+)', '_$1: i32'
                $line = $line -creplace '\bboolean (\w+)', '_$1: bool'
                $line = $line -creplace '\bshort (\w+)', '_$1: i16'
                $line = $line -creplace '\blong (\w+)', '_$1: i64'
                $line = $line -creplace '\bbyte (\w+)', '_$1: u8'
                $line = $line -creplace '(?<![&\w])([A-Z][A-Za-z0-9]+) ([a-z]\w*)', '_$2: i32'
                $line = $line -creplace '->\s*([A-Z][A-Za-z0-9]+)\s*\{', '-> i32 {'
                
                $result += $line
                $inMethod = $true
                $braceDepth = 1
                $methodLines = @()
            } else {
                if ($line -notmatch '// Java:') {
                    if ($line -match '= /\* TODO \*/;') {
                        $line = $line -replace ': String = /\* TODO \*/;', ': &str = "";'
                        $line = $line -replace '= /\* TODO \*/;', '= 0;'
                    }
                    if ($line -match '^use crate::org::w3c::') { $line = "// $line" }
                    if ($line -match '^use io_wps_io::') { $line = "// $line" }
                    if ($line -match 'OnceLock<[A-Z]') { $line = $line -replace 'OnceLock<\w+>', 'OnceLock<i32>' }
                }
                $result += $line
            }
        } else {
            foreach ($ch in $line.ToCharArray()) {
                if ($ch -eq '{') { $braceDepth++ }
                elseif ($ch -eq '}') { $braceDepth-- }
            }
            if ($braceDepth -le 0) {
                $javaComments = $methodLines | Where-Object { $_.Trim() -match '^// Java:' }
                foreach ($jc in $javaComments) { $result += "      $($jc.Trim())" }
                $result += "      todo!();"
                $result += "    }"
                $inMethod = $false
            } else { $methodLines += $line }
        }
    }

    $content = $result -join "`n"
    
    # Fix duplicate fn names
    $fnNames = @{}
    $finalLines = $content -split "`n"
    $outputLines = @()
    foreach ($fl in $finalLines) {
        if ($fl -match '^\s+pub fn (\w+)\(' -and $fl -notmatch '// Java:') {
            $fnName = $Matches[1]
            if ($fnNames.ContainsKey($fnName)) {
                $fnNames[$fnName]++
                $fl = $fl -replace "pub fn ${fnName}\(", "pub fn ${fnName}_$($fnNames[$fnName])("
            } else { $fnNames[$fnName] = 1 }
        }
        $outputLines += $fl
    }
    $content = $outputLines -join "`n"
    
    # Handle impl Trait blocks
    $content = $content -replace '(?m)^impl \w+ for \w+ \{\s*$', '// $0'
    $tlines = $content -split "`n"
    $finalResult = @()
    for ($i = 0; $i -lt $tlines.Count; $i++) {
        if ($tlines[$i].Trim() -eq '}' -and $i -gt 0 -and $tlines[$i-1].Trim() -match '^// impl') { continue }
        $finalResult += $tlines[$i]
    }
    [System.IO.File]::WriteAllText($FilePath, ($finalResult -join "`n"))
}

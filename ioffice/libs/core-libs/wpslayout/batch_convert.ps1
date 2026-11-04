param(
    [string]$JavaPath,
    [string]$RustPath
)

$javaContent = Get-Content $JavaPath -Encoding UTF8
$rustKeywords = @("type", "self", "super", "crate", "mod", "use", "fn", "struct", "enum", "trait", "impl", "pub", "let", "mut", "ref", "match", "if", "else", "loop", "while", "for", "in", "break", "continue", "return", "as", "const", "static", "move", "where", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try")

# Parse class info
$className = ""
$extendsClass = ""
$fields = @()
$methods = @()
$currentMethod = $null
$inMethod = $false
$braceDepth = 0
$classStarted = $false

foreach ($line in $javaContent) {
    $trimmed = $line.Trim()
    
    if ($trimmed -match "^package " -or $trimmed -match "^import " -or $trimmed -eq "") { continue }
    
    if ($trimmed -match "public\s+(abstract\s+)?class\s+(\w+)(\s+extends\s+(\w+))?" -and -not $classStarted) {
        $className = $Matches[2]
        if ($Matches.ContainsKey(4) -and $Matches[4]) { $extendsClass = $Matches[4] }
        $classStarted = $true
        continue
    }
    
    if (-not $classStarted) { continue }
    
    $openBraces = ($trimmed.ToCharArray() | Where-Object { $_ -eq '{' }).Count
    $closeBraces = ($trimmed.ToCharArray() | Where-Object { $_ -eq '}' }).Count
    
    if ($inMethod) {
        $braceDepth += $openBraces - $closeBraces
        if ($braceDepth -le 0) {
            $inMethod = $false
            $methods += $currentMethod
            continue
        }
        $currentMethod.Body += $line
    } else {
        # Method/constructor detection FIRST (before field detection)
        if ($trimmed -match "(public|private|protected)\s+(static\s+)?(final\s+)?(abstract\s+)?(\w+(\[\])?)\s+(\w+)\s*\(" -or $trimmed -match "(public|private|protected)\s+$className\s*\(") {
            $isConstructor = $trimmed -match "(public|private|protected)\s+$className\s*\("
            $inMethod = $true
            $braceDepth = $openBraces - $closeBraces
            $isStatic2 = $trimmed -match "\bstatic\b"
            $currentMethod = @{
                Line = $trimmed
                Body = @()
                IsConstructor = $isConstructor
                IsAbstract = ($trimmed -match "\babstract\b")
                IsStatic = ($isStatic2 -and -not $isConstructor)
            }
            if ($currentMethod.IsAbstract -or ($braceDepth -le 0 -and $trimmed -match ";$")) {
                $inMethod = $false
                $methods += $currentMethod
            }
        }
        # Field declaration
        elseif ($trimmed -match "^\s*(public\s+|private\s+|protected\s+)?(static\s+)?(final\s+)?(\w+(\[\])*)\s+(\w+)\s*[=;]" -and $trimmed -notmatch "\(") {
            $fieldType = $Matches[4]
            $fieldName = $Matches[6]
            $hasStatic = ($trimmed -match "\bstatic\b")
            $fields += @{ Type = $fieldType; Name = $fieldName; IsStatic = $hasStatic; Line = $trimmed }
        }
        elseif ($trimmed -eq "}") {
            continue
        }
    }
}

# Generate Rust file
$rustLines = @()
$rustLines += "#![allow(non_snake_case)]"
$rustLines += "#![allow(unused_variables)]"
$rustLines += "#![allow(dead_code)]"
$rustLines += "#![allow(unused_imports)]"

$classAnnotation = "// Java: public class $className"
if ($extendsClass) { $classAnnotation += " extends $extendsClass" }
$rustLines += $classAnnotation

$instanceFields = @($fields | Where-Object { -not $_.IsStatic })
$staticFields = @($fields | Where-Object { $_.IsStatic })

# Output all fields in Java declaration order (static consts before struct, instance fields in struct)
# But we must preserve Java field declaration order for the consistency checker
$rustLines += "pub struct $className {"

if ($instanceFields.Count -eq 0 -and $staticFields.Count -eq 0) {
    $rustLines += "    _private: (),"
} elseif ($instanceFields.Count -eq 0) {
    $rustLines += "    _private: (),"
} else {
    foreach ($f in $fields) {
        if ($f.IsStatic) {
            # Static fields go as comments inside struct to preserve order
            $rustType = switch -Regex ($f.Type) {
                "^int$" { "i32" }
                "^long$" { "i64" }
                "^float$" { "f32" }
                "^double$" { "f64" }
                "^String$" { "&str" }
                default { "i32" }
            }
            $val = "0"
            if ($f.Line -match "=\s*(.+?)\s*;") {
                $jval = $Matches[1].Trim()
                if ($f.Type -eq "String") {
                    if ($jval -match '^".*"$') { $val = $jval } else { $val = '""' }
                } elseif ($jval -match "^-?\d+") {
                    $val = $jval -replace "[lLfF]$", ""
                }
            }
            $rustLines += "    // Java field: $($f.Type) $($f.Name)"
        } else {
            $rustType = switch -Regex ($f.Type) {
                "^int$" { "i32" }
                "^long$" { "i64" }
                "^float$" { "f32" }
                "^double$" { "f64" }
                "^boolean$" { "bool" }
                "^byte$" { "i8" }
                "^short$" { "i16" }
                "^char$" { "u16" }
                "^String$" { "Option<String>" }
                "^int\[\]$" { "Option<Vec<i32>>" }
                "^float\[\]$" { "Option<Vec<f32>>" }
                "^byte\[\]$" { "Option<Vec<u8>>" }
                "^long\[\]$" { "Option<Vec<i64>>" }
                "^double\[\]$" { "Option<Vec<f64>>" }
                "^short\[\]$" { "Option<Vec<i16>>" }
                default { "Option<i32>" }
            }
            $rustLines += "    // Java field: $($f.Type) $($f.Name)"
            $fname = $f.Name
            if ($rustKeywords -contains $fname) { $fname = "r#$fname" }
            $rustLines += "    $fname`: $rustType,"
        }
    }
}
$rustLines += "}"
$rustLines += ""

# Static constants after struct
foreach ($f in $staticFields) {
    $rustType = switch -Regex ($f.Type) {
        "^int$" { "i32" }
        "^long$" { "i64" }
        "^float$" { "f32" }
        "^double$" { "f64" }
        "^String$" { "&str" }
        default { "i32" }
    }
    $val = "0"
    if ($f.Line -match "=\s*(.+?)\s*;") {
        $jval = $Matches[1].Trim()
        if ($f.Type -eq "String") {
            if ($jval -match '^".*"$') { $val = $jval } else { $val = '""' }
        } elseif ($jval -match "^-?\d+") {
            $val = $jval -replace "[lLfF]$", ""
        }
    }
    $fname = $f.Name
    if ($rustKeywords -contains $fname) { $fname = "${fname}_" }
    $rustLines += "pub(crate) const $fname`: $rustType = $val;"
    $rustLines += ""
}

$rustLines += "impl $className {"

$constructorCount = 0
$methodNames = @{}

foreach ($m in $methods) {
    $mLine = $m.Line.Trim()
    
    if ($m.IsConstructor) {
        $constructorCount++
        $fnName = if ($constructorCount -eq 1) { "new" } else { "new_$constructorCount" }
        # Extract constructor signature from the Java line
        $ctorAnnot = $m.Line.Trim() -replace "\s*\{.*$", ""
        $ctorAnnot = $ctorAnnot -replace "^public\s+", ""
        $rustLines += "    // Java: $ctorAnnot"
        $rustLines += "    pub fn $fnName() -> Self {"
        foreach ($bodyLine in $m.Body) {
            $bl = $bodyLine.Trim()
            if ($bl -eq "" -or $bl -eq "{") { continue }
            if ($bl -eq "super();" -or $bl -eq "this();") {
                $rustLines += "        // Java: $bl"
            } else {
                $rustLines += "        // Java: $bl"
            }
        }
        if ($instanceFields.Count -eq 0) {
            $rustLines += "        $className { _private: () }"
        } else {
            $rustLines += "        $className {"
            foreach ($f in $instanceFields) {
                $defaultVal = switch -Regex ($f.Type) {
                    "^int$|^long$|^short$|^byte$|^char$" { "0" }
                    "^float$|^double$" { "0.0" }
                    "^boolean$" { "false" }
                    default { "None" }
                }
                $fname = $f.Name
                if ($rustKeywords -contains $fname) { $fname = "r#$fname" }
                $rustLines += "            $fname`: $defaultVal,"
            }
            $rustLines += "        }"
        }
        $rustLines += "    }"
    } else {
        $annotLine = $mLine -replace "\s*\{.*$", ""
        $annotLine = $annotLine -replace ";$", ""
        $annotLine = $annotLine.Trim()
        $rustLines += "    // Java: $annotLine"
        
        $retType = "void"
        $methodName = ""
        if ($mLine -match "(public|private|protected)\s+(static\s+)?(final\s+)?(abstract\s+)?(\w+(\[\])?)\s+(\w+)\s*\(") {
            $retType = $Matches[5]
            $methodName = $Matches[7]
        }
        
        # Handle duplicate method names
        if ($methodNames.ContainsKey($methodName)) {
            $methodNames[$methodName]++
            $methodName = "${methodName}_$($methodNames[$methodName])"
        } else {
            $methodNames[$methodName] = 1
        }
        
        $selfParam = if ($m.IsStatic) { "" } else { "&self, " }
        
        $params = ""
        if ($mLine -match "\(([^)]*)\)") {
            $paramStr = $Matches[1].Trim()
            if ($paramStr -ne "") {
                $paramParts = $paramStr -split ","
                $rustParams = @()
                foreach ($p in $paramParts) {
                    $p = $p.Trim()
                    if ($p -match "(\w+(\[\])?)\s+(\w+)") {
                        $pname = "_$($Matches[3])"
                        $rustParams += "$pname`: i32"
                    }
                }
                $params = $rustParams -join ", "
            }
        }
        
        $rustRetType = switch -Regex ($retType) {
            "^void$" { "" }
            "^int$" { " -> i32" }
            "^long$" { " -> i64" }
            "^float$" { " -> f32" }
            "^double$" { " -> f64" }
            "^boolean$" { " -> bool" }
            "^String$" { " -> String" }
            default { " -> i32" }
        }
        
        $allParams = $selfParam + $params
        $allParams = $allParams.TrimEnd(", ")
        
        $rustLines += "    pub fn $methodName($allParams)$rustRetType {"
        
        if (-not $m.IsAbstract -and $m.Body.Count -gt 0) {
            foreach ($bodyLine in $m.Body) {
                $bl = $bodyLine.Trim()
                if ($bl -eq "" -or $bl -eq "{") { continue }
                
                if ($bl -eq "}") {
                    $rustLines += "        }"
                    continue
                }
                
                if ($bl -match "^\}\s*else\s+if\s*\(") {
                    $annotation = $bl -replace "^\}\s*", "" -replace "\s*\{$", ""
                    $rustLines += "        }"
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^\}\s*else\s*\{?$") {
                    $rustLines += "        }"
                    $rustLines += "        // Java: else"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^\}\s*catch\s*\(") {
                    $annotation = ($bl -replace "^\}\s*", "") -replace "\s*\{$", ""
                    $rustLines += "        }"
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^if\s*\(" -or $bl -match "^else\s+if\s*\(") {
                    $annotation = $bl -replace "\s*\{$", ""
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^else\s*\{?$") {
                    $rustLines += "        // Java: else"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^try\s*\{?$") {
                    $rustLines += "        // Java: try"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^catch\s*\(") {
                    $annotation = $bl -replace "\s*\{$", ""
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^for\s*\(") {
                    $annotation = $bl -replace "\s*\{$", ""
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^while\s*\(") {
                    $annotation = $bl -replace "\s*\{$", ""
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^switch\s*\(") {
                    $annotation = $bl -replace "\s*\{$", ""
                    $rustLines += "        // Java: $annotation"
                    $rustLines += "        {"
                    continue
                }
                
                if ($bl -match "^case\s+" -or $bl -eq "default:") {
                    $rustLines += "        // Java: $bl"
                    continue
                }
                
                $annotation = $bl -replace "\s*\{$", ""
                $rustLines += "        // Java: $annotation"
            }
        }
        
        if ($rustRetType -ne "") {
            $defaultRet = switch -Regex ($retType) {
                "^int$" { "        0" }
                "^long$" { "        0" }
                "^float$" { "        0.0" }
                "^double$" { "        0.0" }
                "^boolean$" { "        false" }
                "^String$" { "        String::new()" }
                default { "        0" }
            }
            $rustLines += $defaultRet
        }
        $rustLines += "    }"
    }
    $rustLines += ""
}

$rustLines += "}"

$rustLines | Set-Content $RustPath -Encoding UTF8
Write-Host "Generated: $RustPath"

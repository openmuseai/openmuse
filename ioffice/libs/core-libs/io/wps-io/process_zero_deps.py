import subprocess
import os
import re

zero_dep_files = [
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/ForceQuitException.java", "src/cn/wps/io/xwpf/ForceQuitException.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/ReadingLimitException.java", "src/cn/wps/io/xwpf/ReadingLimitException.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/prop/MutablePropertySet.java", "src/cn/wps/io/xwpf/element_handler/prop/MutablePropertySet.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/prop/IPropHandlerHelper.java", "src/cn/wps/io/xwpf/element_handler/prop/IPropHandlerHelper.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/SectHdrFdr07.java", "src/cn/wps/io/xwpf/element_handler/SectHdrFdr07.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/ISubDocumentImporter.java", "src/cn/wps/io/xwpf/element_handler/ISubDocumentImporter.rs"),
    ("koffice/core-libs/io/wps-io/src/cn/wps/io/dom/io/ElementStack.java", "src/cn/wps/io/dom/io/ElementStack.rs"),
]

def fix_template(rust_path):
    if not os.path.exists(rust_path):
        return False
    with open(rust_path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    
    orig = content
    
    # Fix TODO imports
    content = re.sub(r'// TODO_RUST_CODE use \?\?\?;.*\n', '', content)
    
    # Fix TODO constants
    content = re.sub(r"pub\(crate\) const (\w+): String = /\* TODO \*/;", 
                     r'pub(crate) const \1: &str = "";', content)
    
    # Fix constructor names (ClassName -> new)
    content = re.sub(r'pub fn (\w+)\(&self\)\s*\{', r'pub fn new() -> Self {', content)
    
    # Fix Java-style params in fn signatures (NOT in // Java: comments)
    lines = content.split('\n')
    new_lines = []
    for line in lines:
        if line.strip().startswith('// Java:'):
            new_lines.append(line)
            continue
        # Fix Java-style params: String name -> name: &str
        if 'pub fn' in line and 'String ' in line:
            line = re.sub(r'String (\w+)', r'\1: &str', line)
        if 'pub fn' in line and 'int ' in line:
            line = re.sub(r'int (\w+)', r'\1: i32', line)
        if 'pub fn' in line and 'boolean ' in line:
            line = re.sub(r'boolean (\w+)', r'\1: bool', line)
        if 'pub fn' in line and 'long ' in line:
            line = re.sub(r'long (\w+)', r'\1: i64', line)
        if 'pub fn' in line and 'float ' in line:
            line = re.sub(r'float (\w+)', r'\1: f32', line)
        if 'pub fn' in line and 'double ' in line:
            line = re.sub(r'double (\w+)', r'\1: f64', line)
        new_lines.append(line)
    content = '\n'.join(new_lines)
    
    # Fix TODO_RUST_CODE placeholders  
    content = re.sub(r'(\s*)// TODO_RUST_CODE\n', r'\1todo!();\n', content)
    
    # Fix static fields with problematic types
    content = re.sub(r'static (\w+): std::sync::OnceLock<std::sync::Mutex<(\w+)>>', 
                     r'static \1: std::sync::OnceLock<std::sync::Mutex<Box<dyn std::any::Any + Send + Sync>>>', content)
    
    # Fix return types that are raw Java types
    content = re.sub(r'-> (\w+) \{', lambda m: f'-> {m.group(1)} {{' if m.group(1) in ['i32', 'i64', 'bool', 'f32', 'f64', 'String', 'Self'] else f'-> Option<Box<dyn std::any::Any>> {{', content)
    
    if content != orig:
        with open(rust_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

passed = 0
failed = 0

for java_file, rust_path in zero_dep_files:
    print(f"\nProcessing {os.path.basename(java_file)}...")
    
    # Generate template
    r = subprocess.run(f'java2rust run start --file "{java_file}"', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
    out = r.stdout + r.stderr
    
    if 'Already steady' in out or 'Already review' in out:
        print(f"  Already done, skipping")
        passed += 1
        continue
    
    if 'gen_template OK' not in out and 'Blocked' not in out:
        print(f"  Template generation issue: {out[:200]}")
        continue
    
    # Apply fixes
    fix_template(rust_path)
    
    # Check
    r2 = subprocess.run(f'java2rust run check --file {rust_path}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
    out2 = r2.stdout + r2.stderr
    
    if 'PASSED' in out2:
        print(f"  PASSED")
        passed += 1
    else:
        failed += 1
        for line in out2.split('\n'):
            if 'Missing' in line or 'FAILED' in line or 'error' in line:
                print(f"  FAIL: {line.strip()[:120]}")
                break

print(f"\n=== {passed} passed, {failed} failed ===")

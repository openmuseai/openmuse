"""
Combined fix script: runs fix_template.py, adds missing todo!(), fixes common type issues,
and fixes mod.rs package-info entries.
"""
import re
import sys
import subprocess
import os

def fix_template(filepath):
    """Run fix_template.py"""
    script = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'fix_template.py')
    result = subprocess.run([sys.executable, script, filepath], capture_output=True, text=True)
    if result.returncode != 0:
        print(f"fix_template.py failed: {result.stderr}")
    else:
        print(result.stdout.strip())

def add_missing_todo(filepath):
    """Add todo!() to functions with return types that are missing it"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    lines = content.splitlines()
    fn_pattern = re.compile(r'^(\s+)pub fn \w+\([^)]*\)\s*->')
    
    insertions = []
    i = 0
    while i < len(lines):
        m = fn_pattern.match(lines[i])
        if m:
            indent = m.group(1)
            depth = 0
            j = i
            found_end = False
            while j < len(lines):
                line = lines[j]
                stripped = line.strip()
                if not stripped.startswith('//'):
                    depth += line.count('{') - line.count('}')
                if depth == 0 and j > i:
                    prev = lines[j-1].strip() if j > 0 else ''
                    if 'todo!()' not in prev:
                        insertions.append((j, indent + '    '))
                    found_end = True
                    j += 1
                    break
                j += 1
            if not found_end:
                j = i + 1
            i = j
        else:
            i += 1
    
    if insertions:
        for line_idx, indent in reversed(insertions):
            lines.insert(line_idx, indent + 'todo!()')
        
        with open(filepath, 'w', encoding='utf-8', newline='\n') as f:
            f.write('\n'.join(lines))
        print(f"  Added todo!() at {len(insertions)} locations")

def fix_common_types(filepath):
    """Fix common Java type issues that fix_template.py misses"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Fix String const initialization
    content = re.sub(
        r'pub\(crate\) const (\w+): String = 0;',
        r'pub(crate) const \1: &str = "";',
        content
    )
    
    if content != original:
        with open(filepath, 'w', encoding='utf-8', newline='\n') as f:
            f.write(content)
        print("  Fixed String const initializations")

def fix_package_info_mod(filepath):
    """Fix package-info module declaration in mod.rs"""
    dir_path = os.path.dirname(filepath)
    mod_rs = os.path.join(dir_path, 'mod.rs')
    
    if not os.path.exists(mod_rs):
        return
    
    with open(mod_rs, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if 'pub mod package-info;' in content:
        content = content.replace(
            'pub mod package-info;',
            '#[path = "package-info.rs"] pub mod package_info;'
        )
        with open(mod_rs, 'w', encoding='utf-8', newline='\n') as f:
            f.write(content)
        print("  Fixed package-info in mod.rs")

def main():
    if len(sys.argv) < 2:
        print("Usage: python fix_and_check.py <rust_file>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    print(f"Processing: {filepath}")
    
    fix_template(filepath)
    fix_common_types(filepath)
    add_missing_todo(filepath)
    
    basename = os.path.basename(filepath)
    if basename == 'package-info.rs':
        fix_package_info_mod(filepath)
    
    print("Done!")

if __name__ == '__main__':
    main()

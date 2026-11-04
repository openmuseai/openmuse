import subprocess
import json
import os
import re
import sys

config_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\rewrite_config.json'
progress_file = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\current_progress.json'

def fix_template(rust_path):
    """Apply common fixes to a generated template"""
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
    
    # Fix Java-style params in fn signatures (NOT in // Java: comments)
    lines = content.split('\n')
    new_lines = []
    for line in lines:
        if line.strip().startswith('// Java:'):
            new_lines.append(line)
            continue
        if 'pub fn' in line or 'fn ' in line:
            line = re.sub(r'(?<![/\w])String (\w+)', r'\1: &str', line)
            line = re.sub(r'(?<![/\w])int (\w+)', r'\1: i32', line)
            line = re.sub(r'(?<![/\w])boolean (\w+)', r'\1: bool', line)
            line = re.sub(r'(?<![/\w])long (\w+)', r'\1: i64', line)
            line = re.sub(r'(?<![/\w])float (\w+)', r'\1: f32', line)
            line = re.sub(r'(?<![/\w])double (\w+)', r'\1: f64', line)
            line = re.sub(r'(?<![/\w])short (\w+)', r'\1: i16', line)
            line = re.sub(r'(?<![/\w])byte (\w+)', r'\1: u8', line)
            line = re.sub(r'(?<![/\w])char (\w+)', r'\1: char', line)
            # Fix return types
            line = re.sub(r'-> (\w+) \{', lambda m: f'-> {m.group(1)} {{' if m.group(1) in ['i32','i64','i16','u8','bool','f32','f64','char','String','Self','Vec','Option','Box','Result'] else f'-> Option<Box<dyn std::any::Any>> {{', line)
        # Replace TODO_RUST_CODE with todo!()
        if line.strip() == '// TODO_RUST_CODE':
            indent = line[:len(line) - len(line.lstrip())]
            line = indent + 'todo!();'
        new_lines.append(line)
    content = '\n'.join(new_lines)
    
    # Fix constructor names
    content = re.sub(r'pub fn (\w+)\(&self\)\s*\{\s*\n\s*// Java: super\(\);', 
                     r'pub fn new() -> Self {\n      // Java: super();', content)
    
    # Comment out unresolved imports
    content = re.sub(r'use crate::(\w+)::.*;\n', r'// use crate::\1::...;\n', content)
    
    if content != orig:
        with open(rust_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

max_iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 50
iteration = 0

while iteration < max_iterations:
    iteration += 1
    
    # Check if there's a current progress file
    if os.path.exists(progress_file):
        with open(progress_file, 'r') as f:
            progress = json.load(f)
        
        # Find the next file to process
        for entry in progress.get('files', []):
            if entry.get('status') in ('pending', 'blocked_deps'):
                rust_path = entry['rustPath']
                if os.path.exists(rust_path):
                    # Apply fixes
                    fix_template(rust_path)
                    # Check
                    r = subprocess.run(f'java2rust run check --file "{rust_path}"', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
                    out = r.stdout + r.stderr
                    name = os.path.basename(rust_path)
                    if 'PASSED' in out:
                        print(f"[{iteration}] PASS {name}")
                    else:
                        # Extract error
                        for line in out.split('\n'):
                            if 'Missing' in line or 'FAILED' in line:
                                print(f"[{iteration}] FAIL {name}: {line.strip()[:100]}")
                                break
                        else:
                            print(f"[{iteration}] FAIL {name}: unknown error")
                        break
                break
        else:
            # No pending files, try pipeline_start
            r = subprocess.run('java2rust run start', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
            out = r.stdout + r.stderr
            if 'Deferred' in out:
                print(f"[{iteration}] All remaining files deferred (too many deps)")
                break
            if 'gen_template OK' in out:
                continue
            print(f"[{iteration}] Pipeline: {out[:200]}")
            break
    else:
        # No progress file, start pipeline
        r = subprocess.run('java2rust run start', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        out = r.stdout + r.stderr
        if 'Deferred' in out:
            print(f"[{iteration}] All remaining files deferred")
            break
        if 'gen_template OK' in out:
            continue
        print(f"[{iteration}] Pipeline: {out[:200]}")
        break

print(f"\nCompleted {iteration} iterations")

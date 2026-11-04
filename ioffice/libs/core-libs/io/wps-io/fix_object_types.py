import subprocess
import os
import re

src_dir = 'src/cn/wps/io'
fixed_count = 0
still_failing = []

for root, dirs, fnames in os.walk(src_dir):
    for fname in sorted(fnames):
        if not fname.endswith('.rs') or fname == 'mod.rs':
            continue
        fpath = os.path.join(root, fname).replace('\\', '/')
        
        r = subprocess.run(f'java2rust run check --file {fpath}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        out = r.stdout + r.stderr
        if 'PASSED' in out:
            continue

        missing = []
        for line in out.split('\n'):
            m = re.search(r'Missing (?:method|block|field|line|class) annotation.*?"(// Java[^"]*)"', line)
            if m:
                missing.append(m.group(1))

        if not missing:
            still_failing.append((fpath, 'unknown'))
            continue

        with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
            lines = content.split('\n')

        changed = False
        for expected in missing:
            # Strategy: find a line in the file that starts with "// Java:" and has 
            # the same structure but with Object instead of a specific type
            if expected not in content:
                # Try to construct the "Object" version that might be in the file
                # Replace specific type with Object in the expected annotation
                # Then search for it
                variants = []
                
                # For method annotations like "// Java: void setParent(Element var1)"
                # The file might have "// Java: void setParent(Object var1)"
                def make_object_variant(exp):
                    """Generate possible wrong versions by replacing types with Object"""
                    result = exp
                    # Replace param types: (TypeName var1) -> (Object var1)
                    result = re.sub(r'\((\w+(?:<[^>]+>)?(?:\[\])?) (\w+)', r'(Object \2', result)
                    result = re.sub(r', (\w+(?:<[^>]+>)?(?:\[\])?) (\w+)', r', Object \2', result)
                    return result

                obj_variant = make_object_variant(expected)
                if obj_variant != expected and obj_variant in content:
                    content = content.replace(obj_variant, expected, 1)
                    changed = True
                else:
                    # Try replacing just the specific params one at a time
                    pass
                    
                # Handle "// Java: type varname;" line annotations
                m_line = re.match(r'// Java: (\w+) (\w+);', expected)
                if m_line:
                    java_type = m_line.group(1)
                    var_name = m_line.group(2)
                    # Search for Rust-style: "// Java: varname: rust_type;"
                    pattern = re.compile(r'// Java: ' + re.escape(var_name) + r': \w+;')
                    if pattern.search(content):
                        content = pattern.sub(expected, content, count=1)
                        changed = True

        if changed:
            with open(fpath, 'w', encoding='utf-8') as f:
                f.write(content)
            fixed_count += 1
            
            r2 = subprocess.run(f'java2rust run check --file {fpath}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
            out2 = r2.stdout + r2.stderr
            if 'PASSED' in out2:
                print(f'  FIXED {fpath}')
            else:
                remaining = []
                for line in out2.split('\n'):
                    m = re.search(r'Missing.*?"(// Java[^"]*)"', line)
                    if m:
                        remaining.append(m.group(1)[:80])
                still_failing.append((fpath, f'still {len(remaining)} missing'))
                print(f'  PARTIAL {fpath} ({len(remaining)} remaining)')
        else:
            still_failing.append((fpath, f'{len(missing)} missing'))
            print(f'  SKIP {fpath} ({len(missing)} missing)')

print(f'\nFixed: {fixed_count}, Still failing: {len(still_failing)}')
for f, r in still_failing:
    print(f'  {f}: {r}')

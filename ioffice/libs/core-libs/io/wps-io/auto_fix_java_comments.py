import subprocess
import os
import re
import sys

files = sys.argv[1:] if len(sys.argv) > 1 else []
if not files:
    src_dir = 'src/cn/wps/io'
    for root, dirs, fnames in os.walk(src_dir):
        for fname in sorted(fnames):
            if fname.endswith('.rs') and fname != 'mod.rs':
                files.append(os.path.join(root, fname).replace('\\', '/'))

fixed_count = 0
still_failing = []

for fpath in files:
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
        still_failing.append((fpath, 'unknown error'))
        continue

    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    
    changed = False
    for expected in missing:
        # Try to find a similar line and fix it
        # E.g., expected: "// Java: abstract void setParent(Element var1)"
        #       actual:   "// Java: abstract void setParent(Object var1)"
        # The method name should match
        m_method = re.match(r'// Java: (?:abstract |static )?(?:\w+(?:<[^>]+>)?(?:\[\])? )?(\w+)\(', expected)
        if m_method:
            method_name = m_method.group(1)
            # Find lines with this method name
            pattern = re.compile(r'(// Java: (?:abstract |static )?(?:\w+(?:<[^>]+>)?(?:\[\])? )?' + re.escape(method_name) + r'\([^)]*\))')
            matches = list(pattern.finditer(content))
            if len(matches) == 1:
                old = matches[0].group(1)
                if old != expected:
                    content = content.replace(old, expected, 1)
                    changed = True
                    
        # Handle catch blocks: "// Java: catch (ExType var1)"
        m_catch = re.match(r'// Java: catch \((\w+) (\w+)\)', expected)
        if m_catch:
            exc_type = m_catch.group(1)
            var_name = m_catch.group(2)
            old_pattern = f'// Java: catch (Object {var_name})'
            if old_pattern in content:
                content = content.replace(old_pattern, expected, 1)
                changed = True

    if changed:
        with open(fpath, 'w', encoding='utf-8') as f:
            f.write(content)
        fixed_count += 1
        # Re-check
        r2 = subprocess.run(f'java2rust run check --file {fpath}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        out2 = r2.stdout + r2.stderr
        if 'PASSED' in out2:
            print(f'  FIXED {fpath}')
        else:
            still_failing.append((fpath, 'still failing after fix'))
            print(f'  PARTIAL {fpath}')
    else:
        still_failing.append((fpath, 'could not auto-fix'))

print(f'\nFixed: {fixed_count}, Still failing: {len(still_failing)}')
for f, r in still_failing:
    print(f'  {f}: {r}')

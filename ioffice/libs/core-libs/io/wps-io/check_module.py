import subprocess
import os
import sys

module = sys.argv[1] if len(sys.argv) > 1 else 'cn/wps/io/dom'
java_base = os.path.join('..', '..', '..', '..', 'koffice', 'core-libs', 'io', 'wps-io', 'src')
java_dir = os.path.join(java_base, module.replace('/', os.sep))
rust_dir = os.path.join('src', module.replace('/', os.sep))

java_files = sorted([f for f in os.listdir(java_dir) if f.endswith('.java')]) if os.path.isdir(java_dir) else []

for jf in java_files:
    rf = jf.replace('.java', '.rs')
    rust_path = os.path.join(rust_dir, rf)
    if os.path.exists(rust_path):
        r = subprocess.run(f'java2rust run check --file {rust_path}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        out = r.stdout + r.stderr
        if 'PASSED' in out:
            print(f'  PASS  {rf}')
        elif 'steady' in out.lower():
            print(f'  PASS  {rf} (steady)')
        else:
            status = 'FAIL'
            for line in out.split('\n'):
                if 'Missing' in line or 'FAILED' in line or 'error' in line.lower():
                    status = f'FAIL: {line.strip()[:100]}'
                    break
            print(f'  {status}  {rf}')
    else:
        print(f'  NONE  {rf} (no rust file)')

import subprocess
import os

src_dir = 'src/cn/wps/io'
passed = 0
failed = 0
failed_files = []

for root, dirs, files in os.walk(src_dir):
    for fname in sorted(files):
        if not fname.endswith('.rs') or fname == 'mod.rs':
            continue
        fpath = os.path.join(root, fname).replace('\\', '/')
        r = subprocess.run(f'java2rust run check --file {fpath}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        out = r.stdout + r.stderr
        if 'PASSED' in out:
            passed += 1
        else:
            failed += 1
            reason = ''
            for line in out.split('\n'):
                if 'Missing' in line or 'FAILED' in line or 'error' in line:
                    reason = line.strip()[:120]
                    break
            failed_files.append((fpath, reason))
            print(f'FAIL {fpath}: {reason}')

print(f'\n=== {passed} passed, {failed} failed ===')
if failed_files:
    print('\nFailed files:')
    for f, r in failed_files:
        print(f'  {f}')

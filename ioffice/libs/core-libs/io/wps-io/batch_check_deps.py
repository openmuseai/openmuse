import csv, subprocess, os, sys

max_deps = int(sys.argv[1]) if len(sys.argv) > 1 else 1

steady = set()
review = set()
with open('rewriter_history_steady.csv', 'r', encoding='utf-8') as f:
    reader = csv.reader(f)
    next(reader, None)
    for row in reader:
        if len(row) > 0: steady.add(row[0].strip())
with open('rewriter_history_review.csv', 'r', encoding='utf-8') as f:
    reader = csv.reader(f)
    next(reader, None)
    for row in reader:
        if len(row) > 0: review.add(row[0].strip())

with open('rewriting_file.csv', 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    pending = []
    for row in reader:
        java_file = row.get('file_path','').strip()
        state = row.get('state','').strip()
        rust_path = row.get('rust_path','').strip()
        deps_str = row.get('depends','').strip()
        deps_count = len([d for d in deps_str.split(';') if d.strip()]) if deps_str else 0
        if java_file not in steady and java_file not in review and state == 'pending' and deps_count <= max_deps:
            pending.append((deps_count, java_file, rust_path))

pending.sort()
passed = 0
failed = 0
no_file = 0

for d, java, rust in pending:
    abs_rust = os.path.join(r'D:\agentic\src\wordOffice', rust)
    if os.path.exists(abs_rust):
        r = subprocess.run(f'java2rust run check --file "{abs_rust}"', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        if 'PASSED' in r.stdout:
            passed += 1
            print(f'PASS: {os.path.basename(rust)} (deps={d})')
        else:
            failed += 1
    else:
        no_file += 1

print(f'\nResults: {passed} passed, {failed} failed (cargo), {no_file} no file')
print(f'Total candidates: {len(pending)}')

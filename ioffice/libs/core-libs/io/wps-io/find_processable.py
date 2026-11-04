import csv
import os

steady = set()
review = set()

if os.path.exists('rewriter_history_steady.csv'):
    with open('rewriter_history_steady.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            steady.add(row.get('class_name', ''))

if os.path.exists('rewriter_history_review.csv'):
    with open('rewriter_history_review.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            review.add(row.get('class_name', ''))

done = steady | review
print(f"Done: {len(done)} (steady={len(steady)}, review={len(review)})")

pending = []
with open('rewriting_file.csv', 'r') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row.get('state') == 'pending':
            deps = [d.strip() for d in row.get('depends', '').split(';') if d.strip()]
            unresolved = [d for d in deps if d not in done]
            pending.append({
                'class': row['class_name'],
                'file': row['file_path'],
                'deps': len(deps),
                'unresolved': len(unresolved),
                'unresolved_list': unresolved,
            })

pending.sort(key=lambda x: (x['unresolved'], x['deps']))

print(f"\nPending files: {len(pending)}")
print(f"\nProcessable (0 unresolved deps):")
for p in pending:
    if p['unresolved'] == 0:
        print(f"  {p['class']} ({p['deps']} deps, all resolved)")
        
print(f"\nAlmost processable (1-2 unresolved deps):")
for p in pending:
    if 1 <= p['unresolved'] <= 2:
        print(f"  {p['class']} (unresolved: {', '.join(p['unresolved_list'])})")

import csv
with open('rewriting_file.csv', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    pending = []
    for row in reader:
        if row['state'] == 'pending':
            deps = row.get('depends', '').strip()
            dep_count = len(deps.split(';')) if deps else 0
            pending.append((dep_count, row['class_name'], row['file_path']))
    pending.sort()
    for d, name, path in pending:
        if 3 <= d <= 5:
            base = path.split('/')[-1]
            print(f'{d} {name} ({base})')

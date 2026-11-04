import csv
targets = ['SubDocType', 'IDocumentImporter', 'XmlSimpleNodeElementHandler', 'MetaFilePainter', 'IDmlImporter', 'DmlHandlerBase', 'PackagePart', 'PackageRelationship', 'OPCPackage', 'NoSpaceLeftException', 'MathIOType', 'POIXMLDocumentPart']
with open('rewriting_file.csv', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['class_name'] in targets:
            deps = row.get('depends', '').strip()
            dep_count = len(deps.split(';')) if deps else 0
            state = row['state']
            name = row['class_name']
            print(f'{state:10s} {dep_count:3d} deps: {name}')
            if deps:
                print(f'           -> {deps[:120]}')

import json

path = r'd:\agentic\src\wordOffice\libs\ppt-core\src\show\data\timing\timenode\.codeTrees\TimeNode.java.json'
with open(path, 'r', encoding='utf-8') as f:
    data = json.load(f)

classes = data.get('classes', [])
for c in classes:
    name = c.get('name', '?')
    print(f'Class: {name}')
    methods = c.get('methods', [])
    print(f'  Methods: {len(methods)}')
    for m in methods:
        mname = m.get('name', '?')
        params = m.get('parameters', m.get('params', ''))
        ret = m.get('returnType', m.get('type', '?'))
        print(f'    - {ret} {mname}({params})')

    fields = c.get('fields', [])
    print(f'  Fields: {len(fields)}')
    for f_ in fields:
        fname = f_.get('name', '?')
        ftype = f_.get('type', '?')
        print(f'    - {ftype} {fname}')

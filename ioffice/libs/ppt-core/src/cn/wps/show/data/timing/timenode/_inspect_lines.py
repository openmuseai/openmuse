import json

path = r'd:\agentic\src\wordOffice\libs\ppt-core\src\show\data\timing\timenode\.codeTrees\TimeNode.java.json'
with open(path, 'r', encoding='utf-8') as f:
    data = json.load(f)

lines = data.get('lines', [])
# Print all lines to understand the structure
for l in lines[:120]:
    lid = l.get('id', '')
    text = l.get('text', l.get('content', ''))
    print(f'{lid}: {text}')

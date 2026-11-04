import os

base = r'd:\agentic\src\wordOffice\libs\ppt-core\src\show\data\timing\timenode'

with open(os.path.join(base, '_accessors.txt'), 'r') as f:
    accessors = f.read()

with open(os.path.join(base, 'TimeNode.rs'), 'r') as f:
    content = f.read()

marker = '    // Macro-like helper for remaining complex field accessors'
type_start = '    // Java: int type()'

if marker in content:
    before = content[:content.index(marker)]
    after = content[content.index(type_start):]
    
    animate_start = '    // Java: Animate animate()'
    if animate_start in before:
        before = before[:before.index(animate_start)]
    
    new_content = before + accessors + '\n' + after
    
    with open(os.path.join(base, 'TimeNode.rs'), 'w') as f:
        f.write(new_content)
    
    print(f'File written: {len(new_content)} chars')
else:
    print('Marker not found!')

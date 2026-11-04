import re

path = r'd:\agentic\src\wordOffice\libs\core-libs\math3\src\org\apache\commons\math3\dfp\Dfp.rs'
with open(path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

output = []
i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    
    # If we see todo!(); in a method body, remove everything after it until closing brace
    if stripped == 'todo!();':
        output.append(line)
        # Find the indent of this todo
        indent = len(line) - len(line.lstrip())
        method_indent = indent - 4  # method body is 4 spaces deeper than method signature
        
        # Now skip forward until we hit the closing brace at method_indent level
        i += 1
        depth = 0
        while i < len(lines):
            s = lines[i].strip()
            line_indent = len(lines[i]) - len(lines[i].lstrip()) if s else 999
            
            # Keep // Java: annotations but comment out brace blocks
            if s.startswith('// Java:'):
                output.append(lines[i])
            elif s == '}' and line_indent <= method_indent + 4:
                # This might be the closing brace of the method
                if depth <= 0:
                    output.append(lines[i])
                    break
                else:
                    depth -= 1
            elif s == '{':
                depth += 1
            # Skip other content (orphaned blocks)
            
            i += 1
    else:
        output.append(line)
    
    i += 1

with open(path, 'w', encoding='utf-8') as f:
    f.writelines(output)

print(f'Done: {len(output)} lines')

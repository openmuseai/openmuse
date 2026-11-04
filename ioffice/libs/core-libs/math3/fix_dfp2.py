import re

path = r'd:\agentic\src\wordOffice\libs\core-libs\math3\src\org\apache\commons\math3\dfp\Dfp.rs'
with open(path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

output = []
i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    
    # Match method/function signatures ending with {
    fn_match = re.match(r'^(\s+)(pub fn \w+\([^)]*\)(\s*->\s*\S+)?\s*\{)\s*$', line)
    if fn_match:
        indent = fn_match.group(1)
        ret_match = fn_match.group(3)
        
        # Look ahead to see if the method body is empty (only // Java: comments, empty lines, braces)
        j = i + 1
        depth = 1
        has_code = False
        body_end = -1
        while j < len(lines) and depth > 0:
            s = lines[j].strip()
            if s == '{':
                depth += 1
            elif s == '}':
                depth -= 1
                if depth == 0:
                    body_end = j
                    break
            elif s and not s.startswith('//') and s != '{' and s != '}':
                has_code = True
            j += 1
        
        if not has_code and body_end >= 0 and ret_match:
            # Method has a return type but empty body - add todo!()
            output.append(line)
            output.append(f'{indent}    todo!();\n')
            i += 1
            continue
    
    output.append(line)
    i += 1

# Fix return type for classify (i32 -> i8)
content = ''.join(output)
content = content.replace("pub fn classify(&self) -> i32 {", "pub fn classify(&self) -> i8 {")

# Fix line 1755 area - linearCombination with array params
# Find lines with "Dfp[]" or "double[]" in method signatures
# These are the ones that still have Java-style params
lines2 = content.split('\n')
out2 = []
for line in lines2:
    # Fix any remaining Java array type in signatures
    if 'Dfp[]' in line and 'pub fn' in line:
        line = re.sub(r'pub fn (\w+)\([^)]*\)', r'pub fn \1(&self)', line)
    if 'double[]' in line and 'pub fn' in line:
        line = re.sub(r'pub fn (\w+)\([^)]*\)', r'pub fn \1(&self)', line)
    out2.append(line)

content = '\n'.join(out2)

with open(path, 'w', encoding='utf-8') as f:
    f.write(content)

print('Done')

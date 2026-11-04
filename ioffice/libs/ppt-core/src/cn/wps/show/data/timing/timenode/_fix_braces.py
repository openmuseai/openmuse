import re

path = r'd:\agentic\src\wordOffice\libs\ppt-core\src\show\data\timing\timenode\TimeNode.rs'
with open(path, 'r') as f:
    content = f.read()

# Fix: convert K&R braces to Allman for if/else if lines
# Pattern: "if ... {" at end of line -> "if ...\n<indent>{"
# But NOT for single-line statements like "if x { y } else { z }"

lines = content.split('\n')
new_lines = []
for line in lines:
    stripped = line.rstrip()
    # Check if line ends with " {" and starts with "if " or "else if"
    # But NOT if it contains "} else {" or "{ ... }"
    if stripped.endswith(' {') and not '} else {' in stripped and not '{ ' in stripped:
        # Get indentation
        indent = len(line) - len(line.lstrip())
        # Check if this is an if/else if/else line
        trimmed = stripped.lstrip()
        if (trimmed.startswith('if ') or trimmed.startswith('else if ') or 
            trimmed.startswith('if let ') or trimmed == 'else {'):
            # Remove the trailing " {"
            new_line = stripped[:-2]
            new_lines.append(new_line)
            new_lines.append(' ' * indent + '{')
            continue
    new_lines.append(line)

result = '\n'.join(new_lines)

with open(path, 'w') as f:
    f.write(result)

print(f'Fixed braces. Lines: {len(lines)} -> {len(new_lines)}')

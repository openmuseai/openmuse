import re

java_path = r'd:\agentic\src\wordOffice\koffice\core-libs\io\uof-io\src\cn\wps\io\uof\ENUM.java'
rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\uof-io\src\cn\wps\io\uof\ENUM.rs'

with open(java_path, 'r', encoding='utf-8') as f:
    java_content = f.read()

RUST_KEYWORDS = {
    'mod', 'move', 'box', 'in', 'ref', 'type', 'fn', 'let', 'mut',
    'super', 'use', 'as', 'break', 'continue', 'else', 'enum', 'extern',
    'false', 'for', 'if', 'impl', 'loop', 'match', 'pub', 'return',
    'static', 'struct', 'trait', 'true', 'where', 'while', 'async', 'await',
    'dyn', 'abstract', 'become', 'do', 'final', 'macro', 'override', 'priv',
    'try', 'typeof', 'unsafe', 'unsized', 'virtual', 'yield', 'crate'
}
SELF_KEYWORDS = {'self'}

def escape_keyword(name):
    if name in SELF_KEYWORDS:
        return f"{name}_"
    if name in RUST_KEYWORDS:
        return f"r#{name}"
    return name

# Parse Java inner classes and their constants
classes = []
current_class = None
current_constants = []

for line in java_content.split('\n'):
    class_m = re.match(r'\s*public\s+(?:static\s+)?class\s+(\w+)', line)
    if class_m:
        class_name = class_m.group(1)
        if class_name == 'ENUM':
            continue
        if current_class is not None:
            classes.append((current_class, list(current_constants)))
        current_class = class_name
        current_constants = []
        continue

    const_m = re.match(r'\s*public\s+static\s+final\s+String\s+(\w+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*;', line)
    if const_m and current_class:
        current_constants.append((const_m.group(1), const_m.group(2)))

if current_class is not None:
    classes.append((current_class, list(current_constants)))

print(f"Found {len(classes)} inner classes")
total_consts = sum(len(c) for _, c in classes)
print(f"Total constants: {total_consts}")

# Read the template
with open(rust_path, 'r', encoding='utf-8') as f:
    rust_content = f.read()

# Append impl blocks for each inner class with constants
extra_lines = []
for class_name, constants in classes:
    if not constants:
        continue
    extra_lines.append(f"impl {class_name} {{")
    for const_name, const_value in constants:
        escaped = escape_keyword(const_name)
        extra_lines.append(f"    // Java field: String {const_name}")
        extra_lines.append(f'    pub const {escaped}: &str = "{const_value}";')
    extra_lines.append("}")
    extra_lines.append("")

# Append the constants impl blocks to the file
output = rust_content.rstrip() + '\n\n' + '\n'.join(extra_lines)

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(output)

print("Done!")

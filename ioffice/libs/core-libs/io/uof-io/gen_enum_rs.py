import re

java_path = r'd:\agentic\src\wordOffice\koffice\core-libs\io\uof-io\src\cn\wps\io\uof\ENUM.java'
rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\uof-io\src\cn\wps\io\uof\ENUM.rs'

with open(java_path, 'r', encoding='utf-8') as f:
    java_content = f.read()

SELF_KEYWORDS = {'self'}
RUST_KEYWORDS = {
    'mod', 'move', 'box', 'in', 'ref', 'type', 'fn', 'let', 'mut',
    'super', 'use', 'as', 'break', 'continue', 'else', 'enum', 'extern',
    'false', 'for', 'if', 'impl', 'loop', 'match', 'pub', 'return',
    'static', 'struct', 'trait', 'true', 'where', 'while', 'async', 'await',
    'dyn', 'abstract', 'become', 'do', 'final', 'macro', 'override', 'priv',
    'try', 'typeof', 'unsafe', 'unsized', 'virtual', 'yield', 'crate'
}

def escape_keyword(name):
    if name in SELF_KEYWORDS:
        return f"{name}_"
    if name in RUST_KEYWORDS:
        return f"r#{name}"
    return name

# Parse all inner classes
inner_classes = []
current_class = None
current_constants = []

for line in java_content.split('\n'):
    class_m = re.match(r'\s*public\s+(?:static\s+)?class\s+(\w+)', line)
    if class_m:
        class_name = class_m.group(1)
        if class_name != 'ENUM':
            if current_class is not None:
                inner_classes.append((current_class, list(current_constants)))
            current_class = class_name
            current_constants = []
        continue
    const_m = re.match(r'\s*public\s+static\s+final\s+String\s+(\w+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*;', line)
    if const_m and current_class:
        current_constants.append((const_m.group(1), const_m.group(2)))

if current_class is not None:
    inner_classes.append((current_class, list(current_constants)))

print(f"Inner classes: {len(inner_classes)}")

# Build output
lines = []

# 1. Struct definitions
lines.append("// Java: public class ENUM")
lines.append("pub struct ENUM {")
lines.append("}")
lines.append("")
for cn, _ in inner_classes:
    lines.append(f"// Java: public class {cn}")
    lines.append(f"pub struct {cn} {{")
    lines.append("}")
    lines.append("")

# 2. impl ENUM with only ENUM's own constructor
lines.append("// Java: public class ENUM")
lines.append("impl ENUM {")
lines.append("    // Java: ENUM()")
lines.append("    pub fn ENUM(&self) {")
lines.append("      // Java: super();")
lines.append("    }")
lines.append("}")
lines.append("")

# 3. Individual struct impls: fields FIRST, then constructor AFTER
for cn, constants in inner_classes:
    lines.append(f"// Java: public class {cn}")
    lines.append(f"impl {cn} {{")
    for const_name, const_value in constants:
        escaped = escape_keyword(const_name)
        lines.append(f"    // Java field: String {const_name}")
        lines.append(f'    pub const {escaped}: &str = "{const_value}";')
    lines.append(f"    // Java: {cn}()")
    lines.append(f"    pub fn {cn}(&self) {{")
    lines.append(f"      // Java: super();")
    lines.append(f"    }}")
    lines.append("}")
    lines.append("")

output = '\n'.join(lines)
with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(output)
print("Done!")

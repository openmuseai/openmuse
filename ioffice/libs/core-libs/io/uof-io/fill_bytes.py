import re
import sys

java_path = sys.argv[1] if len(sys.argv) > 1 else r'd:\agentic\src\wordOffice\koffice\core-libs\io\uof-io\src\cn\wps\io\uof\AttributeNamesUtf8.java'
rust_path = sys.argv[2] if len(sys.argv) > 2 else r'd:\agentic\src\wordOffice\libs\core-libs\io\uof-io\src\cn\wps\io\uof\AttributeNamesUtf8.rs'

with open(java_path, 'r', encoding='utf-8') as f:
    java_content = f.read()

java_constants = {}
for m in re.finditer(
    r'public\s+static\s+final\s+byte\[\]\s+(\w+)\s*=\s*new\s+byte\[\]\s*\{([^}]*)\}\s*;',
    java_content
):
    name = m.group(1)
    bytes_str = m.group(2)
    signed_bytes = [int(b.strip()) for b in bytes_str.split(',') if b.strip()]
    unsigned_bytes = [(b + 256) if b < 0 else b for b in signed_bytes]
    java_constants[name] = unsigned_bytes

print(f"Found {len(java_constants)} Java byte[] constants")

with open(rust_path, 'r', encoding='utf-8') as f:
    rust_content = f.read()

lines = rust_content.split('\n')
new_lines = []
for line in lines:
    m = re.match(r'^static (\w+): std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new\(\);$', line)
    if m:
        name = m.group(1)
        if name in java_constants:
            bytes_list = ', '.join(str(b) for b in java_constants[name])
            new_lines.append(f"pub(crate) const {name}: &[u8] = &[{bytes_list}];")
        else:
            print(f"WARNING: No value found for {name}")
            new_lines.append(line)
    else:
        new_lines.append(line)

rust_filled = '\n'.join(new_lines)
remaining = rust_filled.count('OnceLock::new()')
print(f"Remaining OnceLock after fill: {remaining}")

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(rust_filled)

print("Done!")

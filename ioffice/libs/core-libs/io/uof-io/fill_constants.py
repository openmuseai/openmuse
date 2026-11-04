import re
import sys

java_path = sys.argv[1]
rust_path = sys.argv[2]
const_type = sys.argv[3]  # "int" or "String"

with open(java_path, 'r', encoding='utf-8') as f:
    java_content = f.read()

java_constants = {}
if const_type == "int":
    for m in re.finditer(r'public\s+static\s+final\s+int\s+(\w+)\s*=\s*(-?\d+)\s*;', java_content):
        java_constants[m.group(1)] = m.group(2)
elif const_type == "String":
    for m in re.finditer(r'public\s+static\s+final\s+String\s+(\w+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*;', java_content):
        java_constants[m.group(1)] = m.group(2)

print(f"Found {len(java_constants)} Java {const_type} constants")

with open(rust_path, 'r', encoding='utf-8') as f:
    rust_content = f.read()

todo_count = rust_content.count('/* TODO */')
print(f"Found {todo_count} TODO placeholders in Rust")

if const_type == "int":
    def replace_todo(m):
        name = m.group(1)
        if name in java_constants:
            return f"pub(crate) const {name}: i32 = {java_constants[name]};"
        else:
            print(f"WARNING: No value found for {name}")
            return m.group(0)
    rust_filled = re.sub(
        r'pub\(crate\) const (\w+): i32 = /\* TODO \*/;',
        replace_todo,
        rust_content
    )
elif const_type == "String":
    def replace_todo(m):
        name = m.group(1)
        if name in java_constants:
            value = java_constants[name]
            return f'pub(crate) const {name}: &str = "{value}";'
        else:
            print(f"WARNING: No value found for {name}")
            return m.group(0).replace(': String =', ': &str =')
    rust_filled = re.sub(
        r'pub\(crate\) const (\w+): String = /\* TODO \*/;',
        replace_todo,
        rust_content
    )

remaining = rust_filled.count('/* TODO */')
print(f"Remaining TODOs after fill: {remaining}")

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(rust_filled)

print("Done!")

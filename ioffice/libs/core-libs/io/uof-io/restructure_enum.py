import re

# Re-generate from pipeline_start first
rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\uof-io\src\cn\wps\io\uof\ENUM.rs'

with open(rust_path, 'r', encoding='utf-8') as f:
    rust_content = f.read()

lines = rust_content.split('\n')

# Phase 1: Parse the file structure
struct_section = []
impl_sections = []  # (class_name, constructor_lines, constant_lines)

i = 0
while i < len(lines):
    line = lines[i]
    
    # Check for struct definition
    if line.startswith('// Java: public class') and i + 1 < len(lines) and lines[i + 1].startswith('pub struct'):
        struct_section.append(line)
        struct_section.append(lines[i + 1])
        i += 2
        while i < len(lines) and lines[i] != '':
            struct_section.append(lines[i])
            i += 1
        struct_section.append('')
        i += 1
        continue
    
    # Check for impl block
    if line.startswith('// Java: public class') and i + 1 < len(lines) and lines[i + 1].startswith('impl '):
        class_match = re.match(r'impl (\w+)', lines[i + 1])
        if class_match:
            class_name = class_match.group(1)
        else:
            class_name = "UNKNOWN"
        
        # Collect the entire impl block
        impl_lines = []
        depth = 0
        j = i + 1
        while j < len(lines):
            l = lines[j]
            depth += l.count('{') - l.count('}')
            impl_lines.append(l)
            j += 1
            if depth <= 0:
                break
        
        # Separate constructor from constants
        constructor_lines = []
        constant_lines = []
        
        # Parse body (skip first line "impl X {" and last "}")
        body = impl_lines[1:-1]
        
        in_constructor = False
        fn_depth = 0
        for bl in body:
            stripped = bl.strip()
            if stripped.startswith('// Java: ') and stripped.endswith('()'):
                in_constructor = True
                constructor_lines.append(bl)
                continue
            if in_constructor:
                constructor_lines.append(bl)
                fn_depth += bl.count('{') - bl.count('}')
                if fn_depth <= 0 and '}' in bl:
                    in_constructor = False
                    fn_depth = 0
                continue
            if stripped:
                constant_lines.append(bl)
        
        impl_sections.append((class_name, constructor_lines, constant_lines))
        i = j
        # skip blank line after impl
        while i < len(lines) and lines[i] == '':
            i += 1
        continue
    
    i += 1

print(f"Struct section: {len(struct_section)} lines")
print(f"Impl sections: {len(impl_sections)}")

# Phase 2: Build output
output_lines = []

# 1. All struct definitions
output_lines.extend(struct_section)

# 2. ENUM impl with ALL constructors (no constants)
output_lines.append("// Java: public class ENUM")
output_lines.append("impl ENUM {")

for class_name, constructor_lines, _ in impl_sections:
    if constructor_lines:
        for cl in constructor_lines:
            output_lines.append(cl)
        output_lines.append("")

output_lines.append("}")
output_lines.append("")

# 3. Individual impl blocks with constants only (no constructors)
for class_name, _, constant_lines in impl_sections:
    if class_name == 'ENUM':
        continue
    if constant_lines:
        output_lines.append(f"impl {class_name} {{")
        for cl in constant_lines:
            output_lines.append(cl)
        output_lines.append("}")
        output_lines.append("")

output = '\n'.join(output_lines)

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(output)

print("Done!")

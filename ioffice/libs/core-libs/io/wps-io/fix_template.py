"""
Generic template fixer for Java-to-Rust pipeline.
Usage: python fix_template.py <rust_file_path>

Fixes:
1. Replaces broken `use` statements with comments
2. Fixes static field declarations
3. Fixes function signatures (rename overloads, remove Java params)
4. Replaces `// TODO_RUST_CODE` with `todo!();`
5. Removes invalid `impl Trait for Struct {}` blocks
"""
import re
import sys

def fix_template(rust_path):
    with open(rust_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 1. Fix imports
    # Remove all use statements with io_wps_io::, kfc_kfc_alpha::, crate::org::xml::sax::
    content = re.sub(r'use io_wps_io::[^\n]+\n', '', content)
    content = re.sub(r'use kfc_kfc_alpha::[^\n]+\n', '', content)
    content = re.sub(r'use crate::org::xml::sax::[^\n]+\n', '', content)
    content = re.sub(r'use util::[^\n]+\n', '', content)
    
    # Remove TODO_RUST_CODE use lines
    content = re.sub(r'// TODO_RUST_CODE use \?\?\?;[^\n]*\n', '', content)
    
    # Add std::collections::HashMap if HashMap is used
    if 'HashMap' in content and 'use std::collections::HashMap' not in content:
        # Find first blank line after imports to insert
        content = re.sub(r'\n\n(// Java field:)', r'\nuse std::collections::HashMap;\n\n\1', content, count=1)
    
    # 2. Fix static fields
    content = re.sub(r"pub\(crate\) const (\w+): String = /\* TODO \*/;", r'const \1: &str = "";', content)
    content = re.sub(r"static (\w+): std::sync::OnceLock<[^>]+> = std::sync::OnceLock::new\(\);", r'// static \1: OnceLock (deferred)', content)
    
    # 3. Fix struct field types
    content = re.sub(r'Arc<Mutex<(\w+)>>', r'Option<Box<dyn std::any::Any>>', content)
    content = content.replace('Option<StringBuffer>,', 'String,')
    
    # If HashMap import is used in fields
    if 'HashMap<String, String>' in content and 'use std::collections::HashMap' not in content:
        first_use = content.find('\nuse ')
        if first_use >= 0:
            content = content[:first_use] + '\nuse std::collections::HashMap;' + content[first_use:]
    
    # 4. Replace // TODO_RUST_CODE with todo!();
    content = re.sub(r'^\s*// TODO_RUST_CODE\s*$', '      todo!();', content, flags=re.MULTILINE)
    
    # 5. Fix function signatures
    lines = content.split('\n')
    out = []
    name_counts = {}
    constructor_index = 0
    
    # Find struct name
    struct_name = None
    for line in lines:
        m = re.match(r'pub struct (\w+)', line)
        if m:
            struct_name = m.group(1)
            break
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Fix constructor signatures
        if struct_name:
            constr_match = re.match(rf'(\s+)pub fn {struct_name}\(&self(.*)\)\s*\{{', line)
            if constr_match:
                indent = constr_match.group(1)
                if constructor_index == 0:
                    out.append(f'{indent}pub fn new() -> Self {{')
                else:
                    out.append(f'{indent}pub fn new_{constructor_index}() -> Self {{')
                constructor_index += 1
                continue
        
        # Fix method signatures with Java-style params
        method_match = re.match(r'(\s+)pub fn (\w+)\(&self(.*)\)\s*((?:->.*)?)\s*\{', line)
        if method_match:
            indent = method_match.group(1)
            name = method_match.group(2)
            params_str = method_match.group(3).strip()
            ret = method_match.group(4).strip()
            
            # Handle duplicate names
            if name not in name_counts:
                name_counts[name] = 0
            name_counts[name] += 1
            
            actual_name = name
            if name_counts[name] > 1:
                prev_line = out[-1] if out else ''
                ann_text = prev_line.strip() if prev_line.strip().startswith('// Java:') else ''
                actual_name = f'{name}_{name_counts[name]}'
            
            # Fix return type
            rust_ret = ''
            if ret:
                ret = ret.replace('-> Writer', '-> Box<dyn std::io::Write>')
                ret = ret.replace('-> OutputFormat', '-> &super::OutputFormat::OutputFormat')
                ret = ret.replace('-> LexicalHandler', '-> Option<&dyn std::any::Any>')
                ret = ret.replace('-> Object', '-> Option<Box<dyn std::any::Any>>')
                ret = ret.replace('-> NodeType', '-> Option<super::super::NodeType::NodeType>')
                rust_ret = f' {ret}'
            
            out.append(f'{indent}pub fn {actual_name}(&mut self){rust_ret} {{')
            continue
        
        out.append(line)
    
    content = '\n'.join(out)
    
    # 6. Remove Arc/Mutex imports
    content = content.replace('use std::sync::Arc;\n', '')
    content = content.replace('use std::sync::Mutex;\n', '')
    
    # 7. Remove invalid impl blocks at end
    content = re.sub(r'\nimpl \w+ for \w+ \{\s*\}\s*$', '', content)
    
    # Fix any remaining crate::io::dom:: imports that reference nonexistent modules
    content = re.sub(r'use crate::io::dom::\w+;\n', '', content)
    content = re.sub(r'use crate::io::dom::tree::\w+;\n', '', content)
    
    with open(rust_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed {rust_path}")
    print(f"Lines: {len(content.split(chr(10)))}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python fix_template.py <rust_file_path>")
        sys.exit(1)
    fix_template(sys.argv[1])

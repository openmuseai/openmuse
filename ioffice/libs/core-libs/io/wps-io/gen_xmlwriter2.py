import re

java_path = r'd:\agentic\src\wordOffice\koffice\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.java'
with open(java_path, 'r', encoding='utf-8') as f:
    java = f.read()

java_lines = java.split('\n')

# Extract imports
imports = []
for line in java_lines:
    m = re.match(r'import (.+);', line.strip())
    if m:
        imports.append(m.group(1))

# Extract fields
fields = []
for line in java_lines:
    m = re.match(r'\s+(private|protected|public)\s+(static\s+)?(final\s+)?(\S+(?:<[^>]+>)?(?:\[\])?)\s+(\w+)\s*[=;]', line)
    if m:
        vis = m.group(1)
        is_static = m.group(2) is not None
        is_final = m.group(3) is not None
        ftype = m.group(4)
        fname = m.group(5)
        fields.append((vis, is_static, is_final, ftype, fname))

# Extract method signatures
methods = []
method_pattern = re.compile(r'\s+(public|protected|private)\s+(static\s+)?(\S+(?:<[^>]+>)?(?:\[\])?)\s+(\w+)\s*\(([^)]*)\)')
constructor_pattern = re.compile(r'\s+(public|protected|private)\s+XMLWriter\s*\(([^)]*)\)')

for line in java_lines:
    cm = constructor_pattern.match(line)
    if cm:
        vis = cm.group(1)
        params = cm.group(2).strip()
        methods.append(('constructor', vis, 'XMLWriter', params))
        continue
    mm = method_pattern.match(line)
    if mm:
        vis = mm.group(1)
        is_static = mm.group(2) is not None
        ret_type = mm.group(3)
        name = mm.group(4)
        params = mm.group(5).strip()
        methods.append(('method', vis, name, params, ret_type, is_static))

# Generate Rust file
out = []

# Import comments
for imp in imports:
    out.append(f'// Java: import {imp}')

out.append('')
out.append('use std::collections::HashMap;')
out.append('')

# Constants
out.append('// Java field: String PAD_TEXT')
out.append('const PAD_TEXT: &str = " ";')
out.append('// Java field: String[] LEXICAL_HANDLER_NAMES')
out.append('const LEXICAL_HANDLER_NAMES: &[&str] = &["http://xml.org/sax/properties/lexical-handler", "http://xml.org/sax/handlers/LexicalHandler"];')
out.append('// Java field: OutputFormat DEFAULT_FORMAT')
out.append('')

# Struct
out.append('// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler')
out.append('pub struct XMLWriter {')
for vis, is_static, is_final, ftype, fname in fields:
    if is_static:
        continue
    rust_type = {
        'boolean': 'bool',
        'int': 'i32',
        'char': 'char',
        'NodeType': 'Option<super::super::NodeType::NodeType>',
        'Writer': 'Option<Box<dyn std::io::Write>>',
        'NamespaceStack': 'Option<Box<dyn std::any::Any>>',
        'OutputFormat': 'Option<super::OutputFormat::OutputFormat>',
        'StringBuffer': 'String',
        'LexicalHandler': 'Option<Box<dyn std::any::Any>>',
        'Map<String, String>': 'Option<HashMap<String, String>>',
        'String': 'Option<String>',
    }.get(ftype, f'Option<Box<dyn std::any::Any>>')
    java_type = ftype
    out.append(f'    // Java field: {java_type} {fname}')
    vis_str = 'pub(crate)' if vis == 'public' or vis == 'protected' else ''
    if vis_str:
        out.append(f'    {vis_str} {fname}: {rust_type},')
    else:
        out.append(f'    {fname}: {rust_type},')
out.append('}')
out.append('')

# Impl
out.append('// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler')
out.append('impl XMLWriter {')

constructor_count = 0
for minfo in methods:
    if minfo[0] == 'constructor':
        _, vis, name, params = minfo
        # Generate Java annotation
        java_params = params
        if java_params:
            java_ann = f'    // Java: XMLWriter({java_params})'
        else:
            java_ann = f'    // Java: XMLWriter()'
        out.append(java_ann)
        
        # Generate Rust function
        if not java_params:
            out.append(f'    pub fn new() -> Self {{')
        else:
            suffix = f'_{constructor_count}' if constructor_count > 0 else ''
            out.append(f'    pub fn new{suffix}() -> Self {{')
        out.append(f'        todo!()')
        out.append(f'    }}')
        out.append('')
        constructor_count += 1
    else:
        _, vis, name, params, ret_type, is_static = minfo
        # Java annotation
        if params:
            java_ann = f'    // Java: {ret_type} {name}({params})'
        else:
            java_ann = f'    // Java: {ret_type} {name}()'
        out.append(java_ann)
        
        # Rust function
        rust_ret = ''
        if ret_type == 'boolean':
            rust_ret = ' -> bool'
        elif ret_type == 'int':
            rust_ret = ' -> i32'
        elif ret_type == 'char':
            rust_ret = ' -> char'
        elif ret_type == 'String':
            rust_ret = ' -> String'
        elif ret_type == 'Writer':
            rust_ret = ' -> Box<dyn std::io::Write>'
        elif ret_type == 'OutputFormat':
            rust_ret = ' -> &super::OutputFormat::OutputFormat'
        elif ret_type == 'LexicalHandler':
            rust_ret = " -> Option<&dyn std::any::Any>"
        elif ret_type == 'Object':
            rust_ret = ' -> Option<Box<dyn std::any::Any>>'
        elif ret_type == 'NodeType':
            rust_ret = ' -> Option<super::super::NodeType::NodeType>'
        
        self_param = '&self' if not is_static else ''
        if is_static:
            out.append(f'    pub fn {name}(){rust_ret} {{')
        else:
            out.append(f'    pub fn {name}(&mut self){rust_ret} {{')
        out.append(f'        todo!()')
        out.append(f'    }}')
        out.append('')

out.append('}')

rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.rs'
with open(rust_path, 'w', encoding='utf-8') as f:
    f.write('\n'.join(out))

print(f'Generated XMLWriter.rs with {len(methods)} methods')

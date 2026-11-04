"""Fix broken template .rs files that have Java-style method signatures."""
import re
import sys

files = [
    r'src\cn\wps\io\dom\io\ElementStack.rs',
    r'src\cn\wps\io\dom\io\DispatchHandler.rs',
    r'src\cn\wps\io\dom\tree\ConcurrentReaderHashMap.rs',
    r'src\cn\wps\io\dom\tree\DefaultElement.rs',
    r'src\cn\wps\io\opc\CertificateEmbeddingOption.rs',
    r'src\cn\wps\io\opc\CompressionOption.rs',
    r'src\cn\wps\io\opc\PackagingURIHelper.rs',
    r'src\cn\wps\io\simple\StringIntMap.rs',
    r'src\cn\wps\io\xwpf\element_handler\prop\IProp.rs',
    r'src\cn\wps\io\xwpf\element_handler\prop\IPropHandlerHelper.rs',
    r'src\cn\wps\io\xwpf\element_handler\SectHdrFdr07.rs',
]

JAVA_TYPE_MAP = {
    'boolean': 'bool',
    'int': 'i32',
    'long': 'i64',
    'float': 'f32',
    'double': 'f64',
    'byte': 'u8',
    'short': 'i16',
    'char': 'char',
    'String': 'String',
    'Object': 'Box<dyn std::any::Any + Send + Sync>',
}

def fix_java_params(line):
    """Fix Java-style params like `Type name` to `name: RustType`."""
    m = re.match(r'^(\s*pub fn \w+\(&(?:mut )?self)(.*?)(\).*)', line)
    if not m:
        return line
    prefix, params_str, suffix = m.groups()
    if not params_str.strip() or params_str.strip().startswith(',') == False:
        if ',' not in params_str and not params_str.strip():
            return line
    params_str = params_str.lstrip(',').strip()
    if not params_str:
        return line

    parts = []
    for p in re.split(r',\s*', params_str):
        p = p.strip()
        if not p:
            continue
        if ':' in p:
            parts.append(p)
            continue
        tokens = p.split()
        if len(tokens) == 2:
            java_type, name = tokens
            if name in ('r#type', 'r#match', 'r#in', 'r#ref'):
                pass
            elif name in ('type', 'match', 'in', 'ref', 'self', 'super', 'move', 'fn', 'mod', 'use', 'pub', 'let', 'mut', 'const', 'static', 'loop', 'while', 'for', 'if', 'else', 'return', 'break', 'continue', 'struct', 'enum', 'trait', 'impl', 'where', 'async', 'await', 'dyn', 'true', 'false'):
                name = f'r#{name}'
            rust_type = JAVA_TYPE_MAP.get(java_type, f'Box<dyn std::any::Any + Send + Sync>')
            parts.append(f'{name}: {rust_type}')
        elif len(tokens) == 1:
            parts.append(f'_{p}: Box<dyn std::any::Any + Send + Sync>')
        else:
            java_type = ' '.join(tokens[:-1])
            name = tokens[-1]
            rust_type = JAVA_TYPE_MAP.get(java_type, f'Box<dyn std::any::Any + Send + Sync>')
            parts.append(f'{name}: {rust_type}')
    
    return f'{prefix}, {", ".join(parts)}{suffix}'

def fix_return_type(line):
    """Fix Java-style return types."""
    replacements = {
        '-> boolean': '-> bool',
        '-> int': '-> i32',
        '-> long': '-> i64',
        '-> float': '-> f32',
        '-> double': '-> f64',
        '-> byte': '-> u8',
        '-> Object': '-> Box<dyn std::any::Any + Send + Sync>',
    }
    for old, new in replacements.items():
        if old in line:
            line = line.replace(old, new)
    return line

def fix_constructor_name(line):
    """Fix `pub fn ClassName(` to `pub fn new(`."""
    m = re.match(r'^(\s*)pub fn ([A-Z]\w+)\(', line)
    if m:
        indent, name = m.groups()
        return line.replace(f'pub fn {name}(', f'pub fn {name}_ctor(')
    return line

def fix_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    new_lines = []
    changed = False
    for line in lines:
        orig = line
        if 'TODO_RUST_CODE' in line and 'use' in line:
            line = line.replace(line.strip(), f'// {line.strip()}')
            if line != orig: changed = True
        elif re.search(r'pub fn \w+\(', line) and ('// Java:' not in line):
            line = fix_java_params(line)
            line = fix_return_type(line)
            line = fix_constructor_name(line)
            if line != orig: changed = True
        
        # Fix unresolved imports
        if line.strip().startswith('use io_wps_io::') or line.strip().startswith('use util::'):
            line = f'// {line}'
            if line != orig: changed = True
        
        # Fix unresolved types in struct fields
        for t in ['Arc<Mutex<', 'ArrayList', 'StringBuilder']:
            if t in line and '// Java' not in line and 'pub(crate)' in line and ':' in line:
                m2 = re.match(r'^(\s*(?:pub\(crate\)\s+)?)([\w]+):\s*(.*?),?\s*$', line)
                if m2:
                    indent, name, old_type = m2.groups()
                    line = f'{indent}{name}: Option<Box<dyn std::any::Any + Send + Sync>>,\n'
                    if line != orig: changed = True
        
        new_lines.append(line)
    
    if changed:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        print(f'Fixed: {filepath}')
    else:
        print(f'No changes: {filepath}')

for f in files:
    try:
        fix_file(f)
    except Exception as e:
        print(f'Error in {f}: {e}')

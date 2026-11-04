"""Fix generated Rust template for compilation while preserving encoding for consistency checker."""
import re
import sys

def fix_template(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 1. Remove '// TODO_RUST_CODE use ???;' lines
    content = re.sub(r'// TODO_RUST_CODE use \?\?\?;.*\n', '', content)

    # 2. Fix const declarations with /* TODO */
    content = re.sub(r'(pub(?:\(crate\))?\s+const\s+BLOCK_SIZE:\s*i32\s*=)\s*/\* TODO \*/;', r'\1 36;', content)
    content = re.sub(r'pub\(crate\)\s+const\s+serialVersionUID:\s*\w+\s*=\s*/\* TODO \*/;\n?', '', content)
    def fix_const_todo(m):
        decl = m.group(1)
        if 'f64' in decl:
            return decl + ' 0.0;'
        elif 'f32' in decl:
            return decl + ' 0.0;'
        elif 'i32' in decl or 'i64' in decl:
            return decl + ' 0;'
        elif 'bool' in decl:
            return decl + ' false;'
        elif 'str' in decl:
            return decl + ' "";'
        return decl + ' 0;'
    content = re.sub(r'(pub(?:\(crate\))?\s+const\s+\w+:\s*(?:i32|i64|f32|f64|bool|&str)\s*=)\s*/\* TODO \*/;',
                     fix_const_todo, content)
    # Generic catch-all for any remaining /* TODO */ consts
    content = re.sub(r'(const\s+\w+:\s*\w+\s*=)\s*/\* TODO \*/;', r'\1 0;', content)

    # 2b. Fix struct field types - replace complex Java-style types with Box<dyn std::any::Any>
    # Handle nested generics: Arc<Field<T>>, Arc<Mutex<Incrementor>>, T[][], FieldMatrix<T>, etc.
    def fix_field_type(line):
        m = re.match(r'(\s*pub\(crate\)\s+\w+:\s*)(.*)(,?)$', line)
        if not m:
            return line
        prefix = m.group(1)
        type_str = m.group(2).strip().rstrip(',')
        trailing_comma = ',' if m.group(3) else ','
        # Primitive types are fine
        if type_str in ('i32', 'i64', 'f32', 'f64', 'bool', 'usize', 'String'):
            return f'{prefix}{type_str}{trailing_comma}'
        # Vec types are fine
        if type_str.startswith('Vec<') and ('i32' in type_str or 'f64' in type_str or 'f32' in type_str or 'i64' in type_str or 'bool' in type_str or 'u8' in type_str or 'String' in type_str):
            return line
        # Simple known Rust types
        if type_str in ('Vec<f64>', 'Vec<i32>', 'Vec<Vec<f64>>', 'Vec<Vec<i32>>', 'Vec<String>'):
            return line
        # Everything else gets Box<dyn std::any::Any>
        if ('Arc<' in type_str or 'Collection<' in type_str or 'Field<' in type_str or
            'Matrix' in type_str or 'Vector' in type_str or 'int[' in type_str or
            'T[' in type_str or 'double[' in type_str or 'Object' in type_str or
            'Incrementor' in type_str or 'Listener' in type_str or
            type_str.endswith('>') or type_str.endswith('>>') or type_str.endswith('>>>') or
            '<T>' in type_str or 'HashMap' in type_str or 'Map<' in type_str or
            'List<' in type_str or 'Set<' in type_str or 'Iterator' in type_str or
            'Comparator' in type_str or 'Mutex' in type_str):
            return f'{prefix}Box<dyn std::any::Any>{trailing_comma}'
        return line

    lines = content.split('\n')
    new_lines = []
    for line in lines:
        if 'pub(crate)' in line and ':' in line and '//' not in line.split(':')[0]:
            line = fix_field_type(line)
        new_lines.append(line)
    content = '\n'.join(new_lines)
    # Also handle Collection types inline
    content = re.sub(r'Collection<[^>]+>', 'Vec<Box<dyn std::any::Any>>', content)
    # Fix reserved keyword struct names
    for kw in ['for', 'while', 'loop', 'if', 'else', 'match', 'return', 'break', 'continue', 'type', 'impl', 'trait', 'struct', 'enum', 'mod', 'use', 'fn', 'let', 'mut', 'ref', 'self', 'super', 'crate', 'as', 'in', 'where', 'async', 'await', 'dyn', 'abstract', 'become', 'box', 'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield', 'try']:
        content = content.replace(f'pub struct {kw} {{', f'pub struct r#{kw} {{')
        content = content.replace(f'pub trait {kw} {{', f'pub trait r#{kw} {{')
        content = content.replace(f'impl {kw} {{', f'impl r#{kw} {{')

    # Fix malformed 'impl ... for implements {' or 'impl ... for <keyword> {' patterns
    content = re.sub(r'^impl .+ for (implements|for|while|loop|if|else|match|return|type) \{', r'impl r#\1 {', content, flags=re.MULTILINE)

    # 3. Remove all TODO_RUST_CODE placeholders
    content = re.sub(r'\s*// TODO_RUST_CODE\s*$', '', content, flags=re.MULTILINE)
    content = content.replace('// TODO_RUST_CODE', '')

    # 4. Fix function signatures - replace Java-style params with valid Rust
    java_types = ['final ', 'T[]', 'Field<T>', 'FieldMatrix<T>', 'FieldVector<T>',
                  'BlockFieldMatrix<T>', 'BlockRealMatrix', 'RealMatrix ', 'RealVector',
                  'double[]', 'int[', 'int ', 'boolean ', 'String ', 'Object ', 'Comparable',
                  'FieldElement', 'ArrayFieldVector', 'DecompositionSolver', 'RealLinearOperator',
                  'IterationManager', 'IterativeLinearSolverEvent', 'IntegerSequence',
                  'IterationEvent', 'IterationListener', 'Incrementor', 'MaxCountExceedCallback',
                  'NumberFormat', 'Locale', 'long ', 'float ', 'byte ', 'short ', 'char ',
                  'Solver', 'Decomposition', 'Transformer', '-> T;', '-> T {',
                  'double ', 'UnivariateFunction', 'UnivariateDifferentiableFunction',
                  'DerivativeStructure', 'Derivative', 'Iterator', 'Entry ', 'T ',
                  'Map', 'List', 'Set', 'Collection', 'Comparator', 'Stream',
                  'Exception', 'Error', 'Throwable', 'Class', 'Method']

    lines = content.split('\n')
    new_lines = []
    for line in lines:
        # Handle trait method declarations (fn without pub)
        if re.match(r'\s+fn \w+\(', line) and any(jt in line for jt in java_types):
            m = re.match(r'(\s*)fn (\w+)\(&self.*\)(.*)$', line)
            if m:
                indent = m.group(1)
                fname = m.group(2)
                rest = m.group(3)
                ret_match = re.search(r'->\s*(.+?)\s*;', rest)
                if ret_match:
                    ret_type = ret_match.group(1).strip()
                    if ret_type in ('i32', 'i64', 'f64', 'f32', 'bool', 'usize', 'String'):
                        new_ret = ret_type
                    elif ret_type in ('int[]', 'int[][]'):
                        new_ret = 'Vec<i32>'
                    elif ret_type in ('double[]', 'double[][]'):
                        new_ret = 'Vec<f64>'
                    else:
                        new_ret = 'Box<dyn std::any::Any>'
                    new_lines.append(f'{indent}fn {fname}(&self) -> {new_ret};')
                else:
                    new_lines.append(f'{indent}fn {fname}(&self);')
                continue

        if 'pub fn ' in line and any(jt in line for jt in java_types):
            m = re.match(r'(\s*)pub fn (\w+)\(&(?:mut\s+)?self.*\)(.*)$', line)
            if m:
                indent = m.group(1)
                fname = m.group(2)
                rest = m.group(3)
                has_mut = '&mut self' in line
                self_param = '&mut self' if has_mut else '&self'
                ret_match = re.search(r'->\s*(.+?)\s*\{', rest)
                if ret_match:
                    ret_type = ret_match.group(1).strip()
                    if ret_type in ('i32', 'i64', 'f64', 'f32', 'bool', 'usize', 'String'):
                        new_ret = ret_type
                    elif ret_type == 'T':
                        new_ret = 'Box<dyn std::any::Any>'
                    elif ret_type in ('int[]', 'int[][]'):
                        new_ret = 'Vec<i32>'
                    elif ret_type in ('double[]', 'double[][]'):
                        new_ret = 'Vec<f64>'
                    elif ret_type.startswith('Vec<') and not any(jt in ret_type for jt in ['Arc', 'Field', 'Matrix', 'Vector']):
                        new_ret = ret_type
                    elif ret_type.startswith('&') and 'Any' not in ret_type:
                        new_ret = ret_type
                    else:
                        new_ret = 'Box<dyn std::any::Any>'
                    new_lines.append(f'{indent}pub fn {fname}({self_param}) -> {new_ret} {{')
                else:
                    new_lines.append(f'{indent}pub fn {fname}({self_param}) {{')
                continue
            # Try static function (no self)
            m2 = re.match(r'(\s*)pub fn (\w+)\((.+?)\)(.*)$', line)
            if m2 and 'self' not in m2.group(3):
                indent = m2.group(1)
                fname = m2.group(2)
                rest = m2.group(4)
                ret_match = re.search(r'->\s*(.+?)\s*\{', rest)
                if ret_match:
                    ret_type = ret_match.group(1).strip()
                    if ret_type in ('i32', 'i64', 'f64', 'f32', 'bool', 'usize', 'String'):
                        new_ret = ret_type
                    else:
                        new_ret = 'Box<dyn std::any::Any>'
                    new_lines.append(f'{indent}pub fn {fname}() -> {new_ret} {{')
                else:
                    new_lines.append(f'{indent}pub fn {fname}() {{')
                continue
        new_lines.append(line)

    content = '\n'.join(new_lines)

    # 5. Fix duplicate function names (both pub fn and trait fn)
    seen = {}
    lines = content.split('\n')
    new_lines = []
    for line in lines:
        m = re.match(r'(\s*)(?:pub )?fn (\w+)\(', line)
        if m:
            name = m.group(2)
            if name not in seen:
                seen[name] = 0
            seen[name] += 1
            if seen[name] > 1:
                new_name = f'{name}_{seen[name]}'
                line = line.replace(f'pub fn {name}(', f'pub fn {new_name}(', 1)
        new_lines.append(line)
    content = '\n'.join(new_lines)

    # 6. Remove impl Serializable/Cloneable blocks
    content = re.sub(r'\nimpl \w+ for \w+ \{\n\}\n', '\n', content)

    # 6b. Fix duplicate struct/impl names
    struct_seen = {}
    lines = content.split('\n')
    new_lines = []
    for line in lines:
        m = re.match(r'(pub struct )(\w+)( \{)', line)
        if m:
            name = m.group(2)
            if name not in struct_seen:
                struct_seen[name] = 0
            struct_seen[name] += 1
            if struct_seen[name] > 1:
                new_name = f'{name}_{struct_seen[name]}'
                line = f'{m.group(1)}{new_name}{m.group(3)}'
        m2 = re.match(r'(impl )(\w+)( \{)', line)
        if m2:
            name = m2.group(2)
            # Check if this impl name has a corresponding struct suffix
            if name in struct_seen and struct_seen[name] > 1:
                # Count impl occurrences
                if not hasattr(fix_template, '_impl_seen'):
                    fix_template._impl_seen = {}
                if name not in fix_template._impl_seen:
                    fix_template._impl_seen[name] = 0
                fix_template._impl_seen[name] += 1
                if fix_template._impl_seen[name] > 1:
                    new_name = f'{name}_{fix_template._impl_seen[name]}'
                    line = f'{m2.group(1)}{new_name}{m2.group(3)}'
        new_lines.append(line)
    content = '\n'.join(new_lines)
    if hasattr(fix_template, '_impl_seen'):
        del fix_template._impl_seen

    # 7. Add todo!() to empty function bodies with return types
    lines = content.split('\n')
    new_lines = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if 'pub fn ' in line and '->' in line and line.rstrip().endswith('{'):
            new_lines.append(line)
            i += 1
            body_lines = []
            depth = 1
            while i < len(lines) and depth > 0:
                l = lines[i]
                stripped = l.strip()
                depth += stripped.count('{') - stripped.count('}')
                body_lines.append(l)
                i += 1
            has_code = False
            for bl in body_lines[:-1]:
                s = bl.strip()
                if s and not s.startswith('//') and s != '{' and s != '}':
                    has_code = True
                    break
            if not has_code and body_lines:
                for bl in body_lines[:-1]:
                    new_lines.append(bl)
                new_lines.append('        todo!()')
                new_lines.append(body_lines[-1])
            else:
                new_lines.extend(body_lines)
        else:
            new_lines.append(line)
            i += 1

    content = '\n'.join(new_lines)

    with open(path, 'w', encoding='utf-8', newline='') as f:
        f.write(content)
    print(f'Fixed: {path}')

if __name__ == '__main__':
    fix_template(sys.argv[1])

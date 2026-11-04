import re

INPUT = r'd:\agentic\src\wordOffice\libs\core-libs\math3\src\org\apache\commons\math3\dfp\Dfp.rs'

with open(INPUT, 'r', encoding='utf-8') as f:
    lines = f.readlines()

JAVA_CONSTANTS = {
    'RADIX': '10000',
    'MIN_EXP': '-32767',
    'MAX_EXP': '32768',
    'ERR_SCALE': '32760',
    'FINITE': '0',
    'INFINITE': '1',
    'SNAN': '2',
    'QNAN': '3',
    'NAN_STRING': '"NaN"',
    'POS_INFINITY_STRING': '"Infinity"',
    'NEG_INFINITY_STRING': '"-Infinity"',
    'ADD_TRAP': '"add"',
    'MULTIPLY_TRAP': '"multiply"',
    'DIVIDE_TRAP': '"divide"',
    'SQRT_TRAP': '"sqrt"',
    'ALIGN_TRAP': '"align"',
    'TRUNC_TRAP': '"trunc"',
    'NEXT_AFTER_TRAP': '"nextAfter"',
    'LESS_THAN_TRAP': '"lessThan"',
    'GREATER_THAN_TRAP': '"greaterThan"',
    'NEW_INSTANCE_TRAP': '"newInstance"',
}

JAVA_TYPE_MAP = {
    'byte': 'i8',
    'int': 'i32',
    'long': 'i64',
    'float': 'f32',
    'double': 'f64',
    'boolean': 'bool',
    'char': 'char',
    'String': '&str',
    'DfpField': 'DfpField',
    'Dfp': '&Dfp',
    'Object': '&dyn std::any::Any',
    'DfpField.RoundingMode': 'i32',
}

RETURN_TYPE_MAP = {
    'Dfp': 'Dfp',
    'DfpField': 'DfpField',
    'String': 'String',
    'int': 'i32',
    'long': 'i64',
    'boolean': 'bool',
    'byte': 'i8',
    'double': 'f64',
    'void': '',
    'uses': '()',
    'double[]': '[f64; 2]',
}

output = []
method_names_count = {}
in_impl = False
method_stack = []  # stack of (indent, return_type, line_index)

i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.rstrip('\n')
    trimmed = stripped.strip()

    # Fix constants with /* TODO */
    if '= /* TODO */' in stripped:
        for name, val in JAVA_CONSTANTS.items():
            if f' {name}:' in stripped:
                stripped = stripped.replace('/* TODO */', val)
                # Fix String const to &str
                stripped = stripped.replace(': String =', ': &str =')
                break
        output.append(stripped + '\n')
        i += 1
        continue

    # Fix TODO use line for java.util.Arrays
    if trimmed.startswith('// TODO_RUST_CODE use') and 'java.util.Arrays' in trimmed:
        i += 1
        continue

    # Fix struct fields
    if 'pub(crate) mant: Arc<Mutex<Vec<i32>>>' in stripped:
        stripped = stripped.replace('Arc<Mutex<Vec<i32>>>', 'Vec<i32>')
        output.append(stripped + '\n')
        i += 1
        continue

    if 'pub(crate) field: Arc<DfpField>' in stripped:
        # Remove DfpField field entirely - it's a separate class
        # Keep the Java field annotation comment
        i += 1
        continue

    if '// Java field: DfpField field' in trimmed:
        # Skip this annotation too
        i += 1
        continue

    # Add #[derive(Clone, Debug)] before Dfp struct
    if stripped == 'pub struct Dfp {':
        output.append('#[derive(Clone, Debug)]\n')
        output.append(stripped + '\n')
        i += 1
        continue

    # Skip // TODO_RUST_CODE lines
    if trimmed == '// TODO_RUST_CODE':
        i += 1
        continue

    # Track impl blocks
    if trimmed.startswith('impl ') and trimmed.endswith('{'):
        in_impl = True

    # Fix method signatures
    fn_match = re.match(r'^(\s*)pub fn (\w+)\((.+?)\)\s*(\{|->\s*.+\s*\{)\s*$', stripped)
    if fn_match and in_impl:
        indent = fn_match.group(1)
        fn_name = fn_match.group(2)
        params_str = fn_match.group(3)
        tail = fn_match.group(4)

        # Parse return type
        ret_type = ''
        if tail.startswith('->'):
            ret_match = re.match(r'->\s*(.+?)\s*\{', tail)
            if ret_match:
                ret_type = ret_match.group(1).strip()

        # Fix return type
        if ret_type in RETURN_TYPE_MAP:
            ret_type = RETURN_TYPE_MAP[ret_type]

        # Handle overloaded method names
        if fn_name not in method_names_count:
            method_names_count[fn_name] = 0
        else:
            method_names_count[fn_name] += 1

        count = method_names_count[fn_name]
        if count > 0:
            new_fn_name = f'{fn_name}_{count}'
        else:
            new_fn_name = fn_name

        # Parse and fix parameters
        rust_params = ['&self']
        raw_params = params_str.split(',')
        for p in raw_params:
            p = p.strip()
            if p == '&self' or p == '&mut self' or p == 'self':
                continue
            # Remove 'final' keyword
            p = re.sub(r'\bfinal\s+', '', p)
            # Try to parse Java-style "Type name" or "Type[] name"
            param_match = re.match(r'^(\w+(?:\.\w+)?)\s*(\[\])?\s+(\w+)$', p)
            if param_match:
                java_type = param_match.group(1)
                is_array = param_match.group(2) is not None
                param_name = param_match.group(3)

                if is_array:
                    if java_type == 'Dfp':
                        rust_type = '&[Dfp]'
                    elif java_type == 'double':
                        rust_type = '&[f64]'
                    elif java_type == 'int':
                        rust_type = '&[i32]'
                    elif java_type == 'char':
                        rust_type = '&[char]'
                    else:
                        rust_type = f'&[{java_type}]'
                elif java_type in JAVA_TYPE_MAP:
                    rust_type = JAVA_TYPE_MAP[java_type]
                else:
                    rust_type = f'&{java_type}'
                rust_params.append(f'{param_name}: {rust_type}')
            elif p.startswith('Dfp[]'):
                pn = p.replace('Dfp[]', '').strip()
                rust_params.append(f'{pn}: &[Dfp]')
            elif p.startswith('double[]'):
                pn = p.replace('double[]', '').strip()
                rust_params.append(f'{pn}: &[f64]')
            else:
                # Already valid Rust or unknown
                if ':' in p:
                    rust_params.append(p)

        params_joined = ', '.join(rust_params)
        if ret_type and ret_type != '()':
            new_line = f'{indent}pub fn {new_fn_name}({params_joined}) -> {ret_type} {{'
        else:
            new_line = f'{indent}pub fn {new_fn_name}({params_joined}) {{'

        # Track method for adding todo!() at the end
        # Count braces from this line
        brace_depth = new_line.count('{') - new_line.count('}')
        method_stack.append({
            'indent': indent,
            'ret_type': ret_type,
            'depth': brace_depth,
            'start_line': len(output),
            'has_todo': False,
        })

        output.append(new_line + '\n')
        i += 1
        continue

    # Track brace depth for methods
    if method_stack:
        opens = stripped.count('{')
        closes = stripped.count('}')
        method_stack[-1]['depth'] += opens - closes

        if method_stack[-1]['depth'] <= 0:
            # Method closing brace
            info = method_stack.pop()
            ret = info['ret_type']
            if ret and ret != '()' and not info['has_todo']:
                # Insert todo!() before closing brace
                output.append(info['indent'] + '    todo!();\n')
            output.append(stripped + '\n')
            i += 1
            continue

    output.append(stripped + '\n')
    i += 1

with open(INPUT, 'w', encoding='utf-8') as f:
    f.writelines(output)

print(f"Done. Wrote {len(output)} lines.")

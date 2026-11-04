import re, sys

java_path = sys.argv[1]
rust_path = sys.argv[2]

with open(java_path, 'r') as f:
    java_lines = f.readlines()

field_annotations = []
i = 0
while i < len(java_lines):
    stripped = java_lines[i].strip()
    if stripped.startswith('throws ') and not stripped.startswith('throws new'):
        if stripped.endswith(';'):
            text = stripped.rstrip(';').strip()
            field_annotations.append(f'// Java field: {text}')
        elif stripped.endswith(','):
            text = stripped.rstrip(',').strip()
            field_annotations.append(f'// Java field: {text}')
            j = i + 1
            while j < len(java_lines):
                cont = java_lines[j].strip()
                if cont.endswith(';'):
                    cont = cont.rstrip(';').strip()
                    if cont and re.match(r'^[\w,\s]+$', cont):
                        field_annotations.append(f'// Java field: {cont}')
                    break
                elif cont.endswith(','):
                    cont = cont.rstrip(',').strip()
                    if cont and re.match(r'^[\w,\s]+$', cont):
                        field_annotations.append(f'// Java field: {cont}')
                j += 1
    i += 1

with open(rust_path, 'r') as f:
    lines = f.readlines()

output = []
method_count = {}
i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.rstrip('\n')
    trimmed = stripped.strip()

    if trimmed.startswith('use crate::') and '::Field;' in trimmed and '::Field::Field' not in trimmed:
        stripped = stripped.replace('::Field;', '::Field::Field;')
    if trimmed.startswith('use crate::') and '::FieldElement;' in trimmed and '::FieldElement::FieldElement' not in trimmed:
        stripped = stripped.replace('::FieldElement;', '::FieldElement::FieldElement;')

    if re.match(r'^pub trait \w+ \{$', trimmed):
        if field_annotations:
            output.append(stripped + '\n')
            for ann in field_annotations:
                output.append(f'    {ann}\n')
            i += 1
            continue

    if trimmed.startswith('// Java: abstract') and i + 1 < len(lines):
        java_comment = stripped
        next_line = lines[i + 1].rstrip('\n')

        fn_match = re.match(r'^(\s+)fn (\w+)\(&self(.*?)\)\s*(->.*)?;\s*$', next_line)
        if fn_match:
            indent = fn_match.group(1)
            name = fn_match.group(2)
            params_raw = fn_match.group(3)
            ret_part = fn_match.group(4) or ''

            if name not in method_count:
                method_count[name] = 0
            else:
                method_count[name] += 1
            count = method_count[name]
            new_name = f'{name}_{count}' if count > 0 else name

            rust_params = []
            if params_raw.strip().startswith(','):
                params_str = params_raw.strip()[1:].strip()
                for p in params_str.split(','):
                    p = p.strip()
                    if not p:
                        continue
                    p = re.sub(r'\bfinal\s+', '', p)
                    m2 = re.match(r'^(\w+(?:<[^>]+>)?)\s*(\[\]\[\])?\s*(\[\])?\s+(\w+)$', p)
                    if m2:
                        jtype, is_2d, is_arr, pname = m2.group(1), m2.group(2), m2.group(3), m2.group(4)
                        type_map = {'int': 'i32', 'long': 'i64', 'double': 'f64', 'float': 'f32', 'boolean': 'bool'}
                        if is_2d:
                            inner = type_map.get(jtype, 'f64')
                            rust_params.append(f'{pname}: &mut Vec<Vec<{inner}>>')
                        elif is_arr:
                            arr_map = {'int': '&[i32]', 'double': '&[f64]', 'T': '&[T]'}
                            rust_params.append(f'{pname}: {arr_map.get(jtype, "&[f64]")}')
                        elif jtype in type_map:
                            rust_params.append(f'{pname}: {type_map[jtype]}')
                        elif jtype == 'T':
                            rust_params.append(f'{pname}: T')
                        else:
                            rust_params.append(f'{pname}: &dyn std::any::Any')
                    else:
                        rust_params.append(p)

            params_joined = ', '.join(['&self'] + rust_params) if rust_params else '&self'

            ret = ret_part.strip() if ret_part else ''
            if ret:
                ret_type = ret.replace('->', '').strip().rstrip(';')
                ret_lower = ret_type.lower()
                if any(k in ret_type for k in ['Matrix', 'Vector', 'RealLinearOperator']):
                    ret = '-> Box<dyn std::any::Any>'
                elif ret_type == 'T':
                    ret = '-> T'
                elif 'double[][]' in ret_type or 'T[][]' in ret_type:
                    ret = '-> Vec<Vec<f64>>'
                elif 'double[]' in ret_type or 'T[]' in ret_type:
                    ret = '-> Vec<f64>'
                elif 'int[]' in ret_type:
                    ret = '-> Vec<i32>'
                elif ret_type in ('int', 'long', 'double', 'float', 'boolean'):
                    type_map = {'int': 'i32', 'long': 'i64', 'double': 'f64', 'float': 'f32', 'boolean': 'bool'}
                    ret = f'-> {type_map[ret_type]}'
                elif 'Field' in ret_type:
                    ret = '-> std::any::TypeId'
                else:
                    ret = f'-> {ret_type}'

            new_fn = f'{indent}fn {new_name}({params_joined}) {ret};' if ret else f'{indent}fn {new_name}({params_joined});'
            output.append(java_comment + '\n')
            output.append(new_fn + '\n')
            i += 2
            continue

    output.append(stripped + '\n')
    i += 1

with open(rust_path, 'w') as f:
    f.writelines(output)

print(f'Done. {len(output)} lines, {len(field_annotations)} field annotations')

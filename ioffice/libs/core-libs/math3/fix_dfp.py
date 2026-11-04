import re

path = r'd:\agentic\src\wordOffice\libs\core-libs\math3\src\org\apache\commons\math3\dfp\Dfp.rs'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()

# Fix constants and struct
replacements = [
    ('pub(crate) const RADIX: i32 = /* TODO */;', 'pub(crate) const RADIX: i32 = 10000;'),
    ('pub(crate) const MIN_EXP: i32 = /* TODO */;', 'pub(crate) const MIN_EXP: i32 = -32767;'),
    ('pub(crate) const MAX_EXP: i32 = /* TODO */;', 'pub(crate) const MAX_EXP: i32 = 32768;'),
    ('pub(crate) const ERR_SCALE: i32 = /* TODO */;', 'pub(crate) const ERR_SCALE: i32 = 32760;'),
    ('pub(crate) const FINITE: i8 = /* TODO */;', 'pub(crate) const FINITE: i8 = 0;'),
    ('pub(crate) const INFINITE: i8 = /* TODO */;', 'pub(crate) const INFINITE: i8 = 1;'),
    ('pub(crate) const SNAN: i8 = /* TODO */;', 'pub(crate) const SNAN: i8 = 2;'),
    ('pub(crate) const QNAN: i8 = /* TODO */;', 'pub(crate) const QNAN: i8 = 3;'),
    ('pub(crate) const NAN_STRING: String = /* TODO */;', 'pub(crate) const NAN_STRING: &str = "NaN";'),
    ('pub(crate) const POS_INFINITY_STRING: String = /* TODO */;', 'pub(crate) const POS_INFINITY_STRING: &str = "Infinity";'),
    ('pub(crate) const NEG_INFINITY_STRING: String = /* TODO */;', 'pub(crate) const NEG_INFINITY_STRING: &str = "-Infinity";'),
    ('pub(crate) const ADD_TRAP: String = /* TODO */;', 'pub(crate) const ADD_TRAP: &str = "add";'),
    ('pub(crate) const MULTIPLY_TRAP: String = /* TODO */;', 'pub(crate) const MULTIPLY_TRAP: &str = "multiply";'),
    ('pub(crate) const DIVIDE_TRAP: String = /* TODO */;', 'pub(crate) const DIVIDE_TRAP: &str = "divide";'),
    ('pub(crate) const SQRT_TRAP: String = /* TODO */;', 'pub(crate) const SQRT_TRAP: &str = "sqrt";'),
    ('pub(crate) const ALIGN_TRAP: String = /* TODO */;', 'pub(crate) const ALIGN_TRAP: &str = "align";'),
    ('pub(crate) const TRUNC_TRAP: String = /* TODO */;', 'pub(crate) const TRUNC_TRAP: &str = "trunc";'),
    ('pub(crate) const NEXT_AFTER_TRAP: String = /* TODO */;', 'pub(crate) const NEXT_AFTER_TRAP: &str = "nextAfter";'),
    ('pub(crate) const LESS_THAN_TRAP: String = /* TODO */;', 'pub(crate) const LESS_THAN_TRAP: &str = "lessThan";'),
    ('pub(crate) const GREATER_THAN_TRAP: String = /* TODO */;', 'pub(crate) const GREATER_THAN_TRAP: &str = "greaterThan";'),
    ('pub(crate) const NEW_INSTANCE_TRAP: String = /* TODO */;', 'pub(crate) const NEW_INSTANCE_TRAP: &str = "newInstance";'),
    ('pub(crate) mant: Arc<Mutex<Vec<i32>>>,', 'pub(crate) mant: Vec<i32>,'),
    ('// TODO_RUST_CODE use ???; // java.util.Arrays\n', ''),
]
for old, new in replacements:
    content = content.replace(old, new)

content = content.replace('    // Java field: DfpField field\n    pub(crate) field: Arc<DfpField>,\n', '')
content = content.replace('// Java: public class Dfp\npub struct Dfp {', '// Java: public class Dfp\n#[derive(Clone, Debug)]\npub struct Dfp {', 1)

# Remove invalid impl block
content = content.replace('\nimpl a modification of the for implements {\n}\n', '\n')

# Process line by line
lines = content.split('\n')
output = []
method_counter = {}

i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    
    # Skip TODO_RUST_CODE lines
    if stripped == '// TODO_RUST_CODE':
        i += 1
        continue
    
    # Check for method signature with Java-style params
    # Match: pub fn name(params) -> type {
    # or:    pub fn name(params) {
    sig_match = re.match(r'^(\s+)(pub fn )(\w+)\((.*)\)(\s*->\s*\S+)?\s*\{$', line)
    if sig_match:
        indent = sig_match.group(1)
        prefix = sig_match.group(2) 
        name = sig_match.group(3)
        params = sig_match.group(4)
        ret = sig_match.group(5) or ''
        
        # Check if params contain Java types
        is_java_params = bool(re.search(r'\bfinal\b|\bDfpField\b|\bObject\b|(?<!\w)(byte|int|long|double|float)\s+\w|DfpField\.RoundingMode|\bDfp\b\s+\w+(?!.*&self)|Dfp\[\]|double\[\]', params))
        
        if is_java_params or (params.strip() and params.strip() != '&self' and '&self' not in params and '&mut self' not in params):
            # This has Java-style params, needs fixing
            # Fix return type
            ret = ret.replace('DfpField', '!')
            ret = ret.replace('uses', '()')
            
            # Handle duplicate names by adding suffix
            if name in method_counter:
                method_counter[name] += 1
                new_name = f'{name}_{method_counter[name]}'
            else:
                method_counter[name] = 0
                new_name = name
            
            line = f'{indent}{prefix}{new_name}(&self){ret} {{\n'
            output.append(line)
            
            # Now scan forward to find the matching closing brace
            # and add todo!() at the start
            output.append(f'{indent}    todo!();\n')
            
            # Skip to the closing brace of this method
            depth = 1
            i += 1
            while i < len(lines) and depth > 0:
                s = lines[i].strip()
                # Count braces (excluding those in comments)
                if not s.startswith('//'):
                    depth += s.count('{') - s.count('}')
                else:
                    # Only count standalone braces
                    pass
                
                # Keep all // Java: annotations
                if s.startswith('// Java:'):
                    output.append(lines[i] + '\n' if not lines[i].endswith('\n') else lines[i])
                
                if depth <= 0:
                    # This is the closing brace
                    output.append(lines[i] + '\n' if not lines[i].endswith('\n') else lines[i])
                
                i += 1
            continue
        elif '&self' in params:
            # Valid Rust params, keep as-is
            # But check for duplicate names
            if name in method_counter:
                method_counter[name] += 1
                new_name = f'{name}_{method_counter[name]}'
                line = line.replace(f'fn {name}(', f'fn {new_name}(', 1)
            else:
                method_counter[name] = 0
            
            # Fix return type if needed
            if ret and ('DfpField' in ret or 'uses' in ret):
                line = line.replace('DfpField', '!')
                line = line.replace('uses', '()')
    
    # Fix "pub fn shiftRight(&self) -> uses {" 
    if '-> uses' in line:
        # This is the problematic duplicate - skip entire method
        depth = 1 if '{' in line else 0
        i += 1
        while i < len(lines) and depth > 0:
            s = lines[i].strip()
            if not s.startswith('//'):
                depth += s.count('{') - s.count('}')
            i += 1
        continue
    
    output.append(line + '\n' if not line.endswith('\n') else line)
    i += 1

# Now fix methods that only have todo!() + comments + braces (ensure they actually have a return)
content = ''.join(output)

# Remove double newlines that might have been created
content = re.sub(r'\n{3,}', '\n\n', content)

with open(path, 'w', encoding='utf-8') as f:
    f.write(content)

print(f'Done. Output {len(output)} lines')

import re

filepath = r'src/cn/wps/io/dom/tree/DefaultElement.rs'

with open(filepath, 'r', encoding='utf-8') as f:
    content = f.read()

java_type_map = {
    'int': 'i32',
    'long': 'i64',
    'float': 'f32',
    'double': 'f64',
    'boolean': 'bool',
    'String': '&str',
    'StringBuilder': '&mut String',
    'Object': '&dyn std::any::Any',
    'Node': '&dyn std::any::Any',
    'Element': '&dyn std::any::Any',
    'Attribute': '&dyn std::any::Any',
    'CDATA': '&dyn std::any::Any',
    'Comment': '&dyn std::any::Any',
    'Entity': '&dyn std::any::Any',
    'Namespace': '&dyn std::any::Any',
    'ProcessingInstruction': '&dyn std::any::Any',
    'Text': '&dyn std::any::Any',
    'Branch': '&dyn std::any::Any',
    'Document': '&dyn std::any::Any',
    'MCEContext': '&dyn std::any::Any',
    'Attributes': '&dyn std::any::Any',
    'NamespaceStack': '&dyn std::any::Any',
    'IOException': '&dyn std::any::Any',
}

def fix_java_param(match):
    java_type = match.group(1)
    param_name = match.group(2)
    rust_type = java_type_map.get(java_type)
    if rust_type:
        return f'{param_name}: {rust_type}'
    if java_type.startswith('Map<') or java_type.startswith('List<'):
        return f'{param_name}: &dyn std::any::Any'
    return match.group(0)

lines = content.split('\n')
new_lines = []
for line in lines:
    if 'pub fn ' in line and '(' in line:
        # Fix Java-style parameters: Type paramName -> paramName: RustType
        # Handle generic types like Map<String, String>
        line = re.sub(r'Map<[^>]+>\s+(\w+)', r'\1: &dyn std::any::Any', line)
        line = re.sub(r'List<[^>]+>\s+(\w+)', r'\1: &dyn std::any::Any', line)
        # Handle simple types
        for java_type, rust_type in java_type_map.items():
            pattern = rf'\b{re.escape(java_type)}\s+(\w+)'
            line = re.sub(pattern, rf'\1: {rust_type}', line)
    new_lines.append(line)

content = '\n'.join(new_lines)

with open(filepath, 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed Java-style parameters")

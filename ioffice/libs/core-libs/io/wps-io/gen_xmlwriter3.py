import re

java_path = r'd:\agentic\src\wordOffice\koffice\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.java'
rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.rs'

with open(java_path, 'r', encoding='utf-8') as f:
    java_lines = f.readlines()

class JavaMethod:
    def __init__(self, signature, body_lines):
        self.signature = signature
        self.body_lines = body_lines

def collect_method_body(lines, start_i):
    body = []
    i = start_i
    brace_depth = 0
    found_open = False
    
    while i < len(lines):
        line = lines[i].rstrip()
        stripped = line.strip()
        
        in_string = False
        in_char = False
        escaped = False
        for ch in stripped:
            if escaped:
                escaped = False
                continue
            if ch == '\\':
                escaped = True
                continue
            if ch == '"' and not in_char:
                in_string = not in_string
            elif ch == "'" and not in_string:
                in_char = not in_char
            elif not in_string and not in_char:
                if ch == '{':
                    brace_depth += 1
                    found_open = True
                elif ch == '}':
                    brace_depth -= 1
        
        if found_open and brace_depth == 0:
            # Check if closing line has content before }
            if stripped != '}':
                cleaned = clean_java_line(stripped)
                if cleaned:
                    body.append(cleaned)
            return body, i
        
        if found_open and i > start_i:
            if stripped:
                cleaned = clean_java_line(stripped)
                if cleaned:
                    body.append(cleaned)
        
        i += 1
    
    return body, i

def clean_java_line(line):
    """Clean a Java source line for use as a // Java: annotation.
    - Strip trailing ' {' from block openers (if, else, try, catch, for, while, switch)
    - Strip leading '} ' from continuation blocks (} else, } catch, } finally)
    - Skip pure '{' or '}' lines
    """
    s = line.strip()
    if s in ('{', '}'):
        return None
    
    # Handle "} else {", "} else if (...) {", "} catch (...) {", "} finally {"
    if s.startswith('}'):
        s = s[1:].strip()
        if not s:
            return None
    
    # Strip trailing " {" from block openers
    if s.endswith(' {') or s.endswith('{'):
        # Check if this is a block opener (not an array init or lambda)
        block_prefixes = ('if ', 'if(', 'else', 'for ', 'for(', 'while ', 'while(', 
                          'try', 'catch ', 'catch(', 'finally', 'switch ', 'switch(',
                          'do', 'synchronized')
        is_block = False
        for prefix in block_prefixes:
            if s.startswith(prefix):
                is_block = True
                break
        if is_block:
            # Strip trailing {
            s = s.rstrip()
            if s.endswith('{'):
                s = s[:-1].rstrip()
    
    return s if s else None

def parse_java_methods(lines):
    methods = []
    i = 0
    class_started = False
    
    while i < len(lines):
        line = lines[i].rstrip()
        stripped = line.strip()
        
        if not class_started:
            if 'public class XMLWriter' in stripped:
                class_started = True
            i += 1
            continue
        
        constructor_match = re.match(
            r'\s+(public|protected|private)\s+XMLWriter\s*\(([^)]*)\)\s*(?:throws\s+[^{]*)?\s*\{?\s*$', line)
        method_match = re.match(
            r'\s+(public|protected|private)\s+(?:static\s+)?(?:final\s+)?(\S+(?:<[^>]+>)?(?:\[\])?)\s+(\w+)\s*\(([^)]*)\)\s*(?:throws\s+[^{]*)?\s*\{?\s*$', line)
        
        if constructor_match:
            params = constructor_match.group(2).strip()
            sig = f"XMLWriter({params})" if params else "XMLWriter()"
            body, end_i = collect_method_body(lines, i)
            methods.append(JavaMethod(sig, body))
            i = end_i + 1
            continue
        elif method_match:
            ret_type = method_match.group(2)
            name = method_match.group(3)
            params = method_match.group(4).strip()
            sig = f"{ret_type} {name}({params})" if params else f"{ret_type} {name}()"
            body, end_i = collect_method_body(lines, i)
            methods.append(JavaMethod(sig, body))
            i = end_i + 1
            continue
        
        i += 1
    
    return methods

java_methods = parse_java_methods(java_lines)
print(f"Parsed {len(java_methods)} Java methods")

sig_to_body = {}
for m in java_methods:
    sig_to_body[m.signature] = m.body_lines
    if len(m.body_lines) > 10:
        print(f"  {m.signature}: {len(m.body_lines)} body lines")

# Read the current template
with open(rust_path, 'r', encoding='utf-8') as f:
    rust_content = f.read()

rust_lines = rust_content.split('\n')

# Find if isElementSpacePreserved exists in template
has_isElementSpacePreserved = any('isElementSpacePreserved' in l and 'pub fn' in l for l in rust_lines)
print(f"Template has isElementSpacePreserved method: {has_isElementSpacePreserved}")

# Process: for each method with todo!(), inject body annotations
out = []
i = 0
matched = 0

while i < len(rust_lines):
    line = rust_lines[i]
    stripped = line.strip()
    
    ann_match = re.match(r'(\s+)// Java: (.+)', line)
    if ann_match and i + 2 < len(rust_lines):
        indent = ann_match.group(1)
        annotation = ann_match.group(2).strip()
        next_line = rust_lines[i+1].strip() if i+1 < len(rust_lines) else ''
        next2_line = rust_lines[i+2].strip() if i+2 < len(rust_lines) else ''
        
        if next_line.startswith('pub fn') and next2_line == 'todo!()':
            body = sig_to_body.get(annotation)
            
            out.append(line)  # annotation
            out.append(rust_lines[i+1])  # pub fn line
            
            if body:
                body_indent = indent + '    '
                for bline in body:
                    out.append(f'{body_indent}// Java: {bline}')
                out.append(f'{body_indent}todo!()')
                matched += 1
            else:
                out.append(rust_lines[i+2])  # keep todo!()
            
            i += 3
            
            # After writeElement method, add isElementSpacePreserved if not present
            if 'void writeElement(Element element)' in annotation and not has_isElementSpacePreserved:
                # Add the method right after writeElement's closing brace
                while i < len(rust_lines) and rust_lines[i].strip() != '}':
                    out.append(rust_lines[i])
                    i += 1
                if i < len(rust_lines):
                    out.append(rust_lines[i])  # the closing }
                    i += 1
                
                # Insert isElementSpacePreserved method
                out.append('')
                body = sig_to_body.get('boolean isElementSpacePreserved(Element element)')
                out.append(f'{indent}// Java: boolean isElementSpacePreserved(Element element)')
                out.append(f'{indent}pub fn isElementSpacePreserved(&mut self) -> bool {{')
                if body:
                    body_indent = indent + '    '
                    for bline in body:
                        out.append(f'{body_indent}// Java: {bline}')
                    out.append(f'{body_indent}todo!()')
                else:
                    out.append(f'{indent}    todo!()')
                out.append(f'{indent}}}')
                matched += 1
            
            continue
    
    out.append(line)
    i += 1

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write('\n'.join(out))

# Count total // Java: lines
total_java_annotations = sum(1 for l in out if '// Java:' in l)
print(f"\nMatched {matched} methods")
print(f"Total lines: {len(out)}")
print(f"Total // Java: annotations: {total_java_annotations}")

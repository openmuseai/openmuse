import re

rust_path = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.rs'

with open(rust_path, 'r', encoding='utf-8') as f:
    content = f.read()

# 1. Fix imports - replace broken use statements
content = content.replace('use io_wps_io::cn::wps::io::dom::Attribute;', '// use io_wps_io - Attribute (trait in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::CDATA;', '// use io_wps_io - CDATA (trait in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::Comment;', '// use io_wps_io - Comment (trait in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::Document;', '// use io_wps_io - Document (trait in crate)')
content = content.replace('use crate::io::dom::Element;', '// Element not yet defined as module')
content = content.replace('use io_wps_io::cn::wps::io::dom::Entity;', '// use io_wps_io - Entity (trait in crate)')
content = content.replace('use crate::io::dom::Namespace;', '// Namespace not yet defined as module')
content = content.replace('use io_wps_io::cn::wps::io::dom::Node;', '// use io_wps_io - Node (trait in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::NodeType;', '// use io_wps_io - NodeType (in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::ProcessingInstruction;', '// use io_wps_io - PI (trait in crate)')
content = content.replace('use io_wps_io::cn::wps::io::dom::Text;', '// use io_wps_io - Text (trait in crate)')
content = content.replace('use crate::io::dom::tree::NamespaceStack;', '// NamespaceStack not yet defined')
content = content.replace('use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;', '// Attributes (SAX) not available')
content = content.replace('use crate::org::xml::sax::InputSource;', '// SAX InputSource not available')
content = content.replace('use crate::org::xml::sax::Locator;', '// SAX Locator not available')
content = content.replace('use crate::org::xml::sax::SAXException;', '// SAX SAXException not available')
content = content.replace('use crate::org::xml::sax::SAXNotRecognizedException;', '// SAX not available')
content = content.replace('use crate::org::xml::sax::SAXNotSupportedException;', '// SAX not available')
content = content.replace('use crate::org::xml::sax::XMLReader;', '// SAX XMLReader not available')
content = content.replace('use crate::org::xml::sax::ext::LexicalHandler;', '// SAX LexicalHandler not available')
content = content.replace('use crate::org::xml::sax::helpers::XMLFilterImpl;', '// SAX XMLFilterImpl not available')

# Add valid imports
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.BufferedWriter\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.IOException\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.OutputStream\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.OutputStreamWriter\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.UnsupportedEncodingException\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.io.Writer\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.util.HashMap\n',
    'use std::collections::HashMap;\n')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.util.List\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.util.Map\n',
    '')
content = content.replace(
    '// TODO_RUST_CODE use ???; // java.util.StringTokenizer\n',
    '')

# 2. Fix static fields
content = content.replace(
    "pub(crate) const PAD_TEXT: String = /* TODO */;",
    'const PAD_TEXT: &str = " ";')
content = content.replace(
    "static LEXICAL_HANDLER_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();",
    'const LEXICAL_HANDLER_NAMES: &[&str] = &["http://xml.org/sax/properties/lexical-handler", "http://xml.org/sax/handlers/LexicalHandler"];')
content = content.replace(
    "static DEFAULT_FORMAT: std::sync::OnceLock<OutputFormat> = std::sync::OnceLock::new();",
    "// DEFAULT_FORMAT: handled inline")

# 3. Fix struct field types
content = content.replace("pub(crate) lastOutputNodeType: Arc<Mutex<NodeType>>,", 
    "pub(crate) lastOutputNodeType: Option<super::super::NodeType::NodeType>,")
content = content.replace("pub(crate) writer: Arc<Mutex<Writer>>,", 
    "pub(crate) writer: Option<Box<dyn std::io::Write>>,")
content = content.replace("namespaceStack: Option<NamespaceStack>,", 
    "namespaceStack: Option<Box<dyn std::any::Any>>,")
content = content.replace("format: Option<OutputFormat>,", 
    "format: Option<super::OutputFormat::OutputFormat>,")
content = content.replace("buffer: Option<StringBuffer>,", 
    "buffer: String,")
content = content.replace("pub(crate) lexicalHandler: Arc<Mutex<LexicalHandler>>,", 
    "pub(crate) lexicalHandler: Option<Box<dyn std::any::Any>>,")

# 4. Replace ALL // TODO_RUST_CODE with todo!()
# For standalone TODO_RUST_CODE lines (not use statements)
content = re.sub(r'^\s*// TODO_RUST_CODE\s*$', '      todo!();', content, flags=re.MULTILINE)

# 5. Fix function signatures
# First, rename all constructors and overloaded methods
lines = content.split('\n')
out = []

# Track method names for uniquifying
name_counts = {}
constructor_index = 0

i = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    
    # Fix constructor signatures: pub fn XMLWriter(&self, ...) -> pub fn new_X() -> Self
    constr_match = re.match(r'(\s+)pub fn XMLWriter\(&self(.*)\)\s*\{', line)
    if constr_match:
        indent = constr_match.group(1)
        params = constr_match.group(2).strip()
        
        if constructor_index == 0:
            out.append(f'{indent}pub fn new() -> Self {{')
        else:
            out.append(f'{indent}pub fn new_{constructor_index}() -> Self {{')
        constructor_index += 1
        i += 1
        continue
    
    # Fix method signatures: pub fn name(&self, JavaType param, ...) -> ...
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
            # Look at the Java annotation above to determine suffix
            prev_line = out[-1] if out else ''
            ann_text = prev_line.strip() if prev_line.strip().startswith('// Java:') else ''
            
            if name == 'write':
                if 'Document doc' in ann_text: actual_name = 'write_document'
                elif 'Element element' in ann_text: actual_name = 'write_element'
                elif 'CDATA cdata' in ann_text: actual_name = 'write_cdata'
                elif 'Comment comment' in ann_text: actual_name = 'write_comment'
                elif 'Entity entity' in ann_text: actual_name = 'write_entity'
                elif 'Namespace namespace' in ann_text: actual_name = 'write_namespace'
                elif 'ProcessingInstruction' in ann_text: actual_name = 'write_pi'
                elif 'String text' in ann_text: actual_name = 'write_string'
                elif 'Text text' in ann_text: actual_name = 'write_text'
                elif 'Node node' in ann_text: actual_name = 'write_node'
                elif 'Object object' in ann_text: actual_name = 'write_object'
                else: actual_name = f'write_{name_counts[name]}'
            elif name == 'writeClose':
                if 'String qualifiedName' in ann_text: actual_name = 'writeClose_qname'
                else: actual_name = f'writeClose_{name_counts[name]}'
            elif name == 'writeNamespace':
                if 'String prefix' in ann_text: actual_name = 'writeNamespace_prefix_uri'
                else: actual_name = f'writeNamespace_{name_counts[name]}'
            elif name == 'writeAttributes':
                if 'Attributes attributes' in ann_text: actual_name = 'writeAttributes_sax'
                else: actual_name = f'writeAttributes_{name_counts[name]}'
            elif name == 'writeAttribute':
                if 'Attributes attributes' in ann_text: actual_name = 'writeAttribute_sax_index'
                else: actual_name = f'writeAttribute_{name_counts[name]}'
            else:
                actual_name = f'{name}_{name_counts[name]}'
        
        # Fix return type
        rust_ret = ''
        if ret:
            if '-> Writer' in ret: rust_ret = ' -> Box<dyn std::io::Write>'
            elif '-> OutputFormat' in ret: rust_ret = ' -> &super::OutputFormat::OutputFormat'
            elif '-> LexicalHandler' in ret: rust_ret = " -> Option<&dyn std::any::Any>"
            elif '-> Object' in ret: rust_ret = ' -> Option<Box<dyn std::any::Any>>'
            elif '-> NodeType' in ret: rust_ret = ' -> Option<super::super::NodeType::NodeType>'
            else: rust_ret = f' {ret}'
        
        out.append(f'{indent}pub fn {actual_name}(&mut self){rust_ret} {{')
        i += 1
        continue
    
    out.append(line)
    i += 1

content = '\n'.join(out)

# 6. Remove Arc/Mutex imports if present
content = content.replace('use std::sync::Arc;\n', '')
content = content.replace('use std::sync::Mutex;\n', '')

with open(rust_path, 'w', encoding='utf-8') as f:
    f.write(content)

print(f"XMLWriter.rs transformed successfully")
print(f"Lines: {len(content.split(chr(10)))}")

import re

path = r'd:\agentic\src\wordOffice\libs\core-libs\io\wps-io\src\cn\wps\io\dom\io\XMLWriter.rs'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()

# Fix imports - remove all use statements and replace with valid ones
content = re.sub(r'use io_wps_io::.*\n', '', content)
content = re.sub(r'use crate::io::dom::Element;\n', '', content)
content = re.sub(r'use crate::io::dom::Namespace;\n', '', content)
content = re.sub(r'use crate::io::dom::tree::NamespaceStack;\n', '', content)
content = re.sub(r'// TODO_RUST_CODE use \?\?\?;.*\n', '', content)
content = re.sub(r'use kfc_kfc_alpha::.*\n', '', content)
content = re.sub(r'use crate::org::xml::sax::.*\n', '', content)

# Fix the static constants
content = content.replace("pub(crate) const PAD_TEXT: String = /* TODO */;", "const PAD_TEXT: &str = \" \";")
content = content.replace("static LEXICAL_HANDLER_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();",
    'const LEXICAL_HANDLER_NAMES: &[&str] = &["http://xml.org/sax/properties/lexical-handler", "http://xml.org/sax/handlers/LexicalHandler"];')
content = content.replace("static DEFAULT_FORMAT: std::sync::OnceLock<OutputFormat> = std::sync::OnceLock::new();", "")

# Fix struct field types
content = content.replace("pub(crate) lastOutputNodeType: Arc<Mutex<NodeType>>,", "pub(crate) lastOutputNodeType: Option<super::super::NodeType::NodeType>,")
content = content.replace("pub(crate) writer: Arc<Mutex<Writer>>,", "pub(crate) writer: Option<Box<dyn std::io::Write>>,")
content = content.replace("namespaceStack: Option<NamespaceStack>,", "namespaceStack: Option<Box<dyn std::any::Any>>,")
content = content.replace("format: Option<OutputFormat>,", "format: Option<super::OutputFormat::OutputFormat>,")
content = content.replace("buffer: Option<StringBuffer>,", "buffer: String,")
content = content.replace("pub(crate) lexicalHandler: Arc<Mutex<LexicalHandler>>,", "pub(crate) lexicalHandler: Option<Box<dyn std::any::Any>>,")
content = content.replace("namespacesMap: Option<HashMap<String, String>>,", "namespacesMap: Option<std::collections::HashMap<String, String>>,")

# Add use statements after the import comments
import_end = content.find('\n// Java field:')
if import_end == -1:
    import_end = content.find('\n// Java: public class')

# Now fix all the function signatures - replace Java-style params with Rust params
# Pattern: pub fn XXX(&self, TypeName param) or pub fn XXX(&self, TypeName param, TypeName param2)
def fix_fn_sig(content):
    # Fix constructor-like signatures 
    lines = content.split('\n')
    out = []
    skip_until_close = False
    brace_depth = 0
    method_lines = []
    in_method = False
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Match broken function signatures (Java-style params)
        fn_match = re.match(r'(\s*)pub fn (\w+)\(&self(.*)\)\s*(\{?)\s*$', stripped)
        if fn_match and ('String ' in stripped or 'int ' in stripped or 'boolean ' in stripped or 
                         'char ' in stripped or 'Writer ' in stripped or 'OutputFormat ' in stripped or
                         'OutputStream ' in stripped or 'InputSource ' in stripped or 
                         'LexicalHandler ' in stripped or 'Object ' in stripped or
                         'OpenXmlFileType ' in stripped or 'Element ' in stripped or
                         'Attribute ' in stripped or 'CDATA ' in stripped or 'Comment ' in stripped or
                         'Entity ' in stripped or 'Namespace ' in stripped or 'ProcessingInstruction ' in stripped or
                         'Text ' in stripped or 'Node ' in stripped or 'Document ' in stripped or
                         'Locator ' in stripped or 'XMLReader ' in stripped):
            # This function has Java-style params - need to fix
            # For now, just replace with proper Rust params
            # Find matching method annotation above
            out.append(line)
            i += 1
            continue
        
        out.append(line)
        i += 1
    
    return '\n'.join(out)

# Simpler approach: just replace all broken fn signatures
# Replace "pub fn XXX(&self, Java params)" patterns
def fix_all_fns(content):
    # Pattern for broken constructors: pub fn XMLWriter(&self, ...)
    content = re.sub(
        r'pub fn XMLWriter\(&self, Writer writer\)\s*\{',
        'pub fn new_with_writer() -> Self {', content)
    content = re.sub(
        r'pub fn XMLWriter\(&self, Writer writer, OutputFormat format\)\s*\{',
        'pub fn new_with_writer_format() -> Self {', content)
    content = re.sub(
        r'pub fn XMLWriter\(&self\)\s*\{',
        'pub fn new() -> Self {', content)
    content = re.sub(
        r'pub fn XMLWriter\(&self, OutputStream out\)\s*\{',
        'pub fn new_with_output() -> Self {', content)
    content = re.sub(
        r'pub fn XMLWriter\(&self, OutputStream out, OutputFormat format\)\s*\{',
        'pub fn new_with_output_format() -> Self {', content)
    content = re.sub(
        r'pub fn XMLWriter\(&self, OutputFormat format\)\s*\{',
        'pub fn new_with_format() -> Self {', content)
    
    # Fix methods with Java-style params: replace type-name pairs with name: type
    java_types = {
        'Writer': '&mut dyn std::io::Write',
        'OutputStream': '&mut dyn std::io::Write',
        'OutputFormat': '&super::OutputFormat::OutputFormat',
        'String': '&str',
        'boolean': 'bool',
        'int': 'i32',
        'char': 'char',
        'InputSource': '&dyn std::any::Any',
        'LexicalHandler': 'Box<dyn std::any::Any>',
        'Object': '&dyn std::any::Any',
        'Locator': '&dyn std::any::Any',
        'XMLReader': '&dyn std::any::Any',
        'Element': '&dyn std::any::Any',
        'Attribute': '&dyn std::any::Any',
        'CDATA': '&dyn std::any::Any',
        'Comment': '&dyn std::any::Any',
        'Entity': '&dyn std::any::Any',
        'Namespace': '&dyn std::any::Any',
        'ProcessingInstruction': '&dyn std::any::Any',
        'Text': '&dyn std::any::Any',
        'Node': '&dyn std::any::Any',
        'Document': '&dyn std::any::Any',
        'Attributes': '&dyn std::any::Any',
    }
    
    # Replace patterns like "Type name" in function params
    for java_type, rust_type in java_types.items():
        # Match params: , Type name  or ( Type name
        pattern = rf'(?<=\()({java_type}) (\w+)(?=[,\)])'
        replacement = rf'\2: {rust_type}'
        content = re.sub(pattern, replacement, content)
        
        pattern = rf', ({java_type}) (\w+)(?=[,\)])'
        replacement = rf', \2: {rust_type}'
        content = re.sub(pattern, replacement, content)
    
    # Fix "char[] ch" pattern
    content = re.sub(r'char\[\] (\w+)', r'\1: &[char]', content)
    
    return content

content = fix_all_fns(content)

# Replace all TODO_RUST_CODE with nothing (the Java comments stay)
content = content.replace('      // TODO_RUST_CODE\n', '')
content = content.replace('         // TODO_RUST_CODE\n', '')
content = content.replace('            // TODO_RUST_CODE\n', '')
content = content.replace('               // TODO_RUST_CODE\n', '')
content = content.replace('                  // TODO_RUST_CODE\n', '')
content = content.replace('                     // TODO_RUST_CODE\n', '')

# Fix return types - replace Java return types
# Pattern: -> Type { where Type is a Java type
content = re.sub(r'-> Writer\s*\{', '-> Box<dyn std::io::Write> {', content)
content = re.sub(r'-> OutputFormat\s*\{', '-> &super::OutputFormat::OutputFormat {', content)
content = re.sub(r'-> LexicalHandler\s*\{', '-> Option<&dyn std::any::Any> {', content)
content = re.sub(r'-> Object\s*\{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> NodeType\s*\{', '-> Option<super::super::NodeType::NodeType> {', content)

# Remove "Arc<Mutex<" references that are no longer valid
content = content.replace('use std::sync::Arc;\nuse std::sync::Mutex;\n', '')

with open(path, 'w', encoding='utf-8') as f:
    f.write(content)

print("XMLWriter.rs post-processed successfully")

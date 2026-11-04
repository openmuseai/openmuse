import re

filepath = r'src/cn/wps/io/dom/tree/DefaultElement.rs'

with open(filepath, 'r', encoding='utf-8') as f:
    content = f.read()

# 1. Fix imports - replace io_wps_io:: with crate::io:: and comment out broken ones
content = content.replace('use util::cn::wps::base::assertion::IOAssert;', '// use util::cn::wps::base::assertion::IOAssert;')
content = content.replace('use io_wps_io::cn::wps::io::dom::', 'use crate::io::dom::')
content = content.replace('use io_wps_io::cn::wps::io::dom::io::', 'use crate::io::dom::io::')
content = content.replace('use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;', '// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;')

# 2. Fix constants - replace /* TODO */ with actual values
content = content.replace("pub(crate) const TAG: String = /* TODO */;", 'const TAG: &str = "";')
content = content.replace("pub(crate) const DEBUG: bool = /* TODO */;", 'pub(crate) const DEBUG: bool = false;')
content = content.replace("pub(crate) const VERBOSE_TOSTRING: bool = /* TODO */;", 'pub(crate) const VERBOSE_TOSTRING: bool = false;')
content = content.replace("pub(crate) const USE_STRINGVALUE_SEPARATOR: bool = /* TODO */;", 'pub(crate) const USE_STRINGVALUE_SEPARATOR: bool = false;')
content = content.replace("pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = /* TODO */;", 'pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = 140;')

# 3. Fix static field types
content = content.replace(
    'static sPoolSync: std::sync::OnceLock<Object> = std::sync::OnceLock::new();',
    'static S_POOL_SYNC: std::sync::Mutex<()> = std::sync::Mutex::new(());'
)
content = content.replace(
    'static sPool: std::sync::OnceLock<std::sync::Mutex<DefaultElement>> = std::sync::OnceLock::new();',
    'static S_POOL: std::sync::OnceLock<std::sync::Mutex<Option<Box<DefaultElement>>>> = std::sync::OnceLock::new();'
)
content = content.replace(
    'static sPoolSize: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();',
    'static S_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();'
)
content = content.replace(
    'static sMaxPoolSize: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();',
    'static S_MAX_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();'
)
content = content.replace(
    'static sNewInsCount: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();',
    'static S_NEW_INS_COUNT: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();'
)

# 4. Fix struct fields - replace Arc<Mutex<>> with Option<Box<>>
content = content.replace('pub(crate) mLocalName: Arc<Mutex<String>>,', 'pub(crate) mLocalName: Option<Box<dyn std::any::Any + Send + Sync>>,')
content = content.replace('pub(crate) mPrefix: Arc<Mutex<String>>,', 'pub(crate) mPrefix: Option<Box<dyn std::any::Any + Send + Sync>>,')
content = content.replace('pub(crate) mNamespaceURI: Arc<Mutex<String>>,', 'pub(crate) mNamespaceURI: Option<Box<dyn std::any::Any + Send + Sync>>,')
content = content.replace('pub(crate) parentBranch: Arc<Mutex<Branch>>,', 'pub(crate) parentBranch: Option<Box<dyn std::any::Any + Send + Sync>>,')
content = content.replace('pub(crate) mContent: Arc<Mutex<Vec<Node>>>,', 'pub(crate) mContent: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,')
content = content.replace('pub(crate) mAttributes: Arc<Mutex<Vec<Attribute>>>,', 'pub(crate) mAttributes: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,')
content = content.replace('next: Option<DefaultElement>,', 'next: Option<Box<DefaultElement>>,')

# 5. Add field annotations inside impl block
impl_line = 'impl DefaultElement {\n'
field_annotations = """    // Java field: String TAG
    // Java field: boolean DEBUG
    // Java field: boolean VERBOSE_TOSTRING
    // Java field: boolean USE_STRINGVALUE_SEPARATOR
    // Java field: Object sPoolSync
    // Java field: DefaultElement sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: int sNewInsCount
    // Java field: String mLocalName
    // Java field: String mPrefix
    // Java field: String mNamespaceURI
    // Java field: Branch parentBranch
    // Java field: List<Node> mContent
    // Java field: List<Attribute> mAttributes
    // Java field: DefaultElement next
"""
# Replace only the first occurrence of "impl DefaultElement {" (the main impl block)
idx = content.find(impl_line)
if idx >= 0:
    next_line_idx = idx + len(impl_line)
    content = content[:next_line_idx] + field_annotations + content[next_line_idx:]

# 6. Fix method signatures - replace Java-style params with Rust
# Replace (int xxx) params
content = re.sub(r'\((&self(?:, &mut self)?), int (\w+)\)', r'(\1, \2: i32)', content)
content = re.sub(r'\((&self(?:, &mut self)?), int (\w+), int (\w+)\)', r'(\1, \2: i32, \3: i32)', content)
content = re.sub(r'\((&self(?:, &mut self)?), int (\w+), Node (\w+)\)', r'(\1, \2: i32, \3: &dyn std::any::Any)', content)
# Replace (String xxx) params
content = re.sub(r'\((&self), String (\w+)\)', r'(\1, \2: &str)', content)
content = re.sub(r'\((&self), String (\w+), String (\w+)\)', r'(\1, \2: &str, \3: &str)', content)
content = re.sub(r'\((&self), String (\w+), String (\w+), String (\w+)\)', r'(\1, \2: &str, \3: &str, \4: &str)', content)
content = re.sub(r'\((&self), String (\w+), String (\w+), String (\w+), String (\w+)\)', r'(\1, \2: &str, \3: &str, \4: &str, \5: &str)', content)
content = re.sub(r'\((&self), String (\w+), int (\w+)\)', r'(\1, \2: &str, \3: i32)', content)
content = re.sub(r'\((&self), String (\w+), String (\w+), int (\w+)\)', r'(\1, \2: &str, \3: &str, \4: i32)', content)
# Fix Node, Element, Attribute, etc. params
content = re.sub(r'\((&self), Node (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Element (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Attribute (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), CDATA (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Comment (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Entity (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Namespace (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), ProcessingInstruction (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Text (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
content = re.sub(r'\((&self), Object (\w+)\)', r'(\1, \2: &dyn std::any::Any)', content)
# setAttributes special case
content = re.sub(
    r'\((&self), Attributes (\w+), int (\w+), NamespaceStack (\w+), boolean (\w+), MCEContext (\w+)\)',
    r'(\1, \2: &dyn std::any::Any, \3: i32, \4: &dyn std::any::Any, \5: bool, \6: &dyn std::any::Any)',
    content
)

# 7. Fix return types
# Element -> Option<Box<dyn Any>>
content = re.sub(r'-> Element \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Element\[\] \{', '-> Vec<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Node \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Document \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Attribute \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Namespace \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> ProcessingInstruction \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> NodeType \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Object \{', '-> Option<Box<dyn std::any::Any>> {', content)
content = re.sub(r'-> Branch \{', '-> Option<Box<dyn std::any::Any>> {', content)
# List<X> -> Vec<Box<dyn Any>>
content = re.sub(r'-> List<\w+> \{', '-> Vec<Box<dyn std::any::Any>> {', content)
# Iterator<X> -> Box<dyn Iterator<...>>
content = re.sub(r'-> Iterator<\w+> \{', '-> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {', content)
# BackedList<X> -> Vec<Box<dyn Any>>
content = re.sub(r'-> BackedList<\w+> \{', '-> Vec<Box<dyn std::any::Any>> {', content)
# String -> String
content = re.sub(r'-> String \{', '-> String {', content)

# 8. Replace // TODO_RUST_CODE with todo!();
content = content.replace('// TODO_RUST_CODE', 'todo!();')

# 9. Comment out MCEContext import
content = content.replace('use crate::io::dom::io::MCEContext;', '// use crate::io::dom::io::MCEContext;')

# 10. Comment out impl Element for DefaultElement
content = content.replace('impl Element for DefaultElement {', '// impl Element for DefaultElement {')
content = content.replace('\n}\n', '\n}\n', content.count('\n}\n') - 1)  # keep last closing brace

# 11. Fix overloaded methods - rename duplicates
seen_methods = {}
lines = content.split('\n')
new_lines = []
for line in lines:
    m = re.match(r'^(\s+pub fn )(\w+)\(', line)
    if m:
        name = m.group(2)
        if name in seen_methods:
            seen_methods[name] += 1
            line = line.replace(f'pub fn {name}(', f'pub fn {name}_{seen_methods[name]}(', 1)
        else:
            seen_methods[name] = 1
    new_lines.append(line)
content = '\n'.join(new_lines)

with open(filepath, 'w', encoding='utf-8', newline='\n') as f:
    f.write(content)

print("Fixed DefaultElement.rs")

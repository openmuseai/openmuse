import re

java_path = r"d:\agentic\src\wordOffice\koffice\core-libs\io\wps-io\src\cn\wps\io\dom\io\WMap.java"

with open(java_path, 'r', encoding='utf-8') as f:
    java_content = f.read()

lines = java_content.split('\n')

const_lines = []
put_lines = []
in_init = False

for line in lines:
    stripped = line.strip()
    m = re.match(r'private static final String (\w+) = "(.*?)";', stripped)
    if m:
        const_name = m.group(1)
        const_val = m.group(2)
        const_lines.append((const_name, const_val))
        continue
    
    m2 = re.match(r'public static String (\w+) = "(.*?)";', stripped)
    if m2:
        continue
    
    if 'private void init()' in stripped:
        in_init = True
        continue
    
    if in_init:
        m3 = re.match(r'this\.mapobj\.put\("(.*?)", (-?\d+)\);', stripped)
        if m3:
            put_lines.append((m3.group(1), m3.group(2)))

out_lines = []
out_lines.append('use super::super::simple::StringIntMap::StringIntMap;')
out_lines.append('use super::super::simple::XmlName2Id::XmlName2Id;')
out_lines.append('')
out_lines.append('static OBJ: std::sync::OnceLock<WMap> = std::sync::OnceLock::new();')
out_lines.append('')
out_lines.append('// Java: public class WMap implements XmlName2Id')
out_lines.append('pub struct WMap {')
out_lines.append('    // Java field: String PREFIX')
out_lines.append('    // Java field: String MAIN_NAMESPACING')
out_lines.append('    // Java field: String R_NAMESPACING')

for name, val in const_lines:
    out_lines.append(f'    // Java field: String {name}')

out_lines.append('    // Java field: WMap obj')
out_lines.append('    // Java field: StringIntMap mapobj')
out_lines.append('    pub(crate) mapobj: StringIntMap,')
out_lines.append('}')
out_lines.append('')
out_lines.append('// Java: public class WMap implements XmlName2Id')
out_lines.append('impl WMap {')
out_lines.append('    pub const PREFIX: &\'static str = "w";')
out_lines.append('    pub const MAIN_NAMESPACING: &\'static str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";')
out_lines.append('    pub const R_NAMESPACING: &\'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";')

for name, val in const_lines:
    out_lines.append(f'    const {name}: &\'static str = "{val}";')

out_lines.append('')
out_lines.append('    // Java: void init()')
out_lines.append('    fn init(&mut self) {')
for key, val in put_lines:
    out_lines.append(f'        // Java: this.mapobj.put("{key}", {val});')
    out_lines.append(f'        self.mapobj.put("{key}", {val});')
out_lines.append('    }')
out_lines.append('')
out_lines.append('    // Java: WMap()')
out_lines.append('    pub fn new() -> Self {')
out_lines.append('        // Java: super();')
out_lines.append('        let mut wmap = WMap {')
out_lines.append('            mapobj: StringIntMap::new(11),')
out_lines.append('        };')
out_lines.append('        // Java: this.init();')
out_lines.append('        wmap.init();')
out_lines.append('        wmap')
out_lines.append('    }')
out_lines.append('')
out_lines.append('    // Java: static WMap getInstance()')
out_lines.append('    pub fn getInstance() -> &\'static WMap {')
out_lines.append('        // Java: if (obj == null)')
out_lines.append('        // Java: obj = new WMap();')
out_lines.append('        // Java: return obj;')
out_lines.append('        OBJ.get_or_init(|| WMap::new())')
out_lines.append('    }')
out_lines.append('')
out_lines.append('    // Java: int getId(String uri, String name)')
out_lines.append('    pub fn getId_str(&self, _uri: &str, name: &str) -> i32 {')
out_lines.append('        // Java: return this.mapobj.get(name);')
out_lines.append('        self.mapobj.get(name)')
out_lines.append('    }')
out_lines.append('')
out_lines.append('    // Java: int getId(int prefix, int name)')
out_lines.append('    pub fn getId_int(&self, _prefix: i32, name: i32) -> i32 {')
out_lines.append('        // Java: return this.mapobj.get(name);')
out_lines.append('        self.mapobj.get_by_hash(name)')
out_lines.append('    }')
out_lines.append('}')
out_lines.append('')
out_lines.append('impl XmlName2Id for WMap {')
out_lines.append('    // Java: abstract int getId(String var1, String var2)')
out_lines.append('    fn getId_str(&self, var1: &str, var2: &str) -> i32 {')
out_lines.append('        self.mapobj.get(var2)')
out_lines.append('    }')
out_lines.append('')
out_lines.append('    // Java: abstract int getId(int var1, int var2)')
out_lines.append('    fn getId_int(&self, _var1: i32, var2: i32) -> i32 {')
out_lines.append('        self.mapobj.get_by_hash(var2)')
out_lines.append('    }')
out_lines.append('}')
out_lines.append('')

rust_path = r"d:\agentic\src\wordOffice\libs\core-libs\io\wps-io\src\cn\wps\io\dom\io\WMap.rs"
with open(rust_path, 'w', encoding='utf-8') as f:
    f.write('\n'.join(out_lines))

print(f"Generated WMap.rs with {len(const_lines)} constants and {len(put_lines)} put calls")

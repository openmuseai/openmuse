#![allow(dead_code, non_snake_case, non_upper_case_globals)]
use std::collections::HashMap;
use std::sync::OnceLock;
// Java: import cn.wps.kfc.xpxml.reader.XmlName2Id
// Java: import cn.wps.kfc.xsd.StringIntMap

// Java field: String URI
pub(crate) const URI_str: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
// Java field: String PREFIX
pub(crate) const PREFIX_str: &str = "mc";
// Java field: String Ignorable
pub(crate) const Ignorable_str: &str = "Ignorable";
// Java field: String MustUnderstand
pub(crate) const MustUnderstand_str: &str = "MustUnderstand";
// Java field: String ProcessContent
pub(crate) const ProcessContent_str: &str = "ProcessContent";
// Java field: String AlternateContent
pub(crate) const AlternateContent_str: &str = "AlternateContent";
// Java field: String Choice
pub(crate) const Choice_str: &str = "Choice";
// Java field: String Requires
pub(crate) const Requires_str: &str = "Requires";
// Java field: String Fallback
pub(crate) const Fallback_str: &str = "Fallback";
// Java field: McMap obj
static OBJ: OnceLock<McMap> = OnceLock::new();

// Java: public class McMap implements XmlName2Id
pub struct McMap {
    // Java field: StringIntMap mapobj
    pub(crate) mapobj: HashMap<i32, i32>,
}

// Java: public class McMap implements XmlName2Id
impl McMap {
    // Java: void init()
    fn init(&mut self) {
      // Java: this.mapobj.put("Ignorable", 2162689);
      self.mapobj.insert(hash_str("Ignorable"), 2162689);
      // Java: this.mapobj.put("MustUnderstand", 2162690);
      self.mapobj.insert(hash_str("MustUnderstand"), 2162690);
      // Java: this.mapobj.put("ProcessContent", 2162691);
      self.mapobj.insert(hash_str("ProcessContent"), 2162691);
      // Java: this.mapobj.put("AlternateContent", 2162692);
      self.mapobj.insert(hash_str("AlternateContent"), 2162692);
      // Java: this.mapobj.put("Choice", 2162693);
      self.mapobj.insert(hash_str("Choice"), 2162693);
      // Java: this.mapobj.put("Requires", 2162694);
      self.mapobj.insert(hash_str("Requires"), 2162694);
      // Java: this.mapobj.put("Fallback", 2162695);
      self.mapobj.insert(hash_str("Fallback"), 2162695);
    }

    // Java: McMap()
    fn new_internal() -> Self {
      // Java: super();
      let mut m = McMap {
          mapobj: HashMap::new(),
      };
      // Java: this.init();
      m.init();
      m
    }

    // Java: static McMap getInstance()
    pub fn getInstance() -> &'static McMap {
      // Java: if (obj == null)
      // Java: obj = new McMap();
      // Java: return obj;
      OBJ.get_or_init(|| McMap::new_internal())
    }

    // Java: int getId(int prefix, int name)
    pub fn getId(&self, _prefix: i32, name: i32) -> i32 {
      // Java: return this.mapobj.get(name);
      *self.mapobj.get(&name).unwrap_or(&-1)
    }

}

fn hash_str(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

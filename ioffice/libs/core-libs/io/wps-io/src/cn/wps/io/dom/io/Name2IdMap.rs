#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.XmlName2Id
use crate::io::simple::XmlName2Id::XmlName2Id;
use crate::io::dom::io::WMap::WMap;
use crate::io::dom::io::W14Map::{self as w14_mod, W14Map};
use crate::io::dom::io::McMap::{self as mc_mod, McMap};
use crate::io::dom::io::MMap::{self as m_mod, MMap};
use crate::io::dom::io::VMap::VMap;
use crate::io::dom::io::OMap::OMap;
use crate::io::dom::io::W10Map::W10Map;

use std::collections::HashMap;
use std::sync::OnceLock;

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

#[derive(Clone, Copy)]
enum MapKind {
    W,
    W14,
    Mc,
    M,
}

impl MapKind {
    fn get_id_str(self, uri: &str, name: &str) -> i32 {
        match self {
            MapKind::W => WMap::getInstance().getId_str(uri, name),
            MapKind::W14 => W14Map::getInstance().getId_str(uri, name),
            MapKind::Mc => McMap::getInstance().getId_str(uri, name),
            MapKind::M => MMap::getInstance().getId_str(uri, name),
        }
    }

    fn get_id_int(self, prefix: i32, name: i32) -> i32 {
        match self {
            MapKind::W => WMap::getInstance().getId_int(prefix, name),
            MapKind::W14 => W14Map::getInstance().getId_int(prefix, name),
            MapKind::Mc => McMap::getInstance().getId_int(prefix, name),
            MapKind::M => MMap::getInstance().getId_int(prefix, name),
        }
    }
}

// Java: public class Name2IdMap implements XmlName2Id
pub struct Name2IdMap {
    // Java field: HashMap<Integer, XmlName2Id> mName2IdMap
    mName2IdMap: HashMap<i32, MapKind>,
}

// Java field: Name2IdMap obj
static OBJ: OnceLock<Name2IdMap> = OnceLock::new();

// Java: public class Name2IdMap implements XmlName2Id
impl Name2IdMap {
    // Java: Name2IdMap()
    fn new() -> Self {
        // Java: super();
        let mut inst = Name2IdMap {
            mName2IdMap: HashMap::new(),
        };
        // Java: this.init();
        inst.init();
        inst
    }

    // Java: void init()
    fn init(&mut self) {
        // Java: this.mName2IdMap.put(WMap.PREFIX.hashCode(), WMap.getInstance());
        self.mName2IdMap.insert(java_string_hash(WMap::PREFIX), MapKind::W);
        // Java: this.mName2IdMap.put(W14Map.PREFIX.hashCode(), W14Map.getInstance());
        self.mName2IdMap.insert(java_string_hash(w14_mod::PREFIX), MapKind::W14);
        // Java: this.mName2IdMap.put(McMap.PREFIX.hashCode(), McMap.getInstance());
        self.mName2IdMap.insert(java_string_hash(mc_mod::PREFIX), MapKind::Mc);
        // Java: this.mName2IdMap.put(MMap.PREFIX.hashCode(), MMap.getInstance());
        self.mName2IdMap.insert(java_string_hash(m_mod::PREFIX), MapKind::M);
    }

    // Java: static Name2IdMap getInstance()
    pub fn getInstance() -> &'static Name2IdMap {
        // Java: if (null == obj)
        // Java: obj = new Name2IdMap();
        // Java: return obj;
        OBJ.get_or_init(Name2IdMap::new)
    }

    // Java: static int getId3(String uri, String name)
    pub fn getId3(uri: &str, name: &str) -> i32 {
        // Java: if (uri.equalsIgnoreCase(WMap.MAIN_NAMESPACING))
        if uri.eq_ignore_ascii_case(WMap::MAIN_NAMESPACING()) {
        // Java: return WMap.getInstance().getId(uri, name);
          return WMap::getInstance().getId_str(uri, name);
        }
        // Java: else if (uri.equalsIgnoreCase("http://schemas.microsoft.com/office/word/2010/wordml"))
        else if uri.eq_ignore_ascii_case("http://schemas.microsoft.com/office/word/2010/wordml") {
        // Java: return W14Map.getInstance().getId(uri, name);
          return W14Map::getInstance().getId_str(uri, name);
        }
        // Java: else if (uri.equalsIgnoreCase("urn:schemas-microsoft-com:vml"))
        else if uri.eq_ignore_ascii_case("urn:schemas-microsoft-com:vml") {
        // Java: return VMap.getInstance().getId(uri, name);
          return VMap::getInstance().getId_str(uri, name);
        }
        // Java: else if (uri.equalsIgnoreCase("http://schemas.openxmlformats.org/markup-compatibility/2006"))
        else if uri.eq_ignore_ascii_case("http://schemas.openxmlformats.org/markup-compatibility/2006") {
        // Java: return McMap.getInstance().getId(uri, name);
          return McMap::getInstance().getId_str(uri, name);
        }
        // Java: else if (uri.equalsIgnoreCase("http://schemas.openxmlformats.org/officeDocument/2006/math"))
        else if uri.eq_ignore_ascii_case("http://schemas.openxmlformats.org/officeDocument/2006/math") {
        // Java: return MMap.getInstance().getId(uri, name);
          return MMap::getInstance().getId_str(uri, name);
        }
        // Java: else if (uri.equalsIgnoreCase("urn:schemas-microsoft-com:office:office"))
        else if uri.eq_ignore_ascii_case("urn:schemas-microsoft-com:office:office") {
        // Java: return OMap.getInstance().getId(uri, name);
          return OMap::getInstance().getId_str(uri, name);
        }
        // Java: else
        else {
        // Java: return uri.equalsIgnoreCase("urn:schemas-microsoft-com:office:word") ? W10Map.getInstance().getId(uri, name) : -1;
          return if uri.eq_ignore_ascii_case("urn:schemas-microsoft-com:office:word") {
            W10Map::getInstance().getId_str(uri, name)
          } else {
            -1
          };
        }
    }

    // Java: int getId(String uri, String name)
    pub fn getId_str(&self, uri: &str, name: &str) -> i32 {
        // Java: XmlName2Id xmlName2Id = (XmlName2Id)this.mName2IdMap.get(uri.hashCode());
        let xmlName2Id = self.mName2IdMap.get(&java_string_hash(uri));
        // Java: return null == xmlName2Id ? -1 : xmlName2Id.getId(uri, name);
        match xmlName2Id {
            None => -1,
            Some(kind) => kind.get_id_str(uri, name),
        }
    }

    // Java: int getId(int prefix, int name)
    pub fn getId_int(&self, prefix: i32, name: i32) -> i32 {
        // Java: XmlName2Id xmlName2Id = (XmlName2Id)this.mName2IdMap.get(prefix);
        let xmlName2Id = self.mName2IdMap.get(&prefix);
        // Java: return null == xmlName2Id ? -1 : xmlName2Id.getId(prefix, name);
        match xmlName2Id {
            None => -1,
            Some(kind) => kind.get_id_int(prefix, name),
        }
    }
}

impl XmlName2Id for Name2IdMap {
    fn getId_str(&self, var1: &str, var2: &str) -> i32 {
        Name2IdMap::getId_str(self, var1, var2)
    }
    fn getId_int(&self, var1: i32, var2: i32) -> i32 {
        Name2IdMap::getId_int(self, var1, var2)
    }
}

// java package cn.wps.moss.xlsxr.util;

// java import cn.wps.kfc.xpxml.reader.XmlName2Id;
// java import cn.wps.kfc.xsd.custom.DSMap;
// java import cn.wps.kfc.xsd.custom.WpsweMap;
// java import cn.wps.kfc.xsd.dml.A14Map;
// java import cn.wps.kfc.xsd.dml.AMap;
// java import cn.wps.kfc.xsd.dml.C14Map;
// java import cn.wps.kfc.xsd.dml.C15Map;
// java import cn.wps.kfc.xsd.dml.C16R3Map;
// java import cn.wps.kfc.xsd.dml.CMap;
// java import cn.wps.kfc.xsd.dml.CSMap;
// java import cn.wps.kfc.xsd.dml.DgmMap;
// java import cn.wps.kfc.xsd.dml.DspMap;
// java import cn.wps.kfc.xsd.dml.LcMap;
// java import cn.wps.kfc.xsd.etc.EtcMap;
// java import cn.wps.kfc.xsd.mce.McMap;
// java import cn.wps.kfc.xsd.shared.CustomMap;
// java import cn.wps.kfc.xsd.shared.OMap;
// java import cn.wps.kfc.xsd.shared.RMap;
// java import cn.wps.kfc.xsd.shared.XMap;
// java import cn.wps.kfc.xsd.sml.ExtMap;
// java import cn.wps.kfc.xsd.sml.SMap;
// java import cn.wps.kfc.xsd.sml.X15Map;
// java import cn.wps.kfc.xsd.sml.XMMap;
// java import cn.wps.kfc.xsd.vml.VMap;
// java import cn.wps.kfc.xsd.vml.Xdr14Map;
// java import cn.wps.kfc.xsd.vml.XdrMap;
use std::collections::HashMap;
use std::sync::OnceLock;

use kfc_kfc_alpha::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
use kfc_kfc_alpha::kfc::xsd::shared::RMap::RMap;
use kfc_kfc_alpha::kfc::xsd::sml::SMap::SMap;
use kfc_kfc_alpha::kfc::xsd::sml::X15Map::X15Map;
use kfc_kfc_alpha::kfc::xsd::vml::XdrMap::XdrMap;
use kfc_kfc_alpha::kfc::xsd::mce::McMap::McMap;

#[derive(Clone, Copy)]
enum NameMap {
    S,
    A,
    R,
    Xdr,
    Mc,
    X15,
    Later,
}

// java public final class MapManager implements XmlName2Id
pub struct MapManager {
    // java private HashMap<Integer, XmlName2Id> maps_prefix = new HashMap();
    maps_prefix: HashMap<i32, NameMap>,
}

static OBJ: OnceLock<MapManager> = OnceLock::new();

// java public static final String TENCENT_ONLINE_PREFIX = "ns2";
pub const TENCENT_ONLINE_PREFIX: &str = "ns2";

// java public final class MapManager implements XmlName2Id
impl MapManager {
    // java private MapManager()
    fn MapManager() -> Self {
        let mut manager = Self {
            maps_prefix: HashMap::new(),
        };
        // java this.init();
        manager.init();
        manager
    }

    // java private void init()
    fn init(&mut self) {
        // java this.maps_prefix.put(-1, SMap.getInstance());
        self.maps_prefix.insert(-1, NameMap::S);
        // java this.maps_prefix.put("r".hashCode(), RMap.getInstance());
        self.maps_prefix.insert(java_hash("r"), NameMap::R);
        // java this.maps_prefix.put("d3p1".hashCode(), RMap.getInstance());
        self.maps_prefix.insert(java_hash("d3p1"), NameMap::R);
        // java this.maps_prefix.put("xdr".hashCode(), XdrMap.getInstance());
        self.maps_prefix.insert(java_hash("xdr"), NameMap::Xdr);
        // java this.maps_prefix.put("xdr14".hashCode(), Xdr14Map.getInstance());
        self.maps_prefix.insert(java_hash("xdr14"), NameMap::Later);
        // java this.maps_prefix.put("a".hashCode(), AMap.getInstance());
        self.maps_prefix.insert(java_hash("a"), NameMap::A);
        // java this.maps_prefix.put("c".hashCode(), CMap.getInstance());
        self.maps_prefix.insert(java_hash("c"), NameMap::Later);
        // java this.maps_prefix.put("dgm".hashCode(), DgmMap.getInstance());
        self.maps_prefix.insert(java_hash("dgm"), NameMap::Later);
        // java this.maps_prefix.put("dsp".hashCode(), DspMap.getInstance());
        self.maps_prefix.insert(java_hash("dsp"), NameMap::Later);
        // java this.maps_prefix.put("x14".hashCode(), ExtMap.getInstance());
        self.maps_prefix.insert(java_hash("x14"), NameMap::S);
        // java this.maps_prefix.put("xm".hashCode(), XMMap.getInstance());
        self.maps_prefix.insert(java_hash("xm"), NameMap::S);
        // java this.maps_prefix.put("vt".hashCode(), CustomMap.getInstance());
        self.maps_prefix.insert(java_hash("vt"), NameMap::Later);
        // java this.maps_prefix.put("o".hashCode(), OMap.getInstance());
        self.maps_prefix.insert(java_hash("o"), NameMap::Later);
        // java this.maps_prefix.put("v".hashCode(), VMap.getInstance());
        self.maps_prefix.insert(java_hash("v"), NameMap::Later);
        // java this.maps_prefix.put("x".hashCode(), XMap.getInstance());
        self.maps_prefix.insert(java_hash("x"), NameMap::Later);
        // java this.maps_prefix.put("mc".hashCode(), McMap.getInstance());
        self.maps_prefix.insert(java_hash("mc"), NameMap::Mc);
        // java this.maps_prefix.put("c14".hashCode(), C14Map.getInstance());
        self.maps_prefix.insert(java_hash("c14"), NameMap::Later);
        // java this.maps_prefix.put("cs".hashCode(), CSMap.getInstance());
        self.maps_prefix.insert(java_hash("cs"), NameMap::Later);
        // java this.maps_prefix.put("c15".hashCode(), C15Map.getInstance());
        self.maps_prefix.insert(java_hash("c15"), NameMap::Later);
        // java this.maps_prefix.put("x15".hashCode(), X15Map.getInstance());
        self.maps_prefix.insert(java_hash("x15"), NameMap::X15);
        // java this.maps_prefix.put("c16r3".hashCode(), C16R3Map.getInstance());
        self.maps_prefix.insert(java_hash("c16r3"), NameMap::Later);
        // java this.maps_prefix.put("c16r3".hashCode(), C16R3Map.getInstance());
        self.maps_prefix.insert(java_hash("c16r3"), NameMap::Later);
        // java this.maps_prefix.put("etc".hashCode(), EtcMap.getInstance());
        self.maps_prefix.insert(java_hash("etc"), NameMap::Later);
        // java this.maps_prefix.put("lc".hashCode(), LcMap.getInstance());
        self.maps_prefix.insert(java_hash("lc"), NameMap::Later);
        // java this.maps_prefix.put("a14".hashCode(), A14Map.getInstance());
        self.maps_prefix.insert(java_hash("a14"), NameMap::Later);
        // java this.maps_prefix.put("wpswe".hashCode(), WpsweMap.getInstance());
        self.maps_prefix.insert(java_hash("wpswe"), NameMap::Later);
        // java this.maps_prefix.put("ds".hashCode(), DSMap.getInstance());
        self.maps_prefix.insert(java_hash("ds"), NameMap::Later);
        // java this.maps_prefix.put("x".hashCode(), SMap.getInstance());
        self.maps_prefix.insert(java_hash("x"), NameMap::S);
        // java this.maps_prefix.put("s".hashCode(), SMap.getInstance());
        self.maps_prefix.insert(java_hash("s"), NameMap::S);
        // java this.maps_prefix.put("ns2".hashCode(), SMap.getInstance());
        self.maps_prefix.insert(java_hash("ns2"), NameMap::S);
    }

    // java public int getId(int prefix, int name)
    pub fn getId(&self, prefix: i32, name: i32) -> i32 {
        // java XmlName2Id xmlIds = (XmlName2Id)this.maps_prefix.get(prefix);
        let xml_ids = self.maps_prefix.get(&prefix).copied();
        // java if (null != xmlIds)
        if let Some(which) = xml_ids {
            // java int ret = xmlIds.getId(prefix, name);
            let ret = self.lookup(which, prefix, name);
            // java return -1 != ret ? ret : XdrMap.getInstance().getId(prefix, name);
            if ret != -1 {
                ret
            } else {
                XdrMap::getInstance().getId(prefix, name)
            }
        } else {
            // java xmlIds = SMap.getInstance();
            // java int ret = xmlIds.getId(prefix, name);
            let ret = SMap::getInstance().getId(prefix, name);
            // java if (-1 != ret)
            if ret != -1 {
                // java return ret;
                ret
            } else {
                // java xmlIds = XdrMap.getInstance();
                // java ret = xmlIds.getId(prefix, name);
                let ret = XdrMap::getInstance().getId(prefix, name);
                // java return -1 != ret ? ret : -1;
                if ret != -1 { ret } else { -1 }
            }
        }
    }

    // java public static MapManager getInstance()
    pub fn getInstance() -> &'static MapManager {
        // java if (obj == null)
        // java obj = new MapManager();
        let obj = OBJ.get_or_init(Self::MapManager);
        // java return obj;
        obj
    }

    fn lookup(&self, which: NameMap, prefix: i32, name: i32) -> i32 {
        match which {
            NameMap::A => kfc_kfc_alpha::kfc::xsd::dml::AMap::AMap::getInstance().getId(prefix,name),
            NameMap::S => SMap::getInstance().getId(prefix, name),
            NameMap::R => RMap::getInstance().getId(prefix, name),
            NameMap::Xdr => XdrMap::getInstance().getId(prefix, name),
            NameMap::Mc => McMap::getInstance().getId(prefix, name),
            NameMap::X15 => X15Map::getInstance().getId(prefix, name),
            NameMap::Later => deferred_map(),
        }
    }
}

impl XmlName2Id for MapManager {
    fn get_id(&self, prefix: i32, name: i32) -> i32 {
        self.getId(prefix, name)
    }
}

fn deferred_map() -> i32 {
    todo!("DEFERRED_NON_FUNCTION");
}

fn java_hash(s: &str) -> i32 {
    let mut h = 0i32;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(i32::from(c));
    }
    h
}

// Java: import cn.wps.kfc.xpxml.reader.XmlName2Id
use core_libs_kfc::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
// Java: import cn.wps.kfc.xsd.sml.SMap
use core_libs_kfc::kfc::xsd::sml::SMap::SMap;
// Java: import cn.wps.kfc.xsd.vml.XdrMap
use core_libs_kfc::kfc::xsd::vml::XdrMap::XdrMap;
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: String TENCENT_ONLINE_PREFIX
pub const TENCENT_ONLINE_PREFIX: &str = "ns2";

// Java: public class MapManager implements XmlName2Id
#[derive(Clone)]
pub struct MapManager {
    // Java field: HashMap<Integer, XmlName2Id> maps_prefix
    pub(crate) maps_prefix: HashMap<i32, i32>,
}

// Java field: MapManager obj
static OBJ: std::sync::OnceLock<MapManager> = std::sync::OnceLock::new();

// Java: public class MapManager implements XmlName2Id
impl MapManager {
    // Java: MapManager()
    fn MapManager() -> Self {
      // Java: super();
      let mut m = MapManager {
          maps_prefix: HashMap::new(),
      };
      // Java: this.init();
      m.init();
      m
    }

    // Java: void init()
    fn init(&mut self) {
      // Java: this.maps_prefix.put(-1, SMap.getInstance());
      self.maps_prefix.insert(-1, 0);
    }

    // Java: int getId(int prefix, int name)
    pub fn getId(&self, prefix: i32, name: i32) -> i32 {
      // Java: XmlName2Id xmlIds = (XmlName2Id)this.maps_prefix.get(prefix);
      let has_prefix = self.maps_prefix.contains_key(&prefix);
      // Java: if (null != xmlIds)
      if has_prefix {
         // Java: int ret = xmlIds.getId(prefix, name);
         let ret = SMap::getInstance().getId(prefix, name);
         // Java: return -1 != ret ? ret : XdrMap.getInstance().getId(prefix, name);
         if -1 != ret { ret } else { XdrMap::getInstance().getId(prefix, name) }
      }
      // Java: else
      else {
         // Java: xmlIds = SMap.getInstance();
         let smap = SMap::getInstance();
         // Java: int ret = xmlIds.getId(prefix, name);
         let ret = smap.getId(prefix, name);
         // Java: if (-1 != ret)
         if -1 != ret {
            // Java: return ret;
            ret
         }
         // Java: else
         else {
            // Java: xmlIds = XdrMap.getInstance();
            let xdr = XdrMap::getInstance();
            // Java: ret = xmlIds.getId(prefix, name);
            let ret = xdr.getId(prefix, name);
            // Java: return -1 != ret ? ret : -1;
            if -1 != ret { ret } else { -1 }
         }
      }
    }

    // Java: static MapManager getInstance()
    pub fn getInstance() -> &'static MapManager {
      // Java: if (obj == null)
      // Java: obj = new MapManager();
      // Java: return obj;
      OBJ.get_or_init(|| MapManager::MapManager())
    }

}

impl XmlName2Id for MapManager {
    fn get_id(&self, var1: i32, var2: i32) -> i32 {
        self.getId(var1, var2)
    }
}

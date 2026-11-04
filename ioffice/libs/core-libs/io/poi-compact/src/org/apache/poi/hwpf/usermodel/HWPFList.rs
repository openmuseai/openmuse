// Java: import org.apache.poi.hwpf.model.ListData
use crate::org::apache::poi::hwpf::model::ListData::ListData;
// Java: import org.apache.poi.hwpf.model.ListFormatOverride
use crate::org::apache::poi::hwpf::model::ListFormatOverride::ListFormatOverride;
// Java: import org.apache.poi.hwpf.model.StyleSheet
use crate::org::apache::poi::hwpf::model::StyleSheet::StyleSheet;

// Java: public class HWPFList
pub struct HWPFList {
    // Java field: ListData _listData
    pub(crate) _listData: ListData,
    // Java field: ListFormatOverride _override
    pub(crate) _override: ListFormatOverride,
    // Java field: boolean _registered
    _registered: bool,
    // Java field: StyleSheet _styleSheet
    _styleSheet: Option<StyleSheet>,
}

// Java: public class HWPFList
impl HWPFList {
    // Java: HWPFList(boolean numbered, StyleSheet styleSheet)
    pub fn new(numbered: bool, style_sheet: StyleSheet) -> Self {
      // Java: super();
      // Java: this._listData = new ListData((int)(Math.random() * (double)System.currentTimeMillis()), numbered);
      let random_id = (rand_i32() as i64).wrapping_mul(std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap_or_default()
          .as_millis() as i64) as i32;
      let _listData = ListData::new(random_id, numbered);
      // Java: this._override = new ListFormatOverride(this._listData.getLsid());
      let _override = ListFormatOverride::new(_listData.getLsid());
      // Java: this._styleSheet = styleSheet;
      HWPFList {
          _listData,
          _override,
          _registered: false,
          _styleSheet: Some(style_sheet),
      }
    }

    // Java: void setLevelStyle(int level, int styleIndex)
    pub fn setLevelStyle(&mut self, level: i32, style_index: i32) {
      // Java: this._listData.setLevelStyle(level, styleIndex);
      self._listData.setLevelStyle(level, style_index);
    }

    // Java: ListData getListData()
    pub fn getListData(&self) -> &ListData {
      // Java: return this._listData;
      &self._listData
    }

    // Java: ListFormatOverride getOverride()
    pub fn getOverride(&self) -> &ListFormatOverride {
      // Java: return this._override;
      &self._override
    }

}

fn rand_i32() -> i32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    hasher.finish() as i32
}

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.UUSti
use crate::cn::wps::moffice::writer::core::UUSti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.BuildinStyleSheet
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::*;
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::{HashMap};

// java public final class StyleId2Sti
// rust: ★M-STAGE2 P1-D1 —— Java 侧 `DocumentImporter` 与各 SubDocumentImporter
//   **共享同一实例**；Rust 侧以 `Clone` 近似（构造时传递真实状态，而非凭空新建）。
#[derive(Default, Clone)]
pub struct StyleId2Sti {
    // java private Map<String, Integer> mMapStyleId2Sti = new HashMap();
    mMapStyleId2Sti: Option<HashMap<String, i32>>,
    // java private UUSti mStiMaker;
    mStiMaker: UUSti,
}

// java public final class StyleId2Sti
impl StyleId2Sti {
    // java public  StyleId2Sti(UUSti stiMaker)
    pub fn StyleId2Sti(stiMaker: UUSti) -> Self {
      // java IOAssert.assertNotNull("stiMaker should not be null!", stiMaker);
      let sti_opt = Some(&stiMaker);
      IOAssert::assertNotNull_msg("stiMaker should not be null!", &sti_opt);
      // java this.mStiMaker = stiMaker;
      let this = Self {
        mMapStyleId2Sti: Some(HashMap::new()),
        mStiMaker: stiMaker,
      };
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mMapStyleId2Sti)
      if let Some(mMapStyleId2Sti) = self.mMapStyleId2Sti.as_mut() {
        // java this.mMapStyleId2Sti.clear();
        mMapStyleId2Sti.clear();
        // java this.mMapStyleId2Sti = null;
      }
      self.mMapStyleId2Sti = None;
      // java BuildinStyleSheet.clearMapName2TypeSti();
      BuildinStyleSheet::clearMapName2TypeSti();
    }

    // java public Integer getSti(String styleId)
    pub(crate) fn export_pairs(&self) -> Vec<(String, i32)> {
      self.mMapStyleId2Sti
        .as_ref()
        .map(|m| m.iter().map(|(k, v)| (k.clone(), *v)).collect())
        .unwrap_or_default()
    }

    pub fn getSti(&self, styleId: &str) -> Option<i32> {
      // java IOAssert.assertNotNull("styleId should not be null", styleId);
      let style_opt = Some(styleId);
      IOAssert::assertNotNull_msg("styleId should not be null", &style_opt);
      // java IOAssert.assertNotNull("mMapStyleId2Sti should not be null", this.mMapStyleId2Sti);
      IOAssert::assertNotNull_msg("mMapStyleId2Sti should not be null", &self.mMapStyleId2Sti);
      // java return (Integer)this.mMapStyleId2Sti.get(styleId);
      return self
        .mMapStyleId2Sti
        .as_ref()
        .and_then(|m| m.get(styleId).copied());
    }

    // java public int nextUserSti()
    pub fn nextUserSti(&mut self) -> i32 {
      // java return this.mStiMaker.getUUSti();
      return self.mStiMaker.getUUSti();
    }

    // java public int put(String styleId, String name, int styleType, boolean isCustom)
    pub fn put__string_string_int_boolean(&mut self, styleId: &str, name: &str, styleType: i32, isCustom: bool) -> i32 {
      // java BuildinStyleSheet.TypeSti typeSti = BuildinStyleSheet.getTypeSti(name);
      let typeSti: Option<TypeSti> = BuildinStyleSheet::getTypeSti(name);
      // java boolean isMakeUUSti = null == typeSti || styleType != typeSti.type || isCustom;
      let isMakeUUSti: bool = typeSti.is_none()
        || styleType != typeSti.as_ref().unwrap().r#type
        || isCustom;
      // java int coreId = isMakeUUSti ? this.mStiMaker.getUUSti() : typeSti.sti;
      let coreId: i32 = if isMakeUUSti {
        self.mStiMaker.getUUSti()
      } else {
        typeSti.unwrap().sti
      };
      // java Integer previous = (Integer)this.mMapStyleId2Sti.put(styleId, coreId);
      let _previous: Option<i32> = self
        .mMapStyleId2Sti
        .as_mut()
        .and_then(|m| m.insert(styleId.to_string(), coreId));
      // java return coreId;
      return coreId;
    }

    // java public void put(String styleId, Integer sti)
    pub fn put__string_integer(&mut self, styleId: &str, sti: Option<i32>) {
      // java Integer previous = (Integer)this.mMapStyleId2Sti.put(styleId, sti);
      let _previous: Option<i32> = match (&mut self.mMapStyleId2Sti, sti) {
        (Some(m), Some(v)) => m.insert(styleId.to_string(), v),
        (Some(m), None) => {
          // Map.put(key, null): HashMap<String,i32> cannot store null — drop entry
          m.remove(styleId)
        }
        _ => None,
      };
    }

    // java public int put(String styleId)
    pub fn put__string(&mut self, styleId: &str) -> i32 {
      // java IOAssert.assertNotNull("styleId should not be null", styleId);
      let style_opt = Some(styleId);
      IOAssert::assertNotNull_msg("styleId should not be null", &style_opt);
      // java Integer coreId = this.mStiMaker.getUUSti();
      let coreId: i32 = self.mStiMaker.getUUSti();
      // java this.mMapStyleId2Sti.put(styleId, coreId);
      if let Some(m) = self.mMapStyleId2Sti.as_mut() {
        m.insert(styleId.to_string(), coreId);
      }
      // java return coreId;
      return coreId;
    }

}

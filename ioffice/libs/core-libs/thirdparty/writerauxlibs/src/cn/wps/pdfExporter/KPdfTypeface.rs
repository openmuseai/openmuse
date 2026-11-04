#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// Java: import cn.wps.font.FreeTypeJNI
// (thirdparty_font crate not available - using local stubs)
// Java: import cn.wps.font.IFontFamily
// (thirdparty_font crate not available - using local stubs)
// Java: import java.util.Collection
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
// Java: import java.util.Map
use std::sync::{Mutex, OnceLock};

pub trait IFontFamily: Send + Sync {
    fn getName(&self) -> String;
}

struct FreeTypeJNI;
impl FreeTypeJNI {
    fn newTypeface(_path: &str, _index: i32) -> i64 { 0 }
    fn deleteTypeface(_ptr: i64) {}
}

struct KPdfFont;
impl KPdfFont {
    fn getFontIndex(_path: &str, _family: &dyn IFontFamily) -> i32 { 0 }
}

// Java field: Map<String, Long> mTypefaceMap
static M_TYPEFACE_MAP: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();

fn get_typeface_map() -> &'static Mutex<HashMap<String, i64>> {
    M_TYPEFACE_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class KPdfTypeface
pub struct KPdfTypeface {
    // Java field: IFontFamily mFamily
    mFamily: Box<dyn IFontFamily>,
    // Java field: String mFamilyName
    pub(crate) mFamilyName: String,
    // Java field: String mFontPath
    pub(crate) mFontPath: String,
    // Java field: Map<String, Long> mTypefaceMap
    // (static field - see M_TYPEFACE_MAP above)
}

// Java: public class KPdfTypeface
#[allow(non_snake_case)]
impl KPdfTypeface {
    // Java: KPdfTypeface(IFontFamily family, String fontPath)
    pub fn new(family: Box<dyn IFontFamily>, fontPath: String) -> Self {
      // Java: super();
      // Java: this.mFamily = family;
      // Java: this.mFamilyName = family.getName();
      let familyName = family.getName();
      // Java: this.mFontPath = fontPath;
      Self {
          mFamily: family,
          mFamilyName: familyName,
          mFontPath: fontPath,
      }
    }

    // Java: boolean isSameTypeface(IFontFamily family, String fontPath)
    pub fn isSameTypeface(&self, family: &dyn IFontFamily, fontPath: &str) -> bool {
      // Java: return this.mFontPath.equals(fontPath) && KPdfFont.getFontIndex(fontPath, family) == KPdfFont.getFontIndex(fontPath, this.mFamily);
      self.mFontPath == fontPath
          && KPdfFont::getFontIndex(fontPath, family)
              == KPdfFont::getFontIndex(fontPath, self.mFamily.as_ref())
    }

    // Java: String getFamilyName()
    pub fn getFamilyName(&self) -> &str {
      // Java: return this.mFamilyName;
      &self.mFamilyName
    }

    // Java: static long getTypeface(String familyPath, int index)
    pub fn getTypeface(familyPath: &str, index: i32) -> i64 {
      // Java: String path = index > 0 ? familyPath + index : familyPath;
      let path = if index > 0 {
          format!("{}{}", familyPath, index)
      } else {
          familyPath.to_string()
      };
      // Java: Long typeface = (Long)mTypefaceMap.get(path);
      let mut map = get_typeface_map().lock().unwrap();
      // Java: if (typeface == null)
      if let Some(&typeface) = map.get(&path) {
          // Java: return typeface;
          return typeface;
      }
      {
         // Java: typeface = FreeTypeJNI.newTypeface(familyPath, index);
         let typeface = FreeTypeJNI::newTypeface(familyPath, index);
         // Java: mTypefaceMap.put(path, typeface);
         map.insert(path, typeface);
         // Java: return typeface;
         typeface
      }
    }

    // Java: static void clearTypeface()
    pub fn clearTypeface() {
      // Java: Collection<Long> vals = mTypefaceMap.values();
      let mut map = get_typeface_map().lock().unwrap();
      // Java: Iterator<Long> iter = vals.iterator();
      // Java: while(iter.hasNext())
      for &typeface in map.values() {
         // Java: FreeTypeJNI.deleteTypeface((Long)iter.next());
         FreeTypeJNI::deleteTypeface(typeface);
      }
      // Java: mTypefaceMap.clear();
      map.clear();
    }

}

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::*;
// rust: ★M-STAGE1 P2 —— 补显式导入（原 auto-generated 桩缺此依赖）
use crate::cn::wps::moffice::writer::core::BuiltinFontTable::{BuiltinFontTable, font};
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.FontTableImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::FontTableImporter::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import org.apache.poi.hwpf.model.Ffn
use io_poi_compact::org::apache::poi::hwpf::model::Ffn::*;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

// java private static HashMap<String, Integer> sName2Index = new HashMap();
static sName2Index: std::sync::LazyLock<Option<HashMap<String, i32>>> =
                    std::sync::LazyLock::new(|| Some(HashMap::new()));

// java public final class FontTable implements Cloneable
#[derive(Clone)]
pub struct FontTable {
    // java private HashMap<String, FontInfo> mFontTable = new HashMap();
    mFontTable: Option<HashMap<String, FontInfo>>,
    // java private ArrayList<FontInfo> mFonts = new ArrayList();
    mFonts: Option<Vec<FontInfo>>,
}

impl Default for FontTable {
    fn default() -> Self {
        // java private HashMap<String, FontInfo> mFontTable = new HashMap();
        // java private ArrayList<FontInfo> mFonts = new ArrayList();
        Self { mFontTable: Some(HashMap::new()), mFonts: Some(Vec::new()) }
    }
}

// java public final class FontTable implements Cloneable
impl FontTable {
    /// Nullable lookup adapter for Java `get(String)`: the existing Rust
    /// `get__string` unwraps absent entries, while Java returns `null`.
    pub fn get__string_opt(&self, name: &str) -> Option<FontInfo> {
      // java return (FontInfo)this.mFontTable.get(name);
      self.mFontTable.as_ref().and_then(|table| table.get(name)).cloned()
    }

    // java public FontInfo get(String name)
    pub fn get__string(&self, name: &str) -> FontInfo {
      // java Assert.assertNotNull("mFontTable should not be null!", this.mFontTable);
      Assert::assertNotNull_msg("mFontTable should not be null!", &self.mFontTable);
      // java return (FontInfo)this.mFontTable.get(name);
      return self.mFontTable.as_ref().and_then(|m| m.get(name)).cloned().unwrap();
    }

    // java public void add(FontTable fontTable)
    pub fn add__fonttable(&mut self, fontTable: &FontTable) {
      // java Assert.assertNotNull("fontTable should not be null!", fontTable);
      // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
      let _ = fontTable;
      // java int size = fontTable.mFonts.size();
      let mut size: i32 = (fontTable.mFonts.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java FontInfo info = (FontInfo)fontTable.mFonts.as_ref().and_then(|v| v.get(i as usize));
        let mut info: FontInfo = (fontTable.mFonts.as_ref().and_then(|v| v.get(i as usize))).cloned().unwrap();
        // java Assert.assertNotNull("info should not be null!", info);
        // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
        let _ = info;
        // java String name = info.getFontName();
        let mut name: String = info.getFontName();
        // java Assert.assertNotNull("name should not be null!", name);
        // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
        let _ = name;
        // java if (!this.mFontTable.as_ref().map_or(false, |m| m.contains_key(name)))
        if !self.mFontTable.as_ref().map_or(false, |m| m.contains_key(&name)) {
          // java this.add(info);
          self.mFontTable.get_or_insert_with(HashMap::new).insert(info.getFontName(), info.clone());
          self.mFonts.get_or_insert_with(Vec::new).push(info.clone());
        }
      }
    }

    // java public void add(FontInfo info)
    pub fn add__fontinfo(&mut self, info: &FontInfo) {
      // java Assert.assertNotNull("info should not be null!", info);
      // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
      let _ = info;
      // java Assert.assertNotNull("mFontTable should not be null!", this.mFontTable);
      Assert::assertNotNull_msg("mFontTable should not be null!", &self.mFontTable);
      // java Assert.assertNotNull("mFonts should not be null!", this.mFonts);
      Assert::assertNotNull_msg("mFonts should not be null!", &self.mFonts);
      // java String name = info.getFontName();
      let mut name: String = info.getFontName();
      // java Assert.assertNotNull("name should not be null!", name);
      // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
      let _ = name;
      // java if (!this.mFontTable.as_ref().map_or(false, |m| m.contains_key(name)))
      if !self.mFontTable.as_ref().map_or(false, |m| m.contains_key(&name)) {
        // java this.mFontTable.put(name, info);
        self.mFontTable.get_or_insert_with(HashMap::new).insert(name.clone(), info.clone());
        // java this.mFonts.add(info);
        self.mFonts.get_or_insert_with(Vec::new).push(info.clone());
      }
    }

    // java public void add(String name)
    pub fn add__string(&mut self, name: &str) {
      // java Assert.assertNotNull("mFontTable should not be null!", this.mFontTable);
      Assert::assertNotNull_msg("mFontTable should not be null!", &self.mFontTable);
      // java if (!this.mFontTable.as_ref().map_or(false, |m| m.contains_key(name)))
      if !self.mFontTable.as_ref().map_or(false, |m| m.contains_key(name)) {
        // java FontInfo info = this.genFontInfo(name);
        let mut info: FontInfo = self.genFontInfo(name).unwrap();
        // java Assert.assertNotNull("info should not be null!", info);
        // rust: assertNotNull 对非 Option 引用不适用（真实 Assert 签名仅接受 &Option<T>）
        let _ = info;
        // java this.add(info);
        self.mFontTable.get_or_insert_with(HashMap::new).insert(info.getFontName(), info.clone());
          self.mFonts.get_or_insert_with(Vec::new).push(info.clone());
      }
    }

    // java private FontInfo genFontInfo(String name)
    pub fn genFontInfo(&mut self, name: &str) -> Option<FontInfo> {
      // java Integer index = (Integer)sName2Index.get(name);
      let mut index: i32 = sName2Index.as_ref().as_ref().and_then(|m| m.get(name)).copied().unwrap_or(0);
      // java index = null == index ? 0 : index;
      let _ = &index;
      // java Ffn ffn = new Ffn(BuiltinFontTable.font[index], 0);
      // rust: Java `new Ffn(byte[], int)` ↔ Rust `Ffn::new_from_bytes(&[u8], usize)`
      let mut ffn: Ffn = Ffn::new_from_bytes(&font[(index) as usize].iter().map(|b| *b as u8).collect::<Vec<u8>>(), 0);
      // java ffn.setAltFontName(&ffn.getMainFontName());
      ffn.setAltFontName(&ffn.getMainFontName());
      // java ffn.setMainFontName(name);
      ffn.setMainFontName(name);
      // java return FontTableImporter.toFontInfo(ffn);
      return Some(FontTableImporter::toFontInfo(&ffn));
    }

    // java public FontInfo get(int index)
    pub fn get__int(&self, index: i32) -> FontInfo {
      // java int size = this.size();
      let mut size: i32 = self.size();
      // java Assert.assertNotNull("mFonts should not be null!", this.mFonts);
      Assert::assertNotNull_msg("mFonts should not be null!", &self.mFonts);
      // java Assert.assertTrue("0 <= index && index < size should be true!", 0 <= index && index < size);
      Assert::assertTrue_msg("0 <= index && index < size should be true!", 0 <= index && index < size);
      // java return (FontInfo)this.mFonts.get(index);
      return self.mFonts.as_ref().and_then(|v| v.get(index as usize)).cloned().unwrap();
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java Assert.assertNotNull("mFonts should not be null!", this.mFonts);
      Assert::assertNotNull_msg("mFonts should not be null!", &self.mFonts);
      // java return this.mFonts.size();
      return (self.mFonts.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public Object clone()
    pub fn clone(&self) -> FontTable {
      // java FontTable ret = new FontTable();
      let mut ret: FontTable = FontTable::default();
      // java ret.mFonts = (ArrayList)this.mFonts.clone();
      ret.mFonts = self.mFonts.clone();
      // java ret.mFontTable = (HashMap)this.mFontTable.clone();
      ret.mFontTable = self.mFontTable.clone();
      // java return ret;
      return ret;
    }

}

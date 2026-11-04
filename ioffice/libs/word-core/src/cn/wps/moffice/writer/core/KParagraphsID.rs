// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.Set
// Java: import java.util.UUID
use stubs::java::util::UUID::*;
use std::collections::{HashMap, HashSet};

// rust: ★M-STAGE2 P1-D1 —— Java `String.hashCode()` 的真实语义：
//   `s[0]*31^(n-1) + s[1]*31^(n-2) + ... + s[n-1]`，以 `int`（i32）回绕，
//   逐 UTF-16 code unit 计算（与 Java 一致，非 UTF-8 字节）。
fn java_string_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// java public class KParagraphsID
//
// rust: Java 的字段初始化器 `= new HashSet()` / `= new HashMap()` 恒为非 null，
//   故此处不用 `Option` 包装（`Option` 是模板引擎产物，会把
//   `this.mParagraphIds.add(..)` 变成对 `Option` 的非法调用）。
pub struct KParagraphsID {
    // java Set<String> mParagraphIds = new HashSet();
    pub(crate) mParagraphIds: HashSet<String>,
    // java HashMap<Integer, ParaEntry> mId2Entry = new HashMap();
    pub(crate) mId2Entry: HashMap<i32, ParaEntry>,
    // java boolean mCorrectedParaId4IO = false;
    pub(crate) mCorrectedParaId4IO: bool,
}

// rust: 对应 Java 的字段初始化器（`= new HashSet()` / `= new HashMap()` / `= false`）。
impl Default for KParagraphsID {
    fn default() -> Self {
        Self {
            mParagraphIds: HashSet::new(),
            mId2Entry: HashMap::new(),
            mCorrectedParaId4IO: false,
        }
    }
}

// java public class ParaEntry
pub struct ParaEntry {
    // java int mSubDocumentType;
    pub(crate) mSubDocumentType: i32,
    // java IPapxTable.Entry mEntry;
    pub(crate) mEntry: Option<Box<dyn crate::cn::wps::moffice::writer::data::IPapxTable::Entry>>,
}

impl Default for ParaEntry {
    fn default() -> Self {
        Self {
            mSubDocumentType: 0,
            mEntry: None,
        }
    }
}

// java public class KParagraphsID
impl KParagraphsID {
    // java public boolean addParaphId(String paraId)
    pub fn addParaphId(&mut self, paraId: &str) -> bool {
      // java return this.mParagraphIds.add(paraId);
      return self.mParagraphIds.insert(paraId.to_string());
    }

    // java public String createId()
    pub fn createId(&mut self) -> String {
      // java String uuid;
      let mut uuid: String;
      // java for(uuid = UUID.randomUUID().toString().substring(0, 8).toUpperCase(); !this.addParaphId(uuid); uuid = UUID.randomUUID().toString().substring(0, 8).toUpperCase())
      uuid = UUID::randomUUID().toString().chars().take(8).collect::<String>().to_uppercase();
      while !self.addParaphId(&uuid) {
        // java uuid = UUID.randomUUID().toString().substring(0, 8).toUpperCase()
        uuid = UUID::randomUUID().toString().chars().take(8).collect::<String>().to_uppercase();
      }
      // java return uuid;
      return uuid;
    }

    // java public void putId(String id, int documentType, IPapxTable.Entry entry)
    // rust: Java 侧 `mEntry` 保存该引用；Rust 以 `Box<dyn Entry>` 表达独占持有
    //   （`IPapxTable.Entry` 为 trait object，无法按值拷贝）。
    pub fn putId(&mut self, id: Option<&str>, documentType: i32, entry: Box<dyn Entry>) {
      // java if (null != id)
      if let Some(id) = id {
        // java int hashId = id.hashCode();
        let hashId: i32 = java_string_hash_code(id);
        // java this.mId2Entry.put(hashId, new ParaEntry(documentType, entry));
        self.mId2Entry.insert(hashId, ParaEntry::ParaEntry(documentType, entry));
      }
    }

    // java public ParaEntry getParaEntry(String id)
    pub fn getParaEntry(&self, id: Option<&str>) -> Option<&ParaEntry> {
      // java if (null == id)
      if id.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let id = id.unwrap();
        // java int hashId = id.hashCode();
        let hashId: i32 = java_string_hash_code(id);
        // java ParaEntry paraEntry = (ParaEntry)this.mId2Entry.get(hashId);
        let paraEntry: Option<&ParaEntry> = self.mId2Entry.get(&hashId);
        // java if (null != paraEntry)
        if let Some(paraEntry) = paraEntry {
          // java PropertySet propertySet = paraEntry.mEntry.getProperty();
          // rust: `IPapxTable.Entry` 继承 `ITable.Entry`，两处同名 `getProperty`
          //   ⇒ 必须用 UFCS。Java 侧 `paraEntry.mEntry` 的静态类型是
          //   **`IPapxTable.Entry`**，故须选 `IPapxTable::Entry::getProperty`
          //   （其实现在 `PapxTable.rs` 的 `IPapxEntry for Entry`，返回真实属性）。
          //   ★若误选超 trait `ITable.Entry::getProperty`，会命中该 trait 在
          //   `PapxTable.rs:326` 的 `{ None }` 桩 ⇒ `getParaEntry` 恒返回 null
          //   （写进去却永远取不出来的假装配）。
          let propertySet: Option<&PropertySet> = paraEntry
            .mEntry
            .as_ref()
            .and_then(|e| crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getProperty(e.as_ref()));
          // java if (propertySet.contains(282) && id.equals(propertySet.getString(282)))
          if let Some(propertySet) = propertySet {
            if propertySet.contains(282) && id == propertySet.getString(282) {
              // java return paraEntry;
              return Some(paraEntry);
            }
          }
        }
        // java return null;
        return None;
      }
    }

    // java public boolean containsParaId(String paraId)
    pub fn containsParaId(&self, paraId: Option<&str>) -> bool {
      // java if (null == paraId)
      if paraId.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return null != this.mParagraphIds ? this.mParagraphIds.contains(paraId) : false;
        // rust: Java 侧 `mParagraphIds` 由字段初始化器恒非 null ⇒ 三元表达式恒取前支；
        //   此处保留该判据语义（Rust 字段非 Option，条件恒真）。
        return self.mParagraphIds.contains(paraId.unwrap());
      }
    }

    // java public void setCorrectedParaId4IO(boolean isCorrected)
    pub fn setCorrectedParaId4IO(&mut self, isCorrected: bool) {
      // java if (isCorrected != this.mCorrectedParaId4IO)
      if isCorrected != self.mCorrectedParaId4IO {
        // java this.mCorrectedParaId4IO = isCorrected;
        self.mCorrectedParaId4IO = isCorrected;
      }
    }

    // java public boolean isCorrectedParaId4IO()
    pub fn isCorrectedParaId4IO(&self) -> bool {
      // java return this.mCorrectedParaId4IO;
      return self.mCorrectedParaId4IO;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mParagraphIds.clear();
      self.mParagraphIds.clear();
      // java this.mId2Entry.clear();
      self.mId2Entry.clear();
    }

}

// java public class ParaEntry
impl ParaEntry {
    // java   ParaEntry(int documentType, IPapxTable.Entry entry)
    pub fn ParaEntry(documentType: i32, entry: Box<dyn Entry>) -> Self {
      // java this.mSubDocumentType = documentType;
      // java this.mEntry = entry;
      Self { mSubDocumentType: documentType, mEntry: Some(entry) }
    }

    // java public int getDocumentType()
    pub fn getDocumentType(&self) -> i32 {
      // java return this.mSubDocumentType;
      return self.mSubDocumentType;
    }

    // java public IPapxTable.Entry getPapxEntry()
    // rust: Java `return this.mEntry;` 返回的是**引用**（可为 null）⇒ Rust 对应
    //   `Option<&dyn Entry>`，而非转移所有权的 `Box`。
    pub fn getPapxEntry(&self) -> Option<&dyn Entry> {
      // java return this.mEntry;
      return self.mEntry.as_deref();
    }

}

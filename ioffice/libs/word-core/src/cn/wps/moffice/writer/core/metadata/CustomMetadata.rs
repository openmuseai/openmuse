// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantCustom
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantCustom::*;
// Java: import java.util.LinkedList
// Java: import java.util.List

// java public static int PID_START = 2;
// rust: Java `static int` 常量（本类内只读）→ `const i32`
//   （模板落成 `AtomicI32`，与常量/只读语义不符且引发 E0308）。
pub(crate) const PID_START: i32 = 2;

// java public class CustomMetadata
// rust: 派生 `Clone` —— `Metadata.getCustomMetadata()` 按 Java 语义返回引用，
//   Rust 门面按值返回 ⇒ 需 clone。
#[derive(Default, Clone)]
pub struct CustomMetadata {
    // java private List<VariantIndex> mCustomData = new LinkedList();
    mCustomData: Option<Vec<VariantIndex>>,
}

// java static class VariantIndex
// rust: 派生 `Clone` —— `createPid` / `getData` 需按下标取出元素
//   （模板残留 Java `ArrayList.get(i)` + `downcast_ref` 写法）。
#[derive(Default, Clone)]
pub struct VariantIndex {
    // java int mPid;
    pub(crate) mPid: i32,
    // java VariantCustom mCustom;
    pub(crate) mCustom: Option<VariantCustom>,
}

// java public class CustomMetadata
impl CustomMetadata {
    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mCustomData.clear();
      // rust: `Option<Vec<VariantIndex>>`（Java `List` 可空）⇒ 判空后清空。
      if let Some(d) = self.mCustomData.as_mut() { d.clear(); }
    }

    // java public boolean hasData()
    pub fn hasData(&self) -> bool {
      // java return this.mCustomData.size() > 0;
      return (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0;
    }

    // java public int createPid()
    pub fn createPid(&self) -> i32 {
      // java int size = this.mCustomData.size();
      let mut size: i32 = (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java return size > 0 ? ((VariantIndex)this.mCustomData.get(size - 1)).mPid + 1 : PID_START;
      return if size > 0 { self.mCustomData.as_ref().expect("mCustomData")[(size - 1) as usize].mPid + 1 } else { PID_START };
    }

    // java public VariantCustom getData(int pid)
    pub fn getData(&self, pid: i32) -> Option<VariantCustom> {
      // java int size = this.mCustomData.size();
      let mut size: i32 = (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java VariantIndex variant = (VariantIndex)this.mCustomData.get(i);
        let mut variant: VariantIndex = self.mCustomData.as_ref().expect("mCustomData")[i as usize].clone();
        // java if (variant.mPid == pid)
        if variant.mPid == pid {
          // java return variant.mCustom;
          return variant.mCustom;
        }
      }
      // java return null;
      return None;
    }

    // java public Integer getPidByName(String name)
    pub fn getPidByName(&self, name: &str) -> Option<i32> {
      // java int size = this.mCustomData.size();
      let mut size: i32 = (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java VariantIndex variant = (VariantIndex)this.mCustomData.get(i);
        let mut variant: VariantIndex = self.mCustomData.as_ref().expect("mCustomData")[i as usize].clone();
        // java if (this.isEquals(name, variant.mCustom.getName()))
        if self.isEquals(Some(name), Some(&variant.mCustom.as_ref().expect("mCustom").getName())) {
          // java return Some(variant.mPid);
          return Some(variant.mPid);
        }
      }
      // java return null;
      return None;
    }

    // java public void setData(int pid, VariantCustom var)
    pub fn setData(&mut self, pid: i32, var: Option<&VariantCustom>) {
      // java if (var != null)
      if let Some(var) = var {
        // java VariantIndex variantIndex = new VariantIndex(pid, var);
        let mut variantIndex: VariantIndex = VariantIndex::VariantIndex(pid, var);
        // java int size = this.mCustomData.size();
        let mut size: i32 = (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java VariantIndex variant = (VariantIndex)this.mCustomData.get(i);
          let mut variant: VariantIndex = self.mCustomData.as_ref().expect("mCustomData")[i as usize].clone();
          // java if (this.isEquals(var.getName(), variant.mCustom.getName()))
          if self.isEquals(Some(&var.getName()), Some(&variant.mCustom.as_ref().expect("mCustom").getName())) {
            // java return;
            return;
          }
          // java if (variant.mPid >= pid)
          if variant.mPid >= pid {
            // java this.mCustomData.add(i, variantIndex);
            self.mCustomData.as_mut().expect("mCustomData").insert(i as usize, variantIndex);
            // java return;
            return;
          }
        }
        // java this.mCustomData.add(size, variantIndex);
        self.mCustomData.as_mut().expect("mCustomData").push(variantIndex);
      }
    }

    // java public VariantCustom[] ascendingData()
    pub fn ascendingData(&self) -> Vec<VariantCustom> {
      // java int size = this.mCustomData.size();
      let mut size: i32 = (self.mCustomData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java VariantCustom[] variants = new VariantCustom[size];
      let mut variants: Vec<VariantCustom> = vec![Default::default(); (size) as usize];
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java VariantIndex variant = (VariantIndex)this.mCustomData.get(i);
        let mut variant: VariantIndex = self.mCustomData.as_ref().expect("mCustomData")[i as usize].clone();
        // java variants[i] = variant.mCustom;
        variants[(i) as usize] = variant.mCustom.clone().unwrap_or_default();
      }
      // java return variants;
      return variants;
    }

    // java public boolean isEquals(String name1, String name2)
    pub fn isEquals(&self, name1: Option<&str>, name2: Option<&str>) -> bool {
      // java if (null != name1)
      if let Some(name1) = name1 {
        // java return name1.equals(name2);
        return name2.map_or(false, |n2| name1 == n2);
      }
      // java else
      else {
        // java return null == name2;
        return name2.is_none();
      }
    }

}

// java static class VariantIndex
impl VariantIndex {
    // java   VariantIndex(int pid, VariantCustom custom)
    pub fn VariantIndex(pid: i32, custom: &VariantCustom) -> Self {
      // java this.mPid = pid;
      Self { mPid: pid, mCustom: Some(custom.clone()) }
      // java this.mCustom = custom;
    }

}

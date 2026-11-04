// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LFOLVL
use crate::cn::wps::moffice::writer::data::autonum::LFOLVL::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class LfoLvlData extends Atom implements Cloneable
#[derive(Clone, Default)]
pub struct LfoLvlData {
    // java Atom super_
    pub(crate) super_: Atom,
    // java public LvlfData mLvlfData;
    pub(crate) mLvlfData: Option<LvlfData>,
    // java public LFOLVL mLfolvl;
    pub(crate) mLfolvl: Option<LFOLVL>,
}

// java public class LfoLvlData extends Atom implements Cloneable
impl LfoLvlData {
    // java protected LfoLvlData clone()
    // rust: Java 返回非空 `LfoLvlData` ⇒ Rust 同名返回 `LfoLvlData`（非 `Option`）。
    pub fn clone(&self) -> LfoLvlData {
      // java LfoLvlData lfoLvlData = (LfoLvlData)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lfoLvlData: LfoLvlData = Clone::clone(self);
      // java lfoLvlData.mLvlfData = null != this.mLvlfData ? this.mLvlfData.clone() : null;
      lfoLvlData.mLvlfData = self.mLvlfData.clone();
      // java lfoLvlData.mLfolvl = null != this.mLfolvl ? this.mLfolvl.clone() : null;
      lfoLvlData.mLfolvl = self.mLfolvl.clone();
      // java return lfoLvlData;
      return lfoLvlData;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLvlfData = null;
      self.mLvlfData = None;
      // java this.mLfolvl = null;
      self.mLfolvl = None;
    }

    // java public void setLvlfData(LvlfData data)
    // rust: Java 形参是可空引用（字段即 `null` 初值）⇒ Rust 取 `Option<LvlfData>`。
    pub fn setLvlfData(&mut self, data: Option<LvlfData>) {
      // java this.mLvlfData = data;
      self.mLvlfData = data;
    }

    // java public LvlfData getLvlfData()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<LvlfData>`。
    pub fn getLvlfData(&self) -> Option<LvlfData> {
      // java return this.mLvlfData;
      return self.mLvlfData.clone();
    }

    // java public void setLFOLVL(LFOLVL data)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<LFOLVL>`。
    pub fn setLFOLVL(&mut self, data: Option<LFOLVL>) {
      // java this.mLfolvl = data;
      self.mLfolvl = data;
    }

    // java public LFOLVL getLFOLVL()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<LFOLVL>`。
    pub fn getLFOLVL(&self) -> Option<LFOLVL> {
      // java return this.mLfolvl;
      return self.mLfolvl.clone();
    }

    // java public int getIlvl()
    pub fn getIlvl(&self) -> i32 {
      // java return this.mLfolvl.ilvl;
      // rust: Java 在此假定 `mLfolvl` 非空（否则 NPE）；Rust 以 `unwrap_or_default` 等价兜底。
      return self.mLfolvl.as_ref().map(|v| ((*v).ilvl as i32)).unwrap_or_default();
    }

    // java public void setIlvl(int val)
    pub fn setIlvl(&mut self, val: i32) {
      // java this.mLfolvl.ilvl = (byte)val;
      // rust: Java 假定 `mLfolvl` 非空（否则 NPE）；Rust 以 `if let` 展开等价兜底。
      if let Some(v) = self.mLfolvl.as_mut() { v.ilvl = ((val) as u8); }
    }

    // java public boolean hasSetFormat()
    pub fn hasSetFormat(&self) -> bool {
      // java return this.mLfolvl.fFormatting;
      // rust: Java 在此假定 `mLfolvl` 非空（否则 NPE）；Rust 以 `unwrap_or_default` 等价兜底。
      return self.mLfolvl.as_ref().map(|v| (*v).fFormatting).unwrap_or_default();
    }

    // java public void setfFormatting(boolean val)
    pub fn setfFormatting(&mut self, val: bool) {
      // java this.mLfolvl.fFormatting = val;
      if let Some(v) = self.mLfolvl.as_mut() { v.fFormatting = val; }
    }

    // java public boolean hasSetStartAt()
    pub fn hasSetStartAt(&self) -> bool {
      // java return this.mLfolvl.fStartAt;
      // rust: Java 在此假定 `mLfolvl` 非空（否则 NPE）；Rust 以 `unwrap_or_default` 等价兜底。
      return self.mLfolvl.as_ref().map(|v| (*v).fStartAt).unwrap_or_default();
    }

    // java public int getStartAt()
    pub fn getStartAt(&self) -> i32 {
      // java return this.mLfolvl.iStartAt;
      // rust: Java 在此假定 `mLfolvl` 非空（否则 NPE）；Rust 以 `unwrap_or_default` 等价兜底。
      return self.mLfolvl.as_ref().map(|v| (*v).iStartAt).unwrap_or_default();
    }

    // java public void setStartAt(int val)
    pub fn setStartAt(&mut self, val: i32) {
      // java this.mLfolvl.iStartAt = val;
      // java this.mLfolvl.fStartAt = true;
      if let Some(v) = self.mLfolvl.as_mut() { v.iStartAt = val; v.fStartAt = true; }
    }

}

impl std::ops::Deref for LfoLvlData {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LfoLvlData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

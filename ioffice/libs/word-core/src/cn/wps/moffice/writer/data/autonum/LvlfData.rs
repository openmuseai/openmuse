// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
// rust: ★M-STAGE2 P1-D1 / T6 —— 与 `KStyles.rs` 一致，使用 `KStyle` 的软 `TextDocument`
//   （真实 `core/TextDocument.rs` 未实现 `Clone`，Java 侧为共享引用）。该桩列入待消除清单。
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LVLF
use crate::cn::wps::moffice::writer::data::autonum::LVLF::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java private static final int XST_STR_MAX_LENGTH = 31;
const XST_STR_MAX_LENGTH: i32 = 31;

// java public class LvlfData extends Atom implements Cloneable
#[derive(Clone, Default)]
pub struct LvlfData {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<TextDocument>,
    // java @AtomMember(1) public LVLF mLvlf;
    pub(crate) mLvlf: Option<LVLF>,
    // java @AtomMember public PropertySet mPapx;
    pub(crate) mPapx: Option<PropertySet>,
    // java @AtomMember public PropertySet mChpx;
    pub(crate) mChpx: Option<PropertySet>,
    // java @AtomMember public String mXstStr;
    pub(crate) mXstStr: Option<String>,
}

// java public class LvlfData extends Atom implements Cloneable
impl LvlfData {
    // java public  LvlfData(TextDocument textDocument)
    pub fn LvlfData(textDocument: &TextDocument) -> Self {
      // java this.mPapx = PropertySet.empty;
      Self { super_: Atom::new(), mTextDocument: Some(textDocument.clone()), mLvlf: None, mPapx: Some(PropertySet::empty()), mChpx: Some(PropertySet::empty()), mXstStr: Default::default() }
      // java this.mChpx = PropertySet.empty;
      // java this.mTextDocument = textDocument;
    }

    // java public LvlfData clone()
    pub fn clone(&self) -> LvlfData {
      // java LvlfData lvlfData = (LvlfData)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lvlfData: LvlfData = Clone::clone(self);
      // java lvlfData.mLvlf = null != this.mLvlf ? this.mLvlf.clone() : null;
      lvlfData.mLvlf = self.mLvlf.clone();
      // java lvlfData.mPapx = null != this.mPapx ? this.mPapx.clone() : null;
      lvlfData.mPapx = self.mPapx.clone();
      // java lvlfData.mChpx = null != this.mChpx ? this.mChpx.clone() : null;
      lvlfData.mChpx = self.mChpx.clone();
      // java return lvlfData;
      return lvlfData;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLvlf = null;
      self.mLvlf = None;
      // java this.mPapx = null;
      self.mPapx = None;
      // java this.mChpx = null;
      self.mChpx = None;
      // java this.mXstStr = null;
      self.mXstStr = None;
    }

    // java public void setLVLF(LVLF lvlf)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<LVLF>`。
    pub fn setLVLF(&mut self, lvlf: Option<LVLF>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf = lvlf;
      self.mLvlf = lvlf;
    }

    // java public LVLF getLVLF()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<LVLF>`。
    pub fn getLVLF(&self) -> Option<LVLF> {
      // java return this.mLvlf;
      return self.mLvlf.clone();
    }

    // java public PropertySet getParaProp()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<PropertySet>`。
    pub fn getParaProp(&self) -> Option<PropertySet> {
      // java return this.mPapx;
      return self.mPapx.clone();
    }

    // java public void setParaProp(PropertySet set)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<PropertySet>`。
    pub fn setParaProp(&mut self, set: Option<PropertySet>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mPapx = set;
      self.mPapx = set;
    }

    // java public PropertySet getRunProp()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<PropertySet>`。
    pub fn getRunProp(&self) -> Option<PropertySet> {
      // java return this.mChpx;
      return self.mChpx.clone();
    }

    // java public void setRunProp(PropertySet set)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<PropertySet>`。
    pub fn setRunProp(&mut self, set: Option<PropertySet>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mChpx = set;
      self.mChpx = set;
    }

    // java public void setXstStr(String str)
    // rust: Java 形参 `String` 可空 ⇒ Rust 取 `Option<String>`。
    pub fn setXstStr(&mut self, mut str: Option<String>) {
      // java if (null != str && str.length() > 31)
      if let Some(s) = str.as_mut() {
        if (s.chars().count() as i32) > 31 {
          // java str = str.substring(0, 31);
          // rust: Java `String.substring(0, 31)` 无 Rust 对应方法 ⇒ 按字符取前 31 个。
          *s = s.chars().take(31).collect();
        }
      }
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mXstStr = str;
      self.mXstStr = str;
    }

    // java public String getXstStr()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<String>`。
    pub fn getXstStr(&self) -> Option<String> {
      // java return this.mXstStr;
      return self.mXstStr.clone();
    }

    // java public int getStartAt()
    pub fn getStartAt(&self) -> i32 {
      // java return this.mLvlf.iStartAt;
      return self.mLvlf.as_ref().map(|v| v.iStartAt).unwrap_or_default();
    }

    // java public void setStartAt(int val)
    pub fn setStartAt(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.iStartAt = val;
      if let Some(v) = self.mLvlf.as_mut() { v.iStartAt = val; }
    }

    // java public int getNfc()
    pub fn getNfc(&self) -> i32 {
      // java return this.mLvlf.nfc;
      return self.mLvlf.as_ref().map(|v| (v.nfc as i32)).unwrap_or_default();
    }

    // java public void setNfc(int val)
    pub fn setNfc(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.nfc = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() { v.nfc = ((val) as u8); }
    }

    // java public short getFollow()
    pub fn getFollow(&self) -> i16 {
      // java return (short)this.mLvlf.ixchFollow;
      return self.mLvlf.as_ref().map(|v| (v.ixchFollow as i16)).unwrap_or_default();
    }

    // java public void setFollow(int val)
    pub fn setFollow(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.ixchFollow = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() { v.ixchFollow = ((val) as u8); }
    }

    // java public short getJc()
    pub fn getJc(&self) -> i16 {
      // java return (short)this.mLvlf.jc;
      return self.mLvlf.as_ref().map(|v| (v.jc as i16)).unwrap_or_default();
    }

    // java public void setJc(int val)
    pub fn setJc(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.jc = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() { v.jc = ((val) as u8); }
    }

    // java public boolean isLegal()
    pub fn isLegal(&self) -> bool {
      // java return this.mLvlf.fLegal;
      return self.mLvlf.as_ref().map(|v| v.fLegal).unwrap_or_default();
    }

    // java public void setfLegal(boolean val)
    pub fn setfLegal(&mut self, val: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.fLegal = val;
      if let Some(v) = self.mLvlf.as_mut() { v.fLegal = val; }
    }

    // java public boolean isNoRestart()
    pub fn isNoRestart(&self) -> bool {
      // java return this.mLvlf.fNoRestart;
      return self.mLvlf.as_ref().map(|v| v.fNoRestart).unwrap_or_default();
    }

    // java public void setfNoRestart(boolean val)
    pub fn setfNoRestart(&mut self, val: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.fNoRestart = val;
      if let Some(v) = self.mLvlf.as_mut() { v.fNoRestart = val; }
    }

    // java public boolean isIndentSav()
    pub fn isIndentSav(&self) -> bool {
      // java return this.mLvlf.fIndentSav;
      return self.mLvlf.as_ref().map(|v| v.fIndentSav).unwrap_or_default();
    }

    // java public void setfIndentSav(boolean val)
    pub fn setfIndentSav(&mut self, val: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.fIndentSav = val;
      if let Some(v) = self.mLvlf.as_mut() { v.fIndentSav = val; }
    }

    // java public boolean isConverted()
    pub fn isConverted(&self) -> bool {
      // java return this.mLvlf.fConverted;
      return self.mLvlf.as_ref().map(|v| v.fConverted).unwrap_or_default();
    }

    // java public void setfConverted(boolean val)
    pub fn setfConverted(&mut self, val: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.fConverted = val;
      if let Some(v) = self.mLvlf.as_mut() { v.fConverted = val; }
    }

    // java public boolean isTentative()
    pub fn isTentative(&self) -> bool {
      // java return this.mLvlf.fTentative;
      return self.mLvlf.as_ref().map(|v| v.fTentative).unwrap_or_default();
    }

    // java public void setfTentative(boolean val)
    pub fn setfTentative(&mut self, val: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.fTentative = val;
      if let Some(v) = self.mLvlf.as_mut() { v.fTentative = val; }
    }

    // java public byte[] getrgbxchNums()
    pub fn getrgbxchNums(&self) -> Vec<u8> {
      // java return this.mLvlf.rgbxchNums;
      return self.mLvlf.as_ref().and_then(|v| v.rgbxchNums.clone()).unwrap_or_default();
    }

    // java public void setBxchNums(int index, int val)
    pub fn setBxchNums__int_int(&mut self, index: i32, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.rgbxchNums[index] = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() {
        if let Some(arr) = v.rgbxchNums.as_mut() { arr[(index) as usize] = ((val) as u8); }
      }
    }

    // java public void setBxchNums(byte[] rgbxchNums)
    pub fn setBxchNums__bytearr(&mut self, rgbxchNums: &Vec<u8>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.rgbxchNums = rgbxchNums;
      if let Some(v) = self.mLvlf.as_mut() { v.rgbxchNums = Some(rgbxchNums.clone()); }
    }

    // java public int getDxaIndentSav()
    pub fn getDxaIndentSav(&self) -> i32 {
      // java return this.mLvlf.dxaIndentSav;
      return self.mLvlf.as_ref().map(|v| v.dxaIndentSav).unwrap_or_default();
    }

    // java public void setDxaIndentSav(int val)
    pub fn setDxaIndentSav(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.dxaIndentSav = val;
      if let Some(v) = self.mLvlf.as_mut() { v.dxaIndentSav = val; }
    }

    // java public byte getIRestartAfterLevel()
    pub fn getIRestartAfterLevel(&self) -> u8 {
      // java return this.mLvlf.iRestartAfterLevel;
      return self.mLvlf.as_ref().map(|v| v.iRestartAfterLevel).unwrap_or_default();
    }

    // java public void setIRestartAfterLevel(int val)
    pub fn setIRestartAfterLevel(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.iRestartAfterLevel = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() { v.iRestartAfterLevel = ((val) as u8); }
    }

    // java public byte getGrfhic()
    pub fn getGrfhic(&self) -> u8 {
      // java return this.mLvlf.grfhic;
      return self.mLvlf.as_ref().map(|v| v.grfhic).unwrap_or_default();
    }

    // java public void setGrfhic(int val)
    pub fn setGrfhic(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlf.grfhic = (byte)val;
      if let Some(v) = self.mLvlf.as_mut() { v.grfhic = ((val) as u8); }
    }

    // java public void onSwapped(Object[] prev, Object arg)
    pub fn onSwapped(&mut self, prev: &dyn Object, arg: &dyn Object) {
      // java this.mTextDocument.updateDocument();
      // rust: Java 假定 `mTextDocument` 非空；Rust 以 `if let` 展开等价兜底。
      if let Some(d) = self.mTextDocument.as_mut() { d.updateDocument(); }
    }

}

impl std::ops::Deref for LvlfData {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LvlfData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

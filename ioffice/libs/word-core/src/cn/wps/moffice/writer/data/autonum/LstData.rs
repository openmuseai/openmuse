// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// rust: `Atom.setTransaction` 的 Rust 签名使用 `Option<Rc<RefCell<Transaction>>>`。
use std::cell::RefCell;
use std::rc::Rc;
// Java: import cn.wps.moffice.writer.core.TextDocument
// rust: ★M-STAGE2 P1-D1 / T6 —— 与 `KStyles.rs` 一致，使用 `KStyle` 的软 `TextDocument`
//   （真实 `core/TextDocument.rs` 未实现 `Clone`，Java 侧为共享引用）。该桩列入待消除清单。
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LSTF
use crate::cn::wps::moffice::writer::data::autonum::LSTF::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public static final int lvlCountMin = 1;
pub(crate) const lvlCountMin: i32 = 1;
// java public static final int lvlCountMax = 9;
pub(crate) const lvlCountMax: i32 = 9;

// java public class LstData extends Atom implements Cloneable
#[derive(Clone, Default)]
pub struct LstData {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<TextDocument>,
    // java @AtomMember public LSTF mLstf = null;
    pub(crate) mLstf: Option<LSTF>,
    // java @AtomMember private LvlfData[] mLvlfArray = null;
    mLvlfArray: Option<Vec<LvlfData>>,
}

// java public class LstData extends Atom implements Cloneable
impl LstData {
    // java public  LstData(TextDocument textDocument)
    pub fn LstData(textDocument: &TextDocument) -> Self {
      // java this.mTextDocument = textDocument;
      Self { super_: Atom::new(), mTextDocument: Some(textDocument.clone()), mLstf: None, mLvlfArray: Some(Vec::new()) }
    }

    // java public LstData clone()
    pub fn clone(&self) -> LstData {
      // java LstData lstData = (LstData)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lstData: LstData = Clone::clone(self);
      // java lstData.mLstf = null != this.mLstf ? this.mLstf.clone() : null;
      lstData.mLstf = self.mLstf.clone();
      // java lstData.mLvlfArray = null;
      lstData.mLvlfArray = None;
      // java if (null != this.mLvlfArray)
      if let Some(mLvlfArray) = self.mLvlfArray.as_ref() {
        // java lstData.mLvlfArray = new LvlfData[this.mLvlfArray.length];
        lstData.mLvlfArray = Some(vec![Default::default(); (mLvlfArray.len() as i32) as usize]);
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = (mLvlfArray.len() as i32);
        // java for(int len = this.mLvlfArray.length; i < len; ++i)
        while i < len {
          // java lstData.mLvlfArray[i] = null != this.mLvlfArray[i] ? this.mLvlfArray[i].clone() : null;
          if let (Some(dst), Some(src)) = (lstData.mLvlfArray.as_mut(), mLvlfArray.get((i) as usize)) {
            dst[(i) as usize] = src.clone();
          }
          i += 1;
        }
      }
      // java return lstData;
      return lstData;
    }

    // java public void setLETF(LSTF lstf)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<LSTF>`。
    pub fn setLETF(&mut self, lstf: Option<LSTF>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLstf = lstf;
      self.mLstf = lstf;
      // java if (this.mLstf != null)
      if let Some(mLstf) = self.mLstf.as_mut() {
        // java this.mLstf.setTransaction(this.getTransaction());
        mLstf.setTransaction(self.super_.getTransaction());
      }
    }

    // java public LSTF getLSTF()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<LSTF>`。
    pub fn getLSTF(&self) -> Option<LSTF> {
      // java return this.mLstf;
      return self.mLstf.clone();
    }

    // java public void setTransaction(Transaction transaction, boolean bRecursive)
    pub fn setTransaction(&mut self, transaction: Option<Rc<RefCell<Transaction>>>, bRecursive: bool) {
      // java super.setTransaction(transaction, bRecursive);
      self.super_.setTransaction_recursive(transaction, bRecursive);
      // java if (null != this.mLvlfArray)
      if let Some(mLvlfArray) = self.mLvlfArray.as_mut() {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = (mLvlfArray.len() as i32);
        // java for(int len = this.mLvlfArray.length; i < len; ++i)
        while i < len {
          // java this.mLvlfArray[i].setTransaction(this.mTransaction, false);
          if let Some(item) = mLvlfArray.get_mut((i) as usize) {
            item.setTransaction_recursive(self.super_.getTransaction(), false);
          }
          i += 1;
        }
      }
      // java this.mLstf.setTransaction(this.mTransaction);
      if let Some(v) = self.mLstf.as_mut() { v.setTransaction_recursive(self.super_.getTransaction(), false); }
    }

    // java public void setLvlfDataArray(LvlfData[] arr)
    pub fn setLvlfDataArray(&mut self, arr: &Vec<LvlfData>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLvlfArray = arr;
      self.mLvlfArray = Some(arr.clone());
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (self.mLvlfArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int len = this.mLvlfArray.length; i < len; ++i)
      while i < len {
        // java this.mLvlfArray[i].setTransaction(this.mTransaction, false);
        if let Some(dst) = self.mLvlfArray.as_mut() {
          if let Some(item) = dst.get_mut((i) as usize) {
            item.setTransaction_recursive(self.super_.getTransaction(), false);
          }
        }
        i += 1;
      }
    }

    // java public LvlfData[] getLvlfDataArray()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<Vec<LvlfData>>`。
    pub fn getLvlfDataArray(&self) -> Option<Vec<LvlfData>> {
      // java return this.mLvlfArray;
      return self.mLvlfArray.clone();
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLstf = null;
      self.mLstf = None;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (self.mLvlfArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int len = this.mLvlfArray.length; i < len; ++i)
      while i < len {
        // java LvlfData data = this.mLvlfArray[i];
        let data: Option<&mut LvlfData> = self.mLvlfArray.as_mut().and_then(|v| v.get_mut((i) as usize));
        // java if (data != null)
        if let Some(data) = data {
          // java data.dispose();
          data.dispose();
          // java this.mLvlfArray[i] = null;
          // rust: no-stmt —— `Vec<LvlfData>` 元素非可空，无「置 null」对应语句。
        }
        i += 1;
      }
      // java this.mLvlfArray = null;
      self.mLvlfArray = None;
    }

    // java public int getlsid()
    pub fn getlsid(&self) -> i32 {
      // java return this.mLstf.getLsid();
      return self.mLstf.as_ref().map(|v| v.getLsid()).unwrap_or_default();
    }

    // java public int getLevelCount()
    pub fn getLevelCount(&self) -> i32 {
      // java return this.mLvlfArray.length;
      return (self.mLvlfArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public LvlfData getLvlfDataByIlvl(int level)
    pub fn getLvlfDataByIlvl(&self, level: i32) -> Option<LvlfData> {
      // java if (level == 12)
      if level == 12 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Assert.assertTrue("requested level is out of range: " + level, level >= 0 && level <= 8);
        Assert::assertTrue__string_boolean(&format!("requested level is out of range: {}", level), level >= 0 && level <= 8);
        // java return level < this.mLvlfArray.length && level > 0 ? this.mLvlfArray[level] : this.mLvlfArray[0];
        return if level < (self.mLvlfArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) && level > 0 {
          self.mLvlfArray.as_ref().and_then(|v| v.get((level) as usize)).cloned()
        } else {
          self.mLvlfArray.as_ref().and_then(|v| v.first()).cloned()
        };
      }
    }

    // java public int getTemplateCode()
    pub fn getTemplateCode(&self) -> i32 {
      // java return this.mLstf.getTplc();
      return self.mLstf.as_ref().map(|v| v.getTplc()).unwrap_or_default();
    }

    // java public void setTemplateCode(int val)
    pub fn setTemplateCode(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLstf.setTplc(val);
      if let Some(v) = self.mLstf.as_mut() { v.setTplc(val); }
    }

    // java public int getLevelTplc(int level)
    pub fn getLevelTplc(&self, level: i32) -> i32 {
      // java return this.mLstf.getRgtplc(level);
      return self.mLstf.as_ref().map(|v| v.getRgtplc__int(level)).unwrap_or_default();
    }

    // java public void setLevelTplc(int level, int val)
    pub fn setLevelTplc(&mut self, level: i32, val: i32) {
      // java Assert.assertTrue("level should be in [0, 9)", level >= 0 && level < 9);
      Assert::assertTrue__string_boolean(&"level should be in [0, 9)", level >= 0 && level < 9);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java if (null == this.mLstf.getRgtplc() || this.mLstf.getRgtplc().length <= 0)
      let rgtplc_empty = self.mLstf.as_ref().map(|v| (v.getRgtplc().len() as i32) <= 0).unwrap_or(true);
      if rgtplc_empty {
        // java this.mLstf.initRgTplc();
        if let Some(v) = self.mLstf.as_mut() { v.initRgTplc(); }
      }
      // java this.mLstf.setRgtplc(level, val);
      if let Some(v) = self.mLstf.as_mut() { v.setRgtplc__int_int(level, val); }
    }

    // java public void onSwapped(Object[] prev, Object arg)
    pub fn onSwapped(&mut self, prev: &dyn Object, arg: &dyn Object) {
      // java this.mTextDocument.updateDocument();
      if let Some(d) = self.mTextDocument.as_mut() { d.updateDocument(); }
    }

}

impl std::ops::Deref for LstData {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LstData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

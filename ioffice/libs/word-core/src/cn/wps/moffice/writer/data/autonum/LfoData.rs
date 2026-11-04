// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
// Java: import cn.wps.moffice.writer.core.TextDocument
// rust: ★M-STAGE2 P1-D1 / T6 —— 与 `KStyles.rs` 一致，使用 `KStyle` 的软 `TextDocument`。
//   注意：`KStyle.rs:48` 的 "Soft stand-in for core.TextDocument" 是**手写桩**
//   （仅 `transaction`/`textPool`/`styles` 三字段 + `#[derive(Clone)]`），
//   真实 `core/TextDocument.rs` 未实现 `Clone`（Java 侧 `mTextDocument` 为共享引用）。
//   ⇒ 该桩列入待消除清单；本批先与其保持一致以解阻 autonum 链。
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument;
// Java: import java.util.ArrayList
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LFO
use crate::cn::wps::moffice::writer::data::autonum::LFO::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoLvlData
use crate::cn::wps::moffice::writer::data::autonum::LfoLvlData::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public static final int lvlCountMin = 0;
pub(crate) const lvlCountMin: i32 = 0;
// java public static final int lvlCountMax = 9;
pub(crate) const lvlCountMax: i32 = 9;

// java public class LfoData extends Atom implements Cloneable
#[derive(Clone, Default)]
pub struct LfoData {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<TextDocument>,
    // java @AtomMember private LFO mLfo;
    mLfo: Option<LFO>,
    // java @AtomMember(1) public ArrayList<LfoLvlData> mLfoLvlArray;
    pub(crate) mLfoLvlArray: Option<Vec<LfoLvlData>>,
}

// java public class LfoData extends Atom implements Cloneable
impl LfoData {
    // java public  LfoData(TextDocument textDocument)
    pub fn LfoData(textDocument: &TextDocument) -> Self {
      // java this.mTextDocument = textDocument;
      // java this.mTextDocument = textDocument;
      // java this.mLfoLvlArray = new ArrayList();
      Self { super_: Atom::new(), mTextDocument: Some(textDocument.clone()), mLfo: None, mLfoLvlArray: Some(Vec::new()) }
    }

    // java public LfoData clone()
    // rust: Java 返回非空 `LfoData` ⇒ Rust 同名返回 `LfoData`（非 `Option`）。
    pub fn clone(&self) -> LfoData {
      // java LfoData lfoData = (LfoData)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lfoData: LfoData = Clone::clone(self);
      // java lfoData.mLfo = null != this.mLfo ? this.mLfo.clone() : null;
      lfoData.mLfo = self.mLfo.clone();
      // java lfoData.mLfoLvlArray = null;
      lfoData.mLfoLvlArray = None;
      // java if (null != this.mLfoLvlArray)
      if let Some(mLfoLvlArray) = self.mLfoLvlArray.as_ref() {
        // java lfoData.mLfoLvlArray = new ArrayList();
        lfoData.mLfoLvlArray = Some(Vec::new());
        // java for(LfoLvlData lfoLvlData : this.mLfoLvlArray)
        for lfoLvlData in mLfoLvlArray.iter() {
          // java lfoData.mLfoLvlArray.add(lfoLvlData.clone());
          if let Some(v) = lfoData.mLfoLvlArray.as_mut() { v.push(lfoLvlData.clone()); }
        }
      }
      // java return lfoData;
      return lfoData;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLfo = null;
      self.mLfo = None;
      // java this.mLfoLvlArray.clear();
      if let Some(v) = self.mLfoLvlArray.as_mut() { v.clear(); }
      // java this.mLfoLvlArray = null;
      self.mLfoLvlArray = None;
    }

    // java public void append(LfoLvlData data)
    pub fn append(&mut self, data: &LfoLvlData) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLfoLvlArray.add(data);
      if let Some(v) = self.mLfoLvlArray.as_mut() { v.push(data.clone()); }
    }

    // java public LFO getLFO()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<LFO>`。
    pub fn getLFO(&self) -> Option<LFO> {
      // java return this.mLfo;
      return self.mLfo.clone();
    }

    // java public void setLFO(LFO lfo)
    // rust: Java 形参是可空引用 ⇒ Rust 取 `Option<LFO>`。
    pub fn setLFO(&mut self, lfo: Option<LFO>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLfo = lfo;
      self.mLfo = lfo;
    }

    // java public int getLsid()
    pub fn getLsid(&self) -> i32 {
      // java return this.mLfo.lsid;
      // rust: Java 假定 `mLfo` 非空（否则 NPE）；Rust 以 `unwrap_or_default` 等价兜底。
      return self.mLfo.as_ref().map(|v| v.lsid).unwrap_or_default();
    }

    // java public void setLsid(int val)
    pub fn setLsid(&mut self, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLfo.lsid = val;
      if let Some(v) = self.mLfo.as_mut() { v.lsid = val; }
    }

    // java public void setLvlfDataArray(ArrayList<LfoLvlData> arr)
    pub fn setLvlfDataArray(&mut self, arr: &Vec<LfoLvlData>) {
      // java Assert.assertNotNull("arr should not be null.", arr);
      Assert::assertNotNull__string_object(&"arr should not be null.", &Some(arr));
      // java int n = arr.size();
      let mut n: i32 = (arr.len() as i32);
      // java Assert.assertTrue("arr.size() is out of expected length: " + n, n >= 0 && n <= 9);
      Assert::assertTrue__string_boolean(&format!("arr.size() is out of expected length: {}", n), n >= 0 && n <= 9);
      // java this.mLfoLvlArray = arr;
      self.mLfoLvlArray = Some(arr.clone());
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      // rust: 模板漏掉自增（`++i` 被吞）⇒ 补 `i += 1`，否则死循环。
      while i < n {
        // java ((LfoLvlData)this.mLfoLvlArray.get(i)).setTransaction(this.mTransaction, false);
        if let Some(v) = self.mLfoLvlArray.as_mut() {
          if let Some(item) = v.get_mut(i as usize) {
            item.setTransaction_recursive(self.super_.getTransaction(), false);
          }
        }
        i += 1;
      }
    }

    // java public LfoLvlData getLfoLvlByIlvl(int iLvl)
    pub fn getLfoLvlByIlvl(&self, iLvl: i32) -> Option<LfoLvlData> {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (self.mLfoLvlArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int len = this.mLfoLvlArray.size(); i < len; ++i)
      while i < len {
        // java LfoLvlData data = (LfoLvlData)this.mLfoLvlArray.get(i);
        let data: Option<LfoLvlData> = self.mLfoLvlArray.as_ref().and_then(|v| v.get(i as usize)).cloned();
        // java if (data != null && data.getIlvl() == iLvl)
        if data.is_some() && data.as_ref().unwrap().getIlvl() == iLvl {
          // java return data;
          return data;
        }
        i += 1;
      }
      // java return null;
      return None;
    }

    // java public LfoLvlData getLfoLvlByIndex(int index)
    pub fn getLfoLvlByIndex(&self, index: i32) -> Option<LfoLvlData> {
      // java return index >= 0 && index < this.mLfoLvlArray.size() ? (LfoLvlData)this.mLfoLvlArray.get(index) : null;
      return if index >= 0 && index < (self.mLfoLvlArray.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
        self.mLfoLvlArray.as_ref().and_then(|v| v.get(index as usize)).cloned()
      } else { None };
    }

    // java public ArrayList<LfoLvlData> getLfoLvlDatas()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<Vec<LfoLvlData>>`。
    pub fn getLfoLvlDatas(&self) -> Option<Vec<LfoLvlData>> {
      // java return this.mLfoLvlArray;
      return self.mLfoLvlArray.clone();
    }

    // java public void onSwapped(Object[] prev, Object arg)
    pub fn onSwapped(&mut self, prev: &dyn Object, arg: &dyn Object) {
      // java this.mTextDocument.updateDocument();
      // rust: Java 假定 `mTextDocument` 非空；Rust 以 `if let` 展开等价兜底。
      if let Some(d) = self.mTextDocument.as_mut() { d.updateDocument(); }
    }

}

impl std::ops::Deref for LfoData {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LfoData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

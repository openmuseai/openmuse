#![allow(non_snake_case)]
#![allow(dead_code)]

use super::KmoAtomHibernation::KmoAtomHibernation;

// Java: public abstract class KmoAtomData implements KmoAtomHibernation
// rust: Java 侧 `KmoAtom.clone()` 走 `super.clone()`（浅拷贝对象头）；Rust 无对象头，
//       基类只承载 `tip`，故以 `Clone` 表达同一语义。
#[derive(Clone)]
pub struct KmoAtomData {
    // Java field: private int tip = AtomStoreImpl.sharedStoreImpl().tip()
    pub(crate) tip: i32,
}

// Java: public abstract class KmoAtomData implements KmoAtomHibernation
impl KmoAtomData {
    // Java: KmoAtomData()
    pub fn new() -> Self {
      // Java: super();
      // Java: private int tip = AtomStoreImpl.sharedStoreImpl().tip();
      // rust: read the same singleton tip from its atomic mirror; needCopy
      // holds the store mutex while Java data.copy() constructs new data.
      let tip = super::KmoAtomStore::tip_without_relocking_store();
      KmoAtomData { tip }
    }

    // rust-only: 按指定 tip 构造。
    // Java 侧子类 `copy()` 里 `new XxxAtomData()` 走的是上面的构造器（tip 取自 AtomStore），
    // 本入口仅供 Rust 在 `copy()` 中保留源 tip（Java 由 AtomStore.needCopy 的语义保证）。
    pub(crate) fn new_with_tip(tip: i32) -> Self {
        KmoAtomData { tip }
    }

    // Java: public final int tip()
    pub fn tip(&self) -> i32 {
      // Java: return this.tip;
      self.tip
    }

    // Java: public void __forceOldest()
    pub fn __forceOldest(&mut self) {
      // Java: this.tip = 0;
      self.tip = 0;
    }

    // Java: public void hibernate()
    pub fn hibernate(&self) {
    }

    // Java: public void wakeup()
    pub fn wakeup(&self) {
    }

}

// Java: public abstract KmoAtomData copy()
//
// Java 的 `KmoAtomData` 是抽象类、`copy()` 是抽象方法；`KmoAtom<AtomRawDataType extends
// KmoAtomData>` 的泛型上界即「KmoAtomData 的某个具体子类」。Rust 用 trait 表达这层关系：
//   - `as_base()/as_base_mut()` 对应 Java 的 `extends KmoAtomData`（子类持有 base 字段）
//   - `copy()`                 对应 Java 的 `public abstract KmoAtomData copy()`
pub trait KmoAtomDataSub: Sized {
    // Java: extends KmoAtomData（基类部分）
    fn as_base(&self) -> &KmoAtomData;
    fn as_base_mut(&mut self) -> &mut KmoAtomData;

    // Java: public abstract KmoAtomData copy()
    fn copy(&self) -> Self;
}

impl KmoAtomHibernation for KmoAtomData {
    fn hibernate(&self) {
        self.hibernate();
    }
    fn wakeup(&self) {
        self.wakeup();
    }
}

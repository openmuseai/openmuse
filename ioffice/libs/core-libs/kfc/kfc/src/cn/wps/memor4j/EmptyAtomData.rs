#![allow(non_snake_case)]
#![allow(dead_code)]

use super::KmoAtomData::{KmoAtomData, KmoAtomDataSub};

// Java: class EmptyAtomData extends KmoAtomData
#[derive(Clone)]
pub struct EmptyAtomData {
    pub(crate) base: KmoAtomData,
}

// Java: class EmptyAtomData extends KmoAtomData
impl EmptyAtomData {
    // Java: public EmptyAtomData()
    pub fn new() -> Self {
      // Java: super();
      let mut base = KmoAtomData::new();
      // Java: this.__forceOldest();
      base.__forceOldest();
      EmptyAtomData { base }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> KmoAtomData {
      // Java: return this;
      // Java 返回 `this`（同一实例）；Rust 无对象头共享语义，按 tip 复制基类部分。
      KmoAtomData::new_with_tip(self.base.tip)
    }

}

// Java: extends KmoAtomData
impl KmoAtomDataSub for EmptyAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        self.clone()
    }
}

#![allow(non_snake_case)]

// Java: package cn.wps.show.app.shape
// Java: import cn.wps.memor4j.KmoAtomData
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;

// Java: class ShowShapeAtomData extends KmoAtomData
pub struct ShowShapeAtomData {
    // Java: extends KmoAtomData
    pub base: KmoAtomData,
    // Java field: protected ShowShape shape = ShowShape.create();
    //
    // rust: Java 侧 `shape` 是普通字段，子类方法以 `&this` 直接调 `shape.save()`；
    //       Rust `ShowShape::save()` 需要 `&mut self`，而 `KmoShowShape::protocol()` 等
    //       按 Java 签名是 `&self`，故以 `RefCell` 承载同一语义（台账登记为 rust: 偏差）。
    pub shape: std::cell::RefCell<ShowShape>,
    // Java field: protected KmoShowShape parentGroup
    //
    // rust: Java 的 `parentGroup` 是**后向引用**（父形状），Java 引用不占对象体积；
    //       Rust 若用 `Option<KmoShowShape>` 会与 `KmoShowShape -> KmoAtom<ShowShapeAtomData>`
    //       形成无限大小递归类型，故用 `Weak`（非拥有后向引用，语义等价）。
    //       装配策略见 M_PPT_KMOSLIDE_PLAN §1.5 环清单 / §5-R1（裁定：Weak）。
    pub parentGroup: Option<std::rc::Weak<std::cell::RefCell<super::KmoShowShape::KmoShowShape>>>,
}

// Java: class ShowShapeAtomData extends KmoAtomData
impl ShowShapeAtomData {
    // Java: ShowShapeAtomData()
    pub fn new() -> Self {
      // Java: super();
      let base = KmoAtomData::new();
      // Java: protected ShowShape shape = ShowShape.create();
      let shape = std::cell::RefCell::new(ShowShape::create());
      ShowShapeAtomData {
          base,
          shape,
          parentGroup: None,
      }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> ShowShapeAtomData {
      // Java: ShowShapeAtomData cpy = new ShowShapeAtomData();
      let mut cpy = ShowShapeAtomData::new();
      // Java: cpy.shape.load(this.shape.save());
      let bin = self.shape.borrow_mut().save();
      cpy.shape.borrow_mut().load(bin.as_ref());
      // Java: cpy.parentGroup = this.parentGroup;
      cpy.parentGroup = self.parentGroup.clone();
      // Java: return cpy;
      cpy
    }
}

// Java: extends KmoAtomData
impl KmoAtomDataSub for ShowShapeAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        ShowShapeAtomData::copy(self)
    }
}

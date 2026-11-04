#![allow(non_snake_case)]

// Java: package cn.wps.show.app.slide
// Java: import cn.wps.memor4j.KmoAtomData
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// Java: import cn.wps.show.data.slides.Slide
use crate::cn::wps::show::data::slides::Slide::Slide;
use super::KmoNoteSlide::KmoNoteSlide;
use super::KmoSlideLayout::KmoSlideLayout;

// Java: class SlideAtomData extends KmoAtomData
pub struct SlideAtomData {
    // Java: extends KmoAtomData
    pub base: KmoAtomData,
    // Java field: protected Slide slide = Slide.create();
    //
    // rust: Java 侧 `slide` 是普通字段，`copy()` 以 `&this` 调 `this.slide.save()`；
    //       Rust `Slide::save()` 需要 `&mut self`，而 `copy()` 按 Java 签名是 `&self`，
    //       故以 `RefCell` 承载同一语义（与 `ShowShapeAtomData.shape` 同策略，台账登记为 rust: 偏差）。
    pub slide: std::cell::RefCell<Slide>,
    // Java field: protected KmoSlideLayout layout
    //
    // rust: Java 的 `layout` 是**共享引用**（一个 layout 被多张 slide 共用），证据：
    //   · `SlideAtomData.copy()` 里是 `cpy.layout = this.layout;`（引用复制；同一方法里的
    //     `slide` 反而走 `load(this.slide.save())` 深拷贝，形成对照）；
    //   · `KmoSlide.setLayout/redirectLayout/changeLayout` 一律直接赋值 `((SlideAtomData)rawData()).layout = layout;`；
    //   · `KmoSlideMaster` 自持 `List<KmoSlideLayout> layouts`，`layoutAt(index)` 返回的实例
    //     即各 slide 共享的那一个（需要独立副本时走 `KmoSlideLayout.copyFrom`）。
    //   故 Rust 用正向共享句柄 `Rc<RefCell<KmoSlideLayout>>`（M_PPT_KMOSLIDE_PLAN §5-R1 裁定：
    //   正向持有 `Rc<RefCell<T>>`，反指 `Weak`，禁裸指针）；`Option` 对应 Java 的未初始化 null。
    pub layout: Option<std::rc::Rc<std::cell::RefCell<KmoSlideLayout>>>,
    // Java field: protected KmoNoteSlide noteSlide
    //
    // rust: 同 `layout` —— `copy()` 里 `cpy.noteSlide = this.noteSlide;` 是引用复制，
    //       `KmoSlide.addNoteSlide(KmoNoteSlide)` 亦直接赋值，故同样落
    //       `Option<Rc<RefCell<KmoNoteSlide>>>` 共享句柄。
    pub noteSlide: Option<std::rc::Rc<std::cell::RefCell<KmoNoteSlide>>>,
}

// Java: class SlideAtomData extends KmoAtomData
impl SlideAtomData {
    // Java: SlideAtomData()
    pub fn new() -> Self {
      // Java: super();
      let base = KmoAtomData::new();
      // Java: protected Slide slide = Slide.create();
      let slide = std::cell::RefCell::new(Slide::create());
      // Java: protected KmoSlideLayout layout;  —— 默认 null
      let layout = None;
      // Java: protected KmoNoteSlide noteSlide;  —— 默认 null
      let noteSlide = None;
      SlideAtomData {
          base,
          slide,
          layout,
          noteSlide,
      }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> SlideAtomData {
      // Java: SlideAtomData cpy = new SlideAtomData();
      let mut cpy = SlideAtomData::new();
      // Java: cpy.slide.load(this.slide.save());
      let bin = self.slide.borrow_mut().save();
      cpy.slide.borrow_mut().load(bin.as_ref());
      // Java: cpy.layout = this.layout;
      // rust: `Rc::clone` 即 Java 的引用复制（共享同一 layout，非深拷贝）。
      cpy.layout = self.layout.clone();
      // Java: cpy.noteSlide = this.noteSlide;
      // rust: `Rc::clone` 即 Java 的引用复制（共享同一备注页，非深拷贝）。
      cpy.noteSlide = self.noteSlide.clone();
      // Java: return cpy;
      cpy
    }
}

// Java: extends KmoAtomData
impl KmoAtomDataSub for SlideAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        SlideAtomData::copy(self)
    }
}

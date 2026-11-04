#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: package cn.wps.show.app.slide
// Java: import cn.wps.memor4j.KmoAtomData
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// Java: import cn.wps.show.data.noteslides.NoteSlide
// rust: `cn.wps.show.data.noteslides` 在 Rust 侧**整包不存在**（Java `NoteSlide.java` 293 行，
//       文件与目录均缺失）。参见下方字段注释。

// Java: class NoteSlideAtomData extends KmoAtomData
pub struct NoteSlideAtomData {
    // Java: extends KmoAtomData
    pub base: KmoAtomData,
    // Java field: protected NoteSlide noteSlide = NoteSlide.create();
    //
    // ★ SP5 阻塞点（登记，不擅自改语义）：
    //   Java 数据类 `cn.wps.show.data.noteslides.NoteSlide`（293 行）在 Rust 侧不存在，
    //   其移植与内容级解析（`p:notes` 文档）按计划归 M-PPT-NOTES：
    //     M_PPT_KMOSLIDE_PLAN §0「`KmoNoteSlide` 的**内容**……留给 M-PPT-NOTES」
    //     SP5 §4 5.5「`KmoNoteSlide` 死壳转真……**内容级**解析归 M-PPT-NOTES，登记边界」
    //   目标 Rust 类型：`std::cell::RefCell<NoteSlide>`（与 `SlideAtomData.slide` /
    //   `LayoutAtomData.layout` 同策略：Java `copy()` 以 `&this` 调 `noteSlide.save()`）。
    //     owner: show.data.noteslides    wave: M-PPT-NOTES
    // rust: `NoteSlide.rs` 落地后本字段改为 `RefCell<NoteSlide>` 并恢复 `KmoNoteSlide` 的
    //       `protocol()/updateProtocol()/copy()`。
    pub noteSlide: Option<()>,
}

// Java: class NoteSlideAtomData extends KmoAtomData
impl NoteSlideAtomData {
    // Java: NoteSlideAtomData()
    pub fn new() -> Self {
      // Java: super();
      let base = KmoAtomData::new();
      // Java: protected NoteSlide noteSlide = NoteSlide.create();  —— 数据类缺失（见字段注释）
      let noteSlide = None;
      NoteSlideAtomData {
          base,
          noteSlide,
      }
    }

    // Java: public KmoAtomData copy()
    pub fn copy(&self) -> NoteSlideAtomData {
      // Java: NoteSlideAtomData cpy = new NoteSlideAtomData();
      let mut cpy = NoteSlideAtomData::new();
      // Java: cpy.noteSlide.load(this.noteSlide.save());
      // rust: 内容级往返归 M-PPT-NOTES（`NoteSlide` 数据类缺失，见字段注释）。
      // Java: return cpy;
      cpy
    }
}

// Java: extends KmoAtomData
impl KmoAtomDataSub for NoteSlideAtomData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // Java: public KmoAtomData copy()
    fn copy(&self) -> Self {
        NoteSlideAtomData::copy(self)
    }
}

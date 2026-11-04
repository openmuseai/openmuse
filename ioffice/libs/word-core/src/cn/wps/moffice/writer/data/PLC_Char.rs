#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.io.dom.Node
// (PLC.Node — not io.dom.Node)

// java public class PLC_Char extends PLC
// rust: `PLCField` 由 `#[derive(Default)]` 生成 ⇒ 基类槽 `PLC_Char` 亦须 `Default`。
//   `PLC` 已有 `impl Default`（无参、空文档），沿用即可。
#[derive(Default)]
pub struct PLC_Char {
    // java PLC super_
    pub(crate) super_: PLC,
}

// java public class PLC_Char extends PLC
impl PLC_Char {
    // java public  PLC_Char(Document document)
    pub fn PLC_Char(document: &dyn Document) -> Self {
      // java super(document);
      Self {
            super_: PLC::new(Some(document)),
        }
    }

    // rust-only: Rust 门面别名 —— `PLCField.rs:119` 写作 `PLC_Char::new(document)`，
    //   对应 Java `super(document)`（`PLCField.java:...`）。
    //   与仓库既有 `Type::new()` 约定一致；语义与 `PLC_Char()` 逐字相同。
    pub fn new(document: &dyn Document) -> Self {
      Self::PLC_Char(document)
    }

    // java public void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      // java PLC.Node iter = this.lowerBound(start);
      let iter: Node = self.super_.lowerBound__int(start);
      // java this.insertSpaceBefore(iter, end - start);
      self.super_.insertSpaceBefore(iter, end - start);
    }

    // java public void doOnRemoveText(int start, int end)
    pub fn doOnRemoveText(&mut self, start: i32, end: i32) {
      // java PLC.Node s = this.lowerBound(start);
      let mut s: Node = self.super_.lowerBound__int(start);
      // java PLC.Node e;
      let mut e: Node;
      // java PLC.Node s_next;
      let mut s_next: Node;
      // java for(e = this.lowerBound(end); s != e; s = s_next)
      e = self.super_.lowerBound__int(end);
      while !std::ptr::eq(s.as_pos(), e.as_pos()) {
        // java s_next = s.getNext();
        s_next = s.getNext();
        // java this.remove(s);
        self.super_.remove(s);
        s = s_next;
      }
      // java this.eraseSpaceBefore(e, end - start);
      self.super_.eraseSpaceBefore(e, end - start);
    }
}

impl std::ops::Deref for PLC_Char {
    type Target = PLC;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLC_Char {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

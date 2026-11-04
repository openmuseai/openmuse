// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;

// java public class PLC_Range extends PLC
// rust: ★M-STAGE2 P1-D1 —— Java 无显式无参构造，但 Rust 侧 `SubDocument` 的
//   字段初始化需要 `Default`；以 `PLC::new(None)` 构造（Java 侧等价于 document 为 null）。
impl Default for PLC_Range {
    fn default() -> Self {
        Self { super_: PLC::new(None) }
    }
}

pub struct PLC_Range {
    // java PLC super_
    pub(crate) super_: PLC,
}

// java public class PLC_Range extends PLC
impl PLC_Range {
    // java public  PLC_Range(Document document)
    pub fn PLC_Range(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC::new(Some(document)) }
    }

    // java public void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      // java PLC.Node iter = this.upperBound(start);
      let mut iter: Node = self.super_.upperBound__int(start);
      // java this.insertSpaceBefore(iter, end - start);
      self.super_.insertSpaceBefore(iter.clone(), end - start);
    }

    // java public void doOnRemoveText(int start, int end)
    pub fn doOnRemoveText(&mut self, start: i32, end: i32) {
      // java PLC.Node s = this.lowerBound(start);
      let mut s: Node = self.super_.lowerBound__int(start);
      // java PLC.Node e = this.lowerBound(end);
      let mut e: Node = self.super_.lowerBound__int(end);
      // java boolean removeEnd = start == this.getDocument().getLength();
      // rust: `getDocument()` 返回 `Option<&dyn Document>`（Java 侧可能为 null）。
      let mut removeEnd: bool = start == self.super_.getDocument().map_or(0, |d| d.getLength());
      // java if (s != e && !removeEnd && (this.isHead(e) || e.getCP() != end))
      if s != e && !removeEnd && self.super_.isHead(e.clone()) || e.getCP() != end {
        // java e = e.getPrev();
        e = e.getPrev();
        // java int e_pos = e.getCP();
        let mut e_pos: i32 = e.getCP();
        // java this.moveRight(e, end - e_pos);
        self.super_.moveRight(e.clone(), end - e_pos);
      }
      // java while(s != e)
      while s != e {
        // java PLC.Node s_next = s.getNext();
        let mut s_next: Node = s.getNext();
        // java this.remove(s);
        self.super_.remove(s.clone());
        // java s = s_next;
        s = s_next;
      }
      // java this.eraseSpaceBefore(e, end - start);
      self.super_.eraseSpaceBefore(e.clone(), end - start);
    }

}

impl std::ops::Deref for PLC_Range {
    type Target = PLC;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLC_Range {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

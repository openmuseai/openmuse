#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.io.dom.Node
// (PLC.Node — not io.dom.Node)

// java public class PLC_CP extends PLC
pub struct PLC_CP {
    // java PLC super_
    pub(crate) super_: PLC,
}

// java public class PLC_CP extends PLC
impl PLC_CP {
    // java public  PLC_CP(Document document)
    pub fn PLC_CP(document: &dyn Document) -> Self {
      // java super(document);
      Self {
            super_: PLC::new(Some(document)),
        }
    }

    // java public void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      // java PLC.Node iter = this.upperBound(start);
      let iter: Node = self.super_.upperBound__int(start);
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
        // java this.moveLeft(s, s.getCP() - start);
        self.super_.moveLeft(s.clone(), s.getCP() - start);
        s = s_next;
      }
      // java this.eraseSpaceBefore(e, end - start);
      self.super_.eraseSpaceBefore(e, end - start);
    }
}

impl std::ops::Deref for PLC_CP {
    type Target = PLC;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLC_CP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;

// java public class TableSetBRC
#[derive(Clone)]
pub struct TableSetBRC {
    // java int itcFirst = 0;
    pub(crate) itcFirst: i32,
    // java int itcLim = 0;
    pub(crate) itcLim: i32,
    // java int bordersToApply = 1;
    pub(crate) bordersToApply: i32,
    // java BRC brc = null;
    pub(crate) brc: Option<BRC>,
}

impl Default for TableSetBRC {
    fn default() -> Self {
        Self {
            itcFirst: 0,
            itcLim: 0,
            bordersToApply: 1,
            brc: None,
        }
    }
}

// java public class TableSetBRC
impl TableSetBRC {
    pub fn new() -> Self {
      Self::default()
    }

    // java public void setBordersToApply(int value)
    pub fn setBordersToApply(&mut self, value: i32) {
      // java this.bordersToApply = value;
      self.bordersToApply = value;
    }

    // java public int getBordersToApply()
    pub fn getBordersToApply(&self) -> i32 {
      // java return this.bordersToApply;
      return self.bordersToApply;
    }

    // java public void setRange(int first, int last)
    pub fn setRange(&mut self, first: i32, last: i32) {
      // java this.itcFirst = first;
      self.itcFirst = first;
      // java this.itcLim = last;
      self.itcLim = last;
    }

    // java public int getFirst()
    pub fn getFirst(&self) -> i32 {
      // java return this.itcFirst;
      return self.itcFirst;
    }

    // java public int getLast()
    pub fn getLast(&self) -> i32 {
      // java return this.itcLim;
      return self.itcLim;
    }

    // java public void setBRC(BRC brc)
    pub fn setBRC(&mut self, brc: Option<&BRC>) {
      // java this.brc = brc;
      self.brc = brc.cloned();
    }

    // java public BRC getBRC()
    pub fn getBRC(&self) -> Option<BRC> {
      // java return this.brc;
      return self.brc.clone();
    }

}

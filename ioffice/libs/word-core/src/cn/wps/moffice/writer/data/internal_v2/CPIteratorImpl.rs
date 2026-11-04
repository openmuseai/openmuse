// Java: import cn.wps.moffice.writer.data.internal_v2.ListPositionTree
use crate::cn::wps::moffice::writer::data::internal_v2::ListPositionTree::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.CPIterator
use crate::cn::wps::moffice::writer::data::internal_v2::CPIterator::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.CPTable
use crate::cn::wps::moffice::writer::data::internal_v2::CPTable::*;

// java public class CPIteratorImpl extends ListPositionTree.CPIterator
#[derive(Default)]
pub struct CPIteratorImpl {
    // java ListPositionTree.CPIterator super_
    pub(crate) super_: CPIterator,
    // java CPTable table;
    pub(crate) table: Option<CPTable>,
}

// java public class CPIteratorImpl extends ListPositionTree.CPIterator
impl CPIteratorImpl {
    // java public  CPIteratorImpl(CPTable table)
    pub fn CPIteratorImpl(table: &CPTable) -> Self {
      // java super(table.mTree);
      // java this.table = table;
      Self { super_: CPIterator::new(table.mTree), table: table }
    }

    // java public CPTable.Entry next()
    pub fn next(&mut self) -> Box<dyn Entry> {
      // java return (CPTable.Entry)super.next();
      return todo!("cast → Box<dyn Entry>");
    }

    // java public CPTable.Entry prev()
    pub fn prev(&mut self) -> Box<dyn Entry> {
      // java return (CPTable.Entry)super.prev();
      return todo!("cast → Box<dyn Entry>");
    }

    // java public CPTable.Entry getNode()
    pub fn getNode(&self) -> Box<dyn Entry> {
      // java return (CPTable.Entry)super.getNode();
      return todo!("cast → Box<dyn Entry>");
    }

    // java protected void moveRight(int steps)
    pub fn moveRight(&mut self, steps: i32) {
      // java super.moveRight(steps);
      self.super_.moveRight(steps);
      // java this.table.recordMoveRight(this.getNode(), steps);
      self.table.recordMoveRight(self.getNode(), steps);
    }

}

impl std::ops::Deref for CPIteratorImpl {
    type Target = CPIterator;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CPIteratorImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

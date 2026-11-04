// Java: import cn.wps.collections.IndexTreeExtension

use super::ReferenceTable::ReferenceTable;

// Java: public class IndexedReferenceTable extends ReferenceTable
pub struct IndexedReferenceTable {
    pub(crate) base: ReferenceTable,
    // Java field: IndexTreeExtension indexes
    pub(crate) indexes: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class IndexedReferenceTable extends ReferenceTable
impl IndexedReferenceTable {
    // Java: IndexedReferenceTable()
    pub fn new() -> Self {
      // Java: super();
      IndexedReferenceTable {
          base: ReferenceTable::new(),
          indexes: None,
      }
    }

    // Java: TextReference<T> createNode(T data)
    pub fn createNode(&self, data: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
      // Java: return new IndexedTextReference<T>(data);
      data
    }

    // Java: void createTree()
    pub fn createTree(&mut self) {
      // Java: super.createTree();
      self.base.createTree();
      // Java: this.indexes = new IndexTreeExtension();
      // Java: this.mTree.addExtension(this.indexes, 59);
    }

    // Java: int getIndex(TextReference<T> p)
    pub fn getIndex(&self, _p: &dyn std::any::Any) -> i32 {
      // Java: return this.indexes.index(p);
      0
    }

    // Java: TextReference<T> get(int index)
    pub fn get(&self, _index: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.indexes.get(index);
      None
    }

}

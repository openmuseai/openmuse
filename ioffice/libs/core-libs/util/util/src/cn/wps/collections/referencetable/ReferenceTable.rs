// Java: import cn.wps.collections.ITreeExtension
// Java: import cn.wps.collections.Node
// Java: import cn.wps.collections.PositionTreeExtension
// Java: import cn.wps.collections.RBBalancer
// Java: import cn.wps.collections.Tree
// Java: import cn.wps.moffice.transaction.Command
// Java: import cn.wps.moffice.transaction.Transaction

use std::sync::{Arc, Mutex};

// Java: public class ReferenceTable
pub struct ReferenceTable {
    // Java field: Tree mTree
    mTree: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: PositionTreeExtension positions
    positions: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int mCount
    pub(crate) mCount: i32,
    // Java field: Transaction mTransaction
    pub(crate) mTransaction: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: Listener<T> mListener
    mListener: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class CmdInsert
pub struct CmdInsert {
    // Java field: TextReference<T> mNext
    pub(crate) mNext: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int mOffset
    pub(crate) mOffset: i32,
    // Java field: TextReference<T> mNew
    pub(crate) mNew: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int mCp
    pub(crate) mCp: i32,
}

// Java: public class CmdRemove
pub struct CmdRemove {
    // Java field: TextReference<T> mNext
    pub(crate) mNext: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: TextReference<T> mTextReference
    pub(crate) mTextReference: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int cp
    pub(crate) cp: i32,
    // Java field: int offset
    pub(crate) offset: i32,
}

// Java: public class CmdInsertSpace
pub struct CmdInsertSpace {
    // Java field: TextReference<T> leaf
    pub(crate) leaf: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class CmdMoveRight
pub struct CmdMoveRight {
    // Java field: TextReference<T> node
    pub(crate) node: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int steps
    pub(crate) steps: i32,
}

// Java: public class CmdSetData
pub struct CmdSetData {
    // Java field: TextReference<T> node
    pub(crate) node: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: T data
    pub(crate) data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ReferenceTable
impl ReferenceTable {
    // Java: ReferenceTable()
    pub fn new() -> Self {
      // Java: super();
      let mut rt = ReferenceTable {
          mTree: None,
          positions: None,
          mCount: 0,
          mTransaction: None,
          mListener: None,
      };
      // Java: this.createTree();
      rt.createTree();
      rt
    }

    // Java: TextReference<T> createNode(T data)
    pub fn createNode(&self, data: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
      // Java: return new TextReference<T>(data);
      data
    }

    // Java: TextReference<T> createHead()
    pub fn createHead(&self) -> Box<dyn std::any::Any + Send + Sync> {
      // Java: return this.createNode(null);
      Box::new(0i32)
    }

    // Java: void createTree()
    pub fn createTree(&mut self) {
      // Java: this.mTree = new Tree(this.createHead(), new RBBalancer());
      // Java: this.positions = new PositionTreeExtension();
      // Java: this.mTree.addExtension(this.positions, 63);
    }

    // Java: void setTransaction(Transaction transaction)
    pub fn setTransaction(&mut self, transaction: Box<dyn std::any::Any + Send + Sync>) {
      // Java: this.mTransaction = transaction;
      self.mTransaction = Some(transaction);
    }

    // Java: Transaction getTransaction()
    pub fn getTransaction(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.mTransaction;
      &self.mTransaction
    }

    // Java: void setListener(Listener<T> listener)
    pub fn setListener(&mut self, listener: Box<dyn std::any::Any + Send + Sync>) {
      // Java: this.mListener = listener;
      self.mListener = Some(listener);
    }

    // Java: TextReference<T> lowerBound(int cp)
    pub fn lowerBound(&self, _cp: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.positions.lowerBound(cp);
      None
    }

    // Java: TextReference<T> upperBound(int cp)
    pub fn upperBound(&self, _cp: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.positions.upperBound(cp);
      None
    }

    // Java: TextReference<T> findFloor(int cp)
    pub fn findFloor(&self, cp: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: TextReference<T> pLowerBound = this.lowerBound(cp);
      let _pLowerBound = self.lowerBound(cp);
      // Java: if (pLowerBound.isHead())
      // {
      //    // Java: if (this.size() > 0)
      //    // {
      //       // Java: pLowerBound = (TextReference)pLowerBound.getPrev();
      //       // Java: return pLowerBound;
      //    // }
      //    // Java: else
      //    // {
      //       // Java: return null;
      //    // }
      // }
      // Java: else if (pLowerBound.position() != cp)
      // {
      //    // Java: TextReference<T> pPrev = (TextReference)pLowerBound.getPrev();
      //    // Java: return pPrev.isHead() ? null : pPrev;
      // }
      // Java: else
      // {
      //    // Java: return pLowerBound;
      // }
      None
    }

    // Java: TextReference<T> findCeiling(int cp)
    pub fn findCeiling(&self, cp: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: TextReference<T> pUpperBound = this.upperBound(cp);
      let _pUpperBound = self.upperBound(cp);
      // Java: return pUpperBound.isHead() ? null : pUpperBound;
      None
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.mCount;
      self.mCount
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.size() == 0;
      self.size() == 0
    }

    // Java: TextReference<T> leftmost()
    pub fn leftmost(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.mTree.leftmost();
      None
    }

    // Java: TextReference<T> rightmost()
    pub fn rightmost(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.mTree.rightmost();
      None
    }

    // Java: TextReference<T> iterator()
    pub fn iterator(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.leftmost();
      self.leftmost()
    }

    // Java: TextReference<T> head()
    pub fn head(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return (TextReference)this.mTree.getHead();
      None
    }

    // Java: TextReference<T> find(int cp)
    pub fn find(&self, cp: i32) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: TextReference<T> p = this.lowerBound(cp);
      let _p = self.lowerBound(cp);
      // Java: return p != this.head() && p.position() == cp ? p : null;
      None
    }

    // Java: int addReferenceImpl(int cp, int offset, TextReference<T> pos, TextReference<T> pNew)
    pub fn addReferenceImpl(&mut self, cp: i32, _offset: i32, _pos: &dyn std::any::Any, _p_new: &dyn std::any::Any) -> i32 {
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.beforeAddReference(cp, pNew.data);
      // }
      // Java: if (offset == -1)
      if offset == -1
      {
         // Java: this.mTree.insertLeft(pos, pNew);
         // Java: Node pLeftRoot = this.mTree.getLeftRoot(pNew);
         // Java: if (pLeftRoot.isHead())
         // {
            // Java: offset = cp;
            offset = cp;
         // }
         // Java: else
         // {
         //    // Java: offset = cp - this.positions.position(pLeftRoot);
         // }
         // Java: pNew.setOffset(offset);
      }
      // Java: else
      else
      {
         // Java: pNew.setOffset(offset);
         // Java: this.mTree.insertLeft(pos, pNew);
      }
      // Java: ++this.mCount;
      self.mCount += 1;
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdInsert(pNew, pos, cp, offset));
      // }
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.afterAddReference(pNew);
      // }
      // Java: return offset;
      offset
    }

    // Java: void removeReferenceImpl(TextReference<T> pNode, int cp, TextReference<T> pNext)
    pub fn removeReferenceImpl(&mut self, _p_node: &dyn std::any::Any, _cp: i32, _p_next: &dyn std::any::Any) {
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.beforeRemoveReference(pNode);
      // }
      // Java: this.mTree.erase(pNode);
      // Java: --this.mCount;
      self.mCount -= 1;
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdRemove(pNode, pNext, cp));
      // }
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.afterRemoveReference(pNode);
      // }
    }

    // Java: TextReference<T> addReference(int cp, T data)
    pub fn addReference_with_data(&mut self, cp: i32, data: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
      // Java: return this.addReference(cp, this.createNode(data));
      let node = self.createNode(data);
      self.addReference(cp, node)
    }

    // Java: TextReference<T> addReference(int cp, TextReference<T> pNew)
    pub fn addReference(&mut self, _cp: i32, p_new: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
      // Java: _offset: &[i32] = new int[]{0};
      // Java: TextReference<T> node = (TextReference)this.positions.lowerBound(cp, offset);
      // Java: this.addReferenceImpl(cp, offset[0], node, pNew);
      // Java: return pNew;
      p_new
    }

    // Java: void removeReference(TextReference<T> pNode)
    pub fn removeReference(&mut self, _p_node: &dyn std::any::Any) {
      // Java: this.removeReferenceImpl(pNode, pNode.position(), (TextReference)pNode.getNext());
    }

    // Java: void addExtension(ITreeExtension extension, int mask)
    pub fn addExtension(&mut self, _extension: Box<dyn std::any::Any + Send + Sync>, _mask: i32) {
      // Java: this.mTree.addExtension(extension, mask);
    }

    // Java: void insertSpaceBefore(TextReference<T> pFrom, int length)
    pub fn insertSpaceBefore(&mut self, _p_from: &dyn std::any::Any, _length: i32) {
      // Java: this.positions.insertSpaceBefore(pFrom, length);
      // Java: this.recordCmdInsertSpace(pFrom, length);
    }

    // Java: void eraseSpaceBefore(TextReference<T> pFrom, int length)
    pub fn eraseSpaceBefore(&mut self, _p_from: &dyn std::any::Any, _length: i32) {
      // Java: this.positions.eraseSpaceBefore(pFrom, length);
      // Java: this.recordCmdInsertSpace(pFrom, -length);
    }

    // Java: void recordCmdInsertSpace(TextReference<T> pFrom, int length)
    pub fn recordCmdInsertSpace(&self, _p_from: &dyn std::any::Any, _length: i32) {
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdInsertSpace(pFrom, length));
      // }
    }

    // Java: void moveRight(TextReference<T> pNode, int length)
    pub fn moveRight(&mut self, _p_node: &dyn std::any::Any, _length: i32) {
      // Java: this.positions.moveRight(pNode, length);
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdMoveRight(pNode, length));
      // }
    }

    // Java: void moveLeft(TextReference<T> pNode, int length)
    pub fn moveLeft(&mut self, _p_node: &dyn std::any::Any, _length: i32) {
      // Java: this.positions.moveLeft(pNode, length);
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdMoveRight(pNode, -length));
      // }
    }

    // Java: void setData(TextReference<T> pNode, T data)
    pub fn setData(&mut self, _p_node: &dyn std::any::Any, _data: Box<dyn std::any::Any + Send + Sync>) {
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.beforeDataChange(pNode, data);
      // }
      // Java: T pOldData = pNode.data;
      // Java: pNode.data = data;
      // Java: Transaction t = this.getTransaction();
      // Java: if (t != null && t.getState() == 1)
      // {
      //    // Java: t.submitCommand(new CmdSetData(pNode, pOldData));
      // }
      // Java: if (this.mListener != null)
      // {
      //    // Java: this.mListener.afterDataChange(pNode, pOldData);
      // }
    }

}

// Java: public class CmdInsert
impl CmdInsert {
    // Java: CmdInsert(TextReference<T> pNew, TextReference<T> pos, int cp, int offset)
    pub fn new(p_new: Box<dyn std::any::Any + Send + Sync>, pos: Box<dyn std::any::Any + Send + Sync>, cp: i32, offset: i32) -> Self {
         // Java: super();
         // Java: this.mNext = pos;
         // Java: this.mOffset = offset;
         // Java: this.mNew = pNew;
         // Java: this.mCp = cp;
         CmdInsert { mNext: Some(pos), mOffset: offset, mNew: Some(p_new), mCp: cp }
    }

    // Java: void execute()
    pub fn execute(&self) {
         // Java: ReferenceTable.this.addReferenceImpl(this.mCp, this.mOffset, this.mNext, this.mNew);
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
         // Java: ReferenceTable.this.removeReferenceImpl(this.mNew, this.mCp, this.mNext);
    }

}

// Java: public class CmdRemove
impl CmdRemove {
    // Java: CmdRemove(TextReference<T> pTextReference, TextReference<T> pNext, int cp)
    pub fn new(p_text_ref: Box<dyn std::any::Any + Send + Sync>, p_next: Box<dyn std::any::Any + Send + Sync>, cp: i32) -> Self {
         // Java: super();
         // Java: this.mTextReference = pTextReference;
         // Java: this.mNext = pNext;
         // Java: this.cp = cp;
         CmdRemove { mNext: Some(p_next), mTextReference: Some(p_text_ref), cp, offset: 0 }
    }

    // Java: void execute()
    pub fn execute(&self) {
         // Java: ReferenceTable.this.removeReferenceImpl(this.mTextReference, this.cp, this.mNext);
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
         // Java: this.offset = ReferenceTable.this.addReferenceImpl(this.cp, this.offset, this.mNext, this.mTextReference);
    }

}

// Java: public class CmdInsertSpace
impl CmdInsertSpace {
    // Java: CmdInsertSpace(TextReference<T> leaf, int length)
    pub fn new(leaf: Box<dyn std::any::Any + Send + Sync>, length: i32) -> Self {
         // Java: super();
         // Java: this.leaf = leaf;
         // Java: this.length = length;
         CmdInsertSpace { leaf: Some(leaf), length }
    }

    // Java: void execute()
    pub fn execute(&self) {
         // Java: if (this.length < 0)
         // {
         //    // Java: ReferenceTable.this.eraseSpaceBefore(this.leaf, -this.length);
         // }
         // Java: else
         // {
         //    // Java: ReferenceTable.this.insertSpaceBefore(this.leaf, this.length);
         // }
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
         // Java: if (this.length < 0)
         // {
         //    // Java: ReferenceTable.this.insertSpaceBefore(this.leaf, -this.length);
         // }
         // Java: else
         // {
         //    // Java: ReferenceTable.this.eraseSpaceBefore(this.leaf, this.length);
         // }
    }

}

// Java: public class CmdMoveRight
impl CmdMoveRight {
    // Java: CmdMoveRight(TextReference<T> node, int steps)
    pub fn new(node: Box<dyn std::any::Any + Send + Sync>, steps: i32) -> Self {
         // Java: super();
         // Java: this.node = node;
         // Java: this.steps = steps;
         CmdMoveRight { node: Some(node), steps }
    }

    // Java: void execute()
    pub fn execute(&self) {
         // Java: if (this.steps < 0)
         // {
         //    // Java: ReferenceTable.this.moveLeft(this.node, -this.steps);
         // }
         // Java: else
         // {
         //    // Java: ReferenceTable.this.moveRight(this.node, this.steps);
         // }
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
         // Java: if (this.steps < 0)
         // {
         //    // Java: ReferenceTable.this.moveRight(this.node, -this.steps);
         // }
         // Java: else
         // {
         //    // Java: ReferenceTable.this.moveLeft(this.node, this.steps);
         // }
    }

}

// Java: public class CmdSetData
impl CmdSetData {
    // Java: void execute()
    pub fn execute(&self) {
         // Java: T pOldData = this.node.data;
         // Java: ReferenceTable.this.setData(this.node, this.data);
         // Java: this.data = pOldData;
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
         // Java: this.execute();
    }

}

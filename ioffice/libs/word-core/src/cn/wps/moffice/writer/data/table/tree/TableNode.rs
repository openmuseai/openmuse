// Java: import cn.wps.collections.IndexedPositionNode
use util::collections::IndexedPositionNode::*;
// Java: import cn.wps.collections.IndexedPositionTree
use util::collections::IndexedPositionTree::*;
// Java: import cn.wps.collections.Node
use util::collections::Node::*;
use util::collections::PositionNode::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TreeTableCache
use crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::*;

// java public abstract class TableNode extends IndexedPositionNode
pub struct TableNode {
    // java IndexedPositionNode super_
    pub(crate) super_: IndexedPositionNode,
    // java TreeTableCache mCache;
    // ownership glue: shared cache root (Java reference); raw ptr matches collections Node style
    pub(crate) mCache: *mut TreeTableCache,
    // java IndexedPositionTree mTree;
    pub(crate) mTree: Option<IndexedPositionTree>,
    // java int mLength;
    pub(crate) mLength: i32,
    // java int mGeneration;
    pub(crate) mGeneration: i32,
}

// java public abstract class TableNode extends IndexedPositionNode
impl TableNode {
    // java   TableNode(boolean createTree)
    pub fn TableNode(createTree: bool) -> Self {
      // java super(0, 0);
      let mut this = Self {
        super_: IndexedPositionNode::new(0, 0),
        mCache: std::ptr::null_mut(),
        mTree: None,
        mLength: 0,
        mGeneration: 0,
      };
      if createTree {
        // java this.createTree();
        this.createTree();
      }
      this
    }

    // java public int getGeneratioin()
    pub fn getGeneratioin(&self) -> i32 {
      // java return this.mGeneration;
      return self.mGeneration;
    }

    // java abstract TableNode newSubNode()
    pub fn newSubNode(&self) -> *mut TableNode {
      panic!("TableNode::newSubNode not implemented");
    }

    // java abstract TableNode getParentNode()
    pub fn getParentNode(&self) -> *mut TableNode {
      panic!("TableNode::getParentNode not implemented");
    }

    // java abstract void setParentNode(TableNode var1)
    pub fn setParentNode(&mut self, _var1: *mut TableNode) {
      panic!("TableNode::setParentNode not implemented");
    }

    // java public final Document getDocument()
    pub fn getDocument(&self) -> &Document {
      // java return this.mCache.mDocument;
      unsafe {
        (*self.mCache).mDocument.as_ref().expect("mDocument")
      }
    }

    // java final TableTool getTableTool()
    pub fn getTableTool(&self) -> &TableTool {
      // java return this.mCache.mTableTool;
      unsafe {
        (*self.mCache).mTableTool.as_ref().expect("mTableTool")
      }
    }

    // java public KTableManager getTableManager()
    pub fn getTableManager(&self) -> &KTableManager {
      // java return this.mCache.mTableManager;
      unsafe {
        (*self.mCache).mTableManager.as_ref().expect("mTableManager")
      }
    }

    // java final TableNode createSubNode()
    pub fn createSubNode(&mut self) -> *mut TableNode {
      // java TableNode newNode = this.newSubNode();
      let newNode: *mut TableNode = self.newSubNode();
      // java newNode.mCache = this.mCache;
      unsafe {
        (*newNode).mCache = self.mCache;
      }
      // java newNode.setParentNode(this);
      unsafe {
        (*newNode).setParentNode(self as *mut TableNode);
      }
      // java return newNode;
      return newNode;
    }

    // java final void createTree()
    pub fn createTree(&mut self) {
      // java if (this.mTree == null)
      if self.mTree.is_none() {
        // java this.doCreateTree();
        self.doCreateTree();
      }
    }

    // java  void doCreateTree()
    pub fn doCreateTree(&mut self) {
      // java this.mTree = new IndexedPositionTree(this.createSubNode());
      let head = self.createSubNode();
      unsafe {
        let head_pos = head as *mut PositionNode;
        self.mTree = Some(IndexedPositionTree::new(head_pos));
      }
    }

    // java final int offsetFromParent()
    pub fn offsetFromParent(&self) -> i32 {
      // java return this.position();
      return self.super_.position();
    }

    // java final TableNode getFirstChild()
    pub fn getFirstChild(&self) -> *mut TableNode {
      // java if (this.nodeCount() == 0)
      if self.nodeCount() == 0 {
        // java return null;
        return std::ptr::null_mut();
      }
      // java else
      else {
        // java TableNode first = (TableNode)this.mTree.leftmost();
        let first: *mut TableNode = unsafe {
          self.mTree.as_ref().unwrap().base.base.leftmost() as *mut TableNode
        };
        // java return first.isHead() ? null : first;
        unsafe {
          if (*first).isHead() {
            return std::ptr::null_mut();
          } else {
            return first;
          }
        }
      }
    }

    // java final TableNode getNextTableNode()
    pub fn getNextTableNode(&self) -> *mut TableNode {
      // java Node next = this.getNext();
      let mut next: *mut Node = unsafe { self.super_.base.base.getNext() };
      // java if (next.isHead())
      unsafe {
        if !next.is_null() && (*next).isHead() {
          // java next = null;
          next = std::ptr::null_mut();
        }
      }
      // java return (TableNode)next;
      return next as *mut TableNode;
    }

    // java final TableNode getPrevTableNode()
    pub fn getPrevTableNode(&self) -> *mut TableNode {
      // java Node prev = this.getPrev();
      let mut prev: *mut Node = unsafe { self.super_.base.base.getPrev() };
      // java if (prev.isHead())
      unsafe {
        if !prev.is_null() && (*prev).isHead() {
          // java prev = null;
          prev = std::ptr::null_mut();
        }
      }
      // java return (TableNode)prev;
      return prev as *mut TableNode;
    }

    // java final TableNode getNodeByCP(int cp)
    pub fn getNodeByCP(&self, cp: i32) -> *mut TableNode {
      // java if (this.nodeCount() == 0)
      if self.nodeCount() == 0 {
        // java return null;
        return std::ptr::null_mut();
      }
      // java else
      else {
        // java int position = this.getStartCP();
        let position: i32 = self.getStartCP();
        // java Node upper = this.mTree.upperBound(cp - position);
        let upper: *mut PositionNode = unsafe {
          self.mTree.as_ref().unwrap().base.upperBound(cp - position)
        };
        // java TableNode node = (TableNode)upper.getPrev();
        let node: *mut TableNode = unsafe {
          (*upper).base.getPrev() as *mut TableNode
        };
        // java if (node.isHead())
        unsafe {
          if (*node).isHead() {
            // java return null;
            return std::ptr::null_mut();
          }
          // java else
          else {
            // java return position + node.offsetFromParent() + node.mLength <= cp ? null : node;
            if position + (*node).offsetFromParent() + (*node).mLength <= cp {
              return std::ptr::null_mut();
            } else {
              return node;
            }
          }
        }
      }
    }

    // java final TableNode getNodeByOffset(int offsetFromThis, int[] offsetBuf)
    pub fn getNodeByOffset(&mut self, offsetFromThis: i32, offsetBuf: Option<&mut [i32]>) -> *mut TableNode {
      // java return (TableNode)this.mTree.upperBound(offsetFromThis, offsetBuf).getPrev();
      unsafe {
        let upper = self.mTree.as_ref().unwrap().base.upperBound_with_offset(offsetFromThis, offsetBuf);
        (*upper).base.getPrev() as *mut TableNode
      }
    }

    // java final TableNode getNodeStrict(int offset, int[] offsetBuf)
    pub fn getNodeStrict(&mut self, offset: i32, offsetBuf: Option<&mut [i32]>) -> *mut TableNode {
      // java if (this.nodeCount() == 0)
      if self.nodeCount() == 0 {
        // java return null;
        return std::ptr::null_mut();
      }
      // java else
      else {
        // java TableNode node = (TableNode)this.mTree.upperBound(offset, offsetBuf).getPrev();
        let node: *mut TableNode = unsafe {
          let upper = self.mTree.as_ref().unwrap().base.upperBound_with_offset(offset, offsetBuf);
          (*upper).base.getPrev() as *mut TableNode
        };
        // java return !node.isHead() && offsetBuf[0] < node.mLength ? node : null;
        unsafe {
          let buf0 = offsetBuf.as_ref().map(|b| b[0]).unwrap_or(0);
          if !(*node).isHead() && buf0 < (*node).mLength {
            return node;
          } else {
            return std::ptr::null_mut();
          }
        }
      }
    }

    // java final int nodeCount()
    pub fn nodeCount(&self) -> i32 {
      // java return this.mTree == null ? 0 : this.mTree.size();
      return (if self.mTree.is_none() {
        0
      } else {
        unsafe { self.mTree.as_ref().unwrap().base.base.size() }
      });
    }

    // java public int getIndex()
    pub fn getIndex(&self) -> i32 {
      // java return this.index();
      return self.super_.index();
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.nodeCount();
      return self.nodeCount();
    }

    // java public int getStartCP()
    pub fn getStartCP(&self) -> i32 {
      // java int offset = 0;
      let mut offset: i32 = 0;
      // java for(TableNode node = this; node != this.mCache; node = node.getParentNode())
      let mut node: *const TableNode = self as *const TableNode;
      unsafe {
        while (node as *mut TableNode) != (self.mCache as *mut TableNode) {
          // java offset += node.offsetFromParent();
          offset = offset + (*node).offsetFromParent();
          node = (*node).getParentNode();
          if node.is_null() {
            break;
          }
        }
      }
      // java return offset;
      return offset;
    }

    // java public int getEndCP()
    pub fn getEndCP(&self) -> i32 {
      // java return this.getStartCP() + this.mLength;
      return self.getStartCP() + self.mLength;
    }

    // java public long getRange()
    pub fn getRange(&self) -> i64 {
      // java int position = this.getStartCP();
      let position: i32 = self.getStartCP();
      // java return RANGEFactory.getRANGE(position, position + this.mLength);
      return RANGEFactory::getRANGE(position, position + self.mLength);
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java if (this.isHead())
      if self.isHead() {
        // java return "mHead";
        return "mHead".to_string();
      }
      // java else
      else {
        // java StringBuilder builder = new StringBuilder();
        let mut builder: String = String::new();
        // java long range = this.getRange();
        let range: i64 = self.getRange();
        // java builder.append("index:");
        builder.push_str("index:");
        // java builder.append(this.getIndex());
        builder.push_str(&self.getIndex().to_string());
        // java builder.append("\nrange:");
        builder.push_str("\nrange:");
        // java builder.append(RANGEFactory.toString(range));
        builder.push_str(&RANGEFactory::toString(range));
        // java builder.append("\nnodeCount:");
        builder.push_str("\nnodeCount:");
        // java builder.append(this.nodeCount());
        builder.push_str(&self.nodeCount().to_string());
        // java if (this.nodeCount() > 0)
        if self.nodeCount() > 0 {
          // java TableNode subNode = (TableNode)this.mTree.leftmost();
          let mut subNode: *mut TableNode = unsafe {
            self.mTree.as_ref().unwrap().base.base.leftmost() as *mut TableNode
          };
          let mut i: i32 = 0;
          // java for(int i = 0; !subNode.isHead() && i < 10; ++i)
          unsafe {
            while !(*subNode).isHead() && i < 10 {
              // java long subRange = subNode.getRange();
              let subRange: i64 = (*subNode).getRange();
              // java builder.append('\n');
              builder.push('\n');
              // java builder.append(i);
              builder.push_str(&i.to_string());
              // java builder.append(") ");
              builder.push_str(") ");
              // java builder.append(RANGEFactory.toString(subRange));
              builder.push_str(&RANGEFactory::toString(subRange));
              // java subNode = (TableNode)subNode.getNext();
              subNode = (*subNode).super_.base.base.getNext() as *mut TableNode;
              i += 1;
            }
            // java if (!subNode.isHead())
            if !(*subNode).isHead() {
              // java builder.append("\n...");
              builder.push_str("\n...");
            }
          }
        }
        // java return builder.toString();
        return builder;
      }
    }

    // java  void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offsetFromThis: i32, steps: i32, offsetBuf: &mut [i32]) {
      // java this.mLength += steps;
      self.mLength = self.mLength + steps;
      // java if (this.nodeCount() != 0)
      if self.nodeCount() != 0 {
        // java TableNode upper = (TableNode)this.mTree.upperBound(offsetFromThis, offsetBuf);
        let upper: *mut TableNode = unsafe {
          self.mTree.as_ref().unwrap().base.upperBound_with_offset(offsetFromThis, Some(offsetBuf)) as *mut TableNode
        };
        // java TableNode node = (TableNode)upper.getPrev();
        let node: *mut TableNode = unsafe { (*upper).super_.base.base.getPrev() as *mut TableNode };
        // java if (steps > 0)
        if steps > 0 {
          // java if (!node.isHead() && offsetBuf[0] < node.mLength)
          unsafe {
            if !(*node).isHead() && offsetBuf[0] < (*node).mLength {
              // java node.positionFix(offsetBuf[0], steps, offsetBuf);
              (*node).positionFix(offsetBuf[0], steps, offsetBuf);
            }
          }
        }
        // java else
        else {
          // java int deleted = -steps;
          let deleted: i32 = -steps;
          // java if (!node.isHead() && offsetBuf[0] < node.mLength)
          unsafe {
            if !(*node).isHead() && offsetBuf[0] < (*node).mLength {
              // java int subDeleted = Math.min(deleted, node.mLength - offsetBuf[0]);
              let subDeleted: i32 = std::cmp::min(deleted, (*node).mLength - offsetBuf[0]);
              // java node.positionFix(offsetBuf[0], -subDeleted, offsetBuf);
              (*node).positionFix(offsetBuf[0], -subDeleted, offsetBuf);
            }
            // java if (!upper.isHead())
            if !(*upper).isHead() {
              // java int upperDeleted = offsetFromThis + deleted - upper.offsetFromParent();
              let upperDeleted: i32 = offsetFromThis + deleted - (*upper).offsetFromParent();
              // java if (upperDeleted > 0)
              if upperDeleted > 0 {
                // java this.mTree.moveRight(upper, upperDeleted);
                self.mTree.as_ref().unwrap().base.moveRight(upper as *mut PositionNode, upperDeleted);
                // java upper.positionFix(0, -upperDeleted, offsetBuf);
                (*upper).positionFix(0, -upperDeleted, offsetBuf);
              }
            }
          }
        }
        // java this.mTree.insertSpaceBefore(upper, steps);
        unsafe {
          self.mTree.as_ref().unwrap().base.insertSpaceBefore(upper as *mut PositionNode, steps);
        }
      }
    }

    // java  boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32, offsetBuf: &mut [i32]) -> bool {
      // java boolean nodeChanged = false;
      let mut nodeChanged: bool = false;
      // java int length = end - start;
      let length: i32 = end - start;
      // java if (this.nodeCount() == 0)
      if self.nodeCount() == 0 {
        // java return nodeChanged;
        return nodeChanged;
      }
      // java else
      else {
        // java TableNode last = this.getNodeByOffset(end - 1, (int[])null);
        let last: *mut TableNode = self.getNodeByOffset(end - 1, None);
        // java if (last.isHead())
        unsafe {
          if (*last).isHead() {
            // java return nodeChanged;
            return nodeChanged;
          }
          // java else
          else {
            // java int lastOffset = last.offsetFromParent();
            let lastOffset: i32 = (*last).offsetFromParent();
            // java if (lastOffset + last.mLength <= start)
            if lastOffset + (*last).mLength <= start {
              // java return nodeChanged;
              return nodeChanged;
            }
            // java else
            else {
              // java TableNode next = (TableNode)this.mTree.upperBound(start, offsetBuf);
              let mut next: *mut TableNode = self.mTree.as_ref().unwrap().base.upperBound_with_offset(start, Some(offsetBuf)) as *mut TableNode;
              // java TableNode node = (TableNode)next.getPrev();
              let mut node: *mut TableNode = (*next).super_.base.base.getPrev() as *mut TableNode;
              // java if (node != last)
              if node != last {
                // java if (!node.isHead() && offsetBuf[0] < node.mLength)
                if !(*node).isHead() && offsetBuf[0] < (*node).mLength {
                  // java int subEnd = Math.min(node.mLength, offsetBuf[0] + length);
                  let subEnd: i32 = std::cmp::min((*node).mLength, offsetBuf[0] + length);
                  // java if (offsetBuf[0] == 0 && subEnd == node.mLength)
                  if offsetBuf[0] == 0 && subEnd == (*node).mLength {
                    // java this.eraseSubNode(node);
                    self.eraseSubNode(node as *mut Node);
                    // java nodeChanged = true;
                    nodeChanged = true;
                  }
                  // java else
                  else {
                    // java nodeChanged |= node.removeNodeInRange(offsetBuf[0], subEnd, offsetBuf);
                    nodeChanged = nodeChanged | (*node).removeNodeInRange(offsetBuf[0], subEnd, offsetBuf);
                  }
                }
                // java while (next != last)
                while next != last {
                  // java node = next;
                  node = next;
                  // java next = (TableNode)next.getNext();
                  next = (*next).super_.base.base.getNext() as *mut TableNode;
                  // java this.eraseSubNode(node);
                  self.eraseSubNode(node as *mut Node);
                  // java nodeChanged = true;
                  nodeChanged = true;
                }
              }
              // java int subStart = start < lastOffset ? 0 : start - lastOffset;
              let subStart: i32 = (if start < lastOffset { 0 } else { start - lastOffset });
              // java int subEnd = Math.min(last.mLength, end - lastOffset);
              let subEnd: i32 = std::cmp::min((*last).mLength, end - lastOffset);
              // java if (subStart == 0 && subEnd == last.mLength)
              if subStart == 0 && subEnd == (*last).mLength {
                // java this.eraseSubNode(last);
                self.eraseSubNode(last as *mut Node);
                // java nodeChanged = true;
                nodeChanged = true;
              }
              // java else
              else {
                // java nodeChanged |= last.removeNodeInRange(subStart, subEnd, offsetBuf);
                nodeChanged = nodeChanged | (*last).removeNodeInRange(subStart, subEnd, offsetBuf);
              }
              // java return nodeChanged;
              return nodeChanged;
            }
          }
        }
      }
    }

    // java protected void eraseSubNode(Node n)
    pub fn eraseSubNode(&mut self, n: *mut Node) {
      unsafe {
        // java ((TableNode)n).close();
        (*(n as *mut TableNode)).close();
        // java Node parent = n.parent;
        let parent: *mut Node = (*n).parent;
        // java int offset = ((TableNode)n).offset;
        let offset: i32 = (*(n as *mut TableNode)).super_.base.offset;
        // java this.mTree.erase(n);
        self.mTree.as_mut().unwrap().base.base.erase(n);
        // java n.parent = parent;
        (*n).parent = parent;
        // java ((TableNode)n).offset = offset;
        (*(n as *mut TableNode)).super_.base.offset = offset;
      }
    }

    // java protected void close()
    pub fn close(&mut self) {
    }

    // java abstract void valid(int var1, int var2)
    pub fn valid(&mut self, _var1: i32, _var2: i32) {
      panic!("TableNode::valid not implemented");
    }

    fn isHead(&self) -> bool {
      self.super_.base.base.isHead()
    }

}

impl std::ops::Deref for TableNode {
    type Target = IndexedPositionNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

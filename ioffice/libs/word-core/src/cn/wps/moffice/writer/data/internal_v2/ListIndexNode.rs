// Java: import cn.wps.collections.IndexTreeExtension
use util::collections::IndexTreeExtension::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;

// java public class ListIndexNode extends ListNode implements IndexTreeExtension.IIndexTreeNode
#[repr(C)]
pub struct ListIndexNode {
    // java ListNode super_
    pub(crate) super_: ListNode,
    // java int mLeftCount;
    pub(crate) mLeftCount: i32,
}

impl Default for ListIndexNode {
    fn default() -> Self {
        Self {
            super_: ListNode::default(),
            mLeftCount: -1,
        }
    }
}

impl ListIndexNode {
    // java public  ListIndexNode(int offset, int leftCount)
    pub fn ListIndexNode(offset: i32, leftCount: i32) -> Self {
      Self { super_: ListNode::ListNode(offset), mLeftCount: leftCount }
    }

    pub fn new(offset: i32, leftCount: i32) -> Self {
      Self::ListIndexNode(offset, leftCount)
    }

    pub fn index(&self) -> i32 {
      let _ = IndexTreeExtension::new();
      self.mLeftCount
    }

    pub fn getLeftCount(&self) -> i32 {
      self.mLeftCount
    }

    pub fn setLeftCount(&mut self, nLeftCount: i32) {
      self.mLeftCount = nLeftCount;
    }
}

impl std::ops::Deref for ListIndexNode {
    type Target = ListNode;
    fn deref(&self) -> &Self::Target { &self.super_ }
}

impl std::ops::DerefMut for ListIndexNode {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

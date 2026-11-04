// Java: import cn.wps.collections.IntervalNode
use crate::cn::wps::collections::IntervalNode::IntervalNode;
use util::collections::PositionNode::PositionNode;
use util::collections::Node::Node;

// java public abstract class IntervalMapNode extends IntervalNode
#[repr(C)]
pub struct IntervalMapNode {
    // java IntervalNode super_
    pub(crate) super_: IntervalNode,
    // java IntervalMapNode peer;
    pub(crate) peer: *mut IntervalMapNode,
    /// Rust glue for Java virtual getLength: Node1 is the length host, Node2 reads peer.
    pub(crate) is_len_host: bool,
}

// java public abstract class IntervalMapNode extends IntervalNode
impl IntervalMapNode {
    pub fn new() -> Self {
        Self {
            super_: IntervalNode::new(),
            peer: std::ptr::null_mut(),
            is_len_host: false,
        }
    }

    /// Java `IntervalNode.getLength()`: Node1.mLength, Node2.peer.getLength().
    /// Layout matches `IntervalMapNode1` (`IntervalMapNode` + `mLength`) without importing IntervalMap.
    pub unsafe fn virtual_get_length(n: *mut IntervalNode) -> i32 {
        #[repr(C)]
        struct Node1Layout {
            super_: IntervalMapNode,
            mLength: i32,
        }
        if n.is_null() {
            return 0;
        }
        let map = n as *mut IntervalMapNode;
        if (*map).is_len_host {
            (*(map as *mut Node1Layout)).mLength
        } else if !(*map).peer.is_null() {
            (*((*map).peer as *mut Node1Layout)).mLength
        } else {
            0
        }
    }

    // java public final IntervalMapNode getPeer()
    pub fn getPeer(&self) -> *mut IntervalMapNode {
      // java return this.peer;
      return self.peer;
    }

    pub fn as_interval_node_ptr(&mut self) -> *mut IntervalNode {
        &mut self.super_ as *mut IntervalNode
    }

    pub fn as_position_node_ptr(&mut self) -> *mut PositionNode {
        self.super_.as_position_node_ptr()
    }

    pub fn as_node_ptr(&mut self) -> *mut Node {
        self.super_.as_node_ptr()
    }
}

impl std::ops::Deref for IntervalMapNode {
    type Target = IntervalNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for IntervalMapNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl Default for IntervalMapNode {
    fn default() -> Self {
        Self::new()
    }
}

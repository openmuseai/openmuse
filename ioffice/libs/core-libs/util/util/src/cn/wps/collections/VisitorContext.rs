#![allow(dead_code, non_snake_case)]

// Java field: int NODE_LEFT_CHILD
pub const NODE_LEFT_CHILD: i32 = 1;
// Java field: int NODE_RIGHT_CHILD
pub const NODE_RIGHT_CHILD: i32 = 2;
// Java field: int NODE_ROOT
pub const NODE_ROOT: i32 = 0;

// Java: public class VisitorContext
pub struct VisitorContext {
    // Java field: int level
    pub(crate) level: i32,
    // Java field: int nodeType
    pub(crate) nodeType: i32,
    // Java field: int totalSize
    pub(crate) totalSize: i32,
}

// Java: public class VisitorContext
impl VisitorContext {
    // Java: VisitorContext()
    pub fn new() -> Self {
        // Java: super();
        Self {
            level: 0,
            nodeType: 0,
            totalSize: 0,
        }
    }

    // Java: int getLevel()
    pub fn getLevel(&self) -> i32 {
        // Java: return this.level;
        self.level
    }

    // Java: int getNodeType()
    pub fn getNodeType(&self) -> i32 {
        // Java: return this.nodeType;
        self.nodeType
    }

    // Java: int getTotalSize()
    pub fn getTotalSize(&self) -> i32 {
        // Java: return this.totalSize;
        self.totalSize
    }
}

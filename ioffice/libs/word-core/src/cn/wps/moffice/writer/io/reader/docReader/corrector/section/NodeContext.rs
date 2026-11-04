// Java: cn.wps.moffice.writer.io.reader.docReader.corrector.section.NodeContext
//
// ★M-STAGE1 P2-4 —— 由 auto-generated 桩（17 行 / 7 处 `todo!()`）**做实**。
//   Java 原文 26 行：3 个 int 字段 + 3 对 getter/setter，无显式构造器。

// java public class NodeContext
#[derive(Default, Clone, Copy, Debug)]
pub struct NodeContext {
    // java private int mContentStartCp;
    mContentStartCp: i32,
    // java private int mNodeCp;
    mNodeCp: i32,
    // java private int mContentLength;
    mContentLength: i32,
}

// java public class NodeContext
impl NodeContext {
    // rust: Java 无显式构造器（走默认构造）⇒ 此处 `new()` 为 Rust 侧便利方法，
    //   不对应 Java 成员，故不写 `// java` 方法注释。
    pub fn new() -> Self {
        Self::default()
    }

    // java public int getContentStartCp()
    pub fn getContentStartCp(&self) -> i32 {
        // java return this.mContentStartCp;
        return self.mContentStartCp;
    }

    // java public void setContentStartCp(int contentStartCp)
    pub fn setContentStartCp(&mut self, contentStartCp: i32) {
        // java this.mContentStartCp = contentStartCp;
        self.mContentStartCp = contentStartCp;
    }

    // java public int getNodeCp()
    pub fn getNodeCp(&self) -> i32 {
        // java return this.mNodeCp;
        return self.mNodeCp;
    }

    // java public void setNodeCp(int nodeCp)
    pub fn setNodeCp(&mut self, nodeCp: i32) {
        // java this.mNodeCp = nodeCp;
        self.mNodeCp = nodeCp;
    }

    // java public int getContentLength()
    pub fn getContentLength(&self) -> i32 {
        // java return this.mContentLength;
        return self.mContentLength;
    }

    // java public void setContentLength(int contentLength)
    pub fn setContentLength(&mut self, contentLength: i32) {
        // java this.mContentLength = contentLength;
        self.mContentLength = contentLength;
    }
}

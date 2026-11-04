// Java: cn.wps.moffice.writer.io.reader.xml07.cl.ClRefInfo
//
// ★M-STAGE1 P2-12 —— 由**未登记**（⇒ 2 个类型错误长期不可见）修正并登记。
//   原文 15 行：3 个字段 + 1 对访问器。
//
// rust: ★Java 引用语义 —— `private ClRefInfo mNext = null;` 是**自引用链表**，
//   `setNext(next)` 存的是**引用**（Java 对象共享）。
//   Rust 用裸指针保持该共享语义；若用 `Option<Box<Self>>` 值语义，
//   调用方的修改将**不可见**（与 §44.2 `mPLCHeader` 同一类缺陷）。

// java public class ClRefInfo
#[derive(Default)]
pub struct ClRefInfo {
    // java public String mRid;
    pub(crate) mRid: Option<String>,
    // java public Integer mHdrFtrType;
    pub(crate) mHdrFtrType: Option<i32>,
    // java private ClRefInfo mNext = null;
    // rust: ★Java 引用语义（自引用链表）
    mNext: Option<*mut ClRefInfo>,
}

// java public class ClRefInfo
impl ClRefInfo {
    // java public void setNext(ClRefInfo next)
    pub fn setNext(&mut self, next: Option<*mut ClRefInfo>) {
      // java this.mNext = next;
      self.mNext = next;
    }

    // java public ClRefInfo getNext()
    pub fn getNext(&self) -> Option<*mut ClRefInfo> {
      // java return this.mNext;
      return self.mNext;
    }

}

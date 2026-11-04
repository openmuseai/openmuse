// Java: public class OleInfo
// rust: ★M-STAGE2 P1-D1 / T0-2 —— Java `OleInfo` 是仅含无参 ctor 的空基类
//   （`OleInfo.java` 7 行）。子类模板以 `OleInfo::new()` / `..Default::default()` 构造，
//   故补 `Default` 与 `new`（等价于 Java 的 `super()`）。
#[derive(Default)]
pub struct OleInfo {
}

// Java: public class OleInfo
impl OleInfo {
    // Java: OleInfo()
    pub fn OleInfo(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: OleInfo()
    // rust: 模板调用点使用 `new()`；Java 无参 ctor 的 Rust 惯用等价形式。
    pub fn new() -> Self {
        Self::default()
    }
}

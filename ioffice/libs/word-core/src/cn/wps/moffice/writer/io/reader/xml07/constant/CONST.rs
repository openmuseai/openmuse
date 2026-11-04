// Java: cn.wps.moffice.writer.io.reader.xml07.constant.CONST
//
// ★M-STAGE1 P2-12 —— 全仓原缺失，按 Java 原文（10 行）新建。

// java public class CONST
#[derive(Default)]
pub struct CONST {
}

// java public class CONST
impl CONST {
    // rust: 原文只有 6 个 `public static final int` 常量，无方法
    //   ⇒ `new()` 为 Rust 侧便利方法（常量见模块级 `pub const`）。
    pub fn new() -> Self {
        Self::default()
    }
}

// java public static final int HDR_EVEN = 0;
pub const HDR_EVEN: i32 = 0;
// java public static final int HDR_ODD = 1;
pub const HDR_ODD: i32 = 1;
// java public static final int HDR_FIRST = 2;
pub const HDR_FIRST: i32 = 2;
// java public static final int FTR_EVEN = 3;
pub const FTR_EVEN: i32 = 3;
// java public static final int FTR_ODD = 4;
pub const FTR_ODD: i32 = 4;
// java public static final int FTR_FIRST = 5;
pub const FTR_FIRST: i32 = 5;

// cn.wps.moffice.writer.io.reader.dmlReader.context.Branch
//
// ★M-STAGE1 P2-10 —— 由 auto-generated 桩（11 行 / 1 处 todo 占位）做实。
//   原文 8 行（`@Deprecated`）：只有 1 个字段 `ArrayList<Element> mElements`。
//
// rust: 原文 `Element` 是抽象类，`mElements` 里实际装的是 `StartTag` / `EndTag`。
//   Java 用**基类引用 + 动态分派**；Rust 用带 `kind` 的枚举 `ElementRef` 复现
//   （观测等价：`type()` 的返回值即分派结果）。

// java import (same package) StartTag
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::EndTag::EndTag;
// java import (same package) EndTag
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::StartTag::StartTag;

/// rust: 原文抽象基类 `Element` 的动态分派在 Rust 侧的载体。
pub enum ElementRef {
    // rust: `StartTag`
    Start(StartTag),
    // rust: `EndTag`
    End(EndTag),
}

impl ElementRef {
    // java public abstract int type()
    pub fn type__0(&self) -> i32 {
        // java return 0; / return 1;
        return match self {
            ElementRef::Start(t) => t.type__0(),
            ElementRef::End(t) => t.type__0(),
        };
    }
}

// java public class Branch
#[derive(Default)]
pub struct Branch {
    // java public ArrayList<Element> mElements = new ArrayList();
    pub(crate) mElements: Vec<ElementRef>,
}

// java public class Branch
impl Branch {
    // rust: 原文无显式构造器（走默认构造 + 字段初始化器）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }
}

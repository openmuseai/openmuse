// Java: cn.wps.moffice.writer.io.reader.dmlReader.context.Element
//
// ★M-STAGE1 P2-10 —— 全仓原缺失，按 Java 源码新建（Java 原文 `@Deprecated`）。
//
// rust: Java 是 `public abstract class Element`，子类 `StartTag` / `EndTag` 覆写
//   `public abstract int type()`。Rust 用一个显式 `kind` 字段 + `type__0()` 复现
//   （`type` 是 Rust 关键字，无法直接作方法名；§3.8 gate 的 `fn` 名匹配规则
//    要求名字以 Java 名开头，故取 `type__0`）。

// Java: import cn.wps.io.opt.AttributesImpl
use io_wps_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java public static final int TYPE_START = 0;
pub const TYPE_START: i32 = 0;
// java public static final int TYPE_END = 1;
pub const TYPE_END: i32 = 1;

// java public abstract class Element
#[derive(Default)]
pub struct Element {
    // java public int mElement;
    pub(crate) mElement: i32,
    // java public Attributes mAttrs;
    // rust: ★Java `this.mAttrs = new AttributesImpl(attrs)` 只在 `attrs instanceof
    //   AttributesImpl` 时成立（`AttributesImpl(Attributes)` 构造器 else 分支是
    //   `IOAssert.fail("TODO")`）⇒ 形参类型取 `&AttributesImpl`。
    pub(crate) mAttrs: Option<AttributesImpl>,
    // rust: 见文件头 —— 复现 Java 子类的 `abstract int type()` 返回值
    pub(crate) mKind: i32,
}

// java public abstract class Element
impl Element {
    // java public Element(int id, Attributes attrs)
    pub fn Element(id: i32, attrs: Option<&AttributesImpl>) -> Self {
      // java this.mElement = id;
      let mElement: i32 = id;
      // java if (null != attrs)
      // java this.mAttrs = new AttributesImpl(attrs);
      let mAttrs: Option<AttributesImpl> = attrs.map(AttributesImpl::new_from_attrs);
      Self {
        mElement,
        mAttrs,
        mKind: TYPE_START,
      }
    }

    // java public abstract int type()
    pub fn type__0(&self) -> i32 {
      // rust: Java 抽象方法；`StartTag` 返回 0 / `EndTag` 返回 1，见对应文件。
      return self.mKind;
    }
}

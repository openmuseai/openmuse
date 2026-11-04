// Java: cn.wps.moffice.writer.io.reader.dmlReader.context.StartTag
//
// ★M-STAGE1 P2-10 —— 全仓原缺失，按 Java 源码新建（Java 原文 `@Deprecated`）。

// Java: import org.xml.sax.Attributes
use io_wps_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
// Java: （同包）Element
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::Element::Element;
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::Element::TYPE_START;

// java public class StartTag extends Element
#[derive(Default)]
pub struct StartTag {
    // java Element super_
    pub(crate) super_: Element,
}

// java public class StartTag extends Element
impl StartTag {
    // java public StartTag(int id, Attributes attrs)
    pub fn StartTag(id: i32, attrs: Option<&AttributesImpl>) -> Self {
      // java super(id, attrs);
      let mut e: Element = Element::Element(id, attrs);
      // rust: `mKind` 见下方 `type__0()`（Java 的 `type()` 返回值）
      e.mKind = TYPE_START;
      Self { super_: e }
    }

    // java public int type()
    pub fn type__0(&self) -> i32 {
      // java return 0;
      return TYPE_START;
    }
}

impl std::ops::Deref for StartTag {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for StartTag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// Java: cn.wps.moffice.writer.io.reader.dmlReader.context.EndTag
//
// ★M-STAGE1 P2-10 —— 全仓原缺失，按 Java 源码新建（Java 原文 `@Deprecated`）。

// Java: import org.xml.sax.Attributes
use io_wps_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
// Java: （同包）Element
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::Element::Element;
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::Element::TYPE_END;

// java public class EndTag extends Element
#[derive(Default)]
pub struct EndTag {
    // java Element super_
    pub(crate) super_: Element,
}

// java public class EndTag extends Element
impl EndTag {
    // java public EndTag(int id)
    pub fn EndTag(id: i32) -> Self {
      // java super(id, (Attributes)null);
      let mut e: Element = Element::Element(id, None);
      // rust: `mKind` 见下方 `type__0()`（Java 的 `type()` 返回值）
      e.mKind = TYPE_END;
      Self { super_: e }
    }

    // java public int type()
    pub fn type__0(&self) -> i32 {
      // java return 1;
      return TYPE_END;
    }
}

impl std::ops::Deref for EndTag {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EndTag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

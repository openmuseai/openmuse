// ★W8：本文件原先是**自动生成桩（假编译面形态 E）**——15 行、`use crate::*;`、五个 `todo!()`，
//   且**未登记**在 `reader/mod.rs`（文件存在却永不编译，形态 B 的孪生）。
//   Java 真身 `cn/wps/kfc/xpxml/reader/DefaultElementHandler.java` 只有 20 行，
//   是 `ElementHandler` 的**便利空实现基类**（四个方法体皆空）⇒ 完整真实移植。
//
// 下游：`ppt-core` 的 `show/pptxr/handler/**` 与 `drawing/dmlr/handler/**` 的全部 handler
// 在 Java 侧都 `extends DefaultElementHandler`，本类是它们 Rust 侧的真实基类。
// owner: kfc.xpxml.reader    wave: M-PPT-KMOSLIDE-W8

use super::ElementHandler::ElementHandler;
use super::XmlRoAttr::XmlRoAttr;

// Java: public class DefaultElementHandler implements ElementHandler
#[derive(Default, Clone, Copy, Debug)]
pub struct DefaultElementHandler;

// Java: public class DefaultElementHandler implements ElementHandler
impl DefaultElementHandler {
    // Java: public DefaultElementHandler()
    pub fn new() -> Self {
        // Java: super();
        DefaultElementHandler
    }
}

// Java: public class DefaultElementHandler implements ElementHandler
impl ElementHandler for DefaultElementHandler {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        // Java: （空体）
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （空体）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （空体）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return null;
        None
    }
}

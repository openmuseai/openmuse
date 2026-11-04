#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.media.Media
// rust: no-stmt —— `cn.wps.drawing.data.media.Media` 在本仓库**任何**构建下都不参与编译
//   （`drawing/data/mod.rs`：`#[cfg(feature = "full")] pub mod media;`；
//    `drawing/data/media/mod.rs`：全文 `#[cfg(any())] pub mod Media;`）
//   ⇒ 嵌套类 `Media.QuickTimeFile` 无法在类型里直接命名。本类的模型槽借 `HandlerCallback`
//   的关联类型 `QuickTimeFile` 承载（Java 侧由 `HandlerCallbackImpl` 绑定到 `Media.QuickTimeFile`）。
//   本类只把该对象透传给回调，不触碰它的成员，故无需本地模型契约。
//   owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W8a
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// rust: Java 的 `XmlRoAttr extends XmlRoAttrValue`，`temp.toString()` / `temp.toInt()` /
//   `temp.toDouble()` 由父接口提供。Rust 侧 `dyn XmlRoAttr` 的方法解析同样覆盖父 trait
//   `XmlRoAttrValue`（实测：额外 `use` 该 trait 会触发 `unused_imports`），故无需额外导入。

// Java: public class QuickTimeFileHandler extends DefaultElementHandler
pub struct QuickTimeFileHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.QuickTimeFile quickTimeFile
    quickTimeFile: Option<Rc<RefCell<C::QuickTimeFile>>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: public class QuickTimeFileHandler extends DefaultElementHandler
impl<C: HandlerCallback> QuickTimeFileHandler<C> {
    // Java: QuickTimeFileHandler(Media.QuickTimeFile quickTimeFile, HandlerCallback handlerCallback)
    pub fn new(
        quickTimeFile: Option<Rc<RefCell<C::QuickTimeFile>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = QuickTimeFileHandler {
            parent: DefaultElementHandler::new(),
            quickTimeFile,
            handlerCallback,
        };
        // Java: this.quickTimeFile = quickTimeFile;
        // LLM JavaLine Aggregation: merged into struct init above
        // Java: this.handlerCallback = handlerCallback;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

impl<C: HandlerCallback> ElementHandler for QuickTimeFileHandler<C> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((temp = attrsNotCopy.getByName(4259843)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(4259843) {
            // Java: this.handlerCallback.onQuickTimeFileLink(temp.toString(), this.quickTimeFile);
            let link = temp.to_string();
            // rust: `startElement` 取 `&self`，回调取 `&mut self`；句柄先 clone 再 `borrow_mut()`，
            //   借用期覆盖整条回调语句（同 `HyperLinkHandler`）。
            let quickTimeFile = self.quickTimeFile.clone();
            let mut quickTimeFile = quickTimeFile.as_ref().map(|m| m.borrow_mut());
            self.handlerCallback
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .onQuickTimeFileLink(Some(link.as_str()), quickTimeFile.as_deref_mut());
        }
    }

    // Java: public void endElement(int element)   —— 继承空实现
    fn endElement(&self, element: i32) {
        // Java: super.endElement(element);
        self.parent.endElement(element);
    }

    // Java: public void characters(String chars)   —— 继承空实现
    fn characters(&self, chars: &str) {
        // Java: super.characters(chars);
        self.parent.characters(chars);
    }

    // Java: public ElementHandler enterSubElement(int subElement)   —— 继承空实现
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return super.enterSubElement(subElement);
        self.parent.enterSubElement(subElement)
    }
}

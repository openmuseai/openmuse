#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.dmlr.handler.media.EmbeddedWAVAudioFileHandler
// rust: no-stmt —— 该处理器（Java 66 行）在默认构建下没有对应物：`dmlr/handler/media/mod.rs`
//   为空文件，只有本类 `enterSubElement` 的 case 1114221 需要它，见该处 DEFERRED 注。
//   owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toString` / `toBoolean` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的可空引用形参（`HandlerCallback callback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义（同 `CommonSlideDataHandler`
//   的 `type Callback<C> = Option<Shared<C>>`）。
type Callback<C> = Option<Rc<RefCell<C>>>;

// Java: public class HyperLinkHandler extends DefaultElementHandler
pub struct HyperLinkHandler<C: HandlerCallback<Hyperlink = Hyperlink>> {
    // Java field: Hyperlink hyperlink
    hyperlink: Option<Rc<RefCell<Hyperlink>>>,
    // Java field: HandlerCallback callback
    callback: Callback<C>,
}

// Java: public class HyperLinkHandler extends DefaultElementHandler
impl<C: HandlerCallback<Hyperlink = Hyperlink>> HyperLinkHandler<C> {
    // Java: HyperLinkHandler(Hyperlink hyperlink, HandlerCallback callback)
    pub fn HyperLinkHandler(
        hyperlink: Option<Rc<RefCell<Hyperlink>>>,
        callback: Callback<C>,
    ) -> Self {
        // Java: this.hyperlink = hyperlink;
        let hyperlink = hyperlink;
        // Java: this.callback = callback;
        let callback = callback;
        HyperLinkHandler { hyperlink, callback }
    }
}

impl<C: HandlerCallback<Hyperlink = Hyperlink>> ElementHandler for HyperLinkHandler<C> {
    fn kind(&self) -> &'static str {
        "HyperLinkHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // rust: Java 用同一个 `temp` 变量串接 8 次 `getByName`；Rust 侧逐段 `if let Some(temp)`
        //   对照书写（每段一查一用，语义等价）。
        // Java: if (null != attrsNotCopy)
        // rust: no-stmt —— `attrsNotCopy` 是本契约的 `&XmlRoAttrList`，不可能为 null。
        // Java: {
        // Java: if ((temp = attrsNotCopy.getByName(4259841)) != null && null != this.callback)
        if let Some(temp) = attrsNotCopy.get_by_name(4259841) {
            if let Some(callback) = self.callback.as_ref() {
                // Java: this.callback.onHyperlinkRid(temp.toString(), this.hyperlink);
                let rId = temp.to_string();
                // rust: Java 传的是引用；Rust 侧借用期须覆盖整个调用语句，故先把
                //   hyperlink 的 `RefMut` 落地为局部量再传入。
                let hyperlink = self.hyperlink.clone();
                let mut hyperlink = hyperlink.as_ref().map(|h| h.borrow_mut());
                callback
                    .borrow_mut()
                    .onHyperlinkRid(Some(rId.as_str()), hyperlink.as_deref_mut());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114223)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114223) {
            if self.hyperlink.is_some() {
                // Java: this.callback.onHyperlinkAction(temp.toString(), this.hyperlink);
                // rust: 本分支 Java **没有**判 `this.callback != null`（与上一分支不同）；
                //   为对齐「callback 为 null 即 NPE」的 Java 语义，这里用
                //   `expect("NullPointerException")` 而非静默跳过。
                let callback = self.callback.as_ref().expect("NullPointerException");
                let action = temp.to_string();
                let hyperlink = self.hyperlink.clone();
                let mut hyperlink = hyperlink.as_ref().map(|h| h.borrow_mut());
                callback
                    .borrow_mut()
                    .onHyperlinkAction(Some(action.as_str()), hyperlink.as_deref_mut());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114228)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114228) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putEndSound(temp.toBoolean());
                hyperlink.borrow_mut().putEndSound(temp.to_boolean());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114227)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114227) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putHighlightClick(temp.toBoolean());
                hyperlink.borrow_mut().putHighlightClick(temp.to_boolean());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114226)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114226) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putHistory(temp.toBoolean());
                hyperlink.borrow_mut().putHistory(temp.to_boolean());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114222)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114222) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putInvalidUrl(temp.toString());
                let value = temp.to_string();
                hyperlink.borrow_mut().putInvalidUrl(&value);
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114224)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114224) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putTgtFrame(temp.toString());
                let value = temp.to_string();
                hyperlink.borrow_mut().putTgtFrame(&value);
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114225)) != null && null != this.hyperlink)
        if let Some(temp) = attrsNotCopy.get_by_name(1114225) {
            if let Some(hyperlink) = self.hyperlink.as_ref() {
                // Java: this.hyperlink.putTooltip(temp.toString());
                let value = temp.to_string();
                hyperlink.borrow_mut().putTooltip(&value);
            }
        }
        // Java: }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // rust: 该 `null` 初值即下面 `default:` 分支的返回值。
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114221:
            1114221 => {
                // Java: handler = new EmbeddedWAVAudioFileHandler(this.hyperlink.sound(), this.callback);
                // Java: default: return handler;   // Java 的 case 无 break，落到 default 后返回
                //
                // owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: EmbeddedWAVAudioFileHandler
                //   (cn.wps.drawing.dmlr.handler.media.EmbeddedWAVAudioFileHandler)
                //   —— 默认构建下无对应 Rust 文件；且 `Hyperlink` 只有
                //   hasSound/deleteSound/writeSound/forceWriteSound，缺 `sound()` 只读取值器，
                //   无法先取 `this.hyperlink.sound()` 再构造处理器。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

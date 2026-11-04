#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// M-PPT-KMOSLIDE W9：本文件此前**未登记**（`custdatas/mod.rs` 里的 `pub mod` 被注释成模板产物），
// 因此 106 行从不进入编译单元；同名的单元空壳 `shape::sw_shape_hosts::CustDataLstHandler`
// （`sw_shape_hosts.rs:254`，`new<L, C>` 丢弃两个实参）反而在管线里 —— `p:custDataLst` 解析出的
// tags 因此在 `CommonSlideDataHandler` 处被丢弃，`handlerCallback.onTags(...)` 从未被调用。
// 本波按 Java 真身（`CustDataLstHandler.java` 43 行）改写并登记，同时删除上述空壳。
//
// ★契约：规范契约 `kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler`
//   （`startElement(&self, i32, &dyn XmlRoAttr)` / `endElement(&self, i32)` /
//     `characters(&self, &str)` / `enterSubElement(&self, i32) -> Option<Box<dyn ElementHandler>>`），
//   即 `UnifiedXmlReader::read__inputstream_elementhandler` 的形参类型；全部接收者为 `&self`，
//   故内部可变状态一律经 `Rc<RefCell<..>>`。
//
// ★rust 偏差登记（owner: show.pptxr.handler.custdatas · wave: M-PPT-KMOSLIDE-W9）：
//   Java 字段 `CustDataLst custDataLst` 持有模型**活对象**。Rust 侧模型
//   `show::data::CommonSlideData.custDataLst` 是 `Option<Box<CustDataLst>>`，访问器
//   `get_custDataLst_mut() -> &mut CustDataLst`（`CommonSlideData.rs:690`）**无法投影**出
//   `Rc<RefCell<CustDataLst>>`。故本类改持**属主句柄** `Shared<CommonSlideData>`，在
//   `TagsHandler::startElement` 里惰性取 `get_custDataLst_mut().tags()` —— 解析出的 tags
//   仍写入模型**同一个** `CustDataLst`（随后由 `CommonSlideDataHandler::endElement` 的
//   `commonSlideData.save()` 落入 film 协议），对象同一性与 Java 一致，仅句柄层级不同。

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: Java `XmlRoAttr` 是**接口**，Rust 侧是 **trait** ⇒ 形参一律 `&dyn XmlRoAttr`。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use crate::cn::wps::show::data::custdatalst::CustDataLst::{CustDataLst, TagLst};
// Java: import cn.wps.show.data.CommonSlideData
// rust: 属主句柄类型（见文件头偏差登记）。
use crate::cn::wps::show::data::CommonSlideData::CommonSlideData;

// rust: 别名（Java 无）—— `Rc<RefCell<..>>` 承载 Java 的共享可变引用语义（repo 既有约定）。
type Shared<T> = Rc<RefCell<T>>;

// ---------------------------------------------------------------------------
// rust: 本地模型契约 —— `HandlerCallback` 的 tags 槽位投影（owner/wave 见文件头）
// ---------------------------------------------------------------------------

/// Java `HandlerCallback.onTags(String id, CustDataLst.TagLst tagLst)` 的第二个形参在 Rust 侧是
/// **关联类型** `C::TagLst`（`HandlerCallback.rs:80,111`），而模型侧活对象类型是
/// `CustDataLst::TagLst`。二者只有在回调把该关联类型绑定到真实类型时才同名；本 trait 承载这次投影：
///   · `impl TagLstSlot<C> for TagLst` —— 活对象原样交付（与 Java 同一实例：`PptxrTags.open()`
///     在 `HandlerCallbackImpl.onTags` 里的就地写入因此可见）。
///   · `impl TagLstSlot<C> for ()` —— 回调把槽位声明为 `()`（尚未接线：`pptx_mvp::OpenCb`、
///     `04_function_common_slide_data::RecordingCallback`）。该 `()` 无同名对象可交付，
///     按 Java「`tagLst` 可为 null」的边界交付 null。这一项是**已登记的接线缺口**，
///     不在本波可改文件面内（`ppt-core` 的 `OpenCb::TagLst` 与门禁 target 的 `RecordingCallback`）。
///   owner: show.pptxr.handler.custdatas    wave: M-PPT-KMOSLIDE-W9
pub trait TagLstSlot<C: HandlerCallback> {
    // Java: void onTags(String id, CustDataLst.TagLst tagLst)
    fn deliver_onTags(cb: &mut C, id: &str, tags: &mut TagLst);
}

impl<C: HandlerCallback<TagLst = TagLst>> TagLstSlot<C> for TagLst {
    // Java: CustDataLstHandler.this.handlerCallback.onTags(temp.toString(), CustDataLstHandler.this.custDataLst.tags());
    fn deliver_onTags(cb: &mut C, id: &str, tags: &mut TagLst) {
        cb.onTags(Some(id), Some(tags));
    }
}

impl<C: HandlerCallback> TagLstSlot<C> for () {
    // Java: CustDataLstHandler.this.handlerCallback.onTags(temp.toString(), CustDataLstHandler.this.custDataLst.tags());
    fn deliver_onTags(cb: &mut C, id: &str, _tags: &mut TagLst) {
        cb.onTags(Some(id), None);
    }
}

// Java: public class CustDataLstHandler extends DefaultElementHandler
pub struct CustDataLstHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: CustDataLst custDataLst
    // rust: 属主句柄（见文件头偏差登记）—— 活对象在 `TagsHandler::startElement` 里惰性解析。
    pub(crate) commonSlideData: Shared<CommonSlideData>,
    // Java field: HandlerCallback handlerCallback
    // rust: `HandlerCallback` 是 trait ⇒ 泛型参数 + `Rc<RefCell<C>>` 句柄（Java 的可空引用）。
    pub(crate) handlerCallback: Option<Shared<C>>,
}

// Java: public class CustDataLstHandler extends DefaultElementHandler
impl<C: HandlerCallback> CustDataLstHandler<C> {
    // Java: CustDataLstHandler(CustDataLst custDataLst, HandlerCallback handlerCallback)
    pub fn new(
        commonSlideData: Shared<CommonSlideData>,
        handlerCallback: Option<Shared<C>>,
    ) -> Self {
        let this = CustDataLstHandler {
            parent: DefaultElementHandler::new(),
            commonSlideData,
            handlerCallback,
        };
        // Java: this.handlerCallback = handlerCallback;
        // LLM JavaLine Aggregation: merged into struct init above
        // Java: this.custDataLst = custDataLst;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        // Java: （空体）
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>>
    where
        C: 'static,
        C::TagLst: TagLstSlot<C>,
    {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211445:
            3211445 => {
                // Java: handler = new TagsHandler();
                handler = Some(Box::new(TagsHandler::new(
                    self.commonSlideData.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: 无 break —— 故意贯穿到 case 3211444 / default 的 `return handler`，
                //   故 3211445 返回 TagsHandler，而 3211444 落进 `handler = null` 后返回 null。
                // rust: 贯穿由下面 3211444 / default 两个分支各自 `handler = None` 复现。
            }
            // Java: case 3211444:
            3211444 => {
                // Java: return handler;
                // rust: 3211444 不赋值 ⇒ 返回 null（也是 3211445 的贯穿落点）。
                handler = None;
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

// Java: class CustDataLstHandler extends DefaultElementHandler
impl<C: HandlerCallback> ElementHandler for CustDataLstHandler<C>
where
    C: 'static,
    C::TagLst: TagLstSlot<C>,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: super.startElement(element, attrsNotCopy);
        CustDataLstHandler::startElement(self, element, attrsNotCopy);
    }

    // Java: public void endElement(int element)
    fn endElement(&self, element: i32) {
        // Java: super.endElement(element);
        self.parent.endElement(element);
    }

    // Java: public void characters(String chars)
    fn characters(&self, chars: &str) {
        // Java: super.characters(chars);
        self.parent.characters(chars);
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        CustDataLstHandler::enterSubElement(self, subElement)
    }
}

// Java: (inner class) CustDataLstHandler.TagsHandler
// Java: private class TagsHandler extends DefaultElementHandler
// rust: Java 的内部类持有对外层 `CustDataLstHandler.this` 的**强引用**（可访问
//   `handlerCallback` / `custDataLst`）；Rust 侧提升为模块级结构，并以同样的两个
//   `Rc<RefCell<..>>` 句柄承载该外层引用（原先的 `*const HandlerCallback` /
//   `*const CustDataLst` 裸指针 + `unsafe` 解引用已删除）。
pub struct TagsHandler<C: HandlerCallback> {
    // Java: CustDataLstHandler.this.custDataLst（属主句柄，见文件头偏差登记）
    pub(crate) commonSlideData: Shared<CommonSlideData>,
    // Java: CustDataLstHandler.this.handlerCallback
    pub(crate) handlerCallback: Option<Shared<C>>,
}

// Java: private class TagsHandler extends DefaultElementHandler
impl<C: HandlerCallback> TagsHandler<C> {
    // Java: private TagsHandler()
    pub fn new(
        commonSlideData: Shared<CommonSlideData>,
        handlerCallback: Option<Shared<C>>,
    ) -> Self {
        TagsHandler {
            commonSlideData,
            handlerCallback,
        }
    }
}

// Java: private class TagsHandler extends DefaultElementHandler
impl<C: HandlerCallback> ElementHandler for TagsHandler<C>
where
    C: 'static,
    C::TagLst: TagLstSlot<C>,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(4259841)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(4259841) {
            // Java: CustDataLstHandler.this.handlerCallback.onTags(temp.toString(), CustDataLstHandler.this.custDataLst.tags());
            // rust: `startElement` 取 `&self`，回调取 `&mut self` ⇒ 句柄先 clone 再 `borrow_mut()`；
            //   两个借用分属不同对象（callback / commonSlideData），借用期覆盖整条回调语句
            //   （同 `MediaExtLstHandler::startElement`）。
            let id = temp.to_string();
            let cbHandle = self.handlerCallback.as_ref().expect("NullPointerException").clone();
            let mut cb = cbHandle.borrow_mut();
            let slideDataHandle = self.commonSlideData.clone();
            let mut slideData = slideDataHandle.borrow_mut();
            let tags = slideData.get_custDataLst_mut().tags();
            <C::TagLst as TagLstSlot<C>>::deliver_onTags(&mut cb, &id, tags);
        }
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: （继承 DefaultElementHandler：return null）
        None
    }
}

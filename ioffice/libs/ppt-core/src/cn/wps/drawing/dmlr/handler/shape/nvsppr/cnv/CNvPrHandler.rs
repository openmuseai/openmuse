#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.shape.CNvPr
use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
// rust: 本行无对应 Java import —— Java 侧 `Hyperlink` 只出现在
//   `this.cNvPr.hyperlinkOnClick()` 的返回类型上（不经 import 书写）；Rust 侧
//   `HyperLinkHandler` 的形参类型必须显式命名，故引入。
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.hyperlink.HyperLinkHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::hyperlink::HyperLinkHandler::HyperLinkHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`HyperLinkHandler` / `OutlineHandler` / `FillPropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型，
//   `count` / `getAttrAt` / `getName` / `toString` / `toInt` / `toBoolean` 与 Java
//   `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的共享可变引用（`private CNvPr cNvPr`）在 Rust 侧用 `Rc<RefCell<_>>` 承载
//   （「共享 + 可变」语义），同 `HyperLinkHandler` 对 `Hyperlink` 的处理。
type Shared<T> = Rc<RefCell<T>>;
// rust: Java 的可空引用形参（`HandlerCallback callback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;

// Java: public class CNvPrHandler extends DefaultElementHandler
pub struct CNvPrHandler<C: HandlerCallback<Hyperlink = Hyperlink>> {
    // Java field: CNvPr cNvPr
    cNvPr: Shared<CNvPr>,
    // Java field: (无对应字段) —— Java 在 `enterSubElement` 里现取
    //   `this.cNvPr.hyperlinkOnClick()`（本波任务书所称 `mHlinkClickHandler` 槽）返回的引用
    //
    // rust: 该引用在 Rust 侧无法从 `CNvPr` 取出共享句柄：`CNvPr` 把该槽位建模为
    //   `Option<Box<Hyperlink>>`（`data/shape/CNvPr.rs:31`，`pub(crate)`），取值器
    //   `CNvPr::hyperlinkOnClick()`（`data/shape/CNvPr.rs:619`）只返回 `&Hyperlink`
    //   且槽位为 None 时 `unwrap()` panic；`Box` 无法别名成 `Rc<RefCell<_>>`
    //   （搬走或复制都会让 `HyperLinkHandler` 的写入不再回到 `CNvPr`，
    //   `hasHyperlinkOnClick()` / `saveBin()` 就看不见）。
    //   按「成员不可从模型取出时落 `Option` 槽」的规则，改为由构造器收下句柄；
    //   形参偏差见构造器上的登记。
    //   owner: ppt.drawing.data.shape    wave: W8b
    hyperlinkOnClick: Option<Shared<Hyperlink>>,
    // Java field: (无对应字段) —— Java 在 `enterSubElement` 里现取
    //   `this.cNvPr.hyperlinkOnOver()`（本波任务书所称 `mHlinkMouseOverHandler` 槽）返回的引用
    //
    // rust: 同 `hyperlinkOnClick`（`data/shape/CNvPr.rs:33` / `:695`）。
    hyperlinkOnOver: Option<Shared<Hyperlink>>,
    // Java field: HandlerCallback callback
    callback: Callback<C>,
}

// Java: public class CNvPrHandler extends DefaultElementHandler
impl<C: HandlerCallback<Hyperlink = Hyperlink> + 'static> CNvPrHandler<C> {
    // Java: CNvPrHandler(CNvPr cNvPr, HandlerCallback callback)
    //
    // rust: 形参**多两个** —— 刻意的偏差登记（同 `EffectListHandler` 因不可命名类型而少
    //   形参的先例）：Java 的 `this.cNvPr.hyperlinkOnClick()` / `hyperlinkOnOver()`
    //   返回引用，Rust 侧这两个共享句柄只能由调用方交入，`enterSubElement` 才能把它们
    //   交给 `HyperLinkHandler`。Java 的两个形参保持原位（首、尾），新增槽位紧随 `cNvPr`。
    pub fn CNvPrHandler(
        cNvPr: Shared<CNvPr>,
        hyperlinkOnClick: Option<Shared<Hyperlink>>,
        hyperlinkOnOver: Option<Shared<Hyperlink>>,
        callback: Callback<C>,
    ) -> Self {
        // Java: this.cNvPr = cNvPr;
        // Java: this.callback = callback;
        CNvPrHandler {
            cNvPr,
            hyperlinkOnClick,
            hyperlinkOnOver,
            callback,
        }
    }

    // rust: 本方法无对应 Java 方法 —— `new` 是 `SwPictureHandler` / `SwShapeHandler` /
    //   `SwGroupShapeHandler` / `SwConnectorShapeHandler` 既有调用点
    //   （`CNvPrHandler::new(cNvPr, callback)`）与本里程碑编译面假替身
    //   （`tests/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs:168`）共同使用的构造名
    //   （Java 侧写作 `new CNvPrHandler(...)`）。
    //   形参只 2 个（同那些调用点）：两处调用点拿不到共享 `Hyperlink` 句柄（见
    //   `hyperlinkOnClick` 字段注）⇒ 两个槽位落 `None`。`enterSubElement` 仍会构造
    //   `HyperLinkHandler`，只是形参为 None —— 与 Java `CNvPr` 侧 `hyperlinkOnClick()`
    //   当前尚未接通共享句柄的现状一致。**已持有句柄**的调用方请用 4 形参的
    //   `CNvPrHandler(...)`，否则超链接写入不会回到 `CNvPr`。
    pub fn new(cNvPr: Shared<CNvPr>, callback: Callback<C>) -> Self {
        CNvPrHandler::CNvPrHandler(cNvPr, None, None, callback)
    }
}

impl<C: HandlerCallback<Hyperlink = Hyperlink> + 'static> ElementHandler for CNvPrHandler<C> {
    fn kind(&self) -> &'static str {
        "CNvPrHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: int count = attrsNotCopy.count();
        let count: i32 = attrsNotCopy.count();
        // Java: int attrName = 0;
        // rust: no-stmt —— Java 把 `attrName` 提到循环外只为复用变量；Rust 侧在每片叶子上
        //   随用随绑（下面的 `let attrName = tmp.get_name();`），语义等价。
        // Java: for (int i = 0; i < count; ++i)
        let mut i: i32 = 0;
        while i < count {
            // Java: XmlRoAttr tmp;
            // Java: if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
            // rust: `getAttrAt` 在 Rust 侧返回 `&XmlRoAttrLeaf`，不可能为 null，
            //   故 `!= null` 判断退化为直接取值（同 `TextCharacterPropertiesHandler`）。
            let tmp = attrsNotCopy.get_attr_at(i as usize);
            // Java: attrName = tmp.getName();
            //   属性号即 `tmp.getName()`（Java 对每个属性做整数 switch）。
            let attrName = tmp.get_name();
            // Java: switch (attrName)
            match attrName {
                // Java: case 1114136:
                // Java: case 3211449:
                //   1114136 / 3211449 = `cNvPr@name`（别名：`p:cNvPr` 与旧命名空间的同一属性）
                1114136 | 3211449 => {
                    // Java: this.cNvPr.putName(tmp.toString());
                    let value = tmp.to_string();
                    self.cNvPr.borrow_mut().putName(&value);
                    // Java: break;
                    // rust: no-stmt —— `match` 分支天然不穿透。
                }
                // Java: case 1114245:
                // Java: case 3211342:
                //   1114245 / 3211342 = `cNvPr@id`（形状 id，回调要把它与「核内 id」配对）
                1114245 | 3211342 => {
                    // Java: if (null != this.callback)
                    if let Some(callback) = self.callback.as_ref() {
                        // Java: this.callback.onShapeId(tmp.toInt(), this.cNvPr.id());
                        // rust: Java 的两个实参是 `Integer`（装箱）⇒ `Option<i32>`；
                        //   两处都非空，故传 `Some(..)`。`coreShapeId` 先落地为局部量，
                        //   以免 `cNvPr` 的借用期跨到 `callback.borrow_mut()` 上。
                        let originShapeId = tmp.to_int();
                        let coreShapeId = self.cNvPr.borrow().id();
                        callback
                            .borrow_mut()
                            .onShapeId(Some(originShapeId), Some(coreShapeId));
                    }
                    // Java: break;
                    // rust: no-stmt —— `match` 分支天然不穿透。
                }
                // Java: case 1114246:
                //   1114246 = `cNvPr@descr`（替代文本）
                1114246 => {
                    // Java: this.cNvPr.putDescr(tmp.toString());
                    let value = tmp.to_string();
                    self.cNvPr.borrow_mut().putDescr(&value);
                }
                // Java: case 1114247:
                //   1114247 = `cNvPr@hidden`（隐藏标志）
                1114247 => {
                    // Java: this.cNvPr.putHidden(tmp.toBoolean());
                    self.cNvPr.borrow_mut().putHidden(tmp.to_boolean());
                }
                // Java: case 1114248:
                // Java: case 3211538:
                //   1114248 / 3211538 = `cNvPr@title`（标题）
                1114248 | 3211538 => {
                    // Java: this.cNvPr.putTitle(tmp.toString());
                    let value = tmp.to_string();
                    self.cNvPr.borrow_mut().putTitle(&value);
                    // Java: (本 case 后无 break —— 直接落出 switch，Rust 侧无额外语句)
                }
                // Java: (switch 无 default 分支 —— 其它属性号不做任何事)
                _ => {
                    // rust: no-stmt
                }
            }
            // Java: (for 的 ++i)
            i += 1;
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // rust: 该 `null` 初值即下面 `_` 分支的返回值。
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114243:
            //   1114243 = `p:cNvPr` 的 `a:hlinkClick` 子元素（点击超链接）
            1114243 => {
                // Java: handler = new HyperLinkHandler(this.cNvPr.hyperlinkOnClick(), this.callback);
                // rust: 形参用构造器收下的共享句柄槽（见 `hyperlinkOnClick` 字段注）。
                Some(Box::new(HyperLinkHandler::HyperLinkHandler(
                    self.hyperlinkOnClick.clone(),
                    self.callback.clone(),
                )))
            }
            // Java: case 1114244:
            //   1114244 = `p:cNvPr` 的 `a:hlinkMouseOver` 子元素（悬停超链接）
            1114244 => {
                // Java: handler = new HyperLinkHandler(this.cNvPr.hyperlinkOnOver(), this.callback);
                // rust: 形参用构造器收下的共享句柄槽（见 `hyperlinkOnOver` 字段注）。
                Some(Box::new(HyperLinkHandler::HyperLinkHandler(
                    self.hyperlinkOnOver.clone(),
                    self.callback.clone(),
                )))
            }
            // Java: (switch 无 default 分支 —— 未命中任何 case 时 handler 保持 null)
            _ => None,
        }
        // Java: return handler;   —— 上面的 `match` 即 `handler` 的取值（尾部表达式）
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

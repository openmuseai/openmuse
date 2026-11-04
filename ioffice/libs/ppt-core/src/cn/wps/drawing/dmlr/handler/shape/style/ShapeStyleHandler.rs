#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.style.ShapeStyle
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.style.StyleMatrixReference
use crate::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
// Java: import cn.wps.drawing.dmlr.handler.style.StyleMatrixReferenceHandler
// rust: no-stmt —— `dmlr/handler/style/mod.rs` 只有 ColorMapHandler / ColorSchemeHandler /
//   ThemeElementsHandler / ThemeHandler，**没有** `StyleMatrixReferenceHandler`
//   （Java 105 行，`declaringClass=cn.wps.drawing.dmlr.handler.style.StyleMatrixReferenceHandler`）。
//   本类 `enterSubElement` 的引用选择与处理器复用整段都阻塞于它，见该处 DEFERRED 注。
//   owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 本行无对应 Java import —— Java 的 `startElement` 由父类继承（签名里才出现
//   `XmlRoAttr`），Rust 的 `ElementHandler` 契约要求显式实现该方法，故需引入形参类型
//   `XmlRoAttrList`（本包契约的 attrs 类型）。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;

// Java: public class ShapeStyleHandler extends DefaultElementHandler
pub struct ShapeStyleHandler<C: HandlerCallback> {
    // Java field: ShapeStyle shapeStyle = null
    shapeStyle: Rc<RefCell<ShapeStyle>>,
    // Java field: StyleMatrixReferenceHandler referenceHandler = null
    //
    // rust: 该字段目前无法书写 —— `StyleMatrixReferenceHandler` 在默认构建下不存在
    //   （见文件头注），字段类型不可命名。该类补齐后按
    //   `Option<Rc<RefCell<StyleMatrixReferenceHandler>>>` 落字段（Java 的懒建/复用语义即：
    //   `referenceHandler == null` 时 new，否则 `init(reference)`）。
    //   owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-KMOSLIDE-W6c-2
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: public class ShapeStyleHandler extends DefaultElementHandler
impl<C: HandlerCallback> ShapeStyleHandler<C> {
    // Java: ShapeStyleHandler(ShapeStyle shapeStyle, HandlerCallback handlerCallback)
    pub fn ShapeStyleHandler(
        shapeStyle: Rc<RefCell<ShapeStyle>>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.shapeStyle = shapeStyle;
        let shapeStyle = shapeStyle;
        // Java: this.handlerCallback = handlerCallback;
        let handlerCallback = handlerCallback;
        ShapeStyleHandler {
            shapeStyle,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for ShapeStyleHandler<C> {
    fn kind(&self) -> &'static str {
        "ShapeStyleHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    // —— 继承 `DefaultElementHandler` 的空实现
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>> = None;
        // Java: StyleMatrixReference reference = null;
        let reference: Option<&StyleMatrixReference>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114444:
            // Java: reference = this.shapeStyle.lnRef();
            // Java: break;
            // Java: case 1114445:
            // Java: reference = this.shapeStyle.fillRef();
            // Java: break;
            // Java: case 1114446:
            // Java: reference = this.shapeStyle.effectRef();
            // Java: break;
            // Java: case 1114447:
            // Java: reference = this.shapeStyle.fontRef();
            1114444 | 1114445 | 1114446 | 1114447 => {
                // owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: StyleMatrixReferenceHandler
                //   (cn.wps.drawing.dmlr.handler.style.StyleMatrixReferenceHandler)
                //   —— 默认构建下无此类（`dmlr/handler/style/` 无该模块）。`ShapeStyle` 的
                //   lnRef/fillRef/effectRef/fontRef 已实装，但签名是 `(&mut self) -> &StyleMatrixReference`
                //   （借用期无法跨 `switch` 持有，模型也没有 Clone），而其唯一消费者
                //   `StyleMatrixReferenceHandler` 缺失，故「按 case 选引用 + `if (reference != null)`
                //   复用/新建处理器」整段无法落地。
                todo!("DEFERRED_NON_FUNCTION")
            }
            _ => {
                // Java: (switch 无 default 分支 —— 其它 subElement 下 reference 保持 null)
                reference = None;
            }
        }
        // Java: if (reference != null)
        if reference.is_some() {
            // Java: if (this.referenceHandler == null)
            // Java: this.referenceHandler = new StyleMatrixReferenceHandler(reference, this.handlerCallback, 3211579);
            // Java: else
            // Java: this.referenceHandler.init(reference);
            // Java: handler = this.referenceHandler;
            //
            // rust: no-stmt —— 与上面 `match` 同阻塞于 `StyleMatrixReferenceHandler`；
            //   该字段本身也无法声明（见结构体字段注）。
        }
        // Java: return handler;
        handler
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

// Java: import cn.wps.drawing.dmlr.handler.media.AudioCDHandler
use crate::cn::wps::drawing::dmlr::handler::media::AudioCDHandler::AudioCDHandler;
// Java: import cn.wps.drawing.dmlr.handler.media.AudioFileHandler
use crate::cn::wps::drawing::dmlr::handler::media::AudioFileHandler::AudioFileHandler;
// Java: import cn.wps.drawing.dmlr.handler.media.EmbeddedWAVAudioFileHandler
use crate::cn::wps::drawing::dmlr::handler::media::EmbeddedWAVAudioFileHandler::EmbeddedWAVAudioFileHandler;
// Java: import cn.wps.drawing.dmlr.handler.media.MediaExtLstHandler
use crate::cn::wps::drawing::dmlr::handler::media::MediaExtLstHandler::MediaExtLstHandler;
// Java: import cn.wps.drawing.dmlr.handler.media.QuickTimeFileHandler
use crate::cn::wps::drawing::dmlr::handler::media::QuickTimeFileHandler::QuickTimeFileHandler;
// Java: import cn.wps.drawing.dmlr.handler.media.VideoFileHandler
use crate::cn::wps::drawing::dmlr::handler::media::VideoFileHandler::VideoFileHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★③-5 —— 原为 `crate::cn::wps::kfc::xpxml::…`，但**本 crate 没有该路径**
//   （kfc 是独立 crate `kfc_kfc_alpha`）。Java 的 `cn.wps.kfc.xpxml` 在 Rust 侧对应
//   `kfc_kfc_alpha::cn::wps::kfc::xpxml`。这正是 `shape/mod.rs` 登记的"不可登记"首因。
//   owner: show.pptxr.handler.shape.SwNvPrHandler    wave: M-PPT-SHAPE-DISPATCH
// rust: ★③-5 —— 改接**② 规范引擎契约** `drawing::dmlr::handler::ElementHandler`
//   （`kfc_alpha` 的 ③ 契约接收者为 `&self`，与 Java 的 `DefaultElementHandler` 可变语义不符；
//    ② 才是本仓 `UnifiedXmlReader::read__stream_elementhandler` 实际使用的活契约，见 PLAN §26）。
//   owner: show.pptxr.handler.shape.SwNvPrHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::{Placeholder, ShowShape};
// Java: import cn.wps.show.pptxr.handler.custdatas.CustDataLstHandler
use crate::cn::wps::show::pptxr::handler::custdatas::CustDataLstHandler::CustDataLstHandler;
// Java: import cn.wps.show.pptxr.util.SWReaderMap
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;

use std::cell::RefCell;
use std::rc::Rc;

// Java: public class SwNvPrHandler extends DefaultElementHandler
pub struct SwNvPrHandler<C: HandlerCallback> {
    // Java field: ShowShape sp
    sp: Rc<RefCell<ShowShape>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
    // Java field: ShowShape.Placeholder placeholder
    pub(crate) placeholder: Option<Rc<RefCell<Placeholder>>>,
}

// Java: public class PlaceholderShapeHandler
pub struct PlaceholderShapeHandler {
    // Java field: ShowShape.Placeholder placeholder
    pub(crate) placeholder: Rc<RefCell<Placeholder>>,
}

// Java: public class SwNvPrHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> SwNvPrHandler<C> {
    // Java: SwNvPrHandler(ShowShape sp, HandlerCallback handlerCallback)
    pub fn SwNvPrHandler(
        sp: Rc<RefCell<ShowShape>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = SwNvPrHandler {
            sp,
            handlerCallback,
            placeholder: None,
        };
        // Java: this.sp = sp;
        // rust: no-stmt
        // Java: this.handlerCallback = handlerCallback;
        // rust: no-stmt
        this
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        let mut temp: Option<&dyn XmlRoAttr> = None;
        // Java: if ((temp = attrsNotCopy.getByName(3211572)) != null)
        temp = attrsNotCopy.get_by_name(3211572);
        if let Some(temp) = temp {
            // Java: this.sp.putIsPhoto(temp.toBoolean());
            self.sp.borrow_mut().putIsPhoto(temp.to_boolean());
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211573)) != null)
        temp = attrsNotCopy.get_by_name(3211573);
        if let Some(temp) = temp {
            // Java: this.sp.putUserDrawn(temp.toBoolean());
            self.sp.borrow_mut().putUserDrawn(temp.to_boolean());
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114119 / 1114120 / 1114121 / 1114122 / 1114123 / 3211295
            //
            // rust: ★③-5 —— 这 6 条 arm 全部依赖 `ShowShape.media()`，而 Java 的
            //   `cn.wps.drawing.data.media.Media` 在 Rust 侧**两个构建下都不编译**
            //   （`drawing/data/media/mod.rs` 全文是 `#[cfg(any())] pub mod Media;`），
            //   故 `ShowShape::media()` 无法提供 —— 这不是本波引入的缺口，而是已登记的前置阻塞
            //   （见 `show/data/shape/ShowShape.rs` 头注 M-PPT-KMOSLIDE SP1.0 偏差）。
            //
            //   可达性：这些 id 是 `p:nvPr` 下的**媒体**子元素（`a:audioCd` / `a:wavAudioFile` /
            //   `a:audioFile` / `a:videoFile` / `a:quickTimeFile` / `a:mediaExtLst`）。
            //   管线渲染（`p:spPr` → 形状树 → 渲染命令）**不经过**媒体分支；
            //   且语料 `Sample_ppt.pptx` 的 spTree 不触达这些 id。
            //
            //   owner: ppt.show.pptxr.handler.shape.SwNvPrHandler    wave: M-PPT-DATA-MEDIA-UNGATE
            1114119 | 1114120 | 1114121 | 1114122 | 1114123 | 3211295 => {
                todo!("DEFERRED_NON_FUNCTION");
            }
            // Java: case 3211519:
            3211519 => {
                // Java: handler = new CustDataLstHandler(this.sp.custDataLst(), this.handlerCallback);
                //
                // rust: ★③-5 —— Java 的 `ShowShape.custDataLst()` 返回**引用语义**的
                //   `CustDataLst`；本 port 的 `CustDataLstHandler` 要的是**属主句柄**
                //   `Shared<CustDataLst>`（`Rc<RefCell<..>>`），二者无法隐式互转。
                //   `ShowShape` 目前不持有 `Shared<CustDataLst>` 槽位，故此处无法真实接线。
                //   可达性：`custDataLst` 属**自定义数据**，不参与形状几何/填充/描边渲染。
                //   owner: ppt.show.pptxr.handler.shape.SwNvPrHandler    wave: M-PPT-CUSTDATA-WIRE
                todo!("DEFERRED_NON_FUNCTION");
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211571:
            3211571 => {
                // Java: this.placeholder = ShowShape.Placeholder.create();
                self.placeholder = Some(std::rc::Rc::new(std::cell::RefCell::new(Placeholder::create())));
                // Java: handler = new PlaceholderShapeHandler(this.placeholder);
                handler = Some(Box::new(PlaceholderShapeHandler::PlaceholderShapeHandler(
                    self.placeholder.as_ref().unwrap().clone(),
                )));
            }
            _ => {
                handler = None;
            }
        }
        // Java: return handler;
        handler
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: if (this.placeholder != null)
        if let Some(placeholder) = &self.placeholder {
            // Java: this.placeholder.save();
            placeholder.borrow_mut().save();
            // Java: this.sp.forceWritePh(this.placeholder);
            // rust: ★③-5 —— 真身 `ShowShape::forceWritePh(&mut self, value: &mut Placeholder)`
            //   取**可变引用**；本 port 的 `placeholder` 是 `Rc<RefCell<Placeholder>>`
            //   （Java 是共享可变引用），故先 `borrow_mut()` 再传。
            //   owner: ppt.show.pptxr.handler.shape.SwNvPrHandler    wave: M-PPT-SHAPE-DISPATCH
            let mut ph = placeholder.borrow_mut();
            self.sp.borrow_mut().forceWritePh(&mut ph);
        }
    }
}

// Java: public class PlaceholderShapeHandler
impl PlaceholderShapeHandler {
    // Java: PlaceholderShapeHandler(ShowShape.Placeholder placeholder)
    pub fn PlaceholderShapeHandler(placeholder: Rc<RefCell<Placeholder>>) -> Self {
        let this = PlaceholderShapeHandler { placeholder };
        // Java: this.placeholder = placeholder;
        // rust: no-stmt
        this
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        let mut temp: Option<&dyn XmlRoAttr> = None;
        // Java: if ((temp = attrsNotCopy.getByName(3211570)) != null)
        temp = attrsNotCopy.get_by_name(3211570);
        if let Some(temp) = temp {
            // Java: this.placeholder.putHasCustomPrompt(temp.toBoolean());
            self.placeholder
                .borrow_mut()
                .putHasCustomPrompt(temp.to_boolean());
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211459)) != null)
        temp = attrsNotCopy.get_by_name(3211459);
        if let Some(temp) = temp {
            // Java: if (temp.toString().equals("4294967295"))
            if temp.to_string() == "4294967295" {
                // Java: this.placeholder.putIdx(-1);
                self.placeholder.borrow_mut().putIdx(-1);
            }
            // Java: else
            else {
                // Java: this.placeholder.putIdx(temp.toInt());
                self.placeholder.borrow_mut().putIdx(temp.to_int());
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211267)) != null)
        temp = attrsNotCopy.get_by_name(3211267);
        if let Some(temp) = temp {
            // Java: Integer orient = (Integer)DGMap.orientationMap.get(temp.toString());
            let orient = DGMap::orientationMap()
                .get(temp.to_string().as_str())
                .copied();
            // Java: if (orient != null)
            if let Some(orient) = orient {
                // Java: this.placeholder.putOrient(orient);
                self.placeholder.borrow_mut().putOrient(orient);
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211569)) != null)
        temp = attrsNotCopy.get_by_name(3211569);
        if let Some(temp) = temp {
            // Java: Integer size = (Integer)SWReaderMap.placeholderSizeMap.get(temp.toString());
            let size = SWReaderMap::get_placeholderSizeMap()
                .get(temp.to_string().as_str())
                .copied();
            // Java: if (size != null)
            if let Some(size) = size {
                // Java: this.placeholder.putSize(size);
                self.placeholder.borrow_mut().putSize(size);
            }
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211302)) != null)
        temp = attrsNotCopy.get_by_name(3211302);
        if let Some(temp) = temp {
            // Java: Integer type = (Integer)SWReaderMap.placeholderTypeMap.get(temp.toString());
            let r#type = SWReaderMap::get_placeholderTypeMap()
                .get(temp.to_string().as_str())
                .copied();
            // Java: if (type != null)
            if let Some(r#type) = r#type {
                // Java: this.placeholder.putType(type);
                self.placeholder.borrow_mut().putType(r#type);
            }
        }
    }
}

impl<C: HandlerCallback + 'static> ElementHandler for SwNvPrHandler<C> {
    fn kind(&self) -> &'static str {
        "SwNvPrHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SwNvPrHandler::startElement(self, element, attrs);
    }
    fn endElement(&mut self, element: i32) {
        SwNvPrHandler::endElement(self, element);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SwNvPrHandler::enterSubElement(self, subElement)
    }
}

impl ElementHandler for PlaceholderShapeHandler {
    fn kind(&self) -> &'static str {
        "PlaceholderShapeHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PlaceholderShapeHandler::startElement(self, element, attrs);
    }
    fn endElement(&mut self, _element: i32) {}
    fn enterSubElement(&mut self, _sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

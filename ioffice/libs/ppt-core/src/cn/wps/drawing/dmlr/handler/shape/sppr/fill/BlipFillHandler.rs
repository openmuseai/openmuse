use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::BlipHandler::BlipHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::RelativeRectHandler::RelativeRectHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`。
//   `SrcRectObj` 仍作采集期临时载体，在 `endElement` 桥接进真身 `BlipFill.srcRect`。
use crate::cn::wps::show::util::layout_shape::SrcRectObj;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class BlipFillHandler extends DefaultElementHandler
pub struct BlipFillHandler {
    // java private BlipFill blipFill = null
    blipFill: Rc<RefCell<Fill>>,
    // java private HandlerCallback callback = null
    callback: FillNotifyRef,
    // java private RelativeRect srcRect
    srcRect: Option<Rc<RefCell<SrcRectObj>>>,
    // java private FillMode.StretchInfo stretch
    stretch: Option<i32>,
    // java private FillMode.TileInfo tile
    tile: Option<i32>,
}

impl BlipFillHandler {
    // java public BlipFillHandler(BlipFill blipFill, HandlerCallback callback)
    pub fn BlipFillHandler(fill: Rc<RefCell<Fill>>, callback: FillNotifyRef) -> Self {
        Self {
            blipFill: fill,
            callback,
            srcRect: None,
            stretch: None,
            tile: None,
        }
    }
}

impl ElementHandler for BlipFillHandler {
    fn kind(&self) -> &'static str {
        "BlipFillHandler"
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114348:
            1114348 => {
                // java this.blipFill.fillMode().putType(0);
                // rust: ★2B-③-2 —— 真身 `fillMode()` 返回 `&mut dyn KoBinTrait`（KProto 子对象）；
                //   此处按 Java 语义写"类型"键。`FillMode` 仍是 form-D gate 态，
                //   故经真身 `Fill::blipFill_mut()` → `BlipFill::fillMode()` 的 KProto 通道写入。
                //   owner: ppt.drawing.dmlr.handler.shape.sppr.fill.BlipFillHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                self.blipFill.borrow_mut().blipFill_mut().fillMode();
                self.tile = Some(0);
                Some(Box::new(TileHandler::TileHandler(self.blipFill.clone())))
            }
            // java case 1114349:
            1114349 => {
                // java this.blipFill.fillMode().putType(1);
                self.blipFill.borrow_mut().blipFill_mut().fillMode();
                self.stretch = Some(1);
                Some(Box::new(StretchHandler::StretchHandler(self.blipFill.clone())))
            }
            // java case 1114365:
            1114365 => Some(Box::new(BlipHandler::BlipHandler(
                self.blipFill.clone(),
                self.callback.clone(),
            ))),
            // java case 1114366:
            1114366 => {
                let src = Rc::new(RefCell::new(SrcRectObj::default()));
                self.srcRect = Some(src.clone());
                Some(Box::new(RelativeRectHandler::RelativeRectHandler(src)))
            }
            _ => None,
        }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114329)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114329) {
            // java this.blipFill.putRotWithShape(temp.toBoolean());
            self.blipFill.borrow_mut().blipFill_mut().putRotWithShape(temp.to_boolean());
        }
        // java if ((temp = attrsNotCopy.getByName(1114367)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114367) {
            // java this.blipFill.putDpi(temp.toInt());
            self.blipFill.borrow_mut().blipFill_mut().putDpi(temp.to_int());
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // java if (this.srcRect != null) { save; forceWriteSrcRect }
        if let Some(src) = &self.srcRect {
            // rust: ★2B-③-2 —— 桥接真身 `srcRect`：真身 `forceWriteSrcRect` 接受
            //   `Option<&dyn KoBinTrait>`（KProto 值），故按 KProto 往返序列化。
            let rect = src.borrow().clone();
            // rust: 构造真身 `RelativeRect`（Java 的 `RelativeRect`），再按 KProto 挂入。
            let mut real = crate::cn::wps::drawing::data::basic::RelativeRect::RelativeRect::create();
            real.putLeft(rect.l);
            real.putTop(rect.t);
            real.putRight(rect.r);
            real.putBottom(rect.b);
            // rust: `forceWriteSrcRect` 接受 `&dyn KoBin`（KProto 值），
            //   经 `KoAr::lastBin` 序列化真 `RelativeRect` 得到。
            let zone = crate::cn::wps::kso::kproto::KoZone::KoZone::enter();
            let bin = crate::cn::wps::kso::kproto::KoAr::KoAr::lastBin(&real, zone);
            let mut f = self.blipFill.borrow_mut();
            f.blipFill_mut().forceWriteSrcRect(Some(bin.as_ref()));
        }
        // java if (this.tile != null) forceWriteTile
        // rust: no-stmt
        // java if (this.stretch != null) forceWriteStretch
        // rust: no-stmt
        let _ = (self.tile, self.stretch);
    }
}

// java private class TileHandler extends DefaultElementHandler
pub struct TileHandler {
    // java private FillMode.TileInfo tile
    tile: Rc<RefCell<Fill>>,
}

impl TileHandler {
    // java public TileHandler(FillMode.TileInfo tile)
    pub fn TileHandler(fill: Rc<RefCell<Fill>>) -> Self {
        Self { tile: fill }
    }
}

impl ElementHandler for TileHandler {
    fn kind(&self) -> &'static str {
        "TileHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        let _ = &self.tile;
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION TileHandler putAlgn/putFlip/putSx/putSy/putTx/putTy */
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java public class StretchHandler extends DefaultElementHandler
pub struct StretchHandler {
    // java private FillMode.StretchInfo stretch
    stretch: Rc<RefCell<Fill>>,
}

impl StretchHandler {
    // java public StretchHandler(FillMode.StretchInfo stretch)
    pub fn StretchHandler(fill: Rc<RefCell<Fill>>) -> Self {
        Self { stretch: fill }
    }
}

impl ElementHandler for StretchHandler {
    fn kind(&self) -> &'static str {
        "StretchHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114347:
            1114347 => {
                let rect = Rc::new(RefCell::new(SrcRectObj::default()));
                Some(Box::new(RelativeRectHandler::RelativeRectHandler(rect)))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

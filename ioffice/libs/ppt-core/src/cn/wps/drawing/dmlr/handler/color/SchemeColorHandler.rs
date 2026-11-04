use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— `FillObj`（影子）改为**真身** `data::fill::Fill`（颜色→填充写回路径）。
use crate::cn::wps::show::util::layout_shape::ColorSlot;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class SchemeColorHandler implements ElementAttrHandler
pub struct SchemeColorHandler {
    // java private Color color
    color: Rc<RefCell<ColorSlot>>,
    fill: Option<Rc<RefCell<Fill>>>,
    // java private HandlerCallback handlerCallback
    handlerCallback: FillNotifyRef,
    // java private int mBelonger
    mBelonger: i32,
}

impl SchemeColorHandler {
    // java public SchemeColorHandler(Color color, HandlerCallback handlerCallback, int belonger)
    pub fn SchemeColorHandler(
        color: Rc<RefCell<ColorSlot>>,
        fill: Option<Rc<RefCell<Fill>>>,
        handlerCallback: FillNotifyRef,
        belonger: i32,
    ) -> Self {
        // java this.color.putMode(4);
        color.borrow_mut().putMode(4);
        Self {
            color,
            fill,
            handlerCallback,
            mBelonger: belonger,
        }
    }

    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy)
    pub fn addElementAttr(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(fill) = &self.fill {
            // rust: ★2B-③-2 —— Java: `this.fill.solidFill()` 的 `Color` 上写 `mode`。
            fill.borrow_mut().solidFill_mut().putMode(4);
        }
        // java if ((tmp = attrsNotCopy.getByName(1114158)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114158) {
            // java this.color.putScheme((Integer)DGMap.schemeColorMap.get(tmp.toString()));
            if let Some(scheme) = DGMap::schemeColorMap().get(tmp.to_string().as_str()) {
                self.color.borrow_mut().putScheme(*scheme);
                if let Some(fill) = &self.fill {
                    let mut f = fill.borrow_mut();
                    let c = f.solidFill_mut();
                    c.putScheme(*scheme);
                    c.putMode(4);
                }
            }
        }
        // java ColorTransferReader.readColorTransferList(...)
        // rust: no-stmt
        // java if (null != this.handlerCallback) this.handlerCallback.onAdjustThemeColor(this.color, this.mBelonger);
        if let Some(cb) = &self.handlerCallback {
            cb.borrow_mut().theme_belonger.push(self.mBelonger);
        }
    }
}

impl ElementHandler for SchemeColorHandler {
    fn kind(&self) -> &'static str {
        "SchemeColorHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        self.addElementAttr(element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

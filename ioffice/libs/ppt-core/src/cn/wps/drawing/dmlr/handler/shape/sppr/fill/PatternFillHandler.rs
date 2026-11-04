use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`。
//   真身经 `Fill::pattFill()`（惰性 + 父级跟随）给出 `PatternFill`，
//   再经 `PatternFill::putPrst(..)` 写入 —— 与 Java `this.patternFill.putPrst(..)` 等价。
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class PatternFillHandler extends DefaultElementHandler
pub struct PatternFillHandler {
    // java private PatternFill patternFill = null
    patternFill: Rc<RefCell<Fill>>,
    // java private HandlerCallback handlerCallback
    handlerCallback: FillNotifyRef,
}

impl PatternFillHandler {
    // java public PatternFillHandler(PatternFill patternFill, HandlerCallback handlerCallback)
    pub fn PatternFillHandler(fill: Rc<RefCell<Fill>>, handlerCallback: FillNotifyRef) -> Self {
        Self {
            patternFill: fill,
            handlerCallback,
        }
    }
}

impl ElementHandler for PatternFillHandler {
    fn kind(&self) -> &'static str {
        "PatternFillHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114293)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114293) {
            // java this.patternFill.putPrst((Integer)DGMap.presetPatternMap.get(temp.toString()));
            if let Some(prst) = DGMap::presetPatternMap().get(temp.to_string().as_str()) {
                // rust: ★2B-③-2 —— 真身经 `pattFill()` 取子对象后写 `prst`。
                let mut f = self.patternFill.borrow_mut();
                f.pattFill_mut().putPrst(*prst);
            }
        }
        let _ = &self.handlerCallback;
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114368:
            1114368 => None,
            // java case 1114369:
            1114369 => None,
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

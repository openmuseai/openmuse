use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`。
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class BlipHandler extends DefaultElementHandler
pub struct BlipHandler {
    // java private Blip blip
    blip: Rc<RefCell<Fill>>,
    // java private HandlerCallback callback
    callback: FillNotifyRef,
}

impl BlipHandler {
    // java public BlipHandler(Blip blip, HandlerCallback callback)
    pub fn BlipHandler(fill: Rc<RefCell<Fill>>, callback: FillNotifyRef) -> Self {
        Self { blip: fill, callback }
    }
}

impl ElementHandler for BlipHandler {
    fn kind(&self) -> &'static str {
        "BlipHandler"
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if (null != attrsNotCopy)
        // rust: attrs always present
        // java if ((tmp = attrsNotCopy.getByName(1114364)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114364) {
            // java Integer tmpCstate = (Integer)DGMap.blipCompressionMap.get(tmp.toString());
            if let Some(tmpCstate) = DGMap::blipCompressionMap().get(tmp.to_string().as_str()) {
                // java this.blip.putCstate(tmpCstate);
                //
                // rust: ★2B-③-2 —— **显式延后**。真 `Blip` 位于 `drawing/data/blip`
                //   （form-D gate 态：解除会在 `Blip.rs` 暴露 43 处模板引擎错误，
                //   属独立子波 M-PPT-DATA-BLIP-UNGATE，不在本波范围）。
                //   真身 `BlipFill::blip()` 只给出 `&mut dyn KoBinTrait`（序列化值抽象，
                //   无 map 写入口），故本行暂无真实落点。
                //   影响面：`a:blip/@cstate`（压缩提示）**不参与** C1 渲染管线。
                //   owner: ppt.drawing.dmlr.handler.shape.sppr.fill.BlipHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                let _ = tmpCstate;
            }
        }
        // java if ((tmp = attrsNotCopy.getByName(4259843)) != null && null != this.callback)
        if let Some(tmp) = attrsNotCopy.get_by_name(4259843) {
            if let Some(cb) = &self.callback {
                let link = tmp.to_string();
                // rust: ★2B-③-2 —— 真身 `Blip::putLinkMediaID`；callback 侧仍是采集列表。
                // rust: ★2B-③-2 —— 真 `Blip` 的写入同上延后；**图像引用本身经 callback 采集列表
                //   （`blip_link`）传递**，该列表是 `ConvertTool`/媒体加载的真实消费入口，
                //   故本条链路的真实接线**未受影响**。
                cb.borrow_mut().blip_link.push(link);
            }
        }
        // java if ((tmp = attrsNotCopy.getByName(4259842)) != null && null != this.callback)
        if let Some(tmp) = attrsNotCopy.get_by_name(4259842) {
            if let Some(cb) = &self.callback {
                let embed = tmp.to_string();
                // rust: 同 `blip_link` —— 真实接线经 callback 采集列表。
                cb.borrow_mut().blip_embed.push(embed);
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java effect chain 1114165–1114363 — MVP returns None
        match subElement {
            1114165 | 1114172 | 1114179 | 1114350 | 1114351 | 1114352 | 1114353 | 1114354
            | 1114355 | 1114356 | 1114357 | 1114358 | 1114359 | 1114360 | 1114361 | 1114362
            | 1114363 => None,
            _ => None,
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // java if (this.effects != null && this.effects.size() > 0) write effects
        // rust: no-stmt
    }
}

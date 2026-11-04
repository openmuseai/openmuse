// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::TextListStyleHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.show.data.slidemasters.SlideMaster
use crate::cn::wps::show::data::slidemasters::SlideMaster::SlideMaster;

use std::cell::RefCell;
use std::rc::Rc;

// Java: public final class TextStylesHandler extends DefaultElementHandler
pub struct TextStylesHandler<C: HandlerCallback> {
    // Java field: SlideMaster mMasterData
    mMasterData: Rc<RefCell<SlideMaster>>,
    // Java field: HandlerCallback mHandler
    mHandler: Option<Rc<RefCell<C>>>,
}

// Java: public final class TextStylesHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> TextStylesHandler<C> {
    // Java: TextStylesHandler(SlideMaster masterData, HandlerCallback handler)
    pub fn TextStylesHandler(
        masterData: Rc<RefCell<SlideMaster>>,
        handler: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = TextStylesHandler {
            mMasterData: masterData,
            mHandler: handler,
        };
        // Java: this.mMasterData = masterData;
        // rust: no-stmt
        // Java: this.mHandler = handler;
        // rust: no-stmt
        this
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211615:
            3211615 => {
                // Java: return new TextListStyleHandler(this.mMasterData.titleStyle(), this.mHandler);
                Some(Box::new(TextListStyleHandler::new(
                    self.mMasterData.borrow().titleStyle(),
                    self.mHandler.clone(),
                    "title",
                )))
            }
            // Java: case 3211616:
            3211616 => Some(Box::new(TextListStyleHandler::new(
                self.mMasterData.borrow().bodyStyle(),
                self.mHandler.clone(),
                "body",
            ))),
            // Java: case 3211617:
            3211617 => Some(Box::new(TextListStyleHandler::new(
                self.mMasterData.borrow().otherStyle(),
                self.mHandler.clone(),
                "other",
            ))),
            // Java: default:
            _ => None,
        }
    }
}

impl<C: HandlerCallback + 'static> ElementHandlerContract for TextStylesHandler<C> {
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextStylesHandler::enterSubElement(self, subElement)
    }
    fn kind(&self) -> &'static str {
        "TextStylesHandler"
    }
}

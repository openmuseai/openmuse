// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use crate::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use crate::cn::wps::show::app::slide::KmoSlideLayout::KmoSlideLayout;
// Java: import cn.wps.show.data.slidelayouts.SlideLayout
use crate::cn::wps::show::data::slidelayouts::SlideLayout::SlideLayout;
// Java: import cn.wps.show.pptxr.handler.slidemasters.HeaderFooterHandler
use crate::cn::wps::show::pptxr::handler::slidemasters::HeaderFooterHandler::HeaderFooterHandler;
// Java: import cn.wps.show.pptxr.handler.slides.ColorMapOverrideHandler
use crate::cn::wps::show::pptxr::handler::slides::ColorMapOverrideHandler::ColorMapOverrideHandler;
// Java: import cn.wps.show.pptxr.handler.slides.CommonSlideDataHandler
use crate::cn::wps::show::pptxr::handler::slides::CommonSlideDataHandler::CommonSlideDataHandler;
// Java: import cn.wps.show.pptxr.handler.timing.TimingHandler
use crate::cn::wps::show::pptxr::handler::timing::TimingHandler::TimingHandler;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionAlternateContentHandler
use crate::cn::wps::show::pptxr::handler::transition::TransitionAlternateContentHandler::TransitionAlternateContentHandler;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionHandler
use crate::cn::wps::show::pptxr::handler::transition::TransitionHandler::TransitionHandler;
// Java: import cn.wps.show.pptxr.util.SWReaderMap
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;

use std::cell::RefCell;
use std::rc::Rc;

// Java: public class SlideLayoutHandler extends DefaultElementHandler
pub struct SlideLayoutHandler<C: HandlerCallback> {
    // Java field: KmoSlideLayout mLayout
    mLayout: Rc<RefCell<KmoSlideLayout>>,
    // Java field: SlideLayout layoutData
    pub(crate) layoutData: Rc<RefCell<SlideLayout>>,
    // Java field: DgStore mDgStore
    mDgStore: Rc<RefCell<DgStore>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: public class SlideLayoutHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> SlideLayoutHandler<C> {
    // Java: SlideLayoutHandler(KmoSlideLayout layout, DgStore dgStore, HandlerCallback handlerCallback)
    pub fn SlideLayoutHandler(
        layout: Rc<RefCell<KmoSlideLayout>>,
        dgStore: Rc<RefCell<DgStore>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = SlideLayoutHandler {
            mLayout: layout,
            // Java: this.layoutData = SlideLayout.create();
            layoutData: SlideLayout::create(),
            mDgStore: dgStore,
            handlerCallback,
        };
        // Java: this.mLayout = layout;
        // rust: no-stmt
        // Java: this.mDgStore = dgStore;
        // rust: no-stmt
        // Java: this.handlerCallback = handlerCallback;
        // rust: no-stmt
        this
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162692:
            2162692 => Some(Box::new(TransitionAlternateContentHandler::new(
                self.layoutData.borrow().timing(),
                self.layoutData.borrow().transition(),
                self.handlerCallback.clone(),
            ))),
            // Java: case 3211295:
            3211295 => None,
            // Java: case 3211402:
            3211402 => Some(Box::new(TransitionHandler::new(
                self.layoutData.borrow().transition(),
                self.handlerCallback.clone(),
            ))),
            // Java: case 3211599:
            3211599 => Some(Box::new(ColorMapOverrideHandler::ColorMapOverrideHandler(
                self.layoutData.borrow().clrMap(),
            ))),
            // Java: case 3211608:
            3211608 => Some(Box::new(CommonSlideDataHandler::CommonSlideDataHandler(
                self.mLayout.borrow().film(),
                self.mDgStore.clone(),
                self.handlerCallback.clone(),
            ))),
            // Java: case 3211609:
            3211609 => Some(Box::new(TimingHandler::new(
                self.layoutData.borrow().timing(),
                self.handlerCallback.clone(),
            ))),
            // Java: case 3211611:
            3211611 => Some(Box::new(HeaderFooterHandler::HeaderFooterHandler(
                Some(self.layoutData.borrow().hf()),
            ))),
            // Java: default:
            _ => None,
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: int count = attrsNotCopy.count();
        let count = attrsNotCopy.count();
        // Java: XmlRoAttr tmp = null;
        let mut tmp: Option<&dyn XmlRoAttr> = None;
        // Java: int attrName = 0;
        let mut attrName: i32 = 0;
        // Java: for(int i = 0; i < count; ++i)
        let mut i = 0;
        while i < count {
            // Java: if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
            tmp = attrsNotCopy.get_attr_at(i);
            if let Some(tmp) = tmp {
                // Java: attrName = tmp.getName();
                attrName = tmp.get_name();
                // Java: switch (attrName)
                match attrName {
                    // Java: case 3211302:
                    3211302 => {
                        // Java: Integer type = (Integer)SWReaderMap.slideLayoutTypeMap.get(tmp.toString());
                        let r#type = SWReaderMap::get_slideLayoutTypeMap()
                            .get(tmp.to_string().as_str())
                            .copied();
                        // Java: this.layoutData.putType(type == null ? 23 : type);
                        self.layoutData
                            .borrow_mut()
                            .putType(r#type.unwrap_or(23));
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211573:
                    3211573 => {
                        // Java: this.layoutData.putUserDraw(tmp.toBoolean());
                        self.layoutData.borrow_mut().putUserDraw(tmp.to_boolean());
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211600:
                    3211600 => {
                        self.layoutData
                            .borrow_mut()
                            .putShowMasterSp(tmp.to_boolean());
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211601:
                    3211601 => {
                        self.layoutData
                            .borrow_mut()
                            .putShowMasterPhAnim(tmp.to_boolean());
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211612:
                    3211612 => {
                        // Java: this.layoutData.putMatchingName(tmp.toString());
                        self.layoutData
                            .borrow_mut()
                            .putMatchingName(tmp.to_string());
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211613:
                    3211613 => {
                        // Java: this.layoutData.putPreserve(tmp.toBoolean());
                        self.layoutData.borrow_mut().putPreserve(tmp.to_boolean());
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: this.mLayout.updateProtocol(this.layoutData.save());
        self.mLayout
            .borrow_mut()
            .updateProtocol(self.layoutData.borrow_mut().save());
    }
}

impl<C: HandlerCallback + 'static> ElementHandlerContract for SlideLayoutHandler<C> {
    fn startElement(&mut self, element: i32, attrs: &dyn XmlRoAttr) {
        SlideLayoutHandler::startElement(self, element, attrs);
    }
    fn endElement(&mut self, element: i32) {
        SlideLayoutHandler::endElement(self, element);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SlideLayoutHandler::enterSubElement(self, subElement)
    }
    fn kind(&self) -> &'static str {
        "SlideLayoutHandler"
    }
}

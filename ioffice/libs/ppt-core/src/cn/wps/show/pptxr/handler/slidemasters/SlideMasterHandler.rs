// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.dmlr.handler.style.ColorMapHandler
use crate::cn::wps::drawing::dmlr::handler::style::ColorMapHandler::ColorMapHandler;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use crate::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
use crate::cn::wps::show::app::slide::KmoSlideMaster::KmoSlideMaster;
// Java: import cn.wps.show.data.slidemasters.SlideMaster
use crate::cn::wps::show::data::slidemasters::SlideMaster::SlideMaster;
// Java: import cn.wps.show.pptxr.handler.slides.CommonSlideDataHandler
use crate::cn::wps::show::pptxr::handler::slides::CommonSlideDataHandler::CommonSlideDataHandler;
// Java: import cn.wps.show.pptxr.handler.timing.TimingHandler
use crate::cn::wps::show::pptxr::handler::timing::TimingHandler::TimingHandler;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionAlternateContentHandler
use crate::cn::wps::show::pptxr::handler::transition::TransitionAlternateContentHandler::TransitionAlternateContentHandler;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionHandler
use crate::cn::wps::show::pptxr::handler::transition::TransitionHandler::TransitionHandler;
// Java: import cn.wps.show.pptxr.handler.slidemasters.HeaderFooterHandler
use crate::cn::wps::show::pptxr::handler::slidemasters::HeaderFooterHandler::HeaderFooterHandler;
// Java: import cn.wps.show.pptxr.handler.slidemasters.TextStylesHandler
use crate::cn::wps::show::pptxr::handler::slidemasters::TextStylesHandler::TextStylesHandler;
// Java: import cn.wps.show.pptxr.ppt.slidemasters.MasterHandlerCallbackImpl
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import java.util.LinkedHashMap
// Java: import java.util.Map

use std::cell::RefCell;
use std::rc::Rc;

pub trait MasterHandlerCallback: HandlerCallback {
    fn onSlideLayoutIds(&mut self, ids: &[(String, i64)]);
}

// Java: public class SlideMasterHandler extends DefaultElementHandler
pub struct SlideMasterHandler<C: MasterHandlerCallback> {
    // Java field: KmoSlideMaster mMaster
    mMaster: Rc<RefCell<KmoSlideMaster>>,
    // Java field: SlideMaster masterData
    pub(crate) masterData: Rc<RefCell<SlideMaster>>,
    // Java field: DgStore mDgStore
    mDgStore: Rc<RefCell<DgStore>>,
    // Java field: MasterHandlerCallbackImpl handlerCallback
    handlerCallback: Rc<RefCell<C>>,
}

// Java: private class IdListHandler extends DefaultElementHandler
pub struct IdListHandler<C: MasterHandlerCallback> {
    // Java field: Map<String, Long> idMap
    idMap: Rc<RefCell<Vec<(String, i64)>>>,
    handlerCallback: Rc<RefCell<C>>,
}

// Java: public class SlideMasterHandler extends DefaultElementHandler
impl<C: MasterHandlerCallback + 'static> SlideMasterHandler<C> {
    // Java: SlideMasterHandler(KmoSlideMaster master, DgStore dgStore, MasterHandlerCallbackImpl handlerCallback)
    pub fn SlideMasterHandler(
        master: Rc<RefCell<KmoSlideMaster>>,
        dgStore: Rc<RefCell<DgStore>>,
        handlerCallback: Rc<RefCell<C>>,
    ) -> Self {
        // Java: this.mMaster = master;
        let mMaster = master;
        // Java: this.masterData = SlideMaster.open(this.mMaster.protocol());
        let masterData = SlideMaster::open(mMaster.borrow().protocol());
        // Java: this.mDgStore = dgStore;
        let mDgStore = dgStore;
        // Java: this.handlerCallback = handlerCallback;
        SlideMasterHandler {
            mMaster,
            masterData,
            mDgStore,
            handlerCallback,
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp = null;
        let mut tmp: Option<&dyn XmlRoAttr> = None;
        // Java: if ((tmp = attrsNotCopy.getByName(3211613)) != null)
        tmp = attrsNotCopy.get_by_name(3211613);
        if let Some(tmp) = tmp {
            // Java: this.masterData.putPreserve(tmp.toBoolean());
            self.masterData.borrow_mut().putPreserve(tmp.to_boolean());
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162692:
            2162692 => Some(Box::new(TransitionAlternateContentHandler::new(
                self.masterData.borrow().timing(),
                self.masterData.borrow().transition(),
                Some(self.handlerCallback.clone()),
            ))),
            // Java: case 3211295:
            3211295 => None,
            // Java: case 3211402:
            3211402 => Some(Box::new(TransitionHandler::new(
                self.masterData.borrow().transition(),
                Some(self.handlerCallback.clone()),
            ))),
            // Java: case 3211598:
            3211598 => Some(Box::new(ColorMapHandler::new(
                self.masterData.borrow().clrMap(),
            ))),
            // Java: case 3211608:
            3211608 => Some(Box::new(CommonSlideDataHandler::CommonSlideDataHandler(
                self.mMaster.borrow().film(),
                self.mDgStore.clone(),
                Some(self.handlerCallback.clone()),
            ))),
            // Java: case 3211609:
            3211609 => Some(Box::new(TimingHandler::new(
                self.masterData.borrow().timing(),
                Some(self.handlerCallback.clone()),
            ))),
            // Java: case 3211611:
            3211611 => Some(Box::new(HeaderFooterHandler::HeaderFooterHandler(Some(
                self.masterData.borrow().hf(),
            )))),
            // Java: case 3211619:
            3211619 => {
                // Java: Map<String, Long> idMap = new LinkedHashMap();
                let idMap = Rc::new(RefCell::new(Vec::new()));
                // Java: return new IdListHandler(idMap);
                Some(Box::new(IdListHandler::IdListHandler(
                    idMap,
                    self.handlerCallback.clone(),
                )))
            }
            // Java: case 3211620:
            3211620 => Some(Box::new(TextStylesHandler::TextStylesHandler(
                self.masterData.clone(),
                Some(self.handlerCallback.clone()),
            ))),
            // Java: default:
            _ => None,
        }
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: this.mMaster.updateProtocol(this.masterData.save());
        self.mMaster
            .borrow_mut()
            .updateProtocol(self.masterData.borrow_mut().save());
    }
}

// Java: private class IdListHandler extends DefaultElementHandler
impl<C: MasterHandlerCallback + 'static> IdListHandler<C> {
    // Java: IdListHandler(Map<String, Long> idMap)
    pub fn IdListHandler(
        idMap: Rc<RefCell<Vec<(String, i64)>>>,
        handlerCallback: Rc<RefCell<C>>,
    ) -> Self {
        let this = IdListHandler {
            idMap,
            handlerCallback,
        };
        // Java: this.idMap = idMap;
        // rust: no-stmt
        this
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return this;
        Some(Box::new(IdListHandler {
            idMap: self.idMap.clone(),
            handlerCallback: self.handlerCallback.clone(),
        }))
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
        // Java: if (element == 3211619)
        if element == 3211619 {
            // Java: SlideMasterHandler.this.handlerCallback.onSlideLayoutIds(this.idMap);
            self.handlerCallback
                .borrow_mut()
                .onSlideLayoutIds(&self.idMap.borrow());
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: if (element == 3211618)
        if element == 3211618 {
            // Java: XmlRoAttr tmp = null;
            let mut tmp: Option<&dyn XmlRoAttr> = None;
            // Java: if ((tmp = attrsNotCopy.getByName(4259841)) != null)
            tmp = attrsNotCopy.get_by_name(4259841);
            if let Some(rid_attr) = tmp {
                // Java: String rid = tmp.toString();
                let rid = rid_attr.to_string();
                // Java: long pid = 0L;
                let mut pid: i64 = 0;
                // Java: if ((tmp = attrsNotCopy.getByName(3211342)) != null)
                tmp = attrsNotCopy.get_by_name(3211342);
                if let Some(tmp) = tmp {
                    // Java: pid = tmp.toLong();
                    pid = tmp.to_long();
                }
                // Java: this.idMap.put(rid, pid);
                let mut map = self.idMap.borrow_mut();
                if let Some(entry) = map.iter_mut().find(|(k, _)| k == &rid) {
                    entry.1 = pid;
                } else {
                    map.push((rid, pid));
                }
            }
        }
    }
}

impl<C: MasterHandlerCallback + 'static> ElementHandlerContract for SlideMasterHandler<C> {
    fn startElement(&mut self, element: i32, attrs: &dyn XmlRoAttr) {
        SlideMasterHandler::startElement(self, element, attrs);
    }
    fn endElement(&mut self, element: i32) {
        SlideMasterHandler::endElement(self, element);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SlideMasterHandler::enterSubElement(self, subElement)
    }
    fn kind(&self) -> &'static str {
        "SlideMasterHandler"
    }
}

impl<C: MasterHandlerCallback + 'static> ElementHandlerContract for IdListHandler<C> {
    fn startElement(&mut self, element: i32, attrs: &dyn XmlRoAttr) {
        IdListHandler::startElement(self, element, attrs);
    }
    fn endElement(&mut self, element: i32) {
        IdListHandler::endElement(self, element);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        IdListHandler::enterSubElement(self, subElement)
    }
    fn kind(&self) -> &'static str {
        "IdListHandler"
    }
}

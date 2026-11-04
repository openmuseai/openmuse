// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.show.data.timing.timenode.TimeNode
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNode;
// Java: import java.util.List
// Java: import cn.wps.show.pptxr.handler.timing.TimeNodeParallelHandler
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNodeParallelHandler;
// Java: import cn.wps.show.pptxr.handler.timing.TimeNodeSequenceHandler
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNodeSequenceHandler;
// Java: import cn.wps.show.pptxr.handler.timing.TimeNodeExclusiveHandler
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNodeExclusiveHandler;
// Java: import cn.wps.show.pptxr.handler.timing.AnimateBehaviorHandler
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::FixtureHandler;
// Java: import cn.wps.show.pptxr.handler.timing.AnimateColorBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.AnimateEffectBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.AnimateMotionBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.AnimateRotationBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.AnimateScaleBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.CommandBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.SetBehaviorHandler
// Java: import cn.wps.show.pptxr.handler.timing.MediaNodeAudioHandler
// Java: import cn.wps.show.pptxr.handler.timing.MediaNodeVideoHandler
#[allow(unused_imports)]
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as _parked_tn_import;

mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}
use compile_surface::DefaultElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::rc::Rc;

// java public class TimeNodeListHandler extends DefaultElementHandler
pub struct TimeNodeListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<TimeNode> timeNodeList;
    timeNodeList: Rc<RefCell<Vec<Rc<RefCell<TimeNode>>>>>,
    // java private HandlerCallback callback;
    callback: Rc<RefCell<HandlerCallback>>,
}

// java public class ExtLstHandler extends DefaultElementHandler
pub struct ExtLstHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private ExtHandler extHandler = null;
    extHandler: Option<Rc<RefCell<ExtHandler>>>,
}

// java public class ExtHandler extends DefaultElementHandler
pub struct ExtHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private DataModelExt dataModelExt = null;
    dataModelExt: Option<Rc<RefCell<DataModelExt>>>,
}

// java private class DataModelExt extends DefaultElementHandler
pub struct DataModelExt {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java public class TimeNodeListHandler extends DefaultElementHandler
impl TimeNodeListHandler {
    // java public  TimeNodeListHandler(List<TimeNode> timeNodeList, HandlerCallback callback)
    pub fn TimeNodeListHandler(timeNodeList: Rc<RefCell<Vec<Rc<RefCell<TimeNode>>>>>, callback: Rc<RefCell<HandlerCallback>>) -> Self {
      // java this.timeNodeList = timeNodeList;
      Self { super_: DefaultElementHandler::default(), timeNodeList: timeNodeList, callback: callback }
      // java this.callback = callback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211295:
        3211295 => {
          // java handler = new ExtLstHandler();
          handler = Some(Box::new(ExtLstHandler::ExtLstHandler(self.timeNodeList.clone(), self.callback.clone())));
        }
        // java case 3211296: case 3211297: case 3211298: case 3211299: case 3211300: case 3211301: case 3211302: case 3211303: case 3211304: case 3211305: case 3211306: case 3211307: case 3211308: case 3211309: case 3211310: case 3211311: case 3211312: case 3211313: case 3211314: case 3211315: case 3211316: case 3211317: case 3211318: case 3211319: case 3211320: case 3211321: case 3211322:
        3211296 | 3211297 | 3211298 | 3211299 | 3211300 | 3211301 | 3211302 | 3211303 | 3211304 | 3211305 | 3211306 | 3211307 | 3211308 | 3211309 | 3211310 | 3211311 | 3211312 | 3211313 | 3211314 | 3211315 | 3211316 | 3211317 | 3211318 | 3211319 | 3211320 | 3211321 | 3211322 => {
        }
        // java case 3211323:
        3211323 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(0);
          timeNode.borrow_mut().putType(0);
          // java handler = new TimeNodeParallelHandler(...);
          handler = Some(Box::new(TimeNodeParallelHandler::TimeNodeParallelHandler()));
        }
        // java case 3211324:
        3211324 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(1);
          timeNode.borrow_mut().putType(1);
          // java handler = new TimeNodeSequenceHandler(...);
          handler = Some(Box::new(TimeNodeSequenceHandler::TimeNodeSequenceHandler()));
        }
        // java case 3211325:
        3211325 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(8);
          timeNode.borrow_mut().putType(8);
          // java handler = new TimeNodeExclusiveHandler(...);
          handler = Some(Box::new(TimeNodeExclusiveHandler::TimeNodeExclusiveHandler()));
        }
        // java case 3211326:
        3211326 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(9);
          timeNode.borrow_mut().putType(9);
          // java handler = new AnimateBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateBehaviorHandler" }));
        }
        // java case 3211327:
        3211327 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(10);
          timeNode.borrow_mut().putType(10);
          // java handler = new AnimateColorBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateColorBehaviorHandler" }));
        }
        // java case 3211328:
        3211328 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(2);
          timeNode.borrow_mut().putType(2);
          // java handler = new AnimateEffectBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateEffectBehaviorHandler" }));
        }
        // java case 3211329:
        3211329 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(6);
          timeNode.borrow_mut().putType(6);
          // java handler = new AnimateMotionBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateMotionBehaviorHandler" }));
        }
        // java case 3211330:
        3211330 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(11);
          timeNode.borrow_mut().putType(11);
          // java handler = new AnimateRotationBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateRotationBehaviorHandler" }));
        }
        // java case 3211331:
        3211331 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(5);
          timeNode.borrow_mut().putType(5);
          // java handler = new AnimateScaleBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "AnimateScaleBehaviorHandler" }));
        }
        // java case 3211332:
        3211332 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(7);
          timeNode.borrow_mut().putType(7);
          // java handler = new CommandBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "CommandBehaviorHandler" }));
        }
        // java case 3211333:
        3211333 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(3);
          timeNode.borrow_mut().putType(3);
          // java handler = new SetBehaviorHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "SetBehaviorHandler" }));
        }
        // java case 3211334:
        3211334 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(4);
          timeNode.borrow_mut().putType(4);
          // java handler = new MediaNodeAudioHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "MediaNodeAudioHandler" }));
        }
        // java case 3211335:
        3211335 => {
          // java TimeNode timeNode = TimeNode.create();
          let timeNode = TimeNode::create();
          // java this.timeNodeList.add(timeNode);
          self.timeNodeList.borrow_mut().push(timeNode.clone());
          // java timeNode.putType(12);
          timeNode.borrow_mut().putType(12);
          // java handler = new MediaNodeVideoHandler(...);
          handler = Some(Box::new(FixtureHandler { kind_name: "MediaNodeVideoHandler" }));
        }

        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

}

impl std::ops::Deref for TimeNodeListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TimeNodeListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class ExtLstHandler extends DefaultElementHandler
impl ExtLstHandler {
    pub fn ExtLstHandler(_timeNodeList: Rc<RefCell<Vec<Rc<RefCell<TimeNode>>>>>, _callback: Rc<RefCell<HandlerCallback>>) -> Self {
      Self { super_: DefaultElementHandler::default(), extHandler: None }
    }
    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211447:
        3211447 => {
          // java if (this.extHandler == null)
          if self.extHandler.is_none() {
            // java handler = new ExtHandler();
            let ext = ExtHandler { super_: DefaultElementHandler::default(), dataModelExt: None };
            handler = Some(Box::new(ExtHandler { super_: DefaultElementHandler::default(), dataModelExt: None }));
            self.extHandler = Some(Rc::new(RefCell::new(ext)));
          }
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

}

impl std::ops::Deref for ExtLstHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ExtLstHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class ExtHandler extends DefaultElementHandler
impl ExtHandler {
    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 6619137:
        6619137 => {
          // java if (this.dataModelExt == null)
          if self.dataModelExt.is_none() {
            // java handler = new DataModelExt();
            handler = Some(Box::new(DataModelExt { super_: DefaultElementHandler::default() }));
            self.dataModelExt = Some(Rc::new(RefCell::new(DataModelExt { super_: DefaultElementHandler::default() })));
          }
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

}

impl std::ops::Deref for ExtHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ExtHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class DataModelExt extends DefaultElementHandler
impl DataModelExt {
    // java private  DataModelExt()
    pub fn DataModelExt() -> Self {
      Self { super_: DefaultElementHandler::default() }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211326:
        3211326 => {
          // java TimeNode timeNode = TimeNode.create();
          let mut timeNode: TimeNode = TimeNode::create();
          // java TimeNodeListHandler.this.timeNodeList.add(timeNode);
          self.timeNodeList.add(timeNode);
          // java timeNode.putType(9);
          timeNode.putType(9);
          // java handler = new AnimateBehaviorHandler(timeNode.animate(), TimeNodeListHandler.this.callback);
          handler = AnimateBehaviorHandler::new(timeNode.animate(), self.callback);
        }
        // java default:
        _ => {
          // java return handler;
          return handler;
        }
      }
      */
    }

}

impl std::ops::Deref for DataModelExt {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DataModelExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for TimeNodeListHandler {
    fn kind(&self) -> &'static str { "TimeNodeListHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { TimeNodeListHandler::enterSubElement(self, s) }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for ExtLstHandler {
    fn kind(&self) -> &'static str { "ExtLstHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { ExtLstHandler::enterSubElement(self, s) }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for ExtHandler {
    fn kind(&self) -> &'static str { "ExtHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for DataModelExt {
    fn kind(&self) -> &'static str { "DataModelExt" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}

// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.show.data.timing.Build
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::Build;
// Java: import cn.wps.show.data.timing.Timing
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::Timing;
// Java: import cn.wps.show.data.timing.timenode.TimeNode
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNode;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.show.pptxr.handler.timing.TimeNodeListHandler
use crate::cn::wps::show::pptxr::handler::timing::TimeNodeListHandler::TimeNodeListHandler;
// Java: import cn.wps.show.pptxr.handler.timing.BuildListHandler
use crate::cn::wps::show::pptxr::handler::timing::BuildListHandler::BuildListHandler;

mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}
use compile_surface::DefaultElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::rc::Rc;

// java public class TimingHandler extends DefaultElementHandler
pub struct TimingHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Timing timing;
    pub(crate) timing: Rc<RefCell<Timing>>,
    // java private HandlerCallback callback;
    callback: Rc<RefCell<HandlerCallback>>,
    // java private List<TimeNode> timeNodeList;
    pub timeNodeList: Option<Rc<RefCell<Vec<Rc<RefCell<TimeNode>>>>>>,
    // java private List<Build> buildList;
    pub buildList: Option<Rc<RefCell<Vec<Rc<RefCell<Build>>>>>>,
}

// java public class TimingHandler extends DefaultElementHandler
impl TimingHandler {
    // java public  TimingHandler(Timing timing, HandlerCallback callback)
    pub fn TimingHandler(timing: Rc<RefCell<Timing>>, callback: Rc<RefCell<HandlerCallback>>) -> Self {
      // java this.timing = timing;
      Self { super_: DefaultElementHandler::default(), timing: timing, callback: callback, timeNodeList: None, buildList: None }
      // java this.callback = callback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211422:
        3211422 => {
          // java this.timeNodeList = new ArrayList();
          let list = Rc::new(RefCell::new(Vec::new()));
          self.timeNodeList = Some(list.clone());
          // java handler = new TimeNodeListHandler(this.timeNodeList, this.callback);
          handler = Some(Box::new(TimeNodeListHandler::TimeNodeListHandler(list, self.callback.clone())));
        }
        // java case 3211437:
        3211437 => {
          // java this.buildList = new ArrayList();
          let list = Rc::new(RefCell::new(Vec::new()));
          self.buildList = Some(list.clone());
          // java handler = new BuildListHandler(this.buildList, this.callback);
          handler = Some(Box::new(BuildListHandler::BuildListHandler(list, self.callback.clone())));
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (this.timeNodeList != null)
      if let Some(timeNodeList) = self.timeNodeList.clone() {
        // java for (TimeNode timeNode : this.timeNodeList)
        for timeNode in timeNodeList.borrow().iter() {
          // java timeNode.save();
          timeNode.borrow_mut().save();
        }
        // java this.timing.timeNodeLst().write(this.timeNodeList);
        self.timing.borrow_mut().timeNodeLst().write(timeNodeList.borrow().clone());
      }
      // java if (this.buildList != null)
      if let Some(buildList) = self.buildList.clone() {
        // java for (Build build : this.buildList)
        for build in buildList.borrow().iter() {
          // java build.save();
          build.borrow_mut().save();
        }
        // java this.timing.buildLst().write(this.buildList);
        self.timing.borrow_mut().buildLst().write(buildList.borrow().clone());
      }
    }

}

impl std::ops::Deref for TimingHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TimingHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for TimingHandler {
    fn kind(&self) -> &'static str { "TimingHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TimingHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        TimingHandler::endElement(self, element)
    }
}

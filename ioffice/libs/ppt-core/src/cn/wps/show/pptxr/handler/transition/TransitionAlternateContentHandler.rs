// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.show.data.timing.Timing
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::Timing;
// Java: import cn.wps.show.data.transition.Transition
use crate::cn::wps::show::pptxr::handler::transition::transition_model_surface::Transition;
// Java: import cn.wps.show.pptxr.handler.timing.TimingHandler
use crate::cn::wps::show::pptxr::handler::timing::TimingHandler::TimingHandler;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionHandler
use crate::cn::wps::show::pptxr::handler::transition::TransitionHandler::TransitionHandler;

mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}
use compile_surface::DefaultElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::rc::Rc;

// java public class TransitionAlternateContentHandler extends DefaultElementHandler
pub struct TransitionAlternateContentHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Timing timing;
    timing: Rc<RefCell<Timing>>,
    // java private Transition transition;
    transition: Rc<RefCell<Transition>>,
    // java private HandlerCallback callback;
    callback: Rc<RefCell<HandlerCallback>>,
}

// java public class TransitionAlternateContentHandler extends DefaultElementHandler
impl TransitionAlternateContentHandler {
    // java public  TransitionAlternateContentHandler(Timing timing, Transition transition, HandlerCallback callback)
    pub fn TransitionAlternateContentHandler(timing: Rc<RefCell<Timing>>, transition: Rc<RefCell<Transition>>, callback: Rc<RefCell<HandlerCallback>>) -> Self {
      // java this.timing = timing;
      Self { super_: DefaultElementHandler::default(), timing: timing, transition: transition, callback: callback }
      // java this.transition = transition;
      // java this.callback = callback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 2162693:
        2162693 => {
          // java handler = new DefaultElementHandler() { public ElementHandler enterSubElement(int subElement) { ElementHandler handler = null; switch (subElement) { case 3211402: handler = new TransitionHandler(TransitionAlternateContentHandler.this.transition, TransitionAlternateContentHandler.this.callback); break; case 3211609: handler = new TimingHandler(TransitionAlternateContentHandler.this.timing, TransitionAlternateContentHandler.this.callback); } return handler; } };
          handler = Some(Box::new(ChoiceHandler {
            transition: self.transition.clone(),
            timing: self.timing.clone(),
            callback: self.callback.clone(),
          }));
        }
        // java case 2162695: default:
        2162695 | _ => {
          // java return handler;
          return handler;
        }
      }
      // rust: fall-through of 2162693 matches Java return handler
      handler
    }

}

impl std::ops::Deref for TransitionAlternateContentHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TransitionAlternateContentHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

struct ChoiceHandler {
    transition: Rc<RefCell<Transition>>,
    timing: Rc<RefCell<Timing>>,
    callback: Rc<RefCell<HandlerCallback>>,
}

impl ElementHandler for ChoiceHandler {
    fn kind(&self) -> &'static str { "TransitionAlternateContentChoice" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      match subElement {
        3211402 => {
          handler = Some(Box::new(TransitionHandler::TransitionHandler(self.transition.clone(), self.callback.clone())));
        }
        3211609 => {
          handler = Some(Box::new(TimingHandler::TimingHandler(self.timing.clone(), self.callback.clone())));
        }
        _ => {}
      }
      handler
    }
    fn endElement(&mut self, _e: i32) {}
}

impl ElementHandler for TransitionAlternateContentHandler {
    fn kind(&self) -> &'static str { "TransitionAlternateContentHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TransitionAlternateContentHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _e: i32) {}
}

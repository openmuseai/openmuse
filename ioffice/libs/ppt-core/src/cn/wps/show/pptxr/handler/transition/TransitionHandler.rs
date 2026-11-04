// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::show::pptxr::handler::transition::transition_model_surface::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.data.transition.Transition
use crate::cn::wps::show::pptxr::handler::transition::transition_model_surface::Transition;
// Java: import cn.wps.show.pptxr.util.SWReaderMap
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
// Java: import cn.wps.show.pptxr.handler.transition.TransitionSoundActionHandler
// Java: import cn.wps.show.pptxr.handler.transition.ExtLstHandler
#[allow(unused_imports)]
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as _parked_transition_import;

mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}
use compile_surface::DefaultElementHandler;
use crate::cn::wps::show::pptxr::handler::transition::transition_model_surface::ParkedHandler;
use std::cell::RefCell;
use std::rc::Rc;

// java public class TransitionHandler extends DefaultElementHandler
pub struct TransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Transition mTransition;
    mTransition: Rc<RefCell<Transition>>,
    // java private HandlerCallback callback;
    callback: Rc<RefCell<HandlerCallback>>,
}

// java private class PresetTransitionHandler extends DefaultElementHandler
pub struct PresetTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Transition transition;
    transition: Rc<RefCell<Transition>>,
}

// java private class ShredTransitionHandler extends DefaultElementHandler
pub struct ShredTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class RippleTransitionHandler extends DefaultElementHandler
pub struct RippleTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class RevealTransitionHandler extends DefaultElementHandler
pub struct RevealTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class PrismTransitionHandler extends DefaultElementHandler
pub struct PrismTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class GlitterTransitionHandler extends DefaultElementHandler
pub struct GlitterTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class FlyThroughTransitionHandler extends DefaultElementHandler
pub struct FlyThroughTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class LeftRightDirectionTransitionHandler extends DefaultElementHandler
pub struct LeftRightDirectionTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class OrientationTransitionHandler extends DefaultElementHandler
pub struct OrientationTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class EightDirectionTransitionHandler extends DefaultElementHandler
pub struct EightDirectionTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class CornerDirectionTransitionHandler extends DefaultElementHandler
pub struct CornerDirectionTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class SideDirectionTransitionHandler extends DefaultElementHandler
pub struct SideDirectionTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class InOutTransitionHandler extends DefaultElementHandler
pub struct InOutTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class SplitTransitionHandler extends DefaultElementHandler
pub struct SplitTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class WheelTransitionHandler extends DefaultElementHandler
pub struct WheelTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java private class OptinalBlackTransitionHandler extends DefaultElementHandler
pub struct OptinalBlackTransitionHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    mTransition: Rc<RefCell<Transition>>,
}

// java public class TransitionHandler extends DefaultElementHandler
impl TransitionHandler {
    // java public  TransitionHandler(Transition transition, HandlerCallback callback)
    pub fn TransitionHandler(transition: Rc<RefCell<Transition>>, callback: Rc<RefCell<HandlerCallback>>) -> Self {
      // java this.mTransition = transition;
      Self { super_: DefaultElementHandler::default(), mTransition: transition, callback: callback }
      // java this.callback = callback;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = null;
      let mut tmp = None;
      // java if ((tmp = attrsNotCopy.getByName(3211297)) != null)
      tmp = attrsNotCopy.get_by_name(3211297);
      if tmp.is_some() {
        // java this.mTransition.putAdvClick(tmp.toBoolean());
        self.mTransition.borrow_mut().putAdvClick(tmp.as_ref().unwrap().to_boolean());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211298)) != null)
      tmp = attrsNotCopy.get_by_name(3211298);
      if tmp.is_some() {
        // java this.mTransition.putAdvTm(tmp.toInt());
        self.mTransition.borrow_mut().putAdvTm(tmp.as_ref().unwrap().to_int());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211296)) != null)
      tmp = attrsNotCopy.get_by_name(3211296);
      if tmp.is_some() {
        // java this.mTransition.putSpd((Integer)SWReaderMap.transitionSpeedMap.get(tmp.toString()));
        if let Some(v) = SWReaderMap::get_transitionSpeedMap().get(tmp.as_ref().unwrap().to_string().as_str()) { self.mTransition.borrow_mut().putSpd(*v); }
      }
      // java if ((tmp = attrsNotCopy.getByName(3276828)) != null)
      tmp = attrsNotCopy.get_by_name(3276828);
      if tmp.is_some() {
        // java this.mTransition.putDur(tmp.toInt());
        self.mTransition.borrow_mut().putDur(tmp.as_ref().unwrap().to_int());
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211273: case 3211274: case 3211277: case 3211287:
        3211273 | 3211274 | 3211277 | 3211287 => {
          // java handler = new OrientationTransitionHandler();
          handler = Some(Box::new(OrientationTransitionHandler::OrientationTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211275:
        3211275 => {
          // java this.mTransition.putMode(27);
          self.mTransition.borrow_mut().putMode(27);
        }
        // java case 3211276:
        3211276 => {
          // java this.mTransition.putMode(5);
          self.mTransition.borrow_mut().putMode(5);
        }
        // java case 3211278: case 3211284:
        3211278 | 3211284 => {
          // java handler = new EightDirectionTransitionHandler();
          handler = Some(Box::new(EightDirectionTransitionHandler::EightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211279: case 3211281:
        3211279 | 3211281 => {
          // java handler = new OptinalBlackTransitionHandler();
          handler = Some(Box::new(OptinalBlackTransitionHandler::OptinalBlackTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211280:
        3211280 => {
          // java this.mTransition.putMode(17);
          self.mTransition.borrow_mut().putMode(17);
        }
        // java case 3211282:
        3211282 => {
          // java this.mTransition.putMode(22);
          self.mTransition.borrow_mut().putMode(22);
        }
        // java case 3211283:
        3211283 => {
          // java this.mTransition.putMode(18);
          self.mTransition.borrow_mut().putMode(18);
        }
        // java case 3211285: case 3211292:
        3211285 | 3211292 => {
          // java handler = new SideDirectionTransitionHandler();
          handler = Some(Box::new(SideDirectionTransitionHandler::SideDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211286:
        3211286 => {
          // java this.mTransition.putMode(1);
          self.mTransition.borrow_mut().putMode(1);
        }
        // java case 3211288:
        3211288 => {
          // java this.mTransition.putMode(13);
          self.mTransition.borrow_mut().putMode(13);
          // java handler = new SplitTransitionHandler();
          handler = Some(Box::new(SplitTransitionHandler::SplitTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211289:
        3211289 => {
          // java this.mTransition.putMode(9);
          self.mTransition.borrow_mut().putMode(9);
          // java handler = new CornerDirectionTransitionHandler();
          handler = Some(Box::new(CornerDirectionTransitionHandler::CornerDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211290:
        3211290 => {
          // java this.mTransition.putMode(19);
          self.mTransition.borrow_mut().putMode(19);
        }
        // java case 3211291:
        3211291 => {
          // java handler = new WheelTransitionHandler();
          handler = Some(Box::new(WheelTransitionHandler::WheelTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211293:
        3211293 => {
          // java handler = new InOutTransitionHandler();
          handler = Some(Box::new(InOutTransitionHandler::InOutTransitionHandler(self.mTransition.clone())));
        }
        // java case 3211294:
        3211294 => {
          // java handler = new TransitionSoundActionHandler(this.mTransition.soundAction(), this.callback);
          handler = Some(Box::new(ParkedHandler { kind_name: "TransitionSoundActionHandler" }));
        }
        // java case 3211295:
        3211295 => {
          // java handler = new ExtLstHandler(this.mTransition, this.callback);
          handler = Some(Box::new(ParkedHandler { kind_name: "ExtLstHandler" }));
        }
        // java case 3276802:
        3276802 => {
          // java handler = new SideDirectionTransitionHandler();
          handler = Some(Box::new(SideDirectionTransitionHandler::SideDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276803:
        3276803 => {
          // java handler = new LeftRightDirectionTransitionHandler();
          handler = Some(Box::new(LeftRightDirectionTransitionHandler::LeftRightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276804:
        3276804 => {
          // java handler = new LeftRightDirectionTransitionHandler();
          handler = Some(Box::new(LeftRightDirectionTransitionHandler::LeftRightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276805:
        3276805 => {
          // java this.mTransition.putMode(112);
          self.mTransition.borrow_mut().putMode(112);
          // java handler = new RippleTransitionHandler();
          handler = Some(Box::new(RippleTransitionHandler::RippleTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276806:
        3276806 => {
          // java this.mTransition.putMode(108);
          self.mTransition.borrow_mut().putMode(108);
        }
        // java case 3276809:
        3276809 => {
          // java this.mTransition.putMode(110);
          self.mTransition.borrow_mut().putMode(110);
          // java handler = new PrismTransitionHandler();
          handler = Some(Box::new(PrismTransitionHandler::PrismTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276810:
        3276810 => {
          // java handler = new OrientationTransitionHandler();
          handler = Some(Box::new(OrientationTransitionHandler::OrientationTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276811:
        3276811 => {
          // java handler = new OrientationTransitionHandler();
          handler = Some(Box::new(OrientationTransitionHandler::OrientationTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276812:
        3276812 => {
          // java handler = new LeftRightDirectionTransitionHandler();
          handler = Some(Box::new(LeftRightDirectionTransitionHandler::LeftRightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276813:
        3276813 => {
          // java handler = new LeftRightDirectionTransitionHandler();
          handler = Some(Box::new(LeftRightDirectionTransitionHandler::LeftRightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276814:
        3276814 => {
          // java handler = new LeftRightDirectionTransitionHandler();
          handler = Some(Box::new(LeftRightDirectionTransitionHandler::LeftRightDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276815:
        3276815 => {
          // java handler = new SideDirectionTransitionHandler();
          handler = Some(Box::new(SideDirectionTransitionHandler::SideDirectionTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276817:
        3276817 => {
          // java this.mTransition.putMode(107);
          self.mTransition.borrow_mut().putMode(107);
          // java handler = new GlitterTransitionHandler();
          handler = Some(Box::new(GlitterTransitionHandler::GlitterTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276819:
        3276819 => {
          // java handler = new InOutTransitionHandler();
          handler = Some(Box::new(InOutTransitionHandler::InOutTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276820:
        3276820 => {
          // java this.mTransition.putMode(105);
          self.mTransition.borrow_mut().putMode(105);
          // java handler = new FlyThroughTransitionHandler();
          handler = Some(Box::new(FlyThroughTransitionHandler::FlyThroughTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276821:
        3276821 => {
          // java this.mTransition.putMode(103);
          self.mTransition.borrow_mut().putMode(103);
        }
        // java case 3276822:
        3276822 => {
          // java this.mTransition.putMode(113);
          self.mTransition.borrow_mut().putMode(113);
          // java handler = new ShredTransitionHandler();
          handler = Some(Box::new(ShredTransitionHandler::ShredTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276824:
        3276824 => {
          // java this.mTransition.putMode(111);
          self.mTransition.borrow_mut().putMode(111);
          // java handler = new RevealTransitionHandler();
          handler = Some(Box::new(RevealTransitionHandler::RevealTransitionHandler(self.mTransition.clone())));
        }
        // java case 3276827:
        3276827 => {
          // java handler = new WheelTransitionHandler();
          handler = Some(Box::new(WheelTransitionHandler::WheelTransitionHandler(self.mTransition.clone())));
        }
        // java case 3342340:
        3342340 => {
          // java handler = new PresetTransitionHandler(this.mTransition);
          handler = Some(Box::new(PresetTransitionHandler::PresetTransitionHandler(self.mTransition.clone())));
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

}

impl std::ops::Deref for TransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class PresetTransitionHandler extends DefaultElementHandler
impl PresetTransitionHandler {
    // java public  PresetTransitionHandler(Transition transition)
    pub fn PresetTransitionHandler(transition: Rc<RefCell<Transition>>) -> Self {
      // java this.transition = transition;
      Self { super_: DefaultElementHandler::default(), transition: transition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = null;
      let mut tmp = None;
      // java if ((tmp = attrsNotCopy.getByName(3342337)) != null)
      tmp = attrsNotCopy.get_by_name(3342337);
      if tmp.is_some() {
        // java Integer preset = (Integer)SWReaderMap.presetTransitionMap.get(tmp.toString());
        let preset = SWReaderMap::get_presetTransitionMap().get(tmp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (preset != null)
        if let Some(preset) = preset {
          // java this.transition.putMode(preset);
          self.transition.borrow_mut().putMode(preset);
        }
      }
      // java if ((tmp = attrsNotCopy.getByName(3342338)) != null)
      tmp = attrsNotCopy.get_by_name(3342338);
      if tmp.is_some() {
        // java this.transition.putInvX(tmp.toBoolean());
        self.transition.borrow_mut().putInvX(tmp.as_ref().unwrap().to_boolean());
      }
      // java if ((tmp = attrsNotCopy.getByName(3342339)) != null)
      tmp = attrsNotCopy.get_by_name(3342339);
      if tmp.is_some() {
        // java this.transition.putInvY(tmp.toBoolean());
        self.transition.borrow_mut().putInvY(tmp.as_ref().unwrap().to_boolean());
      }
    }

}

impl std::ops::Deref for PresetTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PresetTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class ShredTransitionHandler extends DefaultElementHandler
impl ShredTransitionHandler {
    // java private  ShredTransitionHandler()
    pub fn ShredTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java TransitionHandler.this.mTransition.putShredPattern(1);
      self.mTransition.borrow_mut().putShredPattern(1);
      // java if ((temp = attrsNotCopy.getByName(3276816)) != null)
      temp = attrsNotCopy.get_by_name(3276816);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putShredPattern((Integer)SWReaderMap.transitionShredPatternMap.get(temp.toString()));
        if let Some(v) = SWReaderMap::get_transitionShredPatternMap().get(temp.as_ref().unwrap().to_string().as_str()) { self.mTransition.borrow_mut().putShredPattern(*v); }
      }
      // java TransitionHandler.this.mTransition.putShredDirection(1);
      self.mTransition.borrow_mut().putShredDirection(1);
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionInOutDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionInOutDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putShredDirection(dir);
          self.mTransition.borrow_mut().putShredDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for ShredTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ShredTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class RippleTransitionHandler extends DefaultElementHandler
impl RippleTransitionHandler {
    // java private  RippleTransitionHandler()
    pub fn RippleTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionCornerAndCenterDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionCornerAndCenterDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putRippleDirection(dir);
          self.mTransition.borrow_mut().putRippleDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for RippleTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for RippleTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class RevealTransitionHandler extends DefaultElementHandler
impl RevealTransitionHandler {
    // java private  RevealTransitionHandler()
    pub fn RevealTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276823)) != null)
      temp = attrsNotCopy.get_by_name(3276823);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putRevealThruBlk(temp.toBoolean());
        self.mTransition.borrow_mut().putRevealThruBlk(temp.as_ref().unwrap().to_boolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionLeftRightDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionLeftRightDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putRevealDirection(dir);
          self.mTransition.borrow_mut().putRevealDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for RevealTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for RevealTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class PrismTransitionHandler extends DefaultElementHandler
impl PrismTransitionHandler {
    // java private  PrismTransitionHandler()
    pub fn PrismTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276807)) != null)
      temp = attrsNotCopy.get_by_name(3276807);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putPrismIsContent(temp.toBoolean());
        self.mTransition.borrow_mut().putPrismIsContent(temp.as_ref().unwrap().to_boolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3276808)) != null)
      temp = attrsNotCopy.get_by_name(3276808);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putPrismIsInverted(temp.toBoolean());
        self.mTransition.borrow_mut().putPrismIsInverted(temp.as_ref().unwrap().to_boolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionSideDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionSideDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putPrismDirection(dir);
          self.mTransition.borrow_mut().putPrismDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for PrismTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PrismTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class GlitterTransitionHandler extends DefaultElementHandler
impl GlitterTransitionHandler {
    // java private  GlitterTransitionHandler()
    pub fn GlitterTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java TransitionHandler.this.mTransition.putGlitterPattern(1);
      self.mTransition.borrow_mut().putGlitterPattern(1);
      // java TransitionHandler.this.mTransition.putGlitterDirection(0);
      self.mTransition.borrow_mut().putGlitterDirection(0);
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276816)) != null)
      temp = attrsNotCopy.get_by_name(3276816);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putGlitterPattern((Integer)SWReaderMap.transitionPatternMap.get(temp.toString()));
        if let Some(v) = SWReaderMap::get_transitionPatternMap().get(temp.as_ref().unwrap().to_string().as_str()) { self.mTransition.borrow_mut().putGlitterPattern(*v); }
      }
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionSideDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionSideDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putGlitterDirection(dir);
          self.mTransition.borrow_mut().putGlitterDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for GlitterTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for GlitterTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class FlyThroughTransitionHandler extends DefaultElementHandler
impl FlyThroughTransitionHandler {
    // java private  FlyThroughTransitionHandler()
    pub fn FlyThroughTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java TransitionHandler.this.mTransition.putFlythroughDirection(1);
      self.mTransition.borrow_mut().putFlythroughDirection(1);
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276818)) != null)
      temp = attrsNotCopy.get_by_name(3276818);
      if temp.is_some() {
        // java TransitionHandler.this.mTransition.putFlythroughHasBounce(temp.toBoolean());
        self.mTransition.borrow_mut().putFlythroughHasBounce(temp.as_ref().unwrap().to_boolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionInOutDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionInOutDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putFlythroughDirection(dir);
          self.mTransition.borrow_mut().putFlythroughDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for FlyThroughTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FlyThroughTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class LeftRightDirectionTransitionHandler extends DefaultElementHandler
impl LeftRightDirectionTransitionHandler {
    // java private  LeftRightDirectionTransitionHandler()
    pub fn LeftRightDirectionTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 3276803:
        3276803 => {
          // java TransitionHandler.this.mTransition.putMode(114);
          self.mTransition.borrow_mut().putMode(114);
        }
        // java case 3276804:
        3276804 => {
          // java TransitionHandler.this.mTransition.putMode(104);
          self.mTransition.borrow_mut().putMode(104);
        }
        // java case 3276805: case 3276806: case 3276807: case 3276808: case 3276809: case 3276810: case 3276811: default:
        3276805 | 3276806 | 3276807 | 3276808 | 3276809 | 3276810 | 3276811 => {
        }
        // java case 3276812:
        3276812 => {
          // java TransitionHandler.this.mTransition.putMode(102);
          self.mTransition.borrow_mut().putMode(102);
        }
        // java case 3276813:
        3276813 => {
          // java TransitionHandler.this.mTransition.putMode(106);
          self.mTransition.borrow_mut().putMode(106);
        }
        // java case 3276814:
        3276814 => {
          // java TransitionHandler.this.mTransition.putMode(100);
          self.mTransition.borrow_mut().putMode(100);
        }
        _ => {}
      }
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3276801);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionLeftRightDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionLeftRightDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java switch (element)
          match element {
            // java case 3276803:
            3276803 => {
              // java TransitionHandler.this.mTransition.putSwitchDirection(dir);
              self.mTransition.borrow_mut().putSwitchDirection(dir);
            }
            // java case 3276804:
            3276804 => {
              // java TransitionHandler.this.mTransition.putFlipDirection(dir);
              self.mTransition.borrow_mut().putFlipDirection(dir);
            }
            // java case 3276805: case 3276806: case 3276807: case 3276808: case 3276809: case 3276810: case 3276811: default:
            3276805 | 3276806 | 3276807 | 3276808 | 3276809 | 3276810 | 3276811 => {
            }
            // java case 3276812:
            3276812 => {
              // java TransitionHandler.this.mTransition.putFerrisDirection(dir);
              self.mTransition.borrow_mut().putFerrisDirection(dir);
            }
            // java case 3276813:
            3276813 => {
              // java TransitionHandler.this.mTransition.putGalleryDirection(dir);
              self.mTransition.borrow_mut().putGalleryDirection(dir);
            }
            // java case 3276814:
            3276814 => {
              // java TransitionHandler.this.mTransition.putConveyorDirection(dir);
              self.mTransition.borrow_mut().putConveyorDirection(dir);
            }
            _ => {}
          }
        }
      }
    }

}

impl std::ops::Deref for LeftRightDirectionTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LeftRightDirectionTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class OrientationTransitionHandler extends DefaultElementHandler
impl OrientationTransitionHandler {
    // java private  OrientationTransitionHandler()
    pub fn OrientationTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 3211273:
        3211273 => {
          // java TransitionHandler.this.mTransition.putMode(2);
          self.mTransition.borrow_mut().putMode(2);
        }
        // java case 3211274:
        3211274 => {
          // java TransitionHandler.this.mTransition.putMode(3);
          self.mTransition.borrow_mut().putMode(3);
        }
        // java case 3211277:
        3211277 => {
          // java TransitionHandler.this.mTransition.putMode(21);
          self.mTransition.borrow_mut().putMode(21);
        }
        // java case 3211287:
        3211287 => {
          // java TransitionHandler.this.mTransition.putMode(8);
          self.mTransition.borrow_mut().putMode(8);
        }
        // java case 3276810:
        3276810 => {
          // java TransitionHandler.this.mTransition.putMode(101);
          self.mTransition.borrow_mut().putMode(101);
        }
        // java case 3276811:
        3276811 => {
          // java TransitionHandler.this.mTransition.putMode(118);
          self.mTransition.borrow_mut().putMode(118);
        }
        // java default:
        _ => {
        }
      }
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null || (temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_none() { temp = attrsNotCopy.get_by_name(3276801); }
      if temp.is_some() {
        // java Integer dir = (Integer)DGMap.orientationMap.get(temp.toString());
        let dir = DGMap::orientationMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java switch (element)
          match element {
            // java case 3211273:
            3211273 => {
              // java TransitionHandler.this.mTransition.putBlindsDirection(dir);
              self.mTransition.borrow_mut().putBlindsDirection(dir);
            }
            // java case 3211274:
            3211274 => {
              // java TransitionHandler.this.mTransition.putCheckerDirection(dir);
              self.mTransition.borrow_mut().putCheckerDirection(dir);
            }
            // java case 3211277:
            3211277 => {
              // java TransitionHandler.this.mTransition.putCombDirection(dir);
              self.mTransition.borrow_mut().putCombDirection(dir);
            }
            // java case 3211287:
            3211287 => {
              // java TransitionHandler.this.mTransition.putRandomBarDirection(dir);
              self.mTransition.borrow_mut().putRandomBarDirection(dir);
            }
            // java case 3276810:
            3276810 => {
              // java TransitionHandler.this.mTransition.putDoorsDirection(dir);
              self.mTransition.borrow_mut().putDoorsDirection(dir);
            }
            // java case 3276811:
            3276811 => {
              // java TransitionHandler.this.mTransition.putWindowDirection(dir);
              self.mTransition.borrow_mut().putWindowDirection(dir);
            }
            // java default:
            _ => {
            }
          }
        }
      }
    }

}

impl std::ops::Deref for OrientationTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OrientationTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class EightDirectionTransitionHandler extends DefaultElementHandler
impl EightDirectionTransitionHandler {
    // java private  EightDirectionTransitionHandler()
    pub fn EightDirectionTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java TransitionHandler.this.mTransition.putMode(element == 3211278 ? 4 : 7);
      self.mTransition.borrow_mut().putMode((if element == 3211278 { 4 } else { 7 }));
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionEightDirMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionEightDirMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java if (element == 3211278)
          if element == 3211278 {
            // java TransitionHandler.this.mTransition.putCoverDirection(dir);
            self.mTransition.borrow_mut().putCoverDirection(dir);
          }
          // java else
          else {
            // java TransitionHandler.this.mTransition.putPullDirection(dir);
            self.mTransition.borrow_mut().putPullDirection(dir);
          }
        }
      }
    }

}

impl std::ops::Deref for EightDirectionTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EightDirectionTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class CornerDirectionTransitionHandler extends DefaultElementHandler
impl CornerDirectionTransitionHandler {
    // java private  CornerDirectionTransitionHandler()
    pub fn CornerDirectionTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionCornerDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionCornerDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putStripsDirection(dir);
          self.mTransition.borrow_mut().putStripsDirection(dir);
        }
      }
    }

}

impl std::ops::Deref for CornerDirectionTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CornerDirectionTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SideDirectionTransitionHandler extends DefaultElementHandler
impl SideDirectionTransitionHandler {
    // java private  SideDirectionTransitionHandler()
    pub fn SideDirectionTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 3211285:
        3211285 => {
          // java TransitionHandler.this.mTransition.putMode(20);
          self.mTransition.borrow_mut().putMode(20);
        }
        // java case 3211292:
        3211292 => {
          // java TransitionHandler.this.mTransition.putMode(10);
          self.mTransition.borrow_mut().putMode(10);
        }
        // java case 3276802:
        3276802 => {
          // java TransitionHandler.this.mTransition.putMode(115);
          self.mTransition.borrow_mut().putMode(115);
        }
        // java case 3276815:
        3276815 => {
          // java TransitionHandler.this.mTransition.putMode(109);
          self.mTransition.borrow_mut().putMode(109);
        }
        // java default:
        _ => {
        }
      }
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null || (temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_none() { temp = attrsNotCopy.get_by_name(3276801); }
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionSideDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionSideDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java switch (element)
          match element {
            // java case 3211285:
            3211285 => {
              // java TransitionHandler.this.mTransition.putPushDirection(dir);
              self.mTransition.borrow_mut().putPushDirection(dir);
            }
            // java case 3211292:
            3211292 => {
              // java TransitionHandler.this.mTransition.putWipeDirection(dir);
              self.mTransition.borrow_mut().putWipeDirection(dir);
            }
            // java case 3276802:
            3276802 => {
              // java TransitionHandler.this.mTransition.putVortexDirection(dir);
              self.mTransition.borrow_mut().putVortexDirection(dir);
            }
            // java case 3276815:
            3276815 => {
              // java TransitionHandler.this.mTransition.putPanDirection(dir);
              self.mTransition.borrow_mut().putPanDirection(dir);
            }
            // java default:
            _ => {
            }
          }
        }
      }
    }

}

impl std::ops::Deref for SideDirectionTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SideDirectionTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class InOutTransitionHandler extends DefaultElementHandler
impl InOutTransitionHandler {
    // java private  InOutTransitionHandler()
    pub fn InOutTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 3211293:
        3211293 => {
          // java TransitionHandler.this.mTransition.putMode(11);
          self.mTransition.borrow_mut().putMode(11);
        }
        // java case 3276819:
        3276819 => {
          // java TransitionHandler.this.mTransition.putMode(116);
          self.mTransition.borrow_mut().putMode(116);
        }
        // java default:
        _ => {
        }
      }
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null || (temp = attrsNotCopy.getByName(3276801)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_none() { temp = attrsNotCopy.get_by_name(3276801); }
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionInOutDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionInOutDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java switch (element)
          match element {
            // java case 3211293:
            3211293 => {
              // java TransitionHandler.this.mTransition.putZoomDirection(dir);
              self.mTransition.borrow_mut().putZoomDirection(dir);
            }
            // java case 3276819:
            3276819 => {
              // java TransitionHandler.this.mTransition.putWarpDirection(dir);
              self.mTransition.borrow_mut().putWarpDirection(dir);
            }
            // java default:
            _ => {
            }
          }
        }
      }
    }

}

impl std::ops::Deref for InOutTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for InOutTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SplitTransitionHandler extends DefaultElementHandler
impl SplitTransitionHandler {
    // java private  SplitTransitionHandler()
    pub fn SplitTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211265)) != null)
      temp = attrsNotCopy.get_by_name(3211265);
      if temp.is_some() {
        // java Integer dir = (Integer)SWReaderMap.transitionInOutDirectionMap.get(temp.toString());
        let dir = SWReaderMap::get_transitionInOutDirectionMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (dir != null)
        if let Some(dir) = dir {
          // java TransitionHandler.this.mTransition.putSplitInOutDirection(dir);
          self.mTransition.borrow_mut().putSplitInOutDirection(dir);
        }
      }
      // java if ((temp = attrsNotCopy.getByName(3211267)) != null)
      temp = attrsNotCopy.get_by_name(3211267);
      if temp.is_some() {
        // java Integer orient = (Integer)DGMap.orientationMap.get(temp.toString());
        let orient = DGMap::orientationMap().get(temp.as_ref().unwrap().to_string().as_str()).copied();
        // java if (orient != null)
        if let Some(orient) = orient {
          // java TransitionHandler.this.mTransition.putSplitOrientationDirection(orient);
          self.mTransition.borrow_mut().putSplitOrientationDirection(orient);
        }
      }
    }

}

impl std::ops::Deref for SplitTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SplitTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class WheelTransitionHandler extends DefaultElementHandler
impl WheelTransitionHandler {
    // java private  WheelTransitionHandler()
    pub fn WheelTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java switch (element)
      match element {
        // java case 3211291:
        3211291 => {
          // java TransitionHandler.this.mTransition.putMode(26);
          self.mTransition.borrow_mut().putMode(26);
        }
        // java case 3276827:
        3276827 => {
          // java TransitionHandler.this.mTransition.putMode(117);
          self.mTransition.borrow_mut().putMode(117);
        }
        // java default:
        _ => {
        }
      }
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if (element == 3211291)
      if element == 3211291 {
        // java if ((temp = attrsNotCopy.getByName(3211268)) != null)
        temp = attrsNotCopy.get_by_name(3211268);
      if temp.is_some() {
          // java TransitionHandler.this.mTransition.putWheelSpokes(temp.toInt());
          self.mTransition.borrow_mut().putWheelSpokes(temp.as_ref().unwrap().to_int());
        }
      }
      // java else if (element == 3276827 && (temp = attrsNotCopy.getByName(3276882)) != null)
      else if element == 3276827 { temp = attrsNotCopy.get_by_name(3276882); if temp.is_some() {
        // java TransitionHandler.this.mTransition.putWheelRevereSpokes(temp.toInt());
        self.mTransition.borrow_mut().putWheelRevereSpokes(temp.as_ref().unwrap().to_int());
      } }
    }

}

impl std::ops::Deref for WheelTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for WheelTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class OptinalBlackTransitionHandler extends DefaultElementHandler
impl OptinalBlackTransitionHandler {
    // java private  OptinalBlackTransitionHandler()
    pub fn OptinalBlackTransitionHandler(mTransition: Rc<RefCell<Transition>>) -> Self {
      Self { super_: DefaultElementHandler::default(), mTransition }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java TransitionHandler.this.mTransition.putMode(element == 3211279 ? 0 : 6);
      self.mTransition.borrow_mut().putMode((if element == 3211279 { 0 } else { 6 }));
      // java XmlRoAttr temp = null;
      let mut temp = None;
      // java if ((temp = attrsNotCopy.getByName(3211266)) != null)
      temp = attrsNotCopy.get_by_name(3211266);
      if temp.is_some() {
        // java if (element == 3211279)
        if element == 3211279 {
          // java TransitionHandler.this.mTransition.putCutOptionalBlackTransition(temp.toInt());
          self.mTransition.borrow_mut().putCutOptionalBlackTransition(temp.as_ref().unwrap().to_int());
        }
        // java else if (element == 3211281)
        else if element == 3211281 {
          // java TransitionHandler.this.mTransition.putFadeOptionalBlackTransition(temp.toBoolean());
          self.mTransition.borrow_mut().putFadeOptionalBlackTransition(temp.as_ref().unwrap().to_boolean());
        }
      }
    }

}

impl std::ops::Deref for OptinalBlackTransitionHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OptinalBlackTransitionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}


impl ElementHandler for TransitionHandler {
    fn kind(&self) -> &'static str { "TransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        TransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TransitionHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for PresetTransitionHandler {
    fn kind(&self) -> &'static str { "PresetTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PresetTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for ShredTransitionHandler {
    fn kind(&self) -> &'static str { "ShredTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        ShredTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for RippleTransitionHandler {
    fn kind(&self) -> &'static str { "RippleTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        RippleTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for RevealTransitionHandler {
    fn kind(&self) -> &'static str { "RevealTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        RevealTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for PrismTransitionHandler {
    fn kind(&self) -> &'static str { "PrismTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PrismTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for GlitterTransitionHandler {
    fn kind(&self) -> &'static str { "GlitterTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        GlitterTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for FlyThroughTransitionHandler {
    fn kind(&self) -> &'static str { "FlyThroughTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        FlyThroughTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for LeftRightDirectionTransitionHandler {
    fn kind(&self) -> &'static str { "LeftRightDirectionTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        LeftRightDirectionTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for OrientationTransitionHandler {
    fn kind(&self) -> &'static str { "OrientationTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        OrientationTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for EightDirectionTransitionHandler {
    fn kind(&self) -> &'static str { "EightDirectionTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        EightDirectionTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for CornerDirectionTransitionHandler {
    fn kind(&self) -> &'static str { "CornerDirectionTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        CornerDirectionTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for SideDirectionTransitionHandler {
    fn kind(&self) -> &'static str { "SideDirectionTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SideDirectionTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for InOutTransitionHandler {
    fn kind(&self) -> &'static str { "InOutTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        InOutTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for SplitTransitionHandler {
    fn kind(&self) -> &'static str { "SplitTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SplitTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for WheelTransitionHandler {
    fn kind(&self) -> &'static str { "WheelTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        WheelTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}


impl ElementHandler for OptinalBlackTransitionHandler {
    fn kind(&self) -> &'static str { "OptinalBlackTransitionHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        OptinalBlackTransitionHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

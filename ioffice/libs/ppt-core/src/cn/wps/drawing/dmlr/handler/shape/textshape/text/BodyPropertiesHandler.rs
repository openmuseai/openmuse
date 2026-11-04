mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}

// Java: import cn.wps.drawing.data.geometry.GeomGuide
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextBodyProperties;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.geometry.AdjustValueListHanlder
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.threeD.Scene3DHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.threeD.Shape3DHandler
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.moffice.drawing.geotext.PresetTextWarp
use std::cell::RefCell;
use std::rc::Rc;

pub use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextBodyProperties as BodyPrModel;

// java public class BodyPropertiesHandler extends DefaultElementHandler
pub struct BodyPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextBodyProperties bodyProperties;
    bodyProperties: Rc<RefCell<TextBodyProperties>>,
    // java private HandlerCallback handlerCallback;
    #[allow(dead_code)]
    handlerCallback: Option<Rc<RefCell<HandlerCallback>>>,
}

// java  class NormalAutoFitHandler extends DefaultElementHandler
pub struct NormalAutoFitHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextBodyProperties.Autofit autofit;
    autofit: Rc<RefCell<TextBodyProperties>>,
}

// java  class PresetTextWrapHandler extends DefaultElementHandler
pub struct PresetTextWrapHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextBodyProperties.PresetTextWarp presetTextWarp;
    presetTextWarp: i32,
    // java private List<GeomGuide> adjList;
    adjList: Option<Vec<i32>>,
}

// java  class FlatTextHandler extends DefaultElementHandler
pub struct FlatTextHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    bodyProperties: Rc<RefCell<TextBodyProperties>>,
}

// java public class BodyPropertiesHandler extends DefaultElementHandler
impl BodyPropertiesHandler {
    // java public  BodyPropertiesHandler(TextBodyProperties bodyProperties, HandlerCallback handlerCallback)
    pub fn BodyPropertiesHandler(bodyProperties: Rc<RefCell<TextBodyProperties>>, handlerCallback: Option<Rc<RefCell<HandlerCallback>>>) -> Self {
      // java this.bodyProperties = bodyProperties;
      Self { super_: DefaultElementHandler::default(), bodyProperties: bodyProperties, handlerCallback: handlerCallback }
      // java this.handlerCallback = handlerCallback;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java int count = attrsNotCopy.count();
      let count: i32 = attrsNotCopy.count();
      // java XmlRoAttr tmp = null;
      let mut tmp = None;
      let mut i: i32 = 0;
      // java for (int i = 0; i < count ++i)
      while i < count {
        // java tmp = attrsNotCopy.getAttrAt(i);
        tmp = Some(attrsNotCopy.get_attr_at(i as usize));
        // java if (tmp != null)
        if let Some(tmp) = tmp {
          // java int attrName = tmp.getName();
          let attrName: i32 = tmp.get_name();
          // java switch (attrName)
          match attrName {
            // java case 1114201:
            1114201 => {
              // java this.bodyProperties.putRotation(tmp.toInt());
              self.bodyProperties.borrow_mut().putRotation(tmp.to_int());
            }
            // java case 1114301:
            1114301 => {
              // java this.bodyProperties.putAnchor((Integer)DGMap.textAnchoringTypeMap.get(tmp.toString()));
              if let Some(v) = DGMap::textAnchoringTypeMap().get(tmp.to_string().as_str()) {
                self.bodyProperties.borrow_mut().putAnchor(*v);
              }
            }
            // java case 1114480:
            1114480 => {
              // java this.bodyProperties.putVerticalText((Integer)DGMap.textVerticalTypeMap.get(tmp.toString()));
              if let Some(v) = DGMap::textVerticalTypeMap().get(tmp.to_string().as_str()) {
                self.bodyProperties.borrow_mut().putVerticalText(*v);
              }
            }
            // java case 1114481:
            1114481 => {
              // java this.bodyProperties.putAnchorCenter(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putAnchorCenter(tmp.to_boolean());
            }
            // java case 1114482:
            1114482 => {
              // java this.bodyProperties.putTextHorizontalOverflow((Integer)DGMap.textHorzOverflowTypeMap.get(tmp.toString()));
              if let Some(v) = DGMap::textHorzOverflowTypeMap().get(tmp.to_string().as_str()) {
                self.bodyProperties.borrow_mut().putTextHorizontalOverflow(*v);
              }
            }
            // java case 1114548:
            1114548 => {
              // java this.bodyProperties.putSpcFirstLastPara(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putSpcFirstLastPara(tmp.to_boolean());
            }
            // java case 1114549:
            1114549 => {
              // java this.bodyProperties.putTextVerticalOverflow((Integer)DGMap.textVerOverflowMap.get(tmp.toString()));
              if let Some(v) = DGMap::textVerOverflowMap().get(tmp.to_string().as_str()) {
                self.bodyProperties.borrow_mut().putTextVerticalOverflow(*v);
              }
            }
            // java case 1114550:
            1114550 => {
              // java this.bodyProperties.putTextWrappingType((Integer)DGMap.textWrapMap.get(tmp.toString()));
              if let Some(v) = DGMap::textWrapMap().get(tmp.to_string().as_str()) {
                self.bodyProperties.borrow_mut().putTextWrappingType(*v);
              }
            }
            // java case 1114551:
            1114551 => {
              // java this.bodyProperties.putLeftInset(tmp.toInt());
              self.bodyProperties.borrow_mut().putLeftInset(tmp.to_int());
            }
            // java case 1114552:
            1114552 => {
              // java this.bodyProperties.putTopInset(tmp.toInt());
              self.bodyProperties.borrow_mut().putTopInset(tmp.to_int());
            }
            // java case 1114553:
            1114553 => {
              // java this.bodyProperties.putRightInset(tmp.toInt());
              self.bodyProperties.borrow_mut().putRightInset(tmp.to_int());
            }
            // java case 1114554:
            1114554 => {
              // java this.bodyProperties.putBottomInset(tmp.toInt());
              self.bodyProperties.borrow_mut().putBottomInset(tmp.to_int());
            }
            // java case 1114555:
            1114555 => {
              // java this.bodyProperties.putNumberOfColumns(tmp.toInt());
              self.bodyProperties.borrow_mut().putNumberOfColumns(tmp.to_int());
            }
            // java case 1114556:
            1114556 => {
              // java this.bodyProperties.putSpaceBetweenColumns(tmp.toInt());
              self.bodyProperties.borrow_mut().putSpaceBetweenColumns(tmp.to_int());
            }
            // java case 1114557:
            1114557 => {
              // java this.bodyProperties.putRight2LeftColumns(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putRight2LeftColumns(tmp.to_boolean());
            }
            // java case 1114558:
            1114558 => {
              // java this.bodyProperties.putFromWordArt(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putFromWordArt(tmp.to_boolean());
            }
            // java case 1114559:
            1114559 => {
              // java this.bodyProperties.putForceAntiAlias(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putForceAntiAlias(tmp.to_boolean());
            }
            // java case 1114560:
            1114560 => {
              // java this.bodyProperties.putTextUpright(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putTextUpright(tmp.to_boolean());
            }
            // java case 1114561:
            1114561 => {
              // java this.bodyProperties.putCompatibleLineSpacing(tmp.toBoolean());
              self.bodyProperties.borrow_mut().putCompatibleLineSpacing(tmp.to_boolean());
            }
            // java default:
            _ => {
              // rust: no-stmt
            }
          }
        }
        i += 1;
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java switch (subElement)
      match subElement {
        // java case 1114144:
        1114144 => {
          // java return new Scene3DHandler(this.bodyProperties.scene3d());
          return None;
        }
        // java case 1114145:
        1114145 => {
          // java return new Shape3DHandler(this.bodyProperties.sp3d(), this.handlerCallback);
          return None;
        }
        // java case 1114313:
        1114313 => {
          // java return new FlatTextHandler();
          return Some(Box::new(FlatTextHandler { super_: DefaultElementHandler::default(), bodyProperties: self.bodyProperties.clone() }));
        }
        // java case 1114430:
        1114430 => {
          // java return new PresetTextWrapHandler(this.bodyProperties.presetTextWarp());
          return None;
        }
        // java case 1114545:
        1114545 => {
          // java this.bodyProperties.textAutofit().putType(0);
          self.bodyProperties.borrow_mut().textAutofit().putType(0);
          // java return null;
          return None;
        }
        // java case 1114546:
        1114546 => {
          // java this.bodyProperties.textAutofit().putType(1);
          self.bodyProperties.borrow_mut().textAutofit().putType(1);
          // java return new NormalAutoFitHandler(this.bodyProperties.textAutofit());
          return Some(Box::new(NormalAutoFitHandler::NormalAutoFitHandler(self.bodyProperties.clone())));
        }
        // java case 1114547:
        1114547 => {
          // java this.bodyProperties.textAutofit().putType(2);
          self.bodyProperties.borrow_mut().textAutofit().putType(2);
          // java return null;
          return None;
        }
        // java case 3211295:
        3211295 => {
          // java return null;
          return None;
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
    }

}

impl ElementHandler for BodyPropertiesHandler {
    fn kind(&self) -> &'static str { "BodyPropertiesHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        BodyPropertiesHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        BodyPropertiesHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

// java  class NormalAutoFitHandler extends DefaultElementHandler
impl NormalAutoFitHandler {
    // java public  NormalAutoFitHandler(TextBodyProperties.Autofit autofit)
    pub fn NormalAutoFitHandler(autofit: Rc<RefCell<TextBodyProperties>>) -> Self {
      // java this.autofit = autofit;
      Self { super_: DefaultElementHandler::default(), autofit: autofit }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = null;
      let mut tmp = None;
      // java if ((tmp = attrsNotCopy.getByName(1114543)) != null)
      tmp = attrsNotCopy.get_by_name(1114543);
      if tmp.is_some() {
        // java this.autofit.putFontScale(tmp.toPercentage());
        self.autofit.borrow_mut().textAutofit().putFontScale(tmp.unwrap().to_percentage());
      }
      // java if ((tmp = attrsNotCopy.getByName(1114544)) != null)
      tmp = attrsNotCopy.get_by_name(1114544);
      if tmp.is_some() {
        // java this.autofit.putLnSpcReduction(tmp.toPercentage());
        self.autofit.borrow_mut().textAutofit().putLnSpcReduction(tmp.unwrap().to_percentage());
      }
    }

}

impl ElementHandler for NormalAutoFitHandler {
    fn kind(&self) -> &'static str { "NormalAutoFitHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        NormalAutoFitHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

// java  class PresetTextWrapHandler extends DefaultElementHandler
impl PresetTextWrapHandler {
    // java public  PresetTextWrapHandler(TextBodyProperties.PresetTextWarp presetTextWarp)
    pub fn PresetTextWrapHandler(_presetTextWarp: i32) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::default(), presetTextWarp: 0, adjList: None }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      None
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      */
    }

}

// java  class FlatTextHandler extends DefaultElementHandler
impl FlatTextHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = attrsNotCopy.getByName(1114206);
      let tmp = attrsNotCopy.get_by_name(1114206);
      // java if (tmp != null)
      if tmp.is_some() {
        // java BodyPropertiesHandler.this.bodyProperties.putFlatTx(tmp.toInt());
        self.bodyProperties.borrow_mut().putFlatTx(tmp.unwrap().to_int());
      }
      // java else
      else {
        // java BodyPropertiesHandler.this.bodyProperties.putFlatTx(0);
        self.bodyProperties.borrow_mut().putFlatTx(0);
      }
    }

}

impl ElementHandler for FlatTextHandler {
    fn kind(&self) -> &'static str { "FlatTextHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        FlatTextHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

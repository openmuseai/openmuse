mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}

// Java: import cn.wps.drawing.data.text.TextCharacterProperties
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextCharacterProperties;
// Java: import cn.wps.drawing.dmlr.handler.color.ColorHandler
// Java: import cn.wps.drawing.dmlr.handler.font.TextFontHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::FontMarkHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.hyperlink.HyperLinkHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.effect.EffectContainerHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.effect.EffectListHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.fill.FillPropertiesHandler
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.outline.OutlineHandler
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
use std::cell::RefCell;
use std::rc::Rc;

// java public class TextCharacterPropertiesHandler extends DefaultElementHandler
pub struct TextCharacterPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextCharacterProperties runProp;
    runProp: Rc<RefCell<TextCharacterProperties>>,
    // java private HandlerCallback handlerCallback = null;
    handlerCallback: Option<Rc<RefCell<HandlerCallback>>>,
}

// java  class RightToLeftHandler extends DefaultElementHandler
pub struct RightToLeftHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    runProp: Rc<RefCell<TextCharacterProperties>>,
}

// java public class TextCharacterPropertiesHandler extends DefaultElementHandler
impl TextCharacterPropertiesHandler {
    // java public  TextCharacterPropertiesHandler(TextCharacterProperties runProp, HandlerCallback handlerCallback)
    pub fn TextCharacterPropertiesHandler(runProp: Rc<RefCell<TextCharacterProperties>>, handlerCallback: Option<Rc<RefCell<HandlerCallback>>>) -> Self {
      // java this.runProp = runProp;
      Self { super_: DefaultElementHandler::default(), runProp: runProp, handlerCallback: handlerCallback }
      // java this.handlerCallback = handlerCallback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java switch (subElement)
      match subElement {
        // java case 1114140:
        1114140 => {
          // java return new TextFontHandler(this.runProp.latinFont());
          self.runProp.borrow_mut().latinFont();
          return Some(Box::new(FontMarkHandler { kind_name: "TextFontHandler" }));
        }
        // java case 1114141:
        1114141 => {
          // java return new TextFontHandler(this.runProp.eastAsianFont());
          self.runProp.borrow_mut().eastAsianFont();
          return Some(Box::new(FontMarkHandler { kind_name: "TextFontHandler" }));
        }
        // java case 1114142:
        1114142 => {
          // java return new TextFontHandler(this.runProp.complexScriptFont());
          self.runProp.borrow_mut().complexScriptFont();
          return Some(Box::new(FontMarkHandler { kind_name: "TextFontHandler" }));
        }
        // java case 1114148:
        1114148 => {
          // java return new OutlineHandler(this.runProp.outline(), this.handlerCallback);
          return None;
        }
        // java case 1114243:
        1114243 => {
          // java return new HyperLinkHandler(this.runProp.hlinkClick(), this.handlerCallback);
          return None;
        }
        // java case 1114370: case 1114371: case 1114372: case 1114373: case 1114374: case 3211586:
        1114370 | 1114371 | 1114372 | 1114373 | 1114374 | 3211586 => {
          // java return (new FillPropertiesHandler(this.runProp.fill(), this.handlerCallback)).enterSubElement(subElement);
          return None;
        }
        // java case 1114389:
        1114389 => {
          // java this.runProp.effect().putType(0);
          // rust: no-stmt
          // java return new EffectListHandler(this.runProp.effect().effectLst(), this.handlerCallback);
          return None;
        }
        // java case 1114390:
        1114390 => {
          // java this.runProp.effect().putType(1);
          // rust: no-stmt
          // java return new EffectContainerHandler(this.runProp.effect().effectDag(), this.handlerCallback);
          return None;
        }
        // java case 1114493:
        1114493 => {
          // java return new RightToLeftHandler();
          return Some(Box::new(RightToLeftHandler { super_: DefaultElementHandler::default(), runProp: self.runProp.clone() }));
        }
        // java case 1114579:
        1114579 => {
          // java this.runProp.putUnderlineFollowText(true);
          self.runProp.borrow_mut().putUnderlineFollowText(true);
          // java return null;
          return None;
        }
        // java case 1114580:
        1114580 => {
          // java return new OutlineHandler(this.runProp.underlineStroke(), this.handlerCallback);
          return None;
        }
        // java case 1114581:
        1114581 => {
          // java this.runProp.putUnderlineFillFollowText(true);
          self.runProp.borrow_mut().putUnderlineFillFollowText(true);
          // java return null;
          return None;
        }
        // java case 1114582:
        1114582 => {
          // java return new FillPropertiesHandler(this.runProp.underlineFill(), this.handlerCallback);
          return None;
        }
        // java case 1114583:
        1114583 => {
          // java return new ColorHandler(this.runProp.highlight(), this.handlerCallback);
          return None;
        }
        // java case 1114584:
        1114584 => {
          // java return new TextFontHandler(this.runProp.symbolFont());
          self.runProp.borrow_mut().symbolFont();
          return Some(Box::new(FontMarkHandler { kind_name: "TextFontHandler" }));
        }
        // java case 1114585:
        1114585 => {
          // java return new HyperLinkHandler(this.runProp.hlinkMouseOver(), this.handlerCallback);
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

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = null;
      let mut tmp = None;
      // java int count = attrsNotCopy.count();
      let mut count: i32 = attrsNotCopy.count();
      // java int attrName = 0;
      let mut attrName: i32 = 0;
      let mut i: i32 = 0;
      // java for (int i = 0; i < count ++i)
      while i < count {
        // java if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
        tmp = Some(attrsNotCopy.get_attr_at(i as usize));
        if let Some(tmp) = tmp {
          // java attrName = tmp.getName();
          attrName = tmp.get_name();
          // java switch (attrName)
          match attrName {
            // java case 1114195:
            1114195 => {
              // java this.runProp.options().putBold(tmp.toBoolean());
              self.runProp.borrow_mut().options().putBold(tmp.to_boolean());
            }
            // java case 1114441:
            1114441 => {
              // java this.runProp.putCap((Integer)DGMap.textCapMap.get(tmp.toString()));
              if let Some(v) = DGMap::textCapMap().get(tmp.to_string().as_str()) {
                self.runProp.borrow_mut().putCap(*v);
              }
            }
            // java case 1114504:
            1114504 => {
              // java this.runProp.options().putItalics(tmp.toBoolean());
              self.runProp.borrow_mut().options().putItalics(tmp.to_boolean());
            }
            // java case 1114586:
            1114586 => {
              // java this.runProp.options().putKumimoji(tmp.toBoolean());
              self.runProp.borrow_mut().options().putKumimoji(tmp.to_boolean());
            }
            // java case 1114587:
            1114587 => {
              // java this.runProp.putLang(tmp.toString());
              self.runProp.borrow_mut().putLang(tmp.to_string());
            }
            // java case 1114588:
            1114588 => {
              // java this.runProp.putAltLang(tmp.toString());
              self.runProp.borrow_mut().putAltLang(tmp.to_string());
            }
            // java case 1114589:
            1114589 => {
              // java this.runProp.putFontSize(tmp.toInt());
              self.runProp.borrow_mut().putFontSize(tmp.to_int());
            }
            // java case 1114590:
            1114590 => {
              // java this.runProp.putUnderline((Integer)DGMap.textUnderlineMap.get(tmp.toString()));
              if let Some(v) = DGMap::textUnderlineMap().get(tmp.to_string().as_str()) {
                self.runProp.borrow_mut().putUnderline(*v);
              }
            }
            // java case 1114591:
            1114591 => {
              // java this.runProp.putStrike((Integer)DGMap.textStrikeMap.get(tmp.toString()));
              if let Some(v) = DGMap::textStrikeMap().get(tmp.to_string().as_str()) {
                self.runProp.borrow_mut().putStrike(*v);
              }
            }
            // java case 1114592:
            1114592 => {
              // java this.runProp.putKern(tmp.toInt());
              self.runProp.borrow_mut().putKern(tmp.to_int());
            }
            // java case 1114593:
            1114593 => {
              // java this.runProp.putSpacing(tmp.toInt());
              self.runProp.borrow_mut().putSpacing(tmp.to_int());
            }
            // java case 1114594:
            1114594 => {
              // java this.runProp.options().putNormalizeH(tmp.toBoolean());
              self.runProp.borrow_mut().options().putNormalizeH(tmp.to_boolean());
            }
            // java case 1114595:
            1114595 => {
              // java this.runProp.putBaseline(tmp.toPercentage());
              self.runProp.borrow_mut().putBaseline(tmp.to_percentage());
            }
            // java case 1114596:
            1114596 => {
              // java this.runProp.options().putNoProof(tmp.toBoolean());
              self.runProp.borrow_mut().options().putNoProof(tmp.to_boolean());
            }
            // java case 1114597:
            1114597 => {
              // java this.runProp.options().putDirty(tmp.toBoolean());
              self.runProp.borrow_mut().options().putDirty(tmp.to_boolean());
            }
            // java case 1114598:
            1114598 => {
              // java this.runProp.options().putSpellingErr(tmp.toBoolean());
              self.runProp.borrow_mut().options().putSpellingErr(tmp.to_boolean());
            }
            // java case 1114599:
            1114599 => {
              // java this.runProp.options().putSmtClean(tmp.toBoolean());
              self.runProp.borrow_mut().options().putSmtClean(tmp.to_boolean());
            }
            // java case 1114600:
            1114600 => {
              // java this.runProp.putSmtId(tmp.toInt());
              self.runProp.borrow_mut().putSmtId(tmp.to_int());
            }
            // java case 1114601:
            1114601 => {
              // java this.runProp.putBookmarkLinkTarget(tmp.toString());
              self.runProp.borrow_mut().putBookmarkLinkTarget(tmp.to_string());
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

}

impl ElementHandler for TextCharacterPropertiesHandler {
    fn kind(&self) -> &'static str { "TextCharacterPropertiesHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        TextCharacterPropertiesHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextCharacterPropertiesHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

// java  class RightToLeftHandler extends DefaultElementHandler
impl RightToLeftHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = attrsNotCopy.getByName(1114158);
      let tmp = attrsNotCopy.get_by_name(1114158);
      // java if (tmp != null && tmp.toBoolean())
      if tmp.is_some() && tmp.unwrap().to_boolean() {
        // java TextCharacterPropertiesHandler.this.runProp.options().putRightToLeft(tmp.toBoolean());
        self.runProp.borrow_mut().options().putRightToLeft(tmp.unwrap().to_boolean());
      }
      // java else
      else {
        // java TextCharacterPropertiesHandler.this.runProp.options().putRightToLeft(false);
        self.runProp.borrow_mut().options().putRightToLeft(false);
      }
    }

}

impl ElementHandler for RightToLeftHandler {
    fn kind(&self) -> &'static str { "RightToLeftHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        RightToLeftHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

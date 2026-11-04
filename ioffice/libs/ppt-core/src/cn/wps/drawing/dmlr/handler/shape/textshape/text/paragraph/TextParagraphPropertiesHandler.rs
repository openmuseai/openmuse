mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextParagraphProperties {
        pub slot: i32,
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextFontHandler {}
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextCharacterPropertiesHandler {}
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextSpacing {}
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextTabStop {}
}

// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use compile_surface::TextParagraphProperties as _TextParagraphPropertiesImport;
pub use compile_surface::TextParagraphProperties;
// Java: import cn.wps.drawing.dmlr.handler.color.ColorHandlerFactory
use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
// Java: import cn.wps.drawing.dmlr.handler.font.TextFontHandler
use compile_surface::TextFontHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.fill.BlipHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::BlipHandler::BlipHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.run.TextCharacterPropertiesHandler
use compile_surface::TextCharacterPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::{HandlerCallback, HandlerCallbackState};
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import java.util.ArrayList
use std::vec::Vec;
// Java: import java.util.List
use std::vec::Vec as List;

use compile_surface::{TextSpacing, TextTabStop};
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// java public class TextParagraphPropertiesHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct TextParagraphPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextParagraphProperties textProp;
    textProp: TextParagraphProperties,
    // java private HandlerCallback handlerCallback;
    handlerCallback: HandlerCallbackState,
    pub init_count: i32,
}

// java  class AutoNumberedBulletHandler extends DefaultElementHandler
#[derive(Default)]
pub struct AutoNumberedBulletHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class PictureBulletHandler extends DefaultElementHandler
#[derive(Default)]
pub struct PictureBulletHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class CharacterBulletHandler extends DefaultElementHandler
#[derive(Default)]
pub struct CharacterBulletHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class BulletColorHandler extends DefaultElementHandler
#[derive(Default)]
pub struct BulletColorHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class BulletSizePercentageHandler extends DefaultElementHandler
#[derive(Default)]
pub struct BulletSizePercentageHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class BulletSizePointsHandler extends DefaultElementHandler
#[derive(Default)]
pub struct BulletSizePointsHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java private class LineSpaceHandler extends DefaultElementHandler
#[derive(Default)]
pub struct LineSpaceHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextParagraphProperties.TextSpacing textSpacing;
    textSpacing: TextSpacing,
}

// java private class SpacingPercent extends DefaultElementHandler
#[derive(Default)]
pub struct SpacingPercent {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java private class SpacingPoints extends DefaultElementHandler
#[derive(Default)]
pub struct SpacingPoints {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class TabStopListHandler extends DefaultElementHandler
#[derive(Default)]
pub struct TabStopListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<TextParagraphProperties.TextTabStop> tabStopLst = new ArrayList();
    tabStopLst: Vec<TextTabStop>,
}

// java private class TabStopHandler extends DefaultElementHandler
#[derive(Default)]
pub struct TabStopHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TextParagraphProperties.TextTabStop tabStop;
    tabStop: TextTabStop,
}

// java public class TextParagraphPropertiesHandler extends DefaultElementHandler
impl TextParagraphPropertiesHandler {
    // java public  TextParagraphPropertiesHandler(TextParagraphProperties textProp, HandlerCallback handlerCallback)
    pub fn TextParagraphPropertiesHandler(textProp: TextParagraphProperties, handlerCallback: HandlerCallbackState) -> Self {
      let mut this = Self {
        super_: DefaultElementHandler {},
        textProp: TextParagraphProperties::default(),
        handlerCallback,
        init_count: 0,
      };
      // java this.init(textProp);
      this.init(textProp);
      // java this.handlerCallback = handlerCallback;
      this.handlerCallback = handlerCallback;
      this
    }

    // java public void init(TextParagraphProperties textProp)
    pub fn init(&mut self, textProp: TextParagraphProperties) {
      // java this.textProp = textProp;
      self.textProp = textProp;
      self.init_count += 1;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int count = attrsNotCopy.count();
      let mut count: i32 = attrsNotCopy.count();
      // java XmlRoAttr tmp = null;
      let mut tmp: Box<dyn XmlRoAttr> = Default::default();
      // java int attrName = 0;
      let mut attrName: i32 = 0;
      let mut i: i32 = 0;
      // java for (int i = 0; i < count ++i)
      while i < count {
        // java if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
        if { tmp = attrsNotCopy.getAttrAt(i); tmp }.is_some() {
          // java attrName = tmp.getName();
          attrName = tmp.getName();
          // java switch (attrName)
          match attrName {
            // java case 1114328:
            1114328 => {
              // java this.textProp.putAlignment((Integer)DGMap.textHorzAlignmentMap.get(tmp.toString()));
              self.textProp.putAlignment(((DGMap::textHorzAlignmentMap.get(tmp.toString())) as i32));
            }
            // java case 1114476:
            1114476 => {
              // java this.textProp.putMarginLeft(tmp.toInt());
              self.textProp.putMarginLeft(tmp.toInt());
            }
            // java case 1114477:
            1114477 => {
              // java this.textProp.putMarginRight(tmp.toInt());
              self.textProp.putMarginRight(tmp.toInt());
            }
            // java case 1114493:
            1114493 => {
              // java this.textProp.options().putRightToLeft(tmp.toBoolean());
              self.textProp.options().putRightToLeft(tmp.toBoolean());
            }
            // java case 1114611:
            1114611 => {
              // java this.textProp.putLvl(tmp.toInt());
              self.textProp.putLvl(tmp.toInt());
            }
            // java case 1114612:
            1114612 => {
              // java this.textProp.putIndent(tmp.toInt());
              self.textProp.putIndent(tmp.toInt());
            }
            // java case 1114613:
            1114613 => {
              // java this.textProp.putDefTabSize(tmp.toInt());
              self.textProp.putDefTabSize(tmp.toInt());
            }
            // java case 1114614:
            1114614 => {
              // java this.textProp.options().putEaLnBrk(tmp.toBoolean());
              self.textProp.options().putEaLnBrk(tmp.toBoolean());
            }
            // java case 1114615:
            1114615 => {
              // java this.textProp.putFontAlgn((Integer)DGMap.textFontAlignTypeMap.get(tmp.toString()));
              self.textProp.putFontAlgn(((DGMap::textFontAlignTypeMap.get(tmp.toString())) as i32));
            }
            // java case 1114616:
            1114616 => {
              // java this.textProp.options().putLatinLnBrk(tmp.toBoolean());
              self.textProp.options().putLatinLnBrk(tmp.toBoolean());
            }
            // java case 1114617:
            1114617 => {
              // java this.textProp.options().putHangingPunct(tmp.toBoolean());
              self.textProp.options().putHangingPunct(tmp.toBoolean());
            }
            // java default:
            _ => {
            }
          }
        }
      }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (subElement)
      match subElement {
        // java case 1114563:
        1114563 => {
          // java this.textProp.options().putBuClrFollowTx(true);
          self.textProp.options().putBuClrFollowTx(true);
          // java return null;
          return None;
        }
        // java case 1114564:
        1114564 => {
          // java return new BulletColorHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114565:
        1114565 => {
          // java this.textProp.options().putBuSzFollowTx(true);
          self.textProp.options().putBuSzFollowTx(true);
          // java return null;
          return None;
        }
        // java case 1114566:
        1114566 => {
          // java return new BulletSizePercentageHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114567:
        1114567 => {
          // java return new BulletSizePointsHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114568:
        1114568 => {
          // java this.textProp.options().putBuFontFollowTx(true);
          self.textProp.options().putBuFontFollowTx(true);
          // java return null;
          return None;
        }
        // java case 1114569:
        1114569 => {
          // java return new TextFontHandler(this.textProp.buFont());
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114572:
        1114572 => {
          // java this.textProp.options().putBuNone(true);
          self.textProp.options().putBuNone(true);
          // java return null;
          return None;
        }
        // java case 1114573:
        1114573 => {
          // java return new AutoNumberedBulletHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114574:
        1114574 => {
          // java return new CharacterBulletHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114575:
        1114575 => {
          // java return new PictureBulletHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114606:
        1114606 => {
          // java return new LineSpaceHandler(this.textProp.lnSpc());
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114607:
        1114607 => {
          // java return new LineSpaceHandler(this.textProp.spcBef());
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114608:
        1114608 => {
          // java return new LineSpaceHandler(this.textProp.spcAft());
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114609:
        1114609 => {
          // java return new TabStopListHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114610:
        1114610 => {
          // java return new TextCharacterPropertiesHandler(this.textProp.defRPr(), this.handlerCallback);
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
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
      */
    }

}

impl std::ops::Deref for TextParagraphPropertiesHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextParagraphPropertiesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class AutoNumberedBulletHandler extends DefaultElementHandler
impl AutoNumberedBulletHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int count = attrsNotCopy.count();
      let mut count: i32 = attrsNotCopy.count();
      // java XmlRoAttr tmp = null;
      let mut tmp: Box<dyn XmlRoAttr> = Default::default();
      // java int attrName = 0;
      let mut attrName: i32 = 0;
      let mut i: i32 = 0;
      // java for (int i = 0; i < count ++i)
      while i < count {
        // java if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
        if { tmp = attrsNotCopy.getAttrAt(i); tmp }.is_some() {
          // java attrName = tmp.getName();
          attrName = tmp.getName();
          // java switch (attrName)
          match attrName {
            // java case 1114388:
            1114388 => {
              // java TextParagraphPropertiesHandler.this.textProp.buAutonNum().putType((Integer)DGMap.textAutonumberSchemeMap.get(tmp.toString()));
              self.textProp.buAutonNum().putType(((DGMap::textAutonumberSchemeMap.get(tmp.toString())) as i32));
            }
            // java case 1114570:
            1114570 => {
              // java TextParagraphPropertiesHandler.this.textProp.buAutonNum().putStartAt(tmp.toInt());
              self.textProp.buAutonNum().putStartAt(tmp.toInt());
            }
            // java default:
            _ => {
            }
          }
        }
      }
      */
    }

}

impl std::ops::Deref for AutoNumberedBulletHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for AutoNumberedBulletHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class PictureBulletHandler extends DefaultElementHandler
impl PictureBulletHandler {
    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 1114365:
        1114365 => {
          // java handler = new BlipHandler(TextParagraphPropertiesHandler.this.textProp.buBlip(), TextParagraphPropertiesHandler.this.handlerCallback);
          handler = BlipHandler::new(self.textProp.buBlip(), self.handlerCallback);
        }
        // java default:
        _ => {
          // java return handler;
          return Some(handler);
        }
      }
      */
    }

}

impl std::ops::Deref for PictureBulletHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PictureBulletHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CharacterBulletHandler extends DefaultElementHandler
impl CharacterBulletHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = attrsNotCopy.getByName(1114571);
      let mut tmp: Box<dyn XmlRoAttr> = attrsNotCopy.getByName__int(1114571);
      // java if (tmp != null)
      if true {
        // java if (tmp.toString().length() > 0)
        if (tmp.toString().len() as i32) > 0 {
          // java TextParagraphPropertiesHandler.this.textProp.putBuChar(String.valueOf(tmp.toString().charAt(0)));
          self.textProp.putBuChar(String::valueOf(tmp.toString().charAt(0)));
        }
      }
      */
    }

}

impl std::ops::Deref for CharacterBulletHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CharacterBulletHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class BulletColorHandler extends DefaultElementHandler
impl BulletColorHandler {
    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ColorHandlerFactory.createColorHandler(subElement, TextParagraphPropertiesHandler.this.textProp.buClr(), TextParagraphPropertiesHandler.this.handlerCallback, 1114564);
      return Some(ColorHandlerFactory::createColorHandler(subElement, self.textProp.buClr(), self.handlerCallback, 1114564));
      */
    }

}

impl std::ops::Deref for BulletColorHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BulletColorHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class BulletSizePercentageHandler extends DefaultElementHandler
impl BulletSizePercentageHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = attrsNotCopy.getByName(1114158);
      let mut tmp: Box<dyn XmlRoAttr> = attrsNotCopy.getByName__int(1114158);
      // java if (tmp != null)
      if true {
        // java TextParagraphPropertiesHandler.this.textProp.putBuSzPct((double)tmp.toInt());
        self.textProp.putBuSzPct(((tmp.toInt()) as f64));
      }
      */
    }

}

impl std::ops::Deref for BulletSizePercentageHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BulletSizePercentageHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class BulletSizePointsHandler extends DefaultElementHandler
impl BulletSizePointsHandler {
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = attrsNotCopy.getByName(1114158);
      let mut tmp: Box<dyn XmlRoAttr> = attrsNotCopy.getByName__int(1114158);
      // java if (tmp != null)
      if true {
        // java TextParagraphPropertiesHandler.this.textProp.putBuSzPts(tmp.toInt());
        self.textProp.putBuSzPts(tmp.toInt());
      }
      */
    }

}

impl std::ops::Deref for BulletSizePointsHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BulletSizePointsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class LineSpaceHandler extends DefaultElementHandler
impl LineSpaceHandler {
    // java public  LineSpaceHandler(TextParagraphProperties.TextSpacing textSpacing)
    pub fn LineSpaceHandler(textSpacing: TextSpacing) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.textSpacing = textSpacing;
      Self { super_: DefaultElementHandler::new(), textSpacing: textSpacing }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (subElement)
      match subElement {
        // java case 1114604:
        1114604 => {
          // java return new SpacingPercent();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 1114605:
        1114605 => {
          // java return new SpacingPoints();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

}

impl std::ops::Deref for LineSpaceHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LineSpaceHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SpacingPercent extends DefaultElementHandler
impl SpacingPercent {
    // java private  SpacingPercent()
    pub fn SpacingPercent() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = null;
      let mut tmp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((tmp = attrsNotCopy.getByName(1114158)) != null)
      if { tmp = attrsNotCopy.getByName__int(1114158); tmp }.is_some() {
        // java LineSpaceHandler.this.textSpacing.putSpcPct(tmp.toPercentage());
        self.textSpacing.putSpcPct(tmp.toPercentage());
      }
      */
    }

}

impl std::ops::Deref for SpacingPercent {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SpacingPercent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SpacingPoints extends DefaultElementHandler
impl SpacingPoints {
    // java private  SpacingPoints()
    pub fn SpacingPoints() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = null;
      let mut tmp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((tmp = attrsNotCopy.getByName(1114158)) != null)
      if { tmp = attrsNotCopy.getByName__int(1114158); tmp }.is_some() {
        // java LineSpaceHandler.this.textSpacing.putSpcPts(tmp.toDouble());
        self.textSpacing.putSpcPts(tmp.toDouble());
      }
      */
    }

}

impl std::ops::Deref for SpacingPoints {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SpacingPoints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class TabStopListHandler extends DefaultElementHandler
impl TabStopListHandler {
    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return subElement == 1114602 && this.tabStopLst.size() <= 32 ? new TabStopHandler() : null;
      return Some((if subElement == 1114602 && (self.tabStopLst.len() as i32) <= 32 { Some(TabStopHandler::new()) } else { None }));
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java for (TextParagraphProperties.TextTabStop tabStop : this.tabStopLst)
      for tabStop in self.tabStopLst.iter() {
        // java tabStop.save();
        tabStop.save();
      }
      // java TextParagraphPropertiesHandler.this.textProp.tabStopLst().write(this.tabStopLst);
      self.textProp.tabStopLst().write(self.tabStopLst);
      */
    }

}

impl std::ops::Deref for TabStopListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TabStopListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class TabStopHandler extends DefaultElementHandler
impl TabStopHandler {
    // java private  TabStopHandler()
    pub fn TabStopHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.tabStop = new TextParagraphProperties.TextTabStop();
      Self { super_: DefaultElementHandler::new(), tabStop: TextTabStop::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr tmp = null;
      let mut tmp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((tmp = attrsNotCopy.getByName(1114328)) != null && DGMap.tabStopAlgnMap.get(tmp.toString()) != null)
      if { tmp = attrsNotCopy.getByName__int(1114328); tmp }.is_some() && DGMap::tabStopAlgnMap.get(tmp.toString()).is_some() {
        // java this.tabStop.putAlignment((Integer)DGMap.tabStopAlgnMap.get(tmp.toString()));
        self.tabStop.putAlignment(((DGMap::tabStopAlgnMap.get(tmp.toString())) as i32));
      }
      // java if ((tmp = attrsNotCopy.getByName(1114342)) != null)
      if { tmp = attrsNotCopy.getByName__int(1114342); tmp }.is_some() {
        // java this.tabStop.putPos(tmp.toInt());
        self.tabStop.putPos(tmp.toInt());
      }
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TabStopListHandler.this.tabStopLst.add(this.tabStop);
      self.tabStopLst.add(self.tabStop);
      */
    }

}

impl std::ops::Deref for TabStopHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TabStopHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for TextParagraphPropertiesHandler {
    fn kind(&self) -> &'static str {
        "TextParagraphPropertiesHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        TextParagraphPropertiesHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        TextParagraphPropertiesHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

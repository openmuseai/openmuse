#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.drawing.app.text.KmoTextBody
// KmoTextBody module is #[cfg(any())] disabled; using local stub
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::drawing::data::text::TextParagraphProperties::TextParagraphProperties;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.textbox.Text
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;

use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

pub struct KmoTextBody;
impl KmoTextBody {
    pub fn hasText(&self) -> bool { false }
    pub fn hasParagraphList(&self) -> bool { false }
    pub fn seek(&mut self, _cp: i32, _expand: bool) -> bool { false }
    pub fn format(&self, _type: i32) -> Element { None }
}

// Java: public class ShapeText extends Text
pub struct ShapeText {
    pub base: Text,
    // Java field: KmoTextBody kmoTextBody
    pub(crate) kmoTextBody: Option<Arc<Mutex<KmoTextBody>>>,
    // Java field: boolean mHasTypoText
    pub(crate) mHasTypoText: bool,
    // Java field: TextContext mTextContext
    pub(crate) mTextContext: Option<TextContext>,
}

impl Deref for ShapeText {
    type Target = Text;
    fn deref(&self) -> &Self::Target { &self.base }
}

// Java: public class TextContext
pub struct TextContext {
    // Java field: int mPageNumber
    pub(crate) mPageNumber: i32,
}

// Java: public class ShapeText extends Text
impl ShapeText {
    // Java: ShapeText(Shape shape, KmoTextBody kmoTextBody)
    pub fn ShapeText(shape: Element, kmoTextBody: Option<Arc<Mutex<KmoTextBody>>>) -> Self {
      Self::new(shape, kmoTextBody)
    }

    pub fn new(shape: Element, kmoTextBody: Option<Arc<Mutex<KmoTextBody>>>) -> Self {
      // Java: super(shape);
      let base = Text::new_with_shape(shape);
      // Java: this.kmoTextBody = kmoTextBody;
      let mut instance = ShapeText {
          base,
          kmoTextBody,
          mHasTypoText: false,
          mTextContext: None,
      };
      // Java: this.mHasTypoText = this.hasRealText() || this.hasBullets();
      instance.mHasTypoText = instance.hasRealText() || instance.hasBullets();
      instance
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: return this.kmoTextBody != null;
      self.kmoTextBody.is_some()
    }

    // Java: boolean hasTypoText()
    pub fn hasTypoText(&self) -> bool {
      // Java: return this.mHasTypoText;
      self.mHasTypoText
    }

    // Java: boolean hasRealText()
    pub fn hasRealText(&self) -> bool {
      // Java: return this.kmoTextBody != null && this.kmoTextBody.hasText();
      if let Some(ref tb) = self.kmoTextBody {
          let tb = tb.lock().unwrap();
          tb.hasText()
      } else {
          false
      }
    }

    // Java: boolean hasBullets()
    pub fn hasBullets(&self) -> bool {
      // Java: if (null == this.kmoTextBody)
      if self.kmoTextBody.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else if (!this.kmoTextBody.hasParagraphList())
      else if !self.kmoTextBody.as_ref().unwrap().lock().unwrap().hasParagraphList()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: this.kmoTextBody.seek(0, false);
         let mut tb = self.kmoTextBody.as_ref().unwrap().lock().unwrap();
         tb.seek(0, false);
         // Java: KoAr prop = this.kmoTextBody.format(0);
         let prop = tb.format(0);
         // Java: if (null != prop)
         if let Some(ref prop_arc) = prop
         {
            // Java: TextParagraphProperties paraProp = (TextParagraphProperties)prop;
            let paraProp = prop_arc.downcast_ref::<TextParagraphProperties>();
            // Java: if (paraProp.options().hasBuNone())
            if paraProp.map_or(false, |p| p.options().hasBuNone())
            {
               // Java: return false;
               return false;
            }
            // Java: if (paraProp.hasBuChar() || paraProp.hasBuAutonNum() || paraProp.hasBuBlip())
            if paraProp.map_or(false, |p| p.hasBuChar() || p.hasBuAutonNum() || p.hasBuBlip())
            {
               // Java: return true;
               return true;
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: KmoTextBody getKmoTextBody()
    pub fn getKmoTextBody(&self) -> Option<Arc<Mutex<KmoTextBody>>> {
      // Java: return this.kmoTextBody;
      self.kmoTextBody.clone()
    }

    // Java: TextContext getTextContext()
    pub fn getTextContext(&self) -> Option<&TextContext> {
      // Java: return this.mTextContext;
      self.mTextContext.as_ref()
    }

}

// Java: public class TextContext
impl TextContext {
    pub fn new() -> Self {
        TextContext { mPageNumber: 0 }
    }

    // Java: void setPageNumber(int number)
    pub fn setPageNumber(&mut self, number: i32) {
         // Java: this.mPageNumber = number;
         self.mPageNumber = number;
    }

    // Java: int getPageNumber()
    pub fn getPageNumber(&self) -> i32 {
         // Java: return this.mPageNumber;
         self.mPageNumber
    }

}

#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

use std::sync::{Arc, Mutex};
use std::any::Any;

// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.effects.Shadow
use crate::cn::wps::moffice::drawing::effects::Shadow::Shadow;
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// use crate::cn::wps::moffice::drawing::geotext::GeoText::GeoText;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use crate::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
use crate::cn::wps::moffice::drawing::graphics::RenderModel::RenderModel;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use crate::cn::wps::moffice::drawing::line::LineProperty::LineProperty;

type Element = Option<Arc<dyn Any + Send + Sync>>;

struct Math;
impl Math {
    fn min(a: i32, b: i32) -> i32 { if a < b { a } else { b } }
    fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
}

// Java field: int colorFlagPaletteIndex
pub(crate) const colorFlagPaletteIndex: i32 = 1;
// Java field: int colorFlagPaletteRGB
pub(crate) const colorFlagPaletteRGB: i32 = 2;
// Java field: int colorFlagSystemRGB
pub(crate) const colorFlagSystemRGB: i32 = 4;
// Java field: int colorFlagSchemeIndex
pub(crate) const colorFlagSchemeIndex: i32 = 8;
// Java field: int colorFlagSysIndex
pub(crate) const colorFlagSysIndex: i32 = 16;
// Java field: int colorFillColor
pub(crate) const colorFillColor: i32 = 240;
// Java field: int colorLineOrFillColor
pub(crate) const colorLineOrFillColor: i32 = 241;
// Java field: int colorLineColor
pub(crate) const colorLineColor: i32 = 242;
// Java field: int colorShadowColor
pub(crate) const colorShadowColor: i32 = 243;
// Java field: int colorThis
pub(crate) const colorThis: i32 = 244;
// Java field: int colorFillBackColor
pub(crate) const colorFillBackColor: i32 = 245;
// Java field: int colorLineBackColor
pub(crate) const colorLineBackColor: i32 = 246;
// Java field: int colorFillThenLine
pub(crate) const colorFillThenLine: i32 = 247;
// Java field: int colorIndexMask
pub(crate) const colorIndexMask: i32 = 255;
// Java field: int colorProcessMask
pub(crate) const colorProcessMask: i32 = 16776960;
// Java field: int colorModificationMask
pub(crate) const colorModificationMask: i32 = 3840;
// Java field: int colorModFlagMask
pub(crate) const colorModFlagMask: i32 = 61440;
// Java field: int colorDarken
pub(crate) const colorDarken: i32 = 256;
// Java field: int colorLighten
pub(crate) const colorLighten: i32 = 512;
// Java field: int colorAdd
pub(crate) const colorAdd: i32 = 768;
// Java field: int colorSubtract
pub(crate) const colorSubtract: i32 = 1024;
// Java field: int colorReverseSubtract
pub(crate) const colorReverseSubtract: i32 = 1280;
// Java field: int colorBlackWhite
pub(crate) const colorBlackWhite: i32 = 1536;
// Java field: int colorInvert
pub(crate) const colorInvert: i32 = 8192;
// Java field: int colorInvert128
pub(crate) const colorInvert128: i32 = 16384;
// Java field: int colorGray
pub(crate) const colorGray: i32 = 32768;
// Java field: int colorBParamMask
pub(crate) const colorBParamMask: i32 = 16711680;
// Java field: int colorBParamShift
pub(crate) const colorBParamShift: i32 = 16;

// Java: public class ColorTranslator
pub struct ColorTranslator {
    // Java field: FillBase mFill
    pub(crate) mFill: Element,
    // Java field: LineProperty mLine
    pub(crate) mLine: Element,
    // Java field: Shadow mShadow
    mShadow: Element,
}

// Java: public class ColorTranslator
impl ColorTranslator {
    // Java: ColorTranslator(Shape shape)
    pub fn new_from_shape(shape: &mut Shape) -> Self {
      // Java: super();
      // Java: this.mFill = shape.getFill();
      let mFill = shape.getFill();
      // Java: this.mLine = shape.getLineProperty();
      let mLine = shape.getLineProperty();
      // Java: this.mShadow = shape.getShadow();
      let mShadow = shape.getShadow();
      ColorTranslator { mFill, mLine, mShadow }
    }

    // Java: ColorTranslator(RenderModel shape)
    pub fn new_from_render_model(shape: &dyn RenderModel) -> Self {
      // Java: super();
      // Java: this.mFill = shape.getFill();
      let mFill: Element = shape.getFill().map(|f| Arc::new(f) as Arc<dyn Any + Send + Sync>);
      // Java: this.mLine = shape.getLineProperty();
      let mLine = shape.getLineProperty();
      // Java: this.mShadow = shape.getShadow();
      let mShadow = shape.getShadow();
      ColorTranslator { mFill, mLine, mShadow }
    }

    // Java: ColorTranslator(GeoText geoText)
    // GeoText module currently disabled, constructor stubbed
    pub fn new_from_geo_text(mFill: Element, mLine: Element, mShadow: Element) -> Self {
      // Java: super();
      // Java: this.mFill = geoText.getFill();
      // Java: this.mLine = geoText.getStroke();
      // Java: this.mShadow = geoText.getShadow();
      ColorTranslator { mFill, mLine, mShadow }
    }

    fn get_fill(&self) -> Option<&FillBase> {
        self.mFill.as_ref()?.downcast_ref::<FillBase>()
    }

    fn get_line(&self) -> Option<&LineProperty> {
        self.mLine.as_ref()?.downcast_ref::<LineProperty>()
    }

    fn get_shadow(&self) -> Option<&Shadow> {
        self.mShadow.as_ref()?.downcast_ref::<Shadow>()
    }

    // Java: int translateColor(int color, int alpha)
    pub fn translateColor(&self, color: i32, alpha: i32) -> i32 {
      // Java: return isSysIndexColor(color) ? this.translateSysIndexColor(color, alpha) : color & 16777215 | alpha << 24 & -16777216;
      if Self::isSysIndexColor(color) {
          self.translateSysIndexColor(color, alpha)
      } else {
          (color & 16777215) | ((alpha << 24) & (-16777216i32))
      }
    }

    // Java: static boolean isSysIndexColor(int color)
    pub fn isSysIndexColor(color: i32) -> bool {
      // Java: return color >> 24 == 16;
      (color >> 24) == 16
    }

    // Java: int translateSysIndexColor(int color, int alpha)
    pub fn translateSysIndexColor(&self, color: i32, alpha: i32) -> i32 {
      // Java: int origin = 0;
      let mut origin: i32 = 0;
      // Java: int index = extractColorIndex(color);
      let index = Self::extractColorIndex(color);
      // Java: switch (index)
      match index {
         // Java: case 240:
         240 => {
            // Java: origin = this.getFillColor();
            origin = self.getFillColor();
            // Java: break;
         }
         // Java: case 241:
         241 => {
            // Java: origin = this.getLineOrFillColor();
            origin = self.getLineOrFillColor();
            // Java: break;
         }
         // Java: case 242:
         242 => {
            // Java: origin = this.getLineColor();
            origin = self.getLineColor();
            // Java: break;
         }
         // Java: case 243:
         243 => {
            // Java: origin = this.getShadowColor();
            origin = self.getShadowColor();
         }
         // Java: case 245:
         245 => {
            // Java: origin = this.getFillBackColor();
            origin = self.getFillBackColor();
            // Java: break;
         }
         // Java: case 246:
         246 => {
            // Java: origin = this.getLineBackColor();
            origin = self.getLineBackColor();
            // Java: break;
         }
         // Java: case 247:
         247 => {
            // Java: origin = this.getFillThenColor();
            origin = self.getFillThenColor();
         }
         // Java: case 244:
         // Java: default:
         _ => {
            // Java: break;
         }
      }
      // Java: return this.translateSysIndexColor(origin, color, alpha);
      self.translateSysIndexColor_full(origin, color, alpha)
    }

    // Java: int getShadowColor()
    pub fn getShadowColor(&self) -> i32 {
      // Java: int color = 8421504;
      let mut color: i32 = 8421504;
      // Java: if (this.mShadow != null)
      if let Some(shadow) = self.get_shadow() {
         // Java: int shadowColor = this.mShadow.getShadowColor();
         let shadowColor = shadow.getShadowColor();
         // Java: color = this.translateColor(shadowColor, 255);
         color = self.translateColor(shadowColor, 255);
      }
      // Java: return color;
      color
    }

    // Java: int getLineColor()
    pub fn getLineColor(&self) -> i32 {
      // Java: int color = 0;
      let mut color: i32 = 0;
      // Java: if (this.mLine != null)
      if let Some(line) = self.get_line() {
         // Java: int lineColor = this.mLine.getColor();
         let lineColor = line.getColor();
         // Java: color = this.translateColor(lineColor, 255);
         color = self.translateColor(lineColor, 255);
      }
      // Java: return color;
      color
    }

    // Java: int getLineBackColor()
    pub fn getLineBackColor(&self) -> i32 {
      // Java: int color = 16777215;
      let mut color: i32 = 16777215;
      // Java: if (this.mLine != null)
      if let Some(line) = self.get_line() {
         // Java: int lineColor = this.mLine.getBackColor();
         let lineColor = line.getBackColor();
         // Java: color = this.translateColor(lineColor, 255);
         color = self.translateColor(lineColor, 255);
      }
      // Java: return color;
      color
    }

    // Java: int getFillColor()
    pub fn getFillColor(&self) -> i32 {
      // Java: int color = -1;
      let mut color: i32 = -1;
      // Java: if (this.mFill != null)
      if let Some(fill) = self.get_fill() {
         // Java: int fillColor = FillBase.getMainColor(this.mFill);
         let fillColor = FillBase::getMainColor(Some(fill));
         // Java: color = this.translateColor(fillColor, 255);
         color = self.translateColor(fillColor, 255);
      }
      // Java: return color;
      color
    }

    // Java: int getFillBackColor()
    pub fn getFillBackColor(&self) -> i32 {
      // Java: int color = -1;
      let mut color: i32 = -1;
      // Java: if (this.mFill != null)
      if let Some(fill) = self.get_fill() {
         // Java: int fillColor = this.mFill.getBackColor();
         let fillColor = fill.getBackColor();
         // Java: color = this.translateColor(fillColor, 255);
         color = self.translateColor(fillColor, 255);
      }
      // Java: return color;
      color
    }

    // Java: int getLineOrFillColor()
    pub fn getLineOrFillColor(&self) -> i32 {
      // Java: return this.mLine != null && this.mLine.getfLine() ? this.getLineColor() : this.getFillColor();
      if self.get_line().map_or(false, |l| l.getfLine()) {
          self.getLineColor()
      } else {
          self.getFillColor()
      }
    }

    // Java: int getFillThenColor()
    pub fn getFillThenColor(&self) -> i32 {
      // Java: return this.mFill != null && this.mFill.getFillOn() ? this.getFillColor() : this.getLineColor();
      if self.get_fill().map_or(false, |f| f.getFillOn()) {
          self.getFillColor()
      } else {
          self.getLineColor()
      }
    }

    // Java: static int extractColorIndex(int color)
    pub fn extractColorIndex(color: i32) -> i32 {
      // Java: return color & 255;
      color & 255
    }

    // Java: static int extractModification(int color)
    pub fn extractModification(color: i32) -> i32 {
      // Java: return color & 3840;
      color & 3840
    }

    // Java: static int extractProcessParam(int color)
    pub fn extractProcessParam(color: i32) -> i32 {
      // Java: return (color & 16711680) >> 16;
      (color & 16711680) >> 16
    }

    // Java: int translateSysIndexColor(int origin, int color, int alpha)
    pub fn translateSysIndexColor_full(&self, origin: i32, color: i32, alpha: i32) -> i32 {
      // Java: int destColor = 0;
      let mut destColor: i32 = 0;
      // Java: int para = extractProcessParam(color);
      let para = Self::extractProcessParam(color);
      // Java: switch (extractModification(color))
      match Self::extractModification(color) {
         // Java: case 256:
         256 => {
            // Java: destColor = darkenColor(origin, para);
            destColor = Self::darkenColor(origin, para);
            // Java: break;
         }
         // Java: case 512:
         512 => {
            // Java: destColor = lightenColor(origin, para);
            destColor = Self::lightenColor(origin, para);
            // Java: break;
         }
         // Java: case 768:
         768 => {
            // Java: destColor = addColorByGray(origin, para);
            destColor = Self::addColorByGray(origin, para);
            // Java: break;
         }
         // Java: case 1024:
         1024 => {
            // Java: destColor = subColorByGray(origin, para);
            destColor = Self::subColorByGray(origin, para);
            // Java: break;
         }
         // Java: case 1280:
         1280 => {
            // Java: destColor = reverseSubColorByGray(origin, para);
            destColor = Self::reverseSubColorByGray(origin, para);
         }
         _ => {}
      }
      // Java: return destColor & 16777215 | (alpha & 255) << 24;
      (destColor & 16777215) | ((alpha & 255) << 24)
    }

    // Java: static int alphaBlend(int c1, int c2, int alpha)
    pub fn alphaBlend(c1: i32, c2: i32, alpha: i32) -> i32 {
      // Java: return ((c1 - c2) * alpha >> 8) + c2;
      ((c1 - c2) * alpha >> 8) + c2
    }

    // Java: static int darkenColor(int color, int para)
    pub fn darkenColor(color: i32, para: i32) -> i32 {
      // Java: return (color >> 16 & 255) * para / 255 << 16 | (color >> 8 & 255) * para / 255 << 8 | (color >> 0 & 255) * para / 255 << 0;
      ((color >> 16 & 255) * para / 255 << 16) | ((color >> 8 & 255) * para / 255 << 8) | ((color & 255) * para / 255)
    }

    // Java: static int lightenColor(int color, int para)
    pub fn lightenColor(color: i32, para: i32) -> i32 {
      // Java: return ((color >> 16 & 255) - 255) * para / 255 + 255 << 16 | ((color >> 8 & 255) - 255) * para / 255 + 255 << 8 | ((color >> 0 & 255) - 255) * para / 255 + 255 << 0;
      ((((color >> 16 & 255) - 255) * para / 255 + 255) << 16) | ((((color >> 8 & 255) - 255) * para / 255 + 255) << 8) | (((color & 255) - 255) * para / 255 + 255)
    }

    // Java: static int saturatedAdd(int b1, int b2)
    pub fn saturatedAdd(b1: i32, b2: i32) -> i32 {
      // Java: return Math.min(b1 + b2, 255);
      Math::min(b1 + b2, 255)
    }

    // Java: static int saturatedSub(int b1, int b2)
    pub fn saturatedSub(b1: i32, b2: i32) -> i32 {
      // Java: return Math.max(b1 - b2, 0);
      Math::max(b1 - b2, 0)
    }

    // Java: static int addColorByGray(int origin, int param)
    pub fn addColorByGray(origin: i32, param: i32) -> i32 {
      // Java: return saturatedAdd(origin >> 16 & 255, param) << 16 | saturatedAdd(origin >> 8 & 255, param) << 8 | saturatedAdd(origin >> 0 & 255, param) << 0;
      (Self::saturatedAdd(origin >> 16 & 255, param) << 16) | (Self::saturatedAdd(origin >> 8 & 255, param) << 8) | Self::saturatedAdd(origin & 255, param)
    }

    // Java: static int subColorByGray(int origin, int param)
    pub fn subColorByGray(origin: i32, param: i32) -> i32 {
      // Java: return saturatedSub(origin >> 16 & 255, param) << 16 | saturatedSub(origin >> 8 & 255, param) << 8 | saturatedSub(origin >> 0 & 255, param) << 0;
      (Self::saturatedSub(origin >> 16 & 255, param) << 16) | (Self::saturatedSub(origin >> 8 & 255, param) << 8) | Self::saturatedSub(origin & 255, param)
    }

    // Java: static int reverseSubColorByGray(int origin, int param)
    pub fn reverseSubColorByGray(origin: i32, param: i32) -> i32 {
      // Java: return saturatedSub(param, origin >> 16 & 255) << 16 | saturatedSub(param, origin >> 8 & 255) << 8 | saturatedSub(param, origin >> 0 & 255) << 0;
      (Self::saturatedSub(param, origin >> 16 & 255) << 16) | (Self::saturatedSub(param, origin >> 8 & 255) << 8) | Self::saturatedSub(param, origin & 255)
    }

}

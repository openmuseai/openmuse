#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.DefaultShapeValue
use super::super::DefaultShapeValue::DefaultShapeValue;
// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;
// Java: import cn.wps.moffice.drawing.effects.Glow
// Java: import cn.wps.moffice.drawing.effects.Object3D
// Java: import cn.wps.moffice.drawing.effects.Perspective
// Java: import cn.wps.moffice.drawing.effects.Reflection
// Java: import cn.wps.moffice.drawing.effects.Shadow
// Java: import cn.wps.moffice.drawing.graphics.FillBase
// Java: import cn.wps.moffice.drawing.line.LineProperty

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class GeoText extends PropBase
pub struct GeoText {
    pub base: PropBase,
}

// Java: public class GeoText extends PropBase
impl GeoText {
    // Java: GeoText()
    pub fn new() -> Self {
      // Java: super();
      GeoText { base: PropBase::new() }
    }

    // Java: GeoText clone()
    pub fn clone(&self) -> GeoText {
      // Java: GeoText geoText = (GeoText)super.clone();
      let mut geoText = GeoText { base: self.base.clone()? };
      // Java: Shadow shadow = this.getShadow();
      let shadow = self.getShadow();
      // Java: if (shadow != null)
      if shadow.is_some() {
         // Java: geoText.setShadow(shadow.clone());
         geoText.setShadow(shadow);
      }
      // Java: Glow glow = this.getGlow();
      let glow = self.getGlow();
      // Java: if (glow != null)
      if glow.is_some() {
         // Java: geoText.setGlow(glow.clone());
         geoText.setGlow(glow);
      }
      // Java: Reflection reflection = this.getReflection();
      let reflection = self.getReflection();
      // Java: if (reflection != null)
      if reflection.is_some() {
         // Java: geoText.setReflection(reflection);
         geoText.setReflection(reflection);
      }
      // Java: LineProperty line = this.getStroke();
      let line = self.getStroke();
      // Java: if (line != null)
      if line.is_some() {
         // Java: geoText.setStroke(line.clone());
         geoText.setStroke(line);
      }
      // Java: FillBase fill = this.getFill();
      let fill = self.getFill();
      // Java: if (fill != null)
      if fill.is_some() {
         // Java: geoText.setFill(fill);
         geoText.setFill(fill);
      }
      // Java: Perspective perspective = this.getPerspective();
      let perspective = self.getPerspective();
      // Java: if (perspective != null)
      if perspective.is_some() {
         // Java: geoText.setPerspective(perspective);
         geoText.setPerspective(perspective);
      }
      // Java: Object3D object3d = this.getObject3d();
      let object3d = self.getObject3d();
      // Java: if (object3d != null)
      if object3d.is_some() {
         // Java: geoText.setObject3D(object3d);
         geoText.setObject3D(object3d);
      }
      // Java: return geoText;
      geoText
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GeoText) -> bool {
      // Java: if (o != null && o instanceof GeoText)
      {
         // Java: if (o == this)
         if std::ptr::eq(o, self) {
            // Java: return true;
            return true;
         }
         // Java: else
         {
            // Java: GeoText geoText = (GeoText)o;
            let geoText = o;
            // Java: return this.getProperty().equals(geoText.getProperty());
            return self.base.getProperty().base.equals(&geoText.base.getProperty().base);
         }
      }
      // Java: else
      {
         // Java: return false;
         false
      }
    }

    // Java: void setUnicodeText(String text)
    pub fn setUnicodeText(&mut self, text: String) {
      // Java: this.mProperty.setString(414, text);
      self.base.mProperty.as_mut().unwrap().setString(414, text);
    }

    // Java: String getUnicodeText()
    pub fn getUnicodeText(&self) -> String {
      // Java: return this.mProperty.getString(414, DefaultShapeValue.gtextUNICODE);
      self.base.mProperty.as_ref().unwrap().base.getString_default(414, "")
    }

    // Java: void setRTFText(String text)
    pub fn setRTFText(&mut self, text: String) {
      // Java: this.mProperty.setString(415, text);
      self.base.mProperty.as_mut().unwrap().setString(415, text);
    }

    // Java: String getRTFText()
    pub fn getRTFText(&self) -> String {
      // Java: return this.mProperty.getString(415, DefaultShapeValue.gtextRTF);
      self.base.mProperty.as_ref().unwrap().base.getString_default(415, "")
    }

    // Java: void setAlign(int align)
    pub fn setAlign(&mut self, align: i32) {
      // Java: this.mProperty.setInt(416, align);
      self.base.mProperty.as_mut().unwrap().setInt(416, align);
    }

    // Java: int getAlign()
    pub fn getAlign(&self) -> i32 {
      // Java: return this.mProperty.getInt(416, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(416, 1)
    }

    // Java: void setSize(float size)
    pub fn setSize(&mut self, size: f32) {
      // Java: this.mProperty.setFloat(417, size);
      self.base.mProperty.as_mut().unwrap().setFloat(417, size);
    }

    // Java: float getSize()
    pub fn getSize(&self) -> f32 {
      // Java: return this.mProperty.getFloat(417, 36.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(417, 36.0_f32)
    }

    // Java: void setSpacing(float spacing)
    pub fn setSpacing(&mut self, spacing: f32) {
      // Java: this.mProperty.setFloat(418, spacing);
      self.base.mProperty.as_mut().unwrap().setFloat(418, spacing);
    }

    // Java: float getSpacing()
    pub fn getSpacing(&self) -> f32 {
      // Java: return this.mProperty.getFloat(418, 1.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(418, 1.0_f32)
    }

    // Java: void setFont(String font)
    pub fn setFont(&mut self, font: String) {
      // Java: this.mProperty.setString(419, font);
      self.base.mProperty.as_mut().unwrap().setString(419, font);
    }

    // Java: String getFont()
    pub fn getFont(&self) -> String {
      // Java: return this.mProperty.getString(419, DefaultShapeValue.gtextFont);
      self.base.mProperty.as_ref().unwrap().base.getString_default(419, "")
    }

    // Java: void setCSSFont(String font)
    pub fn setCSSFont(&mut self, font: String) {
      // Java: this.mProperty.setString(420, font);
      self.base.mProperty.as_mut().unwrap().setString(420, font);
    }

    // Java: String getCSSFont()
    pub fn getCSSFont(&self) -> String {
      // Java: return this.mProperty.getString(420, DefaultShapeValue.gtextCSSFont);
      self.base.mProperty.as_ref().unwrap().base.getString_default(420, "")
    }

    // Java: void setReverseRows(boolean bReverse)
    pub fn setReverseRows(&mut self, bReverse: bool) {
      // Java: this.mProperty.setBoolean(421, bReverse);
      self.base.mProperty.as_mut().unwrap().setBoolean(421, bReverse);
    }

    // Java: boolean getReverseRows()
    pub fn getReverseRows(&self) -> bool {
      // Java: return this.mProperty.getBoolean(421, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(421, false)
    }

    // Java: void setHasGeoText(boolean bText)
    pub fn setHasGeoText(&mut self, bText: bool) {
      // Java: this.mProperty.setBoolean(422, bText);
      self.base.mProperty.as_mut().unwrap().setBoolean(422, bText);
    }

    // Java: boolean getHasGeoText()
    pub fn getHasGeoText(&self) -> bool {
      // Java: return this.mProperty.getBoolean(422, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(422, false)
    }

    // Java: void setVertical(boolean bVertical)
    pub fn setVertical(&mut self, bVertical: bool) {
      // Java: this.mProperty.setBoolean(423, bVertical);
      self.base.mProperty.as_mut().unwrap().setBoolean(423, bVertical);
    }

    // Java: boolean getVertical()
    pub fn getVertical(&self) -> bool {
      // Java: return this.mProperty.getBoolean(423, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(423, false)
    }

    // Java: void setKern(boolean bKern)
    pub fn setKern(&mut self, bKern: bool) {
      // Java: this.mProperty.setBoolean(424, bKern);
      self.base.mProperty.as_mut().unwrap().setBoolean(424, bKern);
    }

    // Java: boolean getKern()
    pub fn getKern(&self) -> bool {
      // Java: return this.mProperty.getBoolean(424, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(424, false)
    }

    // Java: void setTight(boolean bTight)
    pub fn setTight(&mut self, bTight: bool) {
      // Java: this.mProperty.setBoolean(425, bTight);
      self.base.mProperty.as_mut().unwrap().setBoolean(425, bTight);
    }

    // Java: boolean getTight()
    pub fn getTight(&self) -> bool {
      // Java: return this.mProperty.getBoolean(425, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(425, false)
    }

    // Java: void setStretch(boolean bStretch)
    pub fn setStretch(&mut self, bStretch: bool) {
      // Java: this.mProperty.setBoolean(426, bStretch);
      self.base.mProperty.as_mut().unwrap().setBoolean(426, bStretch);
    }

    // Java: boolean getStretch()
    pub fn getStretch(&self) -> bool {
      // Java: return this.mProperty.getBoolean(426, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(426, false)
    }

    // Java: void setShrinkFit(boolean bShrinkFit)
    pub fn setShrinkFit(&mut self, bShrinkFit: bool) {
      // Java: this.mProperty.setBoolean(427, bShrinkFit);
      self.base.mProperty.as_mut().unwrap().setBoolean(427, bShrinkFit);
    }

    // Java: boolean getShrinkFit()
    pub fn getShrinkFit(&self) -> bool {
      // Java: return this.mProperty.getBoolean(427, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(427, false)
    }

    // Java: void setBestFit(boolean bBestFit)
    pub fn setBestFit(&mut self, bBestFit: bool) {
      // Java: this.mProperty.setBoolean(428, bBestFit);
      self.base.mProperty.as_mut().unwrap().setBoolean(428, bBestFit);
    }

    // Java: boolean getBestFit()
    pub fn getBestFit(&self) -> bool {
      // Java: return this.mProperty.getBoolean(428, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(428, false)
    }

    // Java: void setNormalize(boolean bNormalize)
    pub fn setNormalize(&mut self, bNormalize: bool) {
      // Java: this.mProperty.setBoolean(429, bNormalize);
      self.base.mProperty.as_mut().unwrap().setBoolean(429, bNormalize);
    }

    // Java: boolean getNormalize()
    pub fn getNormalize(&self) -> bool {
      // Java: return this.mProperty.getBoolean(429, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(429, false)
    }

    // Java: void setDxMeasure(boolean bDxMeasure)
    pub fn setDxMeasure(&mut self, bDxMeasure: bool) {
      // Java: this.mProperty.setBoolean(430, bDxMeasure);
      self.base.mProperty.as_mut().unwrap().setBoolean(430, bDxMeasure);
    }

    // Java: boolean getDxMeasure()
    pub fn getDxMeasure(&self) -> bool {
      // Java: return this.mProperty.getBoolean(430, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(430, false)
    }

    // Java: void setBold(boolean bBold)
    pub fn setBold(&mut self, bBold: bool) {
      // Java: this.mProperty.setBoolean(431, bBold);
      self.base.mProperty.as_mut().unwrap().setBoolean(431, bBold);
    }

    // Java: boolean getBold()
    pub fn getBold(&self) -> bool {
      // Java: return this.mProperty.getBoolean(431, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(431, false)
    }

    // Java: void setItalic(boolean bItalic)
    pub fn setItalic(&mut self, bItalic: bool) {
      // Java: this.mProperty.setBoolean(432, bItalic);
      self.base.mProperty.as_mut().unwrap().setBoolean(432, bItalic);
    }

    // Java: boolean getItalic()
    pub fn getItalic(&self) -> bool {
      // Java: return this.mProperty.getBoolean(432, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(432, false)
    }

    // Java: void setUnderline(boolean bUnderline)
    pub fn setUnderline(&mut self, bUnderline: bool) {
      // Java: this.mProperty.setBoolean(433, bUnderline);
      self.base.mProperty.as_mut().unwrap().setBoolean(433, bUnderline);
    }

    // Java: boolean getUnderline()
    pub fn getUnderline(&self) -> bool {
      // Java: return this.mProperty.getBoolean(433, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(433, false)
    }

    // Java: void setFShadow(boolean bShadow)
    pub fn setFShadow(&mut self, bShadow: bool) {
      // Java: this.mProperty.setBoolean(434, bShadow);
      self.base.mProperty.as_mut().unwrap().setBoolean(434, bShadow);
    }

    // Java: boolean getFShadow()
    pub fn getFShadow(&self) -> bool {
      // Java: return this.mProperty.getBoolean(434, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(434, false)
    }

    // Java: void setSmallcaps(boolean bSmallcaps)
    pub fn setSmallcaps(&mut self, bSmallcaps: bool) {
      // Java: this.mProperty.setBoolean(435, bSmallcaps);
      self.base.mProperty.as_mut().unwrap().setBoolean(435, bSmallcaps);
    }

    // Java: boolean getSmallcaps()
    pub fn getSmallcaps(&self) -> bool {
      // Java: return this.mProperty.getBoolean(435, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(435, false)
    }

    // Java: void setStrikethrough(boolean bStrikethrough)
    pub fn setStrikethrough(&mut self, bStrikethrough: bool) {
      // Java: this.mProperty.setBoolean(436, bStrikethrough);
      self.base.mProperty.as_mut().unwrap().setBoolean(436, bStrikethrough);
    }

    // Java: boolean getStrikethrough()
    pub fn getStrikethrough(&self) -> bool {
      // Java: return this.mProperty.getBoolean(436, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(436, false)
    }

    // Java: void setShadow(Shadow shadow)
    pub fn setShadow(&mut self, shadow: Element) {
      // Java: this.getProperty().setValue(172, shadow);
      self.base.mProperty.as_mut().unwrap().setValue(172, shadow);
    }

    // Java: Shadow getShadow()
    pub fn getShadow(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(172);
      // Java: return object == null ? null : (Shadow)object;
      self.base.getProperty().base.getValue(172)
    }

    // Java: void setGlow(Glow glow)
    pub fn setGlow(&mut self, glow: Element) {
      // Java: if (glow != null)
      // Java: glow.setListener(this.getListener());
      // Java: this.getProperty().setValue(262, glow);
      self.base.mProperty.as_mut().unwrap().setValue(262, glow);
    }

    // Java: Glow getGlow()
    pub fn getGlow(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(262);
      // Java: return object == null ? null : (Glow)object;
      self.base.getProperty().base.getValue(262)
    }

    // Java: void setReflection(Reflection reflection)
    pub fn setReflection(&mut self, reflection: Element) {
      // Java: if (reflection != null)
      // Java: reflection.setListener(this.getListener());
      // Java: this.getProperty().setValue(232, reflection);
      self.base.mProperty.as_mut().unwrap().setValue(232, reflection);
    }

    // Java: Reflection getReflection()
    pub fn getReflection(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(232);
      // Java: return object == null ? null : (Reflection)object;
      self.base.getProperty().base.getValue(232)
    }

    // Java: void setStroke(LineProperty line)
    pub fn setStroke(&mut self, line: Element) {
      // Java: if (line != null)
      // Java: line.setListener(this.getListener());
      // Java: this.getProperty().setValue(553, line);
      self.base.mProperty.as_mut().unwrap().setValue(553, line);
    }

    // Java: LineProperty getStroke()
    pub fn getStroke(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(553);
      // Java: return object == null ? null : (LineProperty)object;
      self.base.getProperty().base.getValue(553)
    }

    // Java: void setFill(FillBase fill)
    pub fn setFill(&mut self, fill: Element) {
      // Java: if (fill != null)
      // Java: fill.setListener(this.getListener());
      // Java: this.getProperty().setValue(507, fill);
      self.base.mProperty.as_mut().unwrap().setValue(507, fill);
    }

    // Java: FillBase getFill()
    pub fn getFill(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(507);
      // Java: return object == null ? null : (FillBase)object;
      self.base.getProperty().base.getValue(507)
    }

    // Java: void setPerspective(Perspective perspective)
    pub fn setPerspective(&mut self, perspective: Element) {
      // Java: if (perspective != null)
      // Java: perspective.setListener(this.getListener());
      // Java: this.getProperty().setValue(302, perspective);
      self.base.mProperty.as_mut().unwrap().setValue(302, perspective);
    }

    // Java: Perspective getPerspective()
    pub fn getPerspective(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(302);
      // Java: return object == null ? null : (Perspective)object;
      self.base.getProperty().base.getValue(302)
    }

    // Java: void setObject3D(Object3D object3d)
    pub fn setObject3D(&mut self, object3d: Element) {
      // Java: if (object3d != null)
      // Java: object3d.setListener(this.getListener());
      // Java: this.getProperty().setValue(332, object3d);
      self.base.mProperty.as_mut().unwrap().setValue(332, object3d);
    }

    // Java: Object3D getObject3d()
    pub fn getObject3d(&self) -> Element {
      // Java: Object object = this.getProperty().getValue(332);
      // Java: return object == null ? null : (Object3D)object;
      self.base.getProperty().base.getValue(332)
    }

}

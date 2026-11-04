#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

use super::PropBase::PropBase;
use super::IPos::IPos;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class ShapePos extends PropBase implements IPos
pub struct ShapePos {
    pub base: PropBase,
}

// Java: public class ShapePos extends PropBase implements IPos
impl ShapePos {
    // Java: ShapePos()
    pub fn new() -> Self {
      // Java: super();
      ShapePos { base: PropBase::new() }
    }

    // Java: ShapePos clone()
    pub fn clone(&mut self) -> ShapePos {
      // Java: return (ShapePos)super.clone();
      ShapePos { base: self.base.clone() } // TODO review this
    }

    // Java: void setShapeRect(RectF rectF)
    pub fn setShapeRect(&mut self, rectF: Element) {
      // Java: this.mProperty.setValue(23, rectF);
      self.base.mProperty.as_mut().unwrap().setValue(23, rectF);
    }

    // Java: RectF getShapeRect()
    pub fn getShapeRect(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(23);
      // Java: return object == null ? null : (RectF)object;
      self.base.mProperty.as_ref().unwrap().base.getValue(23)
    }

    // Java: void setShapeRectChange(RectF rectF)
    pub fn setShapeRectChange(&mut self, rectF: Element) {
      // Java: float rot = this.getRotation();
      let _rot = self.getRotation();
      // Java: if (ShapeHelper.isChangeRect((int)rot))
      // TODO: ShapeHelper not yet available
      {
         // Java: rectF = ShapeHelper.getShapeBoundRect(rectF);
      }
      // Java: this.setShapeRect(rectF);
      self.setShapeRect(rectF);
    }

    // Java: RectF getShapeRectChange()
    pub fn getShapeRectChange(&self) -> Element {
      // Java: RectF rectF = this.getShapeRect();
      let rectF = self.getShapeRect();
      // Java: if (rectF == null)
      if rectF.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: float rot = this.getRotation();
         let _rot = self.getRotation();
         // Java: if (ShapeHelper.isChangeRect((int)rot))
         // TODO: ShapeHelper not yet available
         {
            // Java: rectF = ShapeHelper.getShapeBoundRect(rectF);
         }
         // Java: return rectF;
         rectF
      }
    }

    // Java: int getRelativeX()
    pub fn getRelativeX(&self) -> i32 {
      // Java: Object pVal = this.mProperty.getValue(27);
      let pVal = self.base.mProperty.as_ref().unwrap().base.getValue(27);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return this.getRelativeH();
         return self.getRelativeH();
      }
      // Java: else
      {
         // Java: Integer v = (Integer)pVal;
         let v = pVal.unwrap().downcast_ref::<i32>().copied().unwrap_or(0);
         // Java: return v;
         v
      }
    }

    // Java: void setRelativeH(int rel)
    pub fn setRelativeH(&mut self, rel: i32) {
      // Java: this.mProperty.setInt(29, rel);
      self.base.mProperty.as_mut().unwrap().setInt(29, rel);
    }

    // Java: int getRelativeH()
    pub fn getRelativeH(&self) -> i32 {
      // Java: return this.mProperty.getInt(29, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(29, 1)
    }

    // Java: void setPosRelH(int rel)
    pub fn setPosRelH(&mut self, rel: i32) {
      // Java: this.mProperty.setInt(27, rel);
      self.base.mProperty.as_mut().unwrap().setInt(27, rel);
    }

    // Java: int getPosRelH()
    pub fn getPosRelH(&self) -> i32 {
      // Java: return this.mProperty.getInt(27, 2);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(27, 2)
    }

    // Java: int getRelativeY()
    pub fn getRelativeY(&self) -> i32 {
      // Java: Object pVal = this.mProperty.getValue(28);
      let pVal = self.base.mProperty.as_ref().unwrap().base.getValue(28);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return this.getRelativeV();
         return self.getRelativeV();
      }
      // Java: else
      {
         // Java: Integer v = (Integer)pVal;
         let v = pVal.unwrap().downcast_ref::<i32>().copied().unwrap_or(0);
         // Java: return v;
         v
      }
    }

    // Java: void setRelativeV(int rel)
    pub fn setRelativeV(&mut self, rel: i32) {
      // Java: this.mProperty.setInt(30, rel);
      self.base.mProperty.as_mut().unwrap().setInt(30, rel);
    }

    // Java: int getRelativeV()
    pub fn getRelativeV(&self) -> i32 {
      // Java: return this.mProperty.getInt(30, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(30, 1)
    }

    // Java: void setPosRelV(int rel)
    pub fn setPosRelV(&mut self, rel: i32) {
      // Java: this.mProperty.setInt(28, rel);
      self.base.mProperty.as_mut().unwrap().setInt(28, rel);
    }

    // Java: int getPosRelV()
    pub fn getPosRelV(&self) -> i32 {
      // Java: return this.mProperty.getInt(28, 2);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(28, 2)
    }

    // Java: void setPosHPercent(float val)
    pub fn setPosHPercent(&mut self, val: f32) {
      // Java: this.mProperty.setValue(31, val);
      self.base.mProperty.as_mut().unwrap().setFloat(31, val);
    }

    // Java: void clearPosHPercent()
    pub fn clearPosHPercent(&mut self) {
      // Java: this.mProperty.setValue(31, null);
      self.base.mProperty.as_mut().unwrap().setValue(31, None);
    }

    // Java: float getPosHPercent()
    pub fn getPosHPercent(&self) -> f32 {
      // Java: return this.mProperty.getFloat(31, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(31, 0.0_f32)
    }

    // Java: Float getPosHPercentObj()
    pub fn getPosHPercentObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(31);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(31);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setPosVPercent(float val)
    pub fn setPosVPercent(&mut self, val: f32) {
      // Java: this.mProperty.setValue(32, val);
      self.base.mProperty.as_mut().unwrap().setFloat(32, val);
    }

    // Java: void clearPosVPercent()
    pub fn clearPosVPercent(&mut self) {
      // Java: this.mProperty.setValue(32, null);
      self.base.mProperty.as_mut().unwrap().setValue(32, None);
    }

    // Java: float getPosVPercent()
    pub fn getPosVPercent(&self) -> f32 {
      // Java: return this.mProperty.getFloat(32, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(32, 0.0_f32)
    }

    // Java: Float getPosVPercentObj()
    pub fn getPosVPercentObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(32);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(32);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setAlignmentH(int h)
    pub fn setAlignmentH(&mut self, h: i32) {
      // Java: this.mProperty.setInt(33, h);
      self.base.mProperty.as_mut().unwrap().setInt(33, h);
    }

    // Java: int getAlignmentH()
    pub fn getAlignmentH(&self) -> i32 {
      // Java: return this.mProperty.getInt(33, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(33, 0)
    }

    // Java: void setAlignmentV(int v)
    pub fn setAlignmentV(&mut self, v: i32) {
      // Java: this.mProperty.setInt(34, v);
      self.base.mProperty.as_mut().unwrap().setInt(34, v);
    }

    // Java: int getAlignmentV()
    pub fn getAlignmentV(&self) -> i32 {
      // Java: return this.mProperty.getInt(34, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(34, 0)
    }

    // Java: void setSizeRelH(int v)
    pub fn setSizeRelH(&mut self, v: i32) {
      // Java: this.mProperty.setInt(35, v);
      self.base.mProperty.as_mut().unwrap().setInt(35, v);
    }

    // Java: int getSizeRelH()
    pub fn getSizeRelH(&self) -> i32 {
      // Java: return this.mProperty.getInt(35, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(35, 1)
    }

    // Java: Integer getSizeRelHObj()
    pub fn getSizeRelHObj(&self) -> Option<i32> {
      // Java: Object ret = this.mProperty.getValue(35);
      let ret = self.base.mProperty.as_ref().unwrap().base.getValue(35);
      // Java: return null == ret ? null : (Integer)ret;
      ret.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setSizeRelV(int v)
    pub fn setSizeRelV(&mut self, v: i32) {
      // Java: this.mProperty.setInt(36, v);
      self.base.mProperty.as_mut().unwrap().setInt(36, v);
    }

    // Java: int getSizeRelV()
    pub fn getSizeRelV(&self) -> i32 {
      // Java: return this.mProperty.getInt(36, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(36, 1)
    }

    // Java: Integer getSizeRelVObj()
    pub fn getSizeRelVObj(&self) -> Option<i32> {
      // Java: Object ret = this.mProperty.getValue(36);
      let ret = self.base.mProperty.as_ref().unwrap().base.getValue(36);
      // Java: return null == ret ? null : (Integer)ret;
      ret.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setSizeHPercent(float val)
    pub fn setSizeHPercent(&mut self, val: f32) {
      // Java: this.mProperty.setValue(37, val);
      self.base.mProperty.as_mut().unwrap().setFloat(37, val);
    }

    // Java: void clearSizeHPercent()
    pub fn clearSizeHPercent(&mut self) {
      // Java: this.mProperty.setValue(37, null);
      self.base.mProperty.as_mut().unwrap().setValue(37, None);
    }

    // Java: float getSizeHPercent()
    pub fn getSizeHPercent(&self) -> f32 {
      // Java: return this.mProperty.getFloat(37, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(37, 0.0_f32)
    }

    // Java: Float getSizeHPercentObj()
    pub fn getSizeHPercentObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(37);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(37);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setSizeVPercent(float val)
    pub fn setSizeVPercent(&mut self, val: f32) {
      // Java: this.mProperty.setValue(38, val);
      self.base.mProperty.as_mut().unwrap().setFloat(38, val);
    }

    // Java: void clearSizeVPercent()
    pub fn clearSizeVPercent(&mut self) {
      // Java: this.mProperty.setValue(38, null);
      self.base.mProperty.as_mut().unwrap().setValue(38, None);
    }

    // Java: float getSizeVPercent()
    pub fn getSizeVPercent(&self) -> f32 {
      // Java: return this.mProperty.getFloat(38, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(38, 0.0_f32)
    }

    // Java: Float getSizeVPercentObj()
    pub fn getSizeVPercentObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(38);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(38);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setScaleH(float scale)
    pub fn setScaleH(&mut self, scale: f32) {
      // Java: this.mProperty.setFloat(25, scale);
      self.base.mProperty.as_mut().unwrap().setFloat(25, scale);
    }

    // Java: float getScaleH()
    pub fn getScaleH(&self) -> f32 {
      // Java: return this.mProperty.getFloat(25, 1.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(25, 1.0_f32)
    }

    // Java: void setScaleV(float scale)
    pub fn setScaleV(&mut self, scale: f32) {
      // Java: this.mProperty.setFloat(26, scale);
      self.base.mProperty.as_mut().unwrap().setFloat(26, scale);
    }

    // Java: float getScaleV()
    pub fn getScaleV(&self) -> f32 {
      // Java: return this.mProperty.getFloat(26, 1.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(26, 1.0_f32)
    }

    // Java: void setWrapDistLeft(float dist)
    pub fn setWrapDistLeft(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(815, dist);
      self.base.mProperty.as_mut().unwrap().setFloat(815, dist);
    }

    // Java: float getWrapDistLeft()
    pub fn getWrapDistLeft(&self) -> f32 {
      // Java: int textWrap = this.getTextWrap();
      let textWrap = self.getTextWrap();
      // Java: return textWrap != 0 && textWrap != 2 && textWrap != 4 && textWrap != 5 ? 0.0F : this.mProperty.getFloat(815, 9.0F);
      if textWrap != 0 && textWrap != 2 && textWrap != 4 && textWrap != 5 {
          0.0_f32
      } else {
          self.base.mProperty.as_ref().unwrap().base.getFloat_default(815, 9.0_f32)
      }
    }

    // Java: Float getWrapDistLeftObj()
    pub fn getWrapDistLeftObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(815);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(815);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setWrapDistTop(float dist)
    pub fn setWrapDistTop(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(817, dist);
      self.base.mProperty.as_mut().unwrap().setFloat(817, dist);
    }

    // Java: float getWrapDistTop()
    pub fn getWrapDistTop(&self) -> f32 {
      // Java: int textWrap = this.getTextWrap();
      let textWrap = self.getTextWrap();
      // Java: return textWrap != 0 && textWrap != 2 && textWrap != 1 ? 0.0F : this.mProperty.getFloat(817, 0.0F);
      if textWrap != 0 && textWrap != 2 && textWrap != 1 {
          0.0_f32
      } else {
          self.base.mProperty.as_ref().unwrap().base.getFloat_default(817, 0.0_f32)
      }
    }

    // Java: Float getWrapDistTopObj()
    pub fn getWrapDistTopObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(817);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(817);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setWrapDistRight(float dist)
    pub fn setWrapDistRight(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(816, dist);
      self.base.mProperty.as_mut().unwrap().setFloat(816, dist);
    }

    // Java: float getWrapDistRight()
    pub fn getWrapDistRight(&self) -> f32 {
      // Java: int textWrap = this.getTextWrap();
      let textWrap = self.getTextWrap();
      // Java: return textWrap != 0 && textWrap != 2 && textWrap != 4 && textWrap != 5 ? 0.0F : this.mProperty.getFloat(816, 9.0F);
      if textWrap != 0 && textWrap != 2 && textWrap != 4 && textWrap != 5 {
          0.0_f32
      } else {
          self.base.mProperty.as_ref().unwrap().base.getFloat_default(816, 9.0_f32)
      }
    }

    // Java: Float getWrapDistRightObj()
    pub fn getWrapDistRightObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(816);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(816);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setWrapDistBottom(float dist)
    pub fn setWrapDistBottom(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(818, dist);
      self.base.mProperty.as_mut().unwrap().setFloat(818, dist);
    }

    // Java: float getWrapDistBottom()
    pub fn getWrapDistBottom(&self) -> f32 {
      // Java: int textWrap = this.getTextWrap();
      let textWrap = self.getTextWrap();
      // Java: return textWrap != 0 && textWrap != 2 && textWrap != 1 ? 0.0F : this.mProperty.getFloat(818, 0.0F);
      if textWrap != 0 && textWrap != 2 && textWrap != 1 {
          0.0_f32
      } else {
          self.base.mProperty.as_ref().unwrap().base.getFloat_default(818, 0.0_f32)
      }
    }

    // Java: Float getWrapDistBottomObj()
    pub fn getWrapDistBottomObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(818);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(818);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setTextWrap(int wrap)
    pub fn setTextWrap(&mut self, wrap: i32) {
      // Java: this.mProperty.setInt(811, wrap);
      self.base.mProperty.as_mut().unwrap().setInt(811, wrap);
    }

    // Java: int getTextWrap()
    pub fn getTextWrap(&self) -> i32 {
      // Java: return this.mProperty.getInt(811, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(811, 0)
    }

    // Java: void setTextWrapType(int wrap)
    pub fn setTextWrapType(&mut self, wrap: i32) {
      // Java: this.mProperty.setInt(819, wrap);
      self.base.mProperty.as_mut().unwrap().setInt(819, wrap);
    }

    // Java: int getTextWrapType()
    pub fn getTextWrapType(&self) -> i32 {
      // Java: return this.mProperty.getInt(819, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(819, 0)
    }

    // Java: void setRotation(float degree)
    pub fn setRotation(&mut self, degree: f32) {
      // Java: this.mProperty.setValue(24, degree);
      self.base.mProperty.as_mut().unwrap().setFloat(24, degree);
    }

    // Java: float getRotation()
    pub fn getRotation(&self) -> f32 {
      // Java: return this.mProperty.getFloat(24, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(24, 0.0_f32)
    }

    // Java: void setAllowOverlap(boolean bAllow)
    pub fn setAllowOverlap(&mut self, bAllow: bool) {
      // Java: this.mProperty.setBoolean(39, bAllow);
      self.base.mProperty.as_mut().unwrap().setBoolean(39, bAllow);
    }

    // Java: boolean getAllowOverlap()
    pub fn getAllowOverlap(&self) -> bool {
      // Java: return this.mProperty.getBoolean(39, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(39, true)
    }

    // Java: boolean getLayoutInCell()
    pub fn getLayoutInCell(&self) -> bool {
      // Java: return this.mProperty.getBoolean(40, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(40, true)
    }

    // Java: void setLayoutInCell(boolean bInCell)
    pub fn setLayoutInCell(&mut self, bInCell: bool) {
      // Java: this.mProperty.setBoolean(40, bInCell);
      self.base.mProperty.as_mut().unwrap().setBoolean(40, bInCell);
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: super.dispose();
      self.base.dispose();
    }

}

impl IPos for ShapePos {
    fn contains(&mut self, var1: i32) -> bool { self.base.contains(var1) }
    fn setShapeRect(&mut self, var1: RectF) {
        self.base.mProperty.as_mut().unwrap().setValue(23, Some(Arc::new(var1)));
    }
    fn getShapeRect(&self) -> RectF {
        let val = self.base.mProperty.as_ref().unwrap().base.getValue(23);
        val.and_then(|v| v.downcast_ref::<RectF>().cloned()).unwrap_or_else(RectF::new)
    }
    fn setRelativeH(&mut self, var1: i32) { ShapePos::setRelativeH(self, var1) }
    fn getRelativeH(&self) -> i32 { ShapePos::getRelativeH(self) }
    fn getRelativeX(&self) -> i32 { ShapePos::getRelativeX(self) }
    fn setPosRelH(&mut self, var1: i32) { ShapePos::setPosRelH(self, var1) }
    fn getPosRelH(&self) -> i32 { ShapePos::getPosRelH(self) }
    fn setRelativeV(&mut self, var1: i32) { ShapePos::setRelativeV(self, var1) }
    fn getRelativeV(&self) -> i32 { ShapePos::getRelativeV(self) }
    fn getRelativeY(&self) -> i32 { ShapePos::getRelativeY(self) }
    fn setPosRelV(&mut self, var1: i32) { ShapePos::setPosRelV(self, var1) }
    fn getPosRelV(&self) -> i32 { ShapePos::getPosRelV(self) }
    fn setPosHPercent(&mut self, var1: f32) { ShapePos::setPosHPercent(self, var1) }
    fn getPosHPercent(&self) -> f32 { ShapePos::getPosHPercent(self) }
    fn getPosHPercentObj(&self) -> Option<f32> { ShapePos::getPosHPercentObj(self) }
    fn setPosVPercent(&mut self, var1: f32) { ShapePos::setPosVPercent(self, var1) }
    fn getPosVPercent(&self) -> f32 { ShapePos::getPosVPercent(self) }
    fn getPosVPercentObj(&self) -> Option<f32> { ShapePos::getPosVPercentObj(self) }
    fn setAlignmentH(&mut self, var1: i32) { ShapePos::setAlignmentH(self, var1) }
    fn getAlignmentH(&self) -> i32 { ShapePos::getAlignmentH(self) }
    fn setAlignmentV(&mut self, var1: i32) { ShapePos::setAlignmentV(self, var1) }
    fn getAlignmentV(&self) -> i32 { ShapePos::getAlignmentV(self) }
    fn setSizeRelH(&mut self, var1: i32) { ShapePos::setSizeRelH(self, var1) }
    fn getSizeRelH(&self) -> i32 { ShapePos::getSizeRelH(self) }
    fn getSizeRelHObj(&self) -> Option<i32> { ShapePos::getSizeRelHObj(self) }
    fn setSizeRelV(&mut self, var1: i32) { ShapePos::setSizeRelV(self, var1) }
    fn getSizeRelV(&self) -> i32 { ShapePos::getSizeRelV(self) }
    fn getSizeRelVObj(&self) -> Option<i32> { ShapePos::getSizeRelVObj(self) }
    fn setSizeHPercent(&mut self, var1: f32) { ShapePos::setSizeHPercent(self, var1) }
    fn getSizeHPercent(&self) -> f32 { ShapePos::getSizeHPercent(self) }
    fn getSizeHPercentObj(&self) -> Option<f32> { ShapePos::getSizeHPercentObj(self) }
    fn setSizeVPercent(&mut self, var1: f32) { ShapePos::setSizeVPercent(self, var1) }
    fn getSizeVPercent(&self) -> f32 { ShapePos::getSizeVPercent(self) }
    fn getSizeVPercentObj(&self) -> Option<f32> { ShapePos::getSizeVPercentObj(self) }
    fn setScaleH(&mut self, var1: f32) { ShapePos::setScaleH(self, var1) }
    fn getScaleH(&self) -> f32 { ShapePos::getScaleH(self) }
    fn setScaleV(&mut self, var1: f32) { ShapePos::setScaleV(self, var1) }
    fn getScaleV(&self) -> f32 { ShapePos::getScaleV(self) }
    fn setWrapDistLeft(&mut self, var1: f32) { ShapePos::setWrapDistLeft(self, var1) }
    fn getWrapDistLeft(&self) -> f32 { ShapePos::getWrapDistLeft(self) }
    fn getWrapDistLeftObj(&self) -> Option<f32> { ShapePos::getWrapDistLeftObj(self) }
    fn setWrapDistTop(&mut self, var1: f32) { ShapePos::setWrapDistTop(self, var1) }
    fn getWrapDistTop(&self) -> f32 { ShapePos::getWrapDistTop(self) }
    fn getWrapDistTopObj(&self) -> Option<f32> { ShapePos::getWrapDistTopObj(self) }
    fn setWrapDistRight(&mut self, var1: f32) { ShapePos::setWrapDistRight(self, var1) }
    fn getWrapDistRight(&self) -> f32 { ShapePos::getWrapDistRight(self) }
    fn getWrapDistRightObj(&self) -> Option<f32> { ShapePos::getWrapDistRightObj(self) }
    fn setWrapDistBottom(&mut self, var1: f32) { ShapePos::setWrapDistBottom(self, var1) }
    fn getWrapDistBottom(&self) -> f32 { ShapePos::getWrapDistBottom(self) }
    fn getWrapDistBottomObj(&self) -> Option<f32> { ShapePos::getWrapDistBottomObj(self) }
    fn setTextWrap(&mut self, var1: i32) { ShapePos::setTextWrap(self, var1) }
    fn getTextWrap(&self) -> i32 { ShapePos::getTextWrap(self) }
    fn setTextWrapType(&mut self, var1: i32) { ShapePos::setTextWrapType(self, var1) }
    fn getTextWrapType(&self) -> i32 { ShapePos::getTextWrapType(self) }
    fn setRotation(&mut self, var1: f32) { ShapePos::setRotation(self, var1) }
    fn getRotation(&self) -> f32 { ShapePos::getRotation(self) }
    fn setAllowOverlap(&mut self, var1: bool) { ShapePos::setAllowOverlap(self, var1) }
    fn getAllowOverlap(&self) -> bool { ShapePos::getAllowOverlap(self) }
    fn getLayoutInCell(&self) -> bool { ShapePos::getLayoutInCell(self) }
    fn setLayoutInCell(&mut self, var1: bool) { ShapePos::setLayoutInCell(self, var1) }
    fn isEmpty(&mut self) -> bool { self.base.isEmpty() }
    fn dispose(&mut self) { ShapePos::dispose(self) }
}

#![allow(non_snake_case, dead_code)]

use super::Effect::Effect;
use std::ops::Deref;

// Java: public class Reflection extends Effect implements Cloneable
pub struct Reflection {
    pub base: Effect,
}

pub trait Cloneable {}
impl Cloneable for Reflection {}

impl Deref for Reflection {
    type Target = Effect;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Reflection extends Effect implements Cloneable
impl Reflection {
    // Java: Reflection()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent Effect, merged into struct init
      Reflection { base: Effect::new() }
    }

    // Java: Reflection clone()
    pub fn clone(&mut self) -> Result<Reflection, String> {
      // Java: return (Reflection)super.clone();
      Ok(Reflection { base: self.base.clone() })
    }

    // Java: void setReflectionAlgn(int align)
    pub fn setReflectionAlgn(&mut self, align: i32) {
      // Java: this.mProperty.setInt(233, align);
      self.base.base.mProperty.as_mut().unwrap().setInt(233, align);
    }

    // Java: int getReflectionAlgn()
    pub fn getReflectionAlgn(&self) -> i32 {
      // Java: return this.mProperty.getInt(233, 0);
      self.base.base.mProperty.as_ref().unwrap().base.getInt_default(233, 0)
    }

    // Java: void setReflectionDir(float dir)
    pub fn setReflectionDir(&mut self, dir: f32) {
      // Java: this.mProperty.setFloat(234, dir);
      self.base.base.mProperty.as_mut().unwrap().setFloat(234, dir);
    }

    // Java: float getReflectionDir()
    pub fn getReflectionDir(&self) -> f32 {
      // Java: return this.mProperty.getFloat(234, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(234, 0.0_f32)
    }

    // Java: void setReflectionDist(float dist)
    pub fn setReflectionDist(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(235, dist);
      self.base.base.mProperty.as_mut().unwrap().setFloat(235, dist);
    }

    // Java: float getReflectionDist()
    pub fn getReflectionDist(&self) -> f32 {
      // Java: return this.mProperty.getFloat(235, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(235, 0.0_f32)
    }

    // Java: void setReflectionEndAlpha(float endAlpha)
    pub fn setReflectionEndAlpha(&mut self, endAlpha: f32) {
      // Java: this.mProperty.setFloat(236, endAlpha);
      self.base.base.mProperty.as_mut().unwrap().setFloat(236, endAlpha);
    }

    // Java: float getReflectionEndAlpha()
    pub fn getReflectionEndAlpha(&self) -> f32 {
      // Java: return this.mProperty.getFloat(236, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(236, 0.0_f32)
    }

    // Java: void setReflectionEndPos(float endPos)
    pub fn setReflectionEndPos(&mut self, endPos: f32) {
      // Java: this.mProperty.setFloat(237, endPos);
      self.base.base.mProperty.as_mut().unwrap().setFloat(237, endPos);
    }

    // Java: float getReflectionEndPos()
    pub fn getReflectionEndPos(&self) -> f32 {
      // Java: return this.mProperty.getFloat(237, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(237, 1.0_f32)
    }

    // Java: void setReflectionStartAlpha(float startAlpha)
    pub fn setReflectionStartAlpha(&mut self, startAlpha: f32) {
      // Java: this.mProperty.setFloat(238, startAlpha);
      self.base.base.mProperty.as_mut().unwrap().setFloat(238, startAlpha);
    }

    // Java: float getReflectionStartAlpha()
    pub fn getReflectionStartAlpha(&self) -> f32 {
      // Java: return this.mProperty.getFloat(238, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(238, 1.0_f32)
    }

    // Java: void setReflectionStartPos(float startPos)
    pub fn setReflectionStartPos(&mut self, startPos: f32) {
      // Java: this.mProperty.setFloat(239, startPos);
      self.base.base.mProperty.as_mut().unwrap().setFloat(239, startPos);
    }

    // Java: float getReflectionStartPos()
    pub fn getReflectionStartPos(&self) -> f32 {
      // Java: return this.mProperty.getFloat(239, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(239, 0.0_f32)
    }

    // Java: void setReflectionFadeDir(float fadeDir)
    pub fn setReflectionFadeDir(&mut self, fadeDir: f32) {
      // Java: this.mProperty.setFloat(240, fadeDir);
      self.base.base.mProperty.as_mut().unwrap().setFloat(240, fadeDir);
    }

    // Java: float getReflectionFadeDir()
    pub fn getReflectionFadeDir(&self) -> f32 {
      // Java: return this.mProperty.getFloat(240, 90.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(240, 90.0_f32)
    }

    // Java: void setReflectionSkewX(float skewX)
    pub fn setReflectionSkewX(&mut self, skewX: f32) {
      // Java: this.mProperty.setFloat(241, skewX);
      self.base.base.mProperty.as_mut().unwrap().setFloat(241, skewX);
    }

    // Java: float getReflectionSkewX()
    pub fn getReflectionSkewX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(241, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(241, 0.0_f32)
    }

    // Java: void setReflectionSkewY(float skewY)
    pub fn setReflectionSkewY(&mut self, skewY: f32) {
      // Java: this.mProperty.setFloat(241, skewY);
      self.base.base.mProperty.as_mut().unwrap().setFloat(241, skewY);
    }

    // Java: float getReflectionSkewY()
    pub fn getReflectionSkewY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(242, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(242, 0.0_f32)
    }

    // Java: void setReflectionRotWithShape(boolean rotWithShape)
    pub fn setReflectionRotWithShape(&mut self, rotWithShape: bool) {
      // Java: this.mProperty.setBoolean(243, rotWithShape);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(243, rotWithShape);
    }

    // Java: boolean getReflectionRotWithShape()
    pub fn getReflectionRotWithShape(&self) -> bool {
      // Java: return this.mProperty.getBoolean(243, true);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(243, true)
    }

    // Java: void setReflectionScaleX(float scaleX)
    pub fn setReflectionScaleX(&mut self, scaleX: f32) {
      // Java: this.mProperty.setFloat(244, scaleX);
      self.base.base.mProperty.as_mut().unwrap().setFloat(244, scaleX);
    }

    // Java: float getReflectionScaleX()
    pub fn getReflectionScaleX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(244, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(244, 1.0_f32)
    }

    // Java: void setReflectionScaleY(float scaleY)
    pub fn setReflectionScaleY(&mut self, scaleY: f32) {
      // Java: this.mProperty.setFloat(245, scaleY);
      self.base.base.mProperty.as_mut().unwrap().setFloat(245, scaleY);
    }

    // Java: float getReflectionScaleY()
    pub fn getReflectionScaleY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(245, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(245, 1.0_f32)
    }

    // Java: void setReflectionBlurRad(float rad)
    pub fn setReflectionBlurRad(&mut self, rad: f32) {
      // Java: this.mProperty.setFloat(246, rad);
      self.base.base.mProperty.as_mut().unwrap().setFloat(246, rad);
    }

    // Java: float getReflectionBlurRad()
    pub fn getReflectionBlurRad(&self) -> f32 {
      // Java: return this.mProperty.getFloat(246, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(246, 0.0_f32)
    }

}

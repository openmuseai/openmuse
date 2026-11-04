#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Callout extends PropBase
pub struct Callout {
    pub base: PropBase,
}

impl Deref for Callout {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Callout extends PropBase
impl Callout {
    // Java: Callout()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
      Callout { base: PropBase::new() }
    }

    // Java: Callout clone()
    pub fn clone(&self) -> Result<Callout, String> {
      // Java: return (Callout)super.clone();
      Ok(Callout { base: self.base.clone()})
    }

    // Java: void setOn(boolean v)
    pub fn setOn(&mut self, v: bool) {
      // Java: this.mProperty.setBoolean(125, v);
      self.base.mProperty.as_mut().unwrap().setBoolean(125, v);
    }

    // Java: boolean getOn()
    pub fn getOn(&self) -> bool {
      // Java: return this.mProperty.getBoolean(125, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(125, false)
    }

    // Java: Boolean getOnObj()
    pub fn getOnObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(125);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(125);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setGap(float gap)
    pub fn setGap(&mut self, gap: f32) {
      // Java: this.mProperty.setFloat(114, gap);
      self.base.mProperty.as_mut().unwrap().setFloat(114, gap);
    }

    // Java: float getGap()
    pub fn getGap(&self) -> f32 {
      // Java: return this.mProperty.getFloat(114, 6.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(114, 6.0_f32)
    }

    // Java: Float getGapObj()
    pub fn getGapObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(114);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(114);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
      object
    }

    // Java: void setAngle(int angle)
    pub fn setAngle(&mut self, angle: i32) {
      // Java: this.mProperty.setInt(115, angle);
      self.base.mProperty.as_mut().unwrap().setInt(115, angle);
    }

    // Java: int getAngle()
    pub fn getAngle(&self) -> i32 {
      // Java: return this.mProperty.getInt(115, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(115, 0)
    }

    // Java: Integer getAngleObj()
    pub fn getAngleObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(115);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(115);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
      object
    }

    // Java: void setDropType(int type)
    pub fn setDropType(&mut self, r#type: i32) {
      // Java: this.mProperty.setInt(116, type);
      self.base.mProperty.as_mut().unwrap().setInt(116, r#type);
    }

    // Java: int getDropType()
    pub fn getDropType(&self) -> i32 {
      // Java: return this.mProperty.getInt(116, 3);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(116, 3)
    }

    // Java: Integer getDropTypeObj()
    pub fn getDropTypeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(116);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(116);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
      object
    }

    // Java: void setDrop(float drop)
    pub fn setDrop(&mut self, drop: f32) {
      // Java: this.mProperty.setFloat(117, drop);
      self.base.mProperty.as_mut().unwrap().setFloat(117, drop);
    }

    // Java: float getDrop()
    pub fn getDrop(&self) -> f32 {
      // Java: return this.mProperty.getFloat(117, 9.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(117, 9.0_f32)
    }

    // Java: Float getDropObj()
    pub fn getDropObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(117);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(117);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
      object
    }

    // Java: void setLength(float length)
    pub fn setLength(&mut self, length: f32) {
      // Java: this.mProperty.setFloat(118, length);
      self.base.mProperty.as_mut().unwrap().setFloat(118, length);
    }

    // Java: float getLength()
    pub fn getLength(&self) -> f32 {
      // Java: return this.mProperty.getFloat(118, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(118, 0.0_f32)
    }

    // Java: Float getLengthObj()
    pub fn getLengthObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(118);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(118);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
      object
    }

    // Java: void setAccentBar(boolean bBar)
    pub fn setAccentBar(&mut self, bBar: bool) {
      // Java: this.mProperty.setBoolean(119, bBar);
      self.base.mProperty.as_mut().unwrap().setBoolean(119, bBar);
    }

    // Java: boolean getAccentBar()
    pub fn getAccentBar(&self) -> bool {
      // Java: return this.mProperty.getBoolean(119, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(119, false)
    }

    // Java: Boolean getAccentBarObj()
    pub fn getAccentBarObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(119);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(119);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setTextBorder(boolean bBorder)
    pub fn setTextBorder(&mut self, bBorder: bool) {
      // Java: this.mProperty.setBoolean(120, bBorder);
      self.base.mProperty.as_mut().unwrap().setBoolean(120, bBorder);
    }

    // Java: boolean getTextBorder()
    pub fn getTextBorder(&self) -> bool {
      // Java: return this.mProperty.getBoolean(120, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(120, true)
    }

    // Java: Boolean getTextBorderObj()
    pub fn getTextBorderObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(120);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(120);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setMinusX(boolean bMinus)
    pub fn setMinusX(&mut self, bMinus: bool) {
      // Java: this.mProperty.setBoolean(121, bMinus);
      self.base.mProperty.as_mut().unwrap().setBoolean(121, bMinus);
    }

    // Java: boolean getMinusX()
    pub fn getMinusX(&self) -> bool {
      // Java: return this.mProperty.getBoolean(121, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(121, false)
    }

    // Java: Boolean getMinusXObj()
    pub fn getMinusXObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(121);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(121);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setMinusY(boolean bMinus)
    pub fn setMinusY(&mut self, bMinus: bool) {
      // Java: this.mProperty.setBoolean(122, bMinus);
      self.base.mProperty.as_mut().unwrap().setBoolean(122, bMinus);
    }

    // Java: boolean getMinusY()
    pub fn getMinusY(&self) -> bool {
      // Java: return this.mProperty.getBoolean(122, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(122, false)
    }

    // Java: Boolean getMinusYObj()
    pub fn getMinusYObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(122);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(122);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setDropAuto(boolean bDrop)
    pub fn setDropAuto(&mut self, bDrop: bool) {
      // Java: this.mProperty.setBoolean(123, bDrop);
      self.base.mProperty.as_mut().unwrap().setBoolean(123, bDrop);
    }

    // Java: boolean getDropAuto()
    pub fn getDropAuto(&self) -> bool {
      // Java: return this.mProperty.getBoolean(123, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(123, false)
    }

    // Java: Boolean getDropAutoObj()
    pub fn getDropAutoObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(123);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(123);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void setLengthSpecified(boolean bLengthSpecified)
    pub fn setLengthSpecified(&mut self, bLengthSpecified: bool) {
      // Java: this.mProperty.setBoolean(124, bLengthSpecified);
      self.base.mProperty.as_mut().unwrap().setBoolean(124, bLengthSpecified);
    }

    // Java: boolean getLengthSpecified()
    pub fn getLengthSpecified(&self) -> bool {
      // Java: return this.mProperty.getBoolean(124, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(124, false)
    }

    // Java: Boolean getLengthSpecifiedObj()
    pub fn getLengthSpecifiedObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(124);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(124);
      // Java: return object == null ? null : (Boolean)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
      object
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &[u8]) -> Result<(), String> {
      // Java: super.readExternal(input);
      self.base.readExternal(input)
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut Vec<u8>) -> Result<(), String> {
      // Java: super.writeExternal(output);
      self.base.writeExternal(output)
    }

}

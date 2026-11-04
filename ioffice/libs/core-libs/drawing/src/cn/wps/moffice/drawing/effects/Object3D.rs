#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.DefaultShapeValue
use super::super::DefaultShapeValue::DefaultShapeValue;
use super::Effect::Effect;
use super::ExtrusionPlane::ExtrusionPlane;

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Object3D extends Effect implements Cloneable
pub struct Object3D {
    pub base: Effect,
}

pub trait Cloneable {}
impl Cloneable for Object3D {}

impl Deref for Object3D {
    type Target = Effect;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Object3D extends Effect implements Cloneable
impl Object3D {
    // Java: Object3D()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent Effect, merged into struct init
      Object3D { base: Effect::new() }
    }

    // Java: Object3D clone()
    pub fn clone(&self) -> Result<Object3D, String> {
      // Java: return (Object3D)super.clone();
      Ok(Object3D { base: self.base.clone()? })
    }

    // Java: void setSpecularAmt(float amt)
    pub fn setSpecularAmt(&mut self, amt: f32) {
      // Java: this.mProperty.setFloat(333, amt);
      self.base.base.mProperty.as_mut().unwrap().setFloat(333, amt);
    }

    // Java: float getSpecularAmt()
    pub fn getSpecularAmt(&self) -> f32 {
      // Java: return this.mProperty.getFloat(333, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(333, 0.0_f32)
    }

    // Java: void setDiffuseAmt(float amt)
    pub fn setDiffuseAmt(&mut self, amt: f32) {
      // Java: this.mProperty.setFloat(334, amt);
      self.base.base.mProperty.as_mut().unwrap().setFloat(334, amt);
    }

    // Java: float getDiffuseAmt()
    pub fn getDiffuseAmt(&self) -> f32 {
      // Java: return this.mProperty.getFloat(334, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(334, 1.0_f32)
    }

    // Java: void setShininess(float shininess)
    pub fn setShininess(&mut self, shininess: f32) {
      // Java: this.mProperty.setFloat(335, shininess);
      self.base.base.mProperty.as_mut().unwrap().setFloat(335, shininess);
    }

    // Java: float getShininess()
    pub fn getShininess(&self) -> f32 {
      // Java: return this.mProperty.getFloat(335, 5.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(335, 5.0_f32)
    }

    // Java: void setEdgeThickness(float edgeThickness)
    pub fn setEdgeThickness(&mut self, edgeThickness: f32) {
      // Java: this.mProperty.setFloat(336, edgeThickness);
      self.base.base.mProperty.as_mut().unwrap().setFloat(336, edgeThickness);
    }

    // Java: float getEdgeThickness()
    pub fn getEdgeThickness(&self) -> f32 {
      // Java: return this.mProperty.getFloat(336, 1.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(336, 1.0_f32)
    }

    // Java: void setExtrudeForward(float extrude)
    pub fn setExtrudeForward(&mut self, extrude: f32) {
      // Java: this.mProperty.setFloat(337, extrude);
      self.base.base.mProperty.as_mut().unwrap().setFloat(337, extrude);
    }

    // Java: float getExtrudeForward()
    pub fn getExtrudeForward(&self) -> f32 {
      // Java: return this.mProperty.getFloat(337, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(337, 0.0_f32)
    }

    // Java: void setExtrudeBackward(float extrude)
    pub fn setExtrudeBackward(&mut self, extrude: f32) {
      // Java: this.mProperty.setFloat(338, extrude);
      self.base.base.mProperty.as_mut().unwrap().setFloat(338, extrude);
    }

    // Java: float getExtrudeBackward()
    pub fn getExtrudeBackward(&self) -> f32 {
      // Java: return this.mProperty.getFloat(338, 36.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(338, 36.0_f32)
    }

    // Java: void setExtrudePlane(ExtrusionPlane plane)
    pub fn setExtrudePlane(&mut self, plane: Element) {
      // Java: this.mProperty.setValue(339, plane);
      self.base.base.mProperty.as_mut().unwrap().setValue(339, plane);
    }

    // Java: ExtrusionPlane getExtrudePlane()
    pub fn getExtrudePlane(&self) -> ExtrusionPlane {
      // Java: Object plane = this.mProperty.getValue(339);
      let plane = self.base.base.mProperty.as_ref().unwrap().base.getValue(339);
      // Java: return plane == null ? ExtrusionPlane.XY : (ExtrusionPlane)plane;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let plane = plane.and_then(|v| v.downcast_ref::<ExtrusionPlane>().copied()).unwrap_or(ExtrusionPlane::XY);
      plane
    }

    // Java: void setExtrusionColor(int color)
    pub fn setExtrusionColor(&mut self, color: i32) {
      // Java: this.mProperty.setInt(340, color);
      self.base.base.mProperty.as_mut().unwrap().setInt(340, color);
    }

    // Java: int getExtrusionColor()
    pub fn getExtrusionColor(&self) -> i32 {
      // Java: return this.mProperty.getInt(340, 0);
      self.base.base.mProperty.as_ref().unwrap().base.getInt_default(340, 0)
    }

    // Java: Integer getExtrusionColorObj()
    pub fn getExtrusionColorObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(340);
      let object = self.base.base.mProperty.as_ref().unwrap().base.getValue(340);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
      object
    }

    // Java: void setMetallic(boolean f3DMetallic)
    pub fn setMetallic(&mut self, f3DMetallic: bool) {
      // Java: this.mProperty.setBoolean(341, f3DMetallic);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(341, f3DMetallic);
    }

    // Java: boolean getMetallic()
    pub fn getMetallic(&self) -> bool {
      // Java: return this.mProperty.getBoolean(341, false);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(341, false)
    }

    // Java: void setUseExtrusionColor(boolean fUse)
    pub fn setUseExtrusionColor(&mut self, fUse: bool) {
      // Java: this.mProperty.setBoolean(342, fUse);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(342, fUse);
    }

    // Java: boolean getUseExtrusionColor()
    pub fn getUseExtrusionColor(&self) -> bool {
      // Java: return this.mProperty.getBoolean(342, false);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(342, false)
    }

    // Java: void setLightFace(boolean fLightFace)
    pub fn setLightFace(&mut self, fLightFace: bool) {
      // Java: this.mProperty.setBoolean(343, fLightFace);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(343, fLightFace);
    }

    // Java: boolean getLightFace()
    pub fn getLightFace(&self) -> bool {
      // Java: return this.mProperty.getBoolean(343, true);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(343, true)
    }

    // Java: void set3D(boolean b3D)
    pub fn set3D(&mut self, b3D: bool) {
      // Java: this.mProperty.setBoolean(344, b3D);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(344, b3D);
    }

    // Java: boolean get3D()
    pub fn get3D(&self) -> bool {
      // Java: return this.mProperty.getBoolean(344, false);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(344, false)
    }

    // Java: void setRotationAngle(float angle)
    pub fn setRotationAngle(&mut self, angle: f32) {
      // Java: this.mProperty.setFloat(345, angle);
      self.base.base.mProperty.as_mut().unwrap().setFloat(345, angle);
    }

    // Java: float getRotationAngle()
    pub fn getRotationAngle(&self) -> f32 {
      // Java: return this.mProperty.getFloat(345, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(345, 0.0_f32)
    }

    // Java: void setXRotationAngle(float angle)
    pub fn setXRotationAngle(&mut self, angle: f32) {
      // Java: this.mProperty.setFloat(375, angle);
      self.base.base.mProperty.as_mut().unwrap().setFloat(375, angle);
    }

    // Java: float getXRotationAngle()
    pub fn getXRotationAngle(&self) -> f32 {
      // Java: return this.mProperty.getFloat(375, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(375, 0.0_f32)
    }

    // Java: void setYRotationAngle(float angle)
    pub fn setYRotationAngle(&mut self, angle: f32) {
      // Java: this.mProperty.setFloat(376, angle);
      self.base.base.mProperty.as_mut().unwrap().setFloat(376, angle);
    }

    // Java: float getYRotationAngle()
    pub fn getYRotationAngle(&self) -> f32 {
      // Java: return this.mProperty.getFloat(376, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(376, 0.0_f32)
    }

    // Java: void setRotationAxisX(float axis)
    pub fn setRotationAxisX(&mut self, axis: f32) {
      // Java: this.mProperty.setFloat(346, axis);
      self.base.base.mProperty.as_mut().unwrap().setFloat(346, axis);
    }

    // Java: float getRotationAxisX()
    pub fn getRotationAxisX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(346, 100.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(346, 100.0_f32)
    }

    // Java: void setRotationAxisY(float axis)
    pub fn setRotationAxisY(&mut self, axis: f32) {
      // Java: this.mProperty.setFloat(347, axis);
      self.base.base.mProperty.as_mut().unwrap().setFloat(347, axis);
    }

    // Java: float getRotationAxisY()
    pub fn getRotationAxisY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(347, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(347, 0.0_f32)
    }

    // Java: void setRotationAxisZ(float axis)
    pub fn setRotationAxisZ(&mut self, axis: f32) {
      // Java: this.mProperty.setFloat(348, axis);
      self.base.base.mProperty.as_mut().unwrap().setFloat(348, axis);
    }

    // Java: float getRotationAxisZ()
    pub fn getRotationAxisZ(&self) -> f32 {
      // Java: return this.mProperty.getFloat(348, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(348, 0.0_f32)
    }

    // Java: void setRotationCenterX(float center)
    pub fn setRotationCenterX(&mut self, center: f32) {
      // Java: this.mProperty.setFloat(349, center);
      self.base.base.mProperty.as_mut().unwrap().setFloat(349, center);
    }

    // Java: float getRotationCenterX()
    pub fn getRotationCenterX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(349, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(349, 0.0_f32)
    }

    // Java: void setRotationCenterY(float center)
    pub fn setRotationCenterY(&mut self, center: f32) {
      // Java: this.mProperty.setFloat(350, center);
      self.base.base.mProperty.as_mut().unwrap().setFloat(350, center);
    }

    // Java: float getRotationCenterY()
    pub fn getRotationCenterY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(350, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(350, 0.0_f32)
    }

    // Java: void setRotationCenterZ(float center)
    pub fn setRotationCenterZ(&mut self, center: f32) {
      // Java: this.mProperty.setFloat(351, center);
      self.base.base.mProperty.as_mut().unwrap().setFloat(351, center);
    }

    // Java: float getRotationCenterZ()
    pub fn getRotationCenterZ(&self) -> f32 {
      // Java: return this.mProperty.getFloat(351, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(351, 0.0_f32)
    }

    // Java: int getRenderMode()
    pub fn getRenderMode(&self) -> i32 {
      // Java: return this.mProperty.getInt(352, 0);
      self.base.base.mProperty.as_ref().unwrap().base.getInt_default(352, 0)
    }

    // Java: void setRenderMode(int mode)
    pub fn setRenderMode(&mut self, mode: i32) {
      // Java: this.mProperty.setInt(352, mode);
      self.base.base.mProperty.as_mut().unwrap().setInt(352, mode);
    }

    // Java: float getTolerance()
    pub fn getTolerance(&self) -> f32 {
      // Java: return this.mProperty.getFloat(353, 30000.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(353, 30000.0_f32)
    }

    // Java: void setTolerance(float tolerance)
    pub fn setTolerance(&mut self, tolerance: f32) {
      // Java: this.mProperty.setFloat(353, tolerance);
      self.base.base.mProperty.as_mut().unwrap().setFloat(353, tolerance);
    }

    // Java: void setViewpointX(float viewPoint)
    pub fn setViewpointX(&mut self, viewPoint: f32) {
      // Java: this.mProperty.setFloat(354, viewPoint);
      self.base.base.mProperty.as_mut().unwrap().setFloat(354, viewPoint);
    }

    // Java: float getViewpointX()
    pub fn getViewpointX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(354, DefaultShapeValue.c3DXViewpoint);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(354, DefaultShapeValue::c3DXViewpoint())
    }

    // Java: void setViewpointY(float viewPoint)
    pub fn setViewpointY(&mut self, viewPoint: f32) {
      // Java: this.mProperty.setFloat(355, viewPoint);
      self.base.base.mProperty.as_mut().unwrap().setFloat(355, viewPoint);
    }

    // Java: float getViewpointY()
    pub fn getViewpointY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(355, DefaultShapeValue.c3DYViewpoint);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(355, DefaultShapeValue::c3DYViewpoint())
    }

    // Java: void setViewpointZ(float viewPoint)
    pub fn setViewpointZ(&mut self, viewPoint: f32) {
      // Java: this.mProperty.setFloat(356, viewPoint);
      self.base.base.mProperty.as_mut().unwrap().setFloat(356, viewPoint);
    }

    // Java: float getViewpointZ()
    pub fn getViewpointZ(&self) -> f32 {
      // Java: return this.mProperty.getFloat(356, DefaultShapeValue.c3DZViewpoint);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(356, DefaultShapeValue::c3DZViewpoint())
    }

    // Java: void setOriginX(float origin)
    pub fn setOriginX(&mut self, origin: f32) {
      // Java: this.mProperty.setFloat(357, origin);
      self.base.base.mProperty.as_mut().unwrap().setFloat(357, origin);
    }

    // Java: float getOriginX()
    pub fn getOriginX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(357, 0.5F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(357, 0.5_f32)
    }

    // Java: void setOriginY(float origin)
    pub fn setOriginY(&mut self, origin: f32) {
      // Java: this.mProperty.setFloat(358, origin);
      self.base.base.mProperty.as_mut().unwrap().setFloat(358, origin);
    }

    // Java: float getOriginY()
    pub fn getOriginY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(358, -0.5F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(358, -0.5_f32)
    }

    // Java: float getSkewAngle()
    pub fn getSkewAngle(&self) -> f32 {
      // Java: return this.mProperty.getFloat(359, 225.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(359, 225.0_f32)
    }

    // Java: void setSkewAngle(float angle)
    pub fn setSkewAngle(&mut self, angle: f32) {
      // Java: this.mProperty.setFloat(359, angle);
      self.base.base.mProperty.as_mut().unwrap().setFloat(359, angle);
    }

    // Java: float getSkewAmount()
    pub fn getSkewAmount(&self) -> f32 {
      // Java: return this.mProperty.getFloat(360, 50.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(360, 50.0_f32)
    }

    // Java: void setSkewAmount(float amount)
    pub fn setSkewAmount(&mut self, amount: f32) {
      // Java: this.mProperty.setFloat(360, amount);
      self.base.base.mProperty.as_mut().unwrap().setFloat(360, amount);
    }

    // Java: float getAmbientIntensity()
    pub fn getAmbientIntensity(&self) -> f32 {
      // Java: return this.mProperty.getFloat(361, 0.3F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(361, 0.3_f32)
    }

    // Java: void setAmbientIntensity(float intensity)
    pub fn setAmbientIntensity(&mut self, intensity: f32) {
      // Java: this.mProperty.setFloat(361, intensity);
      self.base.base.mProperty.as_mut().unwrap().setFloat(361, intensity);
    }

    // Java: void set3DKeyX(float key)
    pub fn set3DKeyX(&mut self, key: f32) {
      // Java: this.mProperty.setFloat(362, key);
      self.base.base.mProperty.as_mut().unwrap().setFloat(362, key);
    }

    // Java: float get3DKeyX()
    pub fn get3DKeyX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(362, DefaultShapeValue.c3DKeyX);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(362, DefaultShapeValue::c3DKeyX())
    }

    // Java: void set3DKeyY(float key)
    pub fn set3DKeyY(&mut self, key: f32) {
      // Java: this.mProperty.setFloat(363, key);
      self.base.base.mProperty.as_mut().unwrap().setFloat(363, key);
    }

    // Java: float get3DKeyY()
    pub fn get3DKeyY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(363, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(363, 0.0_f32)
    }

    // Java: void set3DKeyZ(float key)
    pub fn set3DKeyZ(&mut self, key: f32) {
      // Java: this.mProperty.setFloat(364, key);
      self.base.base.mProperty.as_mut().unwrap().setFloat(364, key);
    }

    // Java: float get3DKeyZ()
    pub fn get3DKeyZ(&self) -> f32 {
      // Java: return this.mProperty.getFloat(364, DefaultShapeValue.c3DKeyZ);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(364, DefaultShapeValue::c3DKeyZ())
    }

    // Java: float getKeyIntensity()
    pub fn getKeyIntensity(&self) -> f32 {
      // Java: return this.mProperty.getFloat(365, 0.6F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(365, 0.6_f32)
    }

    // Java: void setKeyIntensity(float intensity)
    pub fn setKeyIntensity(&mut self, intensity: f32) {
      // Java: this.mProperty.setFloat(365, intensity);
      self.base.base.mProperty.as_mut().unwrap().setFloat(365, intensity);
    }

    // Java: void set3DFillX(float fill)
    pub fn set3DFillX(&mut self, fill: f32) {
      // Java: this.mProperty.setFloat(366, fill);
      self.base.base.mProperty.as_mut().unwrap().setFloat(366, fill);
    }

    // Java: float get3DFillX()
    pub fn get3DFillX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(366, DefaultShapeValue.c3DFillX);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(366, DefaultShapeValue::c3DFillX())
    }

    // Java: void set3DFillY(float fill)
    pub fn set3DFillY(&mut self, fill: f32) {
      // Java: this.mProperty.setFloat(367, fill);
      self.base.base.mProperty.as_mut().unwrap().setFloat(367, fill);
    }

    // Java: float get3DFillY()
    pub fn get3DFillY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(367, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(367, 0.0_f32)
    }

    // Java: void set3DFillZ(float fill)
    pub fn set3DFillZ(&mut self, fill: f32) {
      // Java: this.mProperty.setFloat(368, fill);
      self.base.base.mProperty.as_mut().unwrap().setFloat(368, fill);
    }

    // Java: float get3DFillZ()
    pub fn get3DFillZ(&self) -> f32 {
      // Java: return this.mProperty.getFloat(368, DefaultShapeValue.c3DFillZ);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(368, DefaultShapeValue::c3DFillZ())
    }

    // Java: float getFillIntensity()
    pub fn getFillIntensity(&self) -> f32 {
      // Java: return this.mProperty.getFloat(369, 0.6F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(369, 0.6_f32)
    }

    // Java: void setFillIntensity(float intensity)
    pub fn setFillIntensity(&mut self, intensity: f32) {
      // Java: this.mProperty.setFloat(369, intensity);
      self.base.base.mProperty.as_mut().unwrap().setFloat(369, intensity);
    }

    // Java: void setConstrainRotation(boolean fRot)
    pub fn setConstrainRotation(&mut self, fRot: bool) {
      // Java: this.mProperty.setBoolean(370, fRot);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(370, fRot);
    }

    // Java: boolean getConstrainRotation()
    pub fn getConstrainRotation(&self) -> bool {
      // Java: return this.mProperty.getBoolean(370, true);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(370, true)
    }

    // Java: void setRotationCenterAuto(boolean fRot)
    pub fn setRotationCenterAuto(&mut self, fRot: bool) {
      // Java: this.mProperty.setBoolean(371, fRot);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(371, fRot);
    }

    // Java: boolean getRotationCenterAuto()
    pub fn getRotationCenterAuto(&self) -> bool {
      // Java: return this.mProperty.getBoolean(371, false);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(371, false)
    }

    // Java: void setParallel(boolean fParallel)
    pub fn setParallel(&mut self, fParallel: bool) {
      // Java: this.mProperty.setBoolean(372, fParallel);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(372, fParallel);
    }

    // Java: boolean getParallel()
    pub fn getParallel(&self) -> bool {
      // Java: return this.mProperty.getBoolean(372, true);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(372, true)
    }

    // Java: void setKeyHarsh(boolean fHarsh)
    pub fn setKeyHarsh(&mut self, fHarsh: bool) {
      // Java: this.mProperty.setBoolean(373, fHarsh);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(373, fHarsh);
    }

    // Java: boolean getKeyHarsh()
    pub fn getKeyHarsh(&self) -> bool {
      // Java: return this.mProperty.getBoolean(373, true);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(373, true)
    }

    // Java: void setFillHarsh(boolean fHarsh)
    pub fn setFillHarsh(&mut self, fHarsh: bool) {
      // Java: this.mProperty.setBoolean(374, fHarsh);
      self.base.base.mProperty.as_mut().unwrap().setBoolean(374, fHarsh);
    }

    // Java: boolean getFillHarsh()
    pub fn getFillHarsh(&self) -> bool {
      // Java: return this.mProperty.getBoolean(374, false);
      self.base.base.mProperty.as_ref().unwrap().base.getBoolean_default(374, false)
    }

}

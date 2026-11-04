#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Object3DDml extends PropBase
pub struct Object3DDml {
    pub base: PropBase,
}

impl Deref for Object3DDml {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Object3DDml extends PropBase
impl Object3DDml {
    // Java: Object3DDml()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
      Object3DDml { base: PropBase::new() }
    }

    // Java: void setBevelBottom(Bevel bevel)
    pub fn setBevelBottom(&mut self, bevel: Element) {
      // Java: this.mProperty.setValue(377, bevel);
      self.base.mProperty.as_mut().unwrap().setValue(377, bevel);
    }

    // Java: Bevel getBevelBottom()
    pub fn getBevelBottom(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(377);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(377);
      // Java: return val != null ? (Bevel)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val
    }

    // Java: void setBevelTop(Bevel bevel)
    pub fn setBevelTop(&mut self, bevel: Element) {
      // Java: this.mProperty.setValue(378, bevel);
      self.base.mProperty.as_mut().unwrap().setValue(378, bevel);
    }

    // Java: Bevel getBevelTop()
    pub fn getBevelTop(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(378);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(378);
      // Java: return val != null ? (Bevel)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val
    }

    // Java: void setPresetMaterial(int prst)
    pub fn setPresetMaterial(&mut self, prst: i32) {
      // Java: this.mProperty.setInt(386, prst);
      self.base.mProperty.as_mut().unwrap().setInt(386, prst);
    }

    // Java: int getPresetMaterial()
    pub fn getPresetMaterial(&self) -> i32 {
      // Java: return this.mProperty.getInt(386, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(386, 0)
    }

    // Java: Integer getPresetMaterialObj()
    pub fn getPresetMaterialObj(&self) -> Option<i32> {
      // Java: Object val = this.mProperty.getValue(386);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(386);
      // Java: return val != null ? (Integer)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setContourClr(int clr)
    pub fn setContourClr(&mut self, clr: i32) {
      // Java: this.mProperty.setInt(382, clr);
      self.base.mProperty.as_mut().unwrap().setInt(382, clr);
    }

    // Java: int getContourClr()
    pub fn getContourClr(&self) -> i32 {
      // Java: return this.mProperty.getInt(382, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(382, 0)
    }

    // Java: Integer getContourClrObj()
    pub fn getContourClrObj(&self) -> Option<i32> {
      // Java: Object val = this.mProperty.getValue(382);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(382);
      // Java: return val != null ? (Integer)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setExtrusionClr(int clr)
    pub fn setExtrusionClr(&mut self, clr: i32) {
      // Java: this.mProperty.setInt(383, clr);
      self.base.mProperty.as_mut().unwrap().setInt(383, clr);
    }

    // Java: int getExtrusionClr()
    pub fn getExtrusionClr(&self) -> i32 {
      // Java: return this.mProperty.getInt(383, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(383, 0)
    }

    // Java: Integer getExtrusionClrObj()
    pub fn getExtrusionClrObj(&self) -> Option<i32> {
      // Java: Object val = this.mProperty.getValue(383);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(383);
      // Java: return val != null ? (Integer)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setContourWidth(float width)
    pub fn setContourWidth(&mut self, width: f32) {
      // Java: this.mProperty.setFloat(384, width);
      self.base.mProperty.as_mut().unwrap().setFloat(384, width);
    }

    // Java: float getContourWidth()
    pub fn getContourWidth(&self) -> f32 {
      // Java: return this.mProperty.getFloat(384, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(384, 0.0_f32)
    }

    // Java: Float getContourWidthObj()
    pub fn getContourWidthObj(&self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(384);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(384);
      // Java: return val != null ? (Float)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setExtrusionHeight(float height)
    pub fn setExtrusionHeight(&mut self, height: f32) {
      // Java: this.mProperty.setFloat(385, height);
      self.base.mProperty.as_mut().unwrap().setFloat(385, height);
    }

    // Java: float getExtrusionHeight()
    pub fn getExtrusionHeight(&self) -> f32 {
      // Java: return this.mProperty.getFloat(385, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(385, 0.0_f32)
    }

    // Java: Float getExtrusionHeightObj()
    pub fn getExtrusionHeightObj(&self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(385);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(385);
      // Java: return val != null ? (Float)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setShapeDepth(float z)
    pub fn setShapeDepth(&mut self, z: f32) {
      // Java: this.mProperty.setFloat(387, z);
      self.base.mProperty.as_mut().unwrap().setFloat(387, z);
    }

    // Java: float getShapeDepth()
    pub fn getShapeDepth(&self) -> f32 {
      // Java: return this.mProperty.getFloat(387, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(387, 0.0_f32)
    }

    // Java: void setLightRig(LightRig lightRig)
    pub fn setLightRig(&mut self, lightRig: Element) {
      // Java: this.mProperty.setValue(388, lightRig);
      self.base.mProperty.as_mut().unwrap().setValue(388, lightRig);
    }

    // Java: LightRig getLightRig()
    pub fn getLightRig(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(388);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(388);
      // Java: return val != null ? (LightRig)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val
    }

    // Java: void setCamera(Camera camera)
    pub fn setCamera(&mut self, camera: Element) {
      // Java: this.mProperty.setValue(389, camera);
      self.base.mProperty.as_mut().unwrap().setValue(389, camera);
    }

    // Java: Camera getCamera()
    pub fn getCamera(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(389);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(389);
      // Java: return val != null ? (Camera)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val
    }

    // Java: void setBackdropPlane(Plane3D plane)
    pub fn setBackdropPlane(&mut self, plane: Element) {
      // Java: this.mProperty.setValue(390, plane);
      self.base.mProperty.as_mut().unwrap().setValue(390, plane);
    }

    // Java: Plane3D getBackdropPlane()
    pub fn getBackdropPlane(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(390);
      let val = self.base.mProperty.as_ref().unwrap().base.getValue(390);
      // Java: return val != null ? (Plane3D)val : null;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      val
    }

}

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;

use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class FillBase extends PropBase
#[derive(Clone)]
pub struct FillBase {
    pub base: PropBase,
    // Java field: float rot
    pub(crate) rot: f32,
}

// Java: public class FillBase extends PropBase
impl FillBase {
    // Java: FillBase()
    pub fn new() -> Self {
      // Java: super();
      FillBase {
          base: PropBase::new(),
          rot: 0.0,
      }
    }

    // Java: FillBase(int fillType)
    pub fn new_with_fill_type(fill_type: i32) -> Self {
      // Java: super();
      let mut fb = FillBase {
          base: PropBase::new(),
          rot: 0.0,
      };
      // Java: this.setFillType(fillType);
      fb.setFillType(fill_type);
      fb
    }

    // Java: FillBase clone()
    pub fn clone_fill(&mut self) -> FillBase {
      // Java: return (FillBase)super.clone();
      FillBase {
          base: self.base.clone(),
          rot: self.rot,
      }
    }

    fn prop(&self) -> &core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_ref().expect("mProperty is None")
    }

    fn prop_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_mut().expect("mProperty is None")
    }

    // Java: void setFillRot(float rot)
    pub fn setFillRot(&mut self, rot: f32) {
      // Java: this.rot = rot;
      self.rot = rot;
    }

    // Java: float getFillRot()
    pub fn getFillRot(&self) -> f32 {
      // Java: return this.rot;
      self.rot
    }

    // Java: void setFillType(int fillType)
    pub fn setFillType(&mut self, fill_type: i32) {
      // Java: this.mProperty.setInt(463, fillType);
      self.prop_mut().setInt(463, fill_type);
    }

    // Java: int getFillType()
    pub fn getFillType(&self) -> i32 {
      // Java: return this.mProperty.getInt(463, 0);
      self.prop().base.getInt_default(463, 0)
    }

    // Java: void setTransparency(float transparency)
    pub fn setTransparency(&mut self, transparency: f32) {
      // Java: this.mProperty.setFloat(468, transparency);
      self.prop_mut().setFloat(468, transparency);
    }

    // Java: float getTransparency()
    pub fn getTransparency(&self) -> f32 {
      // Java: return this.mProperty.getFloat(468, 0.0F);
      self.prop().base.getFloat_default(468, 0.0)
    }

    // Java: Float getTransparencyObj()
    pub fn getTransparencyObj(&self) -> Option<f32> {
      // Java: return (Float)this.mProperty.getValue(468);
      self.prop().base.getValue(468).and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setBackTransparency(float backTransparency)
    pub fn setBackTransparency(&mut self, back_transparency: f32) {
      // Java: this.mProperty.setFloat(469, backTransparency);
      self.prop_mut().setFloat(469, back_transparency);
    }

    // Java: float getBackTransparency()
    pub fn getBackTransparency(&self) -> f32 {
      // Java: return this.mProperty.getFloat(469, 0.0F);
      self.prop().base.getFloat_default(469, 0.0)
    }

    // Java: Float getBackTransparencyObj()
    pub fn getBackTransparencyObj(&self) -> Option<f32> {
      // Java: return (Float)this.mProperty.getValue(469);
      self.prop().base.getValue(469).and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillToRect(RectF rectF)
    pub fn setFillToRect(&mut self, rect_f: RectF) {
      // Java: this.mProperty.setValue(473, rectF);
      self.prop_mut().setValue(473, Some(Arc::new(rect_f)));
    }

    // Java: RectF getFillToRect()
    pub fn getFillToRect(&self) -> Option<RectF> {
      // Java: Object object = this.mProperty.getValue(473);
      let object = self.prop().base.getValue(473);
      // Java: return object == null ? null : (RectF)object;
      object.and_then(|v| v.downcast_ref::<RectF>().cloned())
    }

    // Java: void setFillRect(RectF rectF)
    pub fn setFillRect(&mut self, rect_f: RectF) {
      // Java: this.mProperty.setValue(474, rectF);
      self.prop_mut().setValue(474, Some(Arc::new(rect_f)));
    }

    // Java: RectF getFillRect()
    pub fn getFillRect(&self) -> Option<RectF> {
      // Java: Object object = this.mProperty.getValue(474);
      let object = self.prop().base.getValue(474);
      // Java: return object == null ? null : (RectF)object;
      object.and_then(|v| v.downcast_ref::<RectF>().cloned())
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
      // Java: this.mProperty.setInt(477, color);
      self.prop_mut().setInt(477, color);
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this.mProperty.getInt(477, -1);
      self.prop().base.getInt_default(477, -1)
    }

    // Java: Integer getColorObj()
    pub fn getColorObj(&self) -> Option<i32> {
      // Java: return (Integer)this.mProperty.getValue(477);
      self.prop().base.getValue(477).and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setBackColor(int color)
    pub fn setBackColor(&mut self, color: i32) {
      // Java: this.mProperty.setInt(478, color);
      self.prop_mut().setInt(478, color);
    }

    // Java: Integer getBackColorObj()
    pub fn getBackColorObj(&self) -> Option<i32> {
      // Java: return (Integer)this.mProperty.getValue(478);
      self.prop().base.getValue(478).and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getBackColor()
    pub fn getBackColor(&self) -> i32 {
      // Java: return this.mProperty.getInt(478, -1);
      self.prop().base.getInt_default(478, -1)
    }

    // Java: void setFillHitTest(boolean fillHitTest)
    pub fn setFillHitTest(&mut self, fill_hit_test: bool) {
      // Java: this.mProperty.setBoolean(493, fillHitTest);
      self.prop_mut().setBoolean(493, fill_hit_test);
    }

    // Java: boolean getFillHitTest()
    pub fn getFillHitTest(&self) -> bool {
      // Java: return this.mProperty.getBoolean(493, true);
      self.prop().base.getBoolean_default(493, true)
    }

    // Java: Boolean getFillHitTestObj()
    pub fn getFillHitTestObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(493);
      let object = self.prop().base.getValue(493);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setFillOn(boolean bOn)
    pub fn setFillOn(&mut self, b_on: bool) {
      // Java: this.mProperty.setBoolean(494, bOn);
      self.prop_mut().setBoolean(494, b_on);
    }

    // Java: boolean getFillOn()
    pub fn getFillOn(&self) -> bool {
      // Java: return this.mProperty.getBoolean(494, true);
      self.prop().base.getBoolean_default(494, true)
    }

    // Java: boolean supportUseBgFill()
    pub fn supportUseBgFill(&self) -> bool {
      // Java: return this.getFillType() != 6;
      self.getFillType() != 6
    }

    // Java: void setUseBgFill(boolean userBg)
    pub fn setUseBgFill(&mut self, user_bg: bool) {
      // Java: this.mProperty.setBoolean(499, userBg);
      self.prop_mut().setBoolean(499, user_bg);
    }

    // Java: boolean getUseBgFill()
    pub fn getUseBgFill(&self) -> bool {
      // Java: return this.mProperty.getBoolean(499, false);
      self.prop().base.getBoolean_default(499, false)
    }

    // Java: void setTileRect(RectF rectF)
    pub fn setTileRect(&mut self, rect_f: RectF) {
      // Java: this.mProperty.setValue(475, rectF);
      self.prop_mut().setValue(475, Some(Arc::new(rect_f)));
    }

    // Java: RectF getTileRect()
    pub fn getTileRect(&self) -> Option<RectF> {
      // Java: Object object = this.mProperty.getValue(475);
      let object = self.prop().base.getValue(475);
      // Java: return object == null ? null : (RectF)object;
      object.and_then(|v| v.downcast_ref::<RectF>().cloned())
    }

    // Java: void setBgWidth(float bgWidth)
    pub fn setBgWidth(&mut self, bg_width: f32) {
      // Java: this.mProperty.setFloat(501, bgWidth);
      self.prop_mut().setFloat(501, bg_width);
    }

    // Java: float getBgWidth()
    pub fn getBgWidth(&self) -> f32 {
      // Java: return this.mProperty.getFloat(501, 0.0F);
      self.prop().base.getFloat_default(501, 0.0)
    }

    // Java: void setBgHeight(float height)
    pub fn setBgHeight(&mut self, height: f32) {
      // Java: this.mProperty.setFloat(502, height);
      self.prop_mut().setFloat(502, height);
    }

    // Java: float getBgHeight()
    pub fn getBgHeight(&self) -> f32 {
      // Java: return this.mProperty.getFloat(502, 0.0F);
      self.prop().base.getFloat_default(502, 0.0)
    }

    // Java: void setShapeRect(RectF rectF)
    pub fn setShapeRect(&mut self, rect_f: RectF) {
      // Java: this.mProperty.setValue(500, rectF);
      self.prop_mut().setValue(500, Some(Arc::new(rect_f)));
    }

    // Java: RectF getShapeRect()
    pub fn getShapeRect(&self) -> Option<RectF> {
      // Java: Object object = this.mProperty.getValue(500);
      let object = self.prop().base.getValue(500);
      // Java: return object == null ? null : (RectF)object;
      object.and_then(|v| v.downcast_ref::<RectF>().cloned())
    }

    // Java: void setIsFillNoLine(boolean b)
    pub fn setIsFillNoLine(&mut self, b: bool) {
      // Java: this.mProperty.setBoolean(503, b);
      self.prop_mut().setBoolean(503, b);
    }

    // Java: boolean getIsFillNoLine()
    pub fn getIsFillNoLine(&self) -> bool {
      // Java: return this.mProperty.getBoolean(503, false);
      self.prop().base.getBoolean_default(503, false)
    }

    // Java: static int getMainColor(FillBase fill)
    pub fn getMainColor(fill: Option<&FillBase>) -> i32 {
      // Java: int mainColor = 16777215;
      let mut main_color: i32 = 16777215;
      // Java: if (fill == null)
      if fill.is_none()
      {
         // Java: return mainColor;
         return main_color;
      }
      // Java: else
      {
         // Java: mainColor = fill.getColor();
         let f = fill.unwrap();
         main_color = f.getColor();
         // Java: if (fill instanceof GradFill)
         {
            // Java: GradFill gradFill = (GradFill)fill;
            // Java: GradStopList gsLst = gradFill.getGradStopList();
            // Java: if (gsLst != null)
            {
               // Java: int listSize = gsLst.size();
               // Java: if (listSize > 0)
               {
                  // Java: mainColor = gsLst.at(0).getColor();
               }
            }
         }
         // Java: return mainColor;
         return main_color;
      }
    }

    // Java: void setRotateWithShape(boolean rotWithShape)
    pub fn setRotateWithShape(&mut self, rot_with_shape: bool) {
      // Java: this.mProperty.setBoolean(467, rotWithShape);
      self.prop_mut().setBoolean(467, rot_with_shape);
    }

    // Java: boolean getRotateWithShape()
    pub fn getRotateWithShape(&self) -> bool {
      // Java: return this.mProperty.getBoolean(467, false);
      self.prop().base.getBoolean_default(467, false)
    }

}

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use crate::cn::wps::moffice::drawing::graphics::FillBase::FillBase;

use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class LineProperty extends PropBase implements Cloneable
pub struct LineProperty {
    pub base: PropBase,
}

// Java: public class LineProperty extends PropBase implements Cloneable
impl LineProperty {
    // Java: LineProperty()
    pub fn new() -> Self {
      // Java: super();
      LineProperty { base: PropBase::new() }
    }

    // Java: LineProperty(int color, float width)
    pub fn new_with_color_width(color: i32, width: f32) -> Self {
      // Java: super();
      let mut lp = LineProperty { base: PropBase::new() };
      // Java: this.setColor(color);
      lp.setColor(color);
      // Java: this.setWidth(width);
      lp.setWidth(width);
      lp
    }

    fn prop(&self) -> &core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_ref().expect("mProperty is None")
    }

    fn prop_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_mut().expect("mProperty is None")
    }

    // Java: LineProperty clone()
    pub fn clone_line(&mut self) -> LineProperty {
      // Java: LineProperty lineProperty = (LineProperty)super.clone();
      let mut line_property = LineProperty { base: self.base.clone() };
      // Java: FillBase fill = this.getFill();
      let fill = self.getFill();
      // Java: if (fill != null)
      if fill.is_some()
      {
         // Java: FillBase newFillBase = fill.clone();
         // Java: lineProperty.setFill(newFillBase);
      }
      // Java: Arrow arrow = this.getStartArrow();
      let arrow = self.getStartArrow();
      // Java: if (arrow != null)
      if arrow.is_some()
      {
         // Java: Arrow newArrow = arrow.clone();
         // Java: lineProperty.setStartArrow(newArrow);
         line_property.setStartArrow_element(arrow);
      }
      // Java: arrow = this.getEndArrow();
      let arrow = self.getEndArrow();
      // Java: if (arrow != null)
      if arrow.is_some()
      {
         // Java: Arrow newArrow = arrow.clone();
         // Java: lineProperty.setEndArrow(newArrow);
         line_property.setEndArrow_element(arrow);
      }
      // Java: return lineProperty;
      line_property
    }

    // Java: Integer getCategoryObj()
    pub fn getCategoryObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(579);
      let object = self.prop().base.getValue(579);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getCategory()
    pub fn getCategory(&self) -> i32 {
      // Java: return this.mProperty.getInt(579, 0);
      self.prop().base.getInt_default(579, 0)
    }

    // Java: void setCategory(int val)
    pub fn setCategory(&mut self, val: i32) {
      // Java: this.mProperty.setInt(579, val);
      self.prop_mut().setInt(579, val);
    }

    // Java: Boolean getfLineObj()
    pub fn getfLineObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(558);
      let object = self.prop().base.getValue(558);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: boolean getfLine()
    pub fn getfLine(&self) -> bool {
      // Java: boolean defaultfLine = 0 == this.getCategory();
      let default_f_line = 0 == self.getCategory();
      // Java: return this.mProperty.getBoolean(558, defaultfLine);
      self.prop().base.getBoolean_default(558, default_f_line)
    }

    // Java: void setfLine(boolean val)
    pub fn setfLine(&mut self, val: bool) {
      // Java: this.mProperty.setBoolean(558, val);
      self.prop_mut().setBoolean(558, val);
    }

    // Java: Integer getColorObj()
    pub fn getColorObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(559);
      let object = self.prop().base.getValue(559);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this.mProperty.getInt(559, 0);
      self.prop().base.getInt_default(559, 0)
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
      // Java: this.mProperty.setInt(559, color);
      self.prop_mut().setInt(559, color);
    }

    // Java: Integer getBackColorObj()
    pub fn getBackColorObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(560);
      let object = self.prop().base.getValue(560);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getBackColor()
    pub fn getBackColor(&self) -> i32 {
      // Java: return this.mProperty.getInt(560, 16777215);
      self.prop().base.getInt_default(560, 16777215)
    }

    // Java: void setBackColor(int color)
    pub fn setBackColor(&mut self, color: i32) {
      // Java: this.mProperty.setInt(560, color);
      self.prop_mut().setInt(560, color);
    }

    // Java: Float getTransparencyObj()
    pub fn getTransparencyObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(561);
      let object = self.prop().base.getValue(561);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: float getTransparency()
    pub fn getTransparency(&self) -> f32 {
      // Java: return this.mProperty.getFloat(561, 0.0F);
      self.prop().base.getFloat_default(561, 0.0)
    }

    // Java: void setTransparency(float v)
    pub fn setTransparency(&mut self, v: f32) {
      // Java: this.mProperty.setFloat(561, v);
      self.prop_mut().setFloat(561, v);
    }

    // Java: Float getWidthObj()
    pub fn getWidthObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(562);
      let object = self.prop().base.getValue(562);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
      // Java: return this.mProperty.getFloat(562, 0.75F);
      self.prop().base.getFloat_default(562, 0.75)
    }

    // Java: void setWidth(float width)
    pub fn setWidth(&mut self, width: f32) {
      // Java: this.mProperty.setFloat(562, width);
      self.prop_mut().setFloat(562, width);
    }

    // Java: Integer getTypeObj()
    pub fn getTypeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(563);
      let object = self.prop().base.getValue(563);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.mProperty.getInt(563, 0);
      self.prop().base.getInt_default(563, 0)
    }

    // Java: void setType(int type)
    pub fn setType(&mut self, type_val: i32) {
      // Java: this.mProperty.setInt(563, type);
      self.prop_mut().setInt(563, type_val);
    }

    // Java: Integer getLineStyleObj()
    pub fn getLineStyleObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(564);
      let object = self.prop().base.getValue(564);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getLineStyle()
    pub fn getLineStyle(&self) -> i32 {
      // Java: return this.mProperty.getInt(564, 0);
      self.prop().base.getInt_default(564, 0)
    }

    // Java: void setLineStyle(int style)
    pub fn setLineStyle(&mut self, style: i32) {
      // Java: this.mProperty.setInt(564, style);
      self.prop_mut().setInt(564, style);
    }

    // Java: Integer getDashingObj()
    pub fn getDashingObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(565);
      let object = self.prop().base.getValue(565);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getDashing()
    pub fn getDashing(&self) -> i32 {
      // Java: return this.mProperty.getInt(565, 0);
      self.prop().base.getInt_default(565, 0)
    }

    // Java: void setDashing(int dashing)
    pub fn setDashing(&mut self, dashing: i32) {
      // Java: this.mProperty.setInt(565, dashing);
      self.prop_mut().setInt(565, dashing);
    }

    // Java: float[] getDashStyle()
    pub fn getDashStyle(&self) -> Option<Vec<f32>> {
      // Java: return (float[])this.mProperty.getValue(566);
      self.prop().base.getValue(566).and_then(|v| v.downcast_ref::<Vec<f32>>().cloned())
    }

    // Java: void setDashStyle(float[] dash)
    pub fn setDashStyle(&mut self, dash: Vec<f32>) {
      // Java: this.mProperty.setValue(566, dash);
      self.prop_mut().setValue(566, Some(Arc::new(dash)));
    }

    // Java: Integer getJoinStyleObj()
    pub fn getJoinStyleObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(567);
      let object = self.prop().base.getValue(567);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getJoinStyle()
    pub fn getJoinStyle(&self) -> i32 {
      // Java: return this.mProperty.getInt(567, 2);
      self.prop().base.getInt_default(567, 2)
    }

    // Java: void setJoinStyle(int join)
    pub fn setJoinStyle(&mut self, join: i32) {
      // Java: this.mProperty.setInt(567, join);
      self.prop_mut().setInt(567, join);
    }

    // Java: Integer getEndCapStyleObj()
    pub fn getEndCapStyleObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(568);
      let object = self.prop().base.getValue(568);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: int getEndCapStyle()
    pub fn getEndCapStyle(&self) -> i32 {
      // Java: return this.mProperty.getInt(568, 2);
      self.prop().base.getInt_default(568, 2)
    }

    // Java: void setEndCapStyle(int cap)
    pub fn setEndCapStyle(&mut self, cap: i32) {
      // Java: this.mProperty.setInt(568, cap);
      self.prop_mut().setInt(568, cap);
    }

    // Java: FillBase getFill()
    pub fn getFill(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(571);
      // Java: return object == null ? null : (FillBase)object;
      self.prop().base.getValue(571)
    }

    // Java: void setFill(FillBase fill)
    pub fn setFill(&mut self, fill: Element) {
      // Java: this.mProperty.setValue(571, fill);
      self.prop_mut().setValue(571, fill);
    }

    // Java: Arrow getStartArrow()
    pub fn getStartArrow(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(569);
      // Java: return object == null ? null : (Arrow)object;
      self.prop().base.getValue(569)
    }

    // Java: void setStartArrow(Arrow arrow)
    pub fn setStartArrow_element(&mut self, arrow: Element) {
      // Java: this.mProperty.setValue(569, arrow);
      self.prop_mut().setValue(569, arrow);
    }

    // Java: void setStartArrow(int head, int width, int length)
    pub fn setStartArrow_dims(&mut self, _head: i32, _width: i32, _length: i32) {
      // Java: Arrow arrow = new Arrow(head, width, length);
      // Java: this.mProperty.setValue(569, arrow);
    }

    // Java: Arrow getEndArrow()
    pub fn getEndArrow(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(570);
      // Java: return object == null ? null : (Arrow)object;
      self.prop().base.getValue(570)
    }

    // Java: void setEndArrow(Arrow arrow)
    pub fn setEndArrow_element(&mut self, arrow: Element) {
      // Java: this.mProperty.setValue(570, arrow);
      self.prop_mut().setValue(570, arrow);
    }

    // Java: void setEndArrow(int head, int width, int length)
    pub fn setEndArrow_dims(&mut self, _head: i32, _width: i32, _length: i32) {
      // Java: Arrow arrow = new Arrow(head, width, length);
      // Java: this.mProperty.setValue(570, arrow);
    }

    // Java: void setInsetPenOK(boolean bOK)
    pub fn setInsetPenOK(&mut self, b_ok: bool) {
      // Java: this.mProperty.setBoolean(573, bOK);
      self.prop_mut().setBoolean(573, b_ok);
    }

    // Java: boolean getInsetPenOK()
    pub fn getInsetPenOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(573, true);
      self.prop().base.getBoolean_default(573, true)
    }

    // Java: Boolean getInsetPenOKObj()
    pub fn getInsetPenOKObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(573);
      let object = self.prop().base.getValue(573);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setInsetPen(boolean val)
    pub fn setInsetPen(&mut self, val: bool) {
      // Java: this.mProperty.setBoolean(574, val);
      self.prop_mut().setBoolean(574, val);
    }

    // Java: boolean getInsetPen()
    pub fn getInsetPen(&self) -> bool {
      // Java: return this.mProperty.getBoolean(574, false);
      self.prop().base.getBoolean_default(574, false)
    }

    // Java: Boolean getInsetPenObj()
    pub fn getInsetPenObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(574);
      let object = self.prop().base.getValue(574);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setArrowHeadsOk(boolean val)
    pub fn setArrowHeadsOk(&mut self, val: bool) {
      // Java: this.mProperty.setBoolean(575, val);
      self.prop_mut().setBoolean(575, val);
    }

    // Java: boolean getArrowHeadsOk()
    pub fn getArrowHeadsOk(&self) -> bool {
      // Java: return this.mProperty.getBoolean(575, false);
      self.prop().base.getBoolean_default(575, false)
    }

    // Java: Boolean getArrowHeadsOkObj()
    pub fn getArrowHeadsOkObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(575);
      let object = self.prop().base.getValue(575);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setfNoLineDrawDash(boolean val)
    pub fn setfNoLineDrawDash(&mut self, val: bool) {
      // Java: this.mProperty.setBoolean(576, val);
      self.prop_mut().setBoolean(576, val);
    }

    // Java: boolean getfNoLineDrawDash()
    pub fn getfNoLineDrawDash(&self) -> bool {
      // Java: return this.mProperty.getBoolean(576, false);
      self.prop().base.getBoolean_default(576, false)
    }

    // Java: Boolean getfNoLineDrawDashObj()
    pub fn getfNoLineDrawDashObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(576);
      let object = self.prop().base.getValue(576);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setfImageAlignShape(boolean val)
    pub fn setfImageAlignShape(&mut self, val: bool) {
      // Java: this.mProperty.setBoolean(577, val);
      self.prop_mut().setBoolean(577, val);
    }

    // Java: boolean getfImageAlignShape()
    pub fn getfImageAlignShape(&self) -> bool {
      // Java: return this.mProperty.getBoolean(577, true);
      self.prop().base.getBoolean_default(577, true)
    }

    // Java: Boolean getfImageAlignShapeObj()
    pub fn getfImageAlignShapeObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(577);
      let object = self.prop().base.getValue(577);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setLineFillBlipFlags(int val)
    pub fn setLineFillBlipFlags(&mut self, val: i32) {
      // Java: this.mProperty.setInt(578, val);
      self.prop_mut().setInt(578, val);
    }

    // Java: int getLineFillBlipFlags()
    pub fn getLineFillBlipFlags(&self) -> i32 {
      // Java: return this.mProperty.getInt(578, 0);
      self.prop().base.getInt_default(578, 0)
    }

    // Java: void setLineMiterLimit(float val)
    pub fn setLineMiterLimit(&mut self, val: f32) {
      // Java: this.mProperty.setValue(572, val);
      self.prop_mut().setValue(572, Some(Arc::new(val)));
    }

    // Java: float getLineMiterLimit()
    pub fn getLineMiterLimit(&self) -> f32 {
      // Java: return this.mProperty.getFloat(572, 0.5F);
      self.prop().base.getFloat_default(572, 0.5)
    }

    // Java: Float getLineMiterLimitObj()
    pub fn getLineMiterLimitObj(&self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(572);
      let object = self.prop().base.getValue(572);
      // Java: return object == null ? null : (Float)object;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

}

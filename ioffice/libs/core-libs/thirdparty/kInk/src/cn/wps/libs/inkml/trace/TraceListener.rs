// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import cn.wps.libs.inkml.InkMLConverter
use crate::cn::wps::libs::inkml::InkMLConverter::InkMLConverter;
// Java: import cn.wps.libs.inkml.InkMLManagerListener
use crate::cn::wps::libs::inkml::InkMLManagerListener::InkMLManagerListener;
// Java: import cn.wps.libs.inkml.InkMLTouchListener
use crate::cn::wps::libs::inkml::InkMLTouchListener::InkMLTouchListener;
// Java: import cn.wps.libs.inkml.InkMLViewListener
use crate::cn::wps::libs::inkml::InkMLViewListener::InkMLViewListener;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;

// Java: public class TraceListener implements InkMLTouchListener, InkMLManagerListener, InkMLViewListener
pub struct TraceListener {
    // Java field: boolean bManagerInk
    bManagerInk: bool,
    // Java field: InkMLTouchListener touchListener
    pub(crate) touchListener: Option<Box<dyn InkMLTouchListener>>,
    // Java field: InkMLManagerListener managerListener
    pub(crate) managerListener: Option<Box<dyn InkMLManagerListener>>,
    // Java field: InkMLViewListener viewListener
    viewListener: Option<Box<dyn InkMLViewListener>>,
    // Java field: InkMLConverter converter
    pub(crate) converter: Option<Box<dyn InkMLConverter>>,
}

// Java: public class TraceListener implements InkMLTouchListener, InkMLManagerListener, InkMLViewListener
impl TraceListener {
    // Java: TraceListener(boolean bManagerInk)
    pub fn TraceListener(b_manager_ink: bool) -> Self {
      // Java: super();
      // Java: this.bManagerInk = bManagerInk;
      Self {
          bManagerInk: b_manager_ink,
          touchListener: None,
          managerListener: None,
          viewListener: None,
          converter: None,
      }
    }

    // Java: void setTouchListener(InkMLTouchListener touchListener)
    pub fn setTouchListener(&mut self, touch_listener: Option<Box<dyn InkMLTouchListener>>) {
      // Java: this.touchListener = touchListener;
      self.touchListener = touch_listener;
    }

    // Java: void setManagerListener(InkMLManagerListener managerListener)
    pub fn setManagerListener(&mut self, manager_listener: Option<Box<dyn InkMLManagerListener>>) {
      // Java: this.managerListener = managerListener;
      self.managerListener = manager_listener;
    }

    // Java: void setViewListener(InkMLViewListener viewListener)
    pub fn setViewListener(&mut self, view_listener: Option<Box<dyn InkMLViewListener>>) {
      // Java: this.viewListener = viewListener;
      self.viewListener = view_listener;
    }

    // Java: void setConverter(InkMLConverter converter)
    pub fn setConverter(&mut self, converter: Option<Box<dyn InkMLConverter>>) {
      // Java: this.converter = converter;
      self.converter = converter;
    }

    // Java: void onTouch(int action, float x, float y, float f, InkCreator inkCreator)
    pub fn onTouch(&self, action: i32, x: f32, y: f32, f: f32, ink_creator: &InkCreator) {
      // Java: if (this.touchListener != null)
      if let Some(ref listener) = self.touchListener
      {
         // Java: this.touchListener.onTouch(action, x, y, f, inkCreator);
         listener.onTouch(action, x, y, f, ink_creator);
      }
    }

    // Java: void invalidate()
    pub fn invalidate(&self) {
      // Java: if (this.viewListener != null)
      if let Some(ref listener) = self.viewListener
      {
         // Java: this.viewListener.invalidate();
         listener.invalidate();
      }
    }

    // Java: void postInvalidate()
    pub fn postInvalidate(&self) {
      // Java: if (this.viewListener != null)
      if let Some(ref listener) = self.viewListener
      {
         // Java: this.viewListener.postInvalidate();
         listener.postInvalidate();
      }
    }

    // Java: void onInkAdd(InkCreator inkCreator)
    pub fn onInkAdd(&self, ink_creator: &InkCreator) {
      // Java: if (this.managerListener != null)
      if let Some(ref listener) = self.managerListener
      {
         // Java: this.managerListener.onInkAdd(inkCreator);
         listener.onInkAdd(ink_creator);
      }
    }

    // Java: void onInkDelete(InkCreator inkCreator)
    pub fn onInkDelete(&self, ink_creator: &InkCreator) {
      // Java: if (this.managerListener != null)
      if let Some(ref listener) = self.managerListener
      {
         // Java: this.managerListener.onInkDelete(inkCreator);
         listener.onInkDelete(ink_creator);
      }
    }

    // Java: PointF adjustEventXY(float x, float y, PointF newPoint)
    pub fn adjustEventXY(&self, x: f32, y: f32, new_point: &mut PointF) {
      // Java: newPoint.set(x, y);
      new_point.x = x;
      new_point.y = y;
      // Java: if (this.converter != null)
      if let Some(ref converter) = self.converter
      {
         // Java: this.converter.adjustEventXY(x, y, newPoint);
         converter.adjustEventXY(x, y, new_point);
      }
      // Java: return newPoint;
    }

    // Java: float adjustStrokeWidth(float width)
    pub fn adjustStrokeWidth(&self, width: f32) -> f32 {
      // Java: return this.converter != null ? this.converter.adjustStrokeWidth(width) : width;
      if let Some(ref converter) = self.converter {
          converter.adjustStrokeWidth(width)
      } else {
          width
      }
    }

    // Java: PointF getInkScaleXY(PointF scale)
    pub fn getInkScaleXY(&self, scale: &mut PointF) {
      // Java: scale.set(1.0F, 1.0F);
      scale.x = 1.0;
      scale.y = 1.0;
      // Java: if (this.converter != null)
      if let Some(ref converter) = self.converter
      {
         // Java: this.converter.getInkScaleXY(scale);
         converter.getInkScaleXY(scale);
      }
      // Java: return scale;
    }

    // Java: PointF adjustEraserXY(float x, float y, PointF newPoint)
    pub fn adjustEraserXY(&self, x: f32, y: f32, new_point: &mut PointF) {
      // Java: newPoint.set(x, y);
      new_point.x = x;
      new_point.y = y;
      // Java: if (this.converter != null)
      if let Some(ref converter) = self.converter
      {
         // Java: this.converter.adjustEraserXY(x, y, newPoint);
         converter.adjustEraserXY(x, y, new_point);
      }
      // Java: return newPoint;
    }

    // Java: float getEraserScale()
    pub fn getEraserScale(&self) -> f32 {
      // Java: return this.converter != null ? this.converter.getEraserScale() : 1.0F;
      if let Some(ref converter) = self.converter {
          converter.getEraserScale()
      } else {
          1.0
      }
    }

    // Java: PointF getEstimateScaleXY(PointF scale)
    pub fn getEstimateScaleXY(&self, scale: &mut PointF) {
      // Java: scale.set(1.0F, 1.0F);
      scale.x = 1.0;
      scale.y = 1.0;
      // Java: if (this.converter != null)
      if let Some(ref converter) = self.converter
      {
         // Java: this.converter.getEstimateScaleXY(scale);
         converter.getEstimateScaleXY(scale);
      }
      // Java: return scale;
    }

    // Java: PointF getEstimateOffsetXY(PointF offset)
    pub fn getEstimateOffsetXY(&self, offset: &mut PointF) {
      // Java: offset.set(0.0F, 0.0F);
      offset.x = 0.0;
      offset.y = 0.0;
      // Java: if (this.converter != null)
      if let Some(ref converter) = self.converter
      {
         // Java: this.converter.getEstimateOffsetXY(offset);
         converter.getEstimateOffsetXY(offset);
      }
      // Java: return offset;
    }

}

impl InkMLTouchListener for TraceListener {
    fn onTouch(&self, var1: i32, var2: f32, var3: f32, var4: f32, var5: &InkCreator) {
        self.onTouch(var1, var2, var3, var4, var5);
    }
}

impl InkMLManagerListener for TraceListener {
    fn onInkAdd(&self, var1: &InkCreator) {
        self.onInkAdd(var1);
    }
    fn onInkDelete(&self, var1: &InkCreator) {
        self.onInkDelete(var1);
    }
}

impl InkMLViewListener for TraceListener {
    fn invalidate(&self) {
        self.invalidate();
    }
    fn postInvalidate(&self) {
        self.postInvalidate();
    }
}

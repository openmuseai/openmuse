// Java: import android.view.MotionEvent
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import cn.wps.libs.inkml.trace.TraceGroup
use crate::cn::wps::libs::inkml::trace::TraceGroup::TraceGroup;
// Java: import cn.wps.libs.inkml.trace.TraceSetting
use crate::cn::wps::libs::inkml::trace::TraceSetting::TraceSetting;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;
// Java: import com.hp.hpl.inkml.impl.InkEstimate
// Java: import com.parser.EventParserImpl
// Java: import com.parser.IEventParser
use crate::com::parser::IEventParser::IEventParser;
// Java: import com.parser.IInkStroker
use crate::com::parser::IInkStroker::IInkStroker;
// Java: import com.parser.PretendEventParser
// Java: import com.parser.writingbrush.WritingEventParser

use crate::cn::wps::libs::inkml::event::IEventDataSupplier::IEventDataSupplier;

pub(crate) const TOUCH_TOLERANCE: f32 = 4.0;

pub struct MotionEvent;

// Java: public class MotionEventParser implements IInkStroker
pub struct MotionEventParser {
    // Java field: TraceGroup traceGroup
    pub(crate) traceGroup: TraceGroup,
    // Java field: TraceSetting setting
    setting: TraceSetting,
    // Java field: WritingEventParser writingEventParser
    writingEventParser: i32,
    // Java field: EventParserImpl normalEventParser
    normalEventParser: i32,
    // Java field: IEventParser eventParser
    eventParser: i32,
    // Java field: InkCreator inkCreator
    inkCreator: Option<InkCreator>,
    // Java field: boolean bMoving
    pub(crate) bMoving: bool,
    // Java field: boolean bCancelEvent
    bCancelEvent: bool,
    // Java field: boolean bUseEstimate
    bUseEstimate: bool,
    // Java field: float TOUCH_TOLERANCE
    _touch_tolerance: f32,
    // Java field: float touchDistance
    touchDistance: f32,
    // Java field: float lastX
    lastX: f32,
    // Java field: float lastY
    lastY: f32,
}

// Java: public class MotionEventParser implements IInkStroker
impl MotionEventParser {
    // Java: MotionEventParser(TraceGroup traceGroup, TraceSetting setting, float dip, boolean bUseEstimate)
    pub fn MotionEventParser(trace_group: TraceGroup, setting: TraceSetting, _dip: f32, b_use_estimate: bool) -> Self {
      // Java: super();
      // Java: this.traceGroup = traceGroup;
      // Java: this.setting = setting;
      // Java: this.bUseEstimate = bUseEstimate;
      // Java: this.writingEventParser = new WritingEventParser(this, dip);
      // Java: this.normalEventParser = new EventParserImpl(this);
      // Java: this.eventParser = (IEventParser)(setting.isWritingPen() ? this.writingEventParser : this.normalEventParser);
      Self {
          traceGroup: trace_group,
          setting,
          writingEventParser: 0,
          normalEventParser: 0,
          eventParser: 0,
          inkCreator: None,
          bMoving: false,
          bCancelEvent: false,
          bUseEstimate: b_use_estimate,
          _touch_tolerance: TOUCH_TOLERANCE,
          touchDistance: TOUCH_TOLERANCE,
          lastX: 0.0,
          lastY: 0.0,
      }
    }

    // Java: void setPretendParser(IEventDataSupplier supplier)
    pub fn setPretendParser(&mut self, _supplier: Option<&dyn IEventDataSupplier>) {
      // Java: if (supplier != null)
      {
         // Java: if (!(this.eventParser instanceof PretendEventParser))
         {
            // Java: this.eventParser = new PretendEventParser(this, supplier);
         }
      }
      // Java: else
      {
         // Java: this.eventParser = (IEventParser)(this.setting.isWritingPen() ? this.writingEventParser : this.normalEventParser);
      }
    }

    // Java: boolean processEvent(MotionEvent event)
    pub fn processEvent(&mut self, _event: &MotionEvent) -> bool {
      // Java: if (event.getAction() == 0)
      {
         // Java: this.bCancelEvent = false;
         self.bCancelEvent = false;
      }
      // Java: if (event.getPointerCount() <= 1 && !this.bCancelEvent)
      if !self.bCancelEvent
      {
         // Java: this.eventParser.parseEvent(event);
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: boolean touchCancel()
    pub fn touchCancel(&mut self) -> bool {
      // Java: this.bCancelEvent = true;
      self.bCancelEvent = true;
      // Java: if (this.bMoving)
      if self.bMoving
      {
         // Java: this.onFinish();
         self.onFinish();
         // Java: this.eventParser.touchCancel();
         // Java: this.traceGroup.invalidate();
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: void onStart(float x, float y, float f)
    pub fn onStart(&mut self, x: f32, y: f32, f: f32) {
      // Java: this.onStartOrMove(x, y, f, 0);
      self.onStartOrMove(x, y, f, 0);
    }

    // Java: void onMove(float x, float y, float f)
    pub fn onMove(&mut self, x: f32, y: f32, f: f32) {
      // Java: this.onStartOrMove(x, y, f, 2);
      self.onStartOrMove(x, y, f, 2);
    }

    // Java: void onStartOrMove(float x, float y, float f, int action)
    pub fn onStartOrMove(&mut self, x: f32, y: f32, _f: f32, _action: i32) {
      // Java: this.bMoving = true;
      self.bMoving = true;
      // Java: if (this.setting.isEraser())
      if self.setting.isEraser()
      {
         // Java: this.traceGroup.setEraserPoint(x, y);
         self.traceGroup.setEraserPoint(x, y);
         // Java: this.traceGroup.onTouch(action, x, y, f, (InkCreator)null);
      }
      // Java: else
      else
      {
         // Java: if (this.inkCreator == null)
         if self.inkCreator.is_none()
         {
            // Java: this.inkCreator = this.traceGroup.createCurrentInkCreator();
            // Java: this.inkCreator.begin(this.setting.getColor(), this.setting.getStrokeWidth(), this.setting.isHighlight(), this.setting.getShape(), this.setting.isWritingPen());
            // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
            // Java: if (inkEstimate != null && this.bUseEstimate)
            {
               // Java: PointF pointXY = this.traceGroup.getEstimateScaleXY(PointF.obtain());
               // Java: inkEstimate.setScale(pointXY.x, pointXY.y);
               // Java: pointXY = this.traceGroup.getEstimateOffsetXY(pointXY);
               // Java: inkEstimate.setOffset(pointXY.x, pointXY.y);
               // Java: pointXY.recycle();
               // Java: inkEstimate.setUse();
            }
            // Java: this.traceGroup.onTouch(0, x, y, f, this.inkCreator);
         }
         // Java: else
         else
         {
            // Java: if (Math.abs(this.lastX - x) < this.touchDistance && Math.abs(this.lastY - y) < this.touchDistance)
            if (self.lastX - x).abs() < self.touchDistance && (self.lastY - y).abs() < self.touchDistance
            {
               // Java: return;
               return;
            }
            // Java: this.traceGroup.onTouch(2, x, y, f, this.inkCreator);
         }
         // Java: PointF pointF = this.traceGroup.adjustEventXY(x, y, PointF.obtain());
         // Java: this.inkCreator.move(pointF.x, pointF.y, f);
         // Java: pointF.recycle();
         // Java: this.lastX = x;
         self.lastX = x;
         // Java: this.lastY = y;
         self.lastY = y;
      }
      // Java: this.traceGroup.invalidate();
    }

    // Java: void onFinish()
    pub fn onFinish(&mut self) {
      // Java: if (this.setting.isEraser())
      if self.setting.isEraser()
      {
         // Java: this.traceGroup.resetEraserPoint();
         self.traceGroup.resetEraserPoint();
         // Java: this.traceGroup.invalidate();
      }
      // Java: else if (this.inkCreator != null)
      else if self.inkCreator.is_some()
      {
         // Java: this.inkCreator.end();
         if let Some(ref mut creator) = self.inkCreator {
             creator.end();
         }
         // Java: this.traceGroup.resetCurrentInkCreator();
         self.traceGroup.resetCurrentInkCreator();
         // Java: this.traceGroup.invalidate();
         // Java: this.traceGroup.onTouch(1, 0.0F, 0.0F, 0.0F, this.inkCreator);
         // Java: this.inkCreator = null;
         self.inkCreator = None;
      }
      // Java: this.bMoving = false;
      self.bMoving = false;
    }

    // Java: float getStrokeWidth()
    pub fn getStrokeWidth(&self) -> f32 {
      // Java: return this.traceGroup.adjustStrokeWidth(this.setting.getStrokeWidth());
      self.setting.getStrokeWidth()
    }

    // Java: void onTipChange()
    pub fn onTipChange(&mut self) {
      // Java: if (this.eventParser == null)
      {
         // Java: this.eventParser = (IEventParser)(this.setting.isWritingPen() ? this.writingEventParser : this.normalEventParser);
      }
    }

    // Java: boolean isMoving()
    pub fn isMoving(&self) -> bool {
      // Java: return this.bMoving;
      self.bMoving
    }

    // Java: void setMoving(boolean moving)
    pub fn setMoving(&mut self, moving: bool) {
      // Java: this.bMoving = moving;
      self.bMoving = moving;
    }

    // Java: void setFilterPointDistance(float distance)
    pub fn setFilterPointDistance(&mut self, distance: f32) {
      // Java: this.touchDistance = distance;
      self.touchDistance = distance;
    }

}

impl IInkStroker for MotionEventParser {
    fn onStart(&mut self, x: f32, y: f32, f: f32) { self.onStart(x, y, f); }
    fn onMove(&mut self, x: f32, y: f32, f: f32) { self.onMove(x, y, f); }
    fn onFinish(&mut self) { self.onFinish(); }
    fn getStrokeWidth(&self) -> f32 { self.getStrokeWidth() }
}

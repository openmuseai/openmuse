// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Paint.Style
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import cn.wps.libs.inkml.trace.TraceGroup
use crate::cn::wps::libs::inkml::trace::TraceGroup::TraceGroup;
// Java: import cn.wps.libs.inkml.trace.TraceSetting
use crate::cn::wps::libs::inkml::trace::TraceSetting::TraceSetting;

pub(crate) const OUTLINE_COLOR: i32 = -6842473;
pub(crate) const INNER_COLOR: i32 = 438181406;

// Java: public class EraserPaint
pub struct EraserPaint {
    // Java field: TraceGroup traceGroup
    traceGroup: TraceGroup,
    // Java field: TraceSetting setting
    pub(crate) setting: TraceSetting,
    // Java field: Paint mOutlinePaint
    mOutlinePaint: i32,
    // Java field: Paint mInnerPaint
    mInnerPaint: i32,
    // Java field: int OUTLINE_COLOR
    _outline_color: i32,
    // Java field: int INNER_COLOR
    _inner_color: i32,
    // Java field: float OUTLINE_STROKE
    OUTLINE_STROKE: f32,
    // Java field: float CIRCLE_RADIUS
    CIRCLE_RADIUS: f32,
}

// Java: public class EraserPaint
impl EraserPaint {
    // Java: EraserPaint(TraceGroup traceGroup, TraceSetting setting)
    pub fn EraserPaint(trace_group: TraceGroup, setting: TraceSetting) -> Self {
      // Java: super();
      // Java: this.traceGroup = traceGroup;
      // Java: this.setting = setting;
      // Java: this.OUTLINE_STROKE = 0.4F * Platform.getDisplayMetrics().density;
      // Java: this.CIRCLE_RADIUS = 9.6F * Platform.getDisplayMetrics().density;
      // Java: this.mOutlinePaint = new Paint(1);
      // Java: this.mOutlinePaint.setColor(-6842473);
      // Java: this.mOutlinePaint.setStrokeWidth(this.OUTLINE_STROKE);
      // Java: this.mOutlinePaint.setStyle(Style.STROKE);
      // Java: this.mInnerPaint = new Paint(1);
      // Java: this.mInnerPaint.setColor(438181406);
      // Java: this.mInnerPaint.setStyle(Style.FILL);
      Self {
          traceGroup: trace_group,
          setting,
          mOutlinePaint: 0,
          mInnerPaint: 0,
          _outline_color: OUTLINE_COLOR,
          _inner_color: INNER_COLOR,
          OUTLINE_STROKE: 0.4 * 2.0,
          CIRCLE_RADIUS: 9.6 * 2.0,
      }
    }

    // Java: boolean needDraw()
    pub fn needDraw(&self) -> bool {
      // Java: return this.setting.isEraser() && this.traceGroup.getEraserPoint() != null;
      self.setting.isEraser() && self.traceGroup.getEraserPoint().is_some()
    }

    // Java: void draw(Canvas canvas)
    pub fn draw(&self) {
      // Java: if (this.setting.isEraser())
      {
         // Java: PointF pointF = this.traceGroup.getEraserPoint();
         // Java: if (pointF != null)
         {
            // Java: float scale = this.traceGroup.getEraserScale();
            // Java: PointF newPointF = this.traceGroup.adjustEraserXY(pointF.x, pointF.y, PointF.obtain());
            // Java: canvas.drawCircle(newPointF.x, newPointF.y, this.CIRCLE_RADIUS * scale, this.mOutlinePaint);
            // Java: canvas.drawCircle(newPointF.x, newPointF.y, this.CIRCLE_RADIUS * scale - this.OUTLINE_STROKE, this.mInnerPaint);
            // Java: newPointF.recycle();
         }
      }
    }

}

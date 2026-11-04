// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.RectF
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import cn.wps.libs.inkml.InkMLConverter
use crate::cn::wps::libs::inkml::InkMLConverter::InkMLConverter;
// Java: import cn.wps.libs.inkml.trace.TraceGroup
use crate::cn::wps::libs::inkml::trace::TraceGroup::TraceGroup;
// Java: import cn.wps.libs.inkml.trace.TraceSetting
use crate::cn::wps::libs::inkml::trace::TraceSetting::TraceSetting;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;
// Java: import com.hp.hpl.inkml.impl.InkPainter
use crate::com::hp::hpl::inkml::r#impl::InkPainter::InkPainter;

// Java: public class TraceGroupPaint
pub struct TraceGroupPaint {
    // Java field: TraceGroup traceGroup
    traceGroup: TraceGroup,
    // Java field: TraceSetting setting
    setting: TraceSetting,
    // Java field: Paint paint
    paint: i32,
    // Java field: Path path
    path: i32,
}

// Java: public class TraceGroupPaint
impl TraceGroupPaint {
    // Java: TraceGroupPaint(TraceGroup traceGroup, TraceSetting setting)
    pub fn TraceGroupPaint(trace_group: TraceGroup, setting: TraceSetting) -> Self {
      // Java: super();
      // Java: this.traceGroup = traceGroup;
      // Java: this.setting = setting;
      // Java: this.paint.setAntiAlias(true);
      Self {
          traceGroup: trace_group,
          setting,
          paint: 0,
          path: 0,
      }
    }

    // Java: boolean needDraw()
    pub fn needDraw(&self) -> bool {
      // Java: int size = this.traceGroup.size();
      let size = self.traceGroup.size();
      // Java: InkCreator currentInkCreator = this.traceGroup.getCurrentInkCreator();
      let current = self.traceGroup.getCurrentInkCreator();
      // Java: InkCreator endCurrentInkCreator = this.traceGroup.getEndCurrentInkCreator();
      let end_current = self.traceGroup.getEndCurrentInkCreator();
      // Java: return size > 0 || currentInkCreator != null || endCurrentInkCreator != null;
      size > 0 || current.is_some() || end_current.is_some()
    }

    // Java: void processDraw(Canvas canvas)
    pub fn processDraw(&self) {
      // Java: int size = this.traceGroup.size();
      // Java: InkCreator currentInkCreator = this.traceGroup.getCurrentInkCreator();
      // Java: InkCreator endCurrentInkCreator = this.traceGroup.getEndCurrentInkCreator();
      // Java: if (size != 0 || currentInkCreator != null || endCurrentInkCreator != null)
      {
         // Java: canvas.save();
         // Java: canvas.translate(0.0F, 0.0F);
         // Java: PointF scale = this.traceGroup.getInkScaleXY(PointF.obtain());
         // Java: canvas.scale(scale.x, scale.y);
         // Java: scale.recycle();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: InkCreator inkCreator = this.traceGroup.getInkCreator(i);
            // Java: if (inkCreator.hasData())
            {
               // Java: Ink ink = inkCreator.getInk();
               // Java: RectF inkRect = ink.getInkRect();
               // Java: canvas.save();
               // Java: canvas.translate(inkRect.left, inkRect.top);
               // Java: InkPainter painter = new InkPainter(ink, false);
               // Java: painter.drawPoundWithPathCache(canvas, inkRect.width(), inkRect.height(), this.setting.getMinStrokeWidth(), false);
               // Java: canvas.restore();
            }
         }
         // Java: if (currentInkCreator != null)
         {
            // Java: currentInkCreator.draw(canvas, this.paint, this.path, this.setting.getMinStrokeWidth(), false, 1.0F, 1.0F);
         }
         // Java: else if (endCurrentInkCreator != null)
         {
            // Java: endCurrentInkCreator.draw(canvas, this.paint, this.path, this.setting.getMinStrokeWidth(), false, 1.0F, 1.0F);
         }
         // Java: canvas.restore();
      }
    }

    // Java: static void processDraw(Canvas canvas, InkCreator inkCreator, InkMLConverter converter)
    pub fn processDraw_static(_ink_creator: &InkCreator, _converter: Option<&dyn InkMLConverter>) {
      // Java: if (canvas != null && inkCreator != null)
      {
         // Java: Paint paint = new Paint();
         // Java: Path path = new Path();
         // Java: if (converter == null)
         {
            // Java: inkCreator.draw(canvas, paint, path, TraceSetting.getDefaultMinStrokeWidth(), false, 1.0F, 1.0F);
         }
         // Java: else
         {
            // Java: canvas.save();
            // Java: canvas.translate(0.0F, 0.0F);
            // Java: PointF scale = new PointF(1.0F, 1.0F);
            // Java: converter.getInkScaleXY(scale);
            // Java: canvas.scale(scale.x, scale.y);
            // Java: inkCreator.draw(canvas, paint, path, TraceSetting.getDefaultMinStrokeWidth(), false, 1.0F, 1.0F);
            // Java: canvas.restore();
         }
      }
    }

}

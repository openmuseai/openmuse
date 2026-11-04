#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.graphics.SizeF
use util::graphics::SizeF::SizeF;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import java.util.ArrayList
// Java: import java.util.List

use super::TraceAdapter::TraceAdapter;

// Java: public class InkPathCache
pub struct InkPathCache {
    // Java field: Path path
    // Java field: List<PointF> points
    // Java field: Paint paint
    // Java field: float offsetX
    pub(crate) offsetX: f32,
    // Java field: float offsetY
    pub(crate) offsetY: f32,
    // Java field: float scaleX
    pub(crate) scaleX: f32,
    // Java field: float scaleY
    pub(crate) scaleY: f32,
    // Java field: float width
    pub(crate) width: f32,
    // Java field: float height
    pub(crate) height: f32,
}

// Java: public class InkPathCache
impl InkPathCache {
    // Java: InkPathCache()
    pub fn InkPathCache() -> Self {
        // Java: super();
        Self {
            offsetX: 0.0,
            offsetY: 0.0,
            scaleX: 1.0,
            scaleY: 1.0,
            width: 0.0,
            height: 0.0,
        }
    }

    // Java: boolean setPath(Ink ink, Path path, List<PointF> points, Paint paint, float width, float height)
    pub fn setPath(&mut self, ink: &Ink, _width: f32, _height: f32) -> bool {
        // Java: if (canSetPath(ink, path))
        if Self::canSetPath_ink(ink) {
            // Java: this.path = path;
            // Java: this.points = points;
            // Java: this.paint = paint;
            // Java: this.width = width;
            self.width = _width;
            // Java: this.height = height;
            self.height = _height;
            // Java: RectF inkRect = ink.getInkRect();
            let ink_rect = ink.getInkRect();
            // Java: SizeF scaleXY = Ink.calScaleXY(inkRect, width, height);
            let scale_xy = Ink::calScaleXY(&ink_rect, _width, _height);
            // Java: this.offsetX = -inkRect.left * scaleXY.width;
            self.offsetX = -ink_rect.left * scale_xy.width;
            // Java: this.offsetY = -inkRect.top * scaleXY.height;
            self.offsetY = -ink_rect.top * scale_xy.height;
            // Java: TraceAdapter trace = (TraceAdapter)ink.getTraces().get(0);
            // Java: this.scaleX = 1.0F / trace.getScaleX();
            // Java: this.scaleY = 1.0F / trace.getScaleY();
            if let Some(trace) = ink.getTraces().first() {
                self.scaleX = 1.0 / trace.getScaleX();
                self.scaleY = 1.0 / trace.getScaleY();
            }
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: void drawPath(Canvas canvas)
    pub fn drawPath(&self) {
        // Java: canvas.save();
        // Java: canvas.translate(this.offsetX, this.offsetY);
        // Java: canvas.scale(this.scaleX, this.scaleY);
        // Java: canvas.drawPath(this.path, this.paint);
        // Java: if (this.points != null && this.points.size() > 0)
        // Java: for(PointF point : this.points)
        // Java: canvas.drawPoint(point.x, point.y, this.paint);
        // Java: canvas.restore();
    }

    // Java: boolean canUsePath(float width, float height)
    pub fn canUsePath(&self, width: f32, height: f32) -> bool {
        // Java: return Math.abs(this.width - width) < 1.0F && Math.abs(this.height - height) < 1.0F;
        (self.width - width).abs() < 1.0 && (self.height - height).abs() < 1.0
    }

    // Java: static boolean canSetPath(Ink ink, Path path)
    pub fn canSetPath_ink(ink: &Ink) -> bool {
        // Java: return path != null && !path.isEmpty() && canSetPath(ink);
        Self::canSetPath(ink)
    }

    // Java: static boolean canSetPath(Ink ink)
    pub fn canSetPath(ink: &Ink) -> bool {
        // Java: if (ink == null)
        // Java: return false;
        // Java: else
        // Java: ArrayList<TraceAdapter> traces = ink.getTraces();
        let traces = ink.getTraces();
        // Java: int size = traces.size();
        let size = traces.len();
        // Java: if (size == 1)
        if size == 1 {
            // Java: return true;
            true
        // Java: else if (size == 0)
        } else if size == 0 {
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: TraceAdapter trace = (TraceAdapter)traces.get(0);
            let trace = &traces[0];
            // Java: float sx = trace.getScaleX();
            let sx = trace.getScaleX();
            // Java: float sy = trace.getScaleY();
            let sy = trace.getScaleY();
            // Java: Brush brush = trace.getBrush();
            let brush = trace.getBrush();
            // Java: if (brush.getMaxFChannel() <= 0.0F)
            if brush.getMaxFChannel() <= 0.0 {
                // Java: return false;
                return false;
            }
            // Java: else
            // Java: int i = 1;
            let mut i = 1;
            // Java: while(i < size)
            while i < size {
                // Java: trace = (TraceAdapter)traces.get(i);
                let t = &traces[i];
                // Java: if (sx == trace.getScaleX() && sy == trace.getScaleY())
                if sx == t.getScaleX() && sy == t.getScaleY() {
                    // Java: Brush brush2 = trace.getBrush();
                    let brush2 = t.getBrush();
                    // Java: if (brush.getColor() == brush2.getColor() && brush.getWidth() == brush2.getWidth() && !(brush2.getMaxFChannel() <= 0.0F))
                    if brush.getColor() == brush2.getColor() && brush.getWidth() == brush2.getWidth() && brush2.getMaxFChannel() > 0.0 {
                        // Java: ++i;
                        i += 1;
                        // Java: continue;
                        continue;
                    }
                    // Java: return false;
                    return false;
                }
                // Java: return false;
                return false;
            }
            // Java: return true;
            true
        }
    }
}

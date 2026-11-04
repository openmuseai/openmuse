// Java: import android.graphics.Canvas
// Java: import android.graphics.DashPathEffect
// Java: import android.graphics.Paint
// Java: import android.graphics.PathEffect
// Java: import android.graphics.Paint.Style

// Java: public class FramePaint
pub struct FramePaint {
}

// Java: public class FramePaint
impl FramePaint {
    // Java: FramePaint()
    pub fn FramePaint() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static void drawDashLine(float startX, float startY, float stopX, float stopY, Canvas canvas, float strokeWidth, int color)
    pub fn drawDashLine(_start_x: f32, _start_y: f32, _stop_x: f32, _stop_y: f32, _stroke_width: f32, _color: i32) {
      // Java: Paint paint = new Paint(1);
      // Java: paint.setStyle(Style.STROKE);
      // Java: paint.setStrokeWidth(strokeWidth);
      // Java: paint.setColor(color);
      // Java: PathEffect effects = new DashPathEffect(new float[]{16.0F, 8.0F}, 16.0F);
      // Java: paint.setPathEffect(effects);
      // Java: canvas.drawLine(startX, startY, stopX, stopY, paint);
    }

}

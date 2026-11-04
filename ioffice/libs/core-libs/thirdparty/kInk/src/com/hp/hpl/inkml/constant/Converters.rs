// Java: import android.graphics.Paint
// Java: import android.graphics.PorterDuffXfermode
// Java: import android.graphics.Xfermode
// Java: import android.graphics.Paint.Cap
// Java: import android.graphics.Paint.Join
// Java: import android.graphics.Paint.Style
// Java: import android.graphics.PorterDuff.Mode
// Java: import com.hp.hpl.inkml.impl.Brush
use crate::com::hp::hpl::inkml::r#impl::Brush::Brush;

pub struct Paint {
    pub anti_alias: bool,
    pub color: i32,
    pub stroke_width: f32,
}

// Java: public class Converters
pub struct Converters {
}

// Java: public class Converters
impl Converters {
    // Java: Converters()
    pub fn Converters() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static void convertPaint(Paint paint, Brush brush, float minStrokeWidth)
    pub fn convertPaint_simple(paint: &mut Paint, brush: &Brush, min_stroke_width: f32) {
      // Java: convertPaint(paint, brush, minStrokeWidth, false, false);
      Converters::convertPaint(paint, brush, min_stroke_width, false, false);
    }

    // Java: static void convertPaint(Paint paint, Brush brush, float minStrokeWidth, boolean ingoreMaskPen, boolean forceMinStroke)
    pub fn convertPaint(_paint: &mut Paint, _brush: &Brush, _min_stroke_width: f32, _ingore_mask_pen: bool, _force_min_stroke: bool) {
      // Java: paint.setAntiAlias(true);
      // Java: int color = brush.getColor();
      // Java: paint.setColor(color);
      // Java: paint.setStrokeCap(getCap(brush));
      // Java: paint.setStrokeJoin(Join.ROUND);
      // Java: if (brush.getShape() != Brush.Shape.rectangle && !(brush.getMaxFChannel() > 0.0F))
      {
         // Java: paint.setStyle(Style.STROKE);
         // Java: if (forceMinStroke)
         {
            // Java: paint.setStrokeWidth(minStrokeWidth);
         }
         // Java: else
         {
            // Java: float strokeWidth = brush.getWidth();
            // Java: strokeWidth = Math.max(minStrokeWidth, strokeWidth);
            // Java: paint.setStrokeWidth(strokeWidth);
         }
      }
      // Java: else
      {
         // Java: paint.setStyle(Style.FILL);
      }
      // Java: Brush.RasterOp rasterOp = brush.getRasterOp();
      // Java: if (rasterOp == Brush.RasterOp.copyPen)
      {
         // Java: paint.setXfermode((Xfermode)null);
      }
      // Java: else if (rasterOp == Brush.RasterOp.maskPen)
      {
         // Java: if (ingoreMaskPen)
         {
            // Java: paint.setXfermode((Xfermode)null);
         }
         // Java: else
         {
            // Java: paint.setXfermode(new PorterDuffXfermode(Mode.DARKEN));
         }
      }
      // Java: else
      {
         // Java: paint.setXfermode(new PorterDuffXfermode(Mode.DARKEN));
      }
    }

}

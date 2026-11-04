#![allow(dead_code, non_snake_case, unused_variables)]

// Java: import android.graphics.Paint
// (android.graphics.Paint not available in Rust)
// Java: import android.graphics.PointF
// (android.graphics.PointF not available in Rust)
// Java: import cn.wps.base.assertion.KSAssert
// (referenced via debug_assert! directly)

pub type Paint = ();
pub type KCanvas = ();

#[derive(Clone, Debug)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

// Java: public class KDraw
pub struct KDraw {
}

// Java: public class KDraw
#[allow(non_snake_case)]
impl KDraw {
    // Java: KDraw()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static void drawPoints(KCanvas canvas, int pointMode, PointF[] points, Paint paint)
    pub fn drawPoints(canvas: &KCanvas, pointMode: i32, points: &[PointF], paint: &Paint) {
      // Java: switch (pointMode)
      match pointMode {
         // Java: case 1:
         1 => {
            // Java: KSAssert.assertFalse(true);
            debug_assert!(false);
         }
         // Java: case 2:
         // Java: default:
         2 => {
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: KSAssert.assertFalse(true);
            debug_assert!(false);
         }
         _ => {}
      }
    }

}

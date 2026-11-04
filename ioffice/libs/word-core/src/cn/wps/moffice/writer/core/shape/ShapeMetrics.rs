// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::*;
// Java: import cn.wps.graphics.Rect
use util::graphics::Rect::*;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;

// java public static final float TO_LAYOUT = 20.0F;
pub(crate) const TO_LAYOUT: f32 = 20.0f32;
// java public static final float FROM_LAYOUT = 0.05F;
pub(crate) const FROM_LAYOUT: f32 = 0.05f32;

// java public class ShapeMetrics
#[derive(Default)]
pub struct ShapeMetrics {
}

// java public class ShapeMetrics
impl ShapeMetrics {
    // java public static int transToLayout(float value)
    pub fn transToLayout__float(value: f32) -> i32 {
      // java return round(value * 20.0F);
      return Self::round(value * 20.0);
    }

    // java public static float transFromLayout(float value)
    pub fn transFromLayout__float(value: f32) -> f32 {
      // java return value * 0.05F;
      return value * 0.05f32;
    }

    // java public static void adjustToLayout(RectF rect)
    pub fn adjustToLayout(rect: &mut RectF) {
      // java rect.set(rect.left * 20.0F, rect.top * 20.0F, rect.right * 20.0F, rect.bottom * 20.0F);
      rect.set(rect.left * 20.0, rect.top * 20.0, rect.right * 20.0, rect.bottom * 20.0);
    }

    // java public static void adjustFromLayout(RectF rect)
    pub fn adjustFromLayout(rect: &mut RectF) {
      // java rect.scale(0.05F, 0.05F);
      rect.scale(0.05f32, 0.05f32);
    }

    // java public static RectF transToLayout(RectF shapeRect)
    pub fn transToLayout__rectf(shapeRect: &RectF) -> RectF {
      // java RectF layoutRect = RectF.obtain();
      let mut layoutRect: RectF = RectF::obtain();
      // java layoutRect.set(shapeRect);
      layoutRect.set(shapeRect.left, shapeRect.top, shapeRect.right, shapeRect.bottom);
      // java layoutRect.scale(20.0F, 20.0F);
      layoutRect.scale(20.0, 20.0);
      // java return layoutRect;
      return layoutRect;
    }

    // java public static void transToLayout(RectF shapeRect, Rect layoutRect)
    pub fn transToLayout__rectf_rect(shapeRect: &RectF, layoutRect: &mut Rect) {
      // java layoutRect.set(round(shapeRect.left * 20.0F), round(shapeRect.top * 20.0F), round(shapeRect.right * 20.0F), round(shapeRect.bottom * 20.0F));
      layoutRect.set(Self::round(shapeRect.left * 20.0), Self::round(shapeRect.top * 20.0), Self::round(shapeRect.right * 20.0), Self::round(shapeRect.bottom * 20.0));
    }

    // java public static void transToLayout(RectF shapeRect, RectF layoutRect)
    pub fn transToLayout__rectf_rectf(shapeRect: &RectF, layoutRect: &mut RectF) {
      // java layoutRect.set(shapeRect.left * 20.0F, shapeRect.top * 20.0F, shapeRect.right * 20.0F, shapeRect.bottom * 20.0F);
      layoutRect.set(shapeRect.left * 20.0, shapeRect.top * 20.0, shapeRect.right * 20.0, shapeRect.bottom * 20.0);
    }

    // java public static RectF transFromLayout(Rect layoutRect)
    pub fn transFromLayout__rect(layoutRect: &Rect) -> RectF {
      // java RectF shapeRect = RectF.obtain();
      let mut shapeRect: RectF = RectF::obtain();
      // java shapeRect.set((float)layoutRect.left, (float)layoutRect.top, (float)layoutRect.right, (float)layoutRect.bottom);
      shapeRect.set(layoutRect.left as f32, layoutRect.top as f32, layoutRect.right as f32, layoutRect.bottom as f32);
      // java shapeRect.scale(0.05F, 0.05F);
      shapeRect.scale(0.05f32, 0.05f32);
      // java return shapeRect;
      return shapeRect;
    }

    // java public static void transFromLayout(RectF layoutRect, RectF shapeRect)
    pub fn transFromLayout__rectf_rectf(layoutRect: &RectF, shapeRect: &mut RectF) {
      // java shapeRect.set(layoutRect);
      shapeRect.set(layoutRect.left, layoutRect.top, layoutRect.right, layoutRect.bottom);
      // java shapeRect.scale(0.05F, 0.05F);
      shapeRect.scale(0.05f32, 0.05f32);
    }

    // java public static PointF transFromLayout(PointF layoutPoint)
    pub fn transFromLayout__pointf(layoutPoint: &PointF) -> PointF {
      // java PointF shapePoint = PointF.obtain();
      let mut shapePoint: PointF = PointF::obtain();
      // java shapePoint.set(layoutPoint);
      shapePoint.set(layoutPoint.x, layoutPoint.y);
      // java shapePoint.scale(0.05F, 0.05F);
      shapePoint.scale(0.05f32, 0.05f32);
      // java return shapePoint;
      return shapePoint;
    }

    // java public static int round(float f)
    pub fn round(f: f32) -> i32 {
      // java return f >= 0.0F ? (int)(f + 0.5F) : (int)(f - 0.5F);
      return if f >= 0.0 { (f + 0.5) as i32 } else { (f - 0.5) as i32 };
    }

}

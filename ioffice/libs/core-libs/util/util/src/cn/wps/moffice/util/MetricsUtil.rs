// Java: import cn.wps.graphics.Rect
// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.graphics.Size
// Java: import cn.wps.graphics.SizeF
use crate::graphics::Rect::Rect;
use crate::graphics::RectF::RectF;
use crate::graphics::Size::Size;
use crate::graphics::SizeF::SizeF;

// Java: public class MetricsUtil
pub struct MetricsUtil;

// Java field: float point2pixel_x_ratio
pub static mut point2pixel_x_ratio: f32 = 1.3333536;
// Java field: float pixel2point_x_ratio
pub static mut pixel2point_x_ratio: f32 = 0.74998856;
// Java field: float point2pixel_y_ratio
pub static mut point2pixel_y_ratio: f32 = 1.3333536;
// Java field: float pixel2point_y_ratio
pub static mut pixel2point_y_ratio: f32 = 0.74998856;
// Java field: float twips2pixel_x_ratio
pub static mut twips2pixel_x_ratio: f32 = 1.3333536 / 20.0;
// Java field: float pixel2twips_x_ratio
pub static mut pixel2twips_x_ratio: f32 = 0.74998856 * 20.0;
// Java field: float twips2pixel_y_ratio
pub static mut twips2pixel_y_ratio: f32 = 1.3333536 / 20.0;
// Java field: float pixel2twips_y_ratio
pub static mut pixel2twips_y_ratio: f32 = 0.74998856 * 20.0;

// Java field: int DPI
pub const DPI: i32 = 96;
// Java field: float POINTS_PER_INCH
pub const POINTS_PER_INCH: f32 = 71.9989;
// Java field: float DLG_POINT_PIXEL
pub const DLG_POINT_PIXEL: f32 = 1.3333536;
// Java field: float PI_PT
pub const PI_PT: f32 = 12.0;
// Java field: float CM_PT
pub const CM_PT: f32 = 28.35;
// Java field: int NORMAL_SCREEN_DPI
pub const NORMAL_SCREEN_DPI: i32 = 480;
// Java field: int LARGE_SCREEN_DPI
pub const LARGE_SCREEN_DPI: i32 = 600;
// Java field: float TWIPS_PT
pub const TWIPS_PT: f32 = 20.0;

// Java: public class MetricsUtil
impl MetricsUtil {
    // Java: MetricsUtil()
    // Java: super();

    // Java: static float point2pixel_x(float nPoint)
    pub fn point2pixel_x(n_point: f32) -> f32 {
        // Java: return nPoint * point2pixel_x_ratio;
        unsafe { n_point * point2pixel_x_ratio }
    }

    // Java: static float pixel2point_x(float nPixel)
    pub fn pixel2point_x(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2point_x_ratio;
        unsafe { n_pixel * pixel2point_x_ratio }
    }

    // Java: static float point2pixel_y(float nPoint)
    pub fn point2pixel_y(n_point: f32) -> f32 {
        // Java: return nPoint * point2pixel_y_ratio;
        unsafe { n_point * point2pixel_y_ratio }
    }

    // Java: static float pixel2point_y(float nPixel)
    pub fn pixel2point_y(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2point_y_ratio;
        unsafe { n_pixel * pixel2point_y_ratio }
    }

    // Java: static float point2pixel(float nPoint)
    pub fn point2pixel_f(n_point: f32) -> f32 {
        // Java: return nPoint * point2pixel_x_ratio;
        unsafe { n_point * point2pixel_x_ratio }
    }

    // Java: static float pixel2point(float nPixel)
    pub fn pixel2point_f(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2point_x_ratio;
        unsafe { n_pixel * pixel2point_x_ratio }
    }

    // Java: static float twips2pixel_x(float nPoint)
    pub fn twips2pixel_x(n_point: f32) -> f32 {
        // Java: return nPoint * twips2pixel_x_ratio;
        unsafe { n_point * twips2pixel_x_ratio }
    }

    // Java: static float pixel2twips_x(float nPixel)
    pub fn pixel2twips_x(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2twips_x_ratio;
        unsafe { n_pixel * pixel2twips_x_ratio }
    }

    // Java: static float twips2pixel_y(float nPoint)
    pub fn twips2pixel_y(n_point: f32) -> f32 {
        // Java: return nPoint * twips2pixel_y_ratio;
        unsafe { n_point * twips2pixel_y_ratio }
    }

    // Java: static float pixel2twips_y(float nPixel)
    pub fn pixel2twips_y(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2twips_y_ratio;
        unsafe { n_pixel * pixel2twips_y_ratio }
    }

    // Java: static float twips2pixel(float nPoint)
    pub fn twips2pixel(n_point: f32) -> f32 {
        // Java: return nPoint * twips2pixel_x_ratio;
        unsafe { n_point * twips2pixel_x_ratio }
    }

    // Java: static float pixel2twips(float nPixel)
    pub fn pixel2twips(n_pixel: f32) -> f32 {
        // Java: return nPixel * pixel2twips_x_ratio;
        unsafe { n_pixel * pixel2twips_x_ratio }
    }

    // Java: static float twips2point(float twips)
    pub fn twips2point(twips: f32) -> f32 {
        // Java: return twips / 20.0F;
        twips / 20.0
    }

    // Java: static int point2twips(float point)
    pub fn point2twips(point: f32) -> i32 {
        // Java: return (int)((double)(point * 20.0F) + (double)0.5F);
        ((point * 20.0) as f64 + 0.5f64) as i32
    }

    // Java: static RectF transform(Rect rect)
    pub fn transform_rect_to_rectf(rect: &Rect) -> RectF {
        // Java: return pixel2point(rect);
        Self::pixel2point_rect(rect)
    }

    // Java: static RectF pixel2point(Rect rect)
    pub fn pixel2point_rect(rect: &Rect) -> RectF {
        // Java: return new RectF((float)rect.left * pixel2point_x_ratio, (float)rect.top * pixel2point_y_ratio, (float)rect.right * pixel2point_x_ratio, (float)rect.bottom * pixel2point_y_ratio);
        unsafe {
            RectF::new_with(
                rect.left as f32 * pixel2point_x_ratio,
                rect.top as f32 * pixel2point_y_ratio,
                rect.right as f32 * pixel2point_x_ratio,
                rect.bottom as f32 * pixel2point_y_ratio,
            )
        }
    }

    // Java: static Rect transform(RectF rect)
    pub fn transform_rectf_to_rect(rect: &RectF) -> Rect {
        // Java: return point2pixel(rect);
        Self::point2pixel_rect(rect)
    }

    // Java: static Rect point2pixel(RectF rect)
    pub fn point2pixel_rect(rect: &RectF) -> Rect {
        // Java: return new Rect((int)(rect.left * point2pixel_x_ratio), (int)(rect.top * point2pixel_y_ratio), (int)(rect.right * point2pixel_x_ratio), (int)(rect.bottom * point2pixel_y_ratio));
        unsafe {
            Rect::new_with(
                (rect.left * point2pixel_x_ratio) as i32,
                (rect.top * point2pixel_y_ratio) as i32,
                (rect.right * point2pixel_x_ratio) as i32,
                (rect.bottom * point2pixel_y_ratio) as i32,
            )
        }
    }

    // Java: static SizeF pixel2point(Size size)
    pub fn pixel2point_size(size: &Size) -> SizeF {
        // Java: return new SizeF((float)size.width * pixel2point_x_ratio, (float)size.height * pixel2point_y_ratio);
        unsafe {
            SizeF::SizeF_wh(
                size.width as f32 * pixel2point_x_ratio,
                size.height as f32 * pixel2point_y_ratio,
            )
        }
    }

    // Java: static Size point2pixel(SizeF size)
    pub fn point2pixel_size(size: &SizeF) -> Size {
        // Java: return new Size((int)(size.width * point2pixel_x_ratio), (int)(size.height * point2pixel_y_ratio));
        unsafe {
            Size::new_with(
                (size.width * point2pixel_x_ratio) as i32,
                (size.height * point2pixel_y_ratio) as i32,
            )
        }
    }

    // Java: static float pointToUnit(float value, int unit)
    pub fn point_to_unit(value: f32, unit: i32) -> f32 {
        // Java: if (unit == 0)
        if unit == 0 {
            // Java: return value / 71.9989F;
            return value / POINTS_PER_INCH;
        }
        // Java: else if (unit == 1)
        else if unit == 1 {
            // Java: return value / 28.35F;
            return value / CM_PT;
        }
        // Java: else if (unit == 2)
        else if unit == 2 {
            // Java: return value / 2.835F;
            return value / 2.835;
        }
        // Java: else
        else {
            // Java: return unit == 4 ? value / 12.0F : value;
            if unit == 4 { value / PI_PT } else { value }
        }
    }

    // Java: static float unitToPoint(float value, int unit)
    pub fn unit_to_point(value: f32, unit: i32) -> f32 {
        // Java: return adjustPoint(changeUnitToPoint(value, unit));
        Self::adjust_point(Self::change_unit_to_point(value, unit))
    }

    // Java: static float adjustPoint(float value)
    pub fn adjust_point(value: f32) -> f32 {
        // Java: if (value == 0.0F)
        if value == 0.0 {
            // Java: return 0.0F;
            return 0.0;
        }
        // Java: else
        else {
            // Java: return value > 0.0F ? (float)((int)((double)(value * 20.0F) + (double)0.5F)) / 20.0F : (float)(-((int)((double)(-value * 20.0F) + (double)0.5F))) / 20.0F;
            if value > 0.0 {
                ((value * 20.0) as f64 + 0.5f64) as i32 as f32 / 20.0
            } else {
                -((((-value) * 20.0) as f64 + 0.5f64) as i32 as f32) / 20.0
            }
        }
    }

    // Java: static float changeUnitToPoint(float value, int unit)
    pub fn change_unit_to_point(value: f32, unit: i32) -> f32 {
        // Java: if (unit == 0)
        if unit == 0 {
            // Java: return value * 71.9989F;
            return value * POINTS_PER_INCH;
        }
        // Java: else if (unit == 1)
        else if unit == 1 {
            // Java: return value * 28.35F;
            return value * CM_PT;
        }
        // Java: else if (unit == 2)
        else if unit == 2 {
            // Java: return value * 2.835F;
            return value * 2.835;
        }
        // Java: else
        else {
            // Java: return unit == 4 ? value * 12.0F : value;
            if unit == 4 { value * PI_PT } else { value }
        }
    }
}

// Java: static
// Java: twips2pixel_x_ratio = point2pixel_x_ratio / 20.0F;
// Java: pixel2twips_x_ratio = pixel2point_x_ratio * 20.0F;
// Java: twips2pixel_y_ratio = point2pixel_y_ratio / 20.0F;
// Java: pixel2twips_y_ratio = pixel2point_y_ratio * 20.0F;

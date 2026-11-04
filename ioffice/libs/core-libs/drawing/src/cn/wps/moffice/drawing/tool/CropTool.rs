// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

// Java: public class CropTool
pub struct CropTool {
}

// Java: public class CropTool
impl CropTool {
    // Java: CropTool()
    pub fn new() -> Self {
      // Java: super();
      CropTool {}
    }

    // Java: static void calRectBound(RectF srcRect, RectF dstRect, float cropLeft, float cropTop, float cropRight, float cropBottom)
    pub fn calRectBound(src_rect: &RectF, dst_rect: &mut RectF, crop_left: f32, crop_top: f32, crop_right: f32, crop_bottom: f32) {
      // Java: float dstWidth = srcRect.width() / (1.0F - cropLeft - cropRight);
      let dst_width = src_rect.width() / (1.0 - crop_left - crop_right);
      // Java: float dstHeight = srcRect.height() / (1.0F - cropTop - cropBottom);
      let dst_height = src_rect.height() / (1.0 - crop_top - crop_bottom);
      // Java: float dstLeft = srcRect.left - cropLeft * dstWidth;
      let dst_left = src_rect.left - crop_left * dst_width;
      // Java: float dstTop = srcRect.top - cropTop * dstHeight;
      let dst_top = src_rect.top - crop_top * dst_height;
      // Java: dstRect.set(dstLeft, dstTop, dstLeft + dstWidth, dstTop + dstHeight);
      dst_rect.set(dst_left, dst_top, dst_left + dst_width, dst_top + dst_height);
    }

    // Java: static void calRectCrop(RectF srcRect, RectF dstRect, RectF cropRect)
    pub fn calRectCrop(src_rect: &RectF, dst_rect: &RectF, crop_rect: &mut RectF) {
      // Java: cropRect.left = (srcRect.left - dstRect.left) / dstRect.width();
      crop_rect.left = (src_rect.left - dst_rect.left) / dst_rect.width();
      // Java: cropRect.top = (srcRect.top - dstRect.top) / dstRect.height();
      crop_rect.top = (src_rect.top - dst_rect.top) / dst_rect.height();
      // Java: cropRect.right = (dstRect.right - srcRect.right) / dstRect.width();
      crop_rect.right = (dst_rect.right - src_rect.right) / dst_rect.width();
      // Java: cropRect.bottom = (dstRect.bottom - srcRect.bottom) / dstRect.height();
      crop_rect.bottom = (dst_rect.bottom - src_rect.bottom) / dst_rect.height();
    }

}

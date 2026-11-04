#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;

// Java: public class PictureCropTool
pub struct PictureCropTool {
}

// Java: public class PictureCropTool
impl PictureCropTool {
    // Java: PictureCropTool()
    pub fn new() -> Self {
      // Java: super();
      PictureCropTool {}
    }

    // Java: static void calRect(Picture pic, RectF srcRect, RectF dstRect)
    pub fn calRect(pic: &Element, srcRect: &RectF, dstRect: &mut RectF) {
      // Java: float dstWidth = srcRect.width() / (1.0F - pic.getCropLeft() - pic.getCropRight());
      // TODO: Picture type not yet available
      let cropLeft: f32 = 0.0;
      let cropRight: f32 = 0.0;
      let cropTop: f32 = 0.0;
      let cropBottom: f32 = 0.0;
      let dstWidth = srcRect.width() / (1.0_f32 - cropLeft - cropRight);
      // Java: float dstHeight = srcRect.height() / (1.0F - pic.getCropTop() - pic.getCropBottom());
      let dstHeight = srcRect.height() / (1.0_f32 - cropTop - cropBottom);
      // Java: float dstLeft = srcRect.left - pic.getCropLeft() * dstWidth;
      let dstLeft = srcRect.left - cropLeft * dstWidth;
      // Java: float dstTop = srcRect.top - pic.getCropTop() * dstHeight;
      let dstTop = srcRect.top - cropTop * dstHeight;
      // Java: dstRect.set(dstLeft, dstTop, dstLeft + dstWidth, dstTop + dstHeight);
      dstRect.set(dstLeft, dstTop, dstLeft + dstWidth, dstTop + dstHeight);
    }

}

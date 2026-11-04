#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.graphics.Bitmap
use crate::graphics::bitmap::{Bitmap, CompressFormat, FromType};
use super::IPicture::IPicture;

// Java: public interface IBitmapRef
pub trait IBitmapRef {
    // Java: abstract IPicture getPicture()
    fn getPicture(&self) -> Option<Box<dyn IPicture>>;

    // Java: abstract Bitmap get()
    fn get(&self) -> Option<Box<dyn Bitmap>>;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}

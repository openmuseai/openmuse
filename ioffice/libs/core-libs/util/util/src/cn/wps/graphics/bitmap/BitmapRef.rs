#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.graphics.Bitmap
use crate::graphics::bitmap::{Bitmap, CompressFormat, FromType};
use super::IPicture::IPicture;
use super::IBitmapRef::IBitmapRef;

// Java: public class BitmapRef implements IBitmapRef
pub struct BitmapRef {
    // Java field: Bitmap bitmap
    pub(crate) bitmap: Option<Box<dyn Bitmap>>,
    // Java field: IPicture pic
    pub(crate) pic: Option<Box<dyn IPicture>>,
}

// Java: public class BitmapRef implements IBitmapRef
impl BitmapRef {
    // Java: BitmapRef(Bitmap bmp, IPicture pic)
    pub fn new(bmp: Box<dyn Bitmap>, pic: Box<dyn IPicture>) -> Self {
        // Java: super();
        Self {
            // Java: this.bitmap = bmp;
            bitmap: Some(bmp),
            // Java: this.pic = pic;
            pic: Some(pic),
        }
    }

    // Java: Bitmap get()
    pub fn get(&self) -> &Option<Box<dyn Bitmap>> {
        // Java: return this.bitmap;
        &self.bitmap
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.pic = null;
        self.pic = None;
        // Java: if (this.bitmap != null && !this.bitmap.isRecycled())
        if let Some(ref mut bmp) = self.bitmap {
            if !bmp.isRecycled() {
                // Java: this.bitmap.recycle();
                bmp.recycle();
                // Java: this.bitmap = null;
            }
        }
        // Java: this.bitmap = null;
        self.bitmap = None;
    }

    // Java: IPicture getPicture()
    pub fn getPicture(&self) -> &Option<Box<dyn IPicture>> {
        // Java: return this.pic;
        &self.pic
    }
}

impl IBitmapRef for BitmapRef {
    fn getPicture(&self) -> Option<Box<dyn IPicture>> {
        None
    }
    fn get(&self) -> Option<Box<dyn Bitmap>> {
        None
    }
    fn dispose(&mut self) {
        BitmapRef::dispose(self);
    }
}

#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use std::sync::{Arc, Mutex};
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import android.graphics.Bitmap
// Bitmap is Android-specific, using Element as placeholder

// Java: public class MyBitmap
pub struct MyBitmap {
    // Java field: Bitmap mBitmap
    pub(crate) mBitmap: Element,
    // Java field: boolean canRecycle
    pub(crate) canRecycle: bool,
}

// Java: public class MyBitmap
impl MyBitmap {
    // Java: MyBitmap(Bitmap mBitmap, boolean canRecycle)
    pub fn new(mBitmap: Element, canRecycle: bool) -> Self {
      // Java: super();
      let mut instance = MyBitmap { mBitmap: None, canRecycle: false };
      instance.init(mBitmap, canRecycle);
      instance
    }

    fn init(&mut self, mBitmap: Element, canRecycle: bool) {
      // Java: this.mBitmap = mBitmap;
      self.mBitmap = mBitmap;
      // Java: this.canRecycle = canRecycle;
      self.canRecycle = canRecycle;
    }

    // Java: boolean isRecycled()
    pub fn isRecycled(&self) -> bool {
      // Java: return null != this.mBitmap ? this.mBitmap.isRecycled() : false;
      self.mBitmap.is_none()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: if (null != this.mBitmap && this.canRecycle && !this.mBitmap.isRecycled())
      if self.mBitmap.is_some() && self.canRecycle && !self.isRecycled() {
         // Java: this.mBitmap.recycle();
         self.mBitmap = None;
      }
    }

    // Java: Bitmap getBitmap()
    pub fn getBitmap(&self) -> &Element {
      // Java: return this.mBitmap;
      &self.mBitmap
    }

    // Java: boolean canRecycle()
    pub fn canRecycle(&self) -> bool {
      // Java: return this.canRecycle;
      self.canRecycle
    }

}

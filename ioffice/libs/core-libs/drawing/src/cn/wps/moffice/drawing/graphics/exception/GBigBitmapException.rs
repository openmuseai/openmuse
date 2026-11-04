#![allow(non_snake_case, dead_code)]

use super::GraphicsException::GraphicsException;
use super::IGraphicsException::IGraphicsException;

// Java: public class GBigBitmapException extends GraphicsException
pub struct GBigBitmapException {
    pub base: GraphicsException,
    // Java field: int mMediaId
    pub(crate) mMediaId: i32,
}

// Java: public class GBigBitmapException extends GraphicsException
impl GBigBitmapException {
    // Java: GBigBitmapException(int mediaId, int expectedWidth, int expectedHeight)
    pub fn new(mediaId: i32, expectedWidth: i32, expectedHeight: i32) -> Self {
      // Java: super(0, expectedWidth, expectedHeight);
      let base = GraphicsException::new(0, expectedWidth, expectedHeight);
      // Java: this.mMediaId = mediaId;
      GBigBitmapException { base, mMediaId: mediaId }
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
      // Java: return this.mMediaId;
      self.mMediaId
    }

}

impl IGraphicsException for GBigBitmapException {
    fn getType(&self) -> i32 { self.base.getType() }
    fn getExpectedWidth(&self) -> i32 { self.base.getExpectedWidth() }
    fn getExpectedHeight(&self) -> i32 { self.base.getExpectedHeight() }
}

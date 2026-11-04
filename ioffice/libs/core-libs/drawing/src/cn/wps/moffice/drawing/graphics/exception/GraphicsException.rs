#![allow(non_snake_case, dead_code)]

use super::IGraphicsException::IGraphicsException;

// Java: public class GraphicsException implements IGraphicsException
pub struct GraphicsException {
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: int mExpectedWidth
    pub(crate) mExpectedWidth: i32,
    // Java field: int mExpectedHeight
    pub(crate) mExpectedHeight: i32,
}

// Java: public class GraphicsException implements IGraphicsException
impl GraphicsException {
    // Java: GraphicsException(int type, int expectedWidth, int expectedHeight)
    pub fn new(r#type: i32, expectedWidth: i32, expectedHeight: i32) -> Self {
      // Java: super();
      // Java: this.mType = type;
      // Java: this.mExpectedWidth = expectedWidth;
      // Java: this.mExpectedHeight = expectedHeight;
      GraphicsException { mType: r#type, mExpectedWidth: expectedWidth, mExpectedHeight: expectedHeight }
    }

    // Java: void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
      // Java: this.mType = type;
      self.mType = r#type;
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.mType;
      self.mType
    }

    // Java: int getExpectedWidth()
    pub fn getExpectedWidth(&self) -> i32 {
      // Java: return this.mExpectedWidth;
      self.mExpectedWidth
    }

    // Java: int getExpectedHeight()
    pub fn getExpectedHeight(&self) -> i32 {
      // Java: return this.mExpectedHeight;
      self.mExpectedHeight
    }

}

impl IGraphicsException for GraphicsException {
    fn getType(&self) -> i32 { self.mType }
    fn getExpectedWidth(&self) -> i32 { self.mExpectedWidth }
    fn getExpectedHeight(&self) -> i32 { self.mExpectedHeight }
}

// Java: public class TextShape3d
pub struct TextShape3d {
    // Java field: TextShape mTopShape
    pub(crate) mTopShape: TextShape,
    // Java field: TextShape mBottomShape
    pub(crate) mBottomShape: TextShape,
    // Java field: Depth mDepth
    pub(crate) mDepth: Depth,
    // Java field: Contour mContour
    pub(crate) mContour: Contour,
}

// Java: public enum TextShapeType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextShapeType {
    NONE,
}

// Java: public class TextShape
pub struct TextShape {
    // Java field: TextShapeType mShapeType
    pub(crate) mShapeType: TextShapeType,
    // Java field: float mWidth
    pub(crate) mWidth: f32,
    // Java field: float mHeight
    pub(crate) mHeight: f32,
}

// Java: public class Depth
pub struct Depth {
    // Java field: int mColor
    pub(crate) mColor: i32,
    // Java field: float mSize
    pub(crate) mSize: f32,
}

// Java: public class Contour
pub struct Contour {
    // Java field: int mColor
    pub(crate) mColor: i32,
    // Java field: float mSize
    pub(crate) mSize: f32,
}

// Java: public class Material
pub struct Material {
}

// Java: public class Lighting
pub struct Lighting {
}

// Java: public class TextShape3d
impl TextShape3d {
    // Java: TextShape3d()
    pub fn new() -> Self {
      // Java: super();
      TextShape3d {
          mTopShape: TextShape::new(),
          mBottomShape: TextShape::new(),
          mDepth: Depth::new(),
          mContour: Contour::new(),
      }
    }

}

// Java: public enum TextShapeType
impl TextShapeType {
    // Java: TextShapeType()
}

// Java: public class TextShape
impl TextShape {
    // Java: TextShape()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.mShapeType = TextShape3d.TextShapeType.NONE;
         // Java: this.mWidth = 0.0F;
         // Java: this.mHeight = 0.0F;
         TextShape { mShapeType: TextShapeType::NONE, mWidth: 0.0, mHeight: 0.0 }
    }

}

// Java: public class Depth
impl Depth {
    // Java: Depth()
    pub fn new() -> Self {
         // Java: super();
         Depth { mColor: 0, mSize: 0.0 }
    }

}

// Java: public class Contour
impl Contour {
    // Java: Contour()
    pub fn new() -> Self {
         // Java: super();
         Contour { mColor: 0, mSize: 0.0 }
    }

}

// Java: public class Material
impl Material {
    // Java: Material()
    pub fn new() -> Self {
         // Java: super();
         Material {}
    }

}

// Java: public class Lighting
impl Lighting {
    // Java: Lighting()
    pub fn new() -> Self {
         // Java: super();
         Lighting {}
    }

}

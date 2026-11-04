// Java: public enum WarpType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarpType {
    NONE,
    ARCH_DOWN_PATH,
    ARCH_UP_PATH,
    CIRCLE_PATH,
    BUTTON_PATH,
    SQUARE,
    STOP,
    TRIANGLE,
    TRIANGLE_INVERTED,
    VSHAPE,
    VSHAPE_INVERTED,
    CLOCKWISE,
    ANCLOCKWISE,
    ARCH_DOWN,
    ARCH_UP,
    CIRCLE,
    BUTTON,
    LEFT_OX_HORN,
    RIGHT_OX_HORN,
    TWOSIDE_NEAR,
    TWOSIDE_FAR,
    WAVE1,
    WAVE2,
    DOUBLE_WAVE1,
    DOUBLE_WAVE2,
    INFLATE,
    DEFLATE,
    INFLATE_BOTTOM,
    DEFLATE_BOTTOM,
    INFLATE_TOP,
    DEFLATE_TOP,
    SINGLE_SUBWAY,
    DOUBLE_SUBWAY,
    LEFT_NEAR_RIGHT_FAR,
    LEFT_FAR_RIGHT_NEAR,
    TRAPEZOID,
    TRAPEZOID_INVERTED,
    LEFT_COLLAR_BADGE,
    RIGHT_COLLAR_BADGE,
    FRONT_NEAR_BACK_FAR,
    FRONT_FAR_BACK_NEAR,
}

// Java: public class TextWarp
pub struct TextWarp {
    // Java field: WarpType mWarpType
    pub mWarpType: WarpType,
}

// Java: public class TextWarp
impl TextWarp {
    // Java: TextWarp()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mWarpType = TextWarp.WarpType.NONE;
      TextWarp { mWarpType: WarpType::NONE }
    }

}

// Java: public enum WarpType
impl WarpType {
    // Java: WarpType()
}

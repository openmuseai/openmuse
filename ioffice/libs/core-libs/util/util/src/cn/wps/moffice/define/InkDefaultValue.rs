#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::sync::OnceLock;

// Java field: int DEFAULT_INK_COLOR
pub(crate) const DEFAULT_INK_COLOR: i32 = -16777216;
// Java field: int DEFAULT_INK_HIGHTLIGHTER_COLOR
pub(crate) const DEFAULT_INK_HIGHTLIGHTER_COLOR: i32 = -256;
// Java field: int DEFAULT_INK_BLACK_COLOR
pub(crate) const DEFAULT_INK_BLACK_COLOR: i32 = -16777216;
// Java field: int DEFAULT_INK_RED_COLOR
pub(crate) const DEFAULT_INK_RED_COLOR: i32 = -65536;
// Java field: int DEFAULT_INK_BLUE_COLOR
pub(crate) const DEFAULT_INK_BLUE_COLOR: i32 = -16776961;
// Java field: int DEFAULT_INK_HIGHTLIGHTER_YELLOW_COLOR
pub(crate) const DEFAULT_INK_HIGHTLIGHTER_YELLOW_COLOR: i32 = -256;
// Java field: int DEFAULT_INK_HIGHTLIGHTER_GREEN_COLOR
pub(crate) const DEFAULT_INK_HIGHTLIGHTER_GREEN_COLOR: i32 = -10158235;
// Java field: int DEFAULT_INK_HIGHTLIGHTER_RED_COLOR
pub(crate) const DEFAULT_INK_HIGHTLIGHTER_RED_COLOR: i32 = -65281;
// Java field: float DEFAULT_INK_STROKE
pub(crate) const DEFAULT_INK_STROKE: f32 = 0.75;
// Java field: float DEFAULT_INK_COMMENT_STROKE
pub(crate) const DEFAULT_INK_COMMENT_STROKE: f32 = 0.0;
// Java field: float DEFAULT_INK_HIGHTLIGHTER_STROKE
pub(crate) const DEFAULT_INK_HIGHTLIGHTER_STROKE: f32 = 6.0;
// Java field: float DEFAULT_DASH_OVAL_STROKE
pub(crate) const DEFAULT_DASH_OVAL_STROKE: f32 = 1.0;
// Java field: float DEFAULT_DASH_OVAL_INTERVAL_SCALE
pub(crate) const DEFAULT_DASH_OVAL_INTERVAL_SCALE: f32 = 4.0;
// Java field: int DEFAULT_PLAY_INK_COLOR
pub(crate) const DEFAULT_PLAY_INK_COLOR: i32 = -65536;
// Java field: float DEFAULT_PLAY_INK_STROKE
pub(crate) const DEFAULT_PLAY_INK_STROKE: f32 = 2.25;
// Java field: float DEFAULT_PDF_INK_STROKE
pub(crate) const DEFAULT_PDF_INK_STROKE: f32 = 1.5;
// Java field: float DEFAULT_PDF_INK_STROKE_PAD
pub(crate) const DEFAULT_PDF_INK_STROKE_PAD: f32 = 0.75;
// Java field: int DEFAULT_PDF_INK_COLOR
pub(crate) const DEFAULT_PDF_INK_COLOR: i32 = -372121;
// Java field: float DEFAULT_PDF_INK_HIGHTLIGHTER_STROKE
pub(crate) const DEFAULT_PDF_INK_HIGHTLIGHTER_STROKE: f32 = 12.0;
// Java field: int DEFAULT_PDF_INK_HIGHTLIGHTER_COLOR
pub(crate) const DEFAULT_PDF_INK_HIGHTLIGHTER_COLOR: i32 = -256;
// Java field: float DEFAULT_OFD_INK_STROKE
pub(crate) const DEFAULT_OFD_INK_STROKE: f32 = 6.0;
// Java field: int DEFAULT_PDF_INK_ALPHA
pub(crate) const DEFAULT_PDF_INK_ALPHA: i32 = 255;
// Java field: int DEFAULT_PDF_INK_HIGH_LIGHT_ALPHA
pub(crate) const DEFAULT_PDF_INK_HIGH_LIGHT_ALPHA: i32 = 64;
// Java field: String TIP_INK_FIRST
pub(crate) const TIP_INK_FIRST: &str = "TIP_INK_FIRST";
// Java field: String TIP_INK_SECOND
pub(crate) const TIP_INK_SECOND: &str = "TIP_INK_SECOND";
// Java field: String TIP_INK_THIRD
pub(crate) const TIP_INK_THIRD: &str = "TIP_INK_THIRD";
// Java field: String TIP_PEN
pub(crate) const TIP_PEN: &str = "TIP_PEN";
// Java field: String TIP_HIGHLIGHTER
pub(crate) const TIP_HIGHLIGHTER: &str = "TIP_HIGHLIGHTER";
// Java field: String TIP_ERASER
pub(crate) const TIP_ERASER: &str = "TIP_ERASER";
// Java field: String TIP_WRITING
pub(crate) const TIP_WRITING: &str = "TIP_WRITING";
// Java field: String TIP_COVER_PEN
pub(crate) const TIP_COVER_PEN: &str = "TIP_COVER_PEN";
// Java field: int DEFAULT_PDF_PEN_MODE_PEN
pub(crate) const DEFAULT_PDF_PEN_MODE_PEN: i32 = 1;
// Java field: int[] ALL_PEN_COLOR
static ALL_PEN_COLOR: OnceLock<Vec<i32>> = OnceLock::new();
// Java field: float[] ALL_PEN_STROKE
static ALL_PEN_STROKE: OnceLock<Vec<f32>> = OnceLock::new();
// Java field: float[] INK_STROKE_POINTS
static INK_STROKE_POINTS: OnceLock<Vec<f32>> = OnceLock::new();
// Java field: float[] INK_STROKE_POINTS_IFLYTEK
static INK_STROKE_POINTS_IFLYTEK: OnceLock<Vec<f32>> = OnceLock::new();
// Java field: String INK_STYLUS_TOUCH_WINDOW
pub(crate) const INK_STYLUS_TOUCH_WINDOW: &str = "ink_stylus_touch_window";
// Java field: String INK_RULE_FINGER_AND_STYLUS_TOUCH
pub(crate) const INK_RULE_FINGER_AND_STYLUS_TOUCH: &str = "ink_rule_finger_and_stylus_touch";
// Java field: String INK_RULE_FINGER_AND_STYLUS_CLICK_SETTING
pub(crate) const INK_RULE_FINGER_AND_STYLUS_CLICK_SETTING: &str = "ink_rule_finger_and_stylus_click_setting";
// Java field: String INK_RULE_STYLUS
pub(crate) const INK_RULE_STYLUS: &str = "ink_rule_style";
// Java field: String INK_TAB_TAG
pub(crate) const INK_TAB_TAG: &str = "INK";
// Java field: String PUBLIC_INK_FUNCTION_GUIDE_KEY
pub(crate) const PUBLIC_INK_FUNCTION_GUIDE_KEY: &str = "_ink_function_guide";
// Java field: int PDF_PEN_IS_NOT_SETTING_PEN_AND_FINGER
pub(crate) const PDF_PEN_IS_NOT_SETTING_PEN_AND_FINGER: i32 = 0;
// Java field: int PDF_PEN_SETTING_PEN_AND_FINGER
pub(crate) const PDF_PEN_SETTING_PEN_AND_FINGER: i32 = 2;
// Java field: int PDF_PEN_SETTING_JUST_PEN
pub(crate) const PDF_PEN_SETTING_JUST_PEN: i32 = 1;
// Java field: int[] PDF_PEN_GESTURE_COLORS
static PDF_PEN_GESTURE_COLORS: OnceLock<Vec<i32>> = OnceLock::new();
// Java field: float MIN_STROKE
pub(crate) const MIN_STROKE: f32 = 0.4;
// Java field: float INK_HIT_DEVIATION
pub(crate) const INK_HIT_DEVIATION: f32 = 10.0;
// Java field: float POINT_2_CM
pub(crate) const POINT_2_CM: f32 = 26.458334;

// Java: public class InkDefaultValue
pub struct InkDefaultValue {
}

// Java: public class InkDefaultValue
impl InkDefaultValue {
    // Java: InkDefaultValue()
    pub fn new() -> Self {
        // Java: super();
        InkDefaultValue {}
    }

    pub fn get_all_pen_color() -> &'static Vec<i32> {
        ALL_PEN_COLOR.get_or_init(|| vec![-256, -10158235, -65281, -16777216, -65536, -16776961])
    }

    pub fn get_all_pen_stroke() -> &'static Vec<f32> {
        ALL_PEN_STROKE.get_or_init(|| vec![6.0, 6.0, 6.0, 0.75, 0.75, 0.75])
    }

    pub fn get_ink_stroke_points() -> &'static Vec<f32> {
        INK_STROKE_POINTS.get_or_init(|| vec![0.25, 0.5, 0.75, 1.0, 1.5, 2.25, 3.0, 4.5, 6.0])
    }

    pub fn get_ink_stroke_points_iflytek() -> &'static Vec<f32> {
        INK_STROKE_POINTS_IFLYTEK.get_or_init(|| vec![1.75, 2.9, 3.5, 4.8])
    }

    pub fn get_pdf_pen_gesture_colors() -> &'static Vec<i32> {
        PDF_PEN_GESTURE_COLORS.get_or_init(|| vec![
            -16777216, -16777088, -8388608, -16744448, -39424, -8355712,
            -16776961, -65536, -10092699, -26368, -4144960, -13395457,
            -2726555, -6697984, -13312, -1, -6697729, -6683, -3342644,
            -13159, -16742400, -8355840, -12763813, -13434778, -10092442,
            -13382452, -256, -10066279, -10092340, -3407770, -16711681,
            -103, -6710785, -6736897, -52327, -3342337, -52, -3355393,
            -3368449, -26164
        ])
    }
}

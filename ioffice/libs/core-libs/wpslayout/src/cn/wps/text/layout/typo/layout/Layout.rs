// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.Rect
// Java: import android.text.TextPaint
// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.text.layout.layouter.WpsLayout
// Java: import cn.wps.text.layout.typo.layout.style.LeadingMarginSpan
// Java: import cn.wps.text.layout.typo.layout.style.ParagraphStyle
// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
// Java: import java.util.Arrays
// Java: import java.util.List

// Java field: int BREAK_STRATEGY_SIMPLE
pub const BREAK_STRATEGY_SIMPLE: i32 = 0;
// Java field: int BREAK_STRATEGY_HIGH_QUALITY
pub const BREAK_STRATEGY_HIGH_QUALITY: i32 = 1;
// Java field: int BREAK_STRATEGY_BALANCED
pub const BREAK_STRATEGY_BALANCED: i32 = 2;
// Java field: int HYPHENATION_FREQUENCY_NONE
pub const HYPHENATION_FREQUENCY_NONE: i32 = 0;
// Java field: int HYPHENATION_FREQUENCY_NORMAL
pub const HYPHENATION_FREQUENCY_NORMAL: i32 = 1;
// Java field: int HYPHENATION_FREQUENCY_FULL
pub const HYPHENATION_FREQUENCY_FULL: i32 = 2;
// Java field: int JUSTIFICATION_MODE_NONE
pub const JUSTIFICATION_MODE_NONE: i32 = 0;
// Java field: int JUSTIFICATION_MODE_INTER_WORD
pub const JUSTIFICATION_MODE_INTER_WORD: i32 = 1;
// Java field: float DEFAULT_LINESPACING_MULTIPLIER
pub const DEFAULT_LINESPACING_MULTIPLIER: f32 = 1.0;
// Java field: float DEFAULT_LINESPACING_ADDITION
pub const DEFAULT_LINESPACING_ADDITION: f32 = 0.0;
// Java field: int DIR_LEFT_TO_RIGHT
pub const DIR_LEFT_TO_RIGHT: i32 = 1;
// Java field: int DIR_RIGHT_TO_LEFT
pub const DIR_RIGHT_TO_LEFT: i32 = -1;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    ALIGN_NORMAL,
    ALIGN_OPPOSITE,
    ALIGN_CENTER,
    ALIGN_LEFT,
    ALIGN_RIGHT,
}

// Java: public class Layout
pub struct Layout {
    // Java field: CharSequence mText
    pub(crate) mText: Option<String>,
    // Java field: int mWidth
    pub mWidth: i32,
    // Java field: float mOriWidth
    pub mOriWidth: f32,
    // Java field: int mAngleRectWidth
    pub mAngleRectWidth: i32,
    // Java field: Alignment mAlignment
    pub mAlignment: Alignment,
    // Java field: boolean mSpannedText
    pub(crate) mSpannedText: bool,
    // Java field: int mTextDir
    pub(crate) mTextDir: i32,
    // Java field: int mVertical
    pub mVertical: i32,
    // Java field: boolean mIsRightToLeft
    pub mIsRightToLeft: bool,
    // Java field: short mAngle
    pub mAngle: i16,
    // Java field: List<Float> mLineHeights
    pub mLineHeights: Option<Vec<f32>>,
    // Java field: List<Float> mLineWidths
    pub mLineWidths: Option<Vec<f32>>,
    // Java field: float sin
    pub sin: f32,
    // Java field: float cos
    pub cos: f32,
    // Java field: float tan
    pub tan: f32,
    // Java field: int mAngleRectHeight
    pub mAngleRectHeight: i32,
}

// Java: public class Layout
impl Layout {
    pub fn new() -> Self {
        Layout {
            mText: None,
            mWidth: 0,
            mOriWidth: 0.0,
            mAngleRectWidth: 0,
            mAlignment: Alignment::ALIGN_NORMAL,
            mSpannedText: false,
            mTextDir: 1,
            mVertical: 0,
            mIsRightToLeft: false,
            mAngle: 0,
            mLineHeights: None,
            mLineWidths: None,
            sin: 0.0,
            cos: 1.0,
            tan: 0.0,
            mAngleRectHeight: 0,
        }
    }

    pub fn getLineCount(&self) -> i32 { 0 }
    pub fn getLineLeft(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineRight(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineTop(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineBaseline(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineAscent(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineDescent(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineBottom(&self, _line: i32) -> f32 { 0.0 }
    pub fn getLineWidth(&self, _line: i32) -> f32 { 0.0 }
    pub fn getHeight(&self) -> f32 { 0.0 }
    pub fn getWidth(&self) -> i32 { self.mWidth }
    pub fn getLineEnd(&self, _line: i32) -> i32 { 0 }
    pub fn getLineStart(&self, _line: i32) -> i32 { 0 }
    pub fn getText(&self) -> &str { "" }
    pub fn getPrimaryHorizontal(&self, _offset: i32) -> f32 { 0.0 }
    pub fn getSecondaryHorizontal(&self, _offset: i32) -> f32 { 0.0 }
}

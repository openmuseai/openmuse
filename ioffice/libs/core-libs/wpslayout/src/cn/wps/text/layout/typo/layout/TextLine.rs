#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class TextLine
pub struct TextLine {
    // Java field: boolean DEBUG
    // Java field: TextLine[] sCached
    // Java field: int TAB_INCREMENT
    // Java field: TextPaint mPaint
    // Java field: CharSequence mText
    // Java field: int mStart
    // Java field: int mLen
    // Java field: int mDir
    // Java field: Layout.Directions mDirections
    // Java field: boolean mHasTabs
    // Java field: Layout.TabStops mTabs
    // Java field: char[] mChars
    // Java field: boolean mCharsValid
    // Java field: Spanned mSpanned
    // Java field: TextPaint mWorkPaint
    // Java field: TextPaint mActivePaint
    // Java field: SpanSet<MetricAffectingSpan> mMetricAffectingSpanSpanSet
    // Java field: SpanSet<CharacterStyle> mCharacterStyleSpanSet
    // Java field: SpanSet<ReplacementSpan> mReplacementSpanSpanSet
    _private: (),
}

pub const TEXT_LINE_DEBUG: bool = false;
pub const TAB_INCREMENT: i32 = 20;

// Java: public class TextLine
impl TextLine {
    // Java: TextLine()
    pub fn new() -> Self {
        // Java: super();
        TextLine { _private: () }
    }
}

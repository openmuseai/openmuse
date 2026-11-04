#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java field: String TAG
// Java field: int[][] sCachedIntBuffer
// Java field: int MARK
// Java field: int POINT
// Java field: int PARAGRAPH
// Java field: int START_MASK
// Java field: int END_MASK
// Java field: int START_SHIFT
// Java field: int SPAN_ADDED
// Java field: int SPAN_START_AT_START
// Java field: int SPAN_START_AT_END
// Java field: int SPAN_END_AT_START
// Java field: int SPAN_END_AT_END
// Java field: int SPAN_START_END_MASK

pub(crate) const TAG: &str = "SpannableStringBuilder";

// Java: public class SpannableStringBuilder implements CharSequence, GetChars, Spannable, Editable, Appendable
pub struct SpannableStringBuilder {
    // Java field: char[] mText
    pub(crate) mText: Vec<char>,
    // Java field: int mGapStart
    mGapStart: i32,
    // Java field: int mGapLength
    mGapLength: i32,
    // Java field: Object[] mSpans
    mSpans: Vec<i32>,
    // Java field: int[] mSpanStarts
    mSpanStarts: Vec<i32>,
    // Java field: int[] mSpanEnds
    mSpanEnds: Vec<i32>,
    // Java field: int[] mSpanMax
    mSpanMax: Vec<i32>,
    // Java field: int[] mSpanFlags
    mSpanFlags: Vec<i32>,
    // Java field: int[] mSpanOrder
    mSpanOrder: Vec<i32>,
    // Java field: int mSpanInsertCount
    mSpanInsertCount: i32,
    // Java field: int mSpanCount
    mSpanCount: i32,
    // Java field: IdentityHashMap<Object, Integer> mIndexOfSpan
    mIndexOfSpan: Option<i32>,
    // Java field: int mLowWaterMark
    mLowWaterMark: i32,
    // Java field: int mTextWatcherDepth
    mTextWatcherDepth: i32,
}

// Java: public class SpannableStringBuilder implements CharSequence, GetChars, Spannable, Editable, Appendable
impl SpannableStringBuilder {
    // Java: SpannableStringBuilder()
    pub fn new() -> Self {
        // Java: this("");
        Self::new_from_source(0)
    }

    // Java: SpannableStringBuilder(CharSequence text)
    pub fn new_from_source(_text: i32) -> Self {
        // Java: this(text, 0, text.length());
        SpannableStringBuilder {
            mText: Vec::new(),
            mGapStart: 0,
            mGapLength: 0,
            mSpans: Vec::new(),
            mSpanStarts: Vec::new(),
            mSpanEnds: Vec::new(),
            mSpanMax: Vec::new(),
            mSpanFlags: Vec::new(),
            mSpanOrder: Vec::new(),
            mSpanInsertCount: 0,
            mSpanCount: 0,
            mIndexOfSpan: None,
            mLowWaterMark: 0,
            mTextWatcherDepth: 0,
        }
    }

    // Java: SpannableStringBuilder(CharSequence text, int start, int end)
    pub fn new_range(_text: i32, _start: i32, _end: i32) -> Self {
        // Java: super();
        Self::new_from_source(_text)
    }

    // Java: static SpannableStringBuilder valueOf(CharSequence source)
    pub fn valueOf(_source: i32) -> Self {
        // Java: return source instanceof SpannableStringBuilder ? (SpannableStringBuilder)source : new SpannableStringBuilder(source);
        Self::new_from_source(_source)
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return this.mText.length - this.mGapLength;
        0
    }
}

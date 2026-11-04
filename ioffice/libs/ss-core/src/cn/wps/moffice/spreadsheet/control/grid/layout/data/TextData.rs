use super::{ParaInfo::ParaInfo, SpanInfo::SpanInfo};
// java public abstract class TextData
#[derive(Clone)]
pub struct TextData {
    pub mSourceText: Option<Vec<u16>>,
    pub mPara: ParaInfo,
    pub mFirstSpan: SpanInfo,
    pub mOtherSpans: Vec<SpanInfo>,
    pub mIsRichText: bool,
    mSpanIndex: usize,
}
impl Default for TextData {
    fn default() -> Self {
        Self {
            mSourceText: None,
            mPara: ParaInfo::default(),
            mFirstSpan: SpanInfo::default(),
            mOtherSpans: vec![],
            mIsRichText: true,
            mSpanIndex: 0,
        }
    }
}
impl TextData {
    // java public float italicFontSize() {
    // java float fontSize = this.mFirstSpan.mIsItalic ? this.mFirstSpan.mFontSize : 0.0F;
    // java int i = 0;
    // java
    // java for(int size = this.mOtherSpans.size(); i < size; ++i) {
    // java SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
    // java if (span.mIsItalic && fontSize < span.mFontSize) {
    // java fontSize = span.mFontSize;
    // java }
    // java }
    // java
    // java return fontSize;
    // java }
    pub fn italicFontSize(&self) -> f32 {
        let mut size = if self.mFirstSpan.mIsItalic {
            self.mFirstSpan.mFontSize
        } else {
            0.0
        };
        for span in &self.mOtherSpans {
            if span.mIsItalic && size < span.mFontSize {
                size = span.mFontSize;
            }
        }
        size
    }
    // java public void clear() {
    // java this.mSourceText = null;
    // java DataAllocator allocator = DataAllocator.getInstance();
    // java
    // java for(int i = this.mOtherSpans.size() - 1; i > -1; --i) {
    // java allocator.disposeSpan((SpanInfo)this.mOtherSpans.get(i));
    // java }
    // java
    // java this.mOtherSpans.clear();
    // java }
    pub fn clear(&mut self) {
        self.mSourceText = None;
        self.mOtherSpans.clear();
    }
    // java public SpanInfo getFirstSpan() {
    // java this.mSpanIndex = 0;
    // java return this.mFirstSpan;
    // java }
    pub fn getFirstSpan(&mut self) -> &SpanInfo {
        self.mSpanIndex = 0;
        &self.mFirstSpan
    }
    // java public SpanInfo getNextSpan() {
    // java return null != this.mOtherSpans && this.mSpanIndex < this.mOtherSpans.size() ? (SpanInfo)this.mOtherSpans.get(this.mSpanIndex++) : null;
    // java }
    pub fn getNextSpan(&mut self) -> Option<&SpanInfo> {
        if self.mSpanIndex < self.mOtherSpans.len() {
            let i = self.mSpanIndex;
            self.mSpanIndex += 1;
            Some(&self.mOtherSpans[i])
        } else {
            None
        }
    }
    // rust: Internal UTF-16 view of Java String; retains Java indexing.
    pub fn source(&self) -> &[u16] {
        self.mSourceText
            .as_ref()
            .expect("NullPointerException mSourceText")
    }
}

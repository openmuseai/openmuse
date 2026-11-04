use crate::cn::wps::text::layout::typo::layout::Spannable::Spannable;

// Java: public class SingleImageSpannable
pub struct SingleImageSpannable {
    // Java field: byte mType
    pub(crate) mType: i8,
    // Java field: Spannable mSpan
    pub(crate) mSpan: Box<dyn Spannable>,
}

// Java: public class SingleImageSpannable
impl SingleImageSpannable {
    // Java: SingleImageSpannable(byte type, CharSequence span)
    pub fn new(r#type: i8, span: Box<dyn Spannable>) -> Self {
      // Java: super();
      // Java: this.mType = type;
      // Java: this.mSpan = (Spannable)span;
      SingleImageSpannable { mType: r#type, mSpan: span }
    }

    // Java: byte getType()
    pub fn getType(&self) -> i8 {
      // Java: return this.mType;
      self.mType
    }

    // Java: Spannable getSpan()
    pub fn getSpan(&self) -> &dyn Spannable {
      // Java: return this.mSpan;
      &*self.mSpan
    }

    // Java: boolean isOuterShadow()
    pub fn isOuterShadow(&self) -> bool {
      // Java: return this.mType == 5;
      self.mType == 5
    }

}

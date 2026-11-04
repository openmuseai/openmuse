use super::ITextExtractor::ITextExtractor;

// Java: public class UnknownExtractor extends ITextExtractor
pub struct UnknownExtractor {
    pub(crate) base: ITextExtractor,
}

// Java: public class UnknownExtractor extends ITextExtractor
impl UnknownExtractor {
    // Java: UnknownExtractor()
    pub(crate) fn new() -> Self {
      // Java: super("", (String)null, (ITEListener)null);
      UnknownExtractor {
          base: ITextExtractor::new("", "", None),
      }
    }
}

use std::any::Any;
use super::UnsupportedVariantTypeException::UnsupportedVariantTypeException;

// Java: public class WritingNotSupportedException extends UnsupportedVariantTypeException
pub struct WritingNotSupportedException {
    pub(crate) base: UnsupportedVariantTypeException,
}

// Java: public class WritingNotSupportedException extends UnsupportedVariantTypeException
impl WritingNotSupportedException {
    // Java: WritingNotSupportedException(long variantType, Object value)
    pub fn new(variantType: i64, value: Box<dyn Any>) -> Self {
        // Java: super(variantType, value);
        WritingNotSupportedException {
            base: UnsupportedVariantTypeException::new(variantType, value),
        }
    }
}

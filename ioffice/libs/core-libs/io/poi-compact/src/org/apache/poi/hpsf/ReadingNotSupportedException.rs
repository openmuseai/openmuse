use std::any::Any;
use super::UnsupportedVariantTypeException::UnsupportedVariantTypeException;

// Java: public class ReadingNotSupportedException extends UnsupportedVariantTypeException
pub struct ReadingNotSupportedException {
    pub(crate) base: UnsupportedVariantTypeException,
}

// Java: public class ReadingNotSupportedException extends UnsupportedVariantTypeException
impl ReadingNotSupportedException {
    // Java: ReadingNotSupportedException(long variantType, Object value)
    pub fn new(variantType: i64, value: Box<dyn Any>) -> Self {
        // Java: super(variantType, value);
        ReadingNotSupportedException {
            base: UnsupportedVariantTypeException::new(variantType, value),
        }
    }
}

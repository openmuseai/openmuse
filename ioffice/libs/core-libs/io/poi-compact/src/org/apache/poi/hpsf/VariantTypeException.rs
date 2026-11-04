use std::any::Any;
use std::ops::Deref;
use super::HPSFException::HPSFException;

// Java: public class VariantTypeException extends HPSFException
pub struct VariantTypeException {
    // Java field: Object value
    pub(crate) value: Box<dyn Any>,
    // Java field: long variantType
    pub(crate) variantType: i64,
    parent: HPSFException,
}

impl Deref for VariantTypeException {
    type Target = HPSFException;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class VariantTypeException extends HPSFException
impl VariantTypeException {
    // Java: VariantTypeException(long variantType, Object value, String msg)
    pub fn new(variantType: i64, value: Box<dyn Any>, msg: String) -> Self {
        // Java: super(msg);
        // Java: this.variantType = variantType;
        // Java: this.value = value;
        VariantTypeException {
            value,
            variantType,
            parent: HPSFException::new_with_msg(&msg),
        }
    }

    // Java: long getVariantType()
    pub fn getVariantType(&self) -> i64 {
        // Java: return this.variantType;
        self.variantType
    }

    // Java: Object getValue()
    pub fn getValue(&self) -> &dyn Any {
        // Java: return this.value;
        &*self.value
    }
}

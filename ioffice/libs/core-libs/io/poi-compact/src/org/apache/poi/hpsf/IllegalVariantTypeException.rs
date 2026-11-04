// Java: import org.apache.poi.util.HexDump

use std::any::Any;
use std::ops::Deref;
use super::VariantTypeException::VariantTypeException;

fn to_hex(value: i64) -> String {
    format!("0x{:X}", value)
}

// Java: public class IllegalVariantTypeException extends VariantTypeException
pub struct IllegalVariantTypeException {
    pub(crate) base: VariantTypeException,
}

impl Deref for IllegalVariantTypeException {
    type Target = VariantTypeException;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class IllegalVariantTypeException extends VariantTypeException
impl IllegalVariantTypeException {
    // Java: IllegalVariantTypeException(long variantType, Object value, String msg)
    pub fn new(variantType: i64, value: Box<dyn Any>, msg: String) -> Self {
        // Java: super(variantType, value, msg);
        IllegalVariantTypeException {
            base: VariantTypeException::new(variantType, value, msg),
        }
    }

    // Java: IllegalVariantTypeException(long variantType, Object value)
    pub fn new_default(variantType: i64, value: Box<dyn Any>) -> Self {
        // Java: this(variantType, value, "The variant type " + variantType + " (" + Variant.getVariantName(variantType) + ", " + HexDump.toHex(variantType) + ") is illegal in this context.");
        let msg = format!(
            "The variant type {} ({}) is illegal in this context.",
            variantType,
            to_hex(variantType),
        );
        IllegalVariantTypeException {
            base: VariantTypeException::new(variantType, value, msg),
        }
    }
}

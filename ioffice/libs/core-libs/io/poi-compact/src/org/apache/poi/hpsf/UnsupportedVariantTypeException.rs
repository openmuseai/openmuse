// Java: import org.apache.poi.util.HexDump

use std::any::Any;
use super::VariantTypeException::VariantTypeException;

fn to_hex(value: i64) -> String {
    format!("0x{:X}", value)
}

// Java: public class UnsupportedVariantTypeException extends VariantTypeException
pub struct UnsupportedVariantTypeException {
    pub(crate) base: VariantTypeException,
}

// Java: public class UnsupportedVariantTypeException extends VariantTypeException
impl UnsupportedVariantTypeException {
    // Java: UnsupportedVariantTypeException(long variantType, Object value)
    pub fn new(variantType: i64, value: Box<dyn Any>) -> Self {
        // Java: super(variantType, value, "HPSF does not yet support the variant type " + variantType + " (" + Variant.getVariantName(variantType) + ", " + HexDump.toHex(variantType) + "). If you want support for this variant type in one of the next POI releases please submit a request for enhancement (RFE) to <http://issues.apache.org/bugzilla/>! Thank you!");
        let msg = format!(
            "HPSF does not yet support the variant type {} ({}) . If you want support for this variant type in one of the next POI releases please submit a request for enhancement (RFE) to <http://issues.apache.org/bugzilla/>! Thank you!",
            variantType,
            to_hex(variantType),
        );
        UnsupportedVariantTypeException {
            base: VariantTypeException::new(variantType, value, msg),
        }
    }
}

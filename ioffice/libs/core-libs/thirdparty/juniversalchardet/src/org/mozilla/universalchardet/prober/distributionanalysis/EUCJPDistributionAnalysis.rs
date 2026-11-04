use super::JISDistributionAnalysis::JISDistributionAnalysis;
use super::CharDistributionAnalysis::CharDistributionAnalysis;

// Java field: int HIGHBYTE_BEGIN
pub(crate) const HIGHBYTE_BEGIN: i32 = 161;
// Java field: int HIGHBYTE_END
pub(crate) const HIGHBYTE_END: i32 = 254;
// Java field: int LOWBYTE_BEGIN
pub(crate) const LOWBYTE_BEGIN: i32 = 161;
// Java field: int LOWBYTE_END
pub(crate) const LOWBYTE_END: i32 = 254;

// Java: public class EUCJPDistributionAnalysis extends JISDistributionAnalysis
pub struct EUCJPDistributionAnalysis;

pub(crate) fn get_order_eucjp(buf: &[u8], offset: usize) -> i32 {
    let highbyte = (buf[offset] & 0xFF) as i32;
    if highbyte >= 161 {
        let lowbyte = (buf[offset + 1] & 0xFF) as i32;
        94 * (highbyte - 161) + lowbyte - 161
    } else {
        -1
    }
}

// Java: public class EUCJPDistributionAnalysis extends JISDistributionAnalysis
impl EUCJPDistributionAnalysis {
    // Java: EUCJPDistributionAnalysis()
    pub fn new() -> CharDistributionAnalysis {
        // Java: super();
        JISDistributionAnalysis::new(get_order_eucjp)
    }

    // Java: int getOrder(byte[] buf, int offset)
    // Java: int highbyte = buf[offset] & 255;
    // Java: if (highbyte >= 161)
    // Java: int lowbyte = buf[offset + 1] & 255;
    // Java: return 94 * (highbyte - 161) + lowbyte - 161;
    // Java: else
    // Java: return -1;
}

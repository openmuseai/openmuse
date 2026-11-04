use super::JISDistributionAnalysis::JISDistributionAnalysis;
use super::CharDistributionAnalysis::CharDistributionAnalysis;

// Java field: int HIGHBYTE_BEGIN_1
pub(crate) const HIGHBYTE_BEGIN_1: i32 = 129;
// Java field: int HIGHBYTE_END_1
pub(crate) const HIGHBYTE_END_1: i32 = 159;
// Java field: int HIGHBYTE_BEGIN_2
pub(crate) const HIGHBYTE_BEGIN_2: i32 = 224;
// Java field: int HIGHBYTE_END_2
pub(crate) const HIGHBYTE_END_2: i32 = 239;
// Java field: int LOWBYTE_BEGIN_1
pub(crate) const LOWBYTE_BEGIN_1: i32 = 64;
// Java field: int LOWBYTE_BEGIN_2
pub(crate) const LOWBYTE_BEGIN_2: i32 = 128;

// Java: public class SJISDistributionAnalysis extends JISDistributionAnalysis
pub struct SJISDistributionAnalysis;

pub(crate) fn get_order_sjis(buf: &[u8], offset: usize) -> i32 {
    // Java: int order = -1;
    let mut order: i32;
    // Java: int highbyte = buf[offset] & 255;
    let highbyte = (buf[offset] & 0xFF) as i32;
    // Java: if (highbyte >= 129 && highbyte <= 159)
    if highbyte >= 129 && highbyte <= 159 {
        // Java: order = 188 * (highbyte - 129);
        order = 188 * (highbyte - 129);
    // Java: else
    } else {
        // Java: if (highbyte < 224 || highbyte > 239)
        if highbyte < 224 || highbyte > 239 {
            // Java: return -1;
            return -1;
        }
        // Java: order = 188 * (highbyte - 224 + 31);
        order = 188 * (highbyte - 224 + 31);
    }
    // Java: int lowbyte = buf[offset + 1] & 255;
    let lowbyte = (buf[offset + 1] & 0xFF) as i32;
    // Java: order += lowbyte - 64;
    order += lowbyte - 64;
    // Java: if (lowbyte >= 128)
    if lowbyte >= 128 {
        // Java: --order;
        order -= 1;
    }
    // Java: return order;
    order
}

// Java: public class SJISDistributionAnalysis extends JISDistributionAnalysis
impl SJISDistributionAnalysis {
    // Java: SJISDistributionAnalysis()
    pub fn new() -> CharDistributionAnalysis {
        // Java: super();
        JISDistributionAnalysis::new(get_order_sjis)
    }

    // Java: int getOrder(byte[] buf, int offset)
    // Java: int order = -1;
    // Java: int highbyte = buf[offset] & 255;
    // Java: if (highbyte >= 129 && highbyte <= 159)
    // Java: order = 188 * (highbyte - 129);
    // Java: else
    // Java: if (highbyte < 224 || highbyte > 239)
    // Java: return -1;
    // Java: order = 188 * (highbyte - 224 + 31);
    // Java: int lowbyte = buf[offset + 1] & 255;
    // Java: order += lowbyte - 64;
    // Java: if (lowbyte >= 128)
    // Java: --order;
    // Java: return order;
}

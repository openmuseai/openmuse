use super::JapaneseContextAnalysis::{JapaneseContextAnalysis, Order};

// Java field: int HIRAGANA_HIGHBYTE
pub(crate) const HIRAGANA_HIGHBYTE: i32 = 130;
// Java field: int HIRAGANA_LOWBYTE_BEGIN
pub(crate) const HIRAGANA_LOWBYTE_BEGIN: i32 = 159;
// Java field: int HIRAGANA_LOWBYTE_END
pub(crate) const HIRAGANA_LOWBYTE_END: i32 = 241;
// Java field: int HIGHBYTE_BEGIN_1
pub(crate) const HIGHBYTE_BEGIN_1: i32 = 129;
// Java field: int HIGHBYTE_END_1
pub(crate) const HIGHBYTE_END_1: i32 = 159;
// Java field: int HIGHBYTE_BEGIN_2
pub(crate) const HIGHBYTE_BEGIN_2: i32 = 224;
// Java field: int HIGHBYTE_END_2
pub(crate) const HIGHBYTE_END_2: i32 = 239;

// Java: public class SJISContextAnalysis extends JapaneseContextAnalysis
pub struct SJISContextAnalysis;

// Java: void getOrder(JapaneseContextAnalysis.Order order, byte[] buf, int offset)
pub(crate) fn get_order_sjis(order: &mut Order, buf: &[u8], offset: usize) {
    // Java: order.order = -1;
    order.order = -1;
    // Java: order.charLength = 1;
    order.char_length = 1;
    // Java: int highbyte = buf[offset] & 255;
    let highbyte = (buf[offset] & 0xFF) as i32;
    // Java: if (highbyte >= 129 && highbyte <= 159 || highbyte >= 224 && highbyte <= 239)
    if (highbyte >= HIGHBYTE_BEGIN_1 && highbyte <= HIGHBYTE_END_1) || (highbyte >= HIGHBYTE_BEGIN_2 && highbyte <= HIGHBYTE_END_2) {
        // Java: order.charLength = 2;
        order.char_length = 2;
    }
    // Java: if (highbyte == 130)
    if highbyte == HIRAGANA_HIGHBYTE {
        // Java: int lowbyte = buf[offset + 1] & 255;
        let lowbyte = (buf[offset + 1] & 0xFF) as i32;
        // Java: if (lowbyte >= 159 && lowbyte <= 241)
        if lowbyte >= HIRAGANA_LOWBYTE_BEGIN && lowbyte <= HIRAGANA_LOWBYTE_END {
            // Java: order.order = lowbyte - 159;
            order.order = lowbyte - HIRAGANA_LOWBYTE_BEGIN;
        }
    }
}

// Java: int getOrder(byte[] buf, int offset)
pub(crate) fn get_order_value_sjis(buf: &[u8], offset: usize) -> i32 {
    // Java: int highbyte = buf[offset] & 255;
    let highbyte = (buf[offset] & 0xFF) as i32;
    // Java: if (highbyte == 130)
    if highbyte == HIRAGANA_HIGHBYTE {
        // Java: int lowbyte = buf[offset + 1] & 255;
        let lowbyte = (buf[offset + 1] & 0xFF) as i32;
        // Java: if (lowbyte >= 159 && lowbyte <= 241)
        if lowbyte >= HIRAGANA_LOWBYTE_BEGIN && lowbyte <= HIRAGANA_LOWBYTE_END {
            // Java: return lowbyte - 159;
            return lowbyte - HIRAGANA_LOWBYTE_BEGIN;
        }
    }
    // Java: return -1;
    -1
}

// Java: public class SJISContextAnalysis extends JapaneseContextAnalysis
impl SJISContextAnalysis {
    // Java: SJISContextAnalysis()
    pub fn new() -> JapaneseContextAnalysis {
        // Java: super();
        JapaneseContextAnalysis::new(get_order_sjis, get_order_value_sjis)
    }

    // Java: void getOrder(JapaneseContextAnalysis.Order order, byte[] buf, int offset)
    // Java: order.order = -1;
    // Java: order.charLength = 1;
    // Java: int highbyte = buf[offset] & 255;
    // Java: if (highbyte >= 129 && highbyte <= 159 || highbyte >= 224 && highbyte <= 239)
    // Java: order.charLength = 2;
    // Java: if (highbyte == 130)
    // Java: int lowbyte = buf[offset + 1] & 255;
    // Java: if (lowbyte >= 159 && lowbyte <= 241)
    // Java: order.order = lowbyte - 159;

    // Java: int getOrder(byte[] buf, int offset)
    // Java: int highbyte = buf[offset] & 255;
    // Java: if (highbyte == 130)
    // Java: int lowbyte = buf[offset + 1] & 255;
    // Java: if (lowbyte >= 159 && lowbyte <= 241)
    // Java: return lowbyte - 159;
    // Java: return -1;
}

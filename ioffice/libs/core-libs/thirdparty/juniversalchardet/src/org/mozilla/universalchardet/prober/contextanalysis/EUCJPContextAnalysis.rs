use super::JapaneseContextAnalysis::{JapaneseContextAnalysis, Order};

// Java field: int HIRAGANA_HIGHBYTE
pub(crate) const HIRAGANA_HIGHBYTE: i32 = 164;
// Java field: int HIRAGANA_LOWBYTE_BEGIN
pub(crate) const HIRAGANA_LOWBYTE_BEGIN: i32 = 161;
// Java field: int HIRAGANA_LOWBYTE_END
pub(crate) const HIRAGANA_LOWBYTE_END: i32 = 243;
// Java field: int SINGLE_SHIFT_2
pub(crate) const SINGLE_SHIFT_2: i32 = 142;
// Java field: int SINGLE_SHIFT_3
pub(crate) const SINGLE_SHIFT_3: i32 = 143;
// Java field: int FIRSTPLANE_HIGHBYTE_BEGIN
pub(crate) const FIRSTPLANE_HIGHBYTE_BEGIN: i32 = 161;
// Java field: int FIRSTPLANE_HIGHBYTE_END
pub(crate) const FIRSTPLANE_HIGHBYTE_END: i32 = 254;

// Java: public class EUCJPContextAnalysis extends JapaneseContextAnalysis
pub struct EUCJPContextAnalysis;

// Java: void getOrder(JapaneseContextAnalysis.Order order, byte[] buf, int offset)
pub(crate) fn get_order_eucjp(order: &mut Order, buf: &[u8], offset: usize) {
    // Java: order.order = -1;
    order.order = -1;
    // Java: order.charLength = 1;
    order.char_length = 1;
    // Java: int firstByte = buf[offset] & 255;
    let first_byte = (buf[offset] & 0xFF) as i32;
    // Java: if (firstByte != 142 && (firstByte < 161 || firstByte > 254))
    if first_byte != SINGLE_SHIFT_2 && (first_byte < FIRSTPLANE_HIGHBYTE_BEGIN || first_byte > FIRSTPLANE_HIGHBYTE_END) {
        // Java: if (firstByte == 143)
        if first_byte == SINGLE_SHIFT_3 {
            // Java: order.charLength = 3;
            order.char_length = 3;
        }
    // Java: else
    } else {
        // Java: order.charLength = 2;
        order.char_length = 2;
    }
    // Java: if (firstByte == 164)
    if first_byte == HIRAGANA_HIGHBYTE {
        // Java: int secondByte = buf[offset + 1] & 255;
        let second_byte = (buf[offset + 1] & 0xFF) as i32;
        // Java: if (secondByte >= 161 && secondByte <= 243)
        if second_byte >= HIRAGANA_LOWBYTE_BEGIN && second_byte <= HIRAGANA_LOWBYTE_END {
            // Java: order.order = secondByte - 161;
            order.order = second_byte - HIRAGANA_LOWBYTE_BEGIN;
        }
    }
}

// Java: int getOrder(byte[] buf, int offset)
pub(crate) fn get_order_value_eucjp(buf: &[u8], offset: usize) -> i32 {
    // Java: int highbyte = buf[offset] & 255;
    let highbyte = (buf[offset] & 0xFF) as i32;
    // Java: if (highbyte == 164)
    if highbyte == HIRAGANA_HIGHBYTE {
        // Java: int lowbyte = buf[offset + 1] & 255;
        let lowbyte = (buf[offset + 1] & 0xFF) as i32;
        // Java: if (lowbyte >= 161 && lowbyte <= 243)
        if lowbyte >= HIRAGANA_LOWBYTE_BEGIN && lowbyte <= HIRAGANA_LOWBYTE_END {
            // Java: return lowbyte - 161;
            return lowbyte - HIRAGANA_LOWBYTE_BEGIN;
        }
    }
    // Java: return -1;
    -1
}

// Java: public class EUCJPContextAnalysis extends JapaneseContextAnalysis
impl EUCJPContextAnalysis {
    // Java: EUCJPContextAnalysis()
    pub fn new() -> JapaneseContextAnalysis {
        // Java: super();
        JapaneseContextAnalysis::new(get_order_eucjp, get_order_value_eucjp)
    }

    // Java: void getOrder(JapaneseContextAnalysis.Order order, byte[] buf, int offset)
    // Java: order.order = -1;
    // Java: order.charLength = 1;
    // Java: int firstByte = buf[offset] & 255;
    // Java: if (firstByte != 142 && (firstByte < 161 || firstByte > 254))
    // Java: if (firstByte == 143)
    // Java: order.charLength = 3;
    // Java: else
    // Java: order.charLength = 2;
    // Java: if (firstByte == 164)
    // Java: int secondByte = buf[offset + 1] & 255;
    // Java: if (secondByte >= 161 && secondByte <= 243)
    // Java: order.order = secondByte - 161;

    // Java: int getOrder(byte[] buf, int offset)
    // Java: int highbyte = buf[offset] & 255;
    // Java: if (highbyte == 164)
    // Java: int lowbyte = buf[offset + 1] & 255;
    // Java: if (lowbyte >= 161 && lowbyte <= 243)
    // Java: return lowbyte - 161;
    // Java: return -1;
}

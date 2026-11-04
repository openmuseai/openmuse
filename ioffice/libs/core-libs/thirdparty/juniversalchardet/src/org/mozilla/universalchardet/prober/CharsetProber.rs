// Java: import java.nio.ByteBuffer

// Java field: float SHORTCUT_THRESHOLD
pub(crate) const SHORTCUT_THRESHOLD: f32 = 0.95;
// Java field: int ASCII_A
pub(crate) const ASCII_A: i32 = 97;
// Java field: int ASCII_Z
pub(crate) const ASCII_Z: i32 = 122;
// Java field: int ASCII_A_CAPITAL
pub(crate) const ASCII_A_CAPITAL: i32 = 65;
// Java field: int ASCII_Z_CAPITAL
pub(crate) const ASCII_Z_CAPITAL: i32 = 90;
// Java field: int ASCII_LT
pub(crate) const ASCII_LT: i32 = 60;
// Java field: int ASCII_GT
pub(crate) const ASCII_GT: i32 = 62;
// Java field: int ASCII_SP
pub(crate) const ASCII_SP: i32 = 32;

// Java: public enum ProbingState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbingState {
    Detecting,
    FoundIt,
    NotMe,
}

// Java: public class CharsetProber
pub trait CharsetProber {
    // Java: CharsetProber()
    // Java: super();

    // Java: abstract String getCharSetName()
    fn get_charset_name(&self) -> &str;

    // Java: abstract ProbingState handleData(byte[] var1, int var2, int var3)
    fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState;

    // Java: abstract ProbingState getState()
    fn get_state(&self) -> ProbingState;

    // Java: abstract void reset()
    fn reset(&mut self);

    // Java: abstract float getConfidence()
    fn get_confidence(&self) -> f32;

    // Java: abstract void setOption()
    fn set_option(&mut self);
}

// Java: public class CharsetProber
// Java: ByteBuffer filterWithoutEnglishLetters(byte[] buf, int offset, int length)
pub fn filter_without_english_letters(buf: &[u8], offset: usize, length: usize) -> Vec<u8> {
    // Java: ByteBuffer out = ByteBuffer.allocate(length);
    let mut out = Vec::with_capacity(length);
    // Java: boolean meetMSB = false;
    let mut meet_msb = false;
    // Java: int prevPtr = offset;
    let mut prev_ptr = offset;
    // Java: int curPtr = offset;
    let mut cur_ptr = offset;
    let max_ptr = offset + length;
    // Java: for(int maxPtr = offset + length; curPtr < maxPtr; ++curPtr)
    while cur_ptr < max_ptr {
        // Java: byte c = buf[curPtr];
        let c = buf[cur_ptr];
        // Java: if (!this.isAscii(c))
        if !is_ascii(c) {
            // Java: meetMSB = true;
            meet_msb = true;
        }
        // Java: else if (this.isAsciiSymbol(c))
        else if is_ascii_symbol(c) {
            // Java: if (meetMSB && curPtr > prevPtr)
            if meet_msb && cur_ptr > prev_ptr {
                // Java: out.put(buf, prevPtr, curPtr - prevPtr);
                out.extend_from_slice(&buf[prev_ptr..cur_ptr]);
                // Java: out.put((byte)32);
                out.push(32);
                // Java: prevPtr = curPtr + 1;
                prev_ptr = cur_ptr + 1;
                // Java: meetMSB = false;
                meet_msb = false;
            }
            // Java: else
            else {
                // Java: prevPtr = curPtr + 1;
                prev_ptr = cur_ptr + 1;
            }
        }
        cur_ptr += 1;
    }
    // Java: if (meetMSB && curPtr > prevPtr)
    if meet_msb && cur_ptr > prev_ptr {
        // Java: out.put(buf, prevPtr, curPtr - prevPtr);
        out.extend_from_slice(&buf[prev_ptr..cur_ptr]);
    }
    // Java: return out;
    out
}

// Java: ByteBuffer filterWithEnglishLetters(byte[] buf, int offset, int length)
pub fn filter_with_english_letters(buf: &[u8], offset: usize, length: usize) -> Vec<u8> {
    // Java: ByteBuffer out = ByteBuffer.allocate(length);
    let mut out = Vec::with_capacity(length);
    // Java: boolean isInTag = false;
    let mut is_in_tag = false;
    // Java: int prevPtr = offset;
    let mut prev_ptr = offset;
    // Java: int curPtr = offset;
    let mut cur_ptr = offset;
    let max_ptr = offset + length;
    // Java: for(int maxPtr = offset + length; curPtr < maxPtr; ++curPtr)
    while cur_ptr < max_ptr {
        // Java: byte c = buf[curPtr];
        let c = buf[cur_ptr];
        // Java: if (c == 62)
        if c == 62 {
            // Java: isInTag = false;
            is_in_tag = false;
        }
        // Java: else if (c == 60)
        else if c == 60 {
            // Java: isInTag = true;
            is_in_tag = true;
        }
        // Java: if (this.isAscii(c) && this.isAsciiSymbol(c))
        if is_ascii(c) && is_ascii_symbol(c) {
            // Java: if (curPtr > prevPtr && !isInTag)
            if cur_ptr > prev_ptr && !is_in_tag {
                // Java: out.put(buf, prevPtr, curPtr - prevPtr);
                out.extend_from_slice(&buf[prev_ptr..cur_ptr]);
                // Java: out.put((byte)32);
                out.push(32);
                // Java: prevPtr = curPtr + 1;
                prev_ptr = cur_ptr + 1;
            }
            // Java: else
            else {
                // Java: prevPtr = curPtr + 1;
                prev_ptr = cur_ptr + 1;
            }
        }
        cur_ptr += 1;
    }
    // Java: if (!isInTag && curPtr > prevPtr)
    if !is_in_tag && cur_ptr > prev_ptr {
        // Java: out.put(buf, prevPtr, curPtr - prevPtr);
        out.extend_from_slice(&buf[prev_ptr..cur_ptr]);
    }
    // Java: return out;
    out
}

// Java: boolean isAscii(byte b)
fn is_ascii(b: u8) -> bool {
    // Java: return (b & 128) == 0;
    (b & 128) == 0
}

// Java: boolean isAsciiSymbol(byte b)
fn is_ascii_symbol(b: u8) -> bool {
    // Java: int c = b & 255;
    let c = b as i32 & 255;
    // Java: return c < 65 || c > 90 && c < 97 || c > 122;
    c < 65 || (c > 90 && c < 97) || c > 122
}

// Java: public enum ProbingState
impl ProbingState {
    // Java: ProbingState()
    pub fn new() -> Self {
        ProbingState::Detecting
    }
}

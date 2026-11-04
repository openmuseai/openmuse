// Java field: int TYPE_UNKNOWN
pub const TYPE_UNKNOWN: i32 = -1;
// Java field: int TYPE_LABEL
pub const TYPE_LABEL: i32 = 0;
// Java field: int TYPE_LABEL_ATTR
pub const TYPE_LABEL_ATTR: i32 = 1;
// Java field: int TYPE_LABEL_E
pub const TYPE_LABEL_E: i32 = 2;
// Java field: int TYPE_LABEL_ATTR_E
pub const TYPE_LABEL_ATTR_E: i32 = 3;
// Java field: int TYPE_E_LABEL
pub const TYPE_E_LABEL: i32 = 4;

// Java: public class HLElementType
pub struct HLElementType;

// Java: public class HLElementType
impl HLElementType {
    // Java: HLElementType()
    pub fn new() -> Self {
        // Java: super();
        HLElementType
    }

    // Java: static int getType(char[] chars, int start, int end)
    pub fn get_type_chars(chars: &[char], start: usize, end: usize) -> i32 {
        // Java: if (chars[start] == '<' && chars[end] == '>' && chars[start + 1] != '!' && chars[start + 1] != '?')
        if chars[start] == '<' && chars[end] == '>' && chars[start + 1] != '!' && chars[start + 1] != '?' {
            // Java: if (chars[end - 1] != '/')
            if chars[end - 1] != '/' {
                // Java: if (chars[start + 1] == '/')
                if chars[start + 1] == '/' {
                    // Java: return 4;
                    return 4;
                }
                // Java: else
                else {
                    // Java: return hasEmptySpace(chars, start + 1, end - 1) ? 1 : 0;
                    return if Self::has_empty_space_chars(chars, start + 1, end - 1) { 1 } else { 0 };
                }
            }
            // Java: else
            else {
                // Java: return hasEmptySpace(chars, start + 1, end - 1) ? 3 : 2;
                return if Self::has_empty_space_chars(chars, start + 1, end - 1) { 3 } else { 2 };
            }
        }
        // Java: else
        // Java: return -1;
        -1
    }

    // Java: static boolean hasEmptySpace(char[] chars, int start, int end)
    fn has_empty_space_chars(chars: &[char], start: usize, end: usize) -> bool {
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: if (HLLabelParser.isEmptyChar(chars[index]))
            if chars[index] == ' ' || chars[index] == '\t' || chars[index] == '\n' || chars[index] == '\r' {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static int getType(byte[] chars, int start, int end)
    pub fn get_type_bytes(chars: &[u8], start: usize, end: usize) -> i32 {
        // Java: if (chars[start] == 60 && chars[end] == 62 && chars[start + 1] != 33 && chars[start + 1] != 63)
        if chars[start] == 60 && chars[end] == 62 && chars[start + 1] != 33 && chars[start + 1] != 63 {
            // Java: if (chars[end - 1] != 47)
            if chars[end - 1] != 47 {
                // Java: if (chars[start + 1] == 47)
                if chars[start + 1] == 47 {
                    // Java: return 4;
                    return 4;
                }
                // Java: else
                else {
                    // Java: return hasEmptySpace(chars, start + 1, end - 1) ? 1 : 0;
                    return if Self::has_empty_space_bytes(chars, start + 1, end - 1) { 1 } else { 0 };
                }
            }
            // Java: else
            else {
                // Java: return hasEmptySpace(chars, start + 1, end - 1) ? 3 : 2;
                return if Self::has_empty_space_bytes(chars, start + 1, end - 1) { 3 } else { 2 };
            }
        }
        // Java: else
        // Java: return -1;
        -1
    }

    // Java: static boolean hasEmptySpace(byte[] chars, int start, int end)
    fn has_empty_space_bytes(chars: &[u8], start: usize, end: usize) -> bool {
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: if (chars[index] == 32)
            if chars[index] == 32 {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }
}

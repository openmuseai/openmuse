#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java field: _t_char_spans: &[char]
// Java field: _s_char_spans: &[char]

// Java: public class CnSTDicInfo
pub struct CnSTDicInfo;

// Java: public class CnSTDicInfo
impl CnSTDicInfo {
    // Java: CnSTDicInfo()
    // Java: super();

    // Java: static boolean searchDic(_spans: &[char], char c)
    fn searchDic(spans: &[char], c: char) -> bool {
        // Java: int start = 0;
        let mut start: usize = 0;
        // Java: int end = spans.length;
        let mut end = spans.len();
        // Java: while(start < end)
        while start < end {
            // Java: int mid = (start + end) / 2;
            let mid = (start + end) / 2;
            // Java: char midc = spans[mid];
            let midc = spans[mid];
            // Java: if (c < midc)
            if c < midc {
                // Java: end = mid;
                end = mid;
            }
            // Java: else
            else {
                // Java: if (c <= midc)
                if c <= midc {
                    // Java: return true;
                    return true;
                }
                // Java: start = mid + 1;
                start = mid + 1;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean isCharInTDic(char c)
    pub fn isCharInTDic(_c: char) -> bool {
        // Java: return searchDic(t_char_spans, c);
        false
    }

    // Java: static boolean isCharInSDic(char c)
    pub fn isCharInSDic(_c: char) -> bool {
        // Java: return searchDic(s_char_spans, c);
        false
    }

    // Java: static int countTChars(String str)
    pub fn countTChars_str(str: &str) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: int i = 0;
        // Java: for(int len = str.length(); i < len; ++i)
        for ch in str.chars() {
            // Java: if (isCharInTDic(str.charAt(i)))
            if Self::isCharInTDic(ch) {
                // Java: ++count;
                count += 1;
            }
        }
        // Java: return count;
        count
    }

    // Java: static int countSChars(String str)
    pub fn countSChars_str(str: &str) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: int i = 0;
        // Java: for(int len = str.length(); i < len; ++i)
        for ch in str.chars() {
            // Java: if (isCharInSDic(str.charAt(i)))
            if Self::isCharInSDic(ch) {
                // Java: ++count;
                count += 1;
            }
        }
        // Java: return count;
        count
    }

    // Java: static int countTChars(_str: &[char])
    pub fn countTChars_arr(str: &[char]) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: int i = 0;
        // Java: for(int len = str.length; i < len; ++i)
        for &ch in str.iter() {
            // Java: if (isCharInTDic(str[i]))
            if Self::isCharInTDic(ch) {
                // Java: ++count;
                count += 1;
            }
        }
        // Java: return count;
        count
    }

    // Java: static int countSChars(_str: &[char])
    pub fn countSChars_arr(str: &[char]) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: int i = 0;
        // Java: for(int len = str.length; i < len; ++i)
        for &ch in str.iter() {
            // Java: if (isCharInSDic(str[i]))
            if Self::isCharInSDic(ch) {
                // Java: ++count;
                count += 1;
            }
        }
        // Java: return count;
        count
    }
}

// Java field: char[] intToBase64
const INT_TO_BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
// Java field: char[] intToBase64WebSafe
const INT_TO_BASE64_WEB_SAFE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
// Java field: char[] intToAltBase64
const INT_TO_ALT_BASE64: &[u8] = b"!\"#$%&'(),-.:<>@[]^`_{|}~abcdefghijklmnopqrstuvwxyz0123456789+?";
// Java field: byte[] base64ToInt
const BASE64_TO_INT: &[i8] = &[
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,62,-1,-1,-1,63,52,53,54,55,56,57,58,59,60,61,-1,-1,-1,-1,-1,-1,
    -1,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
    -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51
];
// Java field: byte[] altBase64ToInt
const ALT_BASE64_TO_INT: &[i8] = &[
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,0,1,2,3,4,5,6,7,8,-1,62,9,10,11,-1,52,53,54,55,56,57,58,59,60,61,12,13,14,-1,15,63,
    16,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,17,-1,18,19,21,
    20,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,22,23,24,25
];

// Java: public class Base64
pub struct Base64 {
}

// Java: public class Base64
impl Base64 {
    // Java: Base64()
    pub fn new() -> Self {
        // Java: super();
        Base64 {}
    }

    // Java: static String byteArrayToBase64(byte[] a)
    pub fn byteArrayToBase64_simple(a: &[u8]) -> String {
        // Java: return byteArrayToBase64(a, false);
        Self::byteArrayToBase64(a, false)
    }

    // Java: static String byteArrayToBase64WebSafe(byte[] a)
    pub fn byteArrayToBase64WebSafe(a: &[u8]) -> String {
        // Java: return byteArrayToBase64(a, true);
        Self::byteArrayToBase64(a, true)
    }

    // Java: static byte[] base64ToByteArray(String s)
    pub fn base64ToByteArray_simple(s: &str) -> Vec<u8> {
        // Java: return base64ToByteArray(s, false);
        Self::base64ToByteArray(s, false)
    }

    // Java: static byte[] altBase64ToByteArray(String s)
    pub fn altBase64ToByteArray(s: &str) -> Vec<u8> {
        // Java: return base64ToByteArray(s, true);
        Self::base64ToByteArray(s, true)
    }

    // Java: static String byteArrayToBase64(byte[] a, boolean webSafe)
    fn byteArrayToBase64(a: &[u8], webSafe: bool) -> String {
        // Java: int aLen = a.length;
        let aLen = a.len();
        // Java: int numFullGroups = aLen / 3;
        let numFullGroups = aLen / 3;
        // Java: int numBytesInPartialGroup = aLen - 3 * numFullGroups;
        let numBytesInPartialGroup = aLen - 3 * numFullGroups;
        // Java: int resultLen = 4 * ((aLen + 2) / 3);
        let resultLen = 4 * ((aLen + 2) / 3);
        // Java: StringBuffer result = new StringBuffer(resultLen);
        let mut result = String::with_capacity(resultLen);
        // Java: char[] intToAlpha = webSafe ? intToBase64WebSafe : intToBase64;
        let intToAlpha = if webSafe { INT_TO_BASE64_WEB_SAFE } else { INT_TO_BASE64 };
        // Java: int inCursor = 0;
        let mut inCursor: usize = 0;
        // Java: for(int i = 0; i < numFullGroups; ++i)
        for _i in 0..numFullGroups {
            // Java: int byte0 = a[inCursor++] & 255;
            let byte0 = a[inCursor] as usize; inCursor += 1;
            // Java: int byte1 = a[inCursor++] & 255;
            let byte1 = a[inCursor] as usize; inCursor += 1;
            // Java: int byte2 = a[inCursor++] & 255;
            let byte2 = a[inCursor] as usize; inCursor += 1;
            // Java: result.append(intToAlpha[byte0 >> 2]);
            result.push(intToAlpha[byte0 >> 2] as char);
            // Java: result.append(intToAlpha[byte0 << 4 & 63 | byte1 >> 4]);
            result.push(intToAlpha[(byte0 << 4) & 63 | byte1 >> 4] as char);
            // Java: result.append(intToAlpha[byte1 << 2 & 63 | byte2 >> 6]);
            result.push(intToAlpha[(byte1 << 2) & 63 | byte2 >> 6] as char);
            // Java: result.append(intToAlpha[byte2 & 63]);
            result.push(intToAlpha[byte2 & 63] as char);
        }
        // Java: if (numBytesInPartialGroup != 0)
        if numBytesInPartialGroup != 0 {
            // Java: int byte0 = a[inCursor++] & 255;
            let byte0 = a[inCursor] as usize; inCursor += 1;
            // Java: result.append(intToAlpha[byte0 >> 2]);
            result.push(intToAlpha[byte0 >> 2] as char);
            // Java: if (numBytesInPartialGroup == 1)
            if numBytesInPartialGroup == 1 {
                // Java: result.append(intToAlpha[byte0 << 4 & 63]);
                result.push(intToAlpha[(byte0 << 4) & 63] as char);
                // Java: result.append("==");
                result.push_str("==");
            }
            // Java: else
            else {
                // Java: int byte1 = a[inCursor++] & 255;
                let byte1 = a[inCursor] as usize; inCursor += 1;
                // Java: result.append(intToAlpha[byte0 << 4 & 63 | byte1 >> 4]);
                result.push(intToAlpha[(byte0 << 4) & 63 | byte1 >> 4] as char);
                // Java: result.append(intToAlpha[byte1 << 2 & 63]);
                result.push(intToAlpha[(byte1 << 2) & 63] as char);
                // Java: result.append('=');
                result.push('=');
            }
        }
        // Java: return result.toString();
        result
    }

    // Java: static byte[] base64ToByteArray(String s, boolean alternate)
    fn base64ToByteArray(s: &str, alternate: bool) -> Vec<u8> {
        // Java: byte[] alphaToInt = alternate ? altBase64ToInt : base64ToInt;
        let alphaToInt = if alternate { ALT_BASE64_TO_INT } else { BASE64_TO_INT };
        // Java: int sLen = s.length();
        let bytes=s.encode_utf16().collect::<Vec<_>>();
        let sLen = bytes.len();
        // Java: int numGroups = sLen / 4;
        let numGroups = sLen / 4;
        // Java: if (4 * numGroups != sLen)
        if 4 * numGroups != sLen {
            // Java: throw new IllegalArgumentException("String length must be a multiple of four.");
            panic!("IllegalArgumentException: String length must be a multiple of four.");
        }
        // Java: else
        else {
            // Java: int missingBytesInLastGroup = 0;
            let mut missingBytesInLastGroup = 0usize;
            // Java: int numFullGroups = numGroups;
            let mut numFullGroups = numGroups;
            // Java: if (sLen != 0)
            if sLen != 0 {
                // Java: if (s.charAt(sLen - 1) == '=')
                if bytes[sLen - 1] == b'=' as u16 {
                    // Java: ++missingBytesInLastGroup;
                    missingBytesInLastGroup += 1;
                    // Java: numFullGroups = numGroups - 1;
                    numFullGroups = numGroups - 1;
                }
                // Java: if (s.charAt(sLen - 2) == '=')
                if bytes[sLen - 2] == b'=' as u16 {
                    // Java: ++missingBytesInLastGroup;
                    missingBytesInLastGroup += 1;
                }
            }
            // Java: byte[] result = new byte[3 * numGroups - missingBytesInLastGroup];
            let mut result = vec![0u8; 3 * numGroups - missingBytesInLastGroup];
            // Java: int inCursor = 0;
            let mut inCursor: usize = 0;
            // Java: int outCursor = 0;
            let mut outCursor: usize = 0;
            // Java: for(int i = 0; i < numFullGroups; ++i)
            for _i in 0..numFullGroups {
                // Java: int ch0 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch0 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: int ch1 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch1 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: int ch2 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch2 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: int ch3 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch3 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: result[outCursor++] = (byte)(ch0 << 2 | ch1 >> 4);
                *result.get_mut(outCursor).expect("ArrayIndexOutOfBoundsException") = ((ch0 << 2) | (ch1 >> 4)) as u8; outCursor += 1;
                // Java: result[outCursor++] = (byte)(ch1 << 4 | ch2 >> 2);
                *result.get_mut(outCursor).expect("ArrayIndexOutOfBoundsException") = ((ch1 << 4) | (ch2 >> 2)) as u8; outCursor += 1;
                // Java: result[outCursor++] = (byte)(ch2 << 6 | ch3);
                *result.get_mut(outCursor).expect("ArrayIndexOutOfBoundsException") = ((ch2 << 6) | ch3) as u8; outCursor += 1;
            }
            // Java: if (missingBytesInLastGroup != 0)
            if missingBytesInLastGroup != 0 {
                // Java: int ch0 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch0 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: int ch1 = base64toInt(s.charAt(inCursor++), alphaToInt);
                let ch1 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                // Java: result[outCursor++] = (byte)(ch0 << 2 | ch1 >> 4);
                *result.get_mut(outCursor).expect("ArrayIndexOutOfBoundsException") = ((ch0 << 2) | (ch1 >> 4)) as u8; outCursor += 1;
                // Java: if (missingBytesInLastGroup == 1)
                if missingBytesInLastGroup == 1 {
                    // Java: int ch2 = base64toInt(s.charAt(inCursor++), alphaToInt);
                    let ch2 = Self::base64toInt(bytes[inCursor], alphaToInt); inCursor += 1;
                    // Java: result[outCursor++] = (byte)(ch1 << 4 | ch2 >> 2);
                    *result.get_mut(outCursor).expect("ArrayIndexOutOfBoundsException") = ((ch1 << 4) | (ch2 >> 2)) as u8; outCursor += 1;
                }
            }
            // Java: return result;
            result
        }
    }

    // Java: static int base64toInt(char c, byte[] alphaToInt)
    fn base64toInt(c: u16, alphaToInt: &[i8]) -> i32 {
        // Java: int result = alphaToInt[c];
        let result = *alphaToInt.get(c as usize).expect("ArrayIndexOutOfBoundsException") as i32;
        // Java: if (result < 0)
        if result < 0 {
            // Java: throw new IllegalArgumentException("Illegal character " + c);
            panic!("IllegalArgumentException: Illegal character {}", char::from_u32(c as u32).unwrap_or('\u{fffd}'));
        }
        // Java: else
        else {
            // Java: return result;
            result
        }
    }
}

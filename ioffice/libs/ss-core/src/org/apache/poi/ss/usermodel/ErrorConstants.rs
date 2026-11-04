// java public class ErrorConstants
pub struct ErrorConstants;

// java public static final int ERROR_NULL = 0;
pub const ERROR_NULL: i32 = 0;
// java public static final int ERROR_DIV_0 = 7;
pub const ERROR_DIV_0: i32 = 7;
// java public static final int ERROR_VALUE = 15;
pub const ERROR_VALUE: i32 = 15;
// java public static final int ERROR_REF = 23;
pub const ERROR_REF: i32 = 23;
// java public static final int ERROR_NAME = 29;
pub const ERROR_NAME: i32 = 29;
// java public static final int ERROR_NUM = 36;
pub const ERROR_NUM: i32 = 36;
// java public static final int ERROR_NA = 42;
pub const ERROR_NA: i32 = 42;

// java public class ErrorConstants
impl ErrorConstants {
    // java protected ErrorConstants()
    fn ErrorConstants() {}

    // java public static final String getText(int errorCode)
    pub fn getText(errorCode: i32) -> String {
        // java switch (errorCode)
        match errorCode {
            // java case 0:
            0 => {
                // java return "#NULL!";
                "#NULL!".to_string()
            }
            // java case 7:
            7 => {
                // java return "#DIV/0!";
                "#DIV/0!".to_string()
            }
            // java case 15:
            15 => {
                // java return "#VALUE!";
                "#VALUE!".to_string()
            }
            // java case 23:
            23 => {
                // java return "#REF!";
                "#REF!".to_string()
            }
            // java case 29:
            29 => {
                // java return "#NAME?";
                "#NAME?".to_string()
            }
            // java case 36:
            36 => {
                // java return "#NUM!";
                "#NUM!".to_string()
            }
            // java case 42:
            42 => {
                // java return "#N/A";
                "#N/A".to_string()
            }
            // java default:
            _ => {
                // java throw new IllegalArgumentException("Bad error code (" + errorCode + ")");
                panic!("IllegalArgumentException: Bad error code ({errorCode})")
            }
        }
    }

    // java public static final int getErrCode(String err)
    pub fn getErrCode(err: Option<&str>) -> i32 {
        // java if ("#NULL!".equals(err))
        if err == Some("#NULL!") {
            // java return 0;
            return 0;
        // java else if ("#DIV/0!".equals(err))
        } else if err == Some("#DIV/0!") {
            // java return 7;
            return 7;
        // java else if ("#VALUE!".equals(err))
        } else if err == Some("#VALUE!") {
            // java return 15;
            return 15;
        // java else if ("#REF!".equals(err))
        } else if err == Some("#REF!") {
            // java return 23;
            return 23;
        // java else if ("#NAME?".equals(err))
        } else if err == Some("#NAME?") {
            // java return 29;
            return 29;
        // java else if ("#NUM!".equals(err))
        } else if err == Some("#NUM!") {
            // java return 36;
            return 36;
        // java else if ("#N/A".equals(err))
        } else if err == Some("#N/A") {
            // java return 42;
            return 42;
        } else {
            // java throw new IllegalArgumentException("Bad error string (" + err + ")");
            panic!(
                "IllegalArgumentException: Bad error string ({})",
                err.unwrap_or("null")
            )
        }
    }

    // java public static final boolean isValidCode(int errorCode)
    pub fn isValidCode(errorCode: i32) -> bool {
        // java switch (errorCode)
        match errorCode {
            // java case 0:
            // java case 7:
            // java case 15:
            // java case 23:
            // java case 29:
            // java case 36:
            // java case 42:
            0 | 7 | 15 | 23 | 29 | 36 | 42 => {
                // java return true;
                true
            }
            // java default:
            _ => {
                // java return false;
                false
            }
        }
    }
}

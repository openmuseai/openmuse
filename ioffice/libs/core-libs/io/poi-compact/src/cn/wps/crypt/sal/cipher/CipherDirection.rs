// Java: import cn.wps.crypt.ConstValue
use crate::cn::wps::crypt::ConstValue::SAL_MAX_ENUM;

// Java: public class CipherDirection
pub struct CipherDirection {
    // Java field: int Cipher_DirectionBoth
    // Java field: int Cipher_DirectionDecode
    // Java field: int Cipher_DirectionEncode
    // Java field: int Cipher_DirectionInvalid
    // Java field: int Cipher_Direction_FORCE_EQUAL_SIZE
}

pub const CIPHER_DIRECTION_BOTH: i32 = 0;
pub const CIPHER_DIRECTION_DECODE: i32 = 1;
pub const CIPHER_DIRECTION_ENCODE: i32 = 2;
pub const CIPHER_DIRECTION_INVALID: i32 = 3;
pub static CIPHER_DIRECTION_FORCE_EQUAL_SIZE: i32 = SAL_MAX_ENUM;

// Java: public class CipherDirection
impl CipherDirection {
    // Java: CipherDirection()
    pub fn new() -> Self {
        // Java: super();
        CipherDirection {}
    }
}

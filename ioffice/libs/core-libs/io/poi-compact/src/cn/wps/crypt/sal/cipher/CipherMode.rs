// Java: import cn.wps.crypt.ConstValue
use crate::cn::wps::crypt::ConstValue::SAL_MAX_ENUM;

// Java: public class CipherMode
pub struct CipherMode {
    // Java field: int Cipher_ModeECB
    // Java field: int Cipher_ModeCBC
    // Java field: int Cipher_ModeStream
    // Java field: int Cipher_ModeCFB
    // Java field: int Cipher_ModeInvalid
    // Java field: int Cipher_Mode_FORCE_EQUAL_SIZE
}

pub const CIPHER_MODE_ECB: i32 = 0;
pub const CIPHER_MODE_CBC: i32 = 1;
pub const CIPHER_MODE_STREAM: i32 = 2;
pub const CIPHER_MODE_CFB: i32 = 3;
pub const CIPHER_MODE_INVALID: i32 = 4;
pub static CIPHER_MODE_FORCE_EQUAL_SIZE: i32 = SAL_MAX_ENUM;

// Java: public class CipherMode
impl CipherMode {
    // Java: CipherMode()
    pub fn new() -> Self {
        // Java: super();
        CipherMode {}
    }
}

// Java: import cn.wps.crypt.ConstValue
use crate::cn::wps::crypt::ConstValue::SAL_MAX_ENUM;

// Java: public class CipherError
pub struct CipherError {
    // Java field: int Cipher_E_None
    // Java field: int Cipher_E_Argument
    // Java field: int Cipher_E_Algorithm
    // Java field: int Cipher_E_Direction
    // Java field: int Cipher_E_Mode
    // Java field: int Cipher_E_BufferSize
    // Java field: int Cipher_E_Memory
    // Java field: int Cipher_E_Unknown
    // Java field: int Cipher_E_FORCE_EQUAL_SIZE
}

pub const CIPHER_E_NONE: i32 = 0;
pub const CIPHER_E_ARGUMENT: i32 = 1;
pub const CIPHER_E_ALGORITHM: i32 = 2;
pub const CIPHER_E_DIRECTION: i32 = 3;
pub const CIPHER_E_MODE: i32 = 4;
pub const CIPHER_E_BUFFER_SIZE: i32 = 5;
pub const CIPHER_E_MEMORY: i32 = 6;
pub const CIPHER_E_UNKNOWN: i32 = 7;
pub static CIPHER_E_FORCE_EQUAL_SIZE: i32 = SAL_MAX_ENUM;

// Java: public class CipherError
impl CipherError {
    // Java: CipherError()
    pub fn new() -> Self {
        // Java: super();
        CipherError {}
    }
}

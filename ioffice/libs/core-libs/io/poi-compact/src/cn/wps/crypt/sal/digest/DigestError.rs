use crate::cn::wps::crypt::ConstValue::SAL_MAX_ENUM;

// Java: public class DigestError
// Java field: int Digest_E_None
pub const DIGEST_E_NONE: i32 = 0;
// Java field: int Digest_E_Argument
pub const DIGEST_E_ARGUMENT: i32 = 1;
// Java field: int Digest_E_Algorithm
pub const DIGEST_E_ALGORITHM: i32 = 2;
// Java field: int Digest_E_BufferSize
pub const DIGEST_E_BUFFER_SIZE: i32 = 3;
// Java field: int Digest_E_Memory
pub const DIGEST_E_MEMORY: i32 = 4;
// Java field: int Digest_E_Unknown
pub const DIGEST_E_UNKNOWN: i32 = 5;
// Java field: int Digest_E_FORCE_EQUAL_SIZE
pub static DIGEST_E_FORCE_EQUAL_SIZE: i32 = SAL_MAX_ENUM;

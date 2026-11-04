// Java: import cn.wps.crypt.sal.cipher.Cipher
use super::cipher::Cipher::Cipher;
// Java: import cn.wps.crypt.sal.cipher.CipherARCFOUR
use super::cipher::CipherARCFOUR::CipherARCFOUR;
// Java: import cn.wps.crypt.sal.cipher.CipherBF
use super::cipher::CipherBF::CipherBF;

// Java: public class CipherUtil
pub struct CipherUtil {
}

// Java: public class CipherUtil
impl CipherUtil {
    // Java: CipherUtil()
    pub fn new() -> Self {
        // Java: super();
        CipherUtil {}
    }

    // Java: static Cipher create(int algorithm, int mode)
    pub fn create(algorithm: i32, mode: i32) -> Option<Box<dyn Cipher>> {
        // Java: Cipher impl = null;
        let mut result: Option<Box<dyn Cipher>> = None;
        // Java: if (algorithm == 0)
        if algorithm == 0 {
            // Java: impl = new CipherBF(mode);
            result = Some(Box::new(CipherBF::new(mode)));
        }
        // Java: else if (algorithm == 1)
        else if algorithm == 1 {
            // Java: impl = new CipherARCFOUR(mode);
            result = Some(Box::new(CipherARCFOUR::new(mode)));
        }
        // Java: return impl;
        result
    }

    // Java: void destory(Cipher cipher)
    pub fn destory(cipher: &mut Option<Box<dyn Cipher>>) {
        // Java: if (cipher != null)
        if let Some(ref mut c) = cipher {
            // Java: cipher.dispose();
            c.dispose();
        }
    }
}

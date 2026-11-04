// Java: import cn.wps.crypt.sal.digest.DigestHMAC_MD5
use super::digest::DigestHMAC_MD5::DigestHMAC_MD5;
// Java: import cn.wps.crypt.sal.digest.DigestHMAC_SHA1
use super::digest::DigestHMAC_SHA1::DigestHMAC_SHA1;
// Java: import cn.wps.crypt.sal.digest.DigestMD2
use super::digest::DigestMD2::DigestMD2;
// Java: import cn.wps.crypt.sal.digest.DigestMD5
use super::digest::DigestMD5::DigestMD5;
// Java: import cn.wps.crypt.sal.digest.DigestSHA
use super::digest::DigestSHA::DigestSHA;
// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use super::digest::DigestSHA1::DigestSHA1;
// Java: import cn.wps.crypt.sal.digest.DigestSHA512
use super::digest::DigestSHA512::DigestSHA512;
// Java: import cn.wps.crypt.sal.digest.IDigest
use super::digest::IDigest::IDigest;

// Java: public class DigestUtil
pub struct DigestUtil {
}

// Java: public class DigestUtil
impl DigestUtil {
    // Java: DigestUtil()
    pub fn new() -> Self {
        // Java: super();
        DigestUtil {}
    }

    // Java: static IDigest create(int algorithm)
    pub fn create(algorithm: i32) -> Option<Box<dyn IDigest>> {
        // Java: IDigest impl = null;
        // Java: switch (algorithm)
        match algorithm {
            // Java: case 0:
            0 => {
                // Java: impl = createMD2();
                // Java: break;
                Some(Self::createMD2())
            }
            // Java: case 1:
            1 => {
                // Java: impl = createMD5();
                // Java: break;
                Some(Self::createMD5())
            }
            // Java: case 2:
            2 => {
                // Java: impl = createSHA();
                // Java: break;
                Some(Self::createSHA())
            }
            // Java: case 3:
            3 => {
                // Java: impl = createSHA1();
                // Java: break;
                Some(Self::createSHA1())
            }
            // Java: case 4:
            4 => {
                // Java: impl = createHMAC_MD5();
                // Java: break;
                Some(Self::createHMAC_MD5())
            }
            // Java: case 5:
            5 => {
                // Java: impl = createHMAC_SHA1();
                // Java: break;
                Some(Self::createHMAC_SHA1())
            }
            // Java: case 6:
            6 => {
                // Java: impl = createSHA512();
                // Java: break;
                Some(Self::createSHA512())
            }
            // Java: default:
            _ => {
                // Java: impl = null;
                None
            }
        }
        // Java: return impl;
    }

    // Java: static IDigest createMD2()
    fn createMD2() -> Box<dyn IDigest> {
        // Java: return new DigestMD2();
        Box::new(DigestMD2::new())
    }

    // Java: static IDigest createMD5()
    fn createMD5() -> Box<dyn IDigest> {
        // Java: return new DigestMD5();
        Box::new(DigestMD5::new())
    }

    // Java: static IDigest createSHA()
    fn createSHA() -> Box<dyn IDigest> {
        // Java: return new DigestSHA();
        Box::new(DigestSHA::new())
    }

    // Java: static IDigest createSHA1()
    fn createSHA1() -> Box<dyn IDigest> {
        // Java: return new DigestSHA1();
        Box::new(DigestSHA1::new())
    }

    // Java: static IDigest createHMAC_MD5()
    fn createHMAC_MD5() -> Box<dyn IDigest> {
        // Java: return new DigestHMAC_MD5();
        Box::new(DigestHMAC_MD5::new())
    }

    // Java: static IDigest createHMAC_SHA1()
    fn createHMAC_SHA1() -> Box<dyn IDigest> {
        // Java: return new DigestHMAC_SHA1();
        Box::new(DigestHMAC_SHA1::new())
    }

    // Java: static IDigest createSHA512()
    fn createSHA512() -> Box<dyn IDigest> {
        // Java: return new DigestSHA512();
        Box::new(DigestSHA512::new())
    }

    // Java: void destory(IDigest digest)
    pub fn destory(digest: &mut Option<Box<dyn IDigest>>) {
        // Java: if (digest != null)
        if let Some(ref mut d) = digest {
            // Java: digest.dispose();
            d.dispose();
        }
    }
}

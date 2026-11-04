// Java: import cn.wps.moffice.kfs.File
// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.crypt.EncryptionInfo
use crate::org::apache::poi::poifs::crypt::EncryptionInfo::EncryptionInfo;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java field: int DECRYPTSUCCESS
pub const DECRYPTSUCCESS: i32 = 0;
// Java field: int DECRYPTFAIL
pub const DECRYPTFAIL: i32 = 1;
// Java field: int DECRYPTERROR
pub const DECRYPTERROR: i32 = 2;

static B_INIT_JNI: std::sync::OnceLock<std::sync::Mutex<bool>> = std::sync::OnceLock::new();

// Java: public class OoxmlNativeDecrypt
pub struct OoxmlNativeDecrypt {
    // Java field: boolean bInitJNI
}

// Java: public class OoxmlNativeDecrypt
impl OoxmlNativeDecrypt {
    // Java: OoxmlNativeDecrypt()
    pub fn new() -> Self {
        // Java: super();
        OoxmlNativeDecrypt {}
    }

    // Java: static private static native int decryptByC(String var0, String var1, byte[] var2, String var3)
    fn decryptByC(_var0: &str, _var1: &str, _var2: &[u8], _var3: &str) -> i32 {
        DECRYPTERROR
    }

    // Java: static boolean isInitJNI()
    pub fn isInitJNI() -> bool {
        // Java: return bInitJNI;
        let mutex = B_INIT_JNI.get_or_init(|| std::sync::Mutex::new(false));
        *mutex.lock().unwrap()
    }

    // Java: static int decrypt(POIFSFileSystem poif, EncryptionInfo info, String dest, String passwd)
    pub fn decrypt(_poif: &POIFSFileSystem, _info: &EncryptionInfo, dest: &str, passwd: &str) -> std::io::Result<i32> {
        // Java: DocumentInputStream dis = poif.getRoot().createDocumentInputStream("EncryptionInfo");
        // Java: File infoFile = File.createTempFile("EncryptionInfo_", ".tmp");
        // Java: OoxmlDecrypt.copyFile(dis, infoFile.getAbsolutePath());
        // Java: dis = poif.getRoot().createDocumentInputStream("EncryptedPackage");
        // Java: File packageFile = File.createTempFile("EncryptedPackage_", ".tmp");
        // Java: OoxmlDecrypt.copyFile(dis, packageFile.getAbsolutePath());
        // Java: int bret = 2;
        let mut bret = 2i32;
        // Java: try
        {
            // Java: bret = decryptByC(infoFile.getAbsolutePath(), packageFile.getAbsolutePath(), passwd.getBytes("UTF-16LE"), dest);
            let passwd_bytes: Vec<u8> = passwd.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
            bret = Self::decryptByC("", "", &passwd_bytes, dest);
        }
        // Java: catch (Throwable t)
        {
            // Java: t.printStackTrace();
            // Java: bret = 2;
        }
        // Java: finally
        {
            // Java: if (infoFile.exists())
            {
                // Java: infoFile.delete();
            }
            // Java: if (packageFile.exists())
            {
                // Java: packageFile.delete();
            }
        }
        // Java: return bret;
        Ok(bret)
    }
}

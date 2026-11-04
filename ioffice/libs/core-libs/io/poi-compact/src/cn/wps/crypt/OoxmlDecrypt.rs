// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.util.IOUtils
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.PushbackInputStream
// Java: import java.security.GeneralSecurityException
// Java: import org.apache.poi.poifs.crypt.Decryptor
// Java: import org.apache.poi.poifs.crypt.EncryptionInfo
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;
use crate::org::apache::poi::poifs::crypt::EncryptionInfo::EncryptionInfo;
use crate::org::apache::poi::poifs::crypt::Decryptor::Decryptor as PoiDecryptor;

// Java: public class OoxmlDecrypt
pub struct OoxmlDecrypt {
}

// Java: public class OoxmlDecrypt
impl OoxmlDecrypt {
    // Java: OoxmlDecrypt()
    pub fn new() -> Self {
        // Java: super();
        OoxmlDecrypt {}
    }

    // Java: static boolean decrypt(String src, RandomAccessFile raf, String dest, String passwd)
    pub fn decrypt(_src: Option<&str>, _raf: Option<&mut dyn std::io::Read>, dest: Option<&str>, passwd: &str) -> std::io::Result<bool> {
        // Java: if (dest != null && (src != null || raf != null))
        if dest.is_some() && (_src.is_some() || _raf.is_some()) {
            // Java: POIFSFileSystem poif = null;
            // Java: if (null != raf)
            if _raf.is_some() {
                // Java: poif = new POIFSFileSystem(raf);
            }
            // Java: else
            else {
                // Java: poif = new POIFSFileSystem(new PushbackInputStream(new FileInputStream(src), 6));
            }
            // Java: EncryptionInfo info = new EncryptionInfo(poif);
            // Java: int major = info.getVersionMajor();
            // Java: int minor = info.getVersionMinor();
            // Java: if (major == 4 && minor == 4 && info.getHeader().getHashAlgorithm() == 32782)
            {
                // Java: if (!OoxmlNativeDecrypt.isInitJNI())
                {
                    // Java: return decryptByJava(poif, info, dest, passwd);
                }
                // Java: int bret = OoxmlNativeDecrypt.decrypt(poif, info, dest, passwd);
                // Java: if (bret == 0)
                {
                    // Java: return true;
                }
                // Java: if (bret == 1)
                {
                    // Java: return false;
                }
            }
            // Java: return decryptByJava(poif, info, dest, passwd);
            return Ok(false);
        }
        // Java: else
        else {
            // Java: return false;
            return Ok(false);
        }
    }

    // Java: static boolean decryptByJava(POIFSFileSystem poif, EncryptionInfo info, String dest, String passwd)
    fn decryptByJava(_poif: &POIFSFileSystem, _info: &EncryptionInfo, _dest: &str, _passwd: &str) -> std::io::Result<bool> {
        // Java: Decryptor decryptor = Decryptor.getInstance(info);
        // Java: boolean isVerified = false;
        let mut isVerified = false;
        // Java: try
        {
            // Java: isVerified = decryptor.verifyPassword(passwd);
        }
        // Java: catch (GeneralSecurityException var9)
        {
            // Java: isVerified = false;
            isVerified = false;
        }
        // Java: if (!isVerified)
        if !isVerified {
            // Java: return false;
            return Ok(false);
        }
        // Java: else
        else {
            // Java: InputStream input = null;
            // Java: try
            {
                // Java: input = decryptor.getDataStream(poif);
            }
            // Java: catch (GeneralSecurityException e)
            {
                // Java: throw new RuntimeException("Unable to process encrypted document", e);
            }
            // Java: if (input == null)
            {
                // Java: return false;
            }
            // Java: else
            {
                // Java: copyFile(input, dest);
                // Java: return true;
                return Ok(true);
            }
        }
    }

    // Java: static void copyFile(InputStream input, String dest)
    pub fn copyFile(input: &mut dyn std::io::Read, dest: &str) -> std::io::Result<()> {
        // Java: FileOutputStream output = null;
        // Java: try
        {
            // Java: output = new FileOutputStream(new File(dest));
            let mut output = std::fs::File::create(dest)?;
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = vec![0u8; 4096];
            // Java: int len = -1;
            let mut len: i32;
            // Java: while((len = input.read(buffer)) > 0)
            loop {
                let n = input.read(&mut buffer)?;
                len = n as i32;
                if len <= 0 { break; }
                {
                    // Java: output.write(buffer, 0, len);
                    std::io::Write::write_all(&mut output, &buffer[..n])?;
                }
            }
            // Java: output.flush();
            std::io::Write::flush(&mut output)?;
        }
        // Java: catch (IOException e)
        {
            // Java: throw e;
        }
        // Java: finally
        {
            // Java: IOUtils.closeStream(output);
        }
        Ok(())
    }
}

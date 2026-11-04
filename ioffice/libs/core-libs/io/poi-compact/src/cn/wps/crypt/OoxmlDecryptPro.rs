// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.util.IOUtils
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.PushbackInputStream
// Java: import java.security.GeneralSecurityException
// Java: import org.apache.poi.poifs.crypt.Decryptor
// Java: import org.apache.poi.poifs.crypt.EncryptionInfoPro
use crate::org::apache::poi::poifs::crypt::EncryptionInfoPro::EncryptionInfoPro;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java: public class OoxmlDecryptPro
pub struct OoxmlDecryptPro {
}

// Java: public class OoxmlDecryptPro
impl OoxmlDecryptPro {
    // Java: OoxmlDecryptPro()
    pub fn new() -> Self {
        // Java: super();
        OoxmlDecryptPro {}
    }

    // Java: static boolean decrypt(EncryptionInfoPro info, DocumentInputStream stream, String src, String dest, String passwd)
    pub fn decrypt(_info: &EncryptionInfoPro, _stream: &mut DocumentInputStream, _src: &str, _dest: &str, _passwd: &str) -> std::io::Result<bool> {
        // Java: if (dest != null && src != null)
        if !_dest.is_empty() && !_src.is_empty() {
            // Java: int major = info.getVersionMajor();
            // Java: int minor = info.getVersionMinor();
            // Java: POIFSFileSystem poif = new POIFSFileSystem(new PushbackInputStream(new FileInputStream(src), 6));
            // Java: if (major == 4 && minor == 4 && info.getHeader().getHashAlgorithm() == 32782)
            {
                // Java: if (!OoxmlNativeDecrypt.isInitJNI())
                {
                    // Java: return decryptByJava(poif, stream, info, dest, passwd);
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
            // Java: return decryptByJava(poif, stream, info, dest, passwd);
            return Ok(false);
        }
        // Java: else
        else {
            // Java: return false;
            return Ok(false);
        }
    }

    // Java: static boolean decryptByJava(POIFSFileSystem poif, DocumentInputStream stream, EncryptionInfoPro info, String dest, String passwd)
    fn decryptByJava(_poif: &POIFSFileSystem, _stream: &mut DocumentInputStream, _info: &EncryptionInfoPro, _dest: &str, _passwd: &str) -> std::io::Result<bool> {
        // Java: Decryptor decryptor = Decryptor.getInstance(info);
        // Java: boolean isVerified = false;
        let mut isVerified = false;
        // Java: try
        {
            // Java: isVerified = decryptor.verifyPassword(passwd);
        }
        // Java: catch (GeneralSecurityException var10)
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
                // Java: input = decryptor.getWpsContent(stream);
            }
            // Java: catch (GeneralSecurityException e)
            {
                // Java: throw new RuntimeException("Unable to process encrypted document", e);
            }
            // Java: int len = decryptor.getDataStreamSize();
            // Java: if (input == null)
            {
                // Java: return false;
            }
            // Java: else
            {
                // Java: copyFile(input, dest, len);
                // Java: return true;
                return Ok(true);
            }
        }
    }

    // Java: static void copyFile(InputStream input, String dest, int size)
    pub fn copyFile(input: &mut dyn std::io::Read, dest: &str, size: i32) -> std::io::Result<()> {
        // Java: FileOutputStream output = null;
        // Java: try
        {
            // Java: output = new FileOutputStream(new File(dest));
            let mut output = std::fs::File::create(dest)?;
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = vec![0u8; 4096];
            // Java: int len = -1;
            let mut len: i32;
            // Java: for(int lenTemp = 0; (len = input.read(buffer)) > 0; output.write(buffer, 0, len))
            let mut lenTemp = 0i32;
            loop {
                let n = input.read(&mut buffer)?;
                len = n as i32;
                if len <= 0 { break; }
                {
                    // Java: lenTemp += len;
                    lenTemp += len;
                    // Java: if (lenTemp > size)
                    if lenTemp > size {
                        // Java: int ignoreSize = lenTemp - size;
                        let ignoreSize = lenTemp - size;
                        // Java: len -= ignoreSize;
                        len -= ignoreSize;
                    }
                }
                std::io::Write::write_all(&mut output, &buffer[..len as usize])?;
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

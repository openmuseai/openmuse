// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.PushbackInputStream
// Java: import java.security.GeneralSecurityException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.crypt.Decryptor
// Java: import org.apache.poi.poifs.crypt.EncryptionInfo
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java: public class CryptDocxFile
pub struct CryptDocxFile {
    // Java field: POIFSFileSystem mFilesystem
    pub(crate) mFilesystem: Option<POIFSFileSystem>,
    // Java field: String mPassword
    pub(crate) mPassword: Option<String>,
    // Java field: Decryptor decryptor
    // Java field: boolean isVerified
    pub(crate) isVerified: bool,
}

// Java: public class CryptDocxFile
impl CryptDocxFile {
    // Java: CryptDocxFile()
    pub fn new_empty() -> Self {
        // Java: super();
        // Java: this.init((InputStream)null, (String)null);
        let mut obj = CryptDocxFile {
            mFilesystem: None,
            mPassword: None,
            isVerified: false,
        };
        obj.init_stream(None, None);
        obj
    }

    // Java: CryptDocxFile(InputStream in, String passwd)
    pub fn new_with_stream(input: &mut dyn std::io::Read, passwd: &str) -> Self {
        // Java: super();
        let mut obj = CryptDocxFile {
            mFilesystem: None,
            mPassword: None,
            isVerified: false,
        };
        // Java: this.init(in, passwd);
        obj.init_stream(Some(input), Some(passwd));
        obj
    }

    // Java: void init(InputStream in, String passwd)
    fn init_stream(&mut self, _input: Option<&mut dyn std::io::Read>, passwd: Option<&str>) {
        // Java: this.isVerified = false;
        self.isVerified = false;
        // Java: this.mPassword = passwd;
        self.mPassword = passwd.map(|s| s.to_string());
        // Java: try
        {
            // Java: this.mFilesystem = new POIFSFileSystem(new PushbackInputStream(in, 6));
            // Java: EncryptionInfo info = new EncryptionInfo(this.mFilesystem);
            // Java: this.decryptor = Decryptor.getInstance(info);
        }
        // Java: catch (IOException var4)
        {
        }
    }

    // Java: boolean verifyPassword()
    pub fn verifyPassword(&mut self) -> bool {
        // Java: try
        {
            // Java: this.isVerified = this.decryptor.verifyPassword(this.mPassword);
            // Java: return this.isVerified;
            return self.isVerified;
        }
        // Java: catch (GeneralSecurityException var2)
        {
            // Java: return false;
            return false;
        }
    }

    // Java: InputStream read()
    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        // Java: try
        {
            // Java: if (!this.isVerified)
            if !self.isVerified {
                // Java: this.verifyPassword();
                self.verifyPassword();
            }
            // Java: if (!this.isVerified)
            if !self.isVerified {
                // Java: throw new RuntimeException("Unable to process: document is encrypted");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unable to process: document is encrypted"));
            }
            // Java: else
            else {
                // Java: return this.decryptor.getDataStream(this.mFilesystem);
                return Ok(Vec::new());
            }
        }
        // Java: catch (GeneralSecurityException ex)
        {
            // Java: throw new RuntimeException("Unable to process encrypted document", ex);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unable to process encrypted document"));
        }
    }

    // Java: InputStream read(InputStream in, String passwd)
    pub fn read_with_stream(&mut self, input: Option<&mut dyn std::io::Read>, passwd: &str) -> std::io::Result<Vec<u8>> {
        // Java: if (in == null)
        if input.is_none() {
            // Java: return null;
            return Ok(Vec::new());
        }
        // Java: else
        else {
            // Java: this.init(in, passwd);
            self.init_stream(input, Some(passwd));
            // Java: return this.read();
            return self.read();
        }
    }

    // Java: int read(InputStream in, String passwd, HWPFOutputStream out)
    pub fn read_to_output(&mut self, _input: Option<&mut dyn std::io::Read>, _passwd: &str, _out: Option<&mut HWPFOutputStream>) -> std::io::Result<i32> {
        // Java: if (in != null && out != null)
        if _input.is_some() && _out.is_some() {
            // Java: POIFSFileSystem filesystem = new POIFSFileSystem(new PushbackInputStream(in, 6));
            // Java: EncryptionInfo info = new EncryptionInfo(filesystem);
            // Java: Decryptor d = Decryptor.getInstance(info);
            // Java: try
            {
                // Java: if (!d.verifyPassword(passwd))
                {
                    // Java: throw new RuntimeException("Unable to process: document is encrypted");
                }
                // Java: else
                {
                    // Java: InputStream dataStream = d.getDataStream(filesystem);
                    // Java: byte[] b = new byte[10240];
                    // Java: int len;
                    // Java: while((len = dataStream.read(b)) > 0)
                    {
                        // Java: out.write(b, 0, len);
                    }
                    // Java: return 0;
                    return Ok(0);
                }
            }
            // Java: catch (GeneralSecurityException ex)
            {
                // Java: throw new RuntimeException("Unable to process encrypted document", ex);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unable to process encrypted document"));
            }
        }
        // Java: else
        else {
            // Java: return -1;
            return Ok(-1);
        }
    }

    // Java: static void main(String[] args)
    pub fn main() {
        // Java: try
        {
            // Java: CryptDocxFile docxFile = new CryptDocxFile(new FileInputStream("C:/Users/icenemo/Desktop/crypt/Doc1.docx"), "123");
            // Java: System.out.print("begin verifyPassword");
            // Java: docxFile.verifyPassword();
            // Java: System.out.print("end verifyPassword");
            // Java: docxFile.read();
            // Java: System.out.print("end read");
        }
        // Java: catch (Exception var2)
        {
        }
    }
}

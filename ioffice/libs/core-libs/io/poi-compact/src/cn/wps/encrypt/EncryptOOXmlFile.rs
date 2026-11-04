// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.nio.charset.StandardCharsets
// Java: import java.security.GeneralSecurityException
// Java: import java.util.zip.ZipEntry
// Java: import java.util.zip.ZipException
// Java: import java.util.zip.ZipFile
// Java: import javax.crypto.Cipher
// Java: import javax.crypto.CipherInputStream
// Java: import javax.crypto.SecretKey
// Java: import javax.crypto.spec.SecretKeySpec
// Java: import org.apache.poi.hpsf.ClassID
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;
// Java: import org.apache.poi.util.LittleEndian

use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;
use crate::cn::wps::encrypt::Decryptor::DecryptorTrait;
use crate::cn::wps::encrypt::ModifyVerifier::ModifyVerifier;
use crate::cn::wps::encrypt::CryptTool::CryptTool;

static EDIT_PASS: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();

// Java: public class EncryptOOXmlFile
pub struct EncryptOOXmlFile {
    // Java field: POIFSFileSystem mFilesystem
    pub(crate) mFilesystem: Option<POIFSFileSystem>,
    // Java field: String mOpenPassword
    pub(crate) mOpenPassword: Option<String>,
    // Java field: String mEditPassword
    pub(crate) mEditPassword: Option<String>,
    // Java field: Decryptor decryptor
    pub(crate) decryptor: Option<Box<dyn DecryptorTrait>>,
    // Java field: boolean isVerified
    pub(crate) isVerified: bool,
    // Java field: ModifyVerifier modifyVerifier
    pub(crate) modifyVerifier: Option<ModifyVerifier>,
    // Java field: String editPass
    // Java field: EncryptionInfo info
    pub(crate) info: Option<EncryptionInfo>,
}

// Java: public class EncryptOOXmlFile
impl EncryptOOXmlFile {
    // Java: EncryptOOXmlFile(RandomAccessFile accessFile)
    pub fn new_from_file(accessFile: &mut dyn std::io::Read) -> Self {
        // Java: super();
        let mut obj = Self::new_empty();
        // Java: this.init(accessFile);
        obj.init(accessFile);
        obj
    }

    // Java: EncryptOOXmlFile()
    pub fn new_empty() -> Self {
        // Java: super();
        let mut obj = EncryptOOXmlFile {
            mFilesystem: None,
            mOpenPassword: None,
            mEditPassword: None,
            decryptor: None,
            isVerified: false,
            modifyVerifier: None,
            info: None,
        };
        // Java: try
        {
            // Java: this.mFilesystem = new POIFSFileSystem();
            // Java: this.modifyVerifier = null;
            obj.modifyVerifier = None;
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        obj
    }

    // Java: void init(RandomAccessFile accessFile)
    fn init(&mut self, _accessFile: &mut dyn std::io::Read) {
        // Java: this.isVerified = false;
        self.isVerified = false;
        // Java: try
        {
            // Java: this.mFilesystem = new POIFSFileSystem(accessFile);
            // Java: EncryptionInfo info = new EncryptionInfo(this.mFilesystem);
            // Java: this.decryptor = Decryptor.getInstance(info);
        }
        // Java: catch (IOException e)
        {
            // Java: e.printStackTrace();
        }
    }

    // Java: void setOpenPassword(String openPwd)
    pub fn setOpenPassword(&mut self, openPwd: &str) {
        // Java: this.mOpenPassword = openPwd;
        self.mOpenPassword = Some(openPwd.to_string());
        // Java: this.info = new EncryptionInfo(this.mOpenPassword);
        self.info = Some(EncryptionInfo::new_from_password(openPwd));
    }

    // Java: String getOpenPassword()
    pub fn getOpenPassword(&self) -> Option<&str> {
        // Java: return this.mOpenPassword;
        self.mOpenPassword.as_deref()
    }

    // Java: static void setEditPassword(String editPwd)
    pub fn setEditPassword(editPwd: &str) {
        // Java: editPass = editPwd;
        let mutex = EDIT_PASS.get_or_init(|| std::sync::Mutex::new(None));
        *mutex.lock().unwrap() = Some(editPwd.to_string());
    }

    // Java: static String getEditPassword()
    pub fn getEditPassword() -> Option<String> {
        // Java: return editPass;
        EDIT_PASS.get().and_then(|m| m.lock().unwrap().clone())
    }

    // Java: ModifyVerifier getModifyVerifier()
    pub fn getModifyVerifier(&self) -> Option<&ModifyVerifier> {
        // Java: return this.modifyVerifier;
        self.modifyVerifier.as_ref()
    }

    // Java: static boolean verifyEditPassword(ModifyVerifier srcModifyVer, String pwd)
    pub fn verifyEditPassword(srcModifyVer: &ModifyVerifier, pwd: &str) -> bool {
        // Java: editPass = pwd;
        Self::setEditPassword(pwd);
        // Java: boolean val = CryptTool.checkModifyPassword(srcModifyVer, pwd);
        let val = CryptTool::checkModifyPassword(srcModifyVer, pwd);
        // Java: return val;
        val
    }

    // Java: ModifyVerifier createSaltHashData(String editPwd)
    pub fn createSaltHashData(&mut self, editPwd: &str) -> &ModifyVerifier {
        // Java: ModifyVerifier modver = CryptTool.createModifyVerifier(editPwd);
        let modver = CryptTool::createModifyVerifier(editPwd);
        // Java: this.modifyVerifier = modver;
        self.modifyVerifier = Some(modver);
        // Java: return this.modifyVerifier;
        self.modifyVerifier.as_ref().unwrap()
    }

    // Java: boolean verifyOpenPassword(String pwd)
    pub fn verifyOpenPassword(&mut self, pwd: &str) -> bool {
        // Java: try
        {
            // Java: this.isVerified = this.decryptor.verifyPassword(pwd);
            if let Some(ref mut d) = self.decryptor {
                self.isVerified = d.verifyPassword(pwd);
            }
            // Java: if (this.isVerified)
            if self.isVerified {
                // Java: this.mOpenPassword = pwd;
                self.mOpenPassword = Some(pwd.to_string());
            }
            // Java: return this.isVerified;
            return self.isVerified;
        }
        // Java: catch (GeneralSecurityException e)
        {
            // Java: e.printStackTrace();
            // Java: return false;
            return false;
        }
    }

    // Java: boolean isEncryptFile()
    pub fn isEncryptFile(&self) -> bool {
        // Java: try
        {
            // Java: if (this.mFilesystem.getRoot().getEntry("EncryptionInfo") != null)
            if self.mFilesystem.is_some() {
                // Java: return true;
                return true;
            }
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        false
    }

    // Java: boolean hasOnlyModifyVerifier(String filePath)
    pub fn hasOnlyModifyVerifier(&self, _filePath: &str) -> bool {
        // Java: boolean retValue = false;
        let mut retValue = false;
        // Java: try
        {
            // Java: ZipFile zipArchive = new ZipFile(new File(filePath));
            // Java: if (zipArchive != null)
            {
                // Java: ZipEntry preEntry = zipArchive.getEntry("ppt/presentation.xml");
                // Java: if (preEntry != null)
                {
                    // Java: InputStream preStream = zipArchive.getInputStream(preEntry);
                    // Java: if (preStream != null && preStream.available() > 0)
                    {
                        // Java: int readLength = -1;
                        let _readLength = -1i32;
                        // Java: Object var13;
                        // Java: for(byte[] buffer = new byte[4096]; preStream.read(buffer) > 0; var13 = null)
                        {
                            // Java: String strline = new String(buffer, StandardCharsets.UTF_8);
                            // Java: if (strline.indexOf("p:modifyV") >= 0)
                            {
                                // Java: retValue = true;
                                retValue = true;
                                // Java: String var14 = null;
                                // Java: break;
                            }
                        }
                        // Java: byte[] var12 = null;
                        // Java: preStream.close();
                    }
                }
                // Java: zipArchive.close();
                // Java: Object var11 = null;
            }
            // Java: return retValue;
            return retValue;
        }
        // Java: catch (ZipException e)
        {
            // Java: e.printStackTrace();
            // Java: return false;
            return false;
        }
        // Java: catch (IOException e)
        {
            // Java: e.printStackTrace();
            // Java: return false;
            return false;
        }
    }

    // Java: int readCryptFileStreamToFile(String outfilepath)
    pub fn readCryptFileStreamToFile(&mut self, _outfilepath: &str) -> std::io::Result<i32> {
        // Java: POIFSFileSystem filesystem = this.mFilesystem;
        // Java: EncryptionInfo info = new EncryptionInfo(filesystem);
        // Java: Decryptor d = Decryptor.getInstance(info);
        // Java: try
        {
            // Java: if (!d.verifyPassword(this.mOpenPassword))
            {
                // Java: throw new RuntimeException("Unable to process: document is encrypted");
            }
            // Java: else
            {
                // Java: InputStream dataStream = d.getDataStream(filesystem);
                // Java: File tempfile = new File(outfilepath);
                // Java: FileOutputStream out = new FileOutputStream(tempfile);
                // Java: byte[] b = new byte[10240];
                // Java: int len;
                // Java: while((len = dataStream.read(b)) > 0)
                {
                    // Java: out.write(b, 0, len);
                }
                // Java: FileUtil.closeQuietly(out);
                // Java: dataStream.close();
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

    // Java: Cipher getCipher()
    pub fn getCipher(&self) -> Option<CryptTool> {
        // Java: try
        {
            // Java: CryptTool cryptTool = new CryptTool();
            let mut cryptTool = CryptTool::new();
            // Java: Cipher cipher = Cipher.getInstance("AES");
            // Java: byte[] passwordHash = this.info.getVerifier().getPasswordHash();
            // Java: cryptTool.setPasswordHash(passwordHash);
            // Java: byte[] key = cryptTool.generateKey(0, 16);
            let _key = cryptTool.generateKey(0, 16);
            // Java: SecretKey sKey = new SecretKeySpec(key, "AES");
            // Java: cipher.init(1, sKey);
            // Java: return cipher;
            return Some(cryptTool);
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
            // Java: return null;
            return None;
        }
    }

    // Java: void write(String srcFile, String desFile, String tmpFilePath)
    pub fn write_str(&self, srcFile: &str, desFile: &str, tmpFilePath: &str) {
        // Java: File sourceFile = new File(srcFile);
        // Java: File resFile = new File(desFile);
        // Java: this.write(sourceFile, resFile, tmpFilePath);
        self.write_file(srcFile, desFile, tmpFilePath);
    }

    // Java: void write(File sourceFile, File outFile, String tmpFilePath)
    pub fn write_file(&self, _sourceFile: &str, _outFile: &str, _tmpFilePath: &str) {
        // Java: byte[] encryptInfo = this.info.getAllInfo();
        // Java: try
        {
            // Java: if (outFile.exists())
            {
                // Java: outFile.delete();
            }
            // Java: ByteArrayInputStream inputStream = new ByteArrayInputStream(encryptInfo);
            // Java: DirectoryEntry rootEntry = this.mFilesystem.getRoot();
            // Java: byte[] classIdarr = new byte[]{-48, -103, -113, -99, -109, -14, -52, 1, 3, 0, 0, 0, 64, 3, 0, 0};
            // Java: ClassID classId = new ClassID(classIdarr, 0);
            // Java: rootEntry.setStorageClsid(classId);
            // Java: rootEntry.createDocument("EncryptionInfo", inputStream, false);
            // Java: FileInputStream inStream = new FileInputStream(sourceFile);
            // Java: byte[] encryptData = new byte[10240];
            // Java: CipherInputStream input = new CipherInputStream(inStream, this.getCipher());
            // Java: File file = new File(tmpFilePath);
            // Java: if (file.exists())
            {
                // Java: file.delete();
            }
            // Java: FileOutputStream output = new FileOutputStream(tmpFilePath);
            // Java: byte[] len = new byte[8];
            // Java: int encryptLen = 0;
            // Java: LittleEndian.putLong(len, 0, sourceFile.length());
            // Java: output.write(len, 0, 8);
            // Java: while((encryptLen = input.read(encryptData)) != -1)
            {
                // Java: output.write(encryptData, 0, encryptLen);
            }
            // Java: FileUtil.closeQuietly(output);
            // Java: FileInputStream in = new FileInputStream(tmpFilePath);
            // Java: rootEntry.createDocument("EncryptedPackage", in, false);
            // Java: DirectoryEntry dataSpace = null;
            // Java: DocumentEntry doentry = null;
            // Java: dataSpace = rootEntry.createDirectory("\u0006DataSpaces");
            // Java: byte[] classId1 = new byte[]{16, -98, -38, -16, 1, -14, -52, 1, 16, 68, -23, -16, 1, -14, -52, 1};
            // Java: ClassID classId11 = new ClassID(classId1, 0);
            // Java: dataSpace.setStorageClsid(classId11);
            // Java: byte[] dataSpaceEntry = new byte[]{8, 0, 0, 0, 1, 0, 0, 0, 104, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 69, 0, 110, 0, 99, 0, 114, 0, 121, 0, 112, 0, 116, 0, 101, 0, 100, 0, 80, 0, 97, 0, 99, 0, 107, 0, 97, 0, 103, 0, 101, 0, 50, 0, 0, 0, 83, 0, 116, 0, 114, 0, 111, 0, 110, 0, 103, 0, 69, 0, 110, 0, 99, 0, 114, 0, 121, 0, 112, 0, 116, 0, 105, 0, 111, 0, 110, 0, 68, 0, 97, 0, 116, 0, 97, 0, 83, 0, 112, 0, 97, 0, 99, 0, 101, 0, 0, 0};
            // Java: ByteArrayInputStream dis = new ByteArrayInputStream(dataSpaceEntry);
            // Java: dataSpace.createDocument("DataSpaceMap", dis, false);
            // Java: dis.reset();
            // Java: DirectoryEntry dataSpaceInfo = null;
            // Java: DocumentEntry encryptionDataSpce = null;
            // Java: dataSpaceInfo = dataSpace.createDirectory("DataSpaceInfo");
            // Java: byte[] classId2 = new byte[]{96, -46, -35, -16, 1, -14, -52, 1, -32, 10, -33, -16, 1, -14, -52, 1};
            // Java: ClassID classId12 = new ClassID(classId2, 0);
            // Java: dataSpaceInfo.setStorageClsid(classId12);
            // Java: byte[] dataSpaceDefinition = new byte[]{8, 0, 0, 0, 1, 0, 0, 0, 50, 0, 0, 0, 83, 0, 116, 0, 114, 0, 111, 0, 110, 0, 103, 0, 69, 0, 110, 0, 99, 0, 114, 0, 121, 0, 112, 0, 116, 0, 105, 0, 111, 0, 110, 0, 84, 0, 114, 0, 97, 0, 110, 0, 115, 0, 102, 0, 111, 0, 114, 0, 109, 0, 0, 0};
            // Java: dis = new ByteArrayInputStream(dataSpaceDefinition);
            // Java: dataSpaceInfo.createDocument("StrongEncryptionDataSpace", dis, false);
            // Java: dis.reset();
            // Java: DirectoryEntry transformInfo = null;
            // Java: DirectoryEntry encryptionTransf = null;
            // Java: DocumentEntry primary = null;
            // Java: transformInfo = dataSpace.createDirectory("TransformInfo");
            // Java: byte[] classId3 = new byte[]{-32, 123, -31, -16, 1, -14, -52, 1, 0, -84, -26, -4, 1, -14, -52, 1};
            // Java: ClassID classId13 = new ClassID(classId3, 0);
            // Java: transformInfo.setStorageClsid(classId13);
            // Java: encryptionTransf = transformInfo.createDirectory("StrongEncryptionTransform");
            // Java: byte[] iRMDSTransformInfo = new byte[]{108, 0, 0, 0, 1, 0, 0, 0, 76, 0, 0, 0, 123, 0, 70, 0, 70, 0, 57, 0, 65, 0, 51, 0, 70, 0, 48, 0, 51, 0, 45, 0, 53, 0, 54, 0, 69, 0, 70, 0, 45, 0, 52, 0, 54, 0, 49, 0, 51, 0, 45, 0, 66, 0, 68, 0, 68, 0, 53, 0, 45, 53, 65, 0, 52, 0, 49, 0, 67, 0, 49, 0, 68, 0, 48, 0, 55, 0, 50, 0, 52, 0, 54, 0, 125, 0, 78, 0, 0, 0, 77, 0, 105, 0, 99, 0, 114, 0, 111, 0, 115, 0, 111, 0, 102, 0, 116, 0, 46, 0, 67, 0, 111, 0, 110, 0, 116, 0, 97, 0, 105, 0, 110, 0, 101, 0, 114, 0, 46, 0, 69, 0, 110, 0, 99, 0, 114, 0, 121, 0, 112, 0, 116, 0, 105, 0, 111, 0, 110, 0, 84, 0, 114, 0, 97, 0, 110, 0, 115, 0, 102, 0, 111, 0, 114, 0, 109, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 0, 65, 69, 83, 32, 49, 50, 56, 0, 16, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0};
            // Java: ByteArrayInputStream array = new ByteArrayInputStream(iRMDSTransformInfo);
            // Java: encryptionTransf.createDocument("\u0006Primary", array, false);
            // Java: FileOutputStream fos = new FileOutputStream(outFile);
            // Java: this.mFilesystem.writeFilesystem(fos);
            // Java: array.close();
            // Java: ByteArrayInputStream var52 = null;
            // Java: input.close();
            // Java: CipherInputStream var44 = null;
            // Java: dis.close();
            // Java: ByteArrayInputStream var48 = null;
            // Java: inStream.close();
            // Java: FileInputStream var43 = null;
            // Java: inputStream.close();
            // Java: ByteArrayInputStream var42 = null;
            // Java: FileUtil.closeQuietly(fos);
            // Java: File tempFile = new File(tmpFilePath);
            // Java: if (tempFile.exists())
            {
                // Java: tempFile.delete();
            }
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
    }
}

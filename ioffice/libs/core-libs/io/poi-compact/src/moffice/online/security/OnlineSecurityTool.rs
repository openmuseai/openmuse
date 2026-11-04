// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.IClassLoaderManager
use util::cn::wps::core::runtime::IClassLoaderManager;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.online.security.data.SecurityCreateDocData
use util_adapter::cn::wps::moffice::online::security::data::SecurityCreateDocData;
// Java: import cn.wps.moffice.online.security.data.SecurityReadDocData
use util_adapter::cn::wps::moffice::online::security::data::SecurityReadDocData;
// Java: import cn.wps.moffice.online.security.data.SecurityRightData
use util_adapter::cn::wps::moffice::online::security::data::SecurityRightData;
// Java: import cn.wps.moffice.online.security.data.SecurityUpdateDocData
use util_adapter::cn::wps::moffice::online::security::data::SecurityUpdateDocData;
// Java: import cn.wps.moffice.online.security.data.SecurityVersionsData
use util_adapter::cn::wps::moffice::online::security::data::SecurityVersionsData;
// Java: import cn.wps.moffice.online.security.exception.OnlineSecurityException
use util_adapter::cn::wps::moffice::online::security::exception::OnlineSecurityException;
// Java: import cn.wps.moffice.online.security.exception.PermissionDeniedException
use util_adapter::cn::wps::moffice::online::security::exception::PermissionDeniedException;
// Java: import cn.wps.moffice.online.security.util.PermissionUtil
use util_adapter::cn::wps::moffice::online::security::util::PermissionUtil;
// Java: import cn.wps.moffice.util.ArtMode
use util::cn::wps::moffice::util::ArtMode;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream
// Java: import java.lang.reflect.Constructor
// TODO_RUST_CODE use ???; // java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException
// TODO_RUST_CODE use ???; // java.lang.reflect.InvocationTargetException
// Java: import java.security.SecureRandom
// TODO_RUST_CODE use ???; // java.security.SecureRandom
// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: String = /* TODO */;
// Java field: IOnlineSecurityAPI smApi
static smApi: std::sync::OnceLock<std::sync::Mutex<IOnlineSecurityAPI>> = std::sync::OnceLock::new();
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = /* TODO */;
// Java field: String CLASS_READER
pub(crate) const CLASS_READER: String = /* TODO */;
// Java field: String CLASS_WRITER
pub(crate) const CLASS_WRITER: String = /* TODO */;
// Java field: String VERIFY_INFO_TAG
pub(crate) const VERIFY_INFO_TAG: String = /* TODO */;
// Java field: SecureRandom random
static random: std::sync::OnceLock<SecureRandom> = std::sync::OnceLock::new();
// Java field: String WPSCONTENT
pub(crate) const WPSCONTENT: String = /* TODO */;

// Java: public class OnlineSecurityTool implements IOnlineSecurityTool
pub struct OnlineSecurityTool {
    // Java field: String mFileName
    mFileName: Option<String>,
    // Java field: String mGuid
    pub(crate) mGuid: Arc<Mutex<String>>,
    // Java field: String mSecretKey
    mSecretKey: Option<String>,
    // Java field: String mSign
    mSign: Option<String>,
    // Java field: String mEncData
    mEncData: Option<String>,
    // Java field: int mPermission
    mPermission: i32,
    // Java field: SecurityRightData mRights
    mRights: Option<SecurityRightData>,
    // Java field: SecurityVersionsData mVersions
    mVersions: Option<SecurityVersionsData>,
    // Java field: int mVersionMajor
    pub(crate) mVersionMajor: i32,
    // Java field: int mVersionMinor
    pub(crate) mVersionMinor: i32,
    // Java field: boolean mSkipOnlineExp
    mSkipOnlineExp: bool,
    // Java field: boolean mEnable
    pub(crate) mEnable: bool,
    // Java field: POIFSFileSystem mPOIFS
    pub(crate) mPOIFS: Arc<Mutex<POIFSFileSystem>>,
    // Java field: byte[] mEvidenceData
    pub(crate) mEvidenceData: Arc<Mutex<Vec<u8>>>,
}

// Java: public class OnlineSecurityTool implements IOnlineSecurityTool
impl OnlineSecurityTool {
    // Java: OnlineSecurityTool()
    pub fn OnlineSecurityTool(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: OnlineSecurityTool(String fileName)
    pub fn OnlineSecurityTool(&self, String fileName) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFileName = fileName;
      // TODO_RUST_CODE
    }

    // Java: OnlineSecurityTool(OnlineSecurityTool other)
    pub fn OnlineSecurityTool(&self, OnlineSecurityTool other) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.copyFrom(other);
      // TODO_RUST_CODE
    }

    // Java: boolean isWpsSecurityFile(String path)
    pub fn isWpsSecurityFile(&self, String path) -> bool {
      // Java: POIFSFileSystem poifs = null;
      // TODO_RUST_CODE
      // Java: if ((poifs = this.isWpsSecurityFile2(path)) != null)
        // TODO_RUST_CODE
      {
         // Java: poifs.dispose();
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: POIFSFileSystem isWpsSecurityFile2(String path)
    pub fn isWpsSecurityFile2(&self, String path) -> POIFSFileSystem {
      // Java: RandomAccessFile raf = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: raf = new RandomAccessFile(new File(path), "r");
         // TODO_RUST_CODE
         // Java: this.mPOIFS = new POIFSFileSystem(raf);
         // TODO_RUST_CODE
         // Java: DirectoryNode rootStorage = this.mPOIFS.getRoot();
         // TODO_RUST_CODE
         // Java: if (null != rootStorage && rootStorage.hasEntry("WpsContent"))
            // TODO_RUST_CODE
         {
            // Java: return this.mPOIFS;
            // TODO_RUST_CODE
         }
      }
      // Java: catch (Throwable var4)
        // TODO_RUST_CODE
      {
         // Java: if (raf != null)
            // TODO_RUST_CODE
         {
            // Java: FileUtil.closeQuietly(raf);
            // TODO_RUST_CODE
            // Java: RandomAccessFile var5 = null;
            // TODO_RUST_CODE
         }
      }
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: boolean decrypt(String srcPath, String destPath, boolean skipOnlineExp)
    pub fn decrypt(&self, String srcPath, String destPath, boolean skipOnlineExp) -> bool {
      // Java: this.mSkipOnlineExp = skipOnlineExp;
      // TODO_RUST_CODE
      // Java: return this.decrypt(srcPath, destPath);
      // TODO_RUST_CODE
    }

    // Java: boolean decrypt(String srcPath, String destPath)
    pub fn decrypt(&self, String srcPath, String destPath) -> bool {
      // Java: if (!OnlineSecurityAPIHelper.checkIfCanUseOnlineSecurity())
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: POIFSFileSystem poifs = this.isWpsSecurityFile2(srcPath);
         // TODO_RUST_CODE
         // Java: if (null == poifs)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: Class<?>[] argTypes = new Class[]{POIFSFileSystem.class};
            // TODO_RUST_CODE
            // Java: Object[] argObjs = new Object[]{poifs};
            // TODO_RUST_CODE
            // Java: IWpsSecurityFile reader = null;
            // TODO_RUST_CODE
            // Java: try
                // TODO_RUST_CODE
            {
               // Java: reader = (IWpsSecurityFile)newInstance("cn.wps.pro.security.read.WpsSecurityFile", argTypes, argObjs);
               // TODO_RUST_CODE
               // Java: Assert.assertNotNull("reader should not be null", reader);
               // TODO_RUST_CODE
               // Java: byte[] buffer = null;
               // TODO_RUST_CODE
               // Java: try
                    // TODO_RUST_CODE
               {
                  // Java: buffer = getEvidenceData(this.getPOIFS());
                  // TODO_RUST_CODE
                  // Java: this.prepareForDecrypt(reader, buffer != null);
                  // TODO_RUST_CODE
               }
               // Java: catch (OnlineSecurityException var14)
                    // TODO_RUST_CODE
               {
                  // Java: Integer errorCode = var14.getSecurityErrorCode();
                  // TODO_RUST_CODE
                  // Java: var14.setDocGuid(this.mGuid);
                  // TODO_RUST_CODE
                  // Java: if (this.mSkipOnlineExp && (errorCode != null && (errorCode == 410001 || errorCode == 400003) || var14 instanceof PermissionDeniedException && "MSG_PERMISSION_DENIED_NEED_LOGIN".equals(var14.getMessage())))
                        // TODO_RUST_CODE
                  {
                     // Java: this.mPOIFS = null;
                     // TODO_RUST_CODE
                     // Java: boolean var10 = false;
                     // TODO_RUST_CODE
                     // Java: return var10;
                     // TODO_RUST_CODE
                  }
                  // Java: throw var14;
                  // TODO_RUST_CODE
               }
               // Java: if (this.isVersionTooOld())
                    // TODO_RUST_CODE
               {
                  // Java: throw new OnlineSecurityException(-2);
                  // TODO_RUST_CODE
               }
               // Java: else if (!this.canRead())
                    // TODO_RUST_CODE
               {
                  // Java: throw new PermissionDeniedException();
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: boolean result = reader.decryptFile(destPath, this.mSecretKey);
                  // TODO_RUST_CODE
                  // Java: if (result)
                        // TODO_RUST_CODE
                  {
                     // Java: this.enable();
                     // TODO_RUST_CODE
                     // Java: getApi().onOnlineSecurityDocOpened();
                     // TODO_RUST_CODE
                     // Java: this.mEvidenceData = buffer;
                     // TODO_RUST_CODE
                  }
                  // Java: boolean var17 = result;
                  // TODO_RUST_CODE
                  // Java: return var17;
                  // TODO_RUST_CODE
               }
            }
            // Java: finally
                // TODO_RUST_CODE
            {
               // Java: this.mSkipOnlineExp = false;
               // TODO_RUST_CODE
               // Java: reader.dispose();
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: boolean isVersionTooOld()
    pub fn isVersionTooOld(&self) -> bool {
      // Java: if (null == this.mVersions)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ArrayList<String> perms = this.mVersions.perm;
         // TODO_RUST_CODE
         // Java: ArrayList<String> trans = this.mVersions.trans;
         // TODO_RUST_CODE
         // Java: if (null != perms && null != trans)
            // TODO_RUST_CODE
         {
            // Java: if (!perms.contains("1.1"))
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
            // Java: else if (!trans.contains("2.2"))
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: return this.mVersionMajor < 1;
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void prepareForDecrypt(IWpsSecurityFile file, boolean forAllUser)
    pub fn prepareForDecrypt(&self, IWpsSecurityFile file, boolean forAllUser) {
      // Java: this.mVersionMajor = file.getVersionMajor();
      // TODO_RUST_CODE
      // Java: this.mVersionMinor = file.getVersionMinor();
      // TODO_RUST_CODE
      // Java: this.mGuid = file.getDoc_guid();
      // TODO_RUST_CODE
      // Java: this.mEncData = file.getEncdata();
      // TODO_RUST_CODE
      // Java: this.mSign = file.getDataSign();
      // TODO_RUST_CODE
      // Java: this.mVersions = getApi().versions();
      // TODO_RUST_CODE
      // Java: SecurityReadDocData result = getApi().readDoc(this.mGuid, this.mSign, this.mEncData, forAllUser);
      // TODO_RUST_CODE
      // Java: if (null == result)
        // TODO_RUST_CODE
      {
         // Java: throw new OnlineSecurityException("no response");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mSecretKey = result.docSecretKey;
         // TODO_RUST_CODE
         // Java: this.mRights = null == result.rights ? null : (SecurityRightData)result.rights.get(0);
         // TODO_RUST_CODE
         // Java: this.mPermission = PermissionUtil.parsePermission(this.mRights);
         // TODO_RUST_CODE
      }
    }

    // Java: void encrypt(String path, String srcPath, InputStream templateInput)
    pub fn encrypt(&self, String path, String srcPath, InputStream templateInput) {
      // Java: this.encrypt(path, srcPath, templateInput, false);
      // TODO_RUST_CODE
    }

    // Java: void encrypt(String path, String srcPath, InputStream templateInput, boolean forAllUser)
    pub fn encrypt(&self, String path, String srcPath, InputStream templateInput, boolean forAllUser) {
      // Java: if (!this.isEnable())
        // TODO_RUST_CODE
      {
         // Java: if (!FileUtil.moveFile(srcPath, path))
            // TODO_RUST_CODE
         {
            // Java: throw new IOException("move file failed");
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Class<?>[] argTypes = new Class[]{String.class, String.class, InputStream.class};
         // TODO_RUST_CODE
         // Java: Object[] argObjs = new Object[]{path, srcPath, templateInput};
         // TODO_RUST_CODE
         // Java: IWpsSecurityFileWrite writer = null;
         // TODO_RUST_CODE
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: writer = (IWpsSecurityFileWrite)newInstance("cn.wps.pro.security.write.WpsSecurityFileWrite", argTypes, argObjs);
            // TODO_RUST_CODE
            // Java: Assert.assertNotNull("writer should not be null", writer);
            // TODO_RUST_CODE
            // Java: this.mSecretKey = getRandomPassword();
            // TODO_RUST_CODE
            // Java: this.prepareForEncrypt(writer, forAllUser);
            // TODO_RUST_CODE
            // Java: writer.save(this.mSecretKey, this.mGuid, this.mEncData);
            // TODO_RUST_CODE
         }
         // Java: finally
            // TODO_RUST_CODE
         {
            // Java: writer.dispose();
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void prepareForEncrypt(IWpsSecurityFileWrite writer, boolean forAllUser)
    pub fn prepareForEncrypt(&self, IWpsSecurityFileWrite writer, boolean forAllUser) {
      // Java: if (null == this.mGuid)
        // TODO_RUST_CODE
      {
         // Java: this.mSign = writer.generateDataSign(this.mSecretKey);
         // TODO_RUST_CODE
         // Java: SecurityCreateDocData result = getApi().createDoc(this.mFileName, this.mSign, this.mSecretKey, (ArrayList)null, forAllUser);
         // TODO_RUST_CODE
         // Java: if (null == result)
            // TODO_RUST_CODE
         {
            // Java: throw new OnlineSecurityException("no response");
            // TODO_RUST_CODE
         }
         // Java: this.mGuid = result.docGuid;
         // TODO_RUST_CODE
         // Java: this.mEncData = result.encData;
         // TODO_RUST_CODE
         // Java: this.mPermission = 63;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: String newSign = writer.generateDataSign(this.mSecretKey);
         // TODO_RUST_CODE
         // Java: SecurityUpdateDocData result = getApi().updateDoc(this.mFileName, this.mGuid, this.mSign, this.mEncData, newSign, this.mSecretKey, (ArrayList)null, forAllUser);
         // TODO_RUST_CODE
         // Java: if (null == result)
            // TODO_RUST_CODE
         {
            // Java: throw new OnlineSecurityException("no response");
            // TODO_RUST_CODE
         }
         // Java: this.mSign = newSign;
         // TODO_RUST_CODE
         // Java: this.mEncData = result.encData;
         // TODO_RUST_CODE
      }
    }

    // Java: void checkOperation(String opId)
    pub fn checkOperation(&self, String opId) {
      // Java: this.checkOperation(opId, false);
      // TODO_RUST_CODE
    }

    // Java: void checkOperation(String opId, boolean forAllUser)
    pub fn checkOperation(&self, String opId, boolean forAllUser) {
      // Java: if (forAllUser && opId != null)
        // TODO_RUST_CODE
      {
         // Java: if ("7".equals(opId) && this.canPrint())
            // TODO_RUST_CODE
         {
            // Java: return;
            // TODO_RUST_CODE
         }
         // Java: if ("6".equals(opId) && this.canCopy())
            // TODO_RUST_CODE
         {
            // Java: return;
            // TODO_RUST_CODE
         }
         // Java: if ("5".equals(opId) && this.canSaveas())
            // TODO_RUST_CODE
         {
            // Java: return;
            // TODO_RUST_CODE
         }
      }
      // Java: if (null == getApi())
        // TODO_RUST_CODE
      {
         // Java: throw new OnlineSecurityException();
         // TODO_RUST_CODE
      }
      // Java: else if (null == this.mGuid)
        // TODO_RUST_CODE
      {
         // Java: throw new OnlineSecurityException();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: getApi().checkOperation(this.mGuid, opId);
         // TODO_RUST_CODE
      }
    }

    // Java: byte[] getEvidenceData()
    pub fn getEvidenceData(&self) -> byte[] {
      // Java: return this.mEvidenceData;
      // TODO_RUST_CODE
    }

    // Java: void setEvidenceData(byte[] evidenceData)
    pub fn setEvidenceData(&self, byte[] evidenceData) {
      // Java: this.mEvidenceData = evidenceData;
      // TODO_RUST_CODE
    }

    // Java: String getGuid()
    pub fn getGuid(&self) -> String {
      // Java: return this.mGuid;
      // TODO_RUST_CODE
    }

    // Java: void enable()
    pub fn enable(&self) {
      // Java: this.mEnable = true;
      // TODO_RUST_CODE
    }

    // Java: void disable()
    pub fn disable(&self) {
      // Java: this.mEnable = false;
      // TODO_RUST_CODE
      // Java: this.clear();
      // TODO_RUST_CODE
    }

    // Java: boolean isEnable()
    pub fn isEnable(&self) -> bool {
      // Java: return this.mEnable;
      // TODO_RUST_CODE
    }

    // Java: void setFileName(String fileName)
    pub fn setFileName(&self, String fileName) {
      // Java: this.mFileName = fileName;
      // TODO_RUST_CODE
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&self) -> i32 {
      // Java: return this.mVersionMajor;
      // TODO_RUST_CODE
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&self) -> i32 {
      // Java: return this.mVersionMinor;
      // TODO_RUST_CODE
    }

    // Java: boolean canRead()
    pub fn canRead(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 1) != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean canEdit()
    pub fn canEdit(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 2) != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean canSaveas()
    pub fn canSaveas(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 4) != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean canAuthorize()
    pub fn canAuthorize(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 32) != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean canShowSaveas()
    pub fn canShowSaveas(&self) -> bool {
      // Java: return !this.isEnable();
      // TODO_RUST_CODE
    }

    // Java: boolean canCopy()
    pub fn canCopy(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 8) != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean canPrint()
    pub fn canPrint(&self) -> bool {
      // Java: return !this.isEnable() || (this.mPermission & 16) != 0;
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mFileName = null;
      // TODO_RUST_CODE
      // Java: this.mGuid = null;
      // TODO_RUST_CODE
      // Java: this.mSecretKey = null;
      // TODO_RUST_CODE
      // Java: this.mSign = null;
      // TODO_RUST_CODE
      // Java: this.mEncData = null;
      // TODO_RUST_CODE
      // Java: this.mPermission = 0;
      // TODO_RUST_CODE
      // Java: this.mRights = null;
      // TODO_RUST_CODE
    }

    // Java: void copyFrom(OnlineSecurityTool other)
    pub fn copyFrom(&self, OnlineSecurityTool other) {
      // Java: this.mFileName = other.mFileName;
      // TODO_RUST_CODE
      // Java: this.mGuid = other.mGuid;
      // TODO_RUST_CODE
      // Java: this.mSecretKey = other.mSecretKey;
      // TODO_RUST_CODE
      // Java: this.mSign = other.mSign;
      // TODO_RUST_CODE
      // Java: this.mEncData = other.mEncData;
      // TODO_RUST_CODE
      // Java: this.mPermission = other.mPermission;
      // TODO_RUST_CODE
      // Java: this.mRights = other.mRights;
      // TODO_RUST_CODE
      // Java: this.mEnable = other.mEnable;
      // TODO_RUST_CODE
    }

    // Java: static String getRandomPassword()
    pub fn getRandomPassword(&self) -> String {
      // Java: return genRandomNum(32);
      // TODO_RUST_CODE
    }

    // Java: static String genRandomNum(int pwd_len)
    pub fn genRandomNum(&self, int pwd_len) -> String {
      // Java: int maxNum = 16;
      // TODO_RUST_CODE
      // Java: int count = 0;
      // TODO_RUST_CODE
      // Java: char[] str = new char[]{'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};
      // TODO_RUST_CODE
      // Java: StringBuilder pwd = new StringBuilder(pwd_len);
      // TODO_RUST_CODE
      // Java: while(count < pwd_len)
        // TODO_RUST_CODE
      {
         // Java: int i = Math.abs(random.nextInt(16));
         // TODO_RUST_CODE
         // Java: if (i >= 0 && i < str.length)
            // TODO_RUST_CODE
         {
            // Java: pwd.append(str[i]);
            // TODO_RUST_CODE
            // Java: ++count;
            // TODO_RUST_CODE
         }
      }
      // Java: return pwd.toString();
      // TODO_RUST_CODE
    }

    // Java: static Object newInstance(String classFullName, Class<?>[] argTypes, Object[] argObjs)
    pub fn newInstance(&self, String classFullName, Class<?>[] argTypes, Object[] argObjs) -> Object {
      // Java: ClassLoader cl = null;
      // TODO_RUST_CODE
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
        // TODO_RUST_CODE
      {
         // Java: cl = IClassLoaderManager.getInstance().getSecureDocClassLoader();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: cl = OnlineSecurityTool.class.getClassLoader();
         // TODO_RUST_CODE
      }
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: Class<?> clazz = cl.loadClass(classFullName);
         // TODO_RUST_CODE
         // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
         // TODO_RUST_CODE
         // Java: return constructor.newInstance(argObjs);
         // TODO_RUST_CODE
      }
      // Java: catch (ClassNotFoundException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (InstantiationException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalAccessException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (NoSuchMethodException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (SecurityException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalArgumentException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: catch (InvocationTargetException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "", e);
         // TODO_RUST_CODE
      }
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: POIFSFileSystem getPOIFS()
    pub fn getPOIFS(&self) -> POIFSFileSystem {
      // Java: return this.mPOIFS;
      // TODO_RUST_CODE
    }

    // Java: static IOnlineSecurityAPI getApi()
    pub fn getApi(&self) -> IOnlineSecurityAPI {
      // Java: if (null == smApi)
        // TODO_RUST_CODE
      {
         // Java: smApi = OnlineSecurityAPIHelper.getImpl();
         // TODO_RUST_CODE
      }
      // Java: return smApi;
      // TODO_RUST_CODE
    }

    // Java: static byte[] getEvidenceData(POIFSFileSystem document)
    pub fn getEvidenceData(&self, POIFSFileSystem document) -> byte[] {
      // Java: if (document == null)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: DirectoryNode rootStorage = document.getRoot();
         // TODO_RUST_CODE
         // Java: if (rootStorage == null)
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: else if (!rootStorage.hasEntry("WpsEvidenceData"))
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: DocumentInputStream is = null;
            // TODO_RUST_CODE
            // Java: try
                // TODO_RUST_CODE
            {
               // Java: is = rootStorage.createDocumentInputStream("WpsEvidenceData");
               // TODO_RUST_CODE
               // Java: if (is != null)
                    // TODO_RUST_CODE
               {
                  // Java: int len = is.available();
                  // TODO_RUST_CODE
                  // Java: byte[] buffer = new byte[len];
                  // TODO_RUST_CODE
                  // Java: is.read(buffer, 0, len);
                  // TODO_RUST_CODE
                  // Java: return buffer;
                  // TODO_RUST_CODE
               }
            }
            // Java: catch (IOException e)
                // TODO_RUST_CODE
            {
               // Java: e.printStackTrace();
               // TODO_RUST_CODE
            }
            // Java: return null;
            // TODO_RUST_CODE
         }
      }
    }

}

impl IOnlineSecurityTool for OnlineSecurityTool {
}

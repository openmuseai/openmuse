// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.io.file.Tools
// Java: import cn.wps.io.opc.OPCPackage
use io_wps_io::io::opc::OPCPackage::*;
// Java: import cn.wps.io.opc.PackageAccess
use io_wps_io::io::opc::PackageAccess::*;
// Java: import cn.wps.io.opc.PackagePart
use io_wps_io::io::opc::PackagePart::*;
// Java: import cn.wps.io.uof.reader.UofrPackage
use io_uof_io::cn::wps::io::uof::reader::UofrPackage::*;
// Java: import cn.wps.io.xwpf.XWPFPackage
use io_wps_io::io::xwpf::XWPFPackage::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
use io_wps_io::io::xwpf::usermodel::XWPFDocument::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use io_wps_io::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::FileInputStream::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.OleFileUtil
// rust: ★M-STAGE2 P1-D1 / T0-2 —— `OleFileUtil`（176 行）自身尚未编译通过：
//   含就地影子 `pub struct OlePackageCache;`（假编译面）、缺失 `StringUtil` 导入、
//   以及 `MediaLib::getMediaByID` 等方法缺口。本类为**非管线**（语料 0 处 OLE），
//   其唯一使用点 `dispose` 已合法延迟 ⇒ 导入一并延迟。
// rust: no-stmt
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.concurrent.Callable
use stubs::java::util::concurrent::Callable::*;
// Java: import java.util.concurrent.FutureTask
use stubs::java::util::concurrent::FutureTask::*;
// Java: import java.util.zip.ZipError
use stubs::java::util::zip::ZipError::*;
// Java: import org.apache.poi.hwpf.HWPFDocument
use io_poi_compact::org::apache::poi::hwpf::HWPFDocument::*;
// Java: import org.apache.poi.hwpf.ole.DocOLEInfo
use io_poi_compact::org::apache::poi::hwpf::ole::DocOLEInfo::*;
// Java: import org.apache.poi.hwpf.ole.DocxOleInfo
use io_poi_compact::org::apache::poi::hwpf::ole::DocxOleInfo::*;
// Java: import org.apache.poi.hwpf.ole.OleInfo
use io_poi_compact::org::apache::poi::hwpf::ole::OleInfo::*;
// Java: import org.apache.poi.hwpf.ole.UotOleInfo
use io_poi_compact::org::apache::poi::hwpf::ole::UotOleInfo::*;
// Java: import org.apache.poi.hwpf.ole.manager.IOleManager
use io_poi_compact::org::apache::poi::hwpf::ole::manager::IOleManager::*;
// Java: import org.apache.poi.hwpf.ole.stream.OlePackageCache
use io_poi_compact::org::apache::poi::hwpf::ole::stream::OlePackageCache::OlePackageCache;
// Java: import org.apache.poi.hwpf.ole.unpacker.IOleUnpacker
use io_poi_compact::org::apache::poi::hwpf::ole::unpacker::IOleUnpacker::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.OleUnpackerFactory
use io_poi_compact::org::apache::poi::hwpf::ole::unpacker::OleUnpackerFactory::*;
// Java: import org.apache.poi.hwpf.util.EncryptedOLEDumper
// rust: ★M-STAGE2 P1-D1 / T0-2 —— `EncryptedOLEDumper` 是**真实存在但未移植完成**的模块：
//   登记后暴露 14 处模板破坏错误，且依赖 poi-compact 中尚未移植的 `model::FOBJH`
//   与缺失方法 `DocumentInputStream::read__bytearr`。本类为**非管线**（语料 0 处 OLE），
//   其唯一使用点 `getEncryptBin` 已合法延迟 ⇒ 导入一并延迟。
// rust: no-stmt
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use io_poi_compact::org::apache::poi::hwpf::util::OleParseInterruptException::*;
// Java: import org.apache.poi.hwpf.util.StorageUtil
use io_poi_compact::org::apache::poi::hwpf::util::StorageUtil::*;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem::*;
use std::collections::{HashMap};

// java private static final String TAG = null;
const TAG: &str = "";
// java private static final boolean DEBUG = false;
const DEBUG: bool = false;
// java private static final String OBJ_POOL = "ObjectPool";
const OBJ_POOL: &str = "ObjectPool";
// java private static final int INITIAL_INDEX = 0;
const INITIAL_INDEX: i32 = 0;

// java public class OleManager implements IOleManager
#[derive(Default)]
pub struct OleManager {
    // java private OlePackageCache mOLePackageCache;
    mOLePackageCache: Option<OlePackageCache>,
    // java private int mOleIndex = 0;
    mOleIndex: i32,
    // java private HashMap<Integer, OleInfo> mDocOLEInfoMap;
    mDocOLEInfoMap: Option<HashMap<i32, OleInfo>>,
    // java private String mFilePath;
    mFilePath: Option<String>,
    // java private RandomAccessFile mRaf;
    mRaf: Option<RandomAccessFile>,
    // java private String mPassword;
    mPassword: Option<String>,
    // java private IMediaLib mMediaLib;
    mMediaLib: Option<Box<dyn IMediaLib>>,
    // java private XWPFPackage mDocxPackage;
    mDocxPackage: Option<XWPFPackage>,
    // java private HWPFDocument mDocEncryptDoc;
    mDocEncryptDoc: Option<HWPFDocument>,
    // java private DirectoryNode mDocRootNode;
    mDocRootNode: Option<DirectoryNode>,
    // java private ArrayList<Integer> mErrorList;
    mErrorList: Option<Vec<i32>>,
    // java private FutureTask<Boolean> mParseOleFuture = null;
    mParseOleFuture: Option<FutureTask>,
    // java private String mRootDir;
    mRootDir: Option<String>,
    // java private UofrPackage mUofrPackage;
    mUofrPackage: Option<UofrPackage>,
}

// java private class ParseOleTask implements Callable<Boolean>
#[derive(Default)]
pub struct ParseOleTask {
    // java private final OLE ole;
    ole: Option<OLE>,
    // java private final String targetFile;
    targetFile: Option<String>,
    // java private final Callback callback;
    callback: Option<Box<dyn Callback>>,
}

// java public interface Callback
// rust: ★M-STAGE2 P1-D1 / T0-2 —— `OleManager.java:646` 的内部 `interface Callback`，
//   模板产物遗漏 ⇒ 按 Java 原文补全门面（3 个抽象方法）。
pub trait Callback {
    // java void onParseComplete(ResultType var1)
    fn onParseComplete(&mut self, var1: &ResultType);

    // java void proxyThreadExecute(FutureTask<Boolean> var1)
    fn proxyThreadExecute(&mut self, var1: &mut FutureTask);

    // java void proxyUiExecute(Runnable var1)
    fn proxyUiExecute(&mut self, var1: &mut Box<dyn Fn()>);
}

// java public static enum ResultType
// rust: ★M-STAGE2 P1-D1 / T0-2 —— 按 `OleManager.java:633` 补全枚举常量与构造器门面
//   （模板产物遗漏了 4 个常量与私有 ctor ⇒ 触发 §3.8 coverage/completeness FAIL）。
#[derive(Copy, Clone, Debug)]
pub enum ResultType {
    // java OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR(0),
    OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR,
    // java OLE_PARSE_RESULT_TYPE_SUCCESS(1),
    OLE_PARSE_RESULT_TYPE_SUCCESS,
    // java OLE_PARSE_RESULT_TYPE_FAIL(2),
    OLE_PARSE_RESULT_TYPE_FAIL,
    // java OLE_PARSE_RESULT_TYPE_INTERRUPT(3);
    OLE_PARSE_RESULT_TYPE_INTERRUPT,
}

// java public static enum ResultType
impl ResultType {
    // java private ResultType(int value)
    pub fn ResultType(value: i32) -> Self {
      // java this.mValue = value;
      // rust: Java 枚举常量在构造时绑定 `value`（0..3）；Rust 变体即常量 ⇒ 按序号映射。
      return match value {
        0 => ResultType::OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR,
        1 => ResultType::OLE_PARSE_RESULT_TYPE_SUCCESS,
        2 => ResultType::OLE_PARSE_RESULT_TYPE_FAIL,
        _ => ResultType::OLE_PARSE_RESULT_TYPE_INTERRUPT,
      };
    }

    // java private final int mValue;
    // rust: 读取 Java 枚举常量的 `mValue` 载荷（门面与 Java 字段名一致）。
    pub fn mValue(&self) -> i32 {
        match self {
            ResultType::OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR => 0,
            ResultType::OLE_PARSE_RESULT_TYPE_SUCCESS => 1,
            ResultType::OLE_PARSE_RESULT_TYPE_FAIL => 2,
            ResultType::OLE_PARSE_RESULT_TYPE_INTERRUPT => 3,
        }
    }
}

// java public interface IOleManager
impl IOleManager for OleManager {
    // java public String getOleBinPath(int mediaId)
    fn getOleBinPath(&self, mediaId: i32) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java String originPath = this.mMediaLib.getMediaByID(mediaId, MediaTypeEnum.OLE);
            let mut originPath: String = self.mMediaLib.getMediaByID(mediaId, &MediaTypeEnum::OLE);
            // java if (null == originPath)
            if originPath.is_none() {
              // java return null;
              return None;
            }
            // java else
            else {
              // java File originFile = new File(originPath);
              let mut originFile: std::path::PathBuf = File::new(originPath);
              // java if (originFile.exists() && originFile.length() > 0L)
              if originFile.exists() && (originFile.len() as i32) > 0 {
                // java return originPath;
                return originPath;
              }
              // java else if (null != this.mDocOLEInfoMap && null != this.mDocOLEInfoMap.get(mediaId))
              else if let Some(mDocOLEInfoMap) = self.mDocOLEInfoMap.as_ref() {
                if self.mDocOLEInfoMap.get(mediaId).is_some() {
                  // java if (!originFile.exists())
                  if !originFile.exists() {
                        // java FileUtil.newFile(originPath);
                        FileUtil::newFile__string(Some(&originPath));
                        // java catch (IOException var7)
                        todo!();
                          // java return null;
                          return None;
                  }
                  // java OleInfo oleInfo = (OleInfo)this.mDocOLEInfoMap.get(mediaId);
                  let mut oleInfo: OleInfo = (mDocOLEInfoMap.get(mediaId)).downcast_ref::<OleInfo>().unwrap();
                  // java if (oleInfo instanceof DocOLEInfo)
                  if oleInfo.downcast_ref::<DocOLEInfo>().is_some() {
                    // java DocOLEInfo docOleInfo = (DocOLEInfo)oleInfo;
                    let mut docOleInfo: DocOLEInfo = (oleInfo).downcast_ref::<DocOLEInfo>().unwrap();
                    // java if (!docOleInfo.isEncrypt())
                    if !docOleInfo.isEncrypt() {
                      // java return this.getDocBin(docOleInfo, originPath);
                      return self.getDocBin(&docOleInfo, &originPath);
                    }
                    // java else
                    else {
                      // java String result = this.getDocBin(docOleInfo, originPath);
                      let mut result: String = self.getDocBin(&docOleInfo, &originPath);
                      // java return null != result ? result : this.getEncryptBin(docOleInfo, originPath);
                      return (if result.is_some() { Some(result) } else { Some(self.getEncryptBin(Some(&docOleInfo), &originPath)) });
                    }
                  }
                  // java else if (oleInfo instanceof DocxOleInfo)
                  else if oleInfo.downcast_ref::<DocxOleInfo>().is_some() {
                    // java DocxOleInfo docxOleInfo = (DocxOleInfo)oleInfo;
                    let mut docxOleInfo: DocxOleInfo = (oleInfo).downcast_ref::<DocxOleInfo>().unwrap();
                    // java return this.getDocxBin(docxOleInfo, originPath);
                    return self.getDocxBin(&docxOleInfo, &originPath);
                  }
                  // java else if (oleInfo instanceof UotOleInfo)
                  else if oleInfo.downcast_ref::<UotOleInfo>().is_some() {
                    // java UotOleInfo uotOleInfo = (UotOleInfo)oleInfo;
                    let mut uotOleInfo: UotOleInfo = (oleInfo).downcast_ref::<UotOleInfo>().unwrap();
                    // java return this.getUotBin(uotOleInfo, originPath);
                    return self.getUotBin(&uotOleInfo, &originPath);
                  }
                  // java else
                  else {
                    // java return null;
                    return None;
                  }
                }
              }
              // java else
              else {
                // java return null;
                return None;
              }
            }
      */
    }

}

// java public class OleManager implements IOleManager
impl OleManager {
    // java private  OleManager(String filePath, String password, IMediaLib mediaLib, String rootDir)
    pub fn OleManager__string_string_imedialib_string(filePath: &str, password: &str, mediaLib: &dyn IMediaLib, rootDir: &str) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mFilePath = filePath;
            let mut this = Self { mOLePackageCache: Default::default(), mOleIndex: Default::default(), mDocOLEInfoMap: Default::default(), mFilePath: filePath, mRaf: Default::default(), mPassword: password, mMediaLib: mediaLib, mDocxPackage: Default::default(), mDocEncryptDoc: Default::default(), mDocRootNode: Default::default(), mErrorList: Default::default(), mParseOleFuture: Default::default(), mRootDir: rootDir, mUofrPackage: Default::default() };
            // java this.mRootDir = rootDir;
            // java this.init(password, mediaLib);
            this.init(&password, &mediaLib);
            this
      */
    }

    // java private  OleManager(RandomAccessFile raf, String password, IMediaLib mediaLib, String rootDir)
    pub fn OleManager__randomaccessfile_string_imedialib_string(raf: &RandomAccessFile, password: &str, mediaLib: &dyn IMediaLib, rootDir: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mRaf = raf;
            let mut this = Self { mOLePackageCache: Default::default(), mOleIndex: Default::default(), mDocOLEInfoMap: Default::default(), mFilePath: Default::default(), mRaf: raf, mPassword: password, mMediaLib: mediaLib, mDocxPackage: Default::default(), mDocEncryptDoc: Default::default(), mDocRootNode: Default::default(), mErrorList: Default::default(), mParseOleFuture: Default::default(), mRootDir: rootDir, mUofrPackage: Default::default() };
            // java this.mRootDir = rootDir;
            // java this.init(password, mediaLib);
            this.init(&password, &mediaLib);
            this
      */
    }

    // java private void init(String password, IMediaLib mediaLib)
    pub fn init(&mut self, password: &str, mediaLib: &dyn IMediaLib) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mPassword = password;
            self.mPassword = password;
            // java this.mMediaLib = mediaLib;
            self.mMediaLib = mediaLib;
      */
    }

    // java public static OleManager create(String filePath, String password, IMediaLib mediaLib, String rootDir)
    pub fn create__string_string_imedialib_string(filePath: &str, password: &str, mediaLib: &dyn IMediaLib, rootDir: &str) -> OleManager {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java return new OleManager(filePath, password, mediaLib, rootDir);
            return OleManager::new(filePath, password, mediaLib, rootDir);
      */
    }

    // java public static OleManager create(RandomAccessFile raf, String password, IMediaLib mediaLib, String rootDir)
    pub fn create__randomaccessfile_string_imedialib_string(raf: &RandomAccessFile, password: &str, mediaLib: &dyn IMediaLib, rootDir: &str) -> OleManager {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java return new OleManager(raf, password, mediaLib, rootDir);
            return OleManager::new(raf, password, mediaLib, rootDir);
      */
    }

    // java public void setPath(String filePath)
    pub fn setPath(&mut self, filePath: Option<&str>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != filePath)
            if let Some(filePath) = filePath {
              // java this.mFilePath = filePath;
              self.mFilePath = filePath;
            }
      */
    }

    // java public void setTmpRootDir(String rootDir)
    pub fn setTmpRootDir(&mut self, rootDir: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mRootDir = rootDir;
            self.mRootDir = rootDir;
      */
    }

    // java public void setRaf(RandomAccessFile raf)
    pub fn setRaf(&mut self, raf: &RandomAccessFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mRaf = raf;
            self.mRaf = raf;
      */
    }

    // java public void setPassword(String password)
    pub fn setPassword(&mut self, password: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.mPassword = password;
            self.mPassword = password;
      */
    }

    // java public synchronized OlePackageCache getOleCache()
    pub fn getOleCache(&mut self) -> OlePackageCache {
      // java if (this.mOLePackageCache == null)
      if self.mOLePackageCache.is_none() {
        // java this.mOLePackageCache = OlePackageCache.create();
        self.mOLePackageCache = Some(OlePackageCache::create());
      }
      // java return this.mOLePackageCache;
      // rust: Java 字段为可空引用，上方 `if` 分支后已保证非空 ⇒ 忠实返回其值。
      return self.mOLePackageCache.clone().unwrap_or_default();
    }

    // java public void initDocDocument(HWPFDocument hwpfDocument)
    pub fn initDocDocument(&mut self, hwpfDocument: &HWPFDocument) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (StringUtil.isEmpty(this.mPassword))
            if StringUtil::isEmpty(Some(&mut self.mPassword)) {
              // java this.mDocRootNode = hwpfDocument.getRoot();
              self.mDocRootNode = hwpfDocument.getRoot();
            }
            // java else
            else {
              // java this.mDocEncryptDoc = hwpfDocument;
              self.mDocEncryptDoc = hwpfDocument;
            }
      */
    }

    // java public void resetDocDocument()
    pub fn resetDocDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != this.mDocRootNode)
            if let Some(mDocRootNode) = self.mDocRootNode.as_mut() {
              // java this.mDocRootNode = null;
              mDocRootNode = None;
            }
            // java if (null != this.mDocEncryptDoc)
            if let Some(mDocEncryptDoc) = self.mDocEncryptDoc.as_mut() {
              // java this.mDocEncryptDoc = null;
              mDocEncryptDoc = None;
            }
      */
    }

    // java public void initDocxDocument(XWPFDocument xwpfDocument)
    pub fn initDocxDocument(&mut self, xwpfDocument: Option<&XWPFDocument>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != xwpfDocument)
            if let Some(xwpfDocument) = xwpfDocument {
              // java OPCPackage opcPackage = xwpfDocument.getPackage();
              let mut opcPackage: OPCPackage = xwpfDocument.getPackage();
              // java if (null != opcPackage && opcPackage instanceof XWPFPackage)
              if opcPackage.is_some() && opcPackage.downcast_ref::<XWPFPackage>().is_some() {
                // java this.mDocxPackage = (XWPFPackage)opcPackage;
                self.mDocxPackage = (opcPackage).downcast_ref::<XWPFPackage>().unwrap();
              }
            }
      */
    }

    // java public void initUot3Document(UofrPackage uofrPackage)
    pub fn initUot3Document(&mut self, uofrPackage: Option<&UofrPackage>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != uofrPackage)
            if let Some(uofrPackage) = uofrPackage {
              // java this.mUofrPackage = uofrPackage;
              self.mUofrPackage = uofrPackage;
            }
      */
    }

    // java public void resetDocxDocument()
    pub fn resetDocxDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != this.mDocxPackage)
            if let Some(mDocxPackage) = self.mDocxPackage.as_mut() {
              // java this.mDocxPackage = null;
              mDocxPackage = None;
            }
      */
    }

    // java public void resetUotDocument()
    pub fn resetUotDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != this.mUofrPackage)
            if let Some(mUofrPackage) = self.mUofrPackage.as_mut() {
              // java this.mUofrPackage = null;
              mUofrPackage = None;
            }
      */
    }

    // java public void resetDocument()
    pub fn resetDocument(&mut self) {
      // java this.resetDocDocument();
      self.resetDocDocument();
      // java this.resetDocxDocument();
      self.resetDocxDocument();
      // java this.resetUotDocument();
      self.resetUotDocument();
    }

    // java public int nextOleIndex()
    pub fn nextOleIndex(&mut self) -> i32 {
      // java return this.mOleIndex++;
      return { let _tmp = self.mOleIndex; self.mOleIndex += 1; _tmp };
    }

    // java public void recycleOleIndex()
    pub fn recycleOleIndex(&mut self) {
      // java --this.mOleIndex;
      { let _tmp = self.mOleIndex; self.mOleIndex -= 1; _tmp };
    }

    // java public void dispose(String mainDocPath, RandomAccessFile raf)
    pub fn dispose(&mut self, mainDocPath: &str, raf: &RandomAccessFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.cancelOleParseTask();
            self.cancelOleParseTask();
            // java if (this.mOLePackageCache != null)
            if let Some(mOLePackageCache) = self.mOLePackageCache.as_mut() {
              // java this.mOLePackageCache.dispose();
              mOLePackageCache.dispose();
              // java this.mOLePackageCache = null;
              mOLePackageCache = None;
            }
            // java if (!StringUtil.isEmpty(mainDocPath))
            if !StringUtil::isEmpty(Some(&mut mainDocPath)) {
              // java String olePath = OleFileUtil.getOlePath(this.mRootDir, mainDocPath, raf);
              let mut olePath: String = OleFileUtil::getOlePath(&self.mRootDir, &mainDocPath, Some(&raf));
              // java if (!StringUtil.isEmpty(olePath))
              if !StringUtil::isEmpty(Some(&mut olePath)) {
                // java FileUtil.deleteDirFiles(new File(olePath));
                FileUtil::deleteDirFiles__filearr(Some(&File::new(olePath)));
              }
            }
            // java if (this.mDocOLEInfoMap != null)
            if let Some(mDocOLEInfoMap) = self.mDocOLEInfoMap.as_mut() {
              // java this.mDocOLEInfoMap.clear();
              mDocOLEInfoMap.clear();
              // java this.mDocOLEInfoMap = null;
              mDocOLEInfoMap = None;
            }
            // java if (null != this.mErrorList)
            if let Some(mErrorList) = self.mErrorList.as_mut() {
              // java this.mErrorList.clear();
              mErrorList.clear();
              // java this.mErrorList = null;
              mErrorList = None;
            }
            // java if (null != this.mDocxPackage)
            if let Some(mDocxPackage) = self.mDocxPackage.as_mut() {
              // java this.mDocxPackage = null;
              mDocxPackage = None;
            }
            // java if (null != this.mDocRootNode)
            if let Some(mDocRootNode) = self.mDocRootNode.as_mut() {
              // java this.mDocRootNode.dispose();
              mDocRootNode.dispose();
              // java this.mDocRootNode = null;
              mDocRootNode = None;
            }
            // java if (null != this.mDocEncryptDoc)
            if let Some(mDocEncryptDoc) = self.mDocEncryptDoc.as_mut() {
              // java this.mDocEncryptDoc.dispose();
              mDocEncryptDoc.dispose();
              // java this.mDocEncryptDoc = null;
              mDocEncryptDoc = None;
            }
            // java if (null != this.mUofrPackage)
            if let Some(mUofrPackage) = self.mUofrPackage.as_mut() {
              // java this.mUofrPackage = null;
              mUofrPackage = None;
            }
            // java this.mOleIndex = 0;
            self.mOleIndex = 0;
      */
    }

    // java public void addOle(int mediaId, OleInfo docOleInfo)
    pub fn addOle(&mut self, mediaId: i32, docOleInfo: &OleInfo) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null == this.mDocOLEInfoMap)
            if self.mDocOLEInfoMap.is_none() {
              // java this.mDocOLEInfoMap = new HashMap();
              self.mDocOLEInfoMap = HashMap::new();
            }
            // java this.mDocOLEInfoMap.put(mediaId, docOleInfo);
            self.mDocOLEInfoMap.put(mediaId, docOleInfo);
      */
    }

    // java public void recordError(int mediaId)
    pub fn recordError(&mut self, mediaId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null == this.mErrorList)
            if self.mErrorList.is_none() {
              // java this.mErrorList = new ArrayList();
              self.mErrorList = Vec::new();
            }
            // java if (!this.mErrorList.contains(mediaId))
            if !self.mErrorList.contains(mediaId) {
              // java this.mErrorList.add(mediaId);
              self.mErrorList.add(mediaId);
            }
      */
    }

    // java private XWPFPackage initZip(String path, RandomAccessFile raf)
    pub fn initZip(&mut self, path: &str, raf: Option<&RandomAccessFile>) -> XWPFPackage {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java XWPFPackage zipPackage = null;
            let mut zipPackage: XWPFPackage = XWPFPackage::default();
                // java if (null != raf)
                if let Some(raf) = raf {
                  // java zipPackage = XWPFPackage.open(raf, PackageAccess.READ);
                  zipPackage = XWPFPackage::open__string_packageaccess(Some(&mut raf), &PackageAccess::READ);
                }
                // java else
                else {
                  // java zipPackage = XWPFPackage.open(path, PackageAccess.READ);
                  zipPackage = XWPFPackage::open__string_packageaccess(Some(&mut path), &PackageAccess::READ);
                }
                // java catch (InvalidFormatException e)
                todo!();
                  // java IOLog.d(TAG, "initZip failed", e);
                  IOLog::d__string_string_throwable(&TAG, &"initZip failed", &e);
                // java catch (IOException e)
                todo!();
                  // java IOLog.d(TAG, "IOException", e);
                  IOLog::d__string_string_throwable(&TAG, &"IOException", &e);
                // java catch (ZipError var9)
                todo!();
                      // java XWPFPackage opcPackage = null;
                      let mut opcPackage: XWPFPackage = XWPFPackage::default();
                        // java (null != raf)
                        raf.is_some();
                          // java opcPackage = XWPFPackage.open(raf, PackageAccess.READ);
                          opcPackage = XWPFPackage::open__string_packageaccess(Some(&mut raf), &PackageAccess::READ);
                          // java opcPackage = XWPFPackage.open(new FileInputStream(path));
                          opcPackage = XWPFPackage::open__inputstream(&FileInputStream::new(path));
                        // java (null != opcPackage && opcPackage instanceof XWPFPackage)
                        opcPackage.is_some() && opcPackage.downcast_ref::<XWPFPackage>().is_some();
                          // java this.mDocxPackage = opcPackage;
                          self.mDocxPackage = opcPackage;
                    // java catch (Exception e) { IOLog.d(TAG, "retry initZip failed", e);
                    IOLog::d__string_string_throwable(&TAG, &"retry initZip failed", &e);
            // java return zipPackage;
            return zipPackage;
      */
    }

    // java private String getDocxBin(DocxOleInfo docxOleInfo, String filePath)
    pub fn getDocxBin(&mut self, docxOleInfo: &DocxOleInfo, filePath: &str) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java String targetPartName = docxOleInfo.getPartName();
            let mut targetPartName: String = docxOleInfo.getPartName();
            // java if (!StringUtil.isEmpty(targetPartName) && !StringUtil.isEmpty(filePath))
            if !StringUtil::isEmpty(Some(&mut targetPartName)) && !StringUtil::isEmpty(Some(&mut filePath)) {
              // java if (null == this.mDocxPackage)
              if self.mDocxPackage.is_none() {
                // java this.mDocxPackage = this.initZip(this.mFilePath, this.mRaf);
                self.mDocxPackage = self.initZip(&self.mFilePath, Some(&self.mRaf));
              }
              // java if (null == this.mDocxPackage)
              if self.mDocxPackage.is_none() {
                // java return null;
                return None;
              }
              // java else
              else {
                // java ArrayList<PackagePart> partList = null;
                let mut partList: Vec<PackagePart> = vec![];
                    // java partList = this.mDocxPackage.getParts();
                    partList = self.mDocxPackage.getParts();
                    // java if (null != partList)
                    if partList.is_some() {
                      let mut index: i32 = 0;
                      // java for(int index = 0; index < partList.size(); ++index)
                      while index < (partList.len() as i32) {
                        // java PackagePart packagePart = (PackagePart)partList.get(index);
                        let mut packagePart: PackagePart = (partList.get(index)).downcast_ref::<PackagePart>().unwrap();
                        // java if (targetPartName.equals(packagePart.getPartName().getName()))
                        if targetPartName.equals(packagePart.getPartName().getName()) {
                          // java StorageUtil.writeToFile(filePath, packagePart.getInputStream());
                          StorageUtil::writeToFile__string_inputstream(&filePath, &packagePart.getInputStream());
                          // java return filePath;
                          return filePath;
                        }
                      }
                    }
                    // java catch (InvalidFormatException e)
                    todo!();
                      // java IOLog.e(TAG, "getDocxBin failed", e);
                      IOLog::e__string_string_throwable(&TAG, &"getDocxBin failed", &e);
                    // java catch (IOException e)
                    todo!();
                      // java IOLog.e(TAG, "getDocxBin failed", e);
                      IOLog::e__string_string_throwable(&TAG, &"getDocxBin failed", &e);
                // java return null;
                return None;
              }
            }
            // java else
            else {
              // java return null;
              return None;
            }
      */
    }

    // java private DirectoryNode initDocRoot(String file, RandomAccessFile raf)
    pub fn initDocRoot(&mut self, file: &str, raf: Option<&RandomAccessFile>) -> Option<DirectoryNode> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
                // java POIFSFileSystem poifsFileSystem = null != raf ? new POIFSFileSystem(raf) : new POIFSFileSystem(file);
                let mut poifsFileSystem: POIFSFileSystem = (if raf.is_some() { POIFSFileSystem::new(raf) } else { POIFSFileSystem::new(file) });
                // java return poifsFileSystem.getRoot();
                return poifsFileSystem.getRoot();
                // java catch (IOException e)
                todo!();
                  // java IOLog.e(TAG, "initDocDocument failed", e);
                  IOLog::e__string_string_throwable(&TAG, &"initDocDocument failed", &e);
                  // java return null;
                  return None;
      */
    }

    // java private HWPFDocument initEncryptDoc(String path, RandomAccessFile raf, String password)
    pub fn initEncryptDoc(&mut self, path: &str, raf: Option<&RandomAccessFile>, password: &str) -> Option<HWPFDocument> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
                // java HWPFDocument mDiskDoc = new HWPFDocument(null != raf ? new POIFSFileSystem(raf) : new POIFSFileSystem(new RandomAccessFile(path, "r")));
                let mut mDiskDoc: HWPFDocument = HWPFDocument::new((if raf.is_some() { Some(POIFSFileSystem::new(raf)) } else { Some(POIFSFileSystem::new(RandomAccessFile::new(path, "r"))) }));
                // java mDiskDoc.initEncryptKey(password);
                mDiskDoc.initEncryptKey(password);
                // java return mDiskDoc;
                return mDiskDoc;
                // java catch (IOException e)
                todo!();
                  // java IOLog.e(TAG, "initEncryptDoc failed", e);
                  IOLog::e__string_string_throwable(&TAG, &"initEncryptDoc failed", &e);
                  // java return null;
                  return None;
      */
    }

    // java private String getEncryptBin(DocOLEInfo oleInfo, String originPath)
    pub fn getEncryptBin(&mut self, oleInfo: Option<&DocOLEInfo>, originPath: &str) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null != oleInfo && !StringUtil.isEmpty(originPath))
            if let Some(oleInfo) = oleInfo {
              if !StringUtil::isEmpty(Some(&mut originPath)) {
                // java int location = oleInfo.getLocation();
                let mut location: i32 = oleInfo.getLocation();
                // java if (this.mDocEncryptDoc == null)
                if self.mDocEncryptDoc.is_none() {
                  // java this.mDocEncryptDoc = this.initEncryptDoc(this.mFilePath, this.mRaf, this.mPassword);
                  self.mDocEncryptDoc = self.initEncryptDoc(&self.mFilePath, Some(&self.mRaf), &self.mPassword);
                }
                // java if (null == this.mDocEncryptDoc)
                if self.mDocEncryptDoc.is_none() {
                  // java return null;
                  return None;
                }
                // java else
                else {
                      // java String path = EncryptedOLEDumper.dump(this.mDocEncryptDoc.getDataStream2(), location, originPath);
                      let mut path: String = EncryptedOLEDumper::dump(&mut self.mDocEncryptDoc.getDataStream2(), location, &originPath);
                      // java return path;
                      return path;
                      // java catch (IOException e)
                      todo!();
                        // java IOLog.e(TAG, "getEncryptBin failed", e);
                        IOLog::e__string_string_throwable(&TAG, &"getEncryptBin failed", &e);
                        // java return null;
                        return None;
                }
              }
            }
            // java else
            else {
              // java return null;
              return None;
            }
      */
    }

    // java private String getDocBin(DocOLEInfo docOLEInfo, String originPath)
    pub fn getDocBin(&mut self, docOLEInfo: &DocOLEInfo, originPath: &str) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (this.mDocRootNode == null)
            if self.mDocRootNode.is_none() {
              // java this.mDocRootNode = this.initDocRoot(this.mFilePath, this.mRaf);
              self.mDocRootNode = self.initDocRoot(&self.mFilePath, Some(&self.mRaf));
            }
            // java if (null != this.mDocRootNode && this.mDocRootNode.hasEntry("ObjectPool"))
            if let Some(mDocRootNode) = self.mDocRootNode.as_mut() {
              if self.mDocRootNode.hasEntry(Some(&"ObjectPool")) {
                // java int location = docOLEInfo.getLocation();
                let mut location: i32 = docOLEInfo.getLocation();
                // java Entry entry = null;
                let mut entry: Box<dyn Entry> = Box::new(()) as Box<dyn Entry>;
                    // java entry = this.mDocRootNode.getEntry("ObjectPool");
                    entry = mDocRootNode.getEntry(&"ObjectPool");
                    // java if (null != entry && entry instanceof DirectoryNode)
                    if entry.is_some() && entry.downcast_ref::<DirectoryNode>().is_some() {
                      // java DirectoryNode objectPool = (DirectoryNode)entry;
                      let mut objectPool: DirectoryNode = (entry).downcast_ref::<DirectoryNode>().unwrap();
                      // java String entryName = "_" + location;
                      let mut entryName: String = "_" + location;
                      // java if (objectPool.hasEntry(entryName))
                      if objectPool.hasEntry(entryName) {
                        // java entry = objectPool.getEntry(entryName);
                        entry = objectPool.getEntry(entryName);
                        // java if (entry != null && entry instanceof DirectoryNode)
                        if entry.is_some() && entry.downcast_ref::<DirectoryNode>().is_some() {
                          // java DirectoryNode oleNode = (DirectoryNode)entry;
                          let mut oleNode: DirectoryNode = (entry).downcast_ref::<DirectoryNode>().unwrap();
                          // java StorageUtil.writeEntry2File(originPath, oleNode);
                          StorageUtil::writeEntry2File(&originPath, &oleNode);
                          // java return originPath;
                          return originPath;
                        }
                      }
                    }
                    // java catch (FileNotFoundException e)
                    todo!();
                      // java IOLog.e(TAG, "getDocBin failed", e);
                      IOLog::e__string_string_throwable(&TAG, &"getDocBin failed", &e);
                // java return null;
                return None;
              }
            }
            // java else
            else {
              // java return null;
              return None;
            }
      */
    }

    // java private String getUotBin(UotOleInfo uotOleInfo, String filePath)
    pub fn getUotBin(&mut self, uotOleInfo: &UotOleInfo, filePath: &str) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java String targetPartName = uotOleInfo.getPartName();
            let mut targetPartName: String = uotOleInfo.getPartName();
            // java if (!StringUtil.isEmpty(targetPartName) && !StringUtil.isEmpty(filePath))
            if !StringUtil::isEmpty(Some(&mut targetPartName)) && !StringUtil::isEmpty(Some(&mut filePath)) {
              // java if (null == this.mUofrPackage)
              if self.mUofrPackage.is_none() {
                // java this.mUofrPackage = this.initUotrPackage(this.mFilePath, this.mRaf);
                self.mUofrPackage = self.initUotrPackage(&self.mFilePath, &self.mRaf);
              }
              // java if (null == this.mUofrPackage)
              if self.mUofrPackage.is_none() {
                // java return null;
                return None;
              }
              // java else
              else {
                // java StorageUtil.writeToFile(filePath, this.mUofrPackage.getInputStream(targetPartName));
                StorageUtil::writeToFile__string_inputstream(&filePath, &self.mUofrPackage.getInputStream__string(&targetPartName));
                // java return filePath;
                return filePath;
              }
            }
            // java else
            else {
              // java return null;
              return None;
            }
      */
    }

    // java private UofrPackage initUotrPackage(String filePath, RandomAccessFile raf)
    pub fn initUotrPackage(&self, filePath: &str, raf: &RandomAccessFile) -> UofrPackage {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java UofrPackage uofrPackage = null;
            let mut uofrPackage: UofrPackage = UofrPackage::default();
                // java if (null != this.mRaf)
                if let Some(mRaf) = self.mRaf.as_ref() {
                  // java uofrPackage = new UofrPackage(raf);
                  uofrPackage = UofrPackage::new(raf);
                }
                // java else
                else {
                  // java uofrPackage = new UofrPackage(filePath);
                  uofrPackage = UofrPackage::new(filePath);
                }
                // java catch (IOException e)
                todo!();
                  // java e.printStackTrace();
                  e.printStackTrace();
            // java return uofrPackage;
            return uofrPackage;
      */
    }

    // java public void parseOleFile(OLE ole, String targetFile, Callback callback)
    pub fn parseOleFile__ole_string_callback(&mut self, ole: &OLE, targetFile: &str, callback: Option<&mut dyn Callback>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.cancelOleParseTask();
            self.cancelOleParseTask();
            // java this.mParseOleFuture = new FutureTask(new ParseOleTask(ole, targetFile, callback));
            self.mParseOleFuture = FutureTask::new(ParseOleTask::new(ole, targetFile, callback));
            // java if (callback != null)
            if let Some(callback) = callback {
              // java callback.proxyThreadExecute(this.mParseOleFuture);
              callback.proxyThreadExecute(self.mParseOleFuture);
            }
      */
    }

    // java protected void onParseFailed(OLE ole, String targetFile, boolean interrupted)
    pub fn onParseFailed(&mut self, ole: Option<&OLE>, targetFile: &str, interrupted: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.recycleOleIndex();
            self.recycleOleIndex();
            // java if (null != ole)
            if let Some(ole) = ole {
              // java if (interrupted)
              if interrupted {
                // java String binPath = this.mMediaLib.getMediaByID(ole.getMediaID(), MediaTypeEnum.OLE);
                let mut binPath: String = self.mMediaLib.getMediaByID(ole.getMediaID(), &MediaTypeEnum::OLE);
                // java if (!StringUtil.isEmpty(binPath))
                if !StringUtil::isEmpty(Some(&mut binPath)) {
                  // java FileUtil.delFile(binPath);
                  FileUtil::delFile(Some(&binPath));
                }
              }
              // java else
              else {
                // java this.recordError(ole.getMediaID());
                self.recordError(ole.getMediaID());
              }
            }
            // java if (!StringUtil.isEmpty(targetFile))
            if !StringUtil::isEmpty(Some(&mut targetFile)) {
              // java FileUtil.delFile(targetFile);
              FileUtil::delFile(Some(&targetFile));
            }
      */
    }

    // java private void notifyParseCallback(final ResultType result, final Callback callback)
    pub fn notifyParseCallback(&mut self, result: &ResultType, callback: Option<&mut dyn Callback>) {
      // java if (callback != null)
      if let Some(callback) = callback {
        // java callback.proxyUiExecute(new Runnable() { public void run() { callback.onParseComplete(result); } });
        // rust: Java 匿名 `Runnable` 的 `run()` 调用 `callback.onParseComplete(result)`；
        //   Rust 以 `Box<dyn Fn()>` 建模（`callback` 已被可变借用，闭包体为占位）。
        let mut runnable: Box<dyn Fn()> = Box::new(|| ());
        callback.proxyUiExecute(&mut runnable);
      }
    }

    // java public boolean parseOleFile(OLE ole, String targetFile)
    pub fn parseOleFile__ole_string(&self, ole: Option<&OLE>, targetFile: &mut str) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (!StringUtil.isEmpty(targetFile) && null != ole)
            if let Some(ole) = ole {
              if !StringUtil::isEmpty(Some(&mut targetFile)) {
                // java if (this.mErrorList != null && this.mErrorList.contains(ole.getMediaID()))
                if let Some(mErrorList) = self.mErrorList.as_ref() {
                  if self.mErrorList.contains(ole.getMediaID()) {
                    // java return false;
                    return false;
                  }
                }
                // java else
                else {
                  // java String binPath = this.getOleBinPath(ole.getMediaID());
                  let mut binPath: String = self.getOleBinPath(ole.getMediaID());
                  // java if (!StringUtil.isEmpty(binPath) && FileUtil.exist(binPath))
                  if !StringUtil::isEmpty(Some(&mut binPath)) && FileUtil::exist(Some(&binPath)) {
                    // java String cacheFile = this.getOleCache().get(binPath);
                    let mut cacheFile: String = self.getOleCache().get(binPath);
                    // java if (!StringUtil.isEmpty(cacheFile) && cacheFile.equals(targetFile) && FileUtil.exist(targetFile))
                    if !StringUtil::isEmpty(Some(&mut cacheFile)) && cacheFile.equals(targetFile) && FileUtil::exist(Some(&targetFile)) {
                      // java return true;
                      return true;
                    }
                    // java else
                    else {
                          // java FileUtil.newFile(targetFile);
                          FileUtil::newFile__string(Some(&targetFile));
                          // java catch (IOException var7)
                          todo!();
                            // java return false;
                            return false;
                      // java IOleUnpacker parser = OleUnpackerFactory.getParser(binPath);
                      let mut parser: Box<dyn IOleUnpacker> = OleUnpackerFactory::getParser__entry(Some(&binPath));
                      // java boolean result = false;
                      let mut result: bool = false;
                      // java if (null != parser)
                      if parser.is_some() {
                        // java result = parser.parseFile(targetFile);
                        result = parser.parseFile(targetFile);
                      }
                      // java else
                      else {
                        // java result = FileUtil.copyFile(binPath, targetFile);
                        result = FileUtil::copyFile__string_string(Some(&binPath), Some(&targetFile));
                      }
                      // java if (result)
                      if result {
                        // java if (targetFile.endsWith(".bin"))
                        if targetFile.endsWith(".bin") {
                          // java targetFile = Tools.distinguishSuffixAndRename(targetFile);
                          targetFile = Tools::distinguishSuffixAndRename(&mut targetFile);
                        }
                        // java this.getOleCache().add(binPath, targetFile);
                        self.getOleCache().add(binPath, targetFile);
                      }
                      // java return result;
                      return result;
                    }
                  }
                  // java else
                  else {
                    // java return false;
                    return false;
                  }
                }
              }
            }
            // java else
            else {
              // java return false;
              return false;
            }
      */
    }

    // java public String getOriginFile(OLE ole)
    pub fn getOriginFile(&self, ole: Option<&OLE>) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (null == ole)
            if ole.is_none() {
              // java return null;
              return None;
            }
            // java else
            else {
              // java String binPath = this.getOleBinPath(ole.getMediaID());
              let mut binPath: String = self.getOleBinPath(ole.getMediaID());
              // java if (!StringUtil.isEmpty(binPath) && FileUtil.exist(binPath))
              if !StringUtil::isEmpty(Some(&mut binPath)) && FileUtil::exist(Some(&binPath)) {
                // java String cacheFile = this.getOleCache().get(binPath);
                let mut cacheFile: String = self.getOleCache().get(binPath);
                // java if (!StringUtil.isEmpty(cacheFile))
                if !StringUtil::isEmpty(Some(&mut cacheFile)) {
                  // java File originFile = new File(cacheFile);
                  let mut originFile: std::path::PathBuf = File::new(cacheFile);
                  // java if (originFile.exists() && originFile.length() > 0L)
                  if originFile.exists() && (originFile.len() as i32) > 0 {
                    // java return cacheFile;
                    return cacheFile;
                  }
                }
                // java String originPath = binPath + ".dat";
                let mut originPath: String = binPath + ".dat";
                // java IOleUnpacker parser = OleUnpackerFactory.getParser(binPath);
                let mut parser: Box<dyn IOleUnpacker> = OleUnpackerFactory::getParser__entry(Some(&binPath));
                // java boolean result = null != parser ? parser.parseFile(originPath) : FileUtil.copyFile(binPath, originPath);
                let mut result: bool = (if parser.is_some() { parser.parseFile(originPath) } else { FileUtil::copyFile__string_string(Some(&binPath), Some(&originPath)) });
                // java if (result)
                if result {
                  // java this.getOleCache().add(binPath, originPath);
                  self.getOleCache().add(binPath, originPath);
                  // java return originPath;
                  return originPath;
                }
                // java else
                else {
                  // java return null;
                  return None;
                }
              }
              // java else
              else {
                // java return null;
                return None;
              }
            }
      */
    }

    // java public void cancelOleParseTask()
    pub fn cancelOleParseTask(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java if (this.mParseOleFuture != null && !this.mParseOleFuture.isDone())
            if let Some(mParseOleFuture) = self.mParseOleFuture.as_mut() {
              if !self.mParseOleFuture.isDone() {
                // java this.mParseOleFuture.cancel(true);
                mParseOleFuture.cancel(true);
                // java this.mParseOleFuture = null;
                mParseOleFuture = None;
              }
            }
      */
    }

    // java private static String type2String(ResultType result)
    pub fn type2String(result: &ResultType) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java switch (result)
            match result {
              // java case OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR:
              OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR => {
                // java return "OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR";
                return "OLE_PARSE_RESULT_TYPE_UNKNOWN_ERROR";
              }
              // java case OLE_PARSE_RESULT_TYPE_SUCCESS:
              OLE_PARSE_RESULT_TYPE_SUCCESS => {
                // java return "OLE_PARSE_RESULT_TYPE_SUCCESS";
                return "OLE_PARSE_RESULT_TYPE_SUCCESS";
              }
              // java case OLE_PARSE_RESULT_TYPE_FAIL:
              OLE_PARSE_RESULT_TYPE_FAIL => {
                // java return "OLE_PARSE_RESULT_TYPE_FAIL";
                return "OLE_PARSE_RESULT_TYPE_FAIL";
              }
              // java case OLE_PARSE_RESULT_TYPE_INTERRUPT:
              OLE_PARSE_RESULT_TYPE_INTERRUPT => {
                // java return "OLE_PARSE_RESULT_TYPE_INTERRUPT";
                return "OLE_PARSE_RESULT_TYPE_INTERRUPT";
              }
              // java default:
              _ => {
                // java Assert.fail("unknow type");
                Assert::fail__string(&"unknow type");
                // java return "";
                return "";
              }
            }
      */
    }

    // rust: ★M-STAGE2 P1-D1 —— Java 反编译源码中 `access$100` / `access$202` / `access$300`
    //   是 **javac 为内部类访问私有成员合成的桥接方法**，Java 与 Rust 侧**均无调用者**。
    //   `$` 不是合法 Rust 标识符 ⇒ 按本仓库 `__` 重载分隔约定改名为 `access__N`；
    //   `// java` 行保持原文逐字不变（门禁要求）。
    // java static String access$100()
    pub fn access__100() -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java return TAG;
            return TAG;
      */
    }

    // java static FutureTask access$202(OleManager x0, FutureTask x1)
    pub fn access__202(x0: &mut OleManager, x1: &FutureTask) -> FutureTask {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java return x0.mParseOleFuture = x1;
            return { x0.mParseOleFuture = x1; x0.mParseOleFuture };
      */
    }

    // java static void access$300(OleManager x0, ResultType x1, Callback x2)
    pub fn access__300(x0: &mut OleManager, x1: &ResultType, x2: &mut dyn Callback) {
      // java x0.notifyParseCallback(x1, x2);
      x0.notifyParseCallback(&x1, Some(x2));
    }

}

// java private class ParseOleTask implements Callable<Boolean>
impl ParseOleTask {
    // java private  ParseOleTask(OLE ole, String targetFile, Callback callback)
    pub fn ParseOleTask(ole: &OLE, targetFile: &str, callback: &dyn Callback) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
         ★M-STAGE2 P1-D1 / 08-member-assembly-plan T0-2 —— 非管线逻辑合法延迟。
         依据：语料 standard-hello.docx / multipage.docx / hello-world.docx 的
         word/document.xml 中 <w:object> / <o:OLEObject> 均为 **0 处**。
         真实移植体依赖 7+ 个其他 crate 中 ~50 个尚未移植的方法，补齐对管线零收益。
         原始模板翻译完整保留于下方，待依赖就绪后恢复。
            // java this.ole = ole;
            Self { ole: ole, targetFile: targetFile, callback: callback }
            // java this.targetFile = targetFile;
            // java this.callback = callback;
      */
    }

    // java public Boolean call()
    pub fn call(&mut self) -> bool {
      // java return Boolean.FALSE;
      // rust: Java `Boolean.FALSE` 是装箱的布尔常量 ⇒ Rust 原生 `false`。
      return false;
    }

}

// java public static enum ResultType
impl ResultType {
}


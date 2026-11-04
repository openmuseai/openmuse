// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import org.apache.poi.hwpf.ole.OleClsTypeMatcher
use io_poi_compact::org::apache::poi::hwpf::ole::OleClsTypeMatcher::*;
// Java: import org.apache.poi.hwpf.ole.stream.OlePackageCache
#[derive(Default)]
pub struct OlePackageCache;

// java private static final String OLE_PREFIX = "OLE_";
const OLE_PREFIX: &str = "OLE_";
// java public static final String OLE_DEFAULT_SUFFIX = ".bin";
pub(crate) const OLE_DEFAULT_SUFFIX: &str = ".bin";
// java private static final String OLE_DIR_NAME = "ole_tmp";
const OLE_DIR_NAME: &str = "ole_tmp";

// java public class OleFileUtil
#[derive(Default)]
pub struct OleFileUtil {
}

// java public class OleFileUtil
impl OleFileUtil {
    // java public static String getOleFile(TextDocument document, OLE ole)
    pub fn getOleFile__textdocument_ole(document: &TextDocument, ole: Option<&OLE>) -> Option<String> {
      // java if (null != ole && ole.getProgId() != null)
      if let Some(ole) = ole {
        if ole.getProgId().is_some() {
          // java OlePackageCache olePackageCache = document.getOleManager().getOleCache();
          let mut olePackageCache: OlePackageCache = document.getOleManager().getOleCache();
          // java String binPath = document.getMediaLib().getMediaByID(ole.getMediaID(), MediaTypeEnum.OLE);
          let mut binPath: String = document.getMediaLib().getMediaByID(ole.getMediaID(), MediaTypeEnum::OLE);
          // java String cacheFile = olePackageCache.get(binPath);
          let mut cacheFile: String = olePackageCache.get(binPath);
          // java if (!StringUtil.isEmpty(cacheFile))
          if !StringUtil::isEmpty(cacheFile) {
            // java return cacheFile;
            return cacheFile;
          }
          // java else
          else {
            // java String suffix = OleClsTypeMatcher.getSuffixByProgId(ole.getProgId());
            let mut suffix: String = OleClsTypeMatcher::getSuffixByProgId(&mut ole.getProgId());
            // java suffix = suffix == null ? ".bin" : "." + suffix;
            suffix = (if suffix.is_none() { Some(".bin".to_string()) } else { Some("." + suffix) });
            // java return getOleFile(document, suffix);
            return Self::getOleFile__textdocument_ole(&document, Some(&suffix));
          }
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static String getOleFile(TextDocument document, String formatSuffix)
    pub fn getOleFile__textdocument_string(document: &TextDocument, formatSuffix: &str) -> Option<String> {
      // java String originPath = document.getPath();
      let mut originPath: String = document.getPath();
      // java RandomAccessFile originRaf = document.getRaf();
      let mut originRaf: RandomAccessFile = document.getRaf();
      // java String originName = getOriginName(originPath, originRaf);
      let mut originName: String = Self::getOriginName(Some(&mut originPath), Some(&originRaf));
      // java return null == originName ? null : getOleFile(document.getTmpRootDir(), originName, originPath, originRaf, document.getOleManager().nextOleIndex(), formatSuffix);
      return (if originName.is_none() { None } else { Some(Self::getOleFile__string_string_string_randomaccessfile_int_string(&document.getTmpRootDir(), &originName, &originPath, &originRaf, document.getOleManager().nextOleIndex(), &formatSuffix)) });
    }

    // java private static String getOriginName(String originPath, RandomAccessFile originRaf)
    pub fn getOriginName(originPath: Option<&mut str>, originRaf: Option<&RandomAccessFile>) -> Option<String> {
      // java if (null == originPath && null == originRaf)
      if originPath.is_none() && originRaf.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java String originName = null;
        let mut originName: String = String::new();
        // java if (null != originPath)
        if let Some(originPath) = originPath {
          // java int separatorIndex = originPath.lastIndexOf(File.separator);
          let mut separatorIndex: i32 = originPath.lastIndexOf(File::separator);
          // java int dotIndex = originPath.lastIndexOf(".");
          let mut dotIndex: i32 = originPath.lastIndexOf(".");
          // java if (separatorIndex + 1 > dotIndex)
          if separatorIndex + 1 > dotIndex {
            // java return null;
            return None;
          }
          // java originName = originPath.substring(separatorIndex + 1, dotIndex);
          originName = originPath.substring(separatorIndex + 1, dotIndex);
        }
        // java else if (null != originRaf)
        else if let Some(originRaf) = originRaf {
          // java String fileName = originRaf.getFileName();
          let mut fileName: String = originRaf.getFileName();
          // java int dotIndex = fileName.indexOf(".");
          let mut dotIndex: i32 = fileName.indexOf(".");
          // java if (-1 == dotIndex)
          if -1 == dotIndex {
            // java return null;
            return None;
          }
          // java originName = fileName.substring(0, dotIndex);
          originName = fileName.substring(0, dotIndex);
        }
        // java return originName;
        return originName;
      }
    }

    // java public static String getOleFile(String rootDir, String originFileName, String originFullPath, RandomAccessFile originRaf, int index, String formatSuffix)
    pub fn getOleFile__string_string_string_randomaccessfile_int_string(rootDir: &str, originFileName: &str, originFullPath: &str, originRaf: &RandomAccessFile, index: i32, formatSuffix: &str) -> String {
          // java String prefix = Platform.getResourceManager().getString("writer_ole_prefix");
          let mut prefix: String = Platform::getResourceManager().getString("writer_ole_prefix");
          // java String suffix = Platform.getResourceManager().getString("writer_ole_suffix");
          let mut suffix: String = Platform::getResourceManager().getString("writer_ole_suffix");
          // java String indexStr = index == 0 ? "" : "(" + index + ")";
          let mut indexStr: String = (if index == 0 { "".to_string() } else { "(" + index + ")" });
          // java String fileName = prefix + "_" + originFileName + "_" + suffix + indexStr + formatSuffix;
          let mut fileName: String = prefix + "_" + originFileName + "_" + suffix + indexStr + formatSuffix;
          // java String fullPath = getOlePath(rootDir, originFullPath, originRaf) + File.separator + fileName;
          let mut fullPath: String = Self::getOlePath(&rootDir, &originFullPath, Some(&originRaf)) + File::separator + fileName;
          // java return fullPath;
          return fullPath;
          // java Exception var11
          todo!();
            // java return Platform.createTempFile("OLE_", formatSuffix).getAbsolutePath();
            return Platform::createTempFile__string_string(&"OLE_", &formatSuffix).getAbsolutePath();
    }

    // java public static String getOlePath(String rootDir, String originFullPath, RandomAccessFile originRaf)
    pub fn getOlePath(rootDir: &str, originFullPath: &str, originRaf: Option<&RandomAccessFile>) -> Option<String> {
      // java return getOleTempDir(rootDir) + Math.abs((long)(null != originRaf ? originRaf.getFileName().hashCode() : originFullPath.hashCode()));
      return Self::getOleTempDir(&rootDir) + Math::abs((((if originRaf.is_some() { Some(originRaf.getFileName().hashCode()) } else { Some(originFullPath.hashCode()) })) as i64));
    }

    // java public static String getOleTempDir(String rootDir)
    pub fn getOleTempDir(rootDir: &str) -> String {
      // java String oleTempDirPath = rootDir + "ole_tmp" + File.separator;
      let mut oleTempDirPath: String = rootDir + "ole_tmp" + File::separator;
      // java return oleTempDirPath;
      return oleTempDirPath;
    }

    // java public static File createTmpOleFile()
    pub fn createTmpOleFile() -> std::path::PathBuf {
      // java return createTmpOleFile(".bin");
      return Self::createTmpOleFile__string(&mut ".bin");
    }

    // java public static File createTmpOleFile(String formatSuffix)
    pub fn createTmpOleFile__string(formatSuffix: &mut str) -> std::path::PathBuf {
      // java if (StringUtil.isEmpty(formatSuffix))
      if StringUtil::isEmpty(formatSuffix) {
        // java formatSuffix = ".bin";
        formatSuffix = ".bin";
      }
      // java File file = Platform.createTempFile("OLE_", formatSuffix);
      let mut file: std::path::PathBuf = Platform::createTempFile__string_string(&"OLE_", &formatSuffix);
      // java return file;
      return file;
    }

}

// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.ole.IOLEDocument
use util_adapter::cn::wps::ole::IOLEDocument::*;
// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage::*;
// Java: import cn.wps.ole.IOLEStream
use util_adapter::cn::wps::ole::IOLEStream::*;
// Java: import cn.wps.ole.OLEDocumentFactory
use util_adapter::cn::wps::ole::OLEDocumentFactory::*;
// Java: import cn.wps.ole.OleUtil
use util_adapter::cn::wps::ole::OleUtil::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.nio.charset.StandardCharsets
use stubs::java::nio::charset::StandardCharsets::*;
// Java: import org.apache.poi.hwpf.util.ByteArrayBuffer
use crate::org::apache::poi::hwpf::util::ByteArrayBuffer::*;
// Java: import org.apache.poi.hwpf.ole.stream.IStreamParser
use crate::org::apache::poi::hwpf::ole::stream::IStreamParser::*;
// Java: import org.apache.poi.hwpf.ole.stream.OlePackageCache
use crate::org::apache::poi::hwpf::ole::stream::OlePackageCache::*;

// java private static final String OLE_10_NATIVE = "\u0001Ole10Native";
const OLE_10_NATIVE: &str = "\u{0001}Ole10Native";
// java private static final int HEADER_STREAM_LENGTH = 4;
const HEADER_STREAM_LENGTH: i32 = 4;
// java private static final int HEADER_START_MARK_LENGTH = 2;
const HEADER_START_MARK_LENGTH: i32 = 2;
// java private static final int HEADER_BLANK_LENGTH = 4;
const HEADER_BLANK_LENGTH: i32 = 4;
// java private static final int HEADER_PATH_LENGTH = 4;
const HEADER_PATH_LENGTH: i32 = 4;
// java private static final int HEADER_FILE_LENGTH = 4;
const HEADER_FILE_LENGTH: i32 = 4;

// java public abstract class NativeStreamParser implements IStreamParser
#[derive(Default)]
pub struct NativeStreamParser {
    // java private OlePackageCache mOlePackageCache;
    mOlePackageCache: Option<OlePackageCache>,
}

// java public interface IStreamParser
impl IStreamParser for NativeStreamParser {
    // java public String getRawFilePath(String binPath)
    fn getRawFilePath(&mut self, binPath: &str) -> String {
      // java String rawFilePath = this.getFromCache(binPath);
      let mut rawFilePath: String = self.getFromCache(&binPath);
      // java if (rawFilePath != null)
      if rawFilePath.is_some() {
        // java return rawFilePath;
        return rawFilePath;
      }
      // java else
      else {
            // java IOLEDocument doc = OLEDocumentFactory.open(binPath, 2);
            let mut doc: Box<dyn IOLEDocument> = OLEDocumentFactory::open__string_int(&binPath, 2);
            // java IOLEStorage stg = doc.getRootStorage();
            let mut stg: Box<dyn IOLEStorage> = doc.getRootStorage();
            // java if (null == stg)
            if stg.is_none() {
              // java return binPath;
              return binPath;
            }
            // java IOLEStream stream = stg.createStream("\u0001Ole10Native");
            let mut stream: Box<dyn IOLEStream> = stg.createStream("\u{0001}Ole10Native");
            // java rawFilePath = this.readHeader(stream);
            rawFilePath = self.readHeader(&mut stream);
            // java if (null == rawFilePath)
            if rawFilePath.is_none() {
              // java rawFilePath = this.getDefaultFilePath();
              rawFilePath = self.getDefaultFilePath();
            }
            // java if (null != rawFilePath)
            if rawFilePath.is_some() {
              // java OleUtil.readFromOLEStream(rawFilePath, stream);
              OleUtil::readFromOLEStream__string_iolestream(&rawFilePath, &stream);
              // java this.addToCache(binPath, rawFilePath);
              self.addToCache(&binPath, &rawFilePath);
              // java return rawFilePath;
              return rawFilePath;
            }
            // java IOException e
            todo!();
              // java IOLog.d("NativeStreamParser", "IOException", e);
              IOLog::d__string_string_throwable(&"NativeStreamParser", &"IOException", &e);
        // java return binPath;
        return binPath;
      }
    }

}

// java public abstract class NativeStreamParser implements IStreamParser
impl NativeStreamParser {
    // java public  NativeStreamParser(OlePackageCache cache)
    pub fn NativeStreamParser(cache: &OlePackageCache) -> Self {
      // java super();
      // java this.mOlePackageCache = cache;
      Self { mOlePackageCache: cache }
    }

    // java private String readHeader(IOLEStream stream)
    pub fn readHeader(&mut self, stream: &mut dyn IOLEStream) -> String {
      // java String fileName = null;
      let mut fileName: String = String::new();
          // java stream.skipBytes(6);
          stream.skipBytes(6);
          // java fileName = readString(stream);
          fileName = Self::readString(&mut stream);
          // java if (null != fileName)
          if fileName.is_some() {
            // java fileName = Platform.getTempDirectory() + File.separator + fileName;
            fileName = Platform::getTempDirectory() + File::separator + fileName;
          }
          // java skipString(stream);
          Self::skipString(&mut stream);
          // java stream.skipBytes(8);
          stream.skipBytes(8);
          // java skipString(stream);
          Self::skipString(&mut stream);
          // java stream.skipBytes(4);
          stream.skipBytes(4);
          // java IOException var4
          todo!();
            // java IOLog.e("NativeOleUnpacker", "skip header failed");
            IOLog::e__string_string(&"NativeOleUnpacker", &"skip header failed");
      // java return fileName;
      return fileName;
    }

    // java private static void skipString(IOLEStream stream)
    pub fn skipString(stream: &mut dyn IOLEStream) {
      // java byte b;
      let mut b: u8;
      // java do (0 != b)
      loop {
          // java b = stream.readByte();
          b = stream.readByte();
      }
    }

    // java private static String readString(IOLEStream stream)
    pub fn readString(stream: &mut dyn IOLEStream) -> String {
      // java String result = null;
      let mut result: String = String::new();
      // java ByteArrayBuffer arrayBuffer = new ByteArrayBuffer(64);
      let mut arrayBuffer: ByteArrayBuffer = ByteArrayBuffer::new(64);
      // java while (true)
        // java byte b = stream.readByte();
        let mut b: u8 = stream.readByte();
        // java if (0 == b)
        if 0 == b {
          // java if (!arrayBuffer.isEmpty())
          if !arrayBuffer.is_empty() {
            // java result = new String(arrayBuffer.toByteArray(), StandardCharsets.UTF_8);
            result = String::new(arrayBuffer.toByteArray(), StandardCharsets::UTF_8);
          }
          // java return result;
          return result;
        }
        // java arrayBuffer.append(b);
        arrayBuffer.append(b);
    }

    // java protected String getFromCache(String binPath)
    pub fn getFromCache(&self, binPath: &str) -> Option<String> {
      // java return this.mOlePackageCache != null ? this.mOlePackageCache.get(binPath) : null;
      return (if self.mOlePackageCache.is_some() { Some(self.mOlePackageCache.get(&binPath)) } else { None });
    }

    // java protected void addToCache(String binPath, String rawFilePath)
    pub fn addToCache(&mut self, binPath: &str, rawFilePath: &str) {
      // java if (this.mOlePackageCache != null)
      if let Some(mOlePackageCache) = self.mOlePackageCache.as_mut() {
        // java this.mOlePackageCache.add(binPath, rawFilePath);
        mOlePackageCache.add(&binPath, &rawFilePath);
      }
    }

    // java public static String getFileName(String binPath)
    pub fn getFileName(binPath: &str) -> Option<String> {
          // java IOLEDocument doc = OLEDocumentFactory.open(binPath, 0);
          let mut doc: Box<dyn IOLEDocument> = OLEDocumentFactory::open__string_int(&binPath, 0);
          // java IOLEStorage stg = doc.getRootStorage();
          let mut stg: Box<dyn IOLEStorage> = doc.getRootStorage();
          // java if (null == stg)
          if stg.is_none() {
            // java return null;
            return None;
          }
          // java else
          else {
            // java IOLEStream stream = stg.createStream("\u0001Ole10Native");
            let mut stream: Box<dyn IOLEStream> = stg.createStream("\u{0001}Ole10Native");
            // java stream.skipBytes(6);
            stream.skipBytes(6);
            // java return readString(stream);
            return Self::readString(&mut stream);
          }
          // java IOException e
          todo!();
            // java IOLog.d("NativeStreamParser", "IOException", e);
            IOLog::d__string_string_throwable(&"NativeStreamParser", &"IOException", &e);
            // java return null;
            return None;
    }

    // java protected abstract String getDefaultFilePath()
    pub fn getDefaultFilePath(&self) -> String {
      panic!("NativeStreamParser::getDefaultFilePath not implemented");
    }

}

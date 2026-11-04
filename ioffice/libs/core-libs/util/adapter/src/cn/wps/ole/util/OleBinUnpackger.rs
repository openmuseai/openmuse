// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.ole.IOLEDocument
// Java: import cn.wps.ole.IOLEStorage
// Java: import cn.wps.ole.IOLEStream
// Java: import cn.wps.ole.OLEDocumentFactory
// Java: import cn.wps.ole.OleUtil
// Java: import java.io.IOException

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: String OLE_10_NATIVE
pub(crate) const OLE_10_NATIVE: &str = "\u{0001}Ole10Native";

// Java: public class OleBinUnpackger
pub struct OleBinUnpackger {
}

// Java: public class OleBinUnpackger
impl OleBinUnpackger {
    // Java: OleBinUnpackger()
    pub fn new() -> Self {
      // Java: super();
      OleBinUnpackger {}
    }

    // Java: static String unpackBin(String binPath)
    pub fn unpackBin(_binPath: &str) -> Option<String> {
      // Java: try
      // Java: IOLEDocument doc = OLEDocumentFactory.open((String)binPath, 2);
      // Java: IOLEStorage stg = doc.getRootStorage();
      // Java: if (null == stg)
      // Java: return null;
      // Java: else
      // Java: IOLEStream stream = stg.createStream("\u0001Ole10Native");
      // Java: skipOLE_10_NATIVE_Header(stream);
      // Java: File file = Platform.createTempFile("audio", ".wav");
      // Java: if (null == file)
      // Java: return null;
      // Java: else
      // Java: String path = file.getAbsolutePath();
      // Java: OleUtil.readFromOLEStream(path, stream);
      // Java: return path;
      // Java: catch (IOException e)
      // Java: IOLog.d(TAG, "IOException", e);
      // Java: return null;
      todo!()
    }

    // Java: static void skipOLE_10_NATIVE_Header(IOLEStream stream)
    fn skipOLE_10_NATIVE_Header(_stream: &dyn std::any::Any) {
      // Java: stream.skipBytes(6);
      // Java: skipString(stream);
      // Java: skipString(stream);
      // Java: stream.skipBytes(8);
      // Java: skipString(stream);
      // Java: stream.skipBytes(4);
      todo!()
    }

    // Java: static void skipString(IOLEStream stream)
    fn skipString(_stream: &dyn std::any::Any) {
      // Java: byte b;
      // Java: do
      // Java: b = stream.readByte();
      todo!()
    }

}

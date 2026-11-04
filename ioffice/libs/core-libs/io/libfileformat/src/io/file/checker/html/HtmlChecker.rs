// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.io.file.FileFormatEnum
use util::cn::wps::io::file::FileFormatEnum;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import java.io.BufferedInputStream
// TODO_RUST_CODE use ???; // java.io.BufferedInputStream
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStreamReader
// TODO_RUST_CODE use ???; // java.io.InputStreamReader
// Java: import org.mozilla.universalchardet.CharsetListener
use thirdparty_juniversalchardet::org::mozilla::universalchardet::CharsetListener;
// Java: import org.mozilla.universalchardet.UniversalDetector
use thirdparty_juniversalchardet::org::mozilla::universalchardet::UniversalDetector;

// Java field: int BUFFERSIZE
static BUFFERSIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int MAX_BUF
static MAX_BUF: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: String HTML_TAG
static HTML_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String HEAD_TAG
static HEAD_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String BODY_TAG
static BODY_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String TABLE_TAG
static TABLE_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String DOCTYPE_TAG
static DOCTYPE_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String SCRIPT_TAG
static SCRIPT_TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class HtmlChecker
pub struct HtmlChecker {
    // Java field: BufferedInputStream _bufferedInputStream
    _bufferedInputStream: Option<BufferedInputStream>,
    // Java field: Boolean _isHtml
    pub(crate) _isHtml: Arc<Mutex<Boolean>>,
    // Java field: String ext
    ext: Option<String>,
}

// Java: public class HtmlChecker
impl HtmlChecker {
    // Java: HtmlChecker(File file)
    pub fn HtmlChecker(&self, File file) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: Assert.assertNotNull("file should not be null", file);
      // TODO_RUST_CODE
      // Java: if (null != file)
        // TODO_RUST_CODE
      {
         // Java: this._bufferedInputStream = new BufferedInputStream(Tools.createFileInputStream(file));
         // TODO_RUST_CODE
         // Java: this.ext = Tools.getLowerCaseExtension(file.getAbsolutePath());
         // TODO_RUST_CODE
      }
    }

    // Java: String getEncoding(BufferedInputStream bufferedInputStream)
    pub fn getEncoding(&self, BufferedInputStream bufferedInputStream) -> String {
      // Java: if (bufferedInputStream == null)
        // TODO_RUST_CODE
      {
         // Java: return "GBK";
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int available = bufferedInputStream.available();
         // TODO_RUST_CODE
         // Java: bufferedInputStream.mark(available);
         // TODO_RUST_CODE
         // Java: byte[] buf = new byte[4096];
         // TODO_RUST_CODE
         // Java: UniversalDetector detector = new UniversalDetector((CharsetListener)null);
         // TODO_RUST_CODE
         // Java: int nread = 0;
         // TODO_RUST_CODE
         // Java: int count = 0;
         // TODO_RUST_CODE
         // Java: int countLimit = Math.min(available, MAX_BUF);
         // TODO_RUST_CODE
         // Java: while(count < countLimit && (nread = bufferedInputStream.read(buf)) > 0 && !detector.isDone())
            // TODO_RUST_CODE
         {
            // Java: count += nread;
            // TODO_RUST_CODE
            // Java: detector.handleData(buf, 0, nread);
            // TODO_RUST_CODE
         }
         // Java: bufferedInputStream.reset();
         // TODO_RUST_CODE
         // Java: detector.dataEnd();
         // TODO_RUST_CODE
         // Java: String encoding = detector.getDetectedCharset();
         // TODO_RUST_CODE
         // Java: return this.encodingfilter(encoding);
         // TODO_RUST_CODE
      }
    }

    // Java: String encodingfilter(String encoding)
    pub fn encodingfilter(&self, String encoding) -> String {
      // Java: if (encoding == null)
        // TODO_RUST_CODE
      {
         // Java: return "GBK";
         // TODO_RUST_CODE
      }
      // Java: else if (encoding == "GB18030")
        // TODO_RUST_CODE
      {
         // Java: return "GBK";
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return encoding == "IBM855" ? "GBK" : encoding;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isHtml()
    pub fn isHtml(&self) -> bool {
      // Java: if (this._isHtml != null)
        // TODO_RUST_CODE
      {
         // Java: return this._isHtml;
         // TODO_RUST_CODE
      }
      // Java: else if (this._bufferedInputStream == null)
        // TODO_RUST_CODE
      {
         // Java: return this._isHtml = false;
         // TODO_RUST_CODE
      }
      // Java: else if (FileFormatEnum.TXT.getExt().equals(this.ext))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: String strData = null;
         // TODO_RUST_CODE
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: String encoding = this.getEncoding(this._bufferedInputStream);
            // TODO_RUST_CODE
            // Java: char[] data = new char[BUFFERSIZE];
            // TODO_RUST_CODE
            // Java: InputStreamReader reader = new InputStreamReader(this._bufferedInputStream, encoding);
            // TODO_RUST_CODE
            // Java: int nRead = reader.read(data);
            // TODO_RUST_CODE
            // Java: if (nRead <= 0)
                // TODO_RUST_CODE
            {
               // Java: return this._isHtml = false;
               // TODO_RUST_CODE
            }
            // Java: strData = new String(data, 0, nRead);
            // TODO_RUST_CODE
         }
         // Java: catch (IOException e)
            // TODO_RUST_CODE
         {
            // Java: String msg = e.getMessage();
            // TODO_RUST_CODE
            // Java: if (null != msg)
                // TODO_RUST_CODE
            {
               // Java: System.out.println(msg);
               // TODO_RUST_CODE
            }
            // Java: return this._isHtml = false;
            // TODO_RUST_CODE
         }
         // Java: if (null != strData && 0 != strData.length())
            // TODO_RUST_CODE
         {
            // Java: strData = strData.toLowerCase();
            // TODO_RUST_CODE
            // Java: this._isHtml = strData.contains(HTML_TAG) || strData.contains(HEAD_TAG) || strData.contains(BODY_TAG) || strData.contains(TABLE_TAG) || strData.contains(DOCTYPE_TAG) || strData.contains(SCRIPT_TAG); return this._isHtml;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: return this._isHtml = false;
            // TODO_RUST_CODE
         }
      }
    }

}

// Java: import cn.wps.base.assertion.IOAssert
use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.ifs.RafStream
use kfs_kfs_app::cn::wps::moffice::kfs::ifs::RafStream;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.BufferedReader
// TODO_RUST_CODE use ???; // java.io.BufferedReader
// Java: import java.io.FileNotFoundException
// TODO_RUST_CODE use ???; // java.io.FileNotFoundException
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream
// Java: import java.io.InputStreamReader
// TODO_RUST_CODE use ???; // java.io.InputStreamReader
// Java: import java.util.Locale
// TODO_RUST_CODE use ???; // java.util.Locale
// Java: import org.mozilla.universalchardet.CharsetListener
use thirdparty_juniversalchardet::org::mozilla::universalchardet::CharsetListener;
// Java: import org.mozilla.universalchardet.Constants
use io_poi_compact::org::apache::poi::hpsf::Constants;
// Java: import org.mozilla.universalchardet.UniversalDetector
use thirdparty_juniversalchardet::org::mozilla::universalchardet::UniversalDetector;

// Java field: String TAG
pub(crate) const TAG: String = /* TODO */;
// Java field: byte[] _EUC_JP_BYTES
static _EUC_JP_BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] _EUC_KR_BYTES
static _EUC_KR_BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = /* TODO */;
// Java field: int MAX_READ_LENGTH
pub(crate) const MAX_READ_LENGTH: i32 = /* TODO */;
// Java field: String THAI_LANGUAGE
pub(crate) const THAI_LANGUAGE: String = /* TODO */;
// Java field: char BOM_CHAR
pub(crate) const BOM_CHAR: char = /* TODO */;

// Java: public class TXTDocument
pub struct TXTDocument {
    // Java field: String _path
    _path: Option<String>,
    // Java field: RandomAccessFile mRaf
    mRaf: RandomAccessFile,
    // Java field: String _encoding
    pub(crate) _encoding: Arc<Mutex<String>>,
    // Java field: BufferedReader _input
    _input: Option<BufferedReader>,
    // Java field: InputStream _fis
    pub(crate) _fis: Arc<Mutex<InputStream>>,
    // Java field: boolean mHasBom
    mHasBom: bool,
}

// Java: public class TXTDocument
impl TXTDocument {
    // Java: TXTDocument(String path, RandomAccessFile raf, String encoding)
    pub fn TXTDocument(&self, String path, RandomAccessFile raf, String encoding) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._path = path;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
      // Java: this._input = null;
      // TODO_RUST_CODE
      // Java: String guessEncoding = this.detectEncoding();
      // TODO_RUST_CODE
      // Java: if (null != guessEncoding)
        // TODO_RUST_CODE
      {
         // Java: this._encoding = guessEncoding;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this._encoding = encoding;
         // TODO_RUST_CODE
      }
      // Java: this._fis = (InputStream)(null != this.mRaf ? new RafStream(this.mRaf) : new FileInputStream(this._path));
      // TODO_RUST_CODE
      // Java: this._input = new BufferedReader(new InputStreamReader(this._fis, this._encoding));
      // TODO_RUST_CODE
    }

    // Java: void updateEncoding(String encoding)
    pub fn updateEncoding(&self, String encoding) {
      // Java: if (null != encoding)
        // TODO_RUST_CODE
      {
         // Java: InputStream fis = null;
         // TODO_RUST_CODE
         // Java: BufferedReader input = null;
         // TODO_RUST_CODE
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: fis = (InputStream)(null != this.mRaf ? new RafStream(this.mRaf) : new FileInputStream(this._path));
            // TODO_RUST_CODE
            // Java: input = new BufferedReader(new InputStreamReader(fis, encoding));
            // TODO_RUST_CODE
         }
         // Java: catch (IOException e)
            // TODO_RUST_CODE
         {
            // Java: IOLog.d(TAG, "invalid encoding", e);
            // TODO_RUST_CODE
            // Java: FileUtil.closeQuietly(fis);
            // TODO_RUST_CODE
         }
         // Java: if (null != fis && null != input)
            // TODO_RUST_CODE
         {
            // Java: FileUtil.closeQuietly(this._fis);
            // TODO_RUST_CODE
            // Java: this._fis = fis;
            // TODO_RUST_CODE
            // Java: this._input = input;
            // TODO_RUST_CODE
            // Java: this._encoding = encoding;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: int guessDocumentLength()
    pub fn guessDocumentLength(&self) -> i32 {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: return this._fis.available();
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: return 0;
         // TODO_RUST_CODE
      }
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> String {
      // Java: return this._encoding;
      // TODO_RUST_CODE
    }

    // Java: String detectEncoding()
    pub fn detectEncoding(&self) -> String {
      // Java: byte[] buf = new byte[1024];
      // TODO_RUST_CODE
      // Java: InputStream fis = (InputStream)(null != this.mRaf ? new RafStream(this.mRaf) : new FileInputStream(this._path));
      // TODO_RUST_CODE
      // Java: UniversalDetector detector = new UniversalDetector((CharsetListener)null);
      // TODO_RUST_CODE
      // Java: int count = 0;
      // TODO_RUST_CODE
      // Java: int nRead = fis.read(buf);
      // TODO_RUST_CODE
      // Java: String encoding = this.getCharsetByBom(buf, nRead);
      // TODO_RUST_CODE
      // Java: if (null == encoding)
        // TODO_RUST_CODE
      {
         // Java: while(nRead > 0 && !detector.isDone() && count < 4096)
            // TODO_RUST_CODE
         {
            // Java: count += nRead;
            // TODO_RUST_CODE
            // Java: detector.handleData(buf, 0, nRead);
            // TODO_RUST_CODE
            // Java: nRead = fis.read(buf);
            // TODO_RUST_CODE
         }
         // Java: this.closeInputStream(fis);
         // TODO_RUST_CODE
         // Java: detector.dataEnd();
         // TODO_RUST_CODE
         // Java: encoding = detector.getDetectedCharset();
         // TODO_RUST_CODE
      }
      // Java: encoding = this.encodingfilter(encoding);
      // TODO_RUST_CODE
      // Java: encoding = this.encodingCorrect(encoding);
      // TODO_RUST_CODE
      // Java: return encoding;
      // TODO_RUST_CODE
    }

    // Java: String getCharsetByBom(byte[] buf, int length)
    pub fn getCharsetByBom(&self, byte[] buf, int length) -> String {
      // Java: if (length > 3)
        // TODO_RUST_CODE
      {
         // Java: int b1 = buf[0] & 255;
         // TODO_RUST_CODE
         // Java: int b2 = buf[1] & 255;
         // TODO_RUST_CODE
         // Java: int b3 = buf[2] & 255;
         // TODO_RUST_CODE
         // Java: int b4 = buf[3] & 255;
         // TODO_RUST_CODE
         // Java: if (b1 == 239 && b2 == 187 && b3 == 191)
            // TODO_RUST_CODE
         {
            // Java: this.mHasBom = true;
            // TODO_RUST_CODE
            // Java: return Constants.CHARSET_UTF_8;
            // TODO_RUST_CODE
         }
         // Java: if (b1 == 254 && b2 == 255)
            // TODO_RUST_CODE
         {
            // Java: this.mHasBom = true;
            // TODO_RUST_CODE
            // Java: return Constants.CHARSET_UTF_16BE;
            // TODO_RUST_CODE
         }
         // Java: if (b1 == 0 && b2 == 0 && b3 == 254 && b4 == 255)
            // TODO_RUST_CODE
         {
            // Java: this.mHasBom = true;
            // TODO_RUST_CODE
            // Java: return Constants.CHARSET_UTF_32BE;
            // TODO_RUST_CODE
         }
         // Java: if (b1 == 255 && b2 == 254 && b3 == 0 && b4 == 0)
            // TODO_RUST_CODE
         {
            // Java: this.mHasBom = true;
            // TODO_RUST_CODE
            // Java: return Constants.CHARSET_UTF_32LE;
            // TODO_RUST_CODE
         }
         // Java: if (b1 == 255 && b2 == 254)
            // TODO_RUST_CODE
         {
            // Java: this.mHasBom = true;
            // TODO_RUST_CODE
            // Java: return Constants.CHARSET_UTF_16LE;
            // TODO_RUST_CODE
         }
      }
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: void closeInputStream(InputStream fis)
    pub fn closeInputStream(&self, InputStream fis) {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: fis.close();
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: IOAssert.fail("it should not reach here");
         // TODO_RUST_CODE
      }
    }

    // Java: String encodingfilter(String encoding)
    pub fn encodingfilter(&self, String encoding) -> String {
      // Java: if (null == encoding)
        // TODO_RUST_CODE
      {
         // Java: return this.getEncodingByCurrentLanguage();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return !encoding.equals(Constants.CHARSET_GB18030) && !encoding.equals(Constants.CHARSET_IBM855) && !encoding.equals(Constants.CHARSET_KOI8_R) && !encoding.equals(Constants.CHARSET_ISO_8859_5) && !encoding.equals(Constants.CHARSET_IBM866) && !encoding.equals(Constants.CHARSET_EUC_TW) ? encoding : Constants.CHARSET_GBK;
         // TODO_RUST_CODE
      }
    }

    // Java: String getEncodingByCurrentLanguage()
    pub fn getEncodingByCurrentLanguage(&self) -> String {
      // Java: String language = Locale.getDefault().getLanguage();
      // TODO_RUST_CODE
      // Java: return "th".equals(language) ? Constants.CHARSET_TIS_620 : Constants.CHARSET_GBK;
      // TODO_RUST_CODE
    }

    // Java: String encodingCorrect(String encoding)
    pub fn encodingCorrect(&self, String encoding) -> String {
      // Java: return (!encoding.equals(Constants.CHARSET_EUC_JP) || !this.isNeededCorrect(_EUC_JP_BYTES)) && (!encoding.equals(Constants.CHARSET_EUC_KR) || !this.isNeededCorrect(_EUC_KR_BYTES)) ? encoding : Constants.CHARSET_GBK;
      // TODO_RUST_CODE
    }

    // Java: boolean isNeededCorrect(byte[] referBytes)
    pub fn isNeededCorrect(&self, byte[] referBytes) -> bool {
      // Java: int nread = 0;
      // TODO_RUST_CODE
      // Java: int count = 0;
      // TODO_RUST_CODE
      // Java: int mark = 0;
      // TODO_RUST_CODE
      // Java: int referSize = referBytes.length;
      // TODO_RUST_CODE
      // Java: byte[] buf = new byte[256];
      // TODO_RUST_CODE
      // Java: InputStream fis = (InputStream)(null != this.mRaf ? new RafStream(this.mRaf) : new FileInputStream(this._path));
      // TODO_RUST_CODE
      // Java: while((nread = fis.read(buf)) > 0 && count < 4096)
        // TODO_RUST_CODE
      {
         // Java: count += nread;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < nread; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (i + 1 < nread)
                // TODO_RUST_CODE
            {
               // Java: for(int j = 0; j < referSize; j += 2)
                    // TODO_RUST_CODE
               {
                  // Java: if (buf[i] == referBytes[j] && buf[i + 1] == referBytes[j + 1])
                        // TODO_RUST_CODE
                  {
                     // Java: ++mark;
                     // TODO_RUST_CODE
                  }
               }
            }
         }
      }
      // Java: return mark * 2 * 2 >= count;
      // TODO_RUST_CODE
    }

    // Java: String getLine()
    pub fn getLine(&self) -> String {
      // Java: String line = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: line = this._input.readLine();
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: IOAssert.fail("it should not reach here");
         // TODO_RUST_CODE
      }
      // Java: return line;
      // TODO_RUST_CODE
    }

    // Java: String getLineWithoutBom()
    pub fn getLineWithoutBom(&self) -> String {
      // Java: String line = this.getLine();
      // TODO_RUST_CODE
      // Java: if (this.mHasBom && null != line && line.length() != 0)
        // TODO_RUST_CODE
      {
         // Java: return '\ufeff' == line.charAt(0) ? line.substring(1) : line;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return line;
         // TODO_RUST_CODE
      }
    }

    // Java: void close()
    pub fn close(&self) {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: this._input.close();
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: IOAssert.fail("it should not reach here");
         // TODO_RUST_CODE
      }
    }

}

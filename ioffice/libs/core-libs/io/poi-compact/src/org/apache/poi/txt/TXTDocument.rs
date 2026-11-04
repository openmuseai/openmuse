#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// TYPE_ORACLE / java2rust: rewritten stub — original template had 100+ errors and
// referenced crates not linked in core_libs_poi_compact. Public API matches
// org.apache.poi.txt.TXTDocument (228-line Java). Full charset/IO wiring deferred.

// java private static final String TAG = null;
const TAG: &str = "TXTDocument";
// java private static final byte[] _EUC_JP_BYTES = new byte[]{-71, -2};
const _EUC_JP_BYTES: [i8; 2] = [-71, -2];
// java private static final byte[] _EUC_KR_BYTES = new byte[]{-80, -95};
const _EUC_KR_BYTES: [i8; 2] = [-80, -95];
// java private static final int BUFFER_SIZE = 1024;
const BUFFER_SIZE: i32 = 1024;
// java private static final int MAX_READ_LENGTH = 4096;
const MAX_READ_LENGTH: i32 = 4096;
// java private static final String THAI_LANGUAGE = "th";
const THAI_LANGUAGE: &str = "th";
// java private static final char BOM_CHAR = '\ufeff';
const BOM_CHAR: char = '\u{feff}';

// java public final class TXTDocument
#[derive(Default)]
pub struct TXTDocument {
    // java private String _path;
    _path: Option<String>,
    // java private final RandomAccessFile mRaf;
    // held as opaque until kfs is wired into poi-compact
    mRaf: Option<()>,
    // java private String _encoding;
    _encoding: Option<String>,
    // java private BufferedReader _input;
    _input: Option<std::io::BufReader<Box<dyn std::io::Read + Send>>>,
    // java private InputStream _fis;
    _fis: Option<Box<dyn std::io::Read + Send>>,
    // java private boolean mHasBom;
    mHasBom: bool,
    lines: Vec<String>,
    line_idx: usize,
}

// java public final class TXTDocument
impl TXTDocument {
    // java public TXTDocument(String path, RandomAccessFile raf, String encoding)
    pub fn new(path: &str, _raf: Option<()>, encoding: &str) -> Self {
        // java this._path = path;
        // java this.mRaf = raf;
        // java this._input = null;
        let mut this = Self {
            _path: Some(path.to_string()),
            mRaf: _raf,
            _encoding: None,
            _input: None,
            _fis: None,
            mHasBom: false,
            lines: Vec::new(),
            line_idx: 0,
        };
        // java String guessEncoding = this.detectEncoding();
        let guessEncoding = this.detectEncoding();
        // java if (null != guessEncoding) { this._encoding = guessEncoding; } else { this._encoding = encoding; }
        this._encoding = Some(guessEncoding.unwrap_or_else(|| encoding.to_string()));
        // open path as line source when possible
        if let Some(ref p) = this._path {
            if let Ok(text) = std::fs::read_to_string(p) {
                this.mHasBom = text.starts_with(BOM_CHAR);
                this.lines = text.lines().map(|s| s.to_string()).collect();
            }
        }
        this
    }

    // java public void updateEncoding(String encoding)
    pub fn updateEncoding(&mut self, encoding: Option<&str>) {
        // java if (null != encoding)
        if let Some(encoding) = encoding {
            // java this._encoding = encoding;
            self._encoding = Some(encoding.to_string());
        }
    }

    // java public int guessDocumentLength()
    pub fn guessDocumentLength(&mut self) -> i32 {
        // java return this._fis.available();
        self.lines.iter().map(|l| l.len() + 1).sum::<usize>() as i32
    }

    // java public String getEncoding()
    pub fn getEncoding(&self) -> Option<String> {
        // java return this._encoding;
        self._encoding.clone()
    }

    // java private String detectEncoding()
    fn detectEncoding(&mut self) -> Option<String> {
        // Full UniversalDetector path deferred; prefer BOM then default GBK-like fallback.
        if let Some(ref p) = self._path {
            if let Ok(mut f) = std::fs::File::open(p) {
                let mut buf = [0u8; 4];
                use std::io::Read;
                if let Ok(n) = f.read(&mut buf) {
                    if let Some(enc) = self.getCharsetByBom(&buf, n as i32) {
                        return Some(enc);
                    }
                }
            }
        }
        self.getEncodingByCurrentLanguage()
    }

    // java private String getCharsetByBom(byte[] buf, int length)
    fn getCharsetByBom(&mut self, buf: &[u8], length: i32) -> Option<String> {
        if length > 3 {
            let b1 = buf[0] as i32 & 255;
            let b2 = buf[1] as i32 & 255;
            let b3 = buf[2] as i32 & 255;
            let b4 = buf[3] as i32 & 255;
            if b1 == 239 && b2 == 187 && b3 == 191 {
                self.mHasBom = true;
                return Some("UTF-8".to_string());
            }
            if b1 == 254 && b2 == 255 {
                self.mHasBom = true;
                return Some("UTF-16BE".to_string());
            }
            if b1 == 0 && b2 == 0 && b3 == 254 && b4 == 255 {
                self.mHasBom = true;
                return Some("UTF-32BE".to_string());
            }
            if b1 == 255 && b2 == 254 && b3 == 0 && b4 == 0 {
                self.mHasBom = true;
                return Some("UTF-32LE".to_string());
            }
            if b1 == 255 && b2 == 254 {
                self.mHasBom = true;
                return Some("UTF-16LE".to_string());
            }
        }
        None
    }

    // java private void closeInputStream(InputStream fis)
    fn closeInputStream(&mut self, _fis: &mut dyn std::io::Read) {
        // no-op for std Read
    }

    // java private String encodingfilter(String encoding)
    fn encodingfilter(&self, encoding: Option<&str>) -> Option<String> {
        match encoding {
            None => self.getEncodingByCurrentLanguage(),
            Some(enc)
                if enc == "GB18030"
                    || enc == "IBM855"
                    || enc == "KOI8-R"
                    || enc == "ISO-8859-5"
                    || enc == "IBM866"
                    || enc == "EUC-TW" =>
            {
                Some("GBK".to_string())
            }
            Some(enc) => Some(enc.to_string()),
        }
    }

    // java private String getEncodingByCurrentLanguage()
    fn getEncodingByCurrentLanguage(&self) -> Option<String> {
        // java String language = Locale.getDefault().getLanguage();
        let language = std::env::var("LANG").unwrap_or_default();
        if language.starts_with(THAI_LANGUAGE) {
            Some("TIS-620".to_string())
        } else {
            Some("GBK".to_string())
        }
    }

    // java private String encodingCorrect(String encoding)
    fn encodingCorrect(&mut self, encoding: &str) -> String {
        let _ = (&_EUC_JP_BYTES, &_EUC_KR_BYTES);
        encoding.to_string()
    }

    // java private boolean isNeededCorrect(byte[] referBytes)
    fn isNeededCorrect(&mut self, _referBytes: &[i8]) -> bool {
        false
    }

    // java public String getLine()
    pub fn getLine(&mut self) -> Option<String> {
        if self.line_idx < self.lines.len() {
            let line = self.lines[self.line_idx].clone();
            self.line_idx += 1;
            Some(line)
        } else {
            None
        }
    }

    // java public String getLineWithoutBom()
    pub fn getLineWithoutBom(&mut self) -> Option<String> {
        // java String line = this.getLine();
        let line = self.getLine();
        // java if (this.mHasBom && null != line && line.length() != 0)
        if self.mHasBom {
            if let Some(ref line) = line {
                if !line.is_empty() && line.chars().next() == Some(BOM_CHAR) {
                    return Some(line.chars().skip(1).collect());
                }
            }
        }
        line
    }

    // java public void close()
    pub fn close(&mut self) {
        self._input = None;
        self._fis = None;
        self.lines.clear();
        self.line_idx = 0;
    }
}

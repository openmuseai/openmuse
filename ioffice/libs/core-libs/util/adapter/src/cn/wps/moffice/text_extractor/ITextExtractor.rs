// Java: import org.apache.poi.IPOIFSFileSystem

use std::sync::{OnceLock, Mutex};

// Java field: ITextExtractor sDefaultExtractor
static S_DEFAULT_EXTRACTOR: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();
// Java field: ITEListener sDefaultListener
static S_DEFAULT_LISTENER: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();
// Java field: int DEFAULT_MAX_LENNTH
pub(crate) const DEFAULT_MAX_LENNTH: i32 = 3000;

// Java: public class ITextExtractor
pub struct ITextExtractor {
    // Java field: int mTextLength
    pub(crate) mTextLength: i32,
    // Java field: String mPath
    pub(crate) mPath: Option<String>,
    // Java field: IPOIFSFileSystem mPoifs
    pub(crate) mPoifs: Option<Box<dyn std::any::Any>>,
    // Java field: String mPassword
    pub(crate) mPassword: Option<String>,
    // Java field: ITEListener mListener
    pub(crate) mListener: Option<Box<dyn std::any::Any>>,
    // Java field: StringBuilder mText
    pub(crate) mText: String,
}

// Java: public class ITextExtractor
impl ITextExtractor {
    // Java: ITextExtractor(String path, String password, ITEListener listener)
    pub fn new(_path: &str, _password: &str, _listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: this((String)path, password, 3000, listener);
      Self::new_with_length(_path, _password, 3000, _listener)
    }

    // Java: ITextExtractor(String path, String password, int length, ITEListener listener)
    pub fn new_with_length(_path: &str, _password: &str, _length: i32, _listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super();
      // Java: this.mPath = path;
      // Java: this.mPassword = password;
      // Java: this.mTextLength = length;
      // Java: this.mListener = sDefaultListener;
      // Java: this.mText = new StringBuilder();
      // Java: if (null != listener)
      // Java: this.mListener = listener;
      ITextExtractor {
          mTextLength: _length,
          mPath: Some(_path.to_string()),
          mPoifs: None,
          mPassword: Some(_password.to_string()),
          mListener: _listener,
          mText: String::new(),
      }
    }

    // Java: ITextExtractor(IPOIFSFileSystem poifs, String password, int length, ITEListener listener)
    pub fn new_with_poifs(_poifs: Option<Box<dyn std::any::Any>>, _password: &str, _length: i32, _listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super();
      // Java: this.mPoifs = poifs;
      // Java: this.mPassword = password;
      // Java: this.mListener = sDefaultListener;
      // Java: this.mTextLength = length;
      // Java: this.mText = new StringBuilder();
      // Java: if (null != listener)
      // Java: this.mListener = listener;
      ITextExtractor {
          mTextLength: _length,
          mPath: None,
          mPoifs: _poifs,
          mPassword: Some(_password.to_string()),
          mListener: _listener,
          mText: String::new(),
      }
    }

    // Java: int getMaxLength()
    pub fn getMaxLength(&self) -> i32 {
      // Java: return this.mTextLength;
      self.mTextLength
    }

    // Java: void appendText(char[] chars, int start, int len)
    pub fn appendText(&mut self, _chars: &[char], _start: i32, _len: i32) {
      // Java: this.mText.append(chars, start, len);
      let start = _start as usize;
      let len = _len as usize;
      let slice = &_chars[start..start + len];
      let s: String = slice.iter().collect();
      self.mText.push_str(&s);
    }

    // Java: boolean enough()
    pub fn enough(&self) -> bool {
      // Java: return this.mText != null && this.mText.length() >= this.getMaxLength();
      self.mText.len() as i32 >= self.getMaxLength()
    }

    // Java: String result()
    pub fn result(&self) -> String {
      // Java: return "";
      String::new()
    }

    // Java: boolean isEncryptedFile()
    pub fn isEncryptedFile(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isWpsSecurityFile(String path)
    pub fn isWpsSecurityFile(&self, _path: &str) -> bool {
      // Java: return false;
      false
    }

}

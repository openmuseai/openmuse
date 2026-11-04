use super::ITextExtractor::ITextExtractor;

// Java field: int TYPE_UNKNOWN
const TYPE_UNKNOWN: i32 = 0;
// Java field: int TYPE_DOC
const TYPE_DOC: i32 = 1;
// Java field: int TYPE_DOCX
const TYPE_DOCX: i32 = 2;
// Java field: int TYPE_XLS
const TYPE_XLS: i32 = 3;
// Java field: int TYPE_XLSX
const TYPE_XLSX: i32 = 4;
// Java field: int TYPE_PPT
const TYPE_PPT: i32 = 5;
// Java field: int TYPE_PPTX
const TYPE_PPTX: i32 = 6;
// Java field: int TYPE_TXT
const TYPE_TXT: i32 = 7;

// Java: public class TextExtractor extends ITextExtractor
pub struct TextExtractor {
    pub(crate) base: ITextExtractor,
    // Java field: TEThread mRunnable
    mRunnable: Option<Box<dyn std::any::Any>>,
}

// Java: public class TextExtractor extends ITextExtractor
impl TextExtractor {
    // Java: TextExtractor(String path, String password, int length, ITEListener listener)
    pub fn new_with_length(_path: &str, _password: &str, _length: i32, _listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super(path, password, length, listener);
      TextExtractor {
          base: ITextExtractor::new_with_length(_path, _password, _length, _listener),
          mRunnable: None,
      }
    }

    // Java: TextExtractor(String path, String password, ITEListener listener)
    pub fn new(_path: &str, _password: &str, _listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super(path, password, listener);
      TextExtractor {
          base: ITextExtractor::new(_path, _password, _listener),
          mRunnable: None,
      }
    }

    // Java: void run()
    pub fn run(&mut self) {
      // Java: this.mRunnable = new TEThread(this);
      // Java: (new Thread(this.mRunnable, "TextExtractor")).start();
      todo!()
    }

    // Java: ITEListener getListener()
    pub fn getListener(&self) -> &Option<Box<dyn std::any::Any>> {
      // Java: return this.mListener;
      &self.base.mListener
    }

    // Java: String result()
    pub fn result(&self) -> String {
      // Java: int type = this.type(this.mPath);
      // Java: ITextExtractor extractor = sDefaultExtractor;
      // Java: switch (type)
      // Java: case 1:
      // Java: case 2:
      // Java: extractor = ReflectionHelper.getWriterTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: case 3:
      // Java: case 4:
      // Java: case 5:
      // Java: case 6:
      // Java: break;
      // Java: case 7:
      // Java: extractor = ReflectionHelper.getTxtTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: break;
      // Java: default:
      // Java: extractor = sDefaultExtractor;
      // Java: return extractor.result();
      todo!()
    }

    // Java: boolean isEncryptedFile()
    pub fn isEncryptedFile(&self) -> bool {
      // Java: int type = this.type(this.mPath);
      // Java: ITextExtractor extractor = sDefaultExtractor;
      // Java: switch (type)
      // Java: case 1:
      // Java: case 2:
      // Java: extractor = ReflectionHelper.getWriterTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: case 3:
      // Java: case 4:
      // Java: case 5:
      // Java: case 6:
      // Java: break;
      // Java: case 7:
      // Java: extractor = ReflectionHelper.getTxtTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: break;
      // Java: default:
      // Java: extractor = sDefaultExtractor;
      // Java: return extractor.isEncryptedFile();
      todo!()
    }

    // Java: boolean isWpsSecurityFile(String path)
    pub fn isWpsSecurityFile(&self, _path: &str) -> bool {
      // Java: int type = this.type(this.mPath);
      // Java: ITextExtractor extractor = sDefaultExtractor;
      // Java: switch (type)
      // Java: case 1:
      // Java: case 2:
      // Java: extractor = ReflectionHelper.getWriterTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: case 3:
      // Java: case 4:
      // Java: case 5:
      // Java: case 6:
      // Java: break;
      // Java: case 7:
      // Java: extractor = ReflectionHelper.getTxtTextExtractor(this, this.mPath, this.mPassword, this.mTextLength, this.mListener);
      // Java: break;
      // Java: default:
      // Java: extractor = sDefaultExtractor;
      // Java: return extractor.isWpsSecurityFile(this.mPath);
      todo!()
    }

    // Java: int type(String path)
    fn r#type(&self, _path: &str) -> i32 {
      // Java: int index = path.lastIndexOf(".");
      // Java: if (index + 1 >= path.length())
      // Java: return 0;
      let index = match _path.rfind('.') {
          Some(i) => i,
          None => return TYPE_UNKNOWN,
      };
      if index + 1 >= _path.len() {
          return TYPE_UNKNOWN;
      }
      // Java: else
      // Java: String ext = path.substring(index + 1);
      let ext = &_path[index + 1..];
      let ext_lower = ext.to_lowercase();
      // Java: if (ext.equalsIgnoreCase("doc"))
      // Java: return 1;
      // Java: else if (ext.equalsIgnoreCase("docx"))
      // Java: return 2;
      // Java: else if (ext.equalsIgnoreCase("xls"))
      // Java: return 3;
      // Java: else if (ext.equalsIgnoreCase("xlsx"))
      // Java: return 4;
      // Java: else if (ext.equalsIgnoreCase("ppt"))
      // Java: return 5;
      // Java: else if (ext.equalsIgnoreCase("pptx"))
      // Java: return 6;
      // Java: else
      // Java: return ext.equalsIgnoreCase("txt") ? 7 : 0;
      match ext_lower.as_str() {
          "doc" => TYPE_DOC,
          "docx" => TYPE_DOCX,
          "xls" => TYPE_XLS,
          "xlsx" => TYPE_XLSX,
          "ppt" => TYPE_PPT,
          "pptx" => TYPE_PPTX,
          "txt" => TYPE_TXT,
          _ => TYPE_UNKNOWN,
      }
    }

}

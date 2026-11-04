// Java: import java.lang.reflect.Method

// Java field: int TYPE_UNKNOWN
pub(crate) const TYPE_UNKNOWN: i32 = 0;
// Java field: int TYPE_DOC
pub(crate) const TYPE_DOC: i32 = 1;
// Java field: int TYPE_XLS
pub(crate) const TYPE_XLS: i32 = 2;
// Java field: int TYPE_PPT
pub(crate) const TYPE_PPT: i32 = 3;
// Java field: int TYPE_PDF
pub(crate) const TYPE_PDF: i32 = 4;

// Java: public class DLThread implements Runnable
pub struct DLThread {
    // Java field: Object mOwner
    mOwner: Option<Box<dyn std::any::Any>>,
    // Java field: String mPath
    mPath: Option<String>,
    // Java field: String mPassword
    mPassword: Option<String>,
    // Java field: IDocumentLoaderListener mListener
    mListener: Option<Box<dyn std::any::Any>>,
    // Java field: Object mContext
    mContext: Option<Box<dyn std::any::Any>>,
    // Java field: IDLProvider mDlProvider
    mDlProvider: Option<Box<dyn std::any::Any>>,
    // Java field: boolean mIgnoreSecurity
    mIgnoreSecurity: bool,
}

// Java: public class DLThread implements Runnable
impl DLThread {
    // Java: DLThread(Object owner, String path, String password, IDocumentLoaderListener listener, Object context, IDLProvider dlProvider, boolean ignoreSecurity)
    pub fn new(_owner: Box<dyn std::any::Any>, _path: &str, _password: &str, _listener: Box<dyn std::any::Any>, _context: Box<dyn std::any::Any>, _dlProvider: Box<dyn std::any::Any>, _ignoreSecurity: bool) -> Self {
      // Java: super();
      DLThread {
          // Java: this.mOwner = owner;
          mOwner: Some(_owner),
          // Java: this.mPath = path;
          mPath: Some(_path.to_string()),
          // Java: this.mPassword = password;
          mPassword: Some(_password.to_string()),
          // Java: this.mListener = listener;
          mListener: Some(_listener),
          // Java: this.mContext = context;
          mContext: Some(_context),
          // Java: this.mDlProvider = dlProvider;
          mDlProvider: Some(_dlProvider),
          // Java: this.mIgnoreSecurity = ignoreSecurity;
          mIgnoreSecurity: _ignoreSecurity,
      }
    }

    // Java: void run()
    pub fn run(&self) {
      // Java: int type = this.type(this.mPath);
      let _type = self.get_type(self.mPath.as_deref().unwrap_or(""));
      // Java: IDocument doc = null;
      let mut _doc: Option<Box<dyn std::any::Any>> = None;
      // Java: switch (type)
      match _type {
         // Java: case 1:
         1 => {
            // Java: doc = ReflectionHelper.getTextDocument(this.mOwner);
            // Java: break;
            todo!()
         }
         // Java: case 2:
         2 => {
            // Java: doc = ReflectionHelper.getKmoBook(this.mOwner);
            // Java: break;
            todo!()
         }
         // Java: case 3:
         3 => {
            // Java: doc = ReflectionHelper.getKmoPresentation(this.mOwner);
            // Java: break;
            todo!()
         }
         // Java: case 4:
         _ => {
            // Java: doc = ReflectionHelper.getPDFDocument(this.mOwner);
            todo!()
         }
      }
      // Java: if (null != doc)
      if _doc.is_some()
      {
         // Java: int ret = doc.open(this.mPath, this.mPassword, this.mContext, this.mDlProvider, this.mIgnoreSecurity);
         let _ret: i32 = todo!();
         // Java: if (3 == ret)
         if 3 == _ret
         {
            // Java: this.mListener.onNeedPassword(doc);
            // Java: return;
            todo!()
         }
         // Java: if (1 == ret && 4 == type)
         if 1 == _ret && 4 == _type
         {
            // Java: try
            {
               // Java: Method isOwner = doc.getClass().getMethod("isOnwer");
               // Java: boolean owner = Boolean.parseBoolean(isOwner.invoke(doc).toString());
               let _owner: bool = todo!();
               // Java: if (!owner)
               if !_owner
               {
                  // Java: this.mListener.onNeedPassword(doc);
                  // Java: return;
                  todo!()
               }
            }
            // Java: catch (Exception var6)
            {
               // Java: this.mListener.onNeedPassword(doc);
               // Java: return;
            }
         }
         // Java: if (null != doc && 4 == ret)
         if _doc.is_some() && 4 == _ret
         {
            // Java: this.mListener.onOpenForceQuit();
            todo!()
         }
         // Java: if (null != doc && 1 != ret)
         if _doc.is_some() && 1 != _ret
         {
            // Java: doc = null;
            _doc = None;
         }
      }
      // Java: this.mListener.onLoadFinish(doc);
      todo!()
    }

    // Java: int type(String path)
    fn get_type(&self, _path: &str) -> i32 {
      // Java: int index = path.lastIndexOf(".");
      let index = _path.rfind('.').map(|i| i as i32).unwrap_or(-1);
      // Java: if (index + 1 >= path.length())
      if (index + 1) >= _path.len() as i32
      {
         // Java: return 0;
         return 0;
      }
      // Java: else
      {
         // Java: String ext = path.substring(index + 1);
         let ext = &_path[(index as usize + 1)..];
         // Java: if (!ext.equalsIgnoreCase("doc") && !ext.equalsIgnoreCase("wps") && !ext.equalsIgnoreCase("docx") && !ext.equalsIgnoreCase("dotx") && !ext.equalsIgnoreCase("wpt") && !ext.equalsIgnoreCase("dot") && !ext.equalsIgnoreCase("docm") && !ext.equalsIgnoreCase("dotm") && !ext.equalsIgnoreCase("rtf") && !ext.equalsIgnoreCase("txt") && !ext.equalsIgnoreCase("uot3"))
         let ext_lower = ext.to_lowercase();
         if !matches!(ext_lower.as_str(), "doc" | "wps" | "docx" | "dotx" | "wpt" | "dot" | "docm" | "dotm" | "rtf" | "txt" | "uot3")
         {
            // Java: if (!ext.equalsIgnoreCase("xls") && !ext.equalsIgnoreCase("et") && !ext.equalsIgnoreCase("xlsx") && !ext.equalsIgnoreCase("xltx") && !ext.equalsIgnoreCase("xlt") && !ext.equalsIgnoreCase("ett") && !ext.equalsIgnoreCase("csv") && !ext.equalsIgnoreCase("xlsm") && !ext.equalsIgnoreCase("xlsb") && !ext.equalsIgnoreCase("xltm"))
            if !matches!(ext_lower.as_str(), "xls" | "et" | "xlsx" | "xltx" | "xlt" | "ett" | "csv" | "xlsm" | "xlsb" | "xltm")
            {
               // Java: if (!ext.equalsIgnoreCase("ppt") && !ext.equalsIgnoreCase("dps") && !ext.equalsIgnoreCase("pptx") && !ext.equalsIgnoreCase("pot") && !ext.equalsIgnoreCase("dpt") && !ext.equalsIgnoreCase("potx") && !ext.equalsIgnoreCase("pptm") && !ext.equalsIgnoreCase("potm"))
               if !matches!(ext_lower.as_str(), "ppt" | "dps" | "pptx" | "pot" | "dpt" | "potx" | "pptm" | "potm")
               {
                  // Java: return ext.equalsIgnoreCase("pdf") ? 4 : 0;
                  return if ext_lower == "pdf" { 4 } else { 0 };
               }
               // Java: else
               {
                  // Java: return 3;
                  return 3;
               }
            }
            // Java: else
            {
               // Java: return 2;
               return 2;
            }
         }
         // Java: else
         {
            // Java: return 1;
            return 1;
         }
      }
    }

}

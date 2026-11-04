// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.online.security.IOnlineSecurityTool

// Java: public class DocumentLoader
pub struct DocumentLoader {
}

// Java: public class DocumentLoader
impl DocumentLoader {
    // Java: DocumentLoader()
    pub fn new() -> Self {
      // Java: super();
      DocumentLoader {}
    }

    // Java: static void openDocument(Object owner, String path, String password, IDocumentLoaderListener listener, Object context, IDLProvider provider)
    pub fn openDocument(_owner: &dyn std::any::Any, _path: &str, _password: &str, _listener: &dyn std::any::Any, _context: &dyn std::any::Any, _provider: &dyn std::any::Any) {
      // Java: openDocument(owner, path, password, listener, context, provider, false);
      todo!()
    }

    // Java: static boolean isSecurityFile(Object owner, String path)
    pub fn isSecurityFile(_owner: &dyn std::any::Any, _path: &str) -> bool {
      // Java: IOnlineSecurityTool securityTool = ReflectionHelper.getSecurityTool(owner);
      // Java: return null == securityTool ? false : securityTool.isWpsSecurityFile(path);
      todo!()
    }

    // Java: static void openDocument(Object owner, String path, String password, IDocumentLoaderListener listener, Object context, IDLProvider provider, boolean ignoreSecurity)
    pub fn openDocument_with_security(_owner: &dyn std::any::Any, _path: &str, _password: &str, _listener: &dyn std::any::Any, _context: &dyn std::any::Any, _provider: &dyn std::any::Any, _ignoreSecurity: bool) {
      // Java: if (null == path)
      {
         // Java: listener.onLoadFinish((IDocument)null);
      }
      // Java: else
      {
         // Java: File file = new File(path);
         // Java: if (!file.exists())
         {
            // Java: listener.onLoadFinish((IDocument)null);
         }
         // Java: else
         {
            // Java: DLThread thread = new DLThread(owner, path, password, listener, context, provider, ignoreSecurity);
            // Java: (new Thread(thread, "DLThread")).start();
            todo!()
         }
      }
    }

}

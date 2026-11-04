// Java: import cn.wps.moffice.writer.io.WriterIOException
use crate::cn::wps::moffice::writer::io::WriterIOException::*;

// java public interface IDocumentCallback
pub trait IDocumentCallback {
    // java  boolean encryptFile(String var1, String var2)
    fn encryptFile(&mut self, var1: &str, var2: &str) -> bool;

    // java  String decryptFile(String var1, String var2)
    fn decryptFile(&mut self, var1: &str, var2: &str) -> String;

    // java  void dispose()
    fn dispose(&mut self);

    // java  boolean isProEncryption()
    fn isProEncryption(&self) -> bool;

}

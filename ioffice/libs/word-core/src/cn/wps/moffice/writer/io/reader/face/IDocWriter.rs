// Java: import cn.wps.moffice.writer.io.WriterAutoWriteException
use crate::cn::wps::moffice::writer::io::WriterAutoWriteException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;

// java public interface IDocWriter
pub trait IDocWriter {
    // java  void write()
    fn write(&mut self);

}

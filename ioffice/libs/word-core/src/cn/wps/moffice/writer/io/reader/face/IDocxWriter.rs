// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use io_wps_io::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;

// java public interface IDocxWriter
pub trait IDocxWriter {
    // java  void write()
    fn write(&mut self);

    // java  byte[] exportAsArray()
    fn exportAsArray(&mut self) -> Vec<u8>;

}

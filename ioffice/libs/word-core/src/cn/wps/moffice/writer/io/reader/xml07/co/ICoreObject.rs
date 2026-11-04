// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::*;

// java public interface ICoreObject
pub trait ICoreObject {
    // java  boolean isReady()
    fn isReady(&self) -> bool;

    // java  void start()
    fn start(&mut self);

    // java  void end()
    fn end(&mut self);

    // java  ICoreObject getNext()
    fn getNext(&self) -> &(dyn ICoreObject);

    // java  void setNext(ICoreObject var1)
    fn setNext(&mut self, var1: &dyn ICoreObject);

    // java  int getType()
    fn getType(&self) -> i32;

    // java  void setOwnerPart(String var1)
    fn setOwnerPart(&mut self, var1: &str);

    // java  String getOwnerPart()
    fn getOwnerPart(&self) -> String;

    // java  void process(Xml07Context var1, IProcInfo var2)
    fn process(&mut self, var1: &Xml07Context, var2: &dyn IProcInfo);

}

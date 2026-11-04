// Java: import cn.wps.ole.IOLEStorage
// Java: import java.io.InputStream
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
use crate::org::apache::poi::poifs::filesystem::DirectoryEntry::DirectoryEntry;

pub trait IOLEStorage: std::any::Any {}

// Java: public interface IOlePacker
pub trait IOlePacker {
    // Java: abstract String[] getEntryNames()
    fn getEntryNames(&self) -> Vec<String>;

    // Java: abstract InputStream getEntryStream(String var1)
    fn getEntryStream(&self, var1: &str) -> Option<Box<dyn std::io::Read>>;

    // Java: abstract void writeData(IOLEStorage var1)
    fn writeData_storage(&mut self, var1: &mut dyn IOLEStorage);

    // Java: abstract void writeData(DirectoryEntry var1)
    fn writeData_entry(&mut self, var1: &mut dyn DirectoryEntry);

    // Java: abstract String packBinFile()
    fn packBinFile(&self) -> String;
}

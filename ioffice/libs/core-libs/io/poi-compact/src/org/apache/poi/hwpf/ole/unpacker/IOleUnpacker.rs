// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use crate::org::apache::poi::hwpf::util::OleParseInterruptException::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;

// java public interface IOleUnpacker
pub trait IOleUnpacker {
    // java  boolean parseFile(String var1)
    fn parseFile(&mut self, var1: &str) -> Option<bool>;

    // java  Entry parseEntry()
    fn parseEntry(&mut self) -> Option<&(dyn Entry)>;

}

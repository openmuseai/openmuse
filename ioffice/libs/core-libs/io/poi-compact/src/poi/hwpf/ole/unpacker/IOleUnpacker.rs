// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use io_poi_compact::org::apache::poi::hwpf::util::OleParseInterruptException;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry;

// Java: public interface IOleUnpacker
pub trait IOleUnpacker {
    // Java: abstract boolean parseFile(String var1)
    fn parseFile(&self, String var1) -> bool;

    // Java: abstract Entry parseEntry()
    fn parseEntry(&self) -> Entry;

}

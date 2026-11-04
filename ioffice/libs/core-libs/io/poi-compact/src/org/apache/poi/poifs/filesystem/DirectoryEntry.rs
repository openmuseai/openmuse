// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Iterator
// Java: import org.apache.poi.hpsf.ClassID

use std::sync::Arc;
use crate::org::apache::poi::hpsf::ClassID::ClassID;
use super::Entry::Entry;

// Java: public interface DirectoryEntry extends Entry
pub trait DirectoryEntry: Entry {
    // Java: abstract Iterator<Entry> getEntries()
    fn getEntries(&self) -> Vec<Arc<dyn Entry>>;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract int getEntryCount()
    fn getEntryCount(&self) -> i32;

    // Java: abstract Entry getEntry(String var1)
    fn getEntry(&self, var1: &str) -> Option<Arc<dyn Entry>>;

    // Java: abstract DocumentEntry createDocument(String var1, InputStream var2)
    fn createDocument(&mut self, var1: &str, var2: &mut dyn std::io::Read) -> std::io::Result<Arc<dyn Entry>>;

    // Java: abstract DocumentEntry createDocument(String var1, InputStream var2, boolean var3)
    fn createDocument_with_flag(&mut self, var1: &str, var2: &mut dyn std::io::Read, var3: bool) -> std::io::Result<Arc<dyn Entry>>;

    // Java: abstract DirectoryEntry createDirectory(String var1)
    fn createDirectory(&mut self, var1: &str) -> std::io::Result<Arc<dyn DirectoryEntry>>;

    // Java: abstract ClassID getStorageClsid()
    fn getStorageClsid(&self) -> &ClassID;

    // Java: abstract void setStorageClsid(ClassID var1)
    fn setStorageClsid(&mut self, var1: ClassID);
}

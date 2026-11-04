// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Enumeration

// Java: public interface UotZipEntrySource
pub trait UotZipEntrySource {
    // Java: abstract Enumeration<? extends ZipEntry> getEntries()
    fn getEntries(&self) -> Vec<ZipEntry>;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract InputStream getInputStream(ZipEntry var1)
    fn getInputStream(&self, var1: &ZipEntry) -> std::io::Result<Box<dyn std::io::Read>>;

    // Java: abstract void close()
    fn close(&mut self);

}

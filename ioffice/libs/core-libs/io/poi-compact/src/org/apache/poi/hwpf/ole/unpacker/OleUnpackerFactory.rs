// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.Iterator
// Java: import org.apache.poi.hwpf.HWPFDocumentCore
use crate::org::apache::poi::hwpf::HWPFDocumentCore::*;
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
use crate::org::apache::poi::poifs::filesystem::DirectoryEntry::DirectoryEntry as PoifsDirectoryEntry;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::Entry as PoifsEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.IOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::IOleUnpacker::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.CommonOleUnpacker
// CARGO_BLOCKER_UNUSED_MODULE: CommonOleUnpacker screened at mod.rs
// Java: import org.apache.poi.hwpf.ole.unpacker.ContentsOleUnpacker
// CARGO_BLOCKER_UNUSED_MODULE: ContentsOleUnpacker screened at mod.rs
// Java: import org.apache.poi.hwpf.ole.unpacker.Native10OleUnpacker
// CARGO_BLOCKER_UNUSED_MODULE: Native10OleUnpacker screened at mod.rs
// Java: import org.apache.poi.hwpf.ole.unpacker.CompoundOleUnpacker
// CARGO_BLOCKER_UNUSED_MODULE: CompoundOleUnpacker screened at mod.rs

// java public class OleUnpackerFactory
#[derive(Default)]
pub struct OleUnpackerFactory {}

// java public class OleUnpackerFactory
impl OleUnpackerFactory {
    // java public  OleUnpackerFactory()
    pub fn OleUnpackerFactory() -> Self {
        Self {}
    }

    // java public static IOleUnpacker getParser(Entry baseEntry)
    pub fn getParser__entry(_baseEntry: Option<&dyn PoifsEntry>) -> Option<Box<dyn IOleUnpacker>> {
        // DEFERRED_NONFEATURE_DEP: OLE unpacker cluster not on wc-02 parse path; oracle=PASS;shell=generated
        None
    }

    // java public static IOleUnpacker getParser(String binPath)
    pub fn getParser__string(_binPath: &str) -> Option<Box<dyn IOleUnpacker>> {
        // DEFERRED_NONFEATURE_DEP: OLE unpacker cluster not on wc-02 parse path; oracle=PASS;shell=generated
        None
    }
}

// Java: import java.io.IOException
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

use super::PlexOfCps::PlexOfCps;
use super::TextPieceTable::TextPieceTable;
use super::CHPX::CHPX;

// Java: public class CHPBinTableReadOnly extends CHPBinTable
pub struct CHPBinTableReadOnly {
    pub _binTable: PlexOfCps,
    pub tpt: Option<*mut TextPieceTable>,
    pub _fcMin: i32,
    pub fComplex: bool,
    pub _documentStream: Option<*mut DocumentInputStream>,
}

// Java: public class CHPBinTableReadOnly extends CHPBinTable
impl CHPBinTableReadOnly {
    // Java: CHPBinTableReadOnly(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int size, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new(document_stream: &mut DocumentInputStream, table_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: *mut TextPieceTable, f_complex: bool) -> Self {
        // Java: super();
        // Java: this._binTable = new PlexOfCps(tableStream, offset, size, 4);
        let _binTable = PlexOfCps::new_from_stream(table_stream, offset, size, 4);
        // Java: this.tpt = tpt;
        // Java: this._fcMin = fcMin;
        // Java: this.fComplex = fComplex;
        // Java: this._documentStream = documentStream;
        CHPBinTableReadOnly {
            _binTable,
            tpt: Some(tpt),
            _fcMin: fc_min,
            fComplex: f_complex,
            _documentStream: Some(document_stream as *mut DocumentInputStream),
        }
    }

    // Java: void panic()
    fn panic_ro(&self) {
        // Java: throw new RuntimeException("Read-Only");
        panic!("Read-Only");
    }

    // Java: void adjustForDelete(int listIndex, int offset, int length)
    pub fn adjustForDelete(&self, _list_index: i32, _offset: i32, _length: i32) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: void insert(int listIndex, int cpStart, SprmBuffer buf)
    pub fn insert(&self, _list_index: i32, _cp_start: i32, _buf: &[u8]) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: void add(int cpStart, int cpEnd, SprmBuffer buf)
    pub fn add(&self, _cp_start: i32, _cp_end: i32, _buf: &[u8]) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: void adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&self, _list_index: i32, _length: i32) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: List<CHPX> getTextRuns()
    pub fn getTextRuns(&self) -> Vec<CHPX> {
        // Java: throw new RuntimeException("Read-Only");
        panic!("Read-Only");
    }

    // Java: void writeTo(HWPFFileSystem sys, int fcMin)
    pub fn writeTo(&self, _fc_min: i32) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: CHPX getNextTextRun()
    pub fn getNextTextRun(&self) -> Option<CHPX> {
        // Java: this.panic();
        self.panic_ro();
        // Java: return null;
        None
    }

    // Java: void mark()
    pub fn mark(&self) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: void reset()
    pub fn reset(&self) {
        // Java: this.panic();
        self.panic_ro();
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&self, _bytes_offset: i32) {
        // Java: this.panic();
        self.panic_ro();
    }
}

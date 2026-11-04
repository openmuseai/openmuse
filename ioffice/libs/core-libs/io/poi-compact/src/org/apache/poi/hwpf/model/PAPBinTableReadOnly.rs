// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use super::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
use crate::org::apache::poi::hwpf::sprm::SprmBuffer::SprmBuffer;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use super::PAPBinTable::PAPBinTable;
use super::PlexOfCps::PlexOfCps;
use super::TextPieceTable::TextPieceTable;
use super::PAPBinTable::PAPX;

// Java: public class PAPBinTableReadOnly extends PAPBinTable
pub struct PAPBinTableReadOnly {
    pub base: PAPBinTable,
}

// Java: public class PAPBinTableReadOnly extends PAPBinTable
impl PAPBinTableReadOnly {
    // Java: PAPBinTableReadOnly(DocumentInputStream documentStream, DocumentInputStream tableStream, DocumentInputStream dataStream, int offset, int size, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new(document_stream: DocumentInputStream, table_stream: DocumentInputStream, data_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: TextPieceTable, f_complex: bool) -> Self {
      // Java: super();
      let mut base = PAPBinTable::new_empty();
      // Java: this._binTable = new PlexOfCps(tableStream, offset, size, 4);
      base._binTable = Some(PlexOfCps::new_from_stream(table_stream, offset, size, 4));
      // Java: this.tpt = tpt;
      base.tpt = Some(Box::new(tpt));
      // Java: this._fcMin = fcMin;
      base._fcMin = fc_min;
      // Java: this.fComplex = fComplex;
      base.fComplex = f_complex;
      // Java: this._dataStream = dataStream;
      base._dataStream = Some(data_stream);
      // Java: this._documentStream = documentStream;
      base._documentStream = Some(document_stream);
      PAPBinTableReadOnly { base }
    }

    // Java: void panic()
    fn do_panic(&self) {
      // Java: throw new RuntimeException("Read-Only");
      panic!("Read-Only");
    }

    // Java: void insert(int listIndex, int cpStart, SprmBuffer buf)
    pub fn insert(&self, _list_index: i32, _cp_start: i32, _buf: SprmBuffer) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void add(int cpStart, int cpEnd, SprmBuffer buf)
    pub fn add(&self, _cp_start: i32, _cp_end: i32, _buf: SprmBuffer) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void adjustForDelete(int listIndex, int offset, int length)
    pub fn adjustForDelete(&self, _list_index: i32, _offset: i32, _length: i32) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&self, _list_index: i32, _length: i32) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: ArrayList<PAPX> getParagraphs()
    pub fn getParagraphs(&self) -> &Vec<PAPX> {
      // Java: this.panic();
      self.do_panic();
      // Java: return null;
      unreachable!()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: this.panic();
      self.do_panic();
      // Java: return false;
      unreachable!()
    }

    // Java: PAPX getNextParagraph()
    pub fn getNextParagraph(&self) -> Option<&PAPX> {
      // Java: this.panic();
      self.do_panic();
      // Java: return null;
      unreachable!()
    }

    // Java: void writeTo(HWPFFileSystem sys, int fcMin)
    pub fn writeTo(&self, _sys: &mut HWPFFileSystem, _fc_min: i32) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void mark()
    pub fn mark(&self) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void reset()
    pub fn reset(&self) {
      // Java: this.panic();
      self.do_panic();
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&self, _bytes_offset: i32) {
      // Java: this.panic();
      self.do_panic();
    }

}

impl std::ops::Deref for PAPBinTableReadOnly {
    type Target = PAPBinTable;
    fn deref(&self) -> &PAPBinTable { &self.base }
}

impl std::ops::DerefMut for PAPBinTableReadOnly {
    fn deref_mut(&mut self) -> &mut PAPBinTable { &mut self.base }
}
